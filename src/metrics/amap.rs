use anyhow::{Ok, Result};
use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        Self {
            data: Arc::new(
                metric_names
                    .iter()
                    .map(|&name| (name, AtomicI64::new(0)))
                    .collect(),
            ),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let counter = self
            .data
            .get(key.as_ref())
            .ok_or_else(|| anyhow::anyhow!("Key not found"))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn dec(&self, key: impl AsRef<str>) -> Result<()> {
        let counter = self
            .data
            .get(key.as_ref())
            .ok_or_else(|| anyhow::anyhow!("Key not found"))?;
        counter.fetch_sub(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl std::fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (key, value) in self.data.iter() {
            s.push_str(&format!("{}: {}\n", key, value.load(Ordering::Relaxed)));
        }
        writeln!(f, "{}", s)
    }
}
