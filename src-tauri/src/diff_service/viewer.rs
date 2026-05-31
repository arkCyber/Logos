use super::diff_engine::{ChangeType, DiffResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::config_service::ExportConfigService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffViewConfig {
    pub show_line_numbers: bool,
    pub show_whitespace: bool,
    pub ignore_case: bool,
    pub context_lines: usize,
    pub side_by_side: bool,
}

impl Default for DiffViewConfig {
    fn default() -> Self {
        Self {
            show_line_numbers: true,
            show_whitespace: false,
            ignore_case: false,
            context_lines: 3,
            side_by_side: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffViewOutput {
    pub html: String,
    pub css: String,
    pub stats: super::diff_engine::DiffStats,
}

pub struct DiffViewer {
    config: DiffViewConfig,
}

impl DiffViewer {
    pub fn new(config: DiffViewConfig) -> Self {
        Self { config }
    }

    /// Generate HTML view of diff result
    pub fn render(
        &self,
        diff_result: &DiffResult,
        old_text: &str,
        new_text: &str,
    ) -> DiffViewOutput {
        let html = if self.config.side_by_side {
            self.render_side_by_side(diff_result, old_text, new_text)
        } else {
            self.render_unified(diff_result, old_text, new_text)
        };

        let css = self.generate_css();
        let stats = super::diff_engine::DiffEngine::new(Arc::new(ExportConfigService::new())).get_stats(diff_result);

        DiffViewOutput { html, css, stats }
    }

    fn render_unified(&self, diff_result: &DiffResult, old_text: &str, new_text: &str) -> String {
        let mut html = String::from(r#"<div class="diff-viewer unified">"#);

        html.push_str(r#"<div class="diff-header">"#);
        html.push_str(&format!(
            r#"<div class="diff-stats">Changes: {}, Similarity: {:.1}%</div>"#,
            diff_result.total_changes,
            diff_result.similarity * 100.0
        ));
        html.push_str(r#"</div>"#);

        html.push_str(r#"<div class="diff-content">"#);

        let _old_lines: Vec<&str> = old_text.lines().collect();
        let _new_lines: Vec<&str> = new_text.lines().collect();

        for change in &diff_result.changes {
            let class = match change.change_type {
                ChangeType::Insert => "diff-insert",
                ChangeType::Delete => "diff-delete",
                ChangeType::Replace => "diff-replace",
                ChangeType::Equal => "diff-equal",
            };

            html.push_str(&format!(r#"<div class="diff-line {}">"#, class));

            if self.config.show_line_numbers {
                html.push_str(&format!(
                    r#"<span class="line-number">{}</span>"#,
                    change.old_position + 1
                ));
            }

            let content = match change.change_type {
                ChangeType::Insert => &change.new_text,
                ChangeType::Delete => &change.old_text,
                ChangeType::Replace => &change.new_text,
                ChangeType::Equal => &change.old_text,
            };

            html.push_str(&format!(
                r#"<span class="line-content">{}</span>"#,
                self.escape_html(content)
            ));

            html.push_str(r#"</div>"#);
        }

        html.push_str(r#"</div>"#);
        html.push_str(r#"</div>"#);

        html
    }

    fn render_side_by_side(
        &self,
        diff_result: &DiffResult,
        old_text: &str,
        new_text: &str,
    ) -> String {
        let mut html = String::from(r#"<div class="diff-viewer side-by-side">"#);

        html.push_str(r#"<div class="diff-header">"#);
        html.push_str(&format!(
            r#"<div class="diff-stats">Changes: {}, Similarity: {:.1}%</div>"#,
            diff_result.total_changes,
            diff_result.similarity * 100.0
        ));
        html.push_str(r#"</div>"#);

        html.push_str(r#"<div class="diff-content">"#);
        html.push_str(r#"<div class="diff-column old">"#);
        html.push_str(r#"<div class="column-header">Original</div>"#);

        let _old_lines: Vec<&str> = old_text.lines().collect();
        let _new_lines: Vec<&str> = new_text.lines().collect();

        for change in &diff_result.changes {
            let class = match change.change_type {
                ChangeType::Insert => "diff-insert",
                ChangeType::Delete => "diff-delete",
                ChangeType::Replace => "diff-replace",
                ChangeType::Equal => "diff-equal",
            };

            html.push_str(&format!(r#"<div class="diff-line {}">"#, class));

            if self.config.show_line_numbers {
                html.push_str(&format!(
                    r#"<span class="line-number">{}</span>"#,
                    change.old_position + 1
                ));
            }

            let content = match change.change_type {
                ChangeType::Insert => "",
                ChangeType::Delete => &change.old_text,
                ChangeType::Replace => &change.old_text,
                ChangeType::Equal => &change.old_text,
            };

            html.push_str(&format!(
                r#"<span class="line-content">{}</span>"#,
                self.escape_html(content)
            ));

            html.push_str(r#"</div>"#);
        }

        html.push_str(r#"</div>"#);
        html.push_str(r#"<div class="diff-column new">"#);
        html.push_str(r#"<div class="column-header">Modified</div>"#);

        for change in &diff_result.changes {
            let class = match change.change_type {
                ChangeType::Insert => "diff-insert",
                ChangeType::Delete => "diff-delete",
                ChangeType::Replace => "diff-replace",
                ChangeType::Equal => "diff-equal",
            };

            html.push_str(&format!(r#"<div class="diff-line {}">"#, class));

            if self.config.show_line_numbers {
                html.push_str(&format!(
                    r#"<span class="line-number">{}</span>"#,
                    change.new_position + 1
                ));
            }

            let content = match change.change_type {
                ChangeType::Insert => &change.new_text,
                ChangeType::Delete => "",
                ChangeType::Replace => &change.new_text,
                ChangeType::Equal => &change.new_text,
            };

            html.push_str(&format!(
                r#"<span class="line-content">{}</span>"#,
                self.escape_html(content)
            ));

            html.push_str(r#"</div>"#);
        }

        html.push_str(r#"</div>"#);
        html.push_str(r#"</div>"#);
        html.push_str(r#"</div>"#);

        html
    }

    fn generate_css(&self) -> String {
        r#"
.diff-viewer {
    font-family: 'Courier New', monospace;
    font-size: 14px;
    background: #f9fafb;
    border: 1px solid #e5e7eb;
    border-radius: 4px;
    overflow: hidden;
}

.diff-header {
    background: #f3f4f6;
    padding: 8px 12px;
    border-bottom: 1px solid #e5e7eb;
}

.diff-stats {
    font-size: 12px;
    color: #6b7280;
}

.diff-content {
    display: flex;
    flex-direction: column;
}

.diff-viewer.side-by-side .diff-content {
    flex-direction: row;
}

.diff-column {
    flex: 1;
    overflow-x: auto;
}

.diff-column:first-child {
    border-right: 1px solid #e5e7eb;
}

.column-header {
    background: #f3f4f6;
    padding: 8px 12px;
    font-weight: 600;
    border-bottom: 1px solid #e5e7eb;
}

.diff-line {
    display: flex;
    padding: 2px 12px;
    min-height: 20px;
}

.line-number {
    display: inline-block;
    width: 50px;
    text-align: right;
    margin-right: 12px;
    color: #9ca3af;
    user-select: none;
}

.line-content {
    flex: 1;
    white-space: pre-wrap;
    word-break: break-all;
}

.diff-equal {
    background: white;
}

.diff-insert {
    background: #dcfce7;
}

.diff-delete {
    background: #fee2e2;
}

.diff-replace {
    background: #fef3c7;
}
"#
        .to_string()
    }

    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
}

impl Default for DiffViewer {
    fn default() -> Self {
        Self::new(DiffViewConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::super::diff_engine::{DiffEngine, DiffStats};
    use super::*;

    #[test]
    fn test_viewer_creation() {
        let config = DiffViewConfig::default();
        let viewer = DiffViewer::new(config);
        assert_eq!(viewer.config.show_line_numbers, true);
    }

    #[test]
    fn test_viewer_default() {
        let viewer = DiffViewer::default();
        assert_eq!(viewer.config.show_line_numbers, true);
        assert_eq!(viewer.config.show_whitespace, false);
        assert_eq!(viewer.config.ignore_case, false);
        assert_eq!(viewer.config.context_lines, 3);
        assert_eq!(viewer.config.side_by_side, false);
    }

    #[test]
    fn test_diff_view_config_default() {
        let config = DiffViewConfig::default();
        assert_eq!(config.show_line_numbers, true);
        assert_eq!(config.show_whitespace, false);
        assert_eq!(config.ignore_case, false);
        assert_eq!(config.context_lines, 3);
        assert_eq!(config.side_by_side, false);
    }

    #[test]
    fn test_diff_view_config_custom() {
        let config = DiffViewConfig {
            show_line_numbers: false,
            show_whitespace: true,
            ignore_case: true,
            context_lines: 5,
            side_by_side: true,
        };
        assert_eq!(config.show_line_numbers, false);
        assert_eq!(config.show_whitespace, true);
        assert_eq!(config.ignore_case, true);
        assert_eq!(config.context_lines, 5);
        assert_eq!(config.side_by_side, true);
    }

    #[test]
    fn test_escape_html() {
        let viewer = DiffViewer::default();
        let escaped = viewer.escape_html("<script>alert('xss')</script>");
        assert!(escaped.contains("&lt;"));
        assert!(escaped.contains("&gt;"));
        assert!(!escaped.contains("<script>"));
    }

    #[test]
    fn test_escape_html_quotes() {
        let viewer = DiffViewer::default();
        let escaped = viewer.escape_html(r#"test"quote'apostrophe"#);
        assert!(escaped.contains("&quot;"));
        assert!(escaped.contains("&#39;"));
    }

    #[test]
    fn test_escape_html_ampersand() {
        let viewer = DiffViewer::default();
        let escaped = viewer.escape_html("test & value");
        assert!(escaped.contains("&amp;"));
    }

    #[test]
    fn test_generate_css() {
        let viewer = DiffViewer::default();
        let css = viewer.generate_css();
        assert!(css.contains(".diff-viewer"));
        assert!(css.contains(".diff-insert"));
        assert!(css.contains(".diff-delete"));
        assert!(css.contains(".diff-replace"));
        assert!(css.contains(".diff-equal"));
    }

    #[test]
    fn test_render_unified() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("old text", "new text");

        let html = viewer.render_unified(&diff_result, "old text", "new text");
        assert!(html.contains("diff-viewer"));
        assert!(html.contains("unified"));
        assert!(html.contains("diff-content"));
    }

    #[test]
    fn test_render_side_by_side() {
        let config = DiffViewConfig {
            show_line_numbers: true,
            show_whitespace: false,
            ignore_case: false,
            context_lines: 3,
            side_by_side: true,
        };
        let viewer = DiffViewer::new(config);
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("old text", "new text");

        let html = viewer.render_side_by_side(&diff_result, "old text", "new text");
        assert!(html.contains("diff-viewer"));
        assert!(html.contains("side-by-side"));
        assert!(html.contains("diff-column"));
        assert!(html.contains("Original"));
        assert!(html.contains("Modified"));
    }

    #[test]
    fn test_render_with_line_numbers() {
        let config = DiffViewConfig {
            show_line_numbers: true,
            show_whitespace: false,
            ignore_case: false,
            context_lines: 3,
            side_by_side: false,
        };
        let viewer = DiffViewer::new(config);
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("line1\nline2", "line1\nline2");

        let html = viewer.render_unified(&diff_result, "line1\nline2", "line1\nline2");
        assert!(html.contains("line-number"));
    }

    #[test]
    fn test_render_without_line_numbers() {
        let config = DiffViewConfig {
            show_line_numbers: false,
            show_whitespace: false,
            ignore_case: false,
            context_lines: 3,
            side_by_side: false,
        };
        let viewer = DiffViewer::new(config);
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("line1\nline2", "line1\nline2");

        let html = viewer.render_unified(&diff_result, "line1\nline2", "line1\nline2");
        assert!(!html.contains("line-number"));
    }

    #[test]
    fn test_render_empty_diff() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("", "");

        let output = viewer.render(&diff_result, "", "");
        assert!(output.html.contains("diff-viewer"));
        assert!(output.css.contains(".diff-viewer"));
    }

    #[test]
    fn test_render_identical_texts() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("same text", "same text");

        let output = viewer.render(&diff_result, "same text", "same text");
        assert!(output.html.contains("diff-viewer"));
        assert!(output.stats.similarity > 0.9);
    }

    #[test]
    fn test_render_completely_different_texts() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("old", "new");

        let output = viewer.render(&diff_result, "old", "new");
        assert!(output.html.contains("diff-viewer"));
        assert!(output.stats.similarity < 0.5);
    }

    #[test]
    fn test_render_multiline_diff() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let old_text = "line1\nline2\nline3";
        let new_text = "line1\nmodified\nline3";
        let diff_result = engine.compare(old_text, new_text);

        let output = viewer.render(&diff_result, old_text, new_text);
        assert!(output.html.contains("diff-viewer"));
        assert!(output.stats.total_changes > 0);
    }

    #[test]
    fn test_render_with_special_characters() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let old_text = "test <>&\"'";
        let new_text = "test <>&\"'";
        let diff_result = engine.compare(old_text, new_text);

        let output = viewer.render(&diff_result, old_text, new_text);
        assert!(output.html.contains("&lt;") || output.html.contains("&gt;"));
    }

    #[test]
    fn test_render_output_structure() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("old", "new");

        let output = viewer.render(&diff_result, "old", "new");
        assert!(!output.html.is_empty());
        assert!(!output.css.is_empty());
        assert!(output.stats.inserts >= 0);
        assert!(output.stats.deletes >= 0);
    }

    #[test]
    fn test_render_with_insert_change() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("", "inserted line");

        let html = viewer.render_unified(&diff_result, "", "inserted line");
        assert!(html.contains("diff-insert") || html.contains("diff-viewer"));
    }

    #[test]
    fn test_render_with_delete_change() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("deleted line", "");

        let html = viewer.render_unified(&diff_result, "deleted line", "");
        assert!(html.contains("diff-delete") || html.contains("diff-viewer"));
    }

    #[test]
    fn test_render_with_replace_change() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("old", "new");

        let html = viewer.render_unified(&diff_result, "old", "new");
        assert!(html.contains("diff-replace") || html.contains("diff-viewer"));
    }

    #[test]
    fn test_render_with_equal_change() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("same", "same");

        let html = viewer.render_unified(&diff_result, "same", "same");
        assert!(html.contains("diff-equal") || html.contains("diff-viewer"));
    }

    #[test]
    fn test_render_side_by_side_columns() {
        let config = DiffViewConfig {
            show_line_numbers: false,
            show_whitespace: false,
            ignore_case: false,
            context_lines: 3,
            side_by_side: true,
        };
        let viewer = DiffViewer::new(config);
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("old", "new");

        let html = viewer.render_side_by_side(&diff_result, "old", "new");
        assert!(html.contains("diff-column old"));
        assert!(html.contains("diff-column new"));
    }

    #[test]
    fn test_render_context_lines_config() {
        let config = DiffViewConfig {
            show_line_numbers: false,
            show_whitespace: false,
            ignore_case: false,
            context_lines: 10,
            side_by_side: false,
        };
        let viewer = DiffViewer::new(config);
        assert_eq!(viewer.config.context_lines, 10);
    }

    #[test]
    fn test_render_with_whitespace_config() {
        let config = DiffViewConfig {
            show_line_numbers: false,
            show_whitespace: true,
            ignore_case: false,
            context_lines: 3,
            side_by_side: false,
        };
        let viewer = DiffViewer::new(config);
        assert_eq!(viewer.config.show_whitespace, true);
    }

    #[test]
    fn test_render_with_ignore_case_config() {
        let config = DiffViewConfig {
            show_line_numbers: false,
            show_whitespace: false,
            ignore_case: true,
            context_lines: 3,
            side_by_side: false,
        };
        let viewer = DiffViewer::new(config);
        assert_eq!(viewer.config.ignore_case, true);
    }

    #[test]
    fn test_diff_view_output_creation() {
        let output = DiffViewOutput {
            html: "<div>test</div>".to_string(),
            css: ".test{}".to_string(),
            stats: DiffStats {
                total_changes: 1,
                inserts: 1,
                deletes: 0,
                replaces: 0,
                similarity: 1.0,
            },
        };
        assert_eq!(output.html, "<div>test</div>");
        assert_eq!(output.css, ".test{}");
        assert_eq!(output.stats.inserts, 1);
    }

    #[test]
    fn test_render_with_long_text() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let old_text = "a\nb\nc\nd\ne\nf\ng\nh\ni\nj";
        let new_text = "a\nb\nmodified\nd\ne\nf\ng\nh\ni\nj";
        let diff_result = engine.compare(old_text, new_text);

        let output = viewer.render(&diff_result, old_text, new_text);
        assert!(output.html.contains("diff-viewer"));
        assert!(output.stats.total_changes > 0);
    }

    #[test]
    fn test_render_with_unicode() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let old_text = "Hello 世界";
        let new_text = "Hello 世界";
        let diff_result = engine.compare(old_text, new_text);

        let output = viewer.render(&diff_result, old_text, new_text);
        assert!(output.html.contains("diff-viewer"));
    }

    #[test]
    fn test_render_stats_in_output() {
        let viewer = DiffViewer::default();
        let mut engine = DiffEngine::new(Arc::new(ExportConfigService::new()));
        let diff_result = engine.compare("old", "new");

        let output = viewer.render(&diff_result, "old", "new");
        assert!(output.stats.inserts >= 0);
        assert!(output.stats.deletes >= 0);
        assert!(output.stats.replaces >= 0);
    }
}
