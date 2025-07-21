use anyhow::Ok;
use rcrash::Metrics;

fn main() -> anyhow::Result<()> {
    let mut metrics = Metrics::new();
    for i in 0..100 {
        metrics.inc("req.page.1");
        metrics.inc("req.page.2");
        if i % 2 == 0 {
            metrics.inc("req.page.3");
        }
    }
    for _ in 0..27 {
        metrics.inc("call.thread.worker.1");
    }
    println!("{:?}", metrics.snapshot());
    Ok(())
}
