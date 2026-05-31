use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Pdf,
    Docx,
    Pptx,
    Xlsx,
    Html,
    Markdown,
    Rtf,
    Epub,
    Odt,
    Txt,
    Svg,
    Png,
}

/// Typst 渲染质量级别
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TypstQuality {
    /// 标准质量 - 基础排版
    Standard,
    /// 高质量 - 启用高级排版特性
    High,
    /// 航空航天级 - 所有高级特性
    Aerospace,
}

impl Default for TypstQuality {
    fn default() -> Self {
        Self::Standard
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub title: String,
    pub author: String,
    pub subject: String,
    pub keywords: Vec<String>,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        Self {
            title: "Untitled Document".to_string(),
            author: "Unknown".to_string(),
            subject: String::new(),
            keywords: Vec::new(),
            created: chrono::Utc::now(),
            modified: chrono::Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_format_variants() {
        let pdf = ExportFormat::Pdf;
        let docx = ExportFormat::Docx;
        let html = ExportFormat::Html;
        let markdown = ExportFormat::Markdown;
        let rtf = ExportFormat::Rtf;
        let epub = ExportFormat::Epub;
        let odt = ExportFormat::Odt;
        let txt = ExportFormat::Txt;
        let svg = ExportFormat::Svg;
        let png = ExportFormat::Png;

        assert_eq!(pdf, ExportFormat::Pdf);
        assert_eq!(docx, ExportFormat::Docx);
        assert_eq!(html, ExportFormat::Html);
        assert_eq!(markdown, ExportFormat::Markdown);
        assert_eq!(rtf, ExportFormat::Rtf);
        assert_eq!(epub, ExportFormat::Epub);
        assert_eq!(odt, ExportFormat::Odt);
        assert_eq!(txt, ExportFormat::Txt);
        assert_eq!(svg, ExportFormat::Svg);
        assert_eq!(png, ExportFormat::Png);
    }

    #[test]
    fn test_export_format_pdf_serialization() {
        let format = ExportFormat::Pdf;
        let json = serde_json::to_string(&format);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"pdf\"");
    }

    #[test]
    fn test_export_format_docx_serialization() {
        let format = ExportFormat::Docx;
        let json = serde_json::to_string(&format);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"docx\"");
    }

    #[test]
    fn test_export_format_html_serialization() {
        let format = ExportFormat::Html;
        let json = serde_json::to_string(&format);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"html\"");
    }

    #[test]
    fn test_export_format_markdown_serialization() {
        let format = ExportFormat::Markdown;
        let json = serde_json::to_string(&format);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"markdown\"");
    }

    #[test]
    fn test_export_format_rtf_serialization() {
        let format = ExportFormat::Rtf;
        let json = serde_json::to_string(&format);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"rtf\"");
    }

    #[test]
    fn test_export_format_epub_serialization() {
        let format = ExportFormat::Epub;
        let json = serde_json::to_string(&format);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"epub\"");
    }

    #[test]
    fn test_export_format_odt_serialization() {
        let format = ExportFormat::Odt;
        let json = serde_json::to_string(&format);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"odt\"");
    }

    #[test]
    fn test_export_format_txt_serialization() {
        let format = ExportFormat::Txt;
        let json = serde_json::to_string(&format);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"txt\"");
    }

    #[test]
    fn test_export_format_deserialization() {
        let format: ExportFormat = serde_json::from_str("\"pdf\"").unwrap();
        assert_eq!(format, ExportFormat::Pdf);
    }

    #[test]
    fn test_export_format_all_formats_deserialization() {
        let formats = vec![
            "pdf", "docx", "html", "markdown", "rtf", "epub", "odt", "txt", "svg", "png",
        ];
        for format_str in formats {
            let format: ExportFormat =
                serde_json::from_str(&format!("\"{}\"", format_str)).unwrap();
            let _ = format;
        }
    }

    #[test]
    fn test_export_format_svg_serialization() {
        let format = ExportFormat::Svg;
        let json = serde_json::to_string(&format);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"svg\"");
    }

    #[test]
    fn test_export_format_png_serialization() {
        let format = ExportFormat::Png;
        let json = serde_json::to_string(&format);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"png\"");
    }

    #[test]
    fn test_document_metadata_creation() {
        let created = chrono::Utc::now();
        let modified = chrono::Utc::now();
        let metadata = DocumentMetadata {
            title: "Test Document".to_string(),
            author: "Test Author".to_string(),
            subject: "Test Subject".to_string(),
            keywords: vec!["test".to_string(), "document".to_string()],
            created,
            modified,
        };
        assert_eq!(metadata.title, "Test Document");
        assert_eq!(metadata.author, "Test Author");
        assert_eq!(metadata.subject, "Test Subject");
        assert_eq!(metadata.keywords.len(), 2);
    }

    #[test]
    fn test_document_metadata_default() {
        let metadata = DocumentMetadata::default();
        assert_eq!(metadata.title, "Untitled Document");
        assert_eq!(metadata.author, "Unknown");
        assert_eq!(metadata.subject, "");
        assert!(metadata.keywords.is_empty());
    }

    #[test]
    fn test_document_metadata_serialization() {
        let created = chrono::Utc::now();
        let modified = chrono::Utc::now();
        let metadata = DocumentMetadata {
            title: "Test Document".to_string(),
            author: "Test Author".to_string(),
            subject: "Test Subject".to_string(),
            keywords: vec!["test".to_string()],
            created,
            modified,
        };
        let json = serde_json::to_string(&metadata);
        assert!(json.is_ok());
    }

    #[test]
    fn test_document_metadata_deserialization() {
        let json = r#"{
            "title": "Test Document",
            "author": "Test Author",
            "subject": "Test Subject",
            "keywords": ["test"],
            "created": "2024-01-01T00:00:00Z",
            "modified": "2024-01-01T00:00:00Z"
        }"#;
        let metadata: Result<DocumentMetadata, _> = serde_json::from_str(json);
        assert!(metadata.is_ok());
    }

    #[test]
    fn test_document_metadata_empty_keywords() {
        let metadata = DocumentMetadata {
            title: "Test".to_string(),
            author: "Test".to_string(),
            subject: "Test".to_string(),
            keywords: vec![],
            created: chrono::Utc::now(),
            modified: chrono::Utc::now(),
        };
        assert!(metadata.keywords.is_empty());
    }

    #[test]
    fn test_document_metadata_multiple_keywords() {
        let metadata = DocumentMetadata {
            title: "Test".to_string(),
            author: "Test".to_string(),
            subject: "Test".to_string(),
            keywords: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            created: chrono::Utc::now(),
            modified: chrono::Utc::now(),
        };
        assert_eq!(metadata.keywords.len(), 3);
    }

    #[test]
    fn test_document_metadata_clone() {
        let metadata = DocumentMetadata {
            title: "Test".to_string(),
            author: "Test".to_string(),
            subject: "Test".to_string(),
            keywords: vec!["test".to_string()],
            created: chrono::Utc::now(),
            modified: chrono::Utc::now(),
        };
        let cloned = metadata.clone();
        assert_eq!(metadata.title, cloned.title);
        assert_eq!(metadata.author, cloned.author);
    }

    #[test]
    fn test_export_format_equality() {
        assert_eq!(ExportFormat::Pdf, ExportFormat::Pdf);
        assert_ne!(ExportFormat::Pdf, ExportFormat::Docx);
    }

    #[test]
    fn test_document_metadata_empty_strings() {
        let metadata = DocumentMetadata {
            title: "".to_string(),
            author: "".to_string(),
            subject: "".to_string(),
            keywords: vec![],
            created: chrono::Utc::now(),
            modified: chrono::Utc::now(),
        };
        assert_eq!(metadata.title, "");
        assert_eq!(metadata.author, "");
        assert_eq!(metadata.subject, "");
    }

    #[test]
    fn test_document_metadata_long_title() {
        let long_title = "a".repeat(1000);
        let metadata = DocumentMetadata {
            title: long_title.clone(),
            author: "Test".to_string(),
            subject: "Test".to_string(),
            keywords: vec![],
            created: chrono::Utc::now(),
            modified: chrono::Utc::now(),
        };
        assert_eq!(metadata.title.len(), 1000);
    }
}
