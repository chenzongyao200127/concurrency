// metrics data structure
// 基本功能：inc/dec/snapshot
use anyhow::{anyhow, Result};
use core::fmt;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Debug, Default, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        self.data
            .read()
            .map_err(|e| anyhow!(e.to_string()))
            .map(|data| data.clone())
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.data.read().map_err(|_| fmt::Error)?;
        for (key, value) in data.iter() {
            write!(f, "{}: {}, ", key, value)?;
        }
        Ok(())
    }
}
