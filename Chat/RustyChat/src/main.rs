use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel(100);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx_copy = tx.clone();
        let mut rx_copy = tx_copy.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut buf_reader = BufReader::new(reader);
            let mut text = String::new();

            loop {
                tokio::select! {

                    num_bytes = buf_reader.read_line(&mut text) => {
                        let readed = num_bytes.unwrap();

                        if readed == 0 {
                            break;
                        }

                        let format_message = format!("{}: {}", addr, text);
                        let _ = tx_copy.send(format_message);
                        text.clear();
                    }

                    recieved_message = rx_copy.recv() => {
                        let tekst = recieved_message.unwrap();

                        writer.write_all(tekst.as_bytes()).await.unwrap();
                    }
                }
            }
        });
    }
}
