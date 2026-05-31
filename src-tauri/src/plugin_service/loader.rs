use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub entry_point: String,
    pub permissions: Vec<String>,
    pub dependencies: Vec<String>,
    pub api_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub manifest: PluginManifest,
    pub path: String,
    pub enabled: bool,
    pub loaded: bool,
}

pub struct PluginLoader {
    // In production, this would use libloading for dynamic library loading
    plugins: Vec<PluginMetadata>,
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Load a plugin from a directory
    pub fn load_plugin(&mut self, plugin_path: &str) -> Result<PluginMetadata, String> {
        let manifest_path = Path::new(plugin_path).join("manifest.json");

        if !manifest_path.exists() {
            return Err(format!("manifest.json not found in {}", plugin_path));
        }

        let manifest_content = std::fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;

        let manifest: PluginManifest = serde_json::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse manifest: {}", e))?;

        let metadata = PluginMetadata {
            id: format!(
                "{}-{}",
                manifest.name.replace(' ', "_").to_lowercase(),
                manifest.version
            ),
            manifest,
            path: plugin_path.to_string(),
            enabled: true,
            loaded: false,
        };

        // In production, this would load the dynamic library
        // For now, we just mark it as loaded
        let mut loaded_metadata = metadata.clone();
        loaded_metadata.loaded = true;

        self.plugins.push(loaded_metadata);

        Ok(metadata)
    }

    /// Unload a plugin
    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let index = self
            .plugins
            .iter()
            .position(|p| p.id == plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;

        self.plugins.remove(index);
        Ok(())
    }

    /// Get all loaded plugins
    #[allow(dead_code)]
    pub fn get_plugins(&self) -> &[PluginMetadata] {
        &self.plugins
    }

    /// Get a specific plugin
    #[allow(dead_code)]
    pub fn get_plugin(&self, plugin_id: &str) -> Option<&PluginMetadata> {
        self.plugins.iter().find(|p| p.id == plugin_id)
    }

    /// Enable a plugin
    pub fn enable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let plugin = self
            .plugins
            .iter_mut()
            .find(|p| p.id == plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;

        plugin.enabled = true;
        Ok(())
    }

    /// Disable a plugin
    pub fn disable_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        let plugin = self
            .plugins
            .iter_mut()
            .find(|p| p.id == plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;

        plugin.enabled = false;
        Ok(())
    }

    /// Validate plugin manifest
    #[allow(dead_code)]
    pub fn validate_manifest(&self, manifest: &PluginManifest) -> Result<(), String> {
        if manifest.name.is_empty() {
            return Err("Plugin name cannot be empty".to_string());
        }

        if manifest.version.is_empty() {
            return Err("Plugin version cannot be empty".to_string());
        }

        if manifest.entry_point.is_empty() {
            return Err("Plugin entry point cannot be empty".to_string());
        }

        // Check API version compatibility
        if manifest.api_version != "1.0" {
            return Err(format!("Unsupported API version: {}", manifest.api_version));
        }

        Ok(())
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader_creation() {
        let loader = PluginLoader::new();
        assert_eq!(loader.get_plugins().len(), 0);
    }

    #[test]
    fn test_loader_default() {
        let loader = PluginLoader::default();
        assert_eq!(loader.get_plugins().len(), 0);
    }

    #[test]
    fn test_validate_manifest() {
        let loader = PluginLoader::new();
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };

        assert!(loader.validate_manifest(&manifest).is_ok());
    }

    #[test]
    fn test_plugin_manifest_creation() {
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };
        assert_eq!(manifest.name, "Test");
        assert_eq!(manifest.version, "1.0");
    }

    #[test]
    fn test_plugin_manifest_serialization() {
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };
        let json = serde_json::to_string(&manifest);
        assert!(json.is_ok());
    }

    #[test]
    fn test_plugin_manifest_deserialization() {
        let json = r#"{
            "name": "Test",
            "version": "1.0",
            "description": "Test plugin",
            "author": "Test",
            "entry_point": "libtest.so",
            "permissions": [],
            "dependencies": [],
            "api_version": "1.0"
        }"#;
        let manifest: Result<PluginManifest, _> = serde_json::from_str(json);
        assert!(manifest.is_ok());
    }

    #[test]
    fn test_plugin_metadata_creation() {
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };
        let metadata = PluginMetadata {
            id: "test-1.0".to_string(),
            manifest,
            path: "/path/to/plugin".to_string(),
            enabled: true,
            loaded: false,
        };
        assert_eq!(metadata.id, "test-1.0");
        assert!(metadata.enabled);
    }

    #[test]
    fn test_plugin_metadata_serialization() {
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };
        let metadata = PluginMetadata {
            id: "test-1.0".to_string(),
            manifest,
            path: "/path/to/plugin".to_string(),
            enabled: true,
            loaded: false,
        };
        let json = serde_json::to_string(&metadata);
        assert!(json.is_ok());
    }

    #[test]
    fn test_plugin_metadata_deserialization() {
        let json = r#"{
            "id": "test-1.0",
            "manifest": {
                "name": "Test",
                "version": "1.0",
                "description": "Test plugin",
                "author": "Test",
                "entry_point": "libtest.so",
                "permissions": [],
                "dependencies": [],
                "api_version": "1.0"
            },
            "path": "/path/to/plugin",
            "enabled": true,
            "loaded": false
        }"#;
        let metadata: Result<PluginMetadata, _> = serde_json::from_str(json);
        assert!(metadata.is_ok());
    }

    #[test]
    fn test_load_plugin_nonexistent() {
        let mut loader = PluginLoader::new();
        let result = loader.load_plugin("/nonexistent/path");
        assert!(result.is_err());
    }

    #[test]
    fn test_unload_plugin_nonexistent() {
        let mut loader = PluginLoader::new();
        let result = loader.unload_plugin("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_plugin_nonexistent() {
        let loader = PluginLoader::new();
        let result = loader.get_plugin("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_enable_plugin_nonexistent() {
        let mut loader = PluginLoader::new();
        let result = loader.enable_plugin("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_disable_plugin_nonexistent() {
        let mut loader = PluginLoader::new();
        let result = loader.disable_plugin("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_manifest_empty_name() {
        let loader = PluginLoader::new();
        let manifest = PluginManifest {
            name: "".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };
        let result = loader.validate_manifest(&manifest);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_manifest_empty_version() {
        let loader = PluginLoader::new();
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };
        let result = loader.validate_manifest(&manifest);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_manifest_empty_entry_point() {
        let loader = PluginLoader::new();
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };
        let result = loader.validate_manifest(&manifest);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_manifest_unsupported_api_version() {
        let loader = PluginLoader::new();
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "2.0".to_string(),
        };
        let result = loader.validate_manifest(&manifest);
        assert!(result.is_err());
    }

    #[test]
    fn test_plugin_manifest_with_permissions() {
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec!["read".to_string(), "write".to_string()],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };
        assert_eq!(manifest.permissions.len(), 2);
    }

    #[test]
    fn test_plugin_manifest_with_dependencies() {
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec!["dep1".to_string(), "dep2".to_string()],
            api_version: "1.0".to_string(),
        };
        assert_eq!(manifest.dependencies.len(), 2);
    }

    #[test]
    fn test_plugin_metadata_disabled() {
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };
        let metadata = PluginMetadata {
            id: "test-1.0".to_string(),
            manifest,
            path: "/path/to/plugin".to_string(),
            enabled: false,
            loaded: false,
        };
        assert!(!metadata.enabled);
    }

    #[test]
    fn test_plugin_metadata_loaded() {
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };
        let metadata = PluginMetadata {
            id: "test-1.0".to_string(),
            manifest,
            path: "/path/to/plugin".to_string(),
            enabled: true,
            loaded: true,
        };
        assert!(metadata.loaded);
    }

    #[test]
    fn test_get_plugins_empty() {
        let loader = PluginLoader::new();
        let plugins = loader.get_plugins();
        assert!(plugins.is_empty());
    }

    #[test]
    fn test_plugin_manifest_all_fields() {
        let manifest = PluginManifest {
            name: "My Plugin".to_string(),
            version: "2.1.0".to_string(),
            description: "A comprehensive plugin".to_string(),
            author: "Developer".to_string(),
            entry_point: "libmyplugin.dylib".to_string(),
            permissions: vec!["network".to_string(), "filesystem".to_string()],
            dependencies: vec!["core".to_string()],
            api_version: "1.0".to_string(),
        };
        assert_eq!(manifest.name, "My Plugin");
        assert_eq!(manifest.version, "2.1.0");
        assert_eq!(manifest.author, "Developer");
    }

    #[test]
    fn test_validate_manifest_valid_api_version() {
        let loader = PluginLoader::new();
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };
        let result = loader.validate_manifest(&manifest);
        assert!(result.is_ok());
    }

    #[test]
    fn test_plugin_manifest_empty_permissions() {
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };
        assert!(manifest.permissions.is_empty());
    }

    #[test]
    fn test_plugin_manifest_empty_dependencies() {
        let manifest = PluginManifest {
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test".to_string(),
            entry_point: "libtest.so".to_string(),
            permissions: vec![],
            dependencies: vec![],
            api_version: "1.0".to_string(),
        };
        assert!(manifest.dependencies.is_empty());
    }
}
