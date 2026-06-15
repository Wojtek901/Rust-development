use axum::{extract::State, routing::get, Json, Router};
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::fs::File;
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::mpsc;

#[derive(serde::Serialize, serde::Deserialize, Clone, sqlx::FromRow)]
struct TemperatureSensorLog{
    sensor_id: String,
    temperature: f64,
}

trait Validatable{
    fn is_valid(&self) -> bool;
}

impl Validatable for TemperatureSensorLog{
    fn is_valid(&self) -> bool {
        self.temperature >= -273.15
    }
}

async fn get_measurements(State(db_pool): State<SqlitePool>) -> Json<Vec<TemperatureSensorLog>>{
    let sql_querry = "SELECT * FROM sensor_logs";
    let db_data = sqlx::query_as::<_, TemperatureSensorLog>(sql_querry).fetch_all(&db_pool).await.unwrap();
    Json(db_data)
}

async fn save_measurements(State(db_pool): State<SqlitePool>, Json(sensor_data): Json<TemperatureSensorLog>){
    if !sensor_data.is_valid(){
        println!("Invalid measurements. Temperature below absolute 0. Recieved value: {}", sensor_data.temperature);
    }
    else{
        sqlx::query("INSERT INTO sensor_logs (sensor_id, temperature) VALUES (?, ?)")
            .bind(sensor_data.sensor_id).bind(sensor_data.temperature).execute(&db_pool).await.unwrap();
    }
}

async fn init_db(db_name: &str) -> Pool<Sqlite>{
    if !Path::new(db_name).exists(){
        File::create(db_name).unwrap();
    }

    let db_url = format!("sqlite://{}", db_name);
    let pool = SqlitePool::connect(&db_url).await.unwrap();
    let db_create_command = "CREATE TABLE IF NOT EXISTS sensor_logs (sensor_id STRING, measurement FLOAT)";

    sqlx::query(&db_create_command).execute(&pool).await.unwrap();

    pool
}

#[tokio::main]
async fn main() {

    // Database
    let db_name = "Sensors.db";
    let pool = init_db(db_name).await;

    // Transmition chanel
    let buffer_size: usize = 100;
    let (tx, mut rx) = mpsc::channel::<TemperatureSensorLog>(buffer_size);

    let rx_pool = pool.clone();
    tokio::spawn(async move{
        while let Some(message) = rx.recv().await{
            sqlx::query("INSERT INTO sensor_logs (sensor_id, temperature) VALUES (?, ?)")
                .bind(message.sensor_id).bind(message.temperature).execute(&rx_pool).await.unwrap();
        }
    });

    // Sensors
    let tcp_sensor_listening_port: &str = "127.0.0.1:8080";
    let tcp_sensor_listener  = TcpListener::bind(tcp_sensor_listening_port).await.unwrap();

    let tcp_server = async move {
        loop {
            let (socket, _) = tcp_sensor_listener.accept().await.unwrap();

            let tx_clone = tx.clone();

            tokio::spawn(async move {
                let mut reader = BufReader::new(socket);
                let mut line = String::new();

                while let Ok(bytes_read) = reader.read_line(&mut line).await {
                    if bytes_read == 0 { break; }

                    if let Ok(sensor_data) = serde_json::from_str::<TemperatureSensorLog>(&line) {

                        if sensor_data.is_valid() {
                            tx_clone.send(sensor_data).await.unwrap();
                        } else {
                            println!("Rejected. Invalid data: {}", line);
                        }
                    }
                    line.clear();
                }
            });
        }
    };

    // Api
    let tcp_app_listening_port: &str = "127.0.0.1:3000";
    let tcp_app_listener  = TcpListener::bind(tcp_app_listening_port).await.unwrap();

    let sensors_path = "/sensors";
    let app = Router::new()
        .route(sensors_path, get(get_measurements))
        .with_state(pool);
    let app_server = axum::serve(tcp_app_listener, app);

    let _ = tokio::join!(app_server, tcp_server);
}
