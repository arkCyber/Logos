use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// 增量编译缓存条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub hash: String,
    pub timestamp: DateTime<Utc>,
    pub dependencies: Vec<String>,
    pub compiled_output: Vec<u8>,
}

/// 增量编译配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalConfig {
    pub enabled: bool,
    pub cache_dir: PathBuf,
    pub max_cache_size_mb: u64,
    pub cache_ttl_seconds: u64,
    pub parallel_compilation: bool,
    pub max_parallel_jobs: usize,
    pub hot_reload: bool,
}

impl Default for IncrementalConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_dir: PathBuf::from(".typst_cache"),
            max_cache_size_mb: 1024,  // 1GB
            cache_ttl_seconds: 86400, // 24 hours
            parallel_compilation: true,
            max_parallel_jobs: 4,
            hot_reload: true,
        }
    }
}

/// 增量编译管理器
pub struct IncrementalCompiler {
    config: IncrementalConfig,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl IncrementalCompiler {
    pub fn new(config: IncrementalConfig) -> Self {
        Self {
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(IncrementalConfig::default())
    }

    /// 计算内容的哈希值
    pub fn compute_hash(&self, content: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// 检查缓存是否有效
    pub fn is_cache_valid(&self, key: &str, current_hash: &str) -> bool {
        if let Ok(cache) = self.cache.read() {
            if let Some(entry) = cache.get(key) {
                // 检查哈希是否匹配
                if entry.hash != current_hash {
                    return false;
                }

                // 检查是否过期
                let now = Utc::now();
                let elapsed = now.signed_duration_since(entry.timestamp);
                if elapsed.num_seconds() > self.config.cache_ttl_seconds as i64 {
                    return false;
                }

                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// 获取缓存的编译结果
    pub fn get_cached(&self, key: &str) -> Option<CacheEntry> {
        if let Ok(cache) = self.cache.read() {
            cache.get(key).cloned()
        } else {
            None
        }
    }

    /// 更新缓存
    pub fn update_cache(
        &self,
        key: String,
        hash: String,
        dependencies: Vec<String>,
        output: Vec<u8>,
    ) {
        if let Ok(mut cache) = self.cache.write() {
            let entry = CacheEntry {
                hash,
                timestamp: Utc::now(),
                dependencies,
                compiled_output: output,
            };
            cache.insert(key, entry);
        }
    }

    /// 清除过期缓存
    pub fn clear_expired(&self) {
        if let Ok(mut cache) = self.cache.write() {
            let now = Utc::now();
            cache.retain(|_, entry| {
                let elapsed = now.signed_duration_since(entry.timestamp);
                elapsed.num_seconds() <= self.config.cache_ttl_seconds as i64
            });
        }
    }

    /// 清除所有缓存
    pub fn clear_all(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }

    /// 获取缓存大小（字节）
    pub fn get_cache_size(&self) -> u64 {
        if let Ok(cache) = self.cache.read() {
            cache
                .values()
                .map(|entry| entry.compiled_output.len() as u64)
                .sum()
        } else {
            0
        }
    }

    /// 如果缓存超过限制，清理最旧的条目
    pub fn enforce_cache_limit(&self) {
        let current_size = self.get_cache_size();
        let max_size = self.config.max_cache_size_mb * 1024 * 1024;

        if current_size > max_size {
            if let Ok(mut cache) = self.cache.write() {
                // 按时间戳排序，删除最旧的条目
                let mut entries: Vec<_> = cache.iter().collect();
                entries.sort_by(|a, b| a.1.timestamp.cmp(&b.1.timestamp));

                let mut size = current_size;
                let mut keys_to_remove = Vec::new();

                for (key, _) in entries {
                    if size <= max_size {
                        break;
                    }
                    if let Some(entry) = cache.get(key) {
                        let entry_size = entry.compiled_output.len() as u64;
                        keys_to_remove.push(key.clone());
                        size -= entry_size;
                    }
                }

                for key in keys_to_remove {
                    cache.remove(&key);
                }
            }
        }
    }

    /// 检查依赖是否变化
    pub fn dependencies_changed(&self, key: &str, current_deps: &[String]) -> bool {
        if let Ok(cache) = self.cache.read() {
            if let Some(entry) = cache.get(key) {
                entry.dependencies != current_deps
            } else {
                true
            }
        } else {
            true
        }
    }

    /// 并行编译多个文档
    pub fn parallel_compile<F>(&self, tasks: Vec<(String, String)>, compile_fn: F) -> Vec<Result<Vec<u8>, String>>
    where
        F: Fn(String) -> Result<Vec<u8>, String> + Send + Sync + Clone + 'static,
    {
        if !self.config.parallel_compilation {
            // 串行编译
            return tasks
                .into_iter()
                .map(|(key, content)| {
                    let hash = self.compute_hash(&content);
                    if let Some(cached) = self.get_cached(&key) {
                        if cached.hash == hash {
                            return Ok(cached.compiled_output);
                        }
                    }
                    compile_fn(content)
                })
                .collect();
        }

        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();
        let cache = Arc::clone(&self.cache);
        let max_jobs = self.config.max_parallel_jobs;

        // 分批处理任务
        let chunks: Vec<_> = tasks.chunks(max_jobs).map(|c| c.to_vec()).collect();

        for chunk in chunks {
            let tx_clone = tx.clone();
            let cache_clone = Arc::clone(&cache);
            let compile_fn_clone = compile_fn.clone();
            let chunk_tasks: Vec<_> = chunk.into_iter().collect();

            thread::spawn(move || {
                for (key, content) in chunk_tasks {
                    let hash = {
                        use sha2::{Digest, Sha256};
                        let mut hasher = Sha256::new();
                        hasher.update(content.as_bytes());
                        format!("{:x}", hasher.finalize())
                    };

                    let result = if let Ok(cache) = cache_clone.read() {
                        if let Some(cached) = cache.get(&key) {
                            if cached.hash == hash {
                                Ok(cached.compiled_output.clone())
                            } else {
                                compile_fn_clone(content)
                            }
                        } else {
                            compile_fn_clone(content)
                        }
                    } else {
                        compile_fn_clone(content)
                    };

                    let _ = tx_clone.send((key, result));
                }
            });
        }

        drop(tx);

        let mut results = HashMap::new();
        for (key, result) in rx {
            results.insert(key, result);
        }

        // 按原始顺序返回结果
        tasks
            .iter()
            .map(|(key, _)| results.get(key).cloned().unwrap_or(Err("Compilation failed".to_string())))
            .collect()
    }

    /// 热重载检测
    pub fn should_hot_reload(&self, key: &str, current_hash: &str) -> bool {
        if !self.config.hot_reload {
            return false;
        }
        !self.is_cache_valid(key, current_hash)
    }
}

impl Default for IncrementalCompiler {
    fn default() -> Self {
        Self::with_default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incremental_config_default() {
        let config = IncrementalConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_cache_size_mb, 1024);
        assert_eq!(config.cache_ttl_seconds, 86400);
    }

    #[test]
    fn test_compute_hash() {
        let compiler = IncrementalCompiler::with_default_config();
        let hash1 = compiler.compute_hash("test content");
        let hash2 = compiler.compute_hash("test content");
        let hash3 = compiler.compute_hash("different content");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_cache_valid() {
        let mut compiler = IncrementalCompiler::with_default_config();
        let key = "test_key".to_string();
        let hash = compiler.compute_hash("test content");

        // 初始状态没有缓存
        assert!(!compiler.is_cache_valid(&key, &hash));

        // 添加缓存
        compiler.update_cache(key.clone(), hash.clone(), vec![], vec![1, 2, 3]);

        // 现在缓存应该有效
        assert!(compiler.is_cache_valid(&key, &hash));

        // 不同的哈希应该无效
        let different_hash = compiler.compute_hash("different");
        assert!(!compiler.is_cache_valid(&key, &different_hash));
    }

    #[test]
    fn test_get_cached() {
        let mut compiler = IncrementalCompiler::with_default_config();
        let key = "test_key".to_string();
        let hash = compiler.compute_hash("test content");

        // 初始状态没有缓存
        assert!(compiler.get_cached(&key).is_none());

        // 添加缓存
        compiler.update_cache(key.clone(), hash, vec![], vec![1, 2, 3]);

        // 现在应该能获取到缓存
        assert!(compiler.get_cached(&key).is_some());
    }

    #[test]
    fn test_update_cache() {
        let mut compiler = IncrementalCompiler::with_default_config();
        let key = "test_key".to_string();
        let hash = compiler.compute_hash("test content");

        compiler.update_cache(key.clone(), hash, vec!["dep1".to_string()], vec![1, 2, 3]);

        let entry = compiler.get_cached(&key).unwrap();
        assert_eq!(entry.dependencies.len(), 1);
        assert_eq!(entry.compiled_output, vec![1, 2, 3]);
    }

    #[test]
    fn test_clear_all() {
        let mut compiler = IncrementalCompiler::with_default_config();
        let key = "test_key".to_string();
        let hash = compiler.compute_hash("test content");

        compiler.update_cache(key.clone(), hash, vec![], vec![1, 2, 3]);
        assert!(compiler.get_cached(&key).is_some());

        compiler.clear_all();
        assert!(compiler.get_cached(&key).is_none());
    }

    #[test]
    fn test_get_cache_size() {
        let mut compiler = IncrementalCompiler::with_default_config();
        let key1 = "key1".to_string();
        let key2 = "key2".to_string();
        let hash = compiler.compute_hash("test");

        compiler.update_cache(key1, hash.clone(), vec![], vec![1, 2, 3]);
        compiler.update_cache(key2, hash, vec![], vec![4, 5, 6, 7]);

        let size = compiler.get_cache_size();
        assert_eq!(size, 7);
    }

    #[test]
    fn test_dependencies_changed() {
        let mut compiler = IncrementalCompiler::with_default_config();
        let key = "test_key".to_string();
        let hash = compiler.compute_hash("test");

        // 初始状态依赖已变化
        assert!(compiler.dependencies_changed(&key, &["dep1".to_string()]));

        // 添加缓存
        compiler.update_cache(key.clone(), hash, vec!["dep1".to_string()], vec![1, 2, 3]);

        // 相同依赖，未变化
        assert!(!compiler.dependencies_changed(&key, &["dep1".to_string()]));

        // 不同依赖，已变化
        assert!(compiler.dependencies_changed(&key, &["dep2".to_string()]));
    }

    #[test]
    fn test_enforce_cache_limit() {
        let mut config = IncrementalConfig::default();
        config.max_cache_size_mb = 0; // 设置为0以强制清理
        let mut compiler = IncrementalCompiler::new(config);

        let key = "test_key".to_string();
        let hash = compiler.compute_hash("test");

        compiler.update_cache(key.clone(), hash, vec![], vec![1, 2, 3]);

        compiler.enforce_cache_limit();

        // 缓存应该被清理
        assert!(compiler.get_cached(&key).is_none());
    }
}
