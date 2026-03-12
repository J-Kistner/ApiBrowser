use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct CachedResponse<T> {
   data: T,
   etag: Option<String>,
   cached_at: i64,
}

pub struct Cache {
   cache_dir: PathBuf,
}

impl Cache {
   pub fn new(event_key: &str) -> Result<Self> {
      let cache_dir = dirs::cache_dir()
         .unwrap_or_else(|| PathBuf::from(".cache"))
         .join("apibrowser")
         .join(event_key);

      fs::create_dir_all(&cache_dir)?;

      Ok(Cache { cache_dir })
   }

   pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<(T, Option<String>)> {
      let path = self.cache_dir.join(format!("{}.json", key));
      if !path.exists() {
         return None;
      }

      let content = fs::read_to_string(&path).ok()?;
      let cached: CachedResponse<T> = serde_json::from_str(&content).ok()?;

      Some((cached.data, cached.etag))
   }

   pub fn set<T: Serialize>(&self, key: &str, data: &T, etag: Option<String>) -> Result<()> {
      let cached = CachedResponse {
         data,
         etag,
         cached_at: chrono::Utc::now().timestamp(),
      };

      let path = self.cache_dir.join(format!("{}.json", key));
      let content = serde_json::to_string_pretty(&cached)?;
      fs::write(&path, content)?;

      Ok(())
   }

   #[allow(dead_code)]
   pub fn clear(&self) -> Result<()> {
      if self.cache_dir.exists() {
         fs::remove_dir_all(&self.cache_dir)?;
         fs::create_dir_all(&self.cache_dir)?;
      }
      Ok(())
   }
}
