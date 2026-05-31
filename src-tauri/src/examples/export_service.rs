// 导出服务应用案例
// 演示如何使用导出服务将文档导出为多种格式

use logos_lib::export_service::{DocumentMetadata, ExportConfig, ExportFormat, ExportGenerator};

fn main() {
    println!("=== 导出服务应用案例 ===\n");

    // 创建导出生成器
    let generator = ExportGenerator::new();

    // 示例文档内容
    let document_content = r#"
# 示例文档

这是一个示例文档，用于演示导出功能。

## 第一节
这里是第一节的内容。

## 第二节
这里是第二节的内容。

### 子节
这是子节内容。
"#;

    // 示例 1: 列出支持的格式
    println!("1. 列出支持的导出格式...");
    let formats = generator.get_supported_formats();
    println!("   支持的格式:");
    for format in &formats {
        println!("   - {:?}", format);
    }
    println!();

    // 示例 2: 导出为 PDF
    println!("2. 导出为 PDF...");
    let pdf_config = ExportConfig {
        format: ExportFormat::Pdf,
        metadata: DocumentMetadata {
            title: "示例文档".to_string(),
            author: "作者".to_string(),
            subject: "导出测试".to_string(),
            keywords: vec!["示例".to_string(), "测试".to_string()],
            creation_date: Some("2024-01-01".to_string()),
        },
        include_toc: true,
        include_page_numbers: true,
        compress_images: true,
        embed_fonts: true,
    };

    match generator.export(document_content, &pdf_config) {
        Ok(result) => {
            println!("   ✓ PDF 导出成功");
            println!("   - 文件大小: {} bytes", result.file_size);
            println!("   - 格式: {:?}", result.format);
        }
        Err(e) => println!("   ✗ PDF 导出失败: {}", e),
    }
    println!();

    // 示例 3: 导出为 HTML
    println!("3. 导出为 HTML...");
    let html_config = ExportConfig {
        format: ExportFormat::Html,
        metadata: DocumentMetadata {
            title: "示例文档".to_string(),
            author: "作者".to_string(),
            subject: "导出测试".to_string(),
            keywords: vec!["示例".to_string(), "测试".to_string()],
            creation_date: Some("2024-01-01".to_string()),
        },
        include_toc: true,
        include_page_numbers: false,
        compress_images: false,
        embed_fonts: false,
    };

    match generator.export(document_content, &html_config) {
        Ok(result) => {
            println!("   ✓ HTML 导出成功");
            println!("   - 文件大小: {} bytes", result.file_size);
            println!("   - 格式: {:?}", result.format);
        }
        Err(e) => println!("   ✗ HTML 导出失败: {}", e),
    }
    println!();

    // 示例 4: 导出为 SVG
    println!("4. 导出为 SVG...");
    let svg_config = ExportConfig {
        format: ExportFormat::Svg,
        metadata: DocumentMetadata {
            title: "示例文档".to_string(),
            author: "作者".to_string(),
            subject: "导出测试".to_string(),
            keywords: vec!["示例".to_string(), "测试".to_string()],
            creation_date: Some("2024-01-01".to_string()),
        },
        include_toc: false,
        include_page_numbers: false,
        compress_images: false,
        embed_fonts: false,
    };

    match generator.export(document_content, &svg_config) {
        Ok(result) => {
            println!("   ✓ SVG 导出成功");
            println!("   - 文件大小: {} bytes", result.file_size);
            println!("   - 格式: {:?}", result.format);
        }
        Err(e) => println!("   ✗ SVG 导出失败: {}", e),
    }
    println!();

    // 示例 5: 导出为 PNG
    println!("5. 导出为 PNG...");
    let png_config = ExportConfig {
        format: ExportFormat::Png,
        metadata: DocumentMetadata {
            title: "示例文档".to_string(),
            author: "作者".to_string(),
            subject: "导出测试".to_string(),
            keywords: vec!["示例".to_string(), "测试".to_string()],
            creation_date: Some("2024-01-01".to_string()),
        },
        include_toc: false,
        include_page_numbers: false,
        compress_images: true,
        embed_fonts: false,
    };

    match generator.export(document_content, &png_config) {
        Ok(result) => {
            println!("   ✓ PNG 导出成功");
            println!("   - 文件大小: {} bytes", result.file_size);
            println!("   - 格式: {:?}", result.format);
        }
        Err(e) => println!("   ✗ PNG 导出失败: {}", e),
    }
    println!();

    // 示例 6: 导出为 Markdown
    println!("6. 导出为 Markdown...");
    let md_config = ExportConfig {
        format: ExportFormat::Markdown,
        metadata: DocumentMetadata::default(),
        include_toc: false,
        include_page_numbers: false,
        compress_images: false,
        embed_fonts: false,
    };

    match generator.export(document_content, &md_config) {
        Ok(result) => {
            println!("   ✓ Markdown 导出成功");
            println!("   - 文件大小: {} bytes", result.file_size);
            println!("   - 格式: {:?}", result.format);
        }
        Err(e) => println!("   ✗ Markdown 导出失败: {}", e),
    }
    println!();

    // 示例 7: 自定义元数据
    println!("7. 使用自定义元数据导出...");
    let custom_metadata = DocumentMetadata {
        title: "自定义标题".to_string(),
        author: "自定义作者".to_string(),
        subject: "自定义主题".to_string(),
        keywords: vec![
            "关键词1".to_string(),
            "关键词2".to_string(),
            "关键词3".to_string(),
        ],
        creation_date: Some("2024-12-31".to_string()),
    };

    let custom_config = ExportConfig {
        format: ExportFormat::Pdf,
        metadata: custom_metadata,
        include_toc: true,
        include_page_numbers: true,
        compress_images: true,
        embed_fonts: true,
    };

    match generator.export(document_content, &custom_config) {
        Ok(result) => {
            println!("   ✓ 自定义元数据导出成功");
            println!("   - 标题: {}", custom_config.metadata.title);
            println!("   - 作者: {}", custom_config.metadata.author);
        }
        Err(e) => println!("   ✗ 导出失败: {}", e),
    }
    println!();

    // 示例 8: 批量导出
    println!("8. 批量导出为多种格式...");
    let export_formats = vec![
        ExportFormat::Pdf,
        ExportFormat::Html,
        ExportFormat::Markdown,
    ];

    for format in export_formats {
        let batch_config = ExportConfig {
            format: format.clone(),
            metadata: DocumentMetadata::default(),
            include_toc: true,
            include_page_numbers: true,
            compress_images: true,
            embed_fonts: true,
        };

        match generator.export(document_content, &batch_config) {
            Ok(result) => {
                println!("   ✓ {:?} 导出成功 ({} bytes)", format, result.file_size);
            }
            Err(e) => {
                println!("   ✗ {:?} 导出失败: {}", format, e);
            }
        }
    }
    println!();

    println!("=== 导出服务案例演示完成 ===");
}
