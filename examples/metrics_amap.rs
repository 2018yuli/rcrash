use std::{thread, time::Duration};

use anyhow::Ok;
use rand::Rng;
use rcrash::MetricsAmap;

fn main() -> anyhow::Result<()> {
    let metrics = MetricsAmap::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "req.page.0",
        "req.page.1",
    ]);

    // start N workers
    for idx in 0..2 {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..2 {
        request_worker(metrics.clone())?;
    }

    for _ in 0..5 {
        thread::sleep(std::time::Duration::from_millis(5000));
        println!("metrics: \r\n{}", metrics.clone());
    }

    Ok(())
}

fn task_worker(idx: usize, metrics: MetricsAmap) -> anyhow::Result<()> {
    thread::spawn(move || {
        loop {
            // do long term stuff
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(800..5000)));
            metrics.inc(&format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });
    Ok(())
}

fn request_worker(metrics: MetricsAmap) -> anyhow::Result<()> {
    thread::spawn(move || {
        loop {
            // process requests
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(0..2);
            metrics.inc(&format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });

    Ok(())
}
