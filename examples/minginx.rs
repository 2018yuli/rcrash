use std::sync::Arc;

use anyhow::Result;
use tokio::{
    io,
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let config = resolve_config();
    let config = Arc::new(config);
    info!("Upstream {}", config.upstream_addr);
    info!("Listening on {}", config.listen_addr);

    let listener = TcpListener::bind(&config.listen_addr).await?;
    loop {
        let (client, addr) = listener.accept().await?;
        let cloned_config = Arc::clone(&config);
        info!("Accept connection from {}", addr);
        tokio::spawn(async move {
            let upstream = TcpStream::connect(&cloned_config.upstream_addr).await?;
            info!("New connection");
            // proxy
            if let Err(e) = proxy(client, upstream).await {
                warn!("proxy error: {:?}", e);
            }
            Ok::<(), anyhow::Error>(())
        });
    }
}

async fn proxy(mut client: TcpStream, mut upstream: TcpStream) -> anyhow::Result<()> {
    let (mut client_reader, mut client_writer) = client.split();
    let (mut upstream_reader, mut upstream_writer) = upstream.split();

    let client_to_upstream = io::copy(&mut client_reader, &mut upstream_writer);
    let upstream_to_client = io::copy(&mut upstream_reader, &mut client_writer);

    // 用 select 或 join! 都行，这里保留你的 join! 逻辑
    if let (Err(e), Err(e2)) = tokio::join!(client_to_upstream, upstream_to_client) {
        warn!("Error occurred: {:?}, {:?}", e, e2);
    }
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

fn init_tracing() {
    // 优先从环境变量读取，例如：RUST_LOG=debug ./your-bin
    // 没有的话默认 info
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(false) // 不打印 target
        .with_line_number(true) // 打印行号
        .with_file(true) // 打印文件
        .compact() // 更紧凑的输出
        .init();
}
