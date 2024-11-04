use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
};

use crate::{macro_input::MacroOptions, AutofillError};

#[derive(serde::Serialize, serde::Deserialize)]
struct CacheEntry {
    hash: String,
    content: String,
    timestamp: std::time::SystemTime,
    rust_version: String,
    options: MacroOptions,
}

pub struct AutofillCache {
    cache_dir: PathBuf,
}

impl AutofillCache {
    pub fn new() -> std::io::Result<Self> {
        let target_dir = std::env::var("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("target"));

        let cache_dir = target_dir.join("autofill");
        std::fs::create_dir_all(&cache_dir)?;

        Ok(Self { cache_dir })
    }

    pub fn get_cached(&self, input: &str, options: &MacroOptions) -> Option<String> {
        let hash = Self::compute_hash(input, options);
        let cache_path = self.get_cache_path(&hash);

        std::fs::read_to_string(&cache_path)
            .ok()
            .and_then(|content| serde_json::from_str::<CacheEntry>(&content).ok())
            .and_then(|entry| {
                if entry.hash == hash
                    && entry.rust_version == rustc_version::version().unwrap().to_string()
                    && &entry.options == options
                {
                    Some(entry.content)
                } else {
                    None
                }
            })
    }

    pub fn store(
        &self,
        input: &str,
        options: MacroOptions,
        content: String,
    ) -> Result<(), AutofillError> {
        let hash = Self::compute_hash(input, &options);
        let cache_path = self.get_cache_path(&hash);

        if let Some(parent) = cache_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let entry = CacheEntry {
            hash,
            content,
            options,
            timestamp: std::time::SystemTime::now(),
            rust_version: rustc_version::version().unwrap().to_string(),
        };

        std::fs::write(cache_path, serde_json::to_string_pretty(&entry)?)?;

        Ok(())
    }

    fn compute_hash(content: &str, options: &MacroOptions) -> String {
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        options.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }

    fn get_cache_path(&self, hash: &str) -> PathBuf {
        self.cache_dir.join(format!("cache_{}.json", hash))
    }
}
