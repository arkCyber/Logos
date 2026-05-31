// 无障碍服务应用案例
// 演示如何使用无障碍服务来管理文档的无障碍树

use logos_lib::accessibility_service::bridge::{
    AccessibilityBridge, AccessibilityNode, AccessibilityRole, AriaAttribute,
};

fn main() {
    println!("=== 无障碍服务应用案例 ===\n");

    // 创建无障碍桥接器
    let mut bridge = AccessibilityBridge::new();

    // 示例 1: 构建无障碍树
    println!("1. 构建无障碍树...");
    let html_content = r#"
<!DOCTYPE html>
<html>
<head>
    <title>示例页面</title>
</head>
<body>
    <h1>主标题</h1>
    <p>这是一个段落。</p>
    <button aria-label="提交">提交</button>
    <a href="/page" aria-label="链接">链接文本</a>
    <img src="image.jpg" alt="图片描述">
</body>
</html>
"#;

    match bridge.build_tree(html_content) {
        Ok(_) => println!("   ✓ 无障碍树构建成功"),
        Err(e) => println!("   ✗ 无障碍树构建失败: {}", e),
    }
    println!();

    // 示例 2: 获取无障碍树
    println!("2. 获取无障碍树...");
    if let Some(tree) = bridge.get_tree() {
        println!("   ✓ 无障碍树信息:");
        println!("   - 根节点 ID: {}", tree.root.id);
        println!("   - 根节点角色: {:?}", tree.root.role);
        println!("   - 总节点数: {}", tree.nodes.len());
    }
    println!();

    // 示例 3: 创建自定义无障碍节点
    println!("3. 创建自定义无障碍节点...");
    let button_node = AccessibilityNode {
        id: "submit-btn".to_string(),
        role: AccessibilityRole::Button,
        label: "提交表单".to_string(),
        description: "点击提交表单数据".to_string(),
        attributes: vec![AriaAttribute {
            name: "aria-pressed".to_string(),
            value: "false".to_string(),
        }],
        children: vec![],
        parent: Some("form".to_string()),
        level: None,
        live: false,
        atomic: false,
        hidden: false,
    };
    println!("   ✓ 创建按钮节点:");
    println!("   - ID: {}", button_node.id);
    println!("   - 角色: {:?}", button_node.role);
    println!("   - 标签: {}", button_node.label);
    println!();

    // 示例 4: 添加节点到树
    println!("4. 添加节点到无障碍树...");
    match bridge.add_node("root".to_string(), button_node) {
        Ok(_) => println!("   ✓ 节点添加成功"),
        Err(e) => println!("   ✗ 节点添加失败: {}", e),
    }
    println!();

    // 示例 5: 设置焦点
    println!("5. 设置焦点...");
    match bridge.set_focus("submit-btn".to_string()) {
        Ok(_) => println!("   ✓ 焦点设置成功"),
        Err(e) => println!("   ✗ 焦点设置失败: {}", e),
    }
    println!();

    // 示例 6: 获取可聚焦节点
    println!("6. 获取可聚焦节点...");
    let focusable_nodes = bridge.get_focusable_nodes();
    println!("   可聚焦节点 ({} 个):", focusable_nodes.len());
    for node in focusable_nodes.iter().take(3) {
        println!("   - {} ({:?})", node.id, node.role);
    }
    println!();

    // 示例 7: 获取标题结构
    println!("7. 获取标题结构...");
    let headings = bridge.get_heading_structure();
    println!("   标题结构:");
    for (id, level, text) in headings.iter().take(5) {
        println!("   - H{}: {} ({})", level, text, id);
    }
    println!();

    // 示例 8: 验证节点属性
    println!("8. 验证节点属性...");
    let test_node = AccessibilityNode {
        id: "test-btn".to_string(),
        role: AccessibilityRole::Button,
        label: "".to_string(), // 空标签
        description: "".to_string(),
        attributes: vec![],
        children: vec![],
        parent: None,
        level: None,
        live: false,
        atomic: false,
        hidden: false,
    };

    let warnings = bridge.validate_attributes(&test_node);
    if warnings.is_empty() {
        println!("   ✓ 节点属性验证通过");
    } else {
        println!("   ✗ 节点属性验证失败:");
        for warning in warnings {
            println!("     - {}", warning);
        }
    }
    println!();

    // 示例 9: 更新节点
    println!("9. 更新节点...");
    let updated_node = AccessibilityNode {
        id: "submit-btn".to_string(),
        role: AccessibilityRole::Button,
        label: "更新后的标签".to_string(),
        description: "更新后的描述".to_string(),
        attributes: vec![AriaAttribute {
            name: "aria-pressed".to_string(),
            value: "true".to_string(),
        }],
        children: vec![],
        parent: Some("form".to_string()),
        level: None,
        live: false,
        atomic: false,
        hidden: false,
    };

    match bridge.update_node("submit-btn".to_string(), updated_node) {
        Ok(_) => println!("   ✓ 节点更新成功"),
        Err(e) => println!("   ✗ 节点更新失败: {}", e),
    }
    println!();

    // 示例 10: 获取统计信息
    println!("10. 获取无障碍统计信息...");
    let stats = bridge.get_stats();
    println!("   统计信息:");
    println!("   - 总节点数: {}", stats.total_nodes);
    println!("   - 标题数: {}", stats.headings);
    println!("   - 图像数: {}", stats.images);
    println!("   - 链接数: {}", stats.links);
    println!("   - 表格数: {}", stats.tables);
    println!("   - 按钮数: {}", stats.buttons);
    println!("   - 缺失标签数: {}", stats.missing_labels);
    println!("   - 隐藏节点数: {}", stats.hidden_nodes);
    println!();

    // 示例 11: 移除节点
    println!("11. 移除节点...");
    match bridge.remove_node("submit-btn".to_string()) {
        Ok(_) => println!("   ✓ 节点移除成功"),
        Err(e) => println!("   ✗ 节点移除失败: {}", e),
    }
    println!();

    println!("=== 无障碍服务案例演示完成 ===");
}
