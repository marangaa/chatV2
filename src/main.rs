use std::env;
use std::error::Error;

mod server;
mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [server|client]", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "server" => server::run().await,
        "client" => client::run().await,
        _ => {
            eprintln!("Invalid option. Use 'server' or 'client'.");
            std::process::exit(1);
        }
    }
}
