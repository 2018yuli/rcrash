use std::{thread, time::Duration};

use anyhow::Ok;
use rand::Rng;
use rcrash::Metrics;

fn main() -> anyhow::Result<()> {
    let metrics = Metrics::new();

    // start N workers
    for idx in 0..10 {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..5 {
        request_worker(metrics.clone())?;
    }

    for _ in 0..5 {
        thread::sleep(std::time::Duration::from_millis(5000));
        println!("metrics: {}", metrics);
    }

    Ok(())
}

fn task_worker(idx: usize, metrics: Metrics) -> anyhow::Result<()> {
    thread::spawn(move || {
        loop {
            // do long term stuff
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(800..5000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });
    Ok(())
}

fn request_worker(metrics: Metrics) -> anyhow::Result<()> {
    thread::spawn(move || {
        loop {
            // process requests
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..5);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });

    Ok(())
}
