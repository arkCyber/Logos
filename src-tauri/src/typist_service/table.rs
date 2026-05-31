/*!
 * 航空航天级表格系统
 * 实现 Typst 的表格功能（创建、格式化、样式）
 */

use serde::{Deserialize, Serialize};

/// 表格对齐方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TableAlign {
    Left,
    Center,
    Right,
    Top,
    Horizon,
    Bottom,
}

/// 表格尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableSize {
    Auto,
    Fixed(f64),
    Relative(f64),
    Fraction(f64),
}

/// 表格边框
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableStroke {
    pub thickness: f64,
    pub color: String,
}

impl Default for TableStroke {
    fn default() -> Self {
        Self {
            thickness: 0.5,
            color: "black".to_string(),
        }
    }
}

/// 表格单元格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    pub content: String,
    pub colspan: usize,
    pub rowspan: usize,
    pub align: Option<TableAlign>,
    pub fill: Option<String>,
    pub stroke: Option<TableStroke>,
}

impl TableCell {
    pub fn new(content: String) -> Self {
        Self {
            content,
            colspan: 1,
            rowspan: 1,
            align: None,
            fill: None,
            stroke: None,
        }
    }

    pub fn with_colspan(mut self, colspan: usize) -> Self {
        self.colspan = colspan;
        self
    }

    pub fn with_rowspan(mut self, rowspan: usize) -> Self {
        self.rowspan = rowspan;
        self
    }

    pub fn with_align(mut self, align: TableAlign) -> Self {
        self.align = Some(align);
        self
    }

    pub fn with_fill(mut self, fill: String) -> Self {
        self.fill = Some(fill);
        self
    }

    pub fn with_stroke(mut self, stroke: TableStroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

/// 表格行
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub is_header: bool,
    pub is_footer: bool,
}

impl TableRow {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            is_header: false,
            is_footer: false,
        }
    }

    pub fn header() -> Self {
        Self {
            cells: Vec::new(),
            is_header: true,
            is_footer: false,
        }
    }

    pub fn footer() -> Self {
        Self {
            cells: Vec::new(),
            is_header: false,
            is_footer: true,
        }
    }

    pub fn add_cell(mut self, cell: TableCell) -> Self {
        self.cells.push(cell);
        self
    }
}

impl Default for TableRow {
    fn default() -> Self {
        Self::new()
    }
}

/// 表格配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableConfig {
    pub columns: Vec<TableSize>,
    pub rows: Vec<TableSize>,
    pub gutter: TableSize,
    pub column_gutter: Option<TableSize>,
    pub row_gutter: Option<TableSize>,
    pub inset: f64,
    pub align: Option<TableAlign>,
    pub fill: Option<String>,
    pub stroke: Option<TableStroke>,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            columns: vec![TableSize::Auto],
            rows: vec![TableSize::Auto],
            gutter: TableSize::Relative(0.5),
            column_gutter: None,
            row_gutter: None,
            inset: 0.5,
            align: None,
            fill: None,
            stroke: None,
        }
    }
}

/// 表格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub rows: Vec<TableRow>,
    pub config: TableConfig,
}

impl Table {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            config: TableConfig::default(),
        }
    }

    pub fn with_config(config: TableConfig) -> Self {
        Self {
            rows: Vec::new(),
            config,
        }
    }

    pub fn add_row(&mut self, row: TableRow) {
        self.rows.push(row);
    }

    /// 转换为 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();

        typst.push_str("#table(\n");

        // 添加列配置
        if !self.config.columns.is_empty() && self.config.columns.len() > 1 {
            typst.push_str("  columns: (");
            for (i, col) in self.config.columns.iter().enumerate() {
                if i > 0 {
                    typst.push_str(", ");
                }
                match col {
                    TableSize::Auto => typst.push_str("auto"),
                    TableSize::Fixed(size) => typst.push_str(&format!("{}pt", size)),
                    TableSize::Relative(size) => typst.push_str(&format!("{}fr", size)),
                    TableSize::Fraction(size) => typst.push_str(&format!("{}%", size * 100.0)),
                }
            }
            typst.push_str("),\n");
        }

        // 添加对齐
        if let Some(align) = &self.config.align {
            typst.push_str(&format!("  align: {},\n", self.align_to_typst(align)));
        }

        // 添加边框
        if let Some(stroke) = &self.config.stroke {
            typst.push_str(&format!(
                "  stroke: {}pt + {},\n",
                stroke.thickness, stroke.color
            ));
        }

        // 添加行
        for row in &self.rows {
            typst.push_str("  ");
            if row.is_header {
                typst.push_str("#table.header[");
            } else if row.is_footer {
                typst.push_str("#table.footer[");
            } else {
                typst.push('[');
            }

            for (i, cell) in row.cells.iter().enumerate() {
                if i > 0 {
                    typst.push_str(", ");
                }
                typst.push_str(&format!("[{}]", html_escape(&cell.content)));
            }

            typst.push_str("],\n");
        }

        typst.push_str(")\n");

        typst
    }

    fn align_to_typst(&self, align: &TableAlign) -> String {
        match align {
            TableAlign::Left => "left".to_string(),
            TableAlign::Center => "center".to_string(),
            TableAlign::Right => "right".to_string(),
            TableAlign::Top => "top".to_string(),
            TableAlign::Horizon => "horizon".to_string(),
            TableAlign::Bottom => "bottom".to_string(),
        }
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<table class=\"typst-table\">\n");

        for row in &self.rows {
            if row.is_header {
                html.push_str("  <thead>\n    <tr>\n");
            } else if row.is_footer {
                html.push_str("  <tfoot>\n    <tr>\n");
            } else {
                html.push_str("  <tbody>\n    <tr>\n");
            }

            for cell in &row.cells {
                let tag = if row.is_header { "th" } else { "td" };
                let colspan = if cell.colspan > 1 {
                    format!(" colspan=\"{}\"", cell.colspan)
                } else {
                    String::new()
                };
                let rowspan = if cell.rowspan > 1 {
                    format!(" rowspan=\"{}\"", cell.rowspan)
                } else {
                    String::new()
                };
                let align = if let Some(align) = &cell.align {
                    format!(" style=\"text-align: {}\"", self.align_to_css(align))
                } else {
                    String::new()
                };

                html.push_str(&format!(
                    "      <{}{}{}{}>{}</{}>\n",
                    tag,
                    colspan,
                    rowspan,
                    align,
                    html_escape(&cell.content),
                    tag
                ));
            }

            if row.is_header {
                html.push_str("    </tr>\n  </thead>\n");
            } else if row.is_footer {
                html.push_str("    </tr>\n  </tfoot>\n");
            } else {
                html.push_str("    </tr>\n  </tbody>\n");
            }
        }

        html.push_str("</table>\n");

        html
    }

    fn align_to_css(&self, align: &TableAlign) -> String {
        match align {
            TableAlign::Left => "left".to_string(),
            TableAlign::Center => "center".to_string(),
            TableAlign::Right => "right".to_string(),
            TableAlign::Top => "top".to_string(),
            TableAlign::Horizon => "middle".to_string(),
            TableAlign::Bottom => "bottom".to_string(),
        }
    }

    /// 获取列数
    pub fn column_count(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.cells.len())
            .max()
            .unwrap_or(0)
    }

    /// 获取行数
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

/// 表格构建器
pub struct TableBuilder {
    table: Table,
}

impl TableBuilder {
    pub fn new() -> Self {
        Self {
            table: Table::new(),
        }
    }

    pub fn with_config(mut self, config: TableConfig) -> Self {
        self.table.config = config;
        self
    }

    pub fn with_columns(mut self, columns: Vec<TableSize>) -> Self {
        self.table.config.columns = columns;
        self
    }

    pub fn with_align(mut self, align: TableAlign) -> Self {
        self.table.config.align = Some(align);
        self
    }

    pub fn with_stroke(mut self, stroke: TableStroke) -> Self {
        self.table.config.stroke = Some(stroke);
        self
    }

    pub fn add_row(mut self, row: TableRow) -> Self {
        self.table.add_row(row);
        self
    }

    pub fn build(self) -> Table {
        self.table
    }
}

impl Default for TableBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let table = Table::new();
        assert!(table.rows.is_empty());
    }

    #[test]
    fn test_table_default() {
        let table = Table::default();
        assert!(table.rows.is_empty());
    }

    #[test]
    fn test_table_config_default() {
        let config = TableConfig::default();
        assert_eq!(config.columns.len(), 1);
        assert_eq!(config.inset, 0.5);
    }

    #[test]
    fn test_table_cell_creation() {
        let cell = TableCell::new("Test".to_string());
        assert_eq!(cell.content, "Test");
        assert_eq!(cell.colspan, 1);
        assert_eq!(cell.rowspan, 1);
    }

    #[test]
    fn test_table_cell_with_colspan() {
        let cell = TableCell::new("Test".to_string()).with_colspan(2);
        assert_eq!(cell.colspan, 2);
    }

    #[test]
    fn test_table_cell_with_rowspan() {
        let cell = TableCell::new("Test".to_string()).with_rowspan(2);
        assert_eq!(cell.rowspan, 2);
    }

    #[test]
    fn test_table_cell_with_align() {
        let cell = TableCell::new("Test".to_string()).with_align(TableAlign::Center);
        assert_eq!(cell.align, Some(TableAlign::Center));
    }

    #[test]
    fn test_table_row_creation() {
        let row = TableRow::new();
        assert!(row.cells.is_empty());
        assert!(!row.is_header);
        assert!(!row.is_footer);
    }

    #[test]
    fn test_table_row_header() {
        let row = TableRow::header();
        assert!(row.is_header);
        assert!(!row.is_footer);
    }

    #[test]
    fn test_table_row_footer() {
        let row = TableRow::footer();
        assert!(!row.is_header);
        assert!(row.is_footer);
    }

    #[test]
    fn test_table_row_add_cell() {
        let row = TableRow::new().add_cell(TableCell::new("Test".to_string()));
        assert_eq!(row.cells.len(), 1);
    }

    #[test]
    fn test_table_add_row() {
        let mut table = Table::new();
        table.add_row(TableRow::new());
        assert_eq!(table.rows.len(), 1);
    }

    #[test]
    fn test_table_column_count() {
        let mut table = Table::new();
        let row = TableRow::new()
            .add_cell(TableCell::new("A".to_string()))
            .add_cell(TableCell::new("B".to_string()))
            .add_cell(TableCell::new("C".to_string()));
        table.add_row(row);
        assert_eq!(table.column_count(), 3);
    }

    #[test]
    fn test_table_row_count() {
        let mut table = Table::new();
        table.add_row(TableRow::new());
        table.add_row(TableRow::new());
        assert_eq!(table.row_count(), 2);
    }

    #[test]
    fn test_to_typst() {
        let mut table = Table::new();
        let row = TableRow::new()
            .add_cell(TableCell::new("A".to_string()))
            .add_cell(TableCell::new("B".to_string()));
        table.add_row(row);
        let typst = table.to_typst();
        assert!(typst.contains("#table("));
        assert!(typst.contains("[A]"));
        assert!(typst.contains("[B]"));
    }

    #[test]
    fn test_to_html() {
        let mut table = Table::new();
        let row = TableRow::new()
            .add_cell(TableCell::new("A".to_string()))
            .add_cell(TableCell::new("B".to_string()));
        table.add_row(row);
        let html = table.to_html();
        assert!(html.contains("<table"));
        assert!(html.contains("<td>A</td>"));
        assert!(html.contains("<td>B</td>"));
    }

    #[test]
    fn test_to_html_header() {
        let mut table = Table::new();
        let row = TableRow::header().add_cell(TableCell::new("Header".to_string()));
        table.add_row(row);
        let html = table.to_html();
        assert!(html.contains("<thead>"));
        assert!(html.contains("<th>Header</th>"));
    }

    #[test]
    fn test_to_html_footer() {
        let mut table = Table::new();
        let row = TableRow::footer().add_cell(TableCell::new("Footer".to_string()));
        table.add_row(row);
        let html = table.to_html();
        assert!(html.contains("<tfoot>"));
        assert!(html.contains("<td>Footer</td>"));
    }

    #[test]
    fn test_table_align_partial_eq() {
        assert_eq!(TableAlign::Left, TableAlign::Left);
        assert_ne!(TableAlign::Left, TableAlign::Center);
    }

    #[test]
    fn test_table_stroke_default() {
        let stroke = TableStroke::default();
        assert_eq!(stroke.thickness, 0.5);
        assert_eq!(stroke.color, "black");
    }

    #[test]
    fn test_table_builder() {
        let table = TableBuilder::new()
            .with_align(TableAlign::Center)
            .add_row(TableRow::new().add_cell(TableCell::new("Test".to_string())))
            .build();

        assert_eq!(table.config.align, Some(TableAlign::Center));
        assert_eq!(table.rows.len(), 1);
    }

    #[test]
    fn test_table_builder_default() {
        let builder = TableBuilder::default();
        let table = builder.build();
        assert!(table.rows.is_empty());
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;"));
    }

    #[test]
    fn test_table_with_config() {
        let config = TableConfig {
            columns: vec![TableSize::Fixed(100.0), TableSize::Fixed(100.0)],
            rows: vec![TableSize::Auto],
            gutter: TableSize::Relative(0.5),
            column_gutter: None,
            row_gutter: None,
            inset: 1.0,
            align: Some(TableAlign::Center),
            fill: Some("white".to_string()),
            stroke: Some(TableStroke::default()),
        };
        let table = Table::with_config(config);
        assert_eq!(table.config.columns.len(), 2);
        assert_eq!(table.config.inset, 1.0);
    }

    #[test]
    fn test_table_cell_with_fill() {
        let cell = TableCell::new("Test".to_string()).with_fill("red".to_string());
        assert_eq!(cell.fill, Some("red".to_string()));
    }

    #[test]
    fn test_table_cell_with_stroke() {
        let stroke = TableStroke {
            thickness: 1.0,
            color: "blue".to_string(),
        };
        let cell = TableCell::new("Test".to_string()).with_stroke(stroke);
        assert!(cell.stroke.is_some());
    }

    #[test]
    fn test_empty_table() {
        let table = Table::new();
        assert_eq!(table.column_count(), 0);
        assert_eq!(table.row_count(), 0);
    }

    #[test]
    fn test_table_with_multiple_rows() {
        let mut table = Table::new();
        table.add_row(TableRow::new().add_cell(TableCell::new("A".to_string())));
        table.add_row(TableRow::new().add_cell(TableCell::new("B".to_string())));
        table.add_row(TableRow::new().add_cell(TableCell::new("C".to_string())));
        assert_eq!(table.row_count(), 3);
    }

    #[test]
    fn test_table_with_header_and_footer() {
        let mut table = Table::new();
        table.add_row(TableRow::header().add_cell(TableCell::new("Header".to_string())));
        table.add_row(TableRow::new().add_cell(TableCell::new("Data".to_string())));
        table.add_row(TableRow::footer().add_cell(TableCell::new("Footer".to_string())));
        assert_eq!(table.row_count(), 3);
    }
}
