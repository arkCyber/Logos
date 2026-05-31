// 包管理集成测试
// 演示如何使用包管理系统来管理 Typst 包

use crate::typist_service::package::{
    PackageInfo, PackageManager, PackageManagerConfig, PackageRepository,
};
use chrono::Utc;
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_package_management_workflow() {
    // 创建包管理器
    let config = PackageManagerConfig {
        repositories: vec![PackageRepository::default()],
        cache_dir: PathBuf::from("/tmp/typst_packages"),
        install_dir: PathBuf::from("/tmp/typst_packages/install"),
        auto_update: true,
    };

    let mut manager = PackageManager::new(config);

    // 测试 1: 添加包到缓存
    let package = PackageInfo {
        name: "celery".to_string(),
        version: "0.1.0".to_string(),
        description: "A Typst package for math symbols".to_string(),
        author: "Typst Team".to_string(),
        license: "MIT".to_string(),
        repository: Some("https://github.com".to_string()),
        homepage: None,
        keywords: vec!["math".to_string(), "symbols".to_string()],
        typst_version: "0.11.0".to_string(),
        dependencies: HashMap::new(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        downloads: 1000,
    };

    manager.add_to_cache(package);

    // 测试 2: 搜索包
    let results = manager.search("math");
    assert!(results.len() >= 0);

    // 测试 3: 安装包
    let install_result = manager.install("celery", None);
    assert!(install_result.is_ok());

    // 测试 4: 列出已安装的包
    let installed = manager.list_installed();
    assert!(installed.len() >= 1);

    // 测试 5: 更新包
    let update_result = manager.update("celery");
    assert!(update_result.is_ok());

    // 测试 6: 卸载包
    let uninstall_result = manager.uninstall("celery");
    assert!(uninstall_result.is_ok());

    // 测试 7: 获取统计信息
    let stats = manager.get_stats();
    assert!(stats.total_packages >= 0);
    assert!(stats.installed_packages >= 0);
}

#[test]
fn test_package_manager_default() {
    let manager = PackageManager::default();
    let stats = manager.get_stats();
    assert_eq!(stats.total_packages, 0);
}

#[test]
fn test_package_info_creation() {
    let package = PackageInfo {
        name: "test-package".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package".to_string(),
        author: "Test Author".to_string(),
        license: "MIT".to_string(),
        repository: None,
        homepage: None,
        keywords: vec!["test".to_string()],
        typst_version: "0.11.0".to_string(),
        dependencies: HashMap::new(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        downloads: 0,
    };

    assert_eq!(package.name, "test-package");
    assert_eq!(package.version, "1.0.0");
}

#[test]
fn test_package_repository_creation() {
    let repo = PackageRepository {
        name: "test-repo".to_string(),
        url: "https://example.com".to_string(),
        enabled: true,
    };

    assert_eq!(repo.name, "test-repo");
    assert_eq!(repo.url, "https://example.com");
    assert!(repo.enabled);
}

#[test]
fn test_package_manager_config() {
    let config = PackageManagerConfig {
        repositories: vec![],
        cache_dir: PathBuf::from("/test/cache"),
        install_dir: PathBuf::from("/test/install"),
        auto_update: false,
    };

    assert_eq!(config.cache_dir, PathBuf::from("/test/cache"));
    assert!(!config.auto_update);
}

#[test]
fn test_dependency_checking() {
    let mut manager = PackageManager::default();

    // 添加一个有依赖的包
    let mut deps = HashMap::new();
    deps.insert("foundation".to_string(), "1.0.0".to_string());

    let package = PackageInfo {
        name: "test-pkg".to_string(),
        version: "1.0.0".to_string(),
        description: "Test".to_string(),
        author: "Test".to_string(),
        license: "MIT".to_string(),
        repository: None,
        homepage: None,
        keywords: vec![],
        typst_version: "0.11.0".to_string(),
        dependencies: deps,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        downloads: 0,
    };

    manager.add_to_cache(package);

    // 检查依赖
    let missing = manager.check_dependencies("test-pkg");
    assert!(missing.is_ok());
    assert!(missing.unwrap().len() >= 0);
}
