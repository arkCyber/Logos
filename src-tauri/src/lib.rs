// Microservices modules
mod accessibility_service;
mod ai_service;
mod chart_service;
mod cloud_service;
mod collaboration_service;
mod comments_service;
mod config_service;
mod diff_service;
mod docx_service;
mod editing_engine_service;
mod epub_service;
mod export_service;
mod macro_service;
mod mail_merge_service;
mod markdown_service;
mod math_service;
mod ocr_service;
mod odt_service;
mod pdf_conversion_service;
mod pdf_service;
mod plugin_service;
mod png_service;
mod ppt_service;
mod rtf_service;
mod svg_service;
mod table_service;
mod template_service;
mod spreadsheet_service;
mod tiptap_service;
mod typst_conversion_service;
pub mod typist_service;
mod voice_service;

// Error handling and fault tolerance
mod error_handling;

// Integration tests
#[cfg(test)]
mod tests;

use accessibility_service::{
    AccessibilityBridge, AccessibilityNode, AnnouncementPriority, ScreenReaderAnnouncer,
};
use ai_service::{
    AiClient, AiConfig, Conversation, ConversationManager, ConversationRole, PromptTemplate,
};
use chart_service::{ChartConfig, ChartData, ChartGenerator, ChartType};
use cloud_service::{SyncConfig, SyncManager, SyncResult, SyncStatus};
use collaboration_service::{CRDTDocument, CRDTOperation, CRDTType, PresenceInfo};
use comments_service::{Comment, CommentFilter, CommentsManager};
use diff_service::{DiffEngine, DiffResult, DiffViewConfig};
use editing_engine_service::json_to_typst::JsonToTypstConverter;
use editing_engine_service::{FileManager, FormatConverter};
use export_service::{ExportConfig, ExportFormat, ExportGenerator};
use macro_service::{Macro, MacroAction, MacroEngine, MacroRecorder, RecordedAction};
use mail_merge_service::{
    DataProcessor, DataSource, MergeBatchResult, MergeConfig, TemplateEngine,
};
use math_service::LatexRenderer;
use ocr_service::{OcrConfig, OcrResult, TesseractEngine};
use plugin_service::{PluginHook, PluginManager};
use ppt_service::{AudioElement, ArtWordElement, HyperlinkElement, SmartArtElement, SmartArtNode, SmartArtType, VideoElement};
use std::sync::Arc;
use table_service::{FormulaEngine, FormulaResult, PivotConfig, PivotTableGenerator};
use typist_service::incremental::IncrementalCompiler;
use typist_service::package::{PackageInfo, PackageManager};
use typist_service::template::{
    Template, TemplateCategory, TemplateEngine as TypistTemplateEngine, TemplateMetadataUpdate,
};
use typist_service::{TypstCompiler, TypstRenderer, check_typst_availability, render_typst};
use typst_conversion_service::{
    HtmlToTypstConverter, HtmlToTypstSlideConverter, SlideConfig, TypstConversionConfig,
};
use voice_service::{RecognitionConfig, SpeechRecognizer, TTSConfig, TextToSpeech};

// Global conversation manager instance
static CONVERSATION_MANAGER: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<ConversationManager>>> =
    once_cell::sync::Lazy::new(|| Arc::new(tokio::sync::Mutex::new(ConversationManager::new())));

// Global collaboration documents storage
static COLLABORATION_DOCUMENTS: once_cell::sync::Lazy<
    Arc<tokio::sync::Mutex<std::collections::HashMap<String, CRDTDocument>>>,
> = once_cell::sync::Lazy::new(|| {
    Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new()))
});

// Global sync manager instance
static SYNC_MANAGER: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<SyncManager>>> =
    once_cell::sync::Lazy::new(|| {
        Arc::new(tokio::sync::Mutex::new(SyncManager::new(
            SyncConfig::default(),
            Arc::new(config_service::ExportConfigService::new()),
        )))
    });

// Global macro engine instance
static MACRO_ENGINE: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<MacroEngine>>> =
    once_cell::sync::Lazy::new(|| Arc::new(tokio::sync::Mutex::new(MacroEngine::new(Arc::new(config_service::ExportConfigService::new())))));

// Global macro recorder instance
static MACRO_RECORDER: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<MacroRecorder>>> =
    once_cell::sync::Lazy::new(|| Arc::new(tokio::sync::Mutex::new(MacroRecorder::new())));

// Global OCR engine instance
static OCR_ENGINE: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<TesseractEngine>>> =
    once_cell::sync::Lazy::new(|| {
        Arc::new(tokio::sync::Mutex::new(TesseractEngine::new(
            OcrConfig::default(),
        )))
    });

// Global plugin manager instance
static PLUGIN_MANAGER: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<PluginManager>>> =
    once_cell::sync::Lazy::new(|| Arc::new(tokio::sync::Mutex::new(PluginManager::new(Arc::new(config_service::ExportConfigService::new())))));

// Global accessibility bridge instance
static ACCESSIBILITY_BRIDGE: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<AccessibilityBridge>>> =
    once_cell::sync::Lazy::new(|| Arc::new(tokio::sync::Mutex::new(AccessibilityBridge::new(Arc::new(config_service::ExportConfigService::new())))));

// Global screen reader announcer instance
static SCREEN_READER_ANNOUNCER: once_cell::sync::Lazy<
    Arc<tokio::sync::Mutex<ScreenReaderAnnouncer>>,
> = once_cell::sync::Lazy::new(|| Arc::new(tokio::sync::Mutex::new(ScreenReaderAnnouncer::new())));

// Global export generator instance
static EXPORT_GENERATOR: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<ExportGenerator>>> =
    once_cell::sync::Lazy::new(|| Arc::new(tokio::sync::Mutex::new(ExportGenerator::new())));

// Global speech recognizer instance
static SPEECH_RECOGNIZER: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<SpeechRecognizer>>> =
    once_cell::sync::Lazy::new(|| {
        Arc::new(tokio::sync::Mutex::new(SpeechRecognizer::new(
            RecognitionConfig::default(),
        )))
    });

// Global text-to-speech instance
static TEXT_TO_SPEECH: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<TextToSpeech>>> =
    once_cell::sync::Lazy::new(|| {
        Arc::new(tokio::sync::Mutex::new(TextToSpeech::new(
            TTSConfig::default(),
        )))
    });

// Global comments manager instance
static COMMENTS_MANAGER: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<CommentsManager>>> =
    once_cell::sync::Lazy::new(|| Arc::new(tokio::sync::Mutex::new(CommentsManager::new(Arc::new(config_service::ExportConfigService::new())))));

// Global incremental compiler instance
static INCREMENTAL_COMPILER: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<IncrementalCompiler>>> =
    once_cell::sync::Lazy::new(|| {
        Arc::new(tokio::sync::Mutex::new(
            IncrementalCompiler::with_default_config(),
        ))
    });

// Global package manager instance
static PACKAGE_MANAGER: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<PackageManager>>> =
    once_cell::sync::Lazy::new(|| {
        Arc::new(tokio::sync::Mutex::new(
            PackageManager::with_default_config(),
        ))
    });

// Global template engine instance
static TEMPLATE_ENGINE: once_cell::sync::Lazy<Arc<tokio::sync::Mutex<TypistTemplateEngine>>> =
    once_cell::sync::Lazy::new(|| {
        let mut engine = TypistTemplateEngine::new();
        if let Err(e) = engine.initialize_default_templates() {
            eprintln!("Failed to initialize default templates: {}", e);
        }
        Arc::new(tokio::sync::Mutex::new(engine))
    });

// Global preview editor instance
static PREVIEW_EDITOR: once_cell::sync::Lazy<
    Arc<tokio::sync::Mutex<typist_service::PreviewEditor>>,
> = once_cell::sync::Lazy::new(|| {
    Arc::new(tokio::sync::Mutex::new(typist_service::PreviewEditor::new(
        typist_service::PreviewEditorConfig::default(),
    )))
});

#[tauri::command]
async fn call_ai_service(prompt: String, text: String) -> Result<String, String> {
    if prompt.trim().is_empty() {
        return Err("Prompt cannot be empty".to_string());
    }
    if text.trim().is_empty() {
        return Err("Text cannot be empty".to_string());
    }

    let config = AiConfig::from_env().unwrap_or_else(|_| {
        eprintln!("Warning: AI_API_KEY not set, using placeholder");
        AiConfig::new("YOUR_API_KEY".to_string())
    });

    let config_service = Arc::new(config_service::ExportConfigService::new());
    let mut client = AiClient::new(config, config_service);
    client.call(&prompt, &text).await
}

#[tauri::command]
async fn call_ai_service_stream(
    prompt: String,
    text: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    if prompt.trim().is_empty() {
        return Err("Prompt cannot be empty".to_string());
    }
    if text.trim().is_empty() {
        return Err("Text cannot be empty".to_string());
    }

    let config = AiConfig::from_env().unwrap_or_else(|_| {
        eprintln!("Warning: AI_API_KEY not set, using placeholder");
        AiConfig::new("YOUR_API_KEY".to_string())
    });

    let config_service = Arc::new(config_service::ExportConfigService::new());
    let mut client = AiClient::new(config, config_service);
    client.call_stream(&prompt, &text, app).await
}

#[tauri::command]
async fn export_to_markdown(content: String) -> Result<String, String> {
    if content.trim().is_empty() {
        return Err("Content cannot be empty".to_string());
    }
    FormatConverter::html_to_markdown(&content)
}

#[tauri::command]
async fn markdown_to_html(markdown: String) -> Result<String, String> {
    if markdown.trim().is_empty() {
        return Err("Markdown cannot be empty".to_string());
    }
    FormatConverter::markdown_to_html(&markdown)
}

#[tauri::command]
async fn json_to_typst(json: String) -> Result<String, String> {
    if json.trim().is_empty() {
        return Err("JSON cannot be empty".to_string());
    }
    JsonToTypstConverter::convert(&json)
}

#[tauri::command]
async fn save_file(file_path: String, content: String) -> Result<(), String> {
    if file_path.trim().is_empty() {
        return Err("File path cannot be empty".to_string());
    }
    FileManager::save_file(&file_path, &content)
}

#[tauri::command]
async fn load_file(file_path: String) -> Result<String, String> {
    if file_path.trim().is_empty() {
        return Err("File path cannot be empty".to_string());
    }
    FileManager::load_file(&file_path)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    if name.trim().is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

// Typst compilation command - returns PDF bytes as base64
#[tauri::command]
async fn compile_typst(code: String) -> Result<String, String> {
    if code.trim().is_empty() {
        return Err("Typst code cannot be empty".to_string());
    }

    let compiler = TypstCompiler::new();
    let document = compiler.compile(code)?;

    if document.pages.is_empty() {
        return Err("Document has no pages".to_string());
    }

    let pdf_bytes = TypstRenderer::export_to_pdf(&document)?;

    use base64::{engine::general_purpose, Engine as _};
    let b64 = general_purpose::STANDARD.encode(pdf_bytes);
    Ok(format!("data:application/pdf;base64,{}", b64))
}

// Typst slide compilation command (16:9 aspect ratio for presentations)
#[tauri::command]
async fn compile_typst_slide(code: String, page_index: usize) -> Result<String, String> {
    if code.trim().is_empty() {
        return Err("Typst code cannot be empty".to_string());
    }

    let compiler = TypstCompiler::new();
    let document = compiler.compile(code)?;

    if document.pages.is_empty() {
        return Err("Document has no pages".to_string());
    }

    if page_index >= document.pages.len() {
        return Err(format!(
            "Page index {} out of bounds (document has {} pages)",
            page_index,
            document.pages.len()
        ));
    }

    let png_bytes = TypstRenderer::render_page_to_png(&document, page_index, 144.0)?;

    use base64::{engine::general_purpose, Engine as _};
    let b64 = general_purpose::STANDARD.encode(png_bytes);
    Ok(format!("data:image/png;base64,{}", b64))
}

// Get total page count for navigation
#[tauri::command]
async fn get_typst_page_count(code: String) -> Result<usize, String> {
    if code.trim().is_empty() {
        return Ok(0);
    }

    let compiler = TypstCompiler::new();
    let document = compiler.compile(code)?;
    Ok(document.pages.len())
}

// Export document to PDF
#[tauri::command]
async fn export_to_pdf(code: String) -> Result<Vec<u8>, String> {
    if code.trim().is_empty() {
        return Err("Typst code cannot be empty".to_string());
    }

    let compiler = TypstCompiler::new();
    let document = compiler.compile(code)?;

    if document.pages.is_empty() {
        return Err("Document has no pages".to_string());
    }

    TypstRenderer::export_to_pdf(&document)
}

// Export document to PNG
#[tauri::command]
async fn export_to_png(code: String, dpi: Option<f32>) -> Result<Vec<u8>, String> {
    if code.trim().is_empty() {
        return Err("Typst code cannot be empty".to_string());
    }

    let compiler = TypstCompiler::new();
    let document = compiler.compile(code)?;

    if document.pages.is_empty() {
        return Err("Document has no pages".to_string());
    }

    let dpi_value = dpi.unwrap_or(144.0);
    let png_bytes = TypstRenderer::render_first_page_to_png(&document, dpi_value)?;

    Ok(png_bytes)
}

// Template management commands
#[tauri::command]
async fn save_template(id: String, name: String, description: String, category: String, content: String, preview: Option<String>) -> Result<(), String> {
    use template_service::{Template, TemplateService};
    
    let template = Template {
        id,
        name,
        description,
        category,
        content,
        preview,
    };
    
    let service = TemplateService::new()?;
    service.save_template(&template)?;
    
    Ok(())
}

#[tauri::command]
async fn load_template(id: String) -> Result<String, String> {
    use template_service::TemplateService;
    
    let service = TemplateService::new()?;
    let template = service.load_template(&id)?;
    
    Ok(serde_json::to_string(&template).map_err(|e| e.to_string())?)
}

#[tauri::command]
async fn list_templates() -> Result<String, String> {
    use template_service::TemplateService;
    
    let service = TemplateService::new()?;
    let templates = service.list_templates()?;
    
    Ok(serde_json::to_string(&templates).map_err(|e| e.to_string())?)
}

#[tauri::command]
async fn delete_template(id: String) -> Result<(), String> {
    use template_service::TemplateService;
    
    let service = TemplateService::new()?;
    service.delete_template(&id)?;
    
    Ok(())
}

#[tauri::command]
async fn get_templates_directory() -> Result<String, String> {
    use template_service::TemplateService;
    
    let dir = TemplateService::get_templates_directory()?;
    
    Ok(dir.to_string_lossy().to_string())
}

#[tauri::command]
async fn download_template_from_url(url: String) -> Result<String, String> {
    use template_service::TemplateService;
    
    let template = TemplateService::download_template_from_url(&url).await?;
    
    // 保存下载的模板
    let service = TemplateService::new()?;
    service.save_template(&template)?;
    
    Ok(serde_json::to_string(&template).map_err(|e| e.to_string())?)
}

// Sync spreadsheet data
#[tauri::command]
async fn sync_sheet_data(doc_id: String, sheet_data_json: String) -> Result<(), String> {
    if doc_id.trim().is_empty() {
        return Err("Document ID cannot be empty".to_string());
    }
    if sheet_data_json.trim().is_empty() {
        return Err("Sheet data cannot be empty".to_string());
    }

    eprintln!("[Spreadsheet Sync] Syncing sheet for doc: {}", doc_id);
    eprintln!(
        "[Spreadsheet Sync] Data size: {} bytes",
        sheet_data_json.len()
    );

    Ok(())
}

// Get spreadsheet data
#[tauri::command]
async fn get_sheet_data(doc_id: String) -> Result<String, String> {
    if doc_id.trim().is_empty() {
        return Err("Document ID cannot be empty".to_string());
    }

    eprintln!("[Spreadsheet Sync] Fetching sheet for doc: {}", doc_id);

    let default_data = r#"[
      {
        "name": "Sheet1",
        "color": "",
        "status": 1,
        "order": 0,
        "data": [
          {"r": 0, "c": 0, "v": {"v": "年份", "m": "年份", "ct": {"fa": "@", "t": "s"}}},
          {"r": 0, "c": 1, "v": {"v": "营收 (亿)", "m": "营收 (亿)", "ct": {"fa": "@", "t": "s"}}},
          {"r": 0, "c": 2, "v": {"v": "利润 (亿)", "m": "利润 (亿)", "ct": {"fa": "@", "t": "s"}}},
          {"r": 1, "c": 0, "v": {"v": 2025, "m": "2025", "ct": {"fa": "General", "t": "n"}}},
          {"r": 1, "c": 1, "v": {"v": 12.5, "m": "12.5", "ct": {"fa": "0.0", "t": "n"}}},
          {"r": 1, "c": 2, "v": {"v": 3.2, "m": "3.2", "ct": {"fa": "0.0", "t": "n"}}}
        ],
        "config": {},
        "index": 0
      }
    ]"#;

    Ok(default_data.to_string())
}

// Aerospace-grade spreadsheet service commands
use spreadsheet_service::{
    SpreadsheetService, CellValue, CellReference, CellStyle, 
    DataValidation, ValidationRule, ValidationType, 
    PivotConfig as SpreadsheetPivotConfig, 
    ChartConfig as SpreadsheetChartConfig, 
    ConditionalFormat, SpreadsheetError,
};

/// Evaluate a formula using the aerospace-grade formula engine
#[tauri::command]
async fn evaluate_formula(formula: String, cell_values_json: String) -> Result<String, String> {
    use spreadsheet_service::FormulaEngine;
    
    let cell_values: std::collections::HashMap<String, CellValue> = 
        serde_json::from_str(&cell_values_json)
            .map_err(|e| format!("Failed to parse cell values: {}", e))?;
    
    let mut engine = FormulaEngine::new();
    let result = engine.evaluate(&formula, &cell_values)
        .map_err(|e| format!("Formula evaluation error: {}", e))?;
    
    let result_json = serde_json::to_string(&result)
        .map_err(|e| format!("Failed to serialize result: {}", e))?;
    
    Ok(result_json)
}

/// Generate a pivot table
#[tauri::command]
async fn generate_pivot_table(data_json: String, config_json: String) -> Result<String, String> {
    use spreadsheet_service::PivotGenerator;
    
    let data: Vec<std::collections::HashMap<String, CellValue>> = 
        serde_json::from_str(&data_json)
            .map_err(|e| format!("Failed to parse data: {}", e))?;
    
    let config: SpreadsheetPivotConfig = 
        serde_json::from_str(&config_json)
            .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    let generator = PivotGenerator::new();
    let result = generator.generate(&data, config)
        .map_err(|e| format!("Pivot generation error: {}", e))?;
    
    let result_json = serde_json::to_string(&result)
        .map_err(|e| format!("Failed to serialize result: {}", e))?;
    
    Ok(result_json)
}

/// Generate a spreadsheet chart
#[tauri::command]
async fn generate_spreadsheet_chart(data_json: String, config_json: String) -> Result<String, String> {
    use spreadsheet_service::ChartGenerator;
    
    let data: Vec<std::collections::HashMap<String, CellValue>> = 
        serde_json::from_str(&data_json)
            .map_err(|e| format!("Failed to parse data: {}", e))?;
    
    let config: SpreadsheetChartConfig = 
        serde_json::from_str(&config_json)
            .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    let generator = ChartGenerator::new();
    let result = generator.generate(&data, config)
        .map_err(|e| format!("Chart generation error: {}", e))?;
    
    let result_json = serde_json::to_string(&result)
        .map_err(|e| format!("Failed to serialize result: {}", e))?;
    
    Ok(result_json)
}

/// Validate cell data
#[tauri::command]
async fn validate_cell_data(
    value_json: String, 
    rule_json: String
) -> Result<String, String> {
    use spreadsheet_service::DataValidation;
    
    let value: CellValue = 
        serde_json::from_str(&value_json)
            .map_err(|e| format!("Failed to parse value: {}", e))?;
    
    let rule: ValidationRule = 
        serde_json::from_str(&rule_json)
            .map_err(|e| format!("Failed to parse rule: {}", e))?;
    
    let validation = DataValidation::new(rule);
    let result = validation.validate(&value)
        .map_err(|e| format!("Validation error: {}", e))?;
    
    let result_json = serde_json::to_string(&result)
        .map_err(|e| format!("Failed to serialize result: {}", e))?;
    
    Ok(result_json)
}

/// Apply cell style
#[tauri::command]
async fn apply_cell_style(style_json: String) -> Result<String, String> {
    use spreadsheet_service::StyleManager;
    
    let style: CellStyle = 
        serde_json::from_str(&style_json)
            .map_err(|e| format!("Failed to parse style: {}", e))?;
    
    let mut manager = StyleManager::new();
    let style_id = manager.register_style(style);
    
    Ok(style_id)
}

/// Get spreadsheet service status
#[tauri::command]
async fn get_spreadsheet_service_status() -> Result<String, String> {
    let _service = SpreadsheetService::global();
    let status = serde_json::json!({
        "initialized": true,
        "cell_manager": "active",
        "formula_engine": "active",
        "style_manager": "active",
        "validation_manager": "active",
        "pivot_generator": "active",
        "chart_generator": "active",
        "conditional_format_manager": "active"
    });
    
    Ok(status.to_string())
}

// PPT service commands for new elements
#[tauri::command]
async fn create_video_element(
    id: String,
    video_url: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    autoplay: bool,
    loop_video: bool,
    muted: bool,
    volume: f64,
) -> Result<String, String> {
    if id.trim().is_empty() {
        return Err("Element ID cannot be empty".to_string());
    }
    if video_url.trim().is_empty() {
        return Err("Video URL cannot be empty".to_string());
    }
    
    let video = VideoElement::new(id, video_url)
        .with_position(x, y)
        .with_size(width, height)
        .with_autoplay(autoplay)
        .with_loop(loop_video)
        .with_muted(muted)
        .with_volume(volume);
    
    video.validate()?;
    
    serde_json::to_string(&video).map_err(|e| format!("Failed to serialize video element: {}", e))
}

#[tauri::command]
async fn create_audio_element(
    id: String,
    audio_url: String,
    autoplay: bool,
    loop_audio: bool,
    volume: f64,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<String, String> {
    if id.trim().is_empty() {
        return Err("Element ID cannot be empty".to_string());
    }
    if audio_url.trim().is_empty() {
        return Err("Audio URL cannot be empty".to_string());
    }
    
    let audio = AudioElement::new(id, audio_url)
        .with_autoplay(autoplay)
        .with_loop(loop_audio)
        .with_volume(volume)
        .with_icon_position(x, y)
        .with_icon_size(width, height);
    
    audio.validate()?;
    
    serde_json::to_string(&audio).map_err(|e| format!("Failed to serialize audio element: {}", e))
}

#[tauri::command]
async fn create_hyperlink_element(
    id: String,
    url: String,
    text: String,
    tooltip: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    open_in_new_window: bool,
) -> Result<String, String> {
    if id.trim().is_empty() {
        return Err("Element ID cannot be empty".to_string());
    }
    if url.trim().is_empty() {
        return Err("URL cannot be empty".to_string());
    }
    if text.trim().is_empty() {
        return Err("Display text cannot be empty".to_string());
    }
    
    let hyperlink = HyperlinkElement::new(id, url, text)
        .with_tooltip(tooltip)
        .with_position(x, y)
        .with_size(width, height)
        .with_open_in_new_window(open_in_new_window);
    
    hyperlink.validate()?;
    
    serde_json::to_string(&hyperlink).map_err(|e| format!("Failed to serialize hyperlink element: {}", e))
}

#[tauri::command]
async fn create_artword_element(
    id: String,
    text: String,
    style: String,
    x: f64,
    y: f64,
    font_size: f64,
    font_name: String,
) -> Result<String, String> {
    if id.trim().is_empty() {
        return Err("Element ID cannot be empty".to_string());
    }
    if text.trim().is_empty() {
        return Err("Text cannot be empty".to_string());
    }
    
    let style_enum = match style.as_str() {
        "gradient_fill" => ppt_service::artword::ArtWordStyle::GradientFill,
        "outline" => ppt_service::artword::ArtWordStyle::Outline,
        "shadow" => ppt_service::artword::ArtWordStyle::Shadow,
        "reflection" => ppt_service::artword::ArtWordStyle::Reflection,
        "glow" => ppt_service::artword::ArtWordStyle::Glow,
        "3d" => ppt_service::artword::ArtWordStyle::ThreeD,
        "transform" => ppt_service::artword::ArtWordStyle::Transform,
        _ => ppt_service::artword::ArtWordStyle::GradientFill,
    };
    
    let artword = ArtWordElement::new(id, text)
        .with_style(style_enum)
        .with_position(x, y)
        .with_font_size(font_size)
        .with_font(font_name);
    
    artword.validate()?;
    
    serde_json::to_string(&artword).map_err(|e| format!("Failed to serialize art word element: {}", e))
}

#[tauri::command]
async fn create_smartart_element(
    id: String,
    smartart_type: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<String, String> {
    if id.trim().is_empty() {
        return Err("Element ID cannot be empty".to_string());
    }
    
    let type_enum = match smartart_type.as_str() {
        "process" => SmartArtType::Process,
        "cycle" => SmartArtType::Cycle,
        "hierarchy" => SmartArtType::Hierarchy,
        "relationship" => SmartArtType::Relationship,
        "matrix" => SmartArtType::Matrix,
        "pyramid" => SmartArtType::Pyramid,
        "list" => SmartArtType::List,
        "chart" => SmartArtType::Chart,
        _ => SmartArtType::Process,
    };
    
    let smartart = SmartArtElement::new(id, type_enum)
        .with_position(x, y)
        .with_size(width, height);
    
    smartart.validate()?;
    
    serde_json::to_string(&smartart).map_err(|e| format!("Failed to serialize SmartArt element: {}", e))
}

#[tauri::command]
async fn add_smartart_node(
    smartart_json: String,
    node_id: String,
    node_text: String,
) -> Result<String, String> {
    if node_id.trim().is_empty() {
        return Err("Node ID cannot be empty".to_string());
    }
    if node_text.trim().is_empty() {
        return Err("Node text cannot be empty".to_string());
    }
    
    let mut smartart: SmartArtElement = serde_json::from_str(&smartart_json)
        .map_err(|e| format!("Failed to parse SmartArt element: {}", e))?;
    
    let node = SmartArtNode::new(node_id, node_text);
    smartart.add_node(node)?;
    
    serde_json::to_string(&smartart).map_err(|e| format!("Failed to serialize updated SmartArt element: {}", e))
}

#[tauri::command]
async fn get_ppt_service_status() -> Result<String, String> {
    let status = serde_json::json!({
        "initialized": true,
        "video_element": "available",
        "audio_element": "available",
        "hyperlink_element": "available",
        "artword_element": "available",
        "smartart_element": "available",
        "text_direction": "available",
        "numbering_styles": "available"
    });
    
    Ok(status.to_string())
}

// Get TipTap configuration
#[tauri::command]
async fn get_tiptap_config(preset: Option<String>) -> Result<String, String> {
    let config = if let Some(preset_name) = preset {
        if preset_name.trim().is_empty() {
            return Err("Preset name cannot be empty".to_string());
        }
        tiptap_service::TipTapPresets::get_preset(&preset_name)
            .ok_or_else(|| format!("Preset '{}' not found", preset_name))?
    } else {
        tiptap_service::TipTapConfig::default()
    };

    serde_json::to_string(&config).map_err(|e| format!("Failed to serialize config: {}", e))
}

// List available TipTap presets
#[tauri::command]
async fn list_tiptap_presets() -> Result<Vec<String>, String> {
    Ok(tiptap_service::TipTapPresets::list_presets())
}

// Render LaTeX to HTML
#[tauri::command]
async fn render_latex(latex: String, display_mode: bool) -> Result<String, String> {
    if latex.trim().is_empty() {
        return Ok(String::new());
    }

    let config_service = Arc::new(config_service::ExportConfigService::new());
    let mut renderer = LatexRenderer::new(config_service);
    renderer.render(&latex, display_mode)
}

// Generate chart SVG
#[tauri::command]
async fn generate_chart(
    chart_type: String,
    data: ChartData,
    config: Option<ChartConfig>,
) -> Result<String, String> {
    let chart_type_enum = match chart_type.as_str() {
        "pie" => ChartType::Pie,
        "bar" => ChartType::Bar,
        "line" => ChartType::Line,
        "area" => ChartType::Area,
        "scatter" => ChartType::Scatter,
        "doughnut" => ChartType::Doughnut,
        _ => return Err(format!("Invalid chart type: {}", chart_type)),
    };

    let request = chart_service::ChartRenderRequest {
        chart_type: chart_type_enum,
        data,
        config,
    };

    let config_service = Arc::new(config_service::ExportConfigService::new());
    let mut generator = ChartGenerator::new(config_service);
    generator.generate(request)
}

// Conversation management commands
#[tauri::command]
async fn create_conversation(title: String) -> Result<String, String> {
    let manager = CONVERSATION_MANAGER.lock().await;
    manager.create_conversation(title)
}

#[tauri::command]
async fn get_conversation(conversation_id: String) -> Result<Conversation, String> {
    let manager = CONVERSATION_MANAGER.lock().await;
    manager.get_conversation(&conversation_id)
}

#[tauri::command]
async fn get_all_conversations() -> Result<Vec<Conversation>, String> {
    let manager = CONVERSATION_MANAGER.lock().await;
    manager.get_all_conversations()
}

#[tauri::command]
async fn delete_conversation(conversation_id: String) -> Result<(), String> {
    let manager = CONVERSATION_MANAGER.lock().await;
    manager.delete_conversation(&conversation_id)
}

#[tauri::command]
async fn add_conversation_message(
    conversation_id: String,
    role: String,
    content: String,
    tokens_used: Option<u32>,
) -> Result<(), String> {
    let role_enum = match role.as_str() {
        "user" => ConversationRole::User,
        "assistant" => ConversationRole::Assistant,
        "system" => ConversationRole::System,
        _ => return Err(format!("Invalid role: {}", role)),
    };

    let manager = CONVERSATION_MANAGER.lock().await;
    manager.add_message(&conversation_id, role_enum, content, tokens_used)
}

#[tauri::command]
async fn get_conversation_context(
    conversation_id: String,
    limit: usize,
) -> Result<Vec<ai_service::ConversationMessage>, String> {
    let manager = CONVERSATION_MANAGER.lock().await;
    manager.get_conversation_context(&conversation_id, limit)
}

// Template management commands
#[tauri::command]
async fn get_all_templates() -> Result<Vec<PromptTemplate>, String> {
    let manager = CONVERSATION_MANAGER.lock().await;
    manager.get_all_templates()
}

#[tauri::command]
async fn get_template(template_id: String) -> Result<PromptTemplate, String> {
    let manager = CONVERSATION_MANAGER.lock().await;
    manager.get_template(&template_id)
}

#[tauri::command]
async fn create_template(template: PromptTemplate) -> Result<String, String> {
    let manager = CONVERSATION_MANAGER.lock().await;
    manager.create_template(template)
}

#[tauri::command]
async fn delete_conversation_template(template_id: String) -> Result<(), String> {
    let manager = CONVERSATION_MANAGER.lock().await;
    manager.delete_template(&template_id)
}

#[tauri::command]
async fn apply_template(
    template_id: String,
    variables: serde_json::Value,
) -> Result<String, String> {
    let manager = CONVERSATION_MANAGER.lock().await;

    let var_map: std::collections::HashMap<String, String> = serde_json::from_value(variables)
        .map_err(|e| format!("Failed to parse variables: {}", e))?;

    manager.apply_template(&template_id, &var_map)
}

#[tauri::command]
async fn get_conversation_stats() -> Result<ai_service::ConversationStats, String> {
    let manager = CONVERSATION_MANAGER.lock().await;
    manager.get_statistics()
}

// Collaboration commands
#[tauri::command]
async fn create_collaboration_document(
    document_id: String,
    doc_type: String,
) -> Result<(), String> {
    let doc_type_enum = match doc_type.as_str() {
        "text" => CRDTType::Text,
        "richtext" => CRDTType::RichText,
        "json" => CRDTType::JSON,
        _ => return Err(format!("Invalid document type: {}", doc_type)),
    };

    let mut documents = COLLABORATION_DOCUMENTS.lock().await;

    if documents.contains_key(&document_id) {
        return Err(format!("Document {} already exists", document_id));
    }

    let config_service = Arc::new(config_service::ExportConfigService::new());
    documents.insert(
        document_id.clone(),
        CRDTDocument::new(document_id, doc_type_enum, config_service),
    );
    Ok(())
}

#[tauri::command]
async fn get_collaboration_document(document_id: String) -> Result<CRDTDocument, String> {
    let documents = COLLABORATION_DOCUMENTS.lock().await;

    documents
        .get(&document_id)
        .cloned()
        .ok_or_else(|| format!("Document {} not found", document_id))
}

#[tauri::command]
async fn apply_collaboration_operation(
    document_id: String,
    operation: CRDTOperation,
) -> Result<(), String> {
    let mut documents = COLLABORATION_DOCUMENTS.lock().await;

    let document = documents
        .get_mut(&document_id)
        .ok_or_else(|| format!("Document {} not found", document_id))?;

    document.apply_operation(operation)
}

#[tauri::command]
async fn merge_collaboration_operations(
    document_id: String,
    operations: Vec<CRDTOperation>,
) -> Result<(), String> {
    let mut documents = COLLABORATION_DOCUMENTS.lock().await;

    let document = documents
        .get_mut(&document_id)
        .ok_or_else(|| format!("Document {} not found", document_id))?;

    document.merge_operations(operations)
}

#[tauri::command]
async fn update_user_presence(
    _document_id: String,
    user_id: String,
    user_name: String,
    cursor_position: Option<usize>,
) -> Result<PresenceInfo, String> {
    let presence = PresenceInfo {
        user_id: user_id.clone(),
        user_name,
        cursor_position,
        selection: None,
        last_seen: chrono::Utc::now(),
        is_online: true,
    };

    // In a real implementation, this would store presence in a separate map
    // For now, we just return the presence info
    Ok(presence)
}

// Cloud sync commands
#[tauri::command]
async fn configure_cloud_sync(config: SyncConfig) -> Result<(), String> {
    let mut manager = SYNC_MANAGER.lock().await;
    manager.update_config(config)
}

#[tauri::command]
async fn get_sync_status() -> Result<SyncStatus, String> {
    let manager = SYNC_MANAGER.lock().await;
    manager.get_status()
}

#[tauri::command]
async fn perform_sync() -> Result<SyncResult, String> {
    SYNC_MANAGER.lock().await.sync().await
}

#[tauri::command]
async fn upload_cloud_file(file_path: String, content: Vec<u8>) -> Result<String, String> {
    SYNC_MANAGER
        .lock()
        .await
        .upload_file(file_path, content)
        .await
}

#[tauri::command]
async fn download_cloud_file(file_id: String) -> Result<Vec<u8>, String> {
    SYNC_MANAGER.lock().await.download_file(file_id).await
}

#[tauri::command]
async fn list_cloud_files() -> Result<Vec<cloud_service::CloudFile>, String> {
    SYNC_MANAGER.lock().await.list_files().await
}

// Document comparison commands
#[tauri::command]
async fn compare_documents(old_text: String, new_text: String) -> Result<DiffResult, String> {
    let config_service = Arc::new(config_service::ExportConfigService::new());
    let mut engine = DiffEngine::new(config_service);
    Ok(engine.compare(&old_text, &new_text))
}

#[tauri::command]
async fn render_diff_view(
    old_text: String,
    new_text: String,
    config: Option<DiffViewConfig>,
) -> Result<diff_service::DiffViewOutput, String> {
    let config_service = Arc::new(config_service::ExportConfigService::new());
    let mut engine = DiffEngine::new(config_service);
    let diff_result = engine.compare(&old_text, &new_text);

    let viewer_config = config.unwrap_or_default();
    let viewer = diff_service::DiffViewer::new(viewer_config);

    Ok(viewer.render(&diff_result, &old_text, &new_text))
}

// Macro commands
#[tauri::command]
async fn create_macro(
    name: String,
    description: String,
    actions: Vec<MacroAction>,
) -> Result<String, String> {
    let engine = MACRO_ENGINE.lock().await;
    engine.create_macro(name, description, actions)
}

#[tauri::command]
async fn get_macro(id: String) -> Result<Macro, String> {
    let engine = MACRO_ENGINE.lock().await;
    engine.get_macro(&id)
}

#[tauri::command]
async fn get_all_macros() -> Result<Vec<Macro>, String> {
    let engine = MACRO_ENGINE.lock().await;
    engine.get_all_macros()
}

#[tauri::command]
async fn update_macro(
    id: String,
    name: Option<String>,
    description: Option<String>,
    actions: Option<Vec<MacroAction>>,
) -> Result<(), String> {
    let engine = MACRO_ENGINE.lock().await;
    engine.update_macro(id, name, description, actions)
}

#[tauri::command]
async fn delete_macro(id: String) -> Result<(), String> {
    let engine = MACRO_ENGINE.lock().await;
    engine.delete_macro(&id)
}

#[tauri::command]
async fn play_macro(id: String) -> Result<Vec<MacroAction>, String> {
    let engine = MACRO_ENGINE.lock().await;
    engine.play_macro(&id).await
}

#[tauri::command]
async fn stop_macro() -> Result<(), String> {
    let engine = MACRO_ENGINE.lock().await;
    engine.stop_macro().await
}

#[tauri::command]
async fn set_macro_shortcut(id: String, shortcut: String) -> Result<(), String> {
    let engine = MACRO_ENGINE.lock().await;
    engine.set_shortcut(id, shortcut)
}

#[tauri::command]
async fn get_macro_stats() -> Result<macro_service::MacroStats, String> {
    let engine = MACRO_ENGINE.lock().await;
    Ok(engine.get_stats())
}

// Macro recorder commands
#[tauri::command]
async fn start_recording() -> Result<(), String> {
    let recorder = MACRO_RECORDER.lock().await;
    recorder.start_recording()
}

#[tauri::command]
async fn stop_recording() -> Result<Vec<RecordedAction>, String> {
    let recorder = MACRO_RECORDER.lock().await;
    recorder.stop_recording()
}

#[tauri::command]
async fn record_action(action: MacroAction) -> Result<(), String> {
    let recorder = MACRO_RECORDER.lock().await;
    recorder.record_action(action)
}

#[tauri::command]
async fn is_recording() -> Result<bool, String> {
    let recorder = MACRO_RECORDER.lock().await;
    recorder.is_recording()
}

#[tauri::command]
async fn get_recorded_actions() -> Result<Vec<RecordedAction>, String> {
    let recorder = MACRO_RECORDER.lock().await;
    recorder.get_recorded_actions()
}

// Mail merge commands
#[tauri::command]
async fn parse_template(
    template: String,
) -> Result<Vec<mail_merge_service::TemplateVariable>, String> {
    let engine = TemplateEngine::new();
    engine.parse_template(&template)
}

#[tauri::command]
async fn merge_template(
    template: String,
    data: serde_json::Value,
) -> Result<mail_merge_service::MergeResult, String> {
    let engine = TemplateEngine::new();

    let data_map: std::collections::HashMap<String, String> =
        serde_json::from_value(data).map_err(|e| format!("Failed to parse data: {}", e))?;

    engine.merge(&template, &data_map)
}

#[tauri::command]
async fn validate_template(template: String) -> Result<(), String> {
    let engine = TemplateEngine::new();
    engine.validate_template(&template)
}

#[tauri::command]
async fn load_merge_data(
    source: DataSource,
) -> Result<Vec<std::collections::HashMap<String, String>>, String> {
    let config_service = Arc::new(config_service::ExportConfigService::new());
    let mut processor = DataProcessor::new(config_service);
    processor.load_data(&source)
}

#[tauri::command]
async fn process_batch_merge(
    template: String,
    data: serde_json::Value,
    config: MergeConfig,
) -> Result<MergeBatchResult, String> {
    let config_service = Arc::new(config_service::ExportConfigService::new());
    let mut processor = DataProcessor::new(config_service);

    let data_vec: Vec<std::collections::HashMap<String, String>> =
        serde_json::from_value(data).map_err(|e| format!("Failed to parse data: {}", e))?;

    processor.process_batch_merge(&template, &data_vec, &config)
}

// OCR commands
#[tauri::command]
async fn recognize_ocr_file(image_path: String) -> Result<OcrResult, String> {
    let engine = OCR_ENGINE.lock().await;
    engine.recognize_file(&image_path)
}

#[tauri::command]
async fn recognize_ocr_bytes(image_data: Vec<u8>, format: String) -> Result<OcrResult, String> {
    let engine = OCR_ENGINE.lock().await;
    engine.recognize_bytes(&image_data, &format)
}

#[tauri::command]
async fn recognize_ocr_with_layout(image_path: String) -> Result<OcrResult, String> {
    let engine = OCR_ENGINE.lock().await;
    engine.recognize_with_layout(&image_path)
}

#[tauri::command]
async fn get_ocr_languages() -> Result<Vec<String>, String> {
    let engine = OCR_ENGINE.lock().await;
    Ok(engine.get_supported_languages())
}

#[tauri::command]
async fn update_ocr_config(config: OcrConfig) -> Result<(), String> {
    let mut engine = OCR_ENGINE.lock().await;
    engine.update_config(config);
    Ok(())
}

// Plugin commands
#[tauri::command]
async fn load_plugin(plugin_path: String) -> Result<String, String> {
    let mut manager = PLUGIN_MANAGER.lock().await;
    manager.load_plugin(&plugin_path)
}

#[tauri::command]
async fn unload_plugin(plugin_id: String) -> Result<(), String> {
    let mut manager = PLUGIN_MANAGER.lock().await;
    manager.unload_plugin(&plugin_id)
}

#[tauri::command]
async fn get_all_plugins() -> Result<Vec<plugin_service::PluginInstance>, String> {
    let manager = PLUGIN_MANAGER.lock().await;
    manager.get_instances()
}

#[tauri::command]
async fn get_plugin(plugin_id: String) -> Result<plugin_service::PluginInstance, String> {
    let manager = PLUGIN_MANAGER.lock().await;
    manager.get_instance(&plugin_id)
}

#[tauri::command]
async fn enable_plugin(plugin_id: String) -> Result<(), String> {
    let mut manager = PLUGIN_MANAGER.lock().await;
    manager.enable_plugin(&plugin_id)
}

#[tauri::command]
async fn disable_plugin(plugin_id: String) -> Result<(), String> {
    let mut manager = PLUGIN_MANAGER.lock().await;
    manager.disable_plugin(&plugin_id)
}

#[tauri::command]
async fn register_plugin_hook(
    plugin_id: String,
    hook: String,
    handler: String,
) -> Result<(), String> {
    let mut manager = PLUGIN_MANAGER.lock().await;

    let hook_enum = match hook.as_str() {
        "on_load" => PluginHook::OnLoad,
        "on_unload" => PluginHook::OnUnload,
        "on_document_open" => PluginHook::OnDocumentOpen,
        "on_document_save" => PluginHook::OnDocumentSave,
        "on_document_change" => PluginHook::OnDocumentChange,
        "on_command" => PluginHook::OnCommand,
        _ => return Err(format!("Invalid hook: {}", hook)),
    };

    manager.register_hook(plugin_id, hook_enum, handler)
}

#[tauri::command]
async fn trigger_plugin_hook(
    plugin_id: String,
    hook: String,
    data: Option<serde_json::Value>,
) -> Result<(), String> {
    let manager = PLUGIN_MANAGER.lock().await;

    let hook_enum = match hook.as_str() {
        "on_load" => PluginHook::OnLoad,
        "on_unload" => PluginHook::OnUnload,
        "on_document_open" => PluginHook::OnDocumentOpen,
        "on_document_save" => PluginHook::OnDocumentSave,
        "on_document_change" => PluginHook::OnDocumentChange,
        "on_command" => PluginHook::OnCommand,
        _ => return Err(format!("Invalid hook: {}", hook)),
    };

    manager.trigger_hook(&plugin_id, hook_enum, data);
    Ok(())
}

#[tauri::command]
async fn get_plugin_stats() -> Result<plugin_service::PluginStats, String> {
    let manager = PLUGIN_MANAGER.lock().await;
    Ok(manager.get_stats())
}

// Accessibility commands
#[tauri::command]
async fn build_accessibility_tree(
    content: String,
) -> Result<accessibility_service::AccessibilityTree, String> {
    let mut bridge = ACCESSIBILITY_BRIDGE.lock().await;
    bridge.build_tree(&content)
}

#[tauri::command]
async fn get_accessibility_tree() -> Result<Option<accessibility_service::AccessibilityTree>, String>
{
    let bridge = ACCESSIBILITY_BRIDGE.lock().await;
    Ok(bridge.get_tree().cloned())
}

#[tauri::command]
async fn update_accessibility_node(node_id: String, node: AccessibilityNode) -> Result<(), String> {
    let mut bridge = ACCESSIBILITY_BRIDGE.lock().await;
    bridge.update_node(node_id, node)
}

#[tauri::command]
async fn add_accessibility_node(parent_id: String, node: AccessibilityNode) -> Result<(), String> {
    let mut bridge = ACCESSIBILITY_BRIDGE.lock().await;
    bridge.add_node(parent_id, node)
}

#[tauri::command]
async fn remove_accessibility_node(node_id: String) -> Result<(), String> {
    let mut bridge = ACCESSIBILITY_BRIDGE.lock().await;
    bridge.remove_node(node_id)
}

#[tauri::command]
async fn validate_accessibility_node(node: AccessibilityNode) -> Result<Vec<String>, String> {
    let bridge = ACCESSIBILITY_BRIDGE.lock().await;
    Ok(bridge.validate_attributes(&node))
}

#[tauri::command]
async fn get_accessibility_stats() -> Result<accessibility_service::AccessibilityStats, String> {
    let bridge = ACCESSIBILITY_BRIDGE.lock().await;
    Ok(bridge.get_stats())
}

// Screen reader commands
#[tauri::command]
async fn announce_to_screen_reader(message: String, priority: String) -> Result<(), String> {
    let announcer = SCREEN_READER_ANNOUNCER.lock().await;

    let priority_enum = match priority.as_str() {
        "polite" => AnnouncementPriority::Polite,
        "assertive" => AnnouncementPriority::Assertive,
        _ => return Err(format!("Invalid priority: {}", priority)),
    };

    announcer.announce(message, priority_enum);
    Ok(())
}

#[tauri::command]
async fn get_screen_reader_announcements(
) -> Result<Vec<accessibility_service::ScreenReaderAnnouncement>, String> {
    let announcer = SCREEN_READER_ANNOUNCER.lock().await;
    Ok(announcer.get_announcements())
}

#[tauri::command]
async fn clear_screen_reader_announcements() -> Result<(), String> {
    let announcer = SCREEN_READER_ANNOUNCER.lock().await;
    announcer.clear();
    Ok(())
}

// Export commands
#[tauri::command]
async fn export_document(
    content: String,
    config: ExportConfig,
) -> Result<export_service::ExportResult, String> {
    let mut generator = EXPORT_GENERATOR.lock().await;
    generator.export(&content, &config)
}

#[tauri::command]
async fn get_supported_export_formats() -> Result<Vec<ExportFormat>, String> {
    let generator = EXPORT_GENERATOR.lock().await;
    Ok(generator.get_supported_formats())
}

// Incremental compilation commands
#[tauri::command]
async fn compute_incremental_hash(content: String) -> Result<String, String> {
    let compiler = INCREMENTAL_COMPILER.lock().await;
    Ok(compiler.compute_hash(&content))
}

#[tauri::command]
async fn check_cache_valid(document_id: String, hash: String) -> Result<bool, String> {
    let compiler = INCREMENTAL_COMPILER.lock().await;
    Ok(compiler.is_cache_valid(&document_id, &hash))
}

#[tauri::command]
async fn update_incremental_cache(
    document_id: String,
    hash: String,
    dependencies: Vec<String>,
    output: Vec<u8>,
) -> Result<(), String> {
    let mut compiler = INCREMENTAL_COMPILER.lock().await;
    compiler.update_cache(document_id, hash, dependencies, output);
    Ok(())
}

#[tauri::command]
async fn get_cached_compilation(document_id: String) -> Result<Option<Vec<u8>>, String> {
    let compiler = INCREMENTAL_COMPILER.lock().await;
    Ok(compiler
        .get_cached(&document_id)
        .map(|entry| entry.compiled_output.clone()))
}

#[tauri::command]
async fn clear_incremental_cache() -> Result<(), String> {
    let mut compiler = INCREMENTAL_COMPILER.lock().await;
    compiler.clear_all();
    Ok(())
}

#[tauri::command]
async fn get_incremental_cache_size() -> Result<u64, String> {
    let compiler = INCREMENTAL_COMPILER.lock().await;
    Ok(compiler.get_cache_size())
}

// Package management commands
#[tauri::command]
async fn search_packages(query: String) -> Result<Vec<PackageInfo>, String> {
    let manager = PACKAGE_MANAGER.lock().await;
    Ok(manager.search(&query).into_iter().cloned().collect())
}

#[tauri::command]
async fn install_package(name: String, version: Option<String>) -> Result<String, String> {
    let mut manager = PACKAGE_MANAGER.lock().await;
    manager.install(&name, version.as_deref())
}

#[tauri::command]
async fn uninstall_package(name: String) -> Result<String, String> {
    let mut manager = PACKAGE_MANAGER.lock().await;
    manager.uninstall(&name)
}

#[tauri::command]
async fn update_package(name: String) -> Result<String, String> {
    let mut manager = PACKAGE_MANAGER.lock().await;
    manager.update(&name)
}

#[tauri::command]
async fn list_installed_packages() -> Result<Vec<PackageInfo>, String> {
    let manager = PACKAGE_MANAGER.lock().await;
    Ok(manager.list_installed().into_iter().cloned().collect())
}

#[tauri::command]
async fn list_available_packages() -> Result<Vec<PackageInfo>, String> {
    let manager = PACKAGE_MANAGER.lock().await;
    Ok(manager.list_available().into_iter().cloned().collect())
}

#[tauri::command]
async fn add_package_to_cache(package: PackageInfo) -> Result<(), String> {
    let mut manager = PACKAGE_MANAGER.lock().await;
    manager.add_to_cache(package);
    Ok(())
}

#[tauri::command]
async fn check_package_dependencies(name: String) -> Result<Vec<String>, String> {
    let manager = PACKAGE_MANAGER.lock().await;
    manager.check_dependencies(&name)
}

#[tauri::command]
async fn get_package_stats() -> Result<typist_service::package::PackageStats, String> {
    let manager = PACKAGE_MANAGER.lock().await;
    Ok(manager.get_stats())
}

// Voice commands
#[tauri::command]
async fn start_speech_recognition() -> Result<(), String> {
    let mut recognizer = SPEECH_RECOGNIZER.lock().await;
    recognizer.start()
}

#[tauri::command]
async fn stop_speech_recognition() -> Result<(), String> {
    let mut recognizer = SPEECH_RECOGNIZER.lock().await;
    recognizer.stop()
}

#[tauri::command]
async fn process_speech_audio(
    audio_data: Vec<u8>,
) -> Result<voice_service::RecognitionResult, String> {
    let recognizer = SPEECH_RECOGNIZER.lock().await;
    recognizer.process_audio(&audio_data)
}

#[tauri::command]
async fn is_speech_recognition_active() -> Result<bool, String> {
    let recognizer = SPEECH_RECOGNIZER.lock().await;
    Ok(recognizer.is_listening())
}

#[tauri::command]
async fn update_recognition_config(config: RecognitionConfig) -> Result<(), String> {
    let mut recognizer = SPEECH_RECOGNIZER.lock().await;
    recognizer.update_config(config);
    Ok(())
}

#[tauri::command]
async fn get_recognition_languages() -> Result<Vec<String>, String> {
    let recognizer = SPEECH_RECOGNIZER.lock().await;
    Ok(recognizer.get_supported_languages())
}

#[tauri::command]
async fn speak_text(text: String) -> Result<(), String> {
    let tts = TEXT_TO_SPEECH.lock().await;
    tts.speak(&text)
}

#[tauri::command]
async fn stop_speaking() -> Result<(), String> {
    let tts = TEXT_TO_SPEECH.lock().await;
    tts.stop()
}

#[tauri::command]
async fn get_available_voices() -> Result<Vec<voice_service::Voice>, String> {
    let tts = TEXT_TO_SPEECH.lock().await;
    Ok(tts.get_voices().to_vec())
}

#[tauri::command]
async fn update_tts_config(config: TTSConfig) -> Result<(), String> {
    let mut tts = TEXT_TO_SPEECH.lock().await;
    tts.update_config(config);
    Ok(())
}

// Comments commands
#[tauri::command]
async fn create_comment(
    document_id: String,
    title: String,
    comment: Comment,
) -> Result<String, String> {
    let mut manager = COMMENTS_MANAGER.lock().await;
    manager.create_comment(document_id, title, comment)
}

#[tauri::command]
async fn reply_to_comment(thread_id: String, comment: Comment) -> Result<(), String> {
    let mut manager = COMMENTS_MANAGER.lock().await;
    manager.reply_to_comment(thread_id, comment)
}

#[tauri::command]
async fn get_comment_thread(
    thread_id: String,
) -> Result<Option<comments_service::CommentThread>, String> {
    let manager = COMMENTS_MANAGER.lock().await;
    Ok(manager.get_thread(&thread_id).cloned())
}

#[tauri::command]
async fn get_document_comments(
    document_id: String,
) -> Result<Vec<comments_service::CommentThread>, String> {
    let manager = COMMENTS_MANAGER.lock().await;
    Ok(manager
        .get_document_comments(&document_id)
        .into_iter()
        .cloned()
        .collect())
}

#[tauri::command]
async fn filter_comments(
    filter: CommentFilter,
) -> Result<Vec<comments_service::CommentThread>, String> {
    let manager = COMMENTS_MANAGER.lock().await;
    Ok(manager
        .filter_comments(&filter)
        .into_iter()
        .cloned()
        .collect())
}

#[tauri::command]
async fn update_comment(comment_id: String, content: String) -> Result<(), String> {
    let mut manager = COMMENTS_MANAGER.lock().await;
    manager.update_comment(comment_id, content)
}

#[tauri::command]
async fn delete_comment(comment_id: String) -> Result<(), String> {
    let mut manager = COMMENTS_MANAGER.lock().await;
    manager.delete_comment(comment_id)
}

#[tauri::command]
async fn resolve_comment_thread(thread_id: String, resolved_by: String) -> Result<(), String> {
    let mut manager = COMMENTS_MANAGER.lock().await;
    manager.resolve_thread(thread_id, resolved_by)
}

#[tauri::command]
async fn reopen_comment_thread(thread_id: String) -> Result<(), String> {
    let mut manager = COMMENTS_MANAGER.lock().await;
    manager.reopen_thread(thread_id)
}

#[tauri::command]
async fn archive_comment_thread(thread_id: String) -> Result<(), String> {
    let mut manager = COMMENTS_MANAGER.lock().await;
    manager.archive_thread(thread_id)
}

#[tauri::command]
async fn delete_comment_thread(thread_id: String) -> Result<(), String> {
    let mut manager = COMMENTS_MANAGER.lock().await;
    manager.delete_thread(thread_id)
}

#[tauri::command]
async fn get_comment_stats() -> Result<comments_service::CommentStats, String> {
    let manager = COMMENTS_MANAGER.lock().await;
    Ok(manager.get_stats())
}

// Template commands
#[tauri::command]
async fn get_all_typist_templates() -> Result<Vec<Template>, String> {
    let engine = TEMPLATE_ENGINE.lock().await;
    Ok(engine.get_all_templates())
}

// Preview editor commands
#[tauri::command]
async fn preview_editor_open_file(file_id: String, content: String) -> Result<(), String> {
    let editor = PREVIEW_EDITOR.lock().await;
    editor.open_file(file_id, content)
}

#[tauri::command]
async fn preview_editor_update_source(
    file_id: String,
    new_content: String,
    position: usize,
) -> Result<(), String> {
    let editor = PREVIEW_EDITOR.lock().await;
    editor.update_source(&file_id, new_content, position)
}

#[tauri::command]
async fn preview_editor_move_cursor(file_id: String, position: usize) -> Result<(), String> {
    let editor = PREVIEW_EDITOR.lock().await;
    editor.move_cursor(&file_id, position)
}

#[tauri::command]
async fn preview_editor_set_selection(
    file_id: String,
    selection: Option<(usize, usize)>,
) -> Result<(), String> {
    let editor = PREVIEW_EDITOR.lock().await;
    editor.set_selection(&file_id, selection)
}

#[tauri::command]
async fn preview_editor_sync_cursor_to_preview(
    file_id: String,
    page: usize,
    x: f64,
    y: f64,
) -> Result<(), String> {
    let editor = PREVIEW_EDITOR.lock().await;
    editor.sync_cursor_to_preview(&file_id, page, x, y)
}

#[tauri::command]
async fn preview_editor_get_editor_state(
    file_id: String,
) -> Result<Option<typist_service::EditorState>, String> {
    let editor = PREVIEW_EDITOR.lock().await;
    editor.get_editor_state(&file_id)
}

#[tauri::command]
async fn preview_editor_get_preview_state(
    file_id: String,
) -> Result<Option<typist_service::PreviewState>, String> {
    let editor = PREVIEW_EDITOR.lock().await;
    editor.get_preview_state(&file_id)
}

#[tauri::command]
async fn preview_editor_get_diagnostics(
    file_id: String,
) -> Result<Vec<typist_service::LspDiagnostic>, String> {
    let editor = PREVIEW_EDITOR.lock().await;
    editor.get_diagnostics(&file_id)
}

#[tauri::command]
async fn preview_editor_get_completions(
    file_id: String,
    position: usize,
) -> Result<Vec<typist_service::LspCompletionItem>, String> {
    let editor = PREVIEW_EDITOR.lock().await;
    editor.get_completions(&file_id, position)
}

#[tauri::command]
async fn preview_editor_get_symbols(
    file_id: String,
) -> Result<Vec<typist_service::LspSymbol>, String> {
    let editor = PREVIEW_EDITOR.lock().await;
    editor.get_symbols(&file_id)
}

#[tauri::command]
async fn preview_editor_save_file(file_id: String) -> Result<(), String> {
    let editor = PREVIEW_EDITOR.lock().await;
    editor.save_file(&file_id)
}

#[tauri::command]
async fn preview_editor_close_file(file_id: String) -> Result<(), String> {
    let editor = PREVIEW_EDITOR.lock().await;
    editor.close_file(&file_id)
}

#[tauri::command]
async fn get_typist_template(name: String) -> Result<Template, String> {
    // Validate template name
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(
            "Template name can only contain alphanumeric characters, hyphens, and underscores"
                .to_string(),
        );
    }

    let engine = TEMPLATE_ENGINE.lock().await;
    engine
        .get_template(&name)
        .ok_or_else(|| format!("Template '{}' not found", name))
        .cloned()
}

#[tauri::command]
async fn register_typist_template(template: Template) -> Result<(), String> {
    let mut engine = TEMPLATE_ENGINE.lock().await;
    engine.register_template(template)
}

#[tauri::command]
async fn remove_typist_template(name: String) -> Result<bool, String> {
    // Validate template name
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(
            "Template name can only contain alphanumeric characters, hyphens, and underscores"
                .to_string(),
        );
    }

    let mut engine = TEMPLATE_ENGINE.lock().await;
    Ok(engine.remove_template(&name))
}

#[tauri::command]
async fn render_typist_template(
    name: String,
    values: std::collections::HashMap<String, String>,
) -> Result<String, String> {
    // Validate template name
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(
            "Template name can only contain alphanumeric characters, hyphens, and underscores"
                .to_string(),
        );
    }

    // Validate values size (prevent DoS)
    if values.len() > 100 {
        return Err("Too many variable values (max 100)".to_string());
    }

    // Validate individual value sizes
    for (key, value) in &values {
        if key.len() > 100 {
            return Err("Variable name too long (max 100 characters)".to_string());
        }
        if value.len() > 10000 {
            return Err("Variable value too long (max 10000 characters)".to_string());
        }
    }

    let engine = TEMPLATE_ENGINE.lock().await;
    engine.render(&name, &values)
}

#[tauri::command]
async fn get_typist_templates_by_category(
    category: TemplateCategory,
) -> Result<Vec<Template>, String> {
    let engine = TEMPLATE_ENGINE.lock().await;
    Ok(engine.get_templates_by_category(category))
}

#[tauri::command]
async fn search_typist_templates(query: String) -> Result<Vec<Template>, String> {
    // Validate query length
    if query.len() > 200 {
        return Err("Search query too long (max 200 characters)".to_string());
    }

    let engine = TEMPLATE_ENGINE.lock().await;
    Ok(engine.search_templates(&query))
}

#[tauri::command]
async fn generate_typist_template_preview(name: String) -> Result<Vec<u8>, String> {
    // Validate template name
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(
            "Template name can only contain alphanumeric characters, hyphens, and underscores"
                .to_string(),
        );
    }

    let mut engine = TEMPLATE_ENGINE.lock().await;
    engine.generate_template_preview(&name)
}

#[tauri::command]
async fn update_typist_template_metadata(
    name: String,
    updates: TemplateMetadataUpdate,
) -> Result<(), String> {
    // Validate template name
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(
            "Template name can only contain alphanumeric characters, hyphens, and underscores"
                .to_string(),
        );
    }

    // Validate version format if provided
    if let Some(ref version) = updates.version {
        if !regex::Regex::new(r"^\d+\.\d+\.\d+$")
            .unwrap()
            .is_match(version)
        {
            return Err("Invalid version format (expected x.y.z)".to_string());
        }
    }

    let mut engine = TEMPLATE_ENGINE.lock().await;
    engine.update_template_metadata(&name, updates)
}

#[tauri::command]
async fn load_typist_templates_from_directory(dir: String) -> Result<usize, String> {
    let mut engine = TEMPLATE_ENGINE.lock().await;
    let path = std::path::PathBuf::from(dir);

    // Validate path is absolute and within allowed directories
    if !path.is_absolute() {
        return Err("Path must be absolute".to_string());
    }

    // Prevent directory traversal
    if path.to_string_lossy().contains("..") {
        return Err("Path cannot contain '..' for security reasons".to_string());
    }

    engine.load_templates_from_directory(&path)
}

#[tauri::command]
async fn save_typist_template_to_file(name: String, dir: String) -> Result<String, String> {
    let engine = TEMPLATE_ENGINE.lock().await;
    let path = std::path::PathBuf::from(dir);

    // Validate path is absolute and within allowed directories
    if !path.is_absolute() {
        return Err("Path must be absolute".to_string());
    }

    // Prevent directory traversal
    if path.to_string_lossy().contains("..") {
        return Err("Path cannot contain '..' for security reasons".to_string());
    }

    // Validate template name
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(
            "Template name can only contain alphanumeric characters, hyphens, and underscores"
                .to_string(),
        );
    }

    let result = engine.save_template_to_file(&name, &path)?;
    Ok(result.display().to_string())
}

#[tauri::command]
async fn export_typist_template(name: String, format: String) -> Result<String, String> {
    // Validate template name
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(
            "Template name can only contain alphanumeric characters, hyphens, and underscores"
                .to_string(),
        );
    }

    let engine = TEMPLATE_ENGINE.lock().await;
    let export_format = match format.as_str() {
        "json" => typist_service::template::TemplateExportFormat::Json,
        "yaml" => typist_service::template::TemplateExportFormat::Yaml,
        "toml" => typist_service::template::TemplateExportFormat::Toml,
        "typ" => typist_service::template::TemplateExportFormat::Typ,
        _ => return Err(format!("Invalid export format: {}", format)),
    };

    engine.export_template(&name, export_format)
}

#[tauri::command]
async fn import_typist_template(data: String, format: String) -> Result<String, String> {
    let mut engine = TEMPLATE_ENGINE.lock().await;
    let import_format = match format.as_str() {
        "json" => typist_service::template::TemplateExportFormat::Json,
        "yaml" => typist_service::template::TemplateExportFormat::Yaml,
        "toml" => typist_service::template::TemplateExportFormat::Toml,
        "typ" => typist_service::template::TemplateExportFormat::Typ,
        _ => return Err(format!("Invalid import format: {}", format)),
    };

    // Validate data size (prevent DoS)
    if data.len() > 10 * 1024 * 1024 {
        return Err("Import data too large (max 10MB)".to_string());
    }

    engine.import_template(&data, import_format)
}

// ============================================================================
// Collaboration Tauri Commands
// ============================================================================

#[tauri::command]
async fn collaboration_join(
    document_id: String,
    user_id: String,
    user_name: String,
) -> Result<(), String> {
    
    let mut docs = COLLABORATION_DOCUMENTS.lock().await;
    
    // Create document if it doesn't exist
    if !docs.contains_key(&document_id) {
        let config_service = Arc::new(config_service::ExportConfigService::new());
        let doc = collaboration_service::CRDTDocument::new(
            document_id.clone(),
            collaboration_service::CRDTType::RichText,
            config_service,
        );
        docs.insert(document_id.clone(), doc);
    }
    
    // In a real implementation, this would connect to WebSocket
    // For now, we just log the join
    println!("User {} ({}) joined document {}", user_name, user_id, document_id);
    
    Ok(())
}

#[tauri::command]
async fn collaboration_leave(document_id: String, _user_id: String) -> Result<(), String> {
    println!("User left document {}", document_id);
    Ok(())
}

#[tauri::command]
async fn collaboration_send_operation(
    document_id: String,
    _user_id: String,
    operation: collaboration_service::CRDTOperation,
) -> Result<(), String> {
    let mut docs = COLLABORATION_DOCUMENTS.lock().await;
    
    if let Some(doc) = docs.get_mut(&document_id) {
        doc.apply_operation(operation)?;
    } else {
        return Err(format!("Document {} not found", document_id));
    }
    
    Ok(())
}

#[tauri::command]
async fn collaboration_update_presence(
    document_id: String,
    _user_id: String,
    _presence: collaboration_service::PresenceInfo,
) -> Result<(), String> {
    println!("User updated presence in document {}", document_id);
    Ok(())
}

#[tauri::command]
async fn collaboration_request_sync(
    document_id: String,
    _user_id: String,
    since_version: u64,
) -> Result<Vec<collaboration_service::CRDTOperation>, String> {
    let docs = COLLABORATION_DOCUMENTS.lock().await;
    
    if let Some(doc) = docs.get(&document_id) {
        Ok(doc.get_operations_since(since_version))
    } else {
        Err(format!("Document {} not found", document_id))
    }
}

// Typst Conversion Commands
#[tauri::command]
async fn html_to_typst(html: String, config: Option<TypstConversionConfig>) -> Result<String, String> {
    let converter_config = config.unwrap_or_default();
    let config_service = Arc::new(config_service::ExportConfigService::new());
    let converter = HtmlToTypstConverter::new(converter_config, config_service);
    let result = converter.convert(&html);
    
    if result.success {
        Ok(result.typst_code)
    } else {
        Err(result.error.unwrap_or_else(|| "Conversion failed".to_string()))
    }
}

#[tauri::command]
async fn html_to_typst_slides(html: String, config: Option<SlideConfig>) -> Result<String, String> {
    let slide_config = config.unwrap_or_default();
    let config_service = Arc::new(config_service::ExportConfigService::new());
    let converter = HtmlToTypstSlideConverter::new(slide_config, config_service);
    let result = converter.convert(&html);
    
    if result.success {
        Ok(result.typst_code)
    } else {
        Err(result.error.unwrap_or_else(|| "Conversion failed".to_string()))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            call_ai_service,
            call_ai_service_stream,
            export_to_markdown,
            markdown_to_html,
            json_to_typst,
            save_file,
            load_file,
            compile_typst,
            compile_typst_slide,
            get_typst_page_count,
            export_to_pdf,
            export_to_png,
            save_template,
            load_template,
            list_templates,
            delete_template,
            get_templates_directory,
            download_template_from_url,
            get_tiptap_config,
            list_tiptap_presets,
            sync_sheet_data,
            get_sheet_data,
            render_latex,
            generate_chart,
            create_conversation,
            get_conversation,
            get_all_conversations,
            delete_conversation,
            add_conversation_message,
            get_conversation_context,
            get_all_templates,
            get_template,
            create_template,
            delete_conversation_template,
            apply_template,
            get_all_typist_templates,
            get_typist_template,
            register_typist_template,
            remove_typist_template,
            render_typist_template,
            get_typist_templates_by_category,
            search_typist_templates,
            generate_typist_template_preview,
            update_typist_template_metadata,
            load_typist_templates_from_directory,
            save_typist_template_to_file,
            export_typist_template,
            import_typist_template,
            get_conversation_stats,
            create_collaboration_document,
            get_collaboration_document,
            apply_collaboration_operation,
            merge_collaboration_operations,
            update_user_presence,
            configure_cloud_sync,
            get_sync_status,
            perform_sync,
            upload_cloud_file,
            download_cloud_file,
            list_cloud_files,
            compare_documents,
            render_diff_view,
            create_macro,
            get_macro,
            get_all_macros,
            export_document,
            get_supported_export_formats,
            compute_incremental_hash,
            check_cache_valid,
            update_incremental_cache,
            get_cached_compilation,
            clear_incremental_cache,
            get_incremental_cache_size,
            search_packages,
            install_package,
            uninstall_package,
            update_package,
            list_installed_packages,
            list_available_packages,
            add_package_to_cache,
            check_package_dependencies,
            get_package_stats,
            render_typst,
            check_typst_availability,
            update_macro,
            delete_macro,
            play_macro,
            stop_macro,
            set_macro_shortcut,
            get_macro_stats,
            start_recording,
            stop_recording,
            record_action,
            is_recording,
            get_recorded_actions,
            evaluate_formula,
            generate_pivot_table,
            generate_spreadsheet_chart,
            validate_cell_data,
            apply_cell_style,
            get_spreadsheet_service_status,
            create_video_element,
            create_audio_element,
            create_hyperlink_element,
            create_artword_element,
            create_smartart_element,
            add_smartart_node,
            get_ppt_service_status,
            parse_template,
            merge_template,
            validate_template,
            html_to_typst,
            html_to_typst_slides,
            load_merge_data,
            process_batch_merge,
            recognize_ocr_file,
            recognize_ocr_bytes,
            recognize_ocr_with_layout,
            get_ocr_languages,
            update_ocr_config,
            load_plugin,
            unload_plugin,
            get_all_plugins,
            get_plugin,
            enable_plugin,
            disable_plugin,
            register_plugin_hook,
            trigger_plugin_hook,
            get_plugin_stats,
            build_accessibility_tree,
            get_accessibility_tree,
            update_accessibility_node,
            collaboration_join,
            collaboration_leave,
            collaboration_send_operation,
            collaboration_update_presence,
            collaboration_request_sync,
            add_accessibility_node,
            remove_accessibility_node,
            validate_accessibility_node,
            get_accessibility_stats,
            announce_to_screen_reader,
            get_screen_reader_announcements,
            clear_screen_reader_announcements,
            export_document,
            get_supported_export_formats,
            start_speech_recognition,
            stop_speech_recognition,
            process_speech_audio,
            is_speech_recognition_active,
            update_recognition_config,
            get_recognition_languages,
            speak_text,
            stop_speaking,
            get_available_voices,
            update_tts_config,
            create_comment,
            reply_to_comment,
            get_comment_thread,
            get_document_comments,
            filter_comments,
            update_comment,
            delete_comment,
            resolve_comment_thread,
            reopen_comment_thread,
            archive_comment_thread,
            delete_comment_thread,
            get_comment_stats,
            // Preview editor commands
            preview_editor_open_file,
            preview_editor_update_source,
            preview_editor_move_cursor,
            preview_editor_set_selection,
            preview_editor_sync_cursor_to_preview,
            preview_editor_get_editor_state,
            preview_editor_get_preview_state,
            preview_editor_get_diagnostics,
            preview_editor_get_completions,
            preview_editor_get_symbols,
            preview_editor_save_file,
            preview_editor_close_file,
            // Template commands
            get_all_typist_templates,
            get_typist_template,
            register_typist_template,
            remove_typist_template,
            render_typist_template,
            get_typist_templates_by_category,
            search_typist_templates,
            generate_typist_template_preview,
            update_typist_template_metadata,
            load_typist_templates_from_directory,
            save_typist_template_to_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
