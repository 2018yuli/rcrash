use std::thread;

use anyhow::Ok;
use rand::Rng;
use rcrash::Metrics;

fn main() -> anyhow::Result<()> {
    let metrics = Metrics::new();

    // start N workers
    for idx in 0..10 {
        worker(idx, metrics.clone());
    }

    for _ in 0..5 {
        request_worker(metrics.clone());
    }

    for _ in 0..5 {
        thread::sleep(std::time::Duration::from_millis(5000));
        println!("metrics: {:?}", metrics.snapshot());
    }

    Ok(())
}

fn worker(idx: usize, metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();

        thread::sleep(std::time::Duration::from_millis(rng.gen_range(500..1000)));
        metrics.inc(format!("call.thread.worker.{idx}")).unwrap();
    });
}

fn request_worker(metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();

        thread::sleep(std::time::Duration::from_millis(rng.gen_range(200..800)));
        let page = rng.gen_range(1..5);
        metrics.inc(format!("req.page.{page}")).unwrap();
    });
}
