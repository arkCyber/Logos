/*!
 * Aerospace-Grade HTML Block Parser for SVG Export
 *
 * Converts common HTML block tags into positioned SVG text/layout elements while
 * preserving basic typography (headings, emphasis, inline color/font-size).
 */

use super::config::{SvgConfig, ViewBox};
use super::element::{SvgElement, validate_text_content};
use super::sanitize::escape_svg_text;
use super::style::{SvgFill, SvgFont, SvgStroke, SvgStyle};

/// Left margin for block layout in SVG user units
const MARGIN_X: f64 = 20.0;

/// Initial vertical cursor for block layout
const START_Y: f64 = 30.0;

/// Default block spacing in SVG user units
const BLOCK_SPACING: f64 = 12.0;

/// Default canvas width for HTML-derived SVG documents
const DEFAULT_CANVAS_WIDTH: f64 = 800.0;

/// Options controlling HTML to SVG conversion limits
#[derive(Debug, Clone)]
pub struct HtmlToSvgOptions {
    pub max_text_length: usize,
    pub max_blocks: usize,
}

impl Default for HtmlToSvgOptions {
    fn default() -> Self {
        Self {
            max_text_length: super::element::MAX_TEXT_CONTENT_LENGTH,
            max_blocks: 10_000,
        }
    }
}

/// Parsed HTML layout result
#[derive(Debug, Clone)]
pub struct HtmlLayoutResult {
    pub config: SvgConfig,
    pub elements: Vec<SvgElement>,
}

#[derive(Debug, Clone, Copy, Default)]
struct CssSides {
    top: f64,
    right: f64,
    bottom: f64,
    left: f64,
}

#[derive(Debug, Clone, Default)]
struct BlockBoxStyle {
    margin: CssSides,
    padding: CssSides,
    border_width: f64,
    border_color: Option<(u8, u8, u8)>,
    background_color: Option<(u8, u8, u8)>,
    width: Option<f64>,
    display_none: bool,
}

#[derive(Debug, Clone)]
struct ParsedBlock {
    text: String,
    color: Option<(u8, u8, u8)>,
    font_size: f64,
    bold: bool,
    italic: bool,
    box_style: BlockBoxStyle,
}

/// Parse HTML content into SVG config and elements using block-flow layout.
pub fn parse_html_layout(html: &str, options: &HtmlToSvgOptions) -> HtmlLayoutResult {
    let blocks = extract_blocks(html, options.max_blocks);
    let mut elements = Vec::new();
    let mut y = START_Y;

    if blocks.is_empty() {
        elements.push(SvgElement::text(
            "text0".to_string(),
            MARGIN_X,
            START_Y,
            "SVG".to_string(),
        ));
        return HtmlLayoutResult {
            config: build_config(START_Y + 20.0),
            elements,
        };
    }

    for (index, block) in blocks.iter().enumerate() {
        if block.box_style.display_none {
            continue;
        }
        if block.text.len() > options.max_text_length {
            continue;
        }
        if validate_text_content(&block.text).is_err() {
            continue;
        }

        let margin = block.box_style.margin;
        let padding = block.box_style.padding;
        let content_width = block.box_style.width.unwrap_or_else(|| {
            (DEFAULT_CANVAS_WIDTH - (2.0 * MARGIN_X) - margin.left - margin.right).max(20.0)
        });
        let box_x = MARGIN_X + margin.left;
        let box_y = y + margin.top;
        let text_height = block.font_size * 1.4;
        let box_height = padding.top + text_height + padding.bottom;
        let text_x = box_x + padding.left;
        let text_y = box_y + padding.top + block.font_size;

        if block.box_style.background_color.is_some() || block.box_style.border_width > 0.0 {
            let mut rect_style = SvgStyle::new();
            if let Some(bg) = block.box_style.background_color {
                rect_style.fill = SvgFill::new().with_color(bg.0, bg.1, bg.2);
            } else {
                rect_style.fill = SvgFill::none();
            }
            if block.box_style.border_width > 0.0 {
                let border_color = block.box_style.border_color.unwrap_or((0, 0, 0));
                rect_style.stroke = SvgStroke::new()
                    .with_color(border_color.0, border_color.1, border_color.2)
                    .with_width(block.box_style.border_width);
            }
            elements.push(
                SvgElement::rect(
                    format!("box{}", index),
                    box_x,
                    box_y,
                    content_width + padding.left + padding.right,
                    box_height,
                )
                .with_style(rect_style),
            );
        }

        let mut style = SvgStyle::new();
        if let Some(color) = block.color {
            style.fill = SvgFill::new().with_color(color.0, color.1, color.2);
        }
        style.font = Some(
            SvgFont::new()
                .with_size(block.font_size)
                .with_weight(if block.bold {
                    "bold".to_string()
                } else {
                    "normal".to_string()
                })
                .with_style(if block.italic {
                    "italic".to_string()
                } else {
                    "normal".to_string()
                }),
        );

        elements.push(
            SvgElement::text(format!("text{}", index), text_x, text_y, block.text.clone())
                .with_style(style),
        );
        y = box_y + box_height + margin.bottom + BLOCK_SPACING;
    }

    if elements.is_empty() {
        elements.push(SvgElement::text(
            "text0".to_string(),
            MARGIN_X,
            START_Y,
            "SVG".to_string(),
        ));
        y = START_Y + 20.0;
    }

    HtmlLayoutResult {
        config: build_config(y + 20.0),
        elements,
    }
}

fn build_config(content_height: f64) -> SvgConfig {
    let height = content_height.max(100.0);
    SvgConfig::new()
        .with_size(DEFAULT_CANVAS_WIDTH, height)
        .with_view_box(ViewBox::new(0.0, 0.0, DEFAULT_CANVAS_WIDTH, height))
}

fn extract_blocks(html: &str, max_blocks: usize) -> Vec<ParsedBlock> {
    let mut blocks = Vec::new();
    let normalized = html
        .replace("<br>", "\n")
        .replace("<br/>", "\n")
        .replace("<br />", "\n");
    let mut cursor = 0;

    while cursor < normalized.len() && blocks.len() < max_blocks {
        if let Some((open_tag, content, next)) = find_next_block(&normalized[cursor..]) {
            blocks.push(parse_block(&open_tag, &content));
            cursor += next;
        } else {
            break;
        }
    }

    if blocks.is_empty() {
        for line in normalized.lines() {
            let text = strip_tags(line);
            if !text.is_empty() {
                blocks.push(ParsedBlock {
                    text,
                    color: None,
                    font_size: 14.0,
                    bold: false,
                    italic: false,
                    box_style: BlockBoxStyle::default(),
                });
            }
            if blocks.len() >= max_blocks {
                break;
            }
        }
    }

    blocks
}

fn find_next_block(html: &str) -> Option<(String, String, usize)> {
    let lower = html.to_ascii_lowercase();
    for tag in ["h1", "h2", "h3", "h4", "h5", "h6", "p", "div", "li"] {
        let open_needle = format!("<{}", tag);
        let Some(rel_start) = lower.find(&open_needle) else {
            continue;
        };
        let start = rel_start;
        let gt_rel = lower[start..].find('>')?;
        let gt = start + gt_rel;
        let content_start = gt + 1;
        let close_needle = format!("</{}>", tag);
        let close_rel = lower[content_start..].find(&close_needle)?;
        let content_end = content_start + close_rel;
        let open_tag = html[start..=gt].to_string();
        let content = html[content_start..content_end].to_string();
        let next = content_end + close_needle.len();
        return Some((open_tag, content, next));
    }
    None
}

fn parse_block(open_tag: &str, content: &str) -> ParsedBlock {
    let tag = extract_tag_name(open_tag).unwrap_or_else(|| "p".to_string());
    let style_attr = extract_attribute(open_tag, "style");
    let inline_style = parse_inline_style(style_attr.as_deref().unwrap_or_default());

    ParsedBlock {
        text: strip_tags(content),
        color: inline_style.color,
        font_size: inline_style
            .font_size
            .unwrap_or_else(|| default_font_size(&tag)),
        bold: contains_tag(content, "strong") || contains_tag(content, "b"),
        italic: contains_tag(content, "em") || contains_tag(content, "i"),
        box_style: inline_style.box_style,
    }
}

fn extract_tag_name(open_tag: &str) -> Option<String> {
    let trimmed = open_tag.trim_start_matches('<').trim_end_matches('>');
    trimmed
        .split_whitespace()
        .next()
        .map(|tag| tag.to_ascii_lowercase())
}

#[derive(Debug, Clone, Default)]
struct InlineStyle {
    color: Option<(u8, u8, u8)>,
    font_size: Option<f64>,
    box_style: BlockBoxStyle,
}

fn parse_inline_style(style: &str) -> InlineStyle {
    let mut parsed = InlineStyle::default();
    for rule in style.split(';') {
        let mut parts = rule.split(':');
        let key = parts.next().unwrap_or("").trim().to_ascii_lowercase();
        let value = parts.next().unwrap_or("").trim();
        match key.as_str() {
            "color" => parsed.color = parse_css_color(value),
            "font-size" => parsed.font_size = parse_css_length(value),
            "margin" => parsed.box_style.margin = parse_css_sides(value),
            "margin-top" => parsed.box_style.margin.top = parse_css_length(value).unwrap_or(0.0),
            "margin-right" => parsed.box_style.margin.right = parse_css_length(value).unwrap_or(0.0),
            "margin-bottom" => parsed.box_style.margin.bottom = parse_css_length(value).unwrap_or(0.0),
            "margin-left" => parsed.box_style.margin.left = parse_css_length(value).unwrap_or(0.0),
            "padding" => parsed.box_style.padding = parse_css_sides(value),
            "padding-top" => parsed.box_style.padding.top = parse_css_length(value).unwrap_or(0.0),
            "padding-right" => parsed.box_style.padding.right = parse_css_length(value).unwrap_or(0.0),
            "padding-bottom" => parsed.box_style.padding.bottom = parse_css_length(value).unwrap_or(0.0),
            "padding-left" => parsed.box_style.padding.left = parse_css_length(value).unwrap_or(0.0),
            "border" => apply_border_shorthand(&mut parsed.box_style, value),
            "border-width" => {
                parsed.box_style.border_width = parse_css_length(value).unwrap_or(1.0);
            }
            "border-color" => parsed.box_style.border_color = parse_css_color(value),
            "background-color" => parsed.box_style.background_color = parse_css_color(value),
            "width" => parsed.box_style.width = parse_css_length(value),
            "display" => {
                if value.eq_ignore_ascii_case("none") {
                    parsed.box_style.display_none = true;
                }
            }
            _ => {}
        }
    }
    parsed
}

fn apply_border_shorthand(box_style: &mut BlockBoxStyle, value: &str) {
    let tokens: Vec<&str> = value.split_whitespace().collect();
    for token in tokens {
        if let Some(width) = parse_css_length(token) {
            box_style.border_width = width;
        } else if let Some(color) = parse_css_color(token) {
            box_style.border_color = Some(color);
        }
    }
}

fn parse_css_sides(value: &str) -> CssSides {
    let values: Vec<f64> = value
        .split_whitespace()
        .filter_map(parse_css_length)
        .collect();
    match values.len() {
        1 => CssSides {
            top: values[0],
            right: values[0],
            bottom: values[0],
            left: values[0],
        },
        2 => CssSides {
            top: values[0],
            right: values[1],
            bottom: values[0],
            left: values[1],
        },
        3 => CssSides {
            top: values[0],
            right: values[1],
            bottom: values[2],
            left: values[1],
        },
        _ if values.len() >= 4 => CssSides {
            top: values[0],
            right: values[1],
            bottom: values[2],
            left: values[3],
        },
        _ => CssSides::default(),
    }
}

fn parse_css_length(value: &str) -> Option<f64> {
    let value = value.trim();
    if value.ends_with('%') {
        let percent: f64 = value.trim_end_matches('%').parse().ok()?;
        return Some(DEFAULT_CANVAS_WIDTH * percent / 100.0);
    }
    if let Some(px) = value.strip_suffix("px") {
        return px.parse().ok();
    }
    value.parse().ok()
}

fn parse_font_size(value: &str) -> Option<f64> {
    parse_css_length(value)
}

fn parse_css_color(value: &str) -> Option<(u8, u8, u8)> {
    let value = value.trim();
    if value.starts_with('#') && value.len() == 7 {
        let r = u8::from_str_radix(&value[1..3], 16).ok()?;
        let g = u8::from_str_radix(&value[3..5], 16).ok()?;
        let b = u8::from_str_radix(&value[5..7], 16).ok()?;
        return Some((r, g, b));
    }
    if value.starts_with("rgb(") && value.ends_with(')') {
        let inner = &value[4..value.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(str::trim).collect();
        if parts.len() == 3 {
            let r = parts[0].parse().ok()?;
            let g = parts[1].parse().ok()?;
            let b = parts[2].parse().ok()?;
            return Some((r, g, b));
        }
    }
    None
}

fn default_font_size(tag: &str) -> f64 {
    match tag {
        "h1" => 28.0,
        "h2" => 24.0,
        "h3" => 20.0,
        "h4" => 18.0,
        "h5" => 16.0,
        "h6" => 14.0,
        _ => 14.0,
    }
}

fn extract_attribute(fragment: &str, attr: &str) -> Option<String> {
    let lower = fragment.to_ascii_lowercase();
    let needle = format!("{}=", attr);
    let start = lower.find(&needle)? + needle.len();
    let bytes = fragment.as_bytes();
    if bytes.get(start) == Some(&b'"') {
        let end = fragment[start + 1..].find('"')? + start + 1;
        return Some(fragment[start + 1..end].to_string());
    }
    None
}

fn contains_tag(html: &str, tag: &str) -> bool {
    html.to_ascii_lowercase().contains(&format!("<{}", tag))
}

fn strip_tags(html: &str) -> String {
    let html = remove_blocked_sections(html, "script");
    let html = remove_blocked_sections(&html, "style");

    let mut result = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn remove_blocked_sections(html: &str, tag: &str) -> String {
    let mut result = html.to_string();
    let open = format!("<{}", tag);
    let close = format!("</{}>", tag);

    loop {
        let lower = result.to_ascii_lowercase();
        let Some(start) = lower.find(&open) else {
            break;
        };
        let Some(close_rel) = lower[start..].find(&close) else {
            result.replace_range(start.., "");
            break;
        };
        let end = start + close_rel + close.len();
        result.replace_range(start..end, "");
    }

    result
}

/// Strip tags and escape text for safe SVG preview snippets.
pub fn html_to_plain_text(html: &str) -> String {
    escape_svg_text(&strip_tags(html))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading_block() {
        let layout = parse_html_layout("<h1>Title</h1><p>Body</p>", &HtmlToSvgOptions::default());
        assert_eq!(layout.elements.len(), 2);
        assert!(layout.config.height >= 100.0);
    }

    #[test]
    fn test_parse_inline_style_color() {
        let layout = parse_html_layout(
            "<p style=\"color: #ff0000; font-size: 18px;\">Red text</p>",
            &HtmlToSvgOptions::default(),
        );
        assert_eq!(layout.elements.len(), 1);
        assert_eq!(layout.elements[0].style.fill.color, Some((255, 0, 0)));
    }

    #[test]
    fn test_parse_bold_and_italic() {
        let layout = parse_html_layout(
            "<p><strong>Bold</strong> and <em>italic</em></p>",
            &HtmlToSvgOptions::default(),
        );
        let font = layout.elements[0].style.font.as_ref().unwrap();
        assert_eq!(font.weight, "bold");
        assert_eq!(font.style, "italic");
    }

    #[test]
    fn test_strip_tags_removes_script() {
        assert_eq!(
            strip_tags("<p>Hello<script>alert(1)</script></p>"),
            "Hello"
        );
    }

    #[test]
    fn test_parse_display_none_skips_block() {
        use crate::svg_service::element::SvgElementType;

        let layout = parse_html_layout(
            "<p style=\"display: none;\">Hidden</p><p>Visible</p>",
            &HtmlToSvgOptions::default(),
        );
        assert_eq!(layout.elements.len(), 1);
        assert!(matches!(layout.elements[0].element_type, SvgElementType::Text));
        assert!(layout.elements[0]
            .text
            .as_ref()
            .unwrap()
            .text
            .contains("Visible"));
    }

    #[test]
    fn test_parse_margin_and_background() {
        use crate::svg_service::element::SvgElementType;

        let layout = parse_html_layout(
            "<p style=\"margin: 10px; background-color: #eeeeee;\">Boxed</p>",
            &HtmlToSvgOptions::default(),
        );
        assert_eq!(layout.elements.len(), 2);
        assert!(matches!(
            layout.elements[0].element_type,
            SvgElementType::Rect
        ));
    }

    #[test]
    fn test_parse_border_shorthand() {
        let layout = parse_html_layout(
            "<p style=\"border: 2px solid #000000;\">Bordered</p>",
            &HtmlToSvgOptions::default(),
        );
        assert_eq!(layout.elements.len(), 2);
        assert!(layout.elements[0].style.stroke.width >= 2.0);
    }
}
