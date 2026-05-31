use serde::{Deserialize, Serialize};

/// 表格单元格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    /// 单元格文本
    pub text: String,
    /// 是否加粗
    pub bold: bool,
    /// 是否斜体
    pub italic: bool,
    /// 背景颜色（RGB）
    pub background_color: Option<(u8, u8, u8)>,
    /// 文本颜色（RGB）
    pub text_color: (u8, u8, u8),
    /// 水平对齐
    pub horizontal_align: String,
    /// 垂直对齐
    pub vertical_align: String,
    /// 列跨度
    pub colspan: usize,
    /// 行跨度
    pub rowspan: usize,
}

impl TableCell {
    /// 创建新的单元格
    pub fn new(text: String) -> Self {
        Self {
            text,
            bold: false,
            italic: false,
            background_color: None,
            text_color: (0, 0, 0),
            horizontal_align: "left".to_string(),
            vertical_align: "middle".to_string(),
            colspan: 1,
            rowspan: 1,
        }
    }

    /// 设置加粗
    pub fn with_bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// 设置背景颜色
    pub fn with_background_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.background_color = Some((r, g, b));
        self
    }

    /// 设置文本颜色
    pub fn with_text_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.text_color = (r, g, b);
        self
    }

    /// 设置列跨度
    pub fn with_colspan(mut self, colspan: usize) -> Self {
        self.colspan = colspan;
        self
    }

    /// 设置行跨度
    pub fn with_rowspan(mut self, rowspan: usize) -> Self {
        self.rowspan = rowspan;
        self
    }
}

/// 表格行
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    /// 行 ID
    pub id: String,
    /// 单元格
    pub cells: Vec<TableCell>,
    /// 是否为标题行
    pub is_header: bool,
    /// 行高度（点）
    pub height: f64,
}

impl TableRow {
    /// 创建新的行
    pub fn new(id: String) -> Self {
        Self {
            id,
            cells: Vec::new(),
            is_header: false,
            height: 24.0,
        }
    }

    /// 添加单元格
    pub fn with_cell(mut self, cell: TableCell) -> Self {
        self.cells.push(cell);
        self
    }

    /// 设置为标题行
    pub fn as_header(mut self) -> Self {
        self.is_header = true;
        self
    }

    /// 设置行高度
    pub fn with_height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }
}

/// 表格样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableStyle {
    /// 边框颜色（RGB）
    pub border_color: (u8, u8, u8),
    /// 边框宽度（点）
    pub border_width: f64,
    /// 是否显示边框
    pub show_border: bool,
    /// 表头背景颜色（RGB）
    pub header_background: (u8, u8, u8),
    /// 表头文本颜色（RGB）
    pub header_text_color: (u8, u8, u8),
    /// 斑马纹颜色（RGB）
    pub stripe_color: Option<(u8, u8, u8)>,
    /// 单元格内边距（点）
    pub cell_padding: f64,
}

impl TableStyle {
    /// 创建默认样式
    pub fn new() -> Self {
        Self {
            border_color: (0, 0, 0),
            border_width: 1.0,
            show_border: true,
            header_background: (200, 200, 200),
            header_text_color: (0, 0, 0),
            stripe_color: None,
            cell_padding: 4.0,
        }
    }

    /// 设置边框颜色
    pub fn with_border_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.border_color = (r, g, b);
        self
    }

    /// 设置是否显示边框
    pub fn with_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// 设置表头背景
    pub fn with_header_background(mut self, r: u8, g: u8, b: u8) -> Self {
        self.header_background = (r, g, b);
        self
    }

    /// 设置斑马纹
    pub fn with_stripe(mut self, r: u8, g: u8, b: u8) -> Self {
        self.stripe_color = Some((r, g, b));
        self
    }
}

impl Default for TableStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// 表格元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableElement {
    /// 表格 ID
    pub id: String,
    /// 行
    pub rows: Vec<TableRow>,
    /// 表格样式
    pub style: TableStyle,
    /// 位置（X, Y 坐标，单位：点）
    pub position: (f64, f64),
    /// 表格宽度（点）
    pub width: f64,
}

impl TableElement {
    /// 创建新的表格
    pub fn new(id: String) -> Self {
        Self {
            id,
            rows: Vec::new(),
            style: TableStyle::new(),
            position: (0.0, 0.0),
            width: 400.0,
        }
    }

    /// 添加行
    pub fn with_row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    /// 设置样式
    pub fn with_style(mut self, style: TableStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置位置
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = (x, y);
        self
    }

    /// 设置宽度
    pub fn with_width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// 获取行数
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// 获取列数（基于第一行）
    pub fn column_count(&self) -> usize {
        self.rows.first().map(|r| r.cells.len()).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_cell_new() {
        let cell = TableCell::new("Test".to_string());
        assert_eq!(cell.text, "Test");
        assert_eq!(cell.colspan, 1);
    }

    #[test]
    fn test_table_cell_with_bold() {
        let cell = TableCell::new("Test".to_string()).with_bold(true);
        assert!(cell.bold);
    }

    #[test]
    fn test_table_cell_with_colspan() {
        let cell = TableCell::new("Test".to_string()).with_colspan(2);
        assert_eq!(cell.colspan, 2);
    }

    #[test]
    fn test_table_row_new() {
        let row = TableRow::new("row1".to_string());
        assert_eq!(row.id, "row1");
        assert!(row.cells.is_empty());
    }

    #[test]
    fn test_table_row_with_cell() {
        let cell = TableCell::new("A".to_string());
        let row = TableRow::new("row1".to_string()).with_cell(cell);
        assert_eq!(row.cells.len(), 1);
    }

    #[test]
    fn test_table_row_as_header() {
        let row = TableRow::new("row1".to_string()).as_header();
        assert!(row.is_header);
    }

    #[test]
    fn test_table_style_new() {
        let style = TableStyle::new();
        assert!(style.show_border);
        assert_eq!(style.border_width, 1.0);
    }

    #[test]
    fn test_table_style_with_border_color() {
        let style = TableStyle::new().with_border_color(255, 0, 0);
        assert_eq!(style.border_color, (255, 0, 0));
    }

    #[test]
    fn test_table_element_new() {
        let table = TableElement::new("table1".to_string());
        assert_eq!(table.id, "table1");
        assert!(table.rows.is_empty());
    }

    #[test]
    fn test_table_element_with_row() {
        let row = TableRow::new("row1".to_string());
        let table = TableElement::new("table1".to_string()).with_row(row);
        assert_eq!(table.rows.len(), 1);
    }

    #[test]
    fn test_table_element_row_count() {
        let row1 = TableRow::new("row1".to_string());
        let row2 = TableRow::new("row2".to_string());
        let table = TableElement::new("table1".to_string())
            .with_row(row1)
            .with_row(row2);
        assert_eq!(table.row_count(), 2);
    }

    #[test]
    fn test_table_element_column_count() {
        let cell1 = TableCell::new("A".to_string());
        let cell2 = TableCell::new("B".to_string());
        let row = TableRow::new("row1".to_string())
            .with_cell(cell1)
            .with_cell(cell2);
        let table = TableElement::new("table1".to_string()).with_row(row);
        assert_eq!(table.column_count(), 2);
    }

    #[test]
    fn test_table_element_chaining() {
        let cell = TableCell::new("A".to_string());
        let row = TableRow::new("row1".to_string()).with_cell(cell);
        let table = TableElement::new("table1".to_string())
            .with_row(row)
            .with_position(100.0, 200.0)
            .with_width(500.0);
        assert_eq!(table.position, (100.0, 200.0));
        assert_eq!(table.width, 500.0);
    }

    #[test]
    fn test_table_element_serialization() {
        let table = TableElement::new("table1".to_string());
        let json = serde_json::to_string(&table);
        assert!(json.is_ok());
    }
}
