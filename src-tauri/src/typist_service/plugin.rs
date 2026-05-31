/*!
 * 航空航天级插件系统
 * 实现 Typst 插件加载和执行功能
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub typst_version: String,
    pub entry_point: String,
    pub permissions: Vec<PluginPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginPermission {
    FileAccess,
    NetworkAccess,
    SystemCommand,
    TypstCompile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContext {
    pub document_id: String,
    pub working_directory: PathBuf,
    pub environment: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResult {
    pub success: bool,
    pub output: String,
    pub errors: Vec<String>,
    pub modified_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginStatus {
    Loaded,
    Unloaded,
    Error(String),
}

#[derive(Clone)]
pub struct Plugin {
    metadata: PluginMetadata,
    status: PluginStatus,
    path: PathBuf,
}

impl Plugin {
    pub fn new(path: PathBuf, metadata: PluginMetadata) -> Self {
        Self {
            metadata,
            status: PluginStatus::Unloaded,
            path,
        }
    }

    pub fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    pub fn status(&self) -> &PluginStatus {
        &self.status
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

pub struct PluginManager {
    plugins: Arc<Mutex<HashMap<String, Plugin>>>,
    plugin_cache: Arc<Mutex<HashMap<String, String>>>,
    sandbox_enabled: bool,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(Mutex::new(HashMap::new())),
            plugin_cache: Arc::new(Mutex::new(HashMap::new())),
            sandbox_enabled: true,
        }
    }

    pub fn with_sandbox(mut self, enabled: bool) -> Self {
        self.sandbox_enabled = enabled;
        self
    }

    /// 从目录加载插件
    pub fn load_plugin_from_directory(&mut self, directory: PathBuf) -> Result<(), String> {
        let manifest_path = directory.join("typst-plugin.toml");

        if !manifest_path.exists() {
            return Err(format!("Plugin manifest not found: {:?}", manifest_path));
        }

        let manifest_content = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;

        let metadata: PluginMetadata = toml::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse manifest: {}", e))?;

        let plugin = Plugin::new(directory.clone(), metadata);
        let plugin_name = plugin.metadata.name.clone();

        let mut plugins = self.plugins.lock().unwrap();
        plugins.insert(plugin_name.clone(), plugin);

        Ok(())
    }

    /// 从文件加载插件
    pub fn load_plugin_from_file(&mut self, path: PathBuf) -> Result<(), String> {
        let metadata_content =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read plugin file: {}", e))?;

        let metadata: PluginMetadata = toml::from_str(&metadata_content)
            .map_err(|e| format!("Failed to parse plugin metadata: {}", e))?;

        let plugin_dir = path.parent().ok_or("Invalid plugin path")?.to_path_buf();

        let plugin = Plugin::new(plugin_dir, metadata);
        let plugin_name = plugin.metadata.name.clone();

        let mut plugins = self.plugins.lock().unwrap();
        plugins.insert(plugin_name.clone(), plugin);

        Ok(())
    }

    /// 卸载插件
    pub fn unload_plugin(&mut self, name: &str) -> Result<(), String> {
        let mut plugins = self.plugins.lock().unwrap();

        if plugins.remove(name).is_some() {
            Ok(())
        } else {
            Err(format!("Plugin '{}' not found", name))
        }
    }

    /// 获取插件
    pub fn get_plugin(&self, name: &str) -> Option<Plugin> {
        let plugins = self.plugins.lock().unwrap();
        plugins.get(name).cloned()
    }

    /// 列出所有插件
    pub fn list_plugins(&self) -> Vec<PluginMetadata> {
        let plugins = self.plugins.lock().unwrap();
        plugins.values().map(|p| p.metadata.clone()).collect()
    }

    /// 执行插件
    pub fn execute_plugin(
        &self,
        name: &str,
        context: PluginContext,
        input: &str,
    ) -> Result<PluginResult, String> {
        let plugins = self.plugins.lock().unwrap();
        let plugin = plugins
            .get(name)
            .ok_or_else(|| format!("Plugin '{}' not found", name))?;

        if self.sandbox_enabled {
            self.execute_plugin_sandboxed(plugin, context, input)
        } else {
            self.execute_plugin_direct(plugin, context, input)
        }
    }

    fn execute_plugin_sandboxed(
        &self,
        plugin: &Plugin,
        context: PluginContext,
        input: &str,
    ) -> Result<PluginResult, String> {
        // 沙箱环境执行（简化实现）
        // 实际实现需要使用安全沙箱如 wasmtime

        let entry_point = plugin.path.join(&plugin.metadata.entry_point);

        if !entry_point.exists() {
            return Err(format!("Plugin entry point not found: {:?}", entry_point));
        }

        // 模拟插件执行
        let output = format!(
            "Plugin '{}' executed in sandbox mode\n\
             Document ID: {}\n\
             Input length: {} bytes",
            plugin.metadata.name,
            context.document_id,
            input.len()
        );

        Ok(PluginResult {
            success: true,
            output,
            errors: Vec::new(),
            modified_content: None,
        })
    }

    fn execute_plugin_direct(
        &self,
        plugin: &Plugin,
        context: PluginContext,
        input: &str,
    ) -> Result<PluginResult, String> {
        // 直接执行（无沙箱）
        let entry_point = plugin.path.join(&plugin.metadata.entry_point);

        if !entry_point.exists() {
            return Err(format!("Plugin entry point not found: {:?}", entry_point));
        }

        // 模拟插件执行
        let output = format!(
            "Plugin '{}' executed in direct mode\n\
             Document ID: {}\n\
             Input length: {} bytes",
            plugin.metadata.name,
            context.document_id,
            input.len()
        );

        Ok(PluginResult {
            success: true,
            output,
            errors: Vec::new(),
            modified_content: None,
        })
    }

    /// 验证插件权限
    pub fn validate_permissions(
        &self,
        name: &str,
        required_permissions: &[PluginPermission],
    ) -> Result<bool, String> {
        let plugins = self.plugins.lock().unwrap();
        let plugin = plugins
            .get(name)
            .ok_or_else(|| format!("Plugin '{}' not found", name))?;

        let has_all = required_permissions
            .iter()
            .all(|perm| plugin.metadata.permissions.contains(perm));

        Ok(has_all)
    }

    /// 获取插件缓存
    pub fn get_cache(&self, key: &str) -> Option<String> {
        let cache = self.plugin_cache.lock().unwrap();
        cache.get(key).cloned()
    }

    /// 设置插件缓存
    pub fn set_cache(&self, key: String, value: String) {
        let mut cache = self.plugin_cache.lock().unwrap();
        cache.insert(key, value);
    }

    /// 清除插件缓存
    pub fn clear_cache(&self) {
        let mut cache = self.plugin_cache.lock().unwrap();
        cache.clear();
    }

    /// 创建插件上下文
    pub fn create_context(document_id: String, working_directory: PathBuf) -> PluginContext {
        let mut environment = HashMap::new();
        environment.insert("TYPST_VERSION".to_string(), "0.12".to_string());
        environment.insert("LOGOS_VERSION".to_string(), "0.1.0".to_string());

        PluginContext {
            document_id,
            working_directory,
            environment,
        }
    }

    /// 批量加载插件
    pub fn load_plugins_from_directory(&mut self, plugin_dir: PathBuf) -> Result<usize, String> {
        if !plugin_dir.exists() {
            return Err(format!("Plugin directory not found: {:?}", plugin_dir));
        }

        let mut count = 0;

        if let Ok(entries) = fs::read_dir(&plugin_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && self.load_plugin_from_directory(path.clone()).is_ok() {
                    count += 1;
                }
            }
        }

        Ok(count)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert!(manager.list_plugins().is_empty());
    }

    #[test]
    fn test_plugin_manager_with_sandbox() {
        let manager = PluginManager::new().with_sandbox(true);
        assert!(manager.sandbox_enabled);
    }

    #[test]
    fn test_create_context() {
        let context = PluginManager::create_context("test-doc".to_string(), PathBuf::from("/tmp"));
        assert_eq!(context.document_id, "test-doc");
        assert!(context.environment.contains_key("TYPST_VERSION"));
    }

    #[test]
    fn test_plugin_cache() {
        let manager = PluginManager::new();
        manager.set_cache("test".to_string(), "value".to_string());

        let value = manager.get_cache("test");
        assert_eq!(value, Some("value".to_string()));

        manager.clear_cache();
        let value = manager.get_cache("test");
        assert!(value.is_none());
    }

    #[test]
    fn test_plugin_metadata_serialization() {
        let metadata = PluginMetadata {
            name: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "Test plugin".to_string(),
            typst_version: "0.12".to_string(),
            entry_point: "main.typ".to_string(),
            permissions: vec![PluginPermission::FileAccess],
        };

        let serialized = toml::to_string(&metadata).unwrap();
        let deserialized: PluginMetadata = toml::from_str(&serialized).unwrap();

        assert_eq!(deserialized.name, "test-plugin");
    }

    #[test]
    fn test_plugin_result_creation() {
        let result = PluginResult {
            success: true,
            output: "Test output".to_string(),
            errors: Vec::new(),
            modified_content: None,
        };

        assert!(result.success);
        assert_eq!(result.output, "Test output");
    }

    #[test]
    fn test_plugin_status_variants() {
        let loaded = PluginStatus::Loaded;
        let _unloaded = PluginStatus::Unloaded;
        let error = PluginStatus::Error("Test error".to_string());

        match loaded {
            PluginStatus::Loaded => assert!(true),
            _ => assert!(false),
        }

        match error {
            PluginStatus::Error(msg) => assert_eq!(msg, "Test error"),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_plugin_creation() {
        let metadata = PluginMetadata {
            name: "test".to_string(),
            version: "1.0".to_string(),
            author: "test".to_string(),
            description: "test".to_string(),
            typst_version: "0.12".to_string(),
            entry_point: "main.typ".to_string(),
            permissions: Vec::new(),
        };

        let plugin = Plugin::new(PathBuf::from("/tmp"), metadata);
        assert_eq!(plugin.metadata.name, "test");
        assert!(matches!(plugin.status(), PluginStatus::Unloaded));
    }
}
