use std::error::Error;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, _rx) = broadcast::channel::<(String, String)>(100); // (username, message)
    println!("Server listening on ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let ws_stream = match accept_async(stream).await {
                Ok(ws) => ws,
                Err(e) => {
                    eprintln!("Error during the websocket handshake: {}", e);
                    return;
                }
            };
            let (mut write, mut read) = ws_stream.split();

            let username = match read.next().await {
                Some(Ok(Message::Text(name))) => name,
                _ => {
                    eprintln!("Failed to read username");
                    return;
                }
            };
            println!("{} has connected", username);

            let write_task = tokio::spawn({
                let username = username.clone(); // Clone username for use in write task
                async move {
                    while let Ok((sender, msg)) = rx.recv().await {
                        if sender != username {
                            if write.send(Message::Text(format!("{}: {}", sender, msg))).await.is_err() {
                                break;
                            }
                        }
                    }
                }
            });

            let read_task = tokio::spawn({
                let tx = tx.clone();
                let username_clone = username.clone(); // Cloning username for use in read task
                async move {
                    while let Some(Ok(msg)) = read.next().await {
                        if let Message::Text(text) = msg {
                            if tx.send((username_clone.clone(), text)).is_err() {
                                eprintln!("Error broadcasting message");
                            }
                        }
                    }
                }
            });

            tokio::select! {
                _ = write_task => (),
                _ = read_task => (),
            }

            println!("{} has disconnected", username);
        });
    }

    Ok(())
}
