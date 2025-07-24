// cargo run --example tokio2

use std::{thread, time::Duration};

use tokio::{fs, runtime::Builder, time::sleep};

fn main() {
    let handle = thread::spawn(|| {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        runtime.spawn(async {
            println!("future1");
            let content = fs::read_to_string("Cargo.toml").await.unwrap();
            println!("content length: {}", content.len());
        });

        runtime.spawn(async {
            println!("future2");
            let ret = expensive_blocking_task("Future2".to_string());
            println!("result: {}", ret);
        });

        runtime.block_on(async {
            println!("future3");
            // thread::sleep(Duration::from_secs(2)); 只会输出 future3
            //  |-因为 thread::sleep 会阻塞当前线程，而 runtime 是单线程的
            //  |-runtime.spawn 创建的 future 会在 EventLoop 的队列中排队
            sleep(Duration::from_secs(2)).await;
        });
    });
    handle.join().unwrap();
}

fn expensive_blocking_task(s: String) -> String {
    thread::sleep(Duration::from_secs(1));
    blake3::hash(s.as_bytes()).to_string()
}
