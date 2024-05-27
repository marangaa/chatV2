use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use std::error::Error;
use std::io::{self, Write};

pub async fn run() -> Result<(), Box<dyn Error>> {
    let (ws_stream, _) = connect_async("ws://127.0.0.1:8080").await?;
    let (mut write, mut read) = ws_stream.split();

    print!("Enter your username: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();
    write.send(Message::Text(username.clone())).await?;

    let read_task = tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => println!("{}", text),
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    });

    let input_task = tokio::spawn(async move {
        loop {
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Error reading input");
                break;
            }
            if write.send(Message::Text(input.trim().to_string())).await.is_err() {
                eprintln!("Error sending message");
                break;
            }
        }
    });

    input_task.await?;
    read_task.abort();

    Ok(())
}
