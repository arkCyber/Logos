// 插件系统应用案例
// 演示如何使用插件系统来管理 WebAssembly 插件

use logos_lib::plugin_service::{PluginHook, PluginInstance, PluginManager};
use std::collections::HashMap;

fn main() {
    println!("=== 插件系统应用案例 ===\n");

    // 创建插件管理器
    let mut manager = PluginManager::new();

    // 示例 1: 获取插件统计信息
    println!("1. 获取初始插件统计信息...");
    let stats = manager.get_stats();
    println!("   统计信息:");
    println!("   - 总插件数: {}", stats.total_plugins);
    println!("   - 已启用: {}", stats.enabled_plugins);
    println!("   - 已加载: {}", stats.loaded_plugins);
    println!("   - 已禁用: {}", stats.disabled_plugins);
    println!();

    // 示例 2: 列出可用的钩子
    println!("2. 列出可用的插件钩子...");
    println!("   可用钩子:");
    println!("   - OnLoad: 插件加载时触发");
    println!("   - OnUnload: 插件卸载时触发");
    println!("   - OnDocumentOpen: 文档打开时触发");
    println!("   - OnDocumentSave: 文档保存时触发");
    println!("   - OnDocumentChange: 文档变化时触发");
    println!("   - OnCommand: 命令执行时触发");
    println!("   - OnRender: 渲染时触发");
    println!("   - OnExport: 导出时触发");
    println!("   - OnTypstCompile: Typst 编译时触发");
    println!("   - OnTypstError: Typst 错误时触发");
    println!();

    // 示例 3: 模拟加载插件
    println!("3. 模拟加载插件...");
    // 注意: 这只是演示，实际加载需要真实的 WASM 文件
    println!("   插件加载流程:");
    println!("   1. 读取插件元数据");
    println!("   2. 验证插件签名");
    println!("   3. 加载 WASM 模块");
    println!("   4. 初始化插件实例");
    println!("   5. 触发 OnLoad 钩子");
    println!("   ✓ 插件加载流程演示完成");
    println!();

    // 示例 4: 注册插件钩子
    println!("4. 注册插件钩子...");
    let plugin_id = "example-plugin".to_string();
    let hook = PluginHook::OnDocumentSave;
    let handler = "handle_document_save".to_string();

    // 模拟注册钩子
    println!("   注册钩子:");
    println!("   - 插件 ID: {}", plugin_id);
    println!("   - 钩子类型: {:?}", hook);
    println!("   - 处理函数: {}", handler);
    println!("   ✓ 钩子注册演示完成");
    println!();

    // 示例 5: 触发单个插件钩子
    println!("5. 触发单个插件钩子...");
    println!("   触发 OnDocumentSave 钩子:");
    println!("   - 插件: {}", plugin_id);
    println!("   - 钩子: {:?}", hook);
    println!("   - 处理函数: {}", handler);
    println!("   ✓ 钩子触发演示完成");
    println!();

    // 示例 6: 触发所有插件的钩子
    println!("6. 触发所有插件的钩子...");
    let hook_all = PluginHook::OnRender;
    println!("   触发 OnRender 钩子 (所有插件):");
    println!("   - 钩子类型: {:?}", hook_all);
    println!("   - 影响插件: 所有已加载插件");
    println!("   ✓ 批量钩子触发演示完成");
    println!();

    // 示例 7: 设置插件配置
    println!("7. 设置插件配置...");
    let config_key = "theme".to_string();
    let config_value = "dark".to_string();
    println!("   配置设置:");
    println!("   - 插件: {}", plugin_id);
    println!("   - 键: {}", config_key);
    println!("   - 值: {}", config_value);
    println!("   ✓ 配置设置演示完成");
    println!();

    // 示例 8: 获取插件配置
    println!("8. 获取插件配置...");
    println!("   配置查询:");
    println!("   - 插件: {}", plugin_id);
    println!("   - 键: {}", config_key);
    println!("   - 值: {}", config_value);
    println!("   ✓ 配置查询演示完成");
    println!();

    // 示例 9: 检查插件状态
    println!("9. 检查插件状态...");
    println!("   状态检查:");
    println!("   - 插件: {}", plugin_id);
    println!("   - 已加载: false (演示)");
    println!("   - 已启用: true (演示)");
    println!("   ✓ 状态检查演示完成");
    println!();

    // 示例 10: 重载插件
    println!("10. 重载插件...");
    println!("   重载流程:");
    println!("   1. 触发 OnUnload 钩子");
    println!("   2. 卸载插件实例");
    println!("   3. 重新加载插件");
    println!("   4. 触发 OnLoad 钩子");
    println!("   ✓ 插件重载演示完成");
    println!();

    // 示例 11: 启用插件
    println!("11. 启用插件...");
    println!("   启用流程:");
    println!("   - 插件: {}", plugin_id);
    println!("   - 状态: 已启用");
    println!("   ✓ 插件启用演示完成");
    println!();

    // 示例 12: 禁用插件
    println!("12. 禁用插件...");
    println!("   禁用流程:");
    println!("   - 插件: {}", plugin_id);
    println!("   - 状态: 已禁用");
    println!("   ✓ 插件禁用演示完成");
    println!();

    // 示例 13: 卸载插件
    println!("13. 卸载插件...");
    println!("   卸载流程:");
    println!("   1. 触发 OnUnload 钩子");
    println!("   2. 清理插件资源");
    println!("   3. 移除插件实例");
    println!("   ✓ 插件卸载演示完成");
    println!();

    // 示例 14: 列出所有插件
    println!("14. 列出所有插件...");
    println!("   插件列表 (演示):");
    println!("   1. example-plugin - 示例插件");
    println!("   2. math-plugin - 数学公式插件");
    println!("   3. export-plugin - 导出插件");
    println!("   ✓ 插件列表演示完成");
    println!();

    // 示例 15: 获取插件实例
    println!("15. 获取插件实例...");
    println!("   实例查询:");
    println!("   - 插件 ID: {}", plugin_id);
    println!("   - 元数据: 版本、作者、描述");
    println!("   - 钩子: 已注册的钩子列表");
    println!("   - 配置: 插件配置字典");
    println!("   ✓ 实例查询演示完成");
    println!();

    println!("=== 插件系统案例演示完成 ===");
}
