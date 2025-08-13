// // apt-get install redis
// // cargo run --example dredis
// // redis-cli
// use anyhow::{Ok, Result};
// use tokio::io::{self, AsyncWriteExt};
// use tracing::{info, warn};
// use tracing_subscriber::fmt;

// const BUFF_SIZE: usize = 4096;

// #[tokio::main]
// async fn main() -> Result<()> {
//     fmt()
//         .with_target(false) // 不打印 target
//         .with_line_number(true) // 打印行号
//         .with_file(true) // 打印文件
//         .compact() // 更紧凑的输出
//         .init();

//     // build a listener
//     let addr = "127.0.0.1:6379";

//     let listener = tokio::net::TcpListener::bind(addr).await?;
//     info!("Dredis: listening on {}", addr);

//     // accept connections
//     loop {
//         let (stream, raddr) = listener.accept().await?;
//         info!("Dredis: new connection from {}", raddr);
//         tokio::spawn(async move {
//             if let Err(e) = process_redis_connection(stream, raddr).await {
//                 info!("Dredis: connection error {}", e);
//             }
//         });
//     }
// }

// async fn process_redis_connection(
//     mut stream: tokio::net::TcpStream,
//     raddr: std::net::SocketAddr,
// ) -> Result<()> {
//     loop {
//         stream.readable().await?;
//         let mut buf = Vec::with_capacity(BUFF_SIZE);

//         match stream.try_read_buf(&mut buf) {
//             std::result::Result::Ok(0) => break,
//             std::result::Result::Ok(n) => {
//                 info!("read bytes {}", n);
//                 let line = String::from_utf8_lossy(&buf);
//                 info!("{:?}\r\n{}", line, line);
//                 stream.write_all(b"+Ok\r\n").await?;
//             }
//             std::result::Result::Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
//             std::result::Result::Err(e) => return Err(e.into()),
//         }
//     }
//     warn!("Dredis: connection closed by {}", raddr);
//     Ok(())
// }
fn main() {
    println!("")
}