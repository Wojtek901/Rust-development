use rand::Rng;
use serde_json::json;
use std::io::Write;

fn main(){
    let data_file = std::fs::File::create("data.txt").unwrap();
    let mut writer = std::io::BufWriter::new(data_file);
    let devices = ["SENSOR-A", "SENSOR-B", "SENSOR-X"];
    let mut rangom_gen = rand::thread_rng();
    let logs_to_generate = 5_000_000;

    for i in 1..logs_to_generate{
        let device_number = rangom_gen.gen_range(0..3);
        let device_id = devices[device_number];
        let measurement = rangom_gen.gen_range(-20.0..30.0);
        let json_string = json!({"device_id": device_id, "measurement": measurement}).to_string() + "\n";
        writer.write_all(json_string.as_bytes()).unwrap();
        if i % 1_000_000 == 0{
            println!("Generated {} lines out of {}", i, logs_to_generate);
        }
    }
}