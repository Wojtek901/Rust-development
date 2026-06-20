use clap::Parser;
use anyhow::Context;
use std::fs::read_to_string;
use rayon::prelude::*;
use serde;

#[derive(Parser)]
#[command(name = "LogCruncher", about="Powerfull tool for logs analys.")]
struct CliArgs{
    file_path: String,

    #[arg(short, long)]
    device: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct LogData{
    device_id: String,
    measurement: f64,
}

fn get_wrong_path_info(path: &str) -> String{
    format!("File <{}> does not exists, or is in different directory", path)
}

fn main() -> anyhow::Result<()>{
    let args = CliArgs::parse();
    let path = args.file_path;
    let device_id = args.device;

    let file_content = read_to_string(&path).context(get_wrong_path_info(&path))?;

    let (count, total_temperature) = file_content
        .par_lines()
        .filter_map(|line| {
            serde_json::from_str::<LogData>(line).ok()
        })
        .filter(|log| {
            log.device_id == device_id
        })
        .map(|log| {
            (1usize, log.measurement)
        })
        .reduce(
            || (0usize, 0.0),
            |thread1, thread2| {
                (
                    thread1.0 + thread2.0,
                    thread1.1 + thread2.1
                )
            }
        );

    if count > 0 {
        let mean_temperature = total_temperature / (count as f64);
        println!("Found logs: {}", count);
        println!("Mean temperature: {:.2}°C", mean_temperature);
    } else {
        println!("No data for device: {}", device_id);
    }

    Ok(())
}