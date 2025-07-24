// cargo run --example tokio2

use std::{thread, time::Duration};

use anyhow::Ok;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel(32);
    let handler = worker(rx);

    tokio::spawn(async move {
        for i in 0..100 {
            println!("Sending Task {i}");
            tx.send(format!("task {i}")).await?;
        }
        Ok(())
    });
    handler.join().unwrap();
    Ok(())
}

fn worker(mut rx: mpsc::Receiver<String>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while let Some(s) = rx.blocking_recv() {
            let ret = expensive_blocking_task(s);
            println!("result {}", ret);
        }
    })
}

fn expensive_blocking_task(s: String) -> String {
    thread::sleep(Duration::from_secs(1));
    blake3::hash(s.as_bytes()).to_string()
}
