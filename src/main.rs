use anyhow::Result;
use clap::{Arg, Command};
use std::time::Duration;
use tokio::{
    fs,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    time::sleep,
};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("srv")
        .arg(
            Arg::new("html-files")
                .long("html-files")
                .value_name("PATH")
                .help("Path to HTML files")
                .default_value("html"),
        )
        .get_matches();

    let html_path = matches.get_one::<String>("html-files").unwrap().clone();

    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("0.0.0.0:7878").await?;
    info!("Server listening");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                info!("New connection from {}", addr);
                let html_path_clone = html_path.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, html_path_clone).await {
                        error!("Error handling connection from {}: {}", addr, e);
                    }
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream, html_path: String) -> Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();

    let request = match lines.next_line().await? {
        Some(l) => {
            info!("Request is {}", l);
            l
        }
        None => "".to_string(),
    };

    let get = "GET / HTTP/1.1";
    let sleep_request = "GET /sleep HTTP/1.1";

    let (status_line, filename) = if request.starts_with(get) {
        ("HTTP/1.1 200 OK", format!("{html_path}/hello.html"))
    } else if request.starts_with(sleep_request) {
        info!("Processing sleep request");
        sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK", format!("{html_path}/hello.html"))
    } else {
        warn!("Unknown request, returning 404");
        ("HTTP/1.1 404 NOT FOUND", format!("{html_path}/404.html"))
    };

    let contents = fs::read_to_string(&filename).await.unwrap_or_else(|e| {
        error!("Failed to read file {}: {}", filename, e);
        "Error reading file".to_string()
    });

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}
