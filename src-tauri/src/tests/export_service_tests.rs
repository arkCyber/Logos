// 导出服务集成测试
// 演示如何使用导出服务将文档导出为多种格式

use crate::export_service::formats::{DocumentMetadata, TypstQuality};
use crate::export_service::{ExportConfig, ExportFormat, ExportGenerator};
use chrono::Utc;

#[test]
fn test_export_service_workflow() {
    let mut generator = ExportGenerator::new();

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

    // 测试 1: 导出为 PDF
    let pdf_config = ExportConfig {
        format: ExportFormat::Pdf,
        metadata: DocumentMetadata {
            title: "示例文档".to_string(),
            author: "作者".to_string(),
            subject: "导出测试".to_string(),
            keywords: vec!["示例".to_string(), "测试".to_string()],
            created: Utc::now(),
            modified: Utc::now(),
        },
        include_toc: true,
        include_page_numbers: true,
        compress_images: true,
        embed_fonts: true,
        use_typst_rendering: false,
        typst_quality: TypstQuality::Standard,
    };

    let pdf_result = generator.export(document_content, &pdf_config);
    assert!(pdf_result.is_ok());
    let result = pdf_result.unwrap();
    assert!(result.success);
    assert_eq!(result.format, ExportFormat::Pdf);

    // 测试 2: 导出为 HTML
    let html_config = ExportConfig {
        format: ExportFormat::Html,
        metadata: DocumentMetadata {
            title: "示例文档".to_string(),
            author: "作者".to_string(),
            subject: "导出测试".to_string(),
            keywords: vec!["示例".to_string(), "测试".to_string()],
            created: Utc::now(),
            modified: Utc::now(),
        },
        include_toc: true,
        include_page_numbers: false,
        compress_images: false,
        embed_fonts: false,
        use_typst_rendering: false,
        typst_quality: TypstQuality::Standard,
    };

    let html_result = generator.export(document_content, &html_config);
    assert!(html_result.is_ok());
    let result = html_result.unwrap();
    assert!(result.success);
    assert_eq!(result.format, ExportFormat::Html);

    // 测试 3: 导出为 Markdown
    let md_config = ExportConfig {
        format: ExportFormat::Markdown,
        metadata: DocumentMetadata::default(),
        include_toc: false,
        include_page_numbers: false,
        compress_images: false,
        embed_fonts: false,
        use_typst_rendering: false,
        typst_quality: TypstQuality::Standard,
    };

    let md_result = generator.export(document_content, &md_config);
    assert!(md_result.is_ok());
    let result = md_result.unwrap();
    assert!(result.success);
    assert_eq!(result.format, ExportFormat::Markdown);
}

#[test]
fn test_export_generator_creation() {
    let _generator = ExportGenerator::new();
    let config = ExportConfig::default();
    assert_eq!(config.format, ExportFormat::Pdf);
}

#[test]
fn test_document_metadata_default() {
    let metadata = DocumentMetadata::default();
    assert_eq!(metadata.title, "Untitled Document");
    assert_eq!(metadata.author, "Unknown");
    assert!(metadata.keywords.is_empty());
}

#[test]
fn test_document_metadata_creation() {
    let metadata = DocumentMetadata {
        title: "Test Document".to_string(),
        author: "Test Author".to_string(),
        subject: "Test Subject".to_string(),
        keywords: vec!["key1".to_string(), "key2".to_string()],
        created: Utc::now(),
        modified: Utc::now(),
    };

    assert_eq!(metadata.title, "Test Document");
    assert_eq!(metadata.author, "Test Author");
    assert_eq!(metadata.keywords.len(), 2);
}

#[test]
fn test_export_config_creation() {
    let config = ExportConfig {
        format: ExportFormat::Pdf,
        metadata: DocumentMetadata::default(),
        include_toc: true,
        include_page_numbers: true,
        compress_images: true,
        embed_fonts: true,
        use_typst_rendering: false,
        typst_quality: TypstQuality::Standard,
    };

    assert_eq!(config.format, ExportFormat::Pdf);
    assert!(config.include_toc);
    assert!(config.include_page_numbers);
    assert!(config.compress_images);
    assert!(config.embed_fonts);
}

#[test]
fn test_export_format_variants() {
    let formats = vec![
        ExportFormat::Pdf,
        ExportFormat::Docx,
        ExportFormat::Html,
        ExportFormat::Markdown,
        ExportFormat::Txt,
        ExportFormat::Svg,
        ExportFormat::Png,
    ];

    for format in formats {
        let _ = format;
    }
}

#[test]
fn test_batch_export() {
    let mut generator = ExportGenerator::new();
    let document_content = "# Test Document\n\nContent here.";

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
            use_typst_rendering: false,
            typst_quality: TypstQuality::Standard,
        };

        let result = generator.export(document_content, &batch_config);
        assert!(result.is_ok());
        let export_result = result.unwrap();
        assert!(export_result.success);
        assert_eq!(export_result.format, format);
    }
}
