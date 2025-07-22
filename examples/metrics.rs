use anyhow::Ok;
use rcrash::Metrics;

fn main() -> anyhow::Result<()> {
    let metrics = Metrics::new();
    for i in 0..100 {
        metrics.inc("req.page.1").unwrap();
        metrics.inc("req.page.2").unwrap();
        if i % 2 == 0 {
            metrics.inc("req.page.3").unwrap();
        }
    }
    for _ in 0..27 {
        metrics.inc("call.thread.worker.1").unwrap();
    }
    println!("{:?}", metrics.snapshot());
    Ok(())
}
