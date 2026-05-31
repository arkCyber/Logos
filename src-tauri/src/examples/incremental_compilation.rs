// 增量编译应用案例
// 演示如何使用增量编译功能来加速 Typst 文档编译

use logos_lib::typist_service::incremental::{IncrementalCompiler, IncrementalConfig};

fn main() {
    println!("=== 增量编译应用案例 ===\n");

    // 创建增量编译器
    let config = IncrementalConfig {
        enabled: true,
        cache_dir: std::path::PathBuf::from(".typst_cache"),
        cache_ttl_seconds: 86400, // 24 hours
        max_cache_size_mb: 100,
    };

    let mut compiler = IncrementalCompiler::new(config);

    // 示例 1: 首次编译
    println!("1. 首次编译文档...");
    let source = r#"
# 标题
这是一个示例文档。

## 第一节
内容...
"#;

    let hash = compiler.compute_hash(source);
    println!("   文档哈希: {}", hash);

    // 模拟编译输出
    let compiled_output = vec![1, 2, 3, 4, 5];
    let dependencies = vec!["style.typ".to_string()];

    compiler.update_cache("document1", hash, compiled_output.clone(), &dependencies);
    println!("   ✓ 编译完成并缓存\n");

    // 示例 2: 检查缓存
    println!("2. 检查缓存...");
    if let Some(cached) = compiler.get_cached("document1") {
        println!("   ✓ 找到缓存条目");
        println!("   - 哈希: {}", cached.hash);
        println!("   - 输出大小: {} bytes", cached.compiled_output.len());
        println!("   - 依赖数: {}", cached.dependencies.len());
    }
    println!();

    // 示例 3: 重新编译（未修改）
    println!("3. 重新编译未修改的文档...");
    let new_hash = compiler.compute_hash(source);
    if compiler.is_cache_valid("document1", new_hash) {
        println!("   ✓ 缓存有效，使用缓存");
    } else {
        println!("   ✗ 缓存无效，需要重新编译");
    }
    println!();

    // 示例 4: 修改文档后重新编译
    println!("4. 修改文档后重新编译...");
    let modified_source = r#"
# 标题
这是一个示例文档。

## 第一节
内容...

## 第二节
新增内容...
"#;

    let modified_hash = compiler.compute_hash(modified_source);
    if compiler.is_cache_valid("document1", modified_hash) {
        println!("   ✓ 缓存有效");
    } else {
        println!("   ✗ 缓存无效，需要重新编译");
        println!("   - 原哈希: {}", hash);
        println!("   - 新哈希: {}", modified_hash);

        // 更新缓存
        let new_output = vec![1, 2, 3, 4, 5, 6, 7, 8];
        compiler.update_cache("document1", modified_hash, new_output, &dependencies);
        println!("   ✓ 已更新缓存");
    }
    println!();

    // 示例 5: 依赖检查
    println!("5. 检查依赖变化...");
    let new_dependencies = vec!["style.typ".to_string(), "theme.typ".to_string()];
    if compiler.dependencies_changed("document1", &new_dependencies) {
        println!("   ✗ 依赖已变化，需要重新编译");
    } else {
        println!("   ✓ 依赖未变化");
    }
    println!();

    // 示例 6: 缓存统计
    println!("6. 缓存统计...");
    let cache_size = compiler.get_cache_size();
    let cache_count = compiler.cache.len();
    println!("   - 缓存大小: {} bytes", cache_size);
    println!("   - 缓存条目数: {}", cache_count);
    println!();

    // 示例 7: 清理过期缓存
    println!("7. 清理过期缓存...");
    compiler.clear_expired();
    println!("   ✓ 已清理过期缓存");
    println!();

    // 示例 8: 强制缓存限制
    println!("8. 强制缓存限制...");
    // 创建新的编译器来测试缓存限制
    let small_config = IncrementalConfig {
        enabled: true,
        cache_dir: std::path::PathBuf::from(".typst_cache"),
        cache_ttl_seconds: 86400,
        max_cache_size_mb: 1, // 1MB 限制
    };
    let mut limited_compiler = IncrementalCompiler::new(small_config);
    limited_compiler.update_cache(
        "test".to_string(),
        hash.clone(),
        dependencies.clone(),
        compiled_output,
    );
    limited_compiler.enforce_cache_limit();
    println!("   ✓ 已强制执行缓存限制");
    println!();

    // 示例 9: 清空所有缓存
    println!("9. 清空所有缓存...");
    compiler.clear_all();
    println!("   ✓ 已清空所有缓存");
    println!();

    println!("=== 增量编译案例演示完成 ===");
}
