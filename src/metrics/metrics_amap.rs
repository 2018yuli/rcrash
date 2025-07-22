use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

// 基本功能，inc/dec/snapshot
#[derive(Debug, Clone)]
pub struct MetricsAmap {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl MetricsAmap {
    pub fn new(metrics_name: &[&'static str]) -> Self {
        // When you call .iter() on a slice &[T], the iterator it returns yields items of type &T.
        let map = metrics_name
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        MetricsAmap {
            data: Arc::new(map),
        }
    }
    pub fn inc(&self, key: &str) -> anyhow::Result<()> {
        let counter = self.data.get(key).ok_or(anyhow::anyhow!("key not found"))?;
        /*
         * Relaxed	无序	不保证可见性    最低
         * Acquire	load	读取共享数据前的同步    中等
         * Release	store	修改共享数据后的发布	中等
         * AcqRel	RMW操作	需要同时获取和释放的原子操作	较高
         * SeqCst	任意	需要严格全局顺序的关键同步点	最高
         * 没有直接提供内置的基于 version 的原子机制，如 Java 中的 AtomicStampedReference 或 AtomicMarkableReference
         */
        counter.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Ok(())
    }

    pub fn dec(&self, key: &str) -> anyhow::Result<()> {
        let counter = self.data.get(key).ok_or(anyhow::anyhow!("key not found"))?;
        counter.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}

impl fmt::Display for MetricsAmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in self.data.iter() {
            let value = value.load(Ordering::Relaxed);
            write!(f, "{}: {}\n", key, value)?;
        }
        Ok(())
    }
}
