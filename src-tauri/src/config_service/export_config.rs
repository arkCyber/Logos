//! Export Configuration Service - Aerospace-Grade Configuration Management
//!
//! Manages export service limits and configuration from TOML files

use super::error::ConfigError;
use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

/// Export limits configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ExportLimits {
    /// Maximum input content size in bytes
    pub max_content_length: usize,
    /// Maximum output file size in bytes
    pub max_output_size: usize,
    /// Maximum metadata field length in characters
    pub max_metadata_length: usize,
}

impl Default for ExportLimits {
    fn default() -> Self {
        Self {
            max_content_length: 524_288_000,
            max_output_size: 1_073_741_824,
            max_metadata_length: 10_000,
        }
    }
}

/// Typst configuration
#[derive(Debug, Clone, Deserialize)]
pub struct TypstConfig {
    /// Typst cache configuration
    pub cache: TypstCacheConfig,
    /// Default quality level
    #[serde(default = "default_typst_quality")]
    pub default_quality: String,
    /// Maximum input size in bytes
    pub max_input_size: usize,
    /// Maximum recursion depth
    pub max_recursion_depth: usize,
    /// Maximum number of slides
    pub max_slides: usize,
}

fn default_typst_quality() -> String { "standard".to_string() }

impl Default for TypstConfig {
    fn default() -> Self {
        Self {
            cache: TypstCacheConfig::default(),
            default_quality: default_typst_quality(),
            max_input_size: 10 * 1024 * 1024,
            max_recursion_depth: 100,
            max_slides: 500,
        }
    }
}

/// Typst cache configuration
#[derive(Debug, Clone, Deserialize)]
pub struct TypstCacheConfig {
    /// Maximum number of cached entries
    pub max_entries: usize,
}

impl Default for TypstCacheConfig {
    fn default() -> Self {
        Self { max_entries: 100 }
    }
}

/// PDF conversion configuration
#[derive(Debug, Clone, Deserialize)]
pub struct PdfConversionConfig {
    /// Maximum PDF file size for conversion
    pub max_pdf_size: usize,
    /// Conversion timeout in seconds
    pub conversion_timeout_seconds: u64,
}

impl Default for PdfConversionConfig {
    fn default() -> Self {
        Self {
            max_pdf_size: 524_288_000,
            conversion_timeout_seconds: 300,
        }
    }
}

/// DOCX export configuration
#[derive(Debug, Clone, Deserialize)]
pub struct DocxConfig {
    /// Maximum HTML content length in bytes
    pub max_html_length: usize,
    /// Maximum Markdown content length in bytes
    pub max_markdown_length: usize,
    /// Maximum paragraph count
    pub max_paragraph_count: usize,
    /// Maximum table count
    pub max_table_count: usize,
    /// Maximum image count
    pub max_image_count: usize,
}

impl Default for DocxConfig {
    fn default() -> Self {
        Self {
            max_html_length: 52_428_800,
            max_markdown_length: 52_428_800,
            max_paragraph_count: 10_000,
            max_table_count: 1_000,
            max_image_count: 500,
        }
    }
}

/// PPTX export configuration
#[derive(Debug, Clone, Deserialize)]
pub struct PptxConfig {
    /// Maximum HTML content length in bytes
    pub max_html_length: usize,
    /// Maximum Markdown content length in bytes
    pub max_markdown_length: usize,
    /// Maximum slide count
    pub max_slide_count: usize,
    /// Maximum output file size in bytes
    pub max_output_size: usize,
    /// Media configuration
    pub media: PptxMediaConfig,
}

impl Default for PptxConfig {
    fn default() -> Self {
        Self {
            max_html_length: 104_857_600,
            max_markdown_length: 104_857_600,
            max_slide_count: 1_000,
            max_output_size: 524_288_000,
            media: PptxMediaConfig::default(),
        }
    }
}

/// PPTX media configuration
#[derive(Debug, Clone, Deserialize)]
pub struct PptxMediaConfig {
    /// Maximum audio size in MB
    pub max_audio_size_mb: usize,
    /// Maximum video size in MB
    pub max_video_size_mb: usize,
    /// Maximum media duration in seconds
    pub max_duration_seconds: f64,
}

impl Default for PptxMediaConfig {
    fn default() -> Self {
        Self {
            max_audio_size_mb: 100,
            max_video_size_mb: 500,
            max_duration_seconds: 3600.0,
        }
    }
}

/// EPUB export configuration
#[derive(Debug, Clone, Deserialize)]
pub struct EpubConfig {
    /// Maximum HTML content length in bytes
    pub max_html_length: usize,
    /// Maximum Markdown content length in bytes
    pub max_markdown_length: usize,
    /// Maximum chapter count
    pub max_chapter_count: usize,
    /// Maximum title length in characters
    pub max_title_length: usize,
}

impl Default for EpubConfig {
    fn default() -> Self {
        Self {
            max_html_length: 52_428_800,
            max_markdown_length: 52_428_800,
            max_chapter_count: 1_000,
            max_title_length: 500,
        }
    }
}

/// ODT export configuration
#[derive(Debug, Clone, Deserialize)]
pub struct OdtConfig {
    /// Maximum HTML content length in bytes
    pub max_html_length: usize,
    /// Maximum Markdown content length in bytes
    pub max_markdown_length: usize,
    /// Maximum paragraph count
    pub max_paragraph_count: usize,
    /// Maximum output file size in bytes
    pub max_output_size: usize,
}

impl Default for OdtConfig {
    fn default() -> Self {
        Self {
            max_html_length: 52_428_800,
            max_markdown_length: 52_428_800,
            max_paragraph_count: 10_000,
            max_output_size: 104_857_600,
        }
    }
}

/// RTF export configuration
#[derive(Debug, Clone, Deserialize)]
pub struct RtfConfig {
    /// Maximum HTML content length in bytes
    pub max_html_length: usize,
    /// Maximum Markdown content length in bytes
    pub max_markdown_length: usize,
    /// Maximum paragraph count
    pub max_paragraph_count: usize,
    /// Maximum output file size in bytes
    pub max_output_size: usize,
}

impl Default for RtfConfig {
    fn default() -> Self {
        Self {
            max_html_length: 52_428_800,
            max_markdown_length: 52_428_800,
            max_paragraph_count: 10_000,
            max_output_size: 104_857_600,
        }
    }
}

/// PNG export configuration
#[derive(Debug, Clone, Deserialize)]
pub struct PngConfig {
    /// Maximum HTML content length in bytes
    pub max_html_length: usize,
    /// Maximum text content length in bytes
    pub max_text_length: usize,
    /// Maximum output file size in bytes
    pub max_output_size: usize,
}

impl Default for PngConfig {
    fn default() -> Self {
        Self {
            max_html_length: 104_857_600,
            max_text_length: 104_857_600,
            max_output_size: 104_857_600,
        }
    }
}

/// SVG export configuration
#[derive(Debug, Clone, Deserialize)]
pub struct SvgConfig {
    /// Maximum HTML content length in bytes
    pub max_html_length: usize,
    /// Maximum element count
    pub max_element_count: usize,
    /// Maximum output file size in bytes
    pub max_output_size: usize,
}

impl Default for SvgConfig {
    fn default() -> Self {
        Self {
            max_html_length: 52_428_800,
            max_element_count: 10_000,
            max_output_size: 104_857_600,
        }
    }
}

/// Markdown conversion configuration
#[derive(Debug, Clone, Deserialize)]
pub struct MarkdownConfig {
    /// Maximum input size in bytes
    pub max_input_size: usize,
    /// Maximum recursion depth
    pub max_recursion_depth: usize,
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            max_input_size: 104_857_600,
            max_recursion_depth: 100,
        }
    }
}

/// AI service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct AiConfig {
    /// Maximum prompt length in characters
    pub max_prompt_length: usize,
    /// Maximum text length in characters
    pub max_text_length: usize,
    /// Maximum stream chunks
    pub max_stream_chunks: usize,
    /// Maximum API key length in characters
    pub max_api_key_length: usize,
    /// Maximum API URL length in characters
    pub max_api_url_length: usize,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            max_prompt_length: 10_000,
            max_text_length: 100_000,
            max_stream_chunks: 10_000,
            max_api_key_length: 256,
            max_api_url_length: 512,
        }
    }
}

/// Chart service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ChartConfig {
    /// Maximum data points
    pub max_data_points: usize,
    /// Maximum chart width in pixels
    pub max_chart_width: u32,
    /// Maximum chart height in pixels
    pub max_chart_height: u32,
    /// Maximum title length in characters
    pub max_title_length: usize,
    /// Maximum label length in characters
    pub max_label_length: usize,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            max_data_points: 1_000,
            max_chart_width: 10_000,
            max_chart_height: 10_000,
            max_title_length: 500,
            max_label_length: 100,
        }
    }
}

/// OCR service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct OcrConfig {
    /// Maximum image size in bytes
    pub max_image_size: usize,
    /// Maximum image dimension in pixels
    pub max_image_dimension: u32,
    /// Minimum image dimension in pixels
    pub min_image_dimension: u32,
    /// Maximum rotation angle in degrees
    pub max_rotation_angle: f32,
}

impl Default for OcrConfig {
    fn default() -> Self {
        Self {
            max_image_size: 524_288_000,
            max_image_dimension: 32_768,
            min_image_dimension: 1,
            max_rotation_angle: 360.0,
        }
    }
}

/// Mail merge service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct MailMergeConfig {
    /// Maximum file size in bytes
    pub max_file_size: usize,
    /// Maximum number of records
    pub max_records: usize,
    /// Maximum fields per record
    pub max_fields_per_record: usize,
    /// Maximum batch size
    pub max_batch_size: usize,
    /// Maximum field name length in characters
    pub max_field_name_length: usize,
    /// Maximum field value length in characters
    pub max_field_value_length: usize,
}

impl Default for MailMergeConfig {
    fn default() -> Self {
        Self {
            max_file_size: 524_288_000,
            max_records: 10_000,
            max_fields_per_record: 500,
            max_batch_size: 1_000,
            max_field_name_length: 200,
            max_field_value_length: 10_000,
        }
    }
}

/// Collaboration service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct CollaborationConfig {
    /// Maximum content length in bytes
    pub max_content_length: usize,
    /// Maximum operation content length in characters
    pub max_operation_content_length: usize,
    /// Maximum number of operations
    pub max_operations: usize,
    /// Maximum number of authors
    pub max_authors: usize,
    /// Maximum document ID length in characters
    pub max_document_id_length: usize,
    /// Maximum author ID length in characters
    pub max_author_id_length: usize,
}

impl Default for CollaborationConfig {
    fn default() -> Self {
        Self {
            max_content_length: 104_857_600,
            max_operation_content_length: 10_000,
            max_operations: 100_000,
            max_authors: 1_000,
            max_document_id_length: 256,
            max_author_id_length: 128,
        }
    }
}

/// Comments service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct CommentsConfig {
    /// Maximum comment length in characters
    pub max_comment_length: usize,
    /// Maximum title length in characters
    pub max_title_length: usize,
    /// Maximum mentions per comment
    pub max_mentions: usize,
    /// Maximum attachments per comment
    pub max_attachments: usize,
}

impl Default for CommentsConfig {
    fn default() -> Self {
        Self {
            max_comment_length: 10_000,
            max_title_length: 500,
            max_mentions: 100,
            max_attachments: 10,
        }
    }
}

/// Math service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct MathConfig {
    /// Maximum LaTeX expression length in characters
    pub max_latex_length: usize,
    /// Maximum nesting depth
    pub max_nesting_depth: usize,
    /// Maximum command length in characters
    pub max_command_length: usize,
}

impl Default for MathConfig {
    fn default() -> Self {
        Self {
            max_latex_length: 100_000,
            max_nesting_depth: 100,
            max_command_length: 50,
        }
    }
}

/// PDF service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct PdfConfig {
    /// Maximum content length in bytes
    pub max_content_length: usize,
    /// Maximum metadata field length in characters
    pub max_metadata_length: usize,
    /// Maximum output PDF size in bytes
    pub max_pdf_size: usize,
    /// Maximum page count
    pub max_page_count: usize,
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            max_content_length: 100 * 1024 * 1024,
            max_metadata_length: 1000,
            max_pdf_size: 500 * 1024 * 1024,
            max_page_count: 10000,
        }
    }
}

/// Macro service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct MacroConfig {
    /// Maximum macro name length in characters
    pub max_macro_name_length: usize,
    /// Maximum description length in characters
    pub max_description_length: usize,
    /// Maximum actions per macro
    pub max_actions_per_macro: usize,
    /// Maximum number of macros
    pub max_macros: usize,
    /// Maximum delay in milliseconds
    pub max_delay_ms: u64,
}

impl Default for MacroConfig {
    fn default() -> Self {
        Self {
            max_macro_name_length: 200,
            max_description_length: 1_000,
            max_actions_per_macro: 1_000,
            max_macros: 500,
            max_delay_ms: 60_000,
        }
    }
}

/// Diff service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct DiffConfig {
    /// Maximum text length in bytes
    pub max_text_length: usize,
    /// Maximum line count
    pub max_line_count: usize,
    /// Maximum line length in characters
    pub max_line_length: usize,
}

impl Default for DiffConfig {
    fn default() -> Self {
        Self {
            max_text_length: 104_857_600,
            max_line_count: 100_000,
            max_line_length: 10_000,
        }
    }
}

/// Cloud service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct CloudConfig {
    /// Maximum file size in bytes
    pub max_file_size: u64,
    /// Maximum file path length in characters
    pub max_file_path_length: usize,
    /// Maximum file name length in characters
    pub max_file_name_length: usize,
    /// Maximum API key/token length
    pub max_token_length: usize,
}

impl Default for CloudConfig {
    fn default() -> Self {
        Self {
            max_file_size: 104_857_600,
            max_file_path_length: 512,
            max_file_name_length: 256,
            max_token_length: 512,
        }
    }
}

/// Accessibility service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct AccessibilityConfig {
    /// Maximum tree nodes
    pub max_tree_nodes: usize,
    /// Maximum tree depth
    pub max_tree_depth: usize,
    /// Maximum node ID length in characters
    pub max_node_id_length: usize,
    /// Maximum text length in characters
    pub max_text_length: usize,
}

impl Default for AccessibilityConfig {
    fn default() -> Self {
        Self {
            max_tree_nodes: 10_000,
            max_tree_depth: 100,
            max_node_id_length: 256,
            max_text_length: 1_000,
        }
    }
}

/// Plugin service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct PluginConfig {
    /// Maximum plugin ID length in characters
    pub max_plugin_id_length: usize,
    /// Maximum plugin name length in characters
    pub max_plugin_name_length: usize,
    /// Maximum number of plugins
    pub max_plugins: usize,
    /// Maximum hooks per plugin
    pub max_hooks_per_plugin: usize,
    /// Maximum config key length in characters
    pub max_config_key_length: usize,
    /// Maximum config value length in characters
    pub max_config_value_length: usize,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            max_plugin_id_length: 200,
            max_plugin_name_length: 500,
            max_plugins: 100,
            max_hooks_per_plugin: 50,
            max_config_key_length: 200,
            max_config_value_length: 10_000,
        }
    }
}

/// Error handling configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorHandlingConfig {
    /// Maximum retry attempts
    pub max_retry_attempts: u32,
    /// Circuit breaker failure threshold
    pub circuit_breaker_threshold: u32,
    /// Circuit breaker timeout in seconds
    pub circuit_breaker_timeout_secs: u64,
}

impl Default for ErrorHandlingConfig {
    fn default() -> Self {
        Self {
            max_retry_attempts: 3,
            circuit_breaker_threshold: 5,
            circuit_breaker_timeout_secs: 60,
        }
    }
}

/// Editing engine configuration
#[derive(Debug, Clone, Deserialize)]
pub struct EditingEngineConfig {
    /// Maximum input size in bytes
    pub max_input_size: usize,
    /// Maximum recursion depth
    pub max_recursion_depth: usize,
    /// Maximum heading level
    pub max_heading_level: u64,
    /// Maximum table columns
    pub max_table_columns: usize,
    /// Maximum table rows
    pub max_table_rows: usize,
    /// Maximum list items
    pub max_list_items: usize,
}

impl Default for EditingEngineConfig {
    fn default() -> Self {
        Self {
            max_input_size: 10 * 1024 * 1024,
            max_recursion_depth: 100,
            max_heading_level: 6,
            max_table_columns: 100,
            max_table_rows: 1000,
            max_list_items: 1000,
        }
    }
}

/// Typist service configuration
#[derive(Debug, Clone, Deserialize)]
pub struct TypistConfig {
    /// Maximum source code length in bytes
    pub max_source_length: usize,
    /// Maximum output size in bytes
    pub max_output_size: usize,
}

impl Default for TypistConfig {
    fn default() -> Self {
        Self {
            max_source_length: 10 * 1024 * 1024,
            max_output_size: 100 * 1024 * 1024,
        }
    }
}

/// Complete export configuration
#[derive(Debug, Clone, Deserialize)]
pub struct ExportConfigFile {
    /// Export limits
    pub limits: ExportLimits,
    /// Typst configuration
    pub typst: TypstConfig,
    /// PDF conversion configuration
    pub pdf_conversion: PdfConversionConfig,
    /// PDF configuration
    pub pdf: PdfConfig,
    /// DOCX configuration
    pub docx: DocxConfig,
    /// PPTX configuration
    pub pptx: PptxConfig,
    /// EPUB configuration
    pub epub: EpubConfig,
    /// ODT configuration
    pub odt: OdtConfig,
    /// RTF configuration
    pub rtf: RtfConfig,
    /// PNG configuration
    pub png: PngConfig,
    /// SVG configuration
    pub svg: SvgConfig,
    /// Markdown configuration
    pub markdown: MarkdownConfig,
    /// AI configuration
    pub ai: AiConfig,
    /// Chart configuration
    pub chart: ChartConfig,
    /// OCR configuration
    pub ocr: OcrConfig,
    /// Mail merge configuration
    pub mail_merge: MailMergeConfig,
    /// Collaboration configuration
    pub collaboration: CollaborationConfig,
    /// Comments configuration
    pub comments: CommentsConfig,
    /// Math configuration
    pub math: MathConfig,
    /// Macro configuration
    pub r#macro: MacroConfig,
    /// Diff configuration
    pub diff: DiffConfig,
    /// Cloud configuration
    pub cloud: CloudConfig,
    /// Accessibility configuration
    pub accessibility: AccessibilityConfig,
    /// Plugin configuration
    pub plugin: PluginConfig,
    /// Error handling configuration
    pub error_handling: ErrorHandlingConfig,
    /// Editing engine configuration
    pub editing_engine: EditingEngineConfig,
    /// Typist service configuration
    pub typist: TypistConfig,
}

impl Default for ExportConfigFile {
    fn default() -> Self {
        Self {
            limits: ExportLimits::default(),
            typst: TypstConfig::default(),
            pdf_conversion: PdfConversionConfig::default(),
            pdf: PdfConfig::default(),
            docx: DocxConfig::default(),
            pptx: PptxConfig::default(),
            epub: EpubConfig::default(),
            odt: OdtConfig::default(),
            rtf: RtfConfig::default(),
            png: PngConfig::default(),
            svg: SvgConfig::default(),
            markdown: MarkdownConfig::default(),
            ai: AiConfig::default(),
            chart: ChartConfig::default(),
            ocr: OcrConfig::default(),
            mail_merge: MailMergeConfig::default(),
            collaboration: CollaborationConfig::default(),
            comments: CommentsConfig::default(),
            math: MathConfig::default(),
            r#macro: MacroConfig::default(),
            diff: DiffConfig::default(),
            cloud: CloudConfig::default(),
            accessibility: AccessibilityConfig::default(),
            plugin: PluginConfig::default(),
            error_handling: ErrorHandlingConfig::default(),
            editing_engine: EditingEngineConfig::default(),
            typist: TypistConfig::default(),
        }
    }
}

/// Export configuration service
#[derive(Debug)]
pub struct ExportConfigService {
    config: Arc<RwLock<ExportConfigFile>>,
    config_path: Option<String>,
}

impl ExportConfigService {
    /// Create a new configuration service with default values
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(ExportConfigFile::default())),
            config_path: None,
        }
    }

    /// Load configuration from a TOML file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(ConfigError::FileNotFound(
                path.to_string_lossy().to_string(),
            ));
        }

        let content = std::fs::read_to_string(path).map_err(|e| {
            ConfigError::IoError(format!("Failed to read config file: {}", e))
        })?;

        // TODO: Fix serde deserialization for ExportConfigFile
        let config: ExportConfigFile = toml::from_str(&content).map_err(|e| {
            ConfigError::ParseFailed(format!("Failed to parse TOML: {}", e))
        })?;
        // let config = ExportConfigFile::default();

        // Validate configuration
        Self::validate_config(&config)?;

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            config_path: Some(path.to_string_lossy().to_string()),
        })
    }

    /// Validate configuration values
    fn validate_config(config: &ExportConfigFile) -> Result<(), ConfigError> {
        // Validate limits
        if config.limits.max_content_length == 0 {
            return Err(ConfigError::InvalidValue(
                "max_content_length must be greater than 0".to_string(),
            ));
        }

        if config.limits.max_output_size == 0 {
            return Err(ConfigError::InvalidValue(
                "max_output_size must be greater than 0".to_string(),
            ));
        }

        if config.limits.max_metadata_length == 0 {
            return Err(ConfigError::InvalidValue(
                "max_metadata_length must be greater than 0".to_string(),
            ));
        }

        // Validate Typst cache
        if config.typst.cache.max_entries == 0 {
            return Err(ConfigError::InvalidValue(
                "typst.cache.max_entries must be greater than 0".to_string(),
            ));
        }

        // Validate PDF conversion
        if config.pdf_conversion.max_pdf_size == 0 {
            return Err(ConfigError::InvalidValue(
                "pdf_conversion.max_pdf_size must be greater than 0".to_string(),
            ));
        }

        if config.pdf_conversion.conversion_timeout_seconds == 0 {
            return Err(ConfigError::InvalidValue(
                "pdf_conversion.conversion_timeout_seconds must be greater than 0".to_string(),
            ));
        }

        // Validate quality level
        let valid_qualities = ["standard", "high", "aerospace"];
        if !valid_qualities.contains(&config.typst.default_quality.as_str()) {
            return Err(ConfigError::InvalidValue(format!(
                "Invalid typst.default_quality: {}. Must be one of: {}",
                config.typst.default_quality,
                valid_qualities.join(", ")
            )));
        }

        Ok(())
    }

    /// Get current configuration
    pub fn get_config(&self) -> ExportConfigFile {
        self.config.read().ok().map(|c| c.clone()).unwrap_or_default()
    }

    /// Get export limits
    pub fn get_limits(&self) -> ExportLimits {
        self.config
            .read()
            .ok()
            .map(|c| c.limits.clone())
            .unwrap_or_default()
    }

    /// Get Typst configuration
    pub fn get_typst_config(&self) -> TypstConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.typst.clone())
            .unwrap_or_default()
    }

    /// Get PDF conversion configuration
    pub fn get_pdf_conversion_config(&self) -> PdfConversionConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.pdf_conversion.clone())
            .unwrap_or_default()
    }

    /// Get PDF configuration
    pub fn get_pdf_config(&self) -> PdfConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.pdf.clone())
            .unwrap_or_default()
    }

    /// Get DOCX configuration
    pub fn get_docx_config(&self) -> DocxConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.docx.clone())
            .unwrap_or_default()
    }

    /// Get PPTX configuration
    pub fn get_pptx_config(&self) -> PptxConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.pptx.clone())
            .unwrap_or_default()
    }

    /// Get EPUB configuration
    pub fn get_epub_config(&self) -> EpubConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.epub.clone())
            .unwrap_or_default()
    }

    /// Get ODT configuration
    pub fn get_odt_config(&self) -> OdtConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.odt.clone())
            .unwrap_or_default()
    }

    /// Get RTF configuration
    pub fn get_rtf_config(&self) -> RtfConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.rtf.clone())
            .unwrap_or_default()
    }

    /// Get PNG configuration
    pub fn get_png_config(&self) -> PngConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.png.clone())
            .unwrap_or_default()
    }

    /// Get SVG configuration
    pub fn get_svg_config(&self) -> SvgConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.svg.clone())
            .unwrap_or_default()
    }

    /// Get Markdown configuration
    pub fn get_markdown_config(&self) -> MarkdownConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.markdown.clone())
            .unwrap_or_default()
    }

    /// Get AI configuration
    pub fn get_ai_config(&self) -> AiConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.ai.clone())
            .unwrap_or_default()
    }

    /// Get Chart configuration
    pub fn get_chart_config(&self) -> ChartConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.chart.clone())
            .unwrap_or_default()
    }

    /// Get OCR configuration
    pub fn get_ocr_config(&self) -> OcrConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.ocr.clone())
            .unwrap_or_default()
    }

    /// Get Mail merge configuration
    pub fn get_mail_merge_config(&self) -> MailMergeConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.mail_merge.clone())
            .unwrap_or_default()
    }

    /// Get Collaboration configuration
    pub fn get_collaboration_config(&self) -> CollaborationConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.collaboration.clone())
            .unwrap_or_default()
    }

    /// Get Comments configuration
    pub fn get_comments_config(&self) -> CommentsConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.comments.clone())
            .unwrap_or_default()
    }

    /// Get Math configuration
    pub fn get_math_config(&self) -> MathConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.math.clone())
            .unwrap_or_default()
    }

    /// Get Macro configuration
    pub fn get_macro_config(&self) -> MacroConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.r#macro.clone())
            .unwrap_or_default()
    }

    /// Get Diff configuration
    pub fn get_diff_config(&self) -> DiffConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.diff.clone())
            .unwrap_or_default()
    }

    /// Get Cloud configuration
    pub fn get_cloud_config(&self) -> CloudConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.cloud.clone())
            .unwrap_or_default()
    }

    /// Get Accessibility configuration
    pub fn get_accessibility_config(&self) -> AccessibilityConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.accessibility.clone())
            .unwrap_or_default()
    }

    /// Get Plugin configuration
    pub fn get_plugin_config(&self) -> PluginConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.plugin.clone())
            .unwrap_or_default()
    }

    /// Get Error handling configuration
    pub fn get_error_handling_config(&self) -> ErrorHandlingConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.error_handling.clone())
            .unwrap_or_default()
    }

    /// Get Editing engine configuration
    pub fn get_editing_engine_config(&self) -> EditingEngineConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.editing_engine.clone())
            .unwrap_or_default()
    }

    /// Get Typist service configuration
    pub fn get_typist_config(&self) -> TypistConfig {
        self.config
            .read()
            .ok()
            .map(|c| c.typist.clone())
            .unwrap_or_default()
    }

    /// Reload configuration from file
    pub fn reload(&mut self) -> Result<(), ConfigError> {
        if let Some(ref path) = self.config_path {
            let new_service = Self::load_from_file(path)?;
            self.config = new_service.config;
            Ok(())
        } else {
            Err(ConfigError::FileNotFound(
                "No configuration file path set".to_string(),
            ))
        }
    }
}

impl Default for ExportConfigService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let service = ExportConfigService::new();
        let config = service.get_config();
        
        assert_eq!(config.limits.max_content_length, 524_288_000);
        assert_eq!(config.limits.max_output_size, 1_073_741_824);
        assert_eq!(config.limits.max_metadata_length, 10_000);
    }

    #[test]
    fn test_get_limits() {
        let service = ExportConfigService::new();
        let limits = service.get_limits();
        
        assert_eq!(limits.max_content_length, 524_288_000);
        assert_eq!(limits.max_output_size, 1_073_741_824);
    }

    #[test]
    fn test_get_typst_config() {
        let service = ExportConfigService::new();
        let typst_config = service.get_typst_config();
        
        assert_eq!(typst_config.cache.max_entries, 100);
        assert_eq!(typst_config.default_quality, "standard");
    }

    #[test]
    fn test_get_pdf_conversion_config() {
        let service = ExportConfigService::new();
        let pdf_config = service.get_pdf_conversion_config();
        
        assert_eq!(pdf_config.max_pdf_size, 524_288_000);
        assert_eq!(pdf_config.conversion_timeout_seconds, 300);
    }

    #[test]
    fn test_validate_config_valid() {
        let config = ExportConfigFile::default();
        assert!(ExportConfigService::validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_invalid_zero() {
        let mut config = ExportConfigFile::default();
        config.limits.max_content_length = 0;
        assert!(ExportConfigService::validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_config_invalid_quality() {
        let mut config = ExportConfigFile::default();
        config.typst.default_quality = "invalid".to_string();
        assert!(ExportConfigService::validate_config(&config).is_err());
    }
}
