// 增量编译集成测试
// 演示如何使用增量编译功能来加速 Typst 文档编译

use crate::typist_service::incremental::{IncrementalCompiler, IncrementalConfig};
use std::path::PathBuf;

#[test]
fn test_incremental_compilation_workflow() {
    // 创建增量编译器
    let config = IncrementalConfig {
        enabled: true,
        cache_dir: PathBuf::from(".typst_cache"),
        cache_ttl_seconds: 86400, // 24 hours
        max_cache_size_mb: 100,
        hot_reload: false,
        max_parallel_jobs: 1,
        parallel_compilation: false,
    };

    let compiler = IncrementalCompiler::new(config);

    // 测试 1: 首次编译
    let source = r#"
# 标题
这是一个示例文档。

## 第一节
内容...
"#;

    let hash = compiler.compute_hash(source);
    assert!(!hash.is_empty());

    // 模拟编译输出
    let compiled_output = vec![1, 2, 3, 4, 5];
    let dependencies = vec!["style.typ".to_string()];

    compiler.update_cache(
        "document1".to_string(),
        hash.clone(),
        dependencies.clone(),
        compiled_output.clone(),
    );

    // 测试 2: 检查缓存
    let cached = compiler.get_cached("document1");
    assert!(cached.is_some());
    let entry = cached.unwrap();
    assert_eq!(entry.hash, hash);
    assert_eq!(entry.compiled_output.len(), 5);
    assert_eq!(entry.dependencies.len(), 1);

    // 测试 3: 重新编译（未修改）
    let new_hash = compiler.compute_hash(source);
    assert!(compiler.is_cache_valid("document1", &new_hash));

    // 测试 4: 修改文档后重新编译
    let modified_source = r#"
# 标题
这是一个示例文档。

## 第一节
内容...

## 第二节
新增内容...
"#;

    let modified_hash = compiler.compute_hash(modified_source);
    assert!(!compiler.is_cache_valid("document1", &modified_hash));

    // 更新缓存
    let new_output = vec![1, 2, 3, 4, 5, 6, 7, 8];
    compiler.update_cache(
        "document1".to_string(),
        modified_hash.clone(),
        dependencies.clone(),
        new_output,
    );

    // 验证缓存已更新
    let updated_cached = compiler.get_cached("document1");
    assert!(updated_cached.is_some());
    assert_eq!(updated_cached.unwrap().hash, modified_hash);

    // 测试 5: 依赖检查
    let new_dependencies = vec!["style.typ".to_string(), "theme.typ".to_string()];
    assert!(compiler.dependencies_changed("document1", &new_dependencies));

    // 测试 6: 缓存统计
    let cache_size = compiler.get_cache_size();
    assert!(cache_size > 0);

    // 测试 7: 清理过期缓存
    compiler.clear_expired();

    // 测试 8: 清空所有缓存
    compiler.clear_all();
    assert!(compiler.get_cached("document1").is_none());
}

#[test]
fn test_incremental_config_default() {
    let config = IncrementalConfig::default();
    assert!(config.enabled);
    assert_eq!(config.cache_ttl_seconds, 86400);
    assert_eq!(config.max_cache_size_mb, 1024);
}

#[test]
fn test_incremental_compiler_with_default_config() {
    let compiler = IncrementalCompiler::with_default_config();
    assert!(compiler.is_cache_valid("nonexistent", "hash") == false);
}

#[test]
fn test_hash_computation() {
    let compiler = IncrementalCompiler::with_default_config();
    let source1 = "test content";
    let source2 = "test content";
    let source3 = "different content";

    let hash1 = compiler.compute_hash(source1);
    let hash2 = compiler.compute_hash(source2);
    let hash3 = compiler.compute_hash(source3);

    assert_eq!(hash1, hash2);
    assert_ne!(hash1, hash3);
}

#[test]
fn test_cache_enforcement() {
    let config = IncrementalConfig {
        enabled: true,
        cache_dir: PathBuf::from(".typst_cache"),
        cache_ttl_seconds: 86400,
        max_cache_size_mb: 1, // 1MB 限制
        hot_reload: false,
        max_parallel_jobs: 1,
        parallel_compilation: false,
    };

    let compiler = IncrementalCompiler::new(config);

    // 添加多个缓存条目
    for i in 0..10 {
        let output = vec![0u8; 100000]; // 100KB 每个条目
        compiler.update_cache(format!("doc{}", i), format!("hash{}", i), vec![], output);
    }

    // 强制执行缓存限制
    compiler.enforce_cache_limit();

    // 验证缓存大小在限制内
    let cache_size = compiler.get_cache_size();
    assert!(cache_size <= 1024 * 1024); // 1MB
}
