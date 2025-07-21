use std::thread;

use anyhow::Ok;
use rand::Rng;
use rcrash::Metrics;

fn main() -> anyhow::Result<()> {
    let mut metrics = Metrics::new();

    // start N workers
    for i in 0..10 {
        worker(i, metrics.clone());
    }

    loop {
        thread::sleep(std::time::Duration::from_millis(5000));
        println!("metrics: {:?}", metrics.snapshot());
    }

    Ok(())
}

fn worker(idx: usize, mut metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();

        thread::sleep(std::time::Duration::from_millis(rng.gen_range(500..1000)));
        metrics.inc(format!("call.thread.worker.{idx}"));
    });
}

fn request_worker(mut metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();

        thread::sleep(std::time::Duration::from_millis(rng.gen_range(200..800)));
        let page = rng.gen_range(1..256);
        metrics.inc(format!("req.page.{page}"));
    });
}
