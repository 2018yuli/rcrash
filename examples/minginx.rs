use std::{
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use anyhow::Ok;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = resolve_config();
    let config = Arc::new(config);
    info!("Upstream {}", config.upstream_addr);
    info!("Listening on {}", config.listen_addr);

    let listener = TcpListener::bind(&config.listen_addr)?;
    loop {
        let (client, addr) = listener.accept()?;
        let cloned_config = Arc::clone(&config);
        info!("Accept connection from {}", addr);
        tokio::spawn(async move {
            let mut upstream = TcpStream::connect(&cloned_config.upstream_addr);
            info!("New connection");
            // proxy
            proxy(&mut client, &mut upstream).await?;
            Ok(())
        });
    }
}

async fn proxy(&mut client: TcpStream, &mut upstream: TcpStream) -> anyhow::Result<()> {
    let mut client_read = tokio::io::BufReader::new(client);

    Ok(())
}

struct Config {
    listen_addr: String,
    upstream_addr: String,
}

fn resolve_config() -> Config {
    Config {
        listen_addr: "127.0.0.1:8080".to_string(),
        upstream_addr: "127.0.0.1:8081".to_string(),
    }
}
