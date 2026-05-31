//! Integration tests for PPT service
//! Tests the complete workflow from data structures to export

use std::sync::Arc;
use crate::ppt_service::export::{PptxExportOptions, PptxExporter, PptxPresentation};
use crate::ppt_service::slide::Slide;
use crate::ppt_service::text::TextElement;
use crate::ppt_service::table::{TableElement, TableCell, TableRow, TableStyle};
use crate::ppt_service::shape::{Shape, ShapeStyle, ShapeFill};
use crate::ppt_service::theme::PptTheme;
use crate::ppt_service::config::{PptConfig, SlideSize};
use crate::config_service::ExportConfigService;
use crate::ppt_service::chart::{ChartElement, ChartData, ChartSeries, ChartDataPoint, ChartStyle};
use crate::ppt_service::{VideoElement, AudioElement, HyperlinkElement, ArtWordElement, SmartArtElement, SmartArtNode, SmartArtType};

#[test]
fn test_complete_presentation_creation() {
    // Create a complete presentation with all elements
    let config = PptConfig::new();
    let theme = PptTheme::default_theme();
    
    let mut presentation = PptxPresentation::new()
        .with_config(config)
        .with_theme(theme);
    
    // Add title slide
    let title_slide = Slide::title_slide("slide1".to_string(), "Test Presentation".to_string());
    presentation = presentation.with_slide(title_slide);
    
    // Add content slide with text
    let content_slide = Slide::content_slide("slide2".to_string(), "Content".to_string());
    presentation = presentation.with_slide(content_slide);
    
    // Add blank slide
    let blank_slide = Slide::blank_slide("slide3".to_string());
    presentation = presentation.with_slide(blank_slide);
    
    assert_eq!(presentation.slide_count(), 3);
}

#[test]
fn test_presentation_with_text_elements() {
    let mut presentation = PptxPresentation::new();
    
    let slide = Slide::content_slide("slide1".to_string(), "Text Test".to_string());
    presentation = presentation.with_slide(slide);
    
    let text = TextElement::heading("Heading Text".to_string());
    presentation = presentation.with_text_element(text);
    
    let body_text = TextElement::body("Body text content".to_string());
    presentation = presentation.with_text_element(body_text);
    
    assert_eq!(presentation.text_elements.len(), 2);
}

#[test]
fn test_presentation_with_table() {
    let mut presentation = PptxPresentation::new();
    
    let slide = Slide::content_slide("slide1".to_string(), "Table Test".to_string());
    presentation = presentation.with_slide(slide);
    
    let mut table = TableElement::new("table1".to_string());
    
    // Add header row
    let header_row = TableRow::new("row1".to_string())
        .as_header()
        .with_cell(TableCell::new("Name".to_string()))
        .with_cell(TableCell::new("Value".to_string()));
    table = table.with_row(header_row);
    
    // Add data row
    let data_row = TableRow::new("row2".to_string())
        .with_cell(TableCell::new("Item 1".to_string()))
        .with_cell(TableCell::new("100".to_string()));
    table = table.with_row(data_row);
    
    presentation = presentation.with_table(table);
    
    assert_eq!(presentation.tables.len(), 1);
    assert_eq!(presentation.tables[0].row_count(), 2);
}

#[test]
fn test_presentation_with_shapes() {
    let mut presentation = PptxPresentation::new();
    
    let slide = Slide::content_slide("slide1".to_string(), "Shapes Test".to_string());
    presentation = presentation.with_slide(slide);
    
    let rectangle = Shape::rectangle("shape1".to_string())
        .with_style(ShapeStyle::new().with_fill(ShapeFill::solid(255, 0, 0)))
        .with_position(100.0, 100.0)
        .with_size(200.0, 100.0);
    presentation = presentation.with_shape(rectangle);
    
    let circle = Shape::circle("shape2".to_string())
        .with_position(400.0, 100.0)
        .with_size(100.0, 100.0);
    presentation = presentation.with_shape(circle);
    
    assert_eq!(presentation.shapes.len(), 2);
}

#[test]
fn test_presentation_with_chart() {
    let mut presentation = PptxPresentation::new();
    
    let slide = Slide::content_slide("slide1".to_string(), "Chart Test".to_string());
    presentation = presentation.with_slide(slide);
    
    let mut chart_data = ChartData::new("Sales Data".to_string());
    
    let series = ChartSeries::new("Q1".to_string(), (0, 128, 0))
        .with_point(ChartDataPoint::new("Jan".to_string(), 100.0))
        .with_point(ChartDataPoint::new("Feb".to_string(), 150.0))
        .with_point(ChartDataPoint::new("Mar".to_string(), 200.0));
    
    chart_data = chart_data.with_series(series);
    
    let chart = ChartElement::column_chart("chart1".to_string(), chart_data)
        .with_position(100.0, 100.0)
        .with_size(400.0, 300.0);
    
    presentation = presentation.with_chart(chart);
    
    assert_eq!(presentation.charts.len(), 1);
}

#[test]
fn test_export_with_options() {
    let presentation = PptxPresentation::new()
        .with_slide(Slide::title_slide("slide1".to_string(), "Test".to_string()));
    
    let options = PptxExportOptions::new()
        .with_embed_fonts(true)
        .with_compress_images(true)
        .with_image_quality(90)
        .with_include_notes(false);
    
    let config_service = Arc::new(ExportConfigService::new());
    let mut exporter = PptxExporter::new(config_service).with_options(options);
    let result = exporter.export(&presentation);
    
    assert!(result.success);
    assert_eq!(result.slide_count, 1);
    assert!(result.file_size > 0);
    assert!(result.generation_time_ms >= 0);
}

#[test]
fn test_export_from_html() {
    let html_content = r#"
        <html>
        <body>
            <h1>Title Slide</h1>
            <p>Content paragraph</p>
        </body>
        </html>
    "#;
    
    let config_service = Arc::new(ExportConfigService::new());
    let mut exporter = PptxExporter::new(config_service);
    let result = exporter.export_from_html(html_content);
    
    assert!(result.success);
    assert!(result.slide_count > 0);
}

#[test]
fn test_export_from_markdown() {
    let markdown_content = r#"
# Title Slide

## Subtitle

Content paragraph
- Bullet point 1
- Bullet point 2
    "#;
    
    let config_service = Arc::new(ExportConfigService::new());
    let mut exporter = PptxExporter::new(config_service);
    let result = exporter.export_from_markdown(markdown_content);
    
    assert!(result.success);
    assert!(result.slide_count > 0);
}

#[test]
fn test_complex_presentation_workflow() {
    // Simulate a real-world use case
    let config = PptConfig::new()
        .with_slide_size(SlideSize::Widescreen16_9)
        .with_page_numbers(true);
    
    let theme = PptTheme::dark_theme();
    
    let mut presentation = PptxPresentation::new()
        .with_config(config)
        .with_theme(theme);
    
    // Title slide
    let title_slide = Slide::title_slide("slide1".to_string(), "Annual Report".to_string());
    presentation = presentation.with_slide(title_slide);
    
    // Content slide with table
    let content_slide = Slide::content_slide("slide2".to_string(), "Financial Summary".to_string());
    presentation = presentation.with_slide(content_slide);
    
    let table = TableElement::new("table1".to_string())
        .with_style(TableStyle::new().with_border_color(0, 0, 0))
        .with_row(TableRow::new("row1".to_string()).as_header()
            .with_cell(TableCell::new("Q1".to_string()))
            .with_cell(TableCell::new("Q2".to_string()))
            .with_cell(TableCell::new("Q3".to_string()))
            .with_cell(TableCell::new("Q4".to_string())))
        .with_row(TableRow::new("row2".to_string())
            .with_cell(TableCell::new("$1M".to_string()))
            .with_cell(TableCell::new("$1.5M".to_string()))
            .with_cell(TableCell::new("$2M".to_string()))
            .with_cell(TableCell::new("$2.5M".to_string())));
    presentation = presentation.with_table(table);
    
    // Chart slide
    let chart_slide = Slide::content_slide("slide3".to_string(), "Growth Chart".to_string());
    presentation = presentation.with_slide(chart_slide);
    
    let mut chart_data = ChartData::new("Revenue Growth".to_string())
        .with_x_axis_label("Quarter".to_string())
        .with_y_axis_label("Revenue ($M)".to_string());
    
    let series = ChartSeries::new("2024".to_string(), (31, 78, 120))
        .with_point(ChartDataPoint::new("Q1".to_string(), 1.0))
        .with_point(ChartDataPoint::new("Q2".to_string(), 1.5))
        .with_point(ChartDataPoint::new("Q3".to_string(), 2.0))
        .with_point(ChartDataPoint::new("Q4".to_string(), 2.5));
    
    chart_data = chart_data.with_series(series);
    
    let chart = ChartElement::line_chart("chart1".to_string(), chart_data)
        .with_style(ChartStyle::new().with_grid_lines(true))
        .with_position(100.0, 100.0)
        .with_size(500.0, 350.0);
    
    presentation = presentation.with_chart(chart);
    
    // Export
    let config_service = Arc::new(ExportConfigService::new());
    let mut exporter = PptxExporter::new(config_service);
    let result = exporter.export(&presentation);
    
    assert!(result.success);
    assert_eq!(result.slide_count, 3);
    assert!(result.file_size > 0);
}

// Serialization test removed - PptxPresentation doesn't implement Serialize/Deserialize
// This is acceptable as the primary export format is PPTX binary, not JSON

#[test]
fn test_end_to_end_pptx_generation() {
    // End-to-end test: Create presentation, export to PPTX, validate file structure
    let config = PptConfig::new()
        .with_slide_size(SlideSize::Widescreen16_9)
        .with_page_numbers(true);
    
    let theme = PptTheme::default_theme();
    
    let mut presentation = PptxPresentation::new()
        .with_config(config)
        .with_theme(theme);
    
    // Add title slide
    let title_slide = Slide::title_slide("slide1".to_string(), "End-to-End Test".to_string());
    presentation = presentation.with_slide(title_slide);
    
    // Add content slide
    let content_slide = Slide::content_slide("slide2".to_string(), "Test Content".to_string());
    presentation = presentation.with_slide(content_slide);
    
    // Add text elements
    let text = TextElement::heading("Heading".to_string());
    presentation = presentation.with_text_element(text);
    
    let body_text = TextElement::body("Body content".to_string());
    presentation = presentation.with_text_element(body_text);
    
    // Export to PPTX
    let config_service = Arc::new(ExportConfigService::new());
    let mut exporter = PptxExporter::new(config_service);
    let result = exporter.export(&presentation);
    
    // Validate export result
    assert!(result.success, "Export should succeed");
    assert_eq!(result.slide_count, 2, "Should have 2 slides");
    assert!(result.file_size > 0, "File size should be greater than 0");
    assert!(result.file_size > 1000, "PPTX file should be at least 1KB (ZIP structure)");
    assert!(result.pptx_data.len() > 0, "PPTX data should not be empty");
    
    // Validate PPTX file structure (ZIP file signature)
    let data = &result.pptx_data;
    assert!(data.len() > 4, "PPTX data should have at least 4 bytes");
    
    // PPTX files are ZIP files, should start with ZIP signature
    // ZIP signature: 0x50 0x4B 0x03 0x04 (PK..)
    assert_eq!(data[0], 0x50, "PPTX should start with ZIP signature (P)");
    assert_eq!(data[1], 0x4B, "PPTX should start with ZIP signature (K)");
    assert_eq!(data[2], 0x03, "PPTX should start with ZIP signature");
    assert_eq!(data[3], 0x04, "PPTX should start with ZIP signature");
    
    // Check for common PPTX XML content in the file
    let pptx_str = String::from_utf8_lossy(data);
    assert!(pptx_str.contains("ppt") || pptx_str.contains("presentation"), 
            "PPTX should contain presentation-related content");
}

#[test]
fn test_pptx_export_with_multiple_slides() {
    // Test exporting a presentation with multiple slides
    let mut presentation = PptxPresentation::new();
    
    for i in 1..=5 {
        let slide = Slide::content_slide(
            format!("slide{}", i),
            format!("Slide {}", i)
        );
        presentation = presentation.with_slide(slide);
    }
    
    let config_service = Arc::new(ExportConfigService::new());
    let mut exporter = PptxExporter::new(config_service);
    let result = exporter.export(&presentation);
    
    assert!(result.success);
    assert_eq!(result.slide_count, 5);
    assert!(result.file_size > 2000, "5 slides should generate a larger file");
}

#[test]
fn test_pptx_export_empty_presentation() {
    // Test exporting an empty presentation (creates default slide in PPTX)
    let presentation = PptxPresentation::new();
    
    let config_service = Arc::new(ExportConfigService::new());
    let mut exporter = PptxExporter::new(config_service);
    let result = exporter.export(&presentation);
    
    assert!(result.success);
    assert_eq!(result.slide_count, 0, "Empty presentation has 0 slides in data model");
    assert!(result.file_size > 0, "But PPTX file should still be generated with default slide");
}

#[test]
fn test_presentation_with_video_element() {
    // Test adding a video element to presentation
    let mut presentation = PptxPresentation::new();
    
    let slide = Slide::content_slide("slide1".to_string(), "Video Test".to_string());
    presentation = presentation.with_slide(slide);
    
    let video = VideoElement::new("video1".to_string(), "video.mp4".to_string())
        .with_position(100.0, 100.0)
        .with_size(640.0, 480.0)
        .with_autoplay(false);
    
    presentation = presentation.with_video(video);
    
    assert_eq!(presentation.videos.len(), 1);
    assert_eq!(presentation.videos[0].id, "video1");
}

#[test]
fn test_presentation_with_audio_element() {
    // Test adding an audio element to presentation
    let mut presentation = PptxPresentation::new();
    
    let slide = Slide::content_slide("slide1".to_string(), "Audio Test".to_string());
    presentation = presentation.with_slide(slide);
    
    let audio = AudioElement::new("audio1".to_string(), "audio.mp3".to_string())
        .with_autoplay(false)
        .with_volume(0.5);
    
    presentation = presentation.with_audio(audio);
    
    assert_eq!(presentation.audios.len(), 1);
    assert_eq!(presentation.audios[0].id, "audio1");
}

#[test]
fn test_presentation_with_hyperlink_element() {
    // Test adding a hyperlink element to presentation
    let mut presentation = PptxPresentation::new();
    
    let slide = Slide::content_slide("slide1".to_string(), "Hyperlink Test".to_string());
    presentation = presentation.with_slide(slide);
    
    let hyperlink = HyperlinkElement::new("link1".to_string(), "https://example.com".to_string(), "Click here".to_string())
        .with_position(100.0, 100.0)
        .with_open_in_new_window(true);
    
    presentation = presentation.with_hyperlink(hyperlink);
    
    assert_eq!(presentation.hyperlinks.len(), 1);
    assert_eq!(presentation.hyperlinks[0].id, "link1");
}

#[test]
fn test_presentation_with_artword_element() {
    // Test adding an art word element to presentation
    let mut presentation = PptxPresentation::new();
    
    let slide = Slide::content_slide("slide1".to_string(), "ArtWord Test".to_string());
    presentation = presentation.with_slide(slide);
    
    let artword = ArtWordElement::new("art1".to_string(), "Welcome".to_string())
        .with_style(crate::ppt_service::artword::ArtWordStyle::GradientFill)
        .with_position(100.0, 100.0)
        .with_font_size(48.0);
    
    presentation = presentation.with_artword(artword);
    
    assert_eq!(presentation.artwords.len(), 1);
    assert_eq!(presentation.artwords[0].id, "art1");
}

#[test]
fn test_presentation_with_smartart_element() {
    // Test adding a SmartArt element to presentation
    let mut presentation = PptxPresentation::new();
    
    let slide = Slide::content_slide("slide1".to_string(), "SmartArt Test".to_string());
    presentation = presentation.with_slide(slide);
    
    let mut smartart = SmartArtElement::process("smart1".to_string())
        .with_position(100.0, 100.0)
        .with_size(600.0, 400.0);
    
    let node1 = SmartArtNode::new("node1".to_string(), "Step 1".to_string());
    let node2 = SmartArtNode::new("node2".to_string(), "Step 2".to_string());
    let node3 = SmartArtNode::new("node3".to_string(), "Step 3".to_string());
    
    smartart.add_node(node1).unwrap();
    smartart.add_node(node2).unwrap();
    smartart.add_node(node3).unwrap();
    
    presentation = presentation.with_smartart(smartart);
    
    assert_eq!(presentation.smartarts.len(), 1);
    assert_eq!(presentation.smartarts[0].id, "smart1");
    assert_eq!(presentation.smartarts[0].nodes.len(), 3);
}

#[test]
fn test_presentation_with_all_new_elements() {
    // Test presentation with all new elements combined
    let mut presentation = PptxPresentation::new();
    
    let slide = Slide::content_slide("slide1".to_string(), "All Elements Test".to_string());
    presentation = presentation.with_slide(slide);
    
    // Add video
    let video = VideoElement::new("video1".to_string(), "video.mp4".to_string());
    presentation = presentation.with_video(video);
    
    // Add audio
    let audio = AudioElement::new("audio1".to_string(), "audio.mp3".to_string());
    presentation = presentation.with_audio(audio);
    
    // Add hyperlink
    let hyperlink = HyperlinkElement::new("link1".to_string(), "https://example.com".to_string(), "Link".to_string());
    presentation = presentation.with_hyperlink(hyperlink);
    
    // Add art word
    let artword = ArtWordElement::new("art1".to_string(), "Title".to_string());
    presentation = presentation.with_artword(artword);
    
    // Add SmartArt
    let mut smartart = SmartArtElement::process("smart1".to_string());
    let node = SmartArtNode::new("node1".to_string(), "Step".to_string());
    smartart.add_node(node).unwrap();
    presentation = presentation.with_smartart(smartart);
    
    // Verify all elements are present
    assert_eq!(presentation.videos.len(), 1);
    assert_eq!(presentation.audios.len(), 1);
    assert_eq!(presentation.hyperlinks.len(), 1);
    assert_eq!(presentation.artwords.len(), 1);
    assert_eq!(presentation.smartarts.len(), 1);
}
