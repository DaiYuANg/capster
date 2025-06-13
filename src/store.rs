use crate::config::StoreConfig;
use crate::memory_store::InMemoryStore;
use crate::redis_store::RedisStore;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;

#[async_trait]
pub trait CaptchaStore: Send + Sync {
  async fn set(&self, key: String, value: String);
  async fn get(&self, key: &str) -> Option<String>;
  async fn remove(&self, key: &str);
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub enum StoreBackend {
  Memory,
  Redis,
}

pub async fn create_store(config: StoreConfig) -> Arc<dyn CaptchaStore> {
  let backend = config.backend;
  match backend {
    StoreBackend::Memory => Arc::new(InMemoryStore::new()),
    StoreBackend::Redis => {
      let redis_store = RedisStore::new(config.url.as_ref()).await.unwrap();
      Arc::new(redis_store)
    }
  }
}
