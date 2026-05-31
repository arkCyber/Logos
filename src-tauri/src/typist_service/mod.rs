//! Typist Service - Aerospace-Grade Typst Rendering Service
//!
//! Safety-critical Typst rendering service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

pub mod advanced_math;
pub mod ast_mapping;
pub mod bibliography;
pub mod box_container;
pub mod cjk_typography;
pub mod code;
pub mod color_management;
pub mod columns;
pub mod compiler;
pub mod counter;
pub mod data_loader;
pub mod equation;
pub mod export;
pub mod figure;
pub mod font_loader;
pub mod fonts;
pub mod footnote;
pub mod foundations;
pub mod glossary;
pub mod gradient;
pub mod grid_system;
pub mod heading;
pub mod image;
pub mod incremental;
pub mod index;
pub mod layout;
pub mod layout_enhanced;
pub mod line;
pub mod link;
pub mod list;
pub mod localization;
pub mod lsp_service;
pub mod master_page;
pub mod math_enhanced;
pub mod metadata;
pub mod model_enhanced;
pub mod operation_converter;
pub mod outline;
pub mod package;
pub mod page;
pub mod page_header_footer;
pub mod paragraph;
pub mod path;
pub mod plugin;
pub mod preview_editor;
pub mod query;
pub mod quote;
pub mod raw;
pub mod reference;
pub mod renderer;
pub mod scripting;
pub mod shapes;
pub mod state;
pub mod styling;
pub mod symbols;
pub mod syntax;
pub mod table;
pub mod template;
pub mod text_enhanced;
pub mod text_formatting;
pub mod theorem;
pub mod typography;
pub mod visualize_enhanced;

#[cfg(test)]
mod tests;

pub use advanced_math::{Binomial, BracketStyle, Cancel, Cases, Matrix, MatrixAlignment, Primes};
pub use ast_mapping::{AstMapping, AstNodeMapping, AstNodeType, SourceLocation, VisualLocation};
pub use bibliography::{
    BibEntry, BibEntryType, Bibliography, BibliographyBuilder, BibliographyConfig, CitationStyle,
};
pub use box_container::{Box, BoxBuilder, BoxConfig, BoxPadding, BoxRadius, BoxSize, BoxStroke};
pub use code::{
    CodeBlock, CodeBlockBuilder, CodeBlockConfig, CodeBlockManager, CodeLanguage, CodeTheme,
};
pub use columns::{Columns, ColumnsBuilder, ColumnsConfig};
pub use compiler::{CompileOptions, TypstCompiler};
pub use counter::{CounterKey, CounterSystem, CounterValue, NumberingStyle};
pub use data_loader::{DataLoader, DataType};
pub use equation::{Equation, EquationAlign, EquationBuilder, EquationConfig};
pub use export::{HtmlExporter, SvgExporter};
pub use figure::{Figure, FigureBuilder, FigureConfig, FigureKind, FigurePlacement};
pub use fonts::{
    Font, FontBuilder, FontConfig, FontFamily, FontStyle as FontStyleType,
    FontWeight as FontWeightType,
};
pub use footnote::{Footnote, FootnoteBuilder, FootnoteConfig};
pub use foundations::{
    Arguments, ArrayOps, Assert, Auto, BoolOps, BytesOps, CalcOps, Content, DateTimeOps,
    DecimalOps, DictionaryOps, DurationOps, EvalOps, ExportTarget, FloatOps,
    FoundationLabel as FoundationLabelType, FoundationSymbol as FoundationSymbolType,
    FoundationValue, FunctionOps, IntOps, ModuleOps, NoneValue, Panic, RegexOps, Repr, Selector,
    SelectorType, Std, StringOps, Sys, TypeOps, VersionOps,
};
pub use glossary::{Glossary, GlossaryBuilder, GlossaryConfig, GlossaryEntry, GlossaryStyle};
pub use gradient::{
    Gradient, GradientBuilder, GradientStop, GradientType, Pattern, TilingMode, Transform,
};
pub use heading::{
    Heading, HeadingBuilder, HeadingConfig, HeadingLevel, NumberingStyle as HeadingNumberingStyle,
};
pub use image::{
    Image, ImageBuilder, ImageConfig, ImageFilter, ImageFit, ImageFormat, ImageScaling, ImageSize,
};
pub use incremental::{CacheEntry, IncrementalCompiler, IncrementalConfig};
pub use index::{Index, IndexBuilder, IndexConfig, IndexEntry, IndexStyle};
pub use layout::{GridLayout, LayoutConfig, StackLayout};
pub use layout_enhanced::{
    Align, AlignConfig, Alignment, Angle, Block, Colbreak, Direction, Fraction, FractionConfig,
    Hide, LayoutInfo, Length, LengthUnit, Measure, MeasureResult, Move, Pad, Place, PlaceConfig,
    PlacePosition, Repeat, H,
};
pub use line::{Line, LineBuilder, LineConfig, Point};
pub use link::{Link, LinkBuilder, LinkConfig, LinkDestination};
pub use list::{List, ListBuilder, ListConfig, ListItem, ListMarker, ListType};
pub use localization::{Language, Localization, LocalizationBuilder, LocalizationConfig};
pub use lsp_service::{
    CompletionItemKind, DiagnosticSeverity, LspCompletionItem, LspConfig, LspDiagnostic,
    LspService, LspSymbol, SymbolKind,
};
pub use math_enhanced::{
    AccentType, AttachType, MathAccent, MathAttach, MathClass, MathClassOp, MathFrac, MathLr,
    MathOp, MathRoot, MathSize, MathSizes, MathStretch, MathStyle, MathStyles, MathUnderover,
    MathVariant, MathVariants, MathVec, RootType,
};
pub use metadata::{DocumentMetadata, Metadata, MetadataBuilder, MetadataEntry, MetadataValue};
pub use model_enhanced::{Cite, ParBreak, Term, Title};
pub use operation_converter::{
    CodeOperation, CodeOperationApplier, ConversionError, OperationConverter, VisualOperation,
};
pub use outline::{
    Outline, OutlineBuilder, OutlineConfig, OutlineEntry, OutlineEntryType, OutlineIndent,
};
pub use package::{
    PackageInfo, PackageManager, PackageManagerConfig, PackageRepository, PackageStats,
};
pub use page::{Page, PageAlignment, PageBuilder, PageConfig, PaperSize};
pub use page_header_footer::{
    FooterConfig, HeaderConfig, HeaderFooterContent, PageHeaderFooter, PageHeaderFooterBuilder,
    PageNumberAlign, PageNumberConfig, PageNumberStyle,
};
pub use paragraph::{
    Paragraph, ParagraphAlign, ParagraphBuilder, ParagraphConfig, ParagraphIndent, ParagraphSpacing,
};
pub use path::{BoundingBox, Path, PathBuilder, PathCommand, PathOperations};
pub use plugin::{
    Plugin, PluginContext, PluginManager, PluginMetadata, PluginPermission, PluginResult,
    PluginStatus,
};
pub use preview_editor::{
    EditorEvent, EditorState, PreviewEditor, PreviewEditorConfig, PreviewState,
};
pub use query::{QueryEngine, QueryResult};
pub use quote::{Attribution, Quote, QuoteBuilder, QuoteConfig};
pub use raw::{Raw, RawBuilder, RawConfig, RawType};
pub use reference::{
    Label, LabelType, Reference, ReferenceBuilder, ReferenceStyle, ReferenceSystem,
};
pub use renderer::{RenderOptions, TypstRenderer};
pub use scripting::{ScriptFunction, ScriptValue, ScriptVariable, Scripting, ScriptingBuilder};
pub use shapes::{Color, Fill, Shape, ShapeBuilder, ShapeType, Stroke};
pub use state::{StateManager, StateValue};
pub use styling::{
    StyleRule, StyleRuleType, StyleSelector, StyleValue, Styling, StylingBuilder, Theme,
};
pub use symbols::{Symbol, SymbolCategory, SymbolRegistry};
pub use syntax::{HighlightedSpan, SyntaxHighlighter, TokenType};
pub use table::{
    Table, TableAlign, TableBuilder, TableCell, TableConfig, TableRow, TableSize, TableStroke,
};
pub use template::{Template, TemplateEngine, TemplateVariable};
pub use text_enhanced::{Highlight, LineBreak, LineBreakType, Lorem, Overline, Strike};
pub use text_formatting::{
    FontStyle, FontWeight, QuoteStyle, TextDecoration, TextFormatter, TextStyle, TextTransform,
};
pub use theorem::{Theorem, TheoremBuilder, TheoremConfig, TheoremType};
pub use typography::{
    FontPairing, FontPairingSystem, KerningPair, KerningTable, OpenTypeFeature, OpenTypeFeatures,
    TypographyConfig, TypographyEngine,
};
pub use cjk_typography::{
    CJKLanguage, CJKTypographyConfig, CJKTypographyEngine, CompressionContext, LineBreakRule,
    PunctuationCompressionRule, PunctuationWidth, WritingMode,
};
pub use color_management::{
    CMYKColor, ColorManagementConfig, ColorManagementSystem, ColorSpace, ICCProfile, PantoneColor,
    RGBColor, RenderingIntent,
};
pub use grid_system::{
    ColumnWidthConfig, ColumnWidthOptimizer, GridConfig, GridSystem, GridType, LayoutBalanceConfig,
    LayoutBalanceEngine, LayoutIssues, LayoutSystem, LayoutSystemConfig, PageMarginConfig,
};
pub use master_page::{
    MasterPage, MasterPageSystem, PageElement, PageElementType, PageInstance, PageStyle,
};
pub use visualize_enhanced::{
    Curve, CurveType, Polygon, VisualizeColor, VisualizePoint, VisualizeStroke,
};

// Tauri command types for Typst rendering
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use std::sync::Mutex;
use crate::config_service::ExportConfigService;

/// Global operation counter for typist service
static OPERATION_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Global error state for typist service
static LAST_ERROR: Mutex<Option<ErrorContext>> = Mutex::new(None);

/// Validate source code length
fn validate_source(source: &str, config_service: &ExportConfigService) -> Result<(), String> {
    let typist_config = config_service.get_typist_config();
    if source.len() > typist_config.max_source_length {
        return Err(format!("Source code exceeds maximum length of {}", typist_config.max_source_length));
    }
    Ok(())
}

/// Validate output size
fn validate_output_size(size: usize, config_service: &ExportConfigService) -> Result<(), String> {
    let typist_config = config_service.get_typist_config();
    if size > typist_config.max_output_size {
        return Err(format!("Output exceeds maximum size of {}", typist_config.max_output_size));
    }
    Ok(())
}

/// Record error context
fn record_error(code: &str, message: &str, source: &str) {
    let mut last_error = LAST_ERROR.lock().unwrap();
    *last_error = Some(ErrorContext::new(
        ErrorSeverity::Error,
        code,
        message,
        source,
    ));
}

/// Get last error
pub fn get_last_error() -> Option<ErrorContext> {
    let last_error = LAST_ERROR.lock().unwrap();
    last_error.clone()
}

/// Get operation count
pub fn get_operation_count() -> u64 {
    OPERATION_COUNTER.load(Ordering::SeqCst)
}

/// Reset error state
pub fn reset_error_state() {
    let mut last_error = LAST_ERROR.lock().unwrap();
    *last_error = None;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypstRenderRequest {
    pub source: String,
    pub format: TypstOutputFormat,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TypstOutputFormat {
    Pdf,
    Svg,
    Png,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypstRenderResponse {
    pub success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn render_typst(
    request: TypstRenderRequest,
) -> Result<TypstRenderResponse, String> {
    OPERATION_COUNTER.fetch_add(1, Ordering::SeqCst);

    let config_service = Arc::new(ExportConfigService::new());

    // Validate source code
    if let Err(e) = validate_source(&request.source, &config_service) {
        record_error("INVALID_SOURCE", &e, "render_typst");
        return Ok(TypstRenderResponse {
            success: false,
            output: None,
            error: Some(e),
        });
    }

    let compiler = TypstCompiler::new();
    
    // Compile the source code
    let document = match compiler.compile(request.source) {
        Ok(doc) => doc,
        Err(e) => {
            let error = format!("Compilation failed: {}", e);
            record_error("COMPILATION_FAILED", &error, "render_typst");
            return Ok(TypstRenderResponse {
                success: false,
                output: None,
                error: Some(error),
            });
        }
    };

    // Render based on format
    let output: Result<String, String> = match request.format {
        TypstOutputFormat::Pdf => {
            let pdf_bytes = TypstRenderer::export_to_pdf(&document)?;
            if let Err(e) = validate_output_size(pdf_bytes.len(), &config_service) {
                record_error("OUTPUT_TOO_LARGE", &e, "render_typst");
                return Err(e);
            }
            use base64::{engine::general_purpose::STANDARD, Engine as _};
            let base64_output = STANDARD.encode(&pdf_bytes);
            Ok(base64_output)
        }
        TypstOutputFormat::Svg => {
            let svg_string = export::SvgExporter::new().export(&document)?;
            if let Err(e) = validate_output_size(svg_string.len(), &config_service) {
                record_error("OUTPUT_TOO_LARGE", &e, "render_typst");
                return Err(e);
            }
            use base64::{engine::general_purpose::STANDARD, Engine as _};
            let base64_output = STANDARD.encode(svg_string.as_bytes());
            Ok(base64_output)
        }
        TypstOutputFormat::Png => {
            let png_bytes = TypstRenderer::render_first_page_to_png(&document, 144.0)?;
            if let Err(e) = validate_output_size(png_bytes.len(), &config_service) {
                record_error("OUTPUT_TOO_LARGE", &e, "render_typst");
                return Err(e);
            }
            use base64::{engine::general_purpose::STANDARD, Engine as _};
            let base64_output = STANDARD.encode(&png_bytes);
            Ok(base64_output)
        }
    };

    match output {
        Ok(data) => {
            reset_error_state();
            Ok(TypstRenderResponse {
                success: true,
                output: Some(data),
                error: None,
            })
        }
        Err(e) => Ok(TypstRenderResponse {
            success: false,
            output: None,
            error: Some(format!("Rendering failed: {}", e)),
        }),
    }
}

#[tauri::command]
pub async fn check_typst_availability() -> Result<bool, String> {
    // Since we're using the Rust library, it's always available
    Ok(true)
}

#[cfg(test)]
pub use font_loader::FontLoader;
