use std::{fmt, sync::Arc};

use dashmap::DashMap;

// 基本功能，inc/dec/snapshot
#[derive(Debug, Clone)]
pub struct MetricsDashMap {
    data: Arc<DashMap<String, i64>>,
}

impl MetricsDashMap {
    pub fn new() -> Self {
        MetricsDashMap {
            data: Arc::new(DashMap::new()),
        }
    }
    pub fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }
}

impl fmt::Display for MetricsDashMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            write!(f, "{}: {}\n", entry.key(), *entry.value())?;
        }
        Ok(())
    }
}
