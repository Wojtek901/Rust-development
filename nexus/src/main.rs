use axum::{extract::State, routing::get, Json, Router};
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::fs::File;
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::mpsc;

#[derive(serde::Serialize, serde::Deserialize, Clone, sqlx::FromRow)]
struct TemperatureSensorLog {
    sensor_id: String,
    temperature: f64,
}

trait Validatable {
    fn is_valid(&self) -> bool;
}

impl Validatable for TemperatureSensorLog {
    fn is_valid(&self) -> bool {
        self.temperature >= -273.15
    }
}

async fn init_db(db_name: &str) -> Pool<Sqlite> {
    if !Path::new(db_name).exists() {
        File::create(db_name).unwrap();
    }
    let db_url = format!("sqlite://{}", db_name);
    let pool = SqlitePool::connect(&db_url).await.unwrap();
    let db_create_command =
        "CREATE TABLE IF NOT EXISTS sensor_logs (sensor_id TEXT, temperature REAL)";
    sqlx::query(db_create_command).execute(&pool).await.unwrap();
    pool
}

async fn get_measurements(State(pool): State<SqlitePool>) -> Json<Vec<TemperatureSensorLog>> {
    let sql_query = "SELECT * FROM sensor_logs";
    let db_data = sqlx::query_as::<_, TemperatureSensorLog>(sql_query)
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(db_data)
}

async fn run_background_worker(
    mut rx: mpsc::Receiver<TemperatureSensorLog>,
    pool: SqlitePool,
) {
    while let Some(message) = rx.recv().await {
        sqlx::query("INSERT INTO sensor_logs (sensor_id, temperature) VALUES (?, ?)")
            .bind(message.sensor_id)
            .bind(message.temperature)
            .execute(&pool)
            .await
            .unwrap();

    }
}

async fn run_tcp_server(port: &str, tx: mpsc::Sender<TemperatureSensorLog>) {
    let listener = TcpListener::bind(port).await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            let mut reader = BufReader::new(socket);
            let mut line = String::new();

            while let Ok(bytes_read) = reader.read_line(&mut line).await {
                if bytes_read == 0 {
                    break;
                }

                if let Ok(sensor_data) = serde_json::from_str::<TemperatureSensorLog>(&line) {
                    if sensor_data.is_valid() {
                        tx_clone.send(sensor_data).await.unwrap();
                    } else {
                        println!("Invalid data: {}", line);
                    }
                }
                line.clear();
            }
        });
    }
}


#[tokio::main]
async fn main() {
    // db init
    let db_name = "Sensors.db";
    let pool = init_db(db_name).await;
    let (tx, rx) = mpsc::channel::<TemperatureSensorLog>(100);

    // background worker
    tokio::spawn(run_background_worker(rx, pool.clone()));

    // app
    let tcp_app_listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let app = Router::new()
        .route("/sensors", get(get_measurements))
        .with_state(pool);
    let axum_server = axum::serve(tcp_app_listener, app);

    // sensors tcp
    let tcp_server = run_tcp_server("127.0.0.1:8080", tx);

    // run
    let _ = tokio::join!(axum_server, tcp_server);
}