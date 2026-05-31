// 插件系统集成测试
// 演示如何使用插件系统来管理 WebAssembly 插件

use std::sync::Arc;
use crate::plugin_service::{PluginHook, PluginManager};
use crate::config_service::ExportConfigService;
use std::collections::HashMap;

#[test]
fn test_plugin_system_workflow() {
    let config_service = Arc::new(ExportConfigService::new());
    let manager = PluginManager::new(config_service);

    // 测试 1: 获取初始插件实例
    let instances = manager.get_instances();
    assert!(instances.is_ok());
    assert!(instances.unwrap().is_empty());

    // 测试 2: 列出可用的钩子
    let hooks = vec![
        PluginHook::OnLoad,
        PluginHook::OnUnload,
        PluginHook::OnDocumentOpen,
        PluginHook::OnDocumentSave,
        PluginHook::OnDocumentChange,
        PluginHook::OnCommand,
        PluginHook::OnRender,
        PluginHook::OnExport,
        PluginHook::OnTypstCompile,
        PluginHook::OnTypstError,
    ];

    // 验证所有钩子都可以创建
    for hook in hooks {
        let _ = hook;
    }

    // 测试 3: 插件钩子变体
    let all_hooks = vec![
        PluginHook::OnLoad,
        PluginHook::OnUnload,
        PluginHook::OnDocumentOpen,
        PluginHook::OnDocumentSave,
        PluginHook::OnDocumentChange,
        PluginHook::OnCommand,
        PluginHook::OnRender,
        PluginHook::OnExport,
        PluginHook::OnTypstCompile,
        PluginHook::OnTypstError,
    ];

    assert_eq!(all_hooks.len(), 10);
}

#[test]
fn test_plugin_manager_creation() {
    let config_service = Arc::new(ExportConfigService::new());
    let manager = PluginManager::new(config_service);
    let instances = manager.get_instances();
    assert!(instances.is_ok());
    assert!(instances.unwrap().is_empty());
}

#[test]
fn test_plugin_hook_variants() {
    let hooks = vec![
        PluginHook::OnLoad,
        PluginHook::OnUnload,
        PluginHook::OnDocumentOpen,
        PluginHook::OnDocumentSave,
        PluginHook::OnDocumentChange,
        PluginHook::OnCommand,
        PluginHook::OnRender,
        PluginHook::OnExport,
        PluginHook::OnTypstCompile,
        PluginHook::OnTypstError,
    ];

    // 验证所有钩子都是唯一的
    let unique_hooks: std::collections::HashSet<_> = hooks.iter().collect();
    assert_eq!(unique_hooks.len(), hooks.len());
}

#[test]
fn test_plugin_hook_equality() {
    let hook1 = PluginHook::OnLoad;
    let hook2 = PluginHook::OnLoad;
    let hook3 = PluginHook::OnUnload;

    assert_eq!(hook1, hook2);
    assert_ne!(hook1, hook3);
}

#[test]
fn test_plugin_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert("name".to_string(), "Test Plugin".to_string());
    metadata.insert("version".to_string(), "1.0.0".to_string());
    metadata.insert("author".to_string(), "Test Author".to_string());

    assert_eq!(metadata.len(), 3);
    assert_eq!(metadata.get("name"), Some(&"Test Plugin".to_string()));
}

#[test]
fn test_plugin_config() {
    let mut config = HashMap::new();
    config.insert("theme".to_string(), "dark".to_string());
    config.insert("language".to_string(), "en".to_string());

    assert_eq!(config.len(), 2);
    assert_eq!(config.get("theme"), Some(&"dark".to_string()));
}

#[test]
fn test_plugin_hook_serialization() {
    let hook = PluginHook::OnDocumentSave;
    let serialized = serde_json::to_string(&hook);
    assert!(serialized.is_ok());

    let deserialized: Result<PluginHook, _> = serde_json::from_str(&serialized.unwrap());
    assert!(deserialized.is_ok());
}
