use clap::{Arg, Command};
use env_logger::Env;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::signal;

#[derive(Serialize, Deserialize)]
struct Config {
    address: String,
    port: u16,
}

fn load_config(file_path: &str) -> Config {
    let config_str = fs::read_to_string(file_path).expect("Unable to read config file");
    serde_json::from_str(&config_str).expect("JSON was not well-formatted")
}

async fn handle_client(mut stream: tokio::net::TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => break, // Connection was closed
            Ok(_) => {
                // Echo the data back to the client
                if let Err(e) = stream.write(&buffer).await {
                    return Err(Box::new(e));
                }
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let matches = Command::new("TCP_Server")
        .version("1.0")
        .author("Pranjal Mandavkar(LossstHeaven) <programmer.pranjal@gmail.com>")
        .about("A simple TCP server")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config_file = matches
        .value_of("config")
        .unwrap_or("configuration/config.json");
    let config = load_config(config_file);

    let listener = TcpListener::bind(format!("{}:{}", config.address, config.port)).await?;
    info!(
        "Server listening on http://{}:{}",
        config.address, config.port
    );

    let server = async {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(async move {
                        if let Err(e) = handle_client(stream).await {
                            error!("Client handling error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    };

    tokio::select! {
        _ = server => {},
        _ = signal::ctrl_c() => {
            info!("Shutting down server...");
        },
    }

    Ok(())
}

