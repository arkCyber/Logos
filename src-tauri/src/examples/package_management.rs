// 包管理应用案例
// 演示如何使用包管理系统来管理 Typst 包

use logos_lib::typist_service::package::{
    PackageInfo, PackageManager, PackageManagerConfig, PackageRepository,
};

fn main() {
    println!("=== 包管理应用案例 ===\n");

    // 创建包管理器
    let config = PackageManagerConfig {
        cache_dir: "/tmp/typst_packages".to_string(),
        auto_update: true,
        max_packages: 100,
    };

    let mut manager = PackageManager::new(config);

    // 示例 1: 添加包仓库
    println!("1. 添加包仓库...");
    let repo = PackageRepository {
        name: "typst-universe".to_string(),
        url: "https://packages.typst.org".to_string(),
        enabled: true,
    };
    manager.add_repository(repo);
    println!("   ✓ 已添加仓库: typst-universe\n");

    // 示例 2: 搜索包
    println!("2. 搜索包...");
    let results = manager.search("math");
    println!("   搜索 'math' 的结果:");
    for (i, pkg) in results.iter().take(3).enumerate() {
        println!("   {}. {} - {}", i + 1, pkg.name, pkg.description);
    }
    println!();

    // 示例 3: 获取包信息
    println!("3. 获取包信息...");
    let package_info = PackageInfo {
        name: "celery".to_string(),
        version: "0.1.0".to_string(),
        description: "A Typst package for math symbols".to_string(),
        author: "Typst Team".to_string(),
        license: "MIT".to_string(),
        dependencies: vec![],
        path: "/tmp/celery.typ".to_string(),
        installed: false,
        enabled: true,
    };
    println!("   包信息:");
    println!("   - 名称: {}", package_info.name);
    println!("   - 版本: {}", package_info.version);
    println!("   - 描述: {}", package_info.description);
    println!("   - 作者: {}", package_info.author);
    println!("   - 许可证: {}", package_info.license);
    println!();

    // 示例 4: 检查依赖
    println!("4. 检查依赖...");
    // 先添加一个测试包到缓存
    let test_package = PackageInfo {
        name: "test-package".to_string(),
        version: "1.0.0".to_string(),
        description: "Test package".to_string(),
        author: "Test".to_string(),
        license: "MIT".to_string(),
        repository: None,
        homepage: None,
        keywords: vec![],
        typst_version: "0.12.0".to_string(),
        dependencies: std::collections::HashMap::new(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        downloads: 0,
    };
    manager.add_to_cache(test_package);

    let missing = manager.check_dependencies("test-package");
    match missing {
        Ok(deps) => {
            if deps.is_empty() {
                println!("   ✓ 所有依赖已满足");
            } else {
                println!("   ✗ 缺失依赖:");
                for dep in deps {
                    println!("     - {}", dep);
                }
            }
        }
        Err(e) => println!("   ✗ 检查依赖失败: {}", e),
    }
    println!();

    // 示例 5: 安装包
    println!("5. 安装包...");
    let install_result = manager.install("celery", None);
    match install_result {
        Ok(_) => println!("   ✓ 包安装成功"),
        Err(e) => println!("   ✗ 包安装失败: {}", e),
    }
    println!();

    // 示例 6: 列出已安装的包
    println!("6. 列出已安装的包...");
    let installed = manager.list_installed();
    println!("   已安装的包 ({} 个):", installed.len());
    for pkg in installed.iter().take(5) {
        println!("   - {} v{}", pkg.name, pkg.version);
    }
    println!();

    // 示例 7: 更新包
    println!("7. 更新包...");
    let update_result = manager.update("celery");
    match update_result {
        Ok(_) => println!("   ✓ 包更新成功"),
        Err(e) => println!("   ✗ 包更新失败: {}", e),
    }
    println!();

    // 示例 8: 卸载包
    println!("8. 卸载包...");
    let uninstall_result = manager.uninstall("celery");
    match uninstall_result {
        Ok(_) => println!("   ✓ 包卸载成功"),
        Err(e) => println!("   ✗ 包卸载失败: {}", e),
    }
    println!();

    // 示例 9: 获取统计信息
    println!("9. 获取统计信息...");
    let stats = manager.get_stats();
    println!("   统计信息:");
    println!("   - 总包数: {}", stats.total_packages);
    println!("   - 已安装: {}", stats.installed_packages);
    println!("   - 总下载量: {}", stats.total_downloads);
    println!();

    // 示例 10: 仓库管理
    println!("10. 仓库管理...");
    println!("   - 当前仓库数: {}", manager.config.repositories.len());
    for repo in &manager.config.repositories {
        println!(
            "   - {} ({}): {}",
            repo.name,
            if repo.enabled { "启用" } else { "禁用" },
            repo.url
        );
    }
    println!();

    println!("=== 包管理案例演示完成 ===");
}
