use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server is running. Listening on port 8080");

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(drone_stream) => {
                println!("New access.");
                let mut drone_stream_copy = drone_stream.try_clone().unwrap();
                thread::spawn(move || {
                    let stream_reader = BufReader::new(drone_stream);
                    let mut message_send_counter: i32 = 0;

                    for line_result in stream_reader.lines() {
                        if let Ok(received_text) = line_result {

                            println!("Recieved data: {}", received_text);
                            message_send_counter += 1;

                            if let Err(e) = writeln!(drone_stream_copy, "ACK: Server recieved {} messages.", message_send_counter) {
                                println!("Transmition error: {}", e);
                                break;
                            }
                        } else {
                            println!("Data read error.");
                            break;
                        }
                    }
                    println!("Machine disconnected.");
                });
            }
            Err(e) => {
                println!("ERROR: {}", e);
            }
        }
    }
}