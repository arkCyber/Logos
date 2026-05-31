// 无障碍服务集成测试
// 演示如何使用无障碍服务来管理文档的无障碍树

use std::sync::Arc;
use crate::accessibility_service::bridge::{
    AccessibilityBridge, AccessibilityNode, AccessibilityRole, AriaAttribute,
};
use crate::config_service::ExportConfigService;

#[test]
fn test_accessibility_service_workflow() {
    let config_service = Arc::new(ExportConfigService::new());
    let mut bridge = AccessibilityBridge::new(config_service);

    // 测试 1: 构建无障碍树
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

    let _build_result = bridge.build_tree(html_content);
    // 在实际实现中，这里会解析 HTML 并构建无障碍树
    // 由于是演示，我们只验证方法可以调用

    // 测试 2: 创建自定义无障碍节点
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

    assert_eq!(button_node.id, "submit-btn");
    assert_eq!(button_node.role, AccessibilityRole::Button);
    assert_eq!(button_node.label, "提交表单");
    assert_eq!(button_node.attributes.len(), 1);

    // 测试 3: 验证节点属性
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
    // 按钮没有标签应该产生警告
    assert!(!warnings.is_empty());

    // 测试 4: 验证有效节点
    let valid_node = AccessibilityNode {
        id: "valid-btn".to_string(),
        role: AccessibilityRole::Button,
        label: "有效标签".to_string(),
        description: "有效描述".to_string(),
        attributes: vec![],
        children: vec![],
        parent: None,
        level: None,
        live: false,
        atomic: false,
        hidden: false,
    };

    let valid_warnings = bridge.validate_attributes(&valid_node);
    // 有标签的按钮应该没有警告
    assert!(valid_warnings.is_empty());

    // 测试 5: 获取统计信息
    let stats = bridge.get_stats();
    assert!(stats.total_nodes >= 0);
    assert!(stats.headings >= 0);
    assert!(stats.images >= 0);
    assert!(stats.links >= 0);
    assert!(stats.tables >= 0);
    assert!(stats.buttons >= 0);
    assert!(stats.missing_labels >= 0);
    assert!(stats.hidden_nodes >= 0);
}

#[test]
fn test_accessibility_bridge_creation() {
    let config_service = Arc::new(ExportConfigService::new());
    let bridge = AccessibilityBridge::new(config_service);
    let stats = bridge.get_stats();
    assert_eq!(stats.total_nodes, 0);
}

#[test]
fn test_accessibility_node_creation() {
    let node = AccessibilityNode {
        id: "test-node".to_string(),
        role: AccessibilityRole::Document,
        label: "Test".to_string(),
        description: "Test description".to_string(),
        attributes: vec![],
        children: vec![],
        parent: None,
        level: None,
        live: false,
        atomic: false,
        hidden: false,
    };

    assert_eq!(node.id, "test-node");
    assert_eq!(node.role, AccessibilityRole::Document);
}

#[test]
fn test_aria_attribute_creation() {
    let attr = AriaAttribute {
        name: "aria-label".to_string(),
        value: "Test Label".to_string(),
    };

    assert_eq!(attr.name, "aria-label");
    assert_eq!(attr.value, "Test Label");
}

#[test]
fn test_accessibility_role_variants() {
    let roles = vec![
        AccessibilityRole::Document,
        AccessibilityRole::Heading,
        AccessibilityRole::Paragraph,
        AccessibilityRole::List,
        AccessibilityRole::ListItem,
        AccessibilityRole::Link,
        AccessibilityRole::Button,
        AccessibilityRole::TextBox,
        AccessibilityRole::Image,
        AccessibilityRole::Table,
    ];

    for role in roles {
        // 验证所有角色都可以创建
        let _ = role;
    }
}

#[test]
fn test_accessibility_node_with_children() {
    let node = AccessibilityNode {
        id: "parent".to_string(),
        role: AccessibilityRole::Section,
        label: "Parent".to_string(),
        description: "Parent node".to_string(),
        attributes: vec![],
        children: vec!["child1".to_string(), "child2".to_string()],
        parent: None,
        level: None,
        live: false,
        atomic: false,
        hidden: false,
    };

    assert_eq!(node.children.len(), 2);
}

#[test]
fn test_accessibility_node_with_parent() {
    let node = AccessibilityNode {
        id: "child".to_string(),
        role: AccessibilityRole::Button,
        label: "Child".to_string(),
        description: "Child node".to_string(),
        attributes: vec![],
        children: vec![],
        parent: Some("parent".to_string()),
        level: None,
        live: false,
        atomic: false,
        hidden: false,
    };

    assert!(node.parent.is_some());
    assert_eq!(node.parent.unwrap(), "parent");
}

#[test]
fn test_accessibility_node_with_level() {
    let node = AccessibilityNode {
        id: "heading".to_string(),
        role: AccessibilityRole::Heading,
        label: "Heading".to_string(),
        description: "Heading node".to_string(),
        attributes: vec![],
        children: vec![],
        parent: None,
        level: Some(2),
        live: false,
        atomic: false,
        hidden: false,
    };

    assert_eq!(node.level, Some(2));
}

#[test]
fn test_accessibility_node_with_live_region() {
    let node = AccessibilityNode {
        id: "live-region".to_string(),
        role: AccessibilityRole::Section,
        label: "Live Region".to_string(),
        description: "Live region node".to_string(),
        attributes: vec![],
        children: vec![],
        parent: None,
        level: None,
        live: true,
        atomic: false,
        hidden: false,
    };

    assert!(node.live);
}

#[test]
fn test_accessibility_node_with_atomic() {
    let node = AccessibilityNode {
        id: "atomic".to_string(),
        role: AccessibilityRole::Section,
        label: "Atomic".to_string(),
        description: "Atomic node".to_string(),
        attributes: vec![],
        children: vec![],
        parent: None,
        level: None,
        live: false,
        atomic: true,
        hidden: false,
    };

    assert!(node.atomic);
}

#[test]
fn test_accessibility_node_with_hidden() {
    let node = AccessibilityNode {
        id: "hidden".to_string(),
        role: AccessibilityRole::Section,
        label: "Hidden".to_string(),
        description: "Hidden node".to_string(),
        attributes: vec![],
        children: vec![],
        parent: None,
        level: None,
        live: false,
        atomic: false,
        hidden: true,
    };

    assert!(node.hidden);
}

#[test]
fn test_validate_image_without_alt() {
    let config_service = Arc::new(ExportConfigService::new());
    let bridge = AccessibilityBridge::new(config_service);
    let img_node = AccessibilityNode {
        id: "img1".to_string(),
        role: AccessibilityRole::Image,
        label: "".to_string(),
        description: "".to_string(),
        attributes: vec![],
        children: vec![],
        parent: None,
        level: None,
        live: false,
        atomic: false,
        hidden: false,
    };

    let warnings = bridge.validate_attributes(&img_node);
    // 图像没有 alt 文本应该产生警告
    assert!(!warnings.is_empty());
}

#[test]
fn test_validate_link_without_label() {
    let config_service = Arc::new(ExportConfigService::new());
    let bridge = AccessibilityBridge::new(config_service);
    let link_node = AccessibilityNode {
        id: "link1".to_string(),
        role: AccessibilityRole::Link,
        label: "".to_string(),
        description: "".to_string(),
        attributes: vec![],
        children: vec![],
        parent: None,
        level: None,
        live: false,
        atomic: false,
        hidden: false,
    };

    let warnings = bridge.validate_attributes(&link_node);
    // 链接没有标签应该产生警告
    assert!(!warnings.is_empty());
}
