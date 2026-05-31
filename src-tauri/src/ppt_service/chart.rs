use serde::{Deserialize, Serialize};

/// 图表类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChartType {
    /// 柱状图
    Column,
    /// 条形图
    Bar,
    /// 折线图
    Line,
    /// 饼图
    Pie,
    /// 散点图
    Scatter,
    /// 面积图
    Area,
    /// 雷达图
    Radar,
    /// 混合图
    Combo,
}

/// 图表数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartDataPoint {
    /// 类别名称
    pub category: String,
    /// 数值
    pub value: f64,
    /// 颜色（RGB）
    pub color: Option<(u8, u8, u8)>,
}

impl ChartDataPoint {
    /// 创建新的数据点
    pub fn new(category: String, value: f64) -> Self {
        Self {
            category,
            value,
            color: None,
        }
    }

    /// 设置颜色
    pub fn with_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = Some((r, g, b));
        self
    }
}

/// 图表数据系列
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartSeries {
    /// 系列名称
    pub name: String,
    /// 数据点
    pub data: Vec<ChartDataPoint>,
    /// 系列颜色（RGB）
    pub color: (u8, u8, u8),
}

impl ChartSeries {
    /// 创建新的数据系列
    pub fn new(name: String, color: (u8, u8, u8)) -> Self {
        Self {
            name,
            data: Vec::new(),
            color,
        }
    }

    /// 添加数据点
    pub fn with_point(mut self, point: ChartDataPoint) -> Self {
        self.data.push(point);
        self
    }

    /// 添加多个数据点
    pub fn with_points(mut self, points: Vec<ChartDataPoint>) -> Self {
        self.data = points;
        self
    }
}

/// 图表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    /// 数据系列
    pub series: Vec<ChartSeries>,
    /// 图表标题
    pub title: String,
    /// X 轴标签
    pub x_axis_label: String,
    /// Y 轴标签
    pub y_axis_label: String,
    /// 是否显示图例
    pub show_legend: bool,
}

impl ChartData {
    /// 创建新的图表数据
    pub fn new(title: String) -> Self {
        Self {
            series: Vec::new(),
            title,
            x_axis_label: String::new(),
            y_axis_label: String::new(),
            show_legend: true,
        }
    }

    /// 添加数据系列
    pub fn with_series(mut self, series: ChartSeries) -> Self {
        self.series.push(series);
        self
    }

    /// 设置 X 轴标签
    pub fn with_x_axis_label(mut self, label: String) -> Self {
        self.x_axis_label = label;
        self
    }

    /// 设置 Y 轴标签
    pub fn with_y_axis_label(mut self, label: String) -> Self {
        self.y_axis_label = label;
        self
    }

    /// 设置是否显示图例
    pub fn with_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }
}

/// 图表样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartStyle {
    /// 背景颜色（RGB）
    pub background_color: (u8, u8, u8),
    /// 边框颜色（RGB）
    pub border_color: (u8, u8, u8),
    /// 边框宽度（点）
    pub border_width: f64,
    /// 字体大小（点）
    pub font_size: f64,
    /// 是否显示网格线
    pub show_grid_lines: bool,
    /// 网格线颜色（RGB）
    pub grid_line_color: (u8, u8, u8),
    /// 是否显示数据标签
    pub show_data_labels: bool,
}

impl ChartStyle {
    /// 创建默认样式
    pub fn new() -> Self {
        Self {
            background_color: (255, 255, 255),
            border_color: (0, 0, 0),
            border_width: 1.0,
            font_size: 12.0,
            show_grid_lines: true,
            grid_line_color: (200, 200, 200),
            show_data_labels: false,
        }
    }

    /// 设置背景颜色
    pub fn with_background_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.background_color = (r, g, b);
        self
    }

    /// 设置字体大小
    pub fn with_font_size(mut self, size: f64) -> Self {
        self.font_size = size;
        self
    }

    /// 设置是否显示网格线
    pub fn with_grid_lines(mut self, show: bool) -> Self {
        self.show_grid_lines = show;
        self
    }

    /// 设置是否显示数据标签
    pub fn with_data_labels(mut self, show: bool) -> Self {
        self.show_data_labels = show;
        self
    }
}

impl Default for ChartStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// 图表元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartElement {
    /// 图表 ID
    pub id: String,
    /// 图表类型
    pub chart_type: ChartType,
    /// 图表数据
    pub data: ChartData,
    /// 图表样式
    pub style: ChartStyle,
    /// 位置（X, Y 坐标，单位：点）
    pub position: (f64, f64),
    /// 图表宽度（点）
    pub width: f64,
    /// 图表高度（点）
    pub height: f64,
}

impl ChartElement {
    /// 创建新的图表
    pub fn new(id: String, chart_type: ChartType, data: ChartData) -> Self {
        Self {
            id,
            chart_type,
            data,
            style: ChartStyle::new(),
            position: (0.0, 0.0),
            width: 400.0,
            height: 300.0,
        }
    }

    /// 设置样式
    pub fn with_style(mut self, style: ChartStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置位置
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = (x, y);
        self
    }

    /// 设置尺寸
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// 创建柱状图
    pub fn column_chart(id: String, data: ChartData) -> Self {
        Self::new(id, ChartType::Column, data)
    }

    /// 创建饼图
    pub fn pie_chart(id: String, data: ChartData) -> Self {
        Self::new(id, ChartType::Pie, data)
    }

    /// 创建折线图
    pub fn line_chart(id: String, data: ChartData) -> Self {
        Self::new(id, ChartType::Line, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_data_point_new() {
        let point = ChartDataPoint::new("A".to_string(), 100.0);
        assert_eq!(point.category, "A");
        assert_eq!(point.value, 100.0);
    }

    #[test]
    fn test_chart_data_point_with_color() {
        let point = ChartDataPoint::new("A".to_string(), 100.0).with_color(255, 0, 0);
        assert_eq!(point.color, Some((255, 0, 0)));
    }

    #[test]
    fn test_chart_series_new() {
        let series = ChartSeries::new("Series 1".to_string(), (255, 0, 0));
        assert_eq!(series.name, "Series 1");
        assert_eq!(series.color, (255, 0, 0));
    }

    #[test]
    fn test_chart_series_with_point() {
        let point = ChartDataPoint::new("A".to_string(), 100.0);
        let series = ChartSeries::new("Series 1".to_string(), (255, 0, 0)).with_point(point);
        assert_eq!(series.data.len(), 1);
    }

    #[test]
    fn test_chart_data_new() {
        let data = ChartData::new("Sales".to_string());
        assert_eq!(data.title, "Sales");
        assert!(data.show_legend);
    }

    #[test]
    fn test_chart_data_with_series() {
        let series = ChartSeries::new("Series 1".to_string(), (255, 0, 0));
        let data = ChartData::new("Sales".to_string()).with_series(series);
        assert_eq!(data.series.len(), 1);
    }

    #[test]
    fn test_chart_style_new() {
        let style = ChartStyle::new();
        assert!(style.show_grid_lines);
        assert_eq!(style.font_size, 12.0);
    }

    #[test]
    fn test_chart_style_with_font_size() {
        let style = ChartStyle::new().with_font_size(14.0);
        assert_eq!(style.font_size, 14.0);
    }

    #[test]
    fn test_chart_element_new() {
        let data = ChartData::new("Test".to_string());
        let chart = ChartElement::new("1".to_string(), ChartType::Column, data);
        assert_eq!(chart.id, "1");
        assert_eq!(chart.chart_type, ChartType::Column);
    }

    #[test]
    fn test_chart_element_column_chart() {
        let data = ChartData::new("Sales".to_string());
        let chart = ChartElement::column_chart("1".to_string(), data);
        assert_eq!(chart.chart_type, ChartType::Column);
    }

    #[test]
    fn test_chart_element_pie_chart() {
        let data = ChartData::new("Distribution".to_string());
        let chart = ChartElement::pie_chart("1".to_string(), data);
        assert_eq!(chart.chart_type, ChartType::Pie);
    }

    #[test]
    fn test_chart_element_line_chart() {
        let data = ChartData::new("Trend".to_string());
        let chart = ChartElement::line_chart("1".to_string(), data);
        assert_eq!(chart.chart_type, ChartType::Line);
    }

    #[test]
    fn test_chart_element_chaining() {
        let data = ChartData::new("Test".to_string());
        let chart = ChartElement::new("1".to_string(), ChartType::Column, data)
            .with_position(100.0, 200.0)
            .with_size(500.0, 400.0);
        assert_eq!(chart.position, (100.0, 200.0));
        assert_eq!(chart.width, 500.0);
    }

    #[test]
    fn test_chart_element_serialization() {
        let data = ChartData::new("Test".to_string());
        let chart = ChartElement::new("1".to_string(), ChartType::Column, data);
        let json = serde_json::to_string(&chart);
        assert!(json.is_ok());
    }
}
