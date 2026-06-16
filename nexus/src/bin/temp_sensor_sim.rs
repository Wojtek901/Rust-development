use tokio::time::{sleep, Duration};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use rand::Rng;

#[derive(serde::Serialize)]
struct TemperatureMessage{
    sensor_id: String,
    temperature: f64,
}

#[tokio::main]
async fn main(){
    let sent_port: &str = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(sent_port).await.unwrap();

    let mut rng = rand::thread_rng();

    let sensor_id: String = String::from("SENSOR-1-TEMP");
    let mut message_to_send = TemperatureMessage{
        sensor_id: sensor_id,
        temperature: 0.0,
    };

    loop{
        let time_interval: u64 = rng.gen_range(100..2000);
        let temperature: f64 = rng.gen_range(-30.0..50.0);
        message_to_send.temperature = temperature;

        let mut message_to_send_json = serde_json::to_string(&message_to_send).unwrap();
        message_to_send_json.push('\n');

        println!("Temp_sensor_sim: Sending message: {}", message_to_send_json);

        stream.write_all(message_to_send_json.as_bytes()).await.unwrap();

        sleep(Duration::from_millis(time_interval)).await;
    }
}
