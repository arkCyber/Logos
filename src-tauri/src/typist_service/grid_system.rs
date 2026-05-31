/*!
 * 航空航天级专业版面控制系统
 * 实现基线网格、文档网格、版面平衡、栏宽优化
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 网格类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GridType {
    /// 基线网格
    Baseline,
    /// 文档网格
    Document,
    /// 多级网格
    MultiLevel,
}

/// 网格配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
    pub grid_type: GridType,
    pub grid_size: f64, // in points
    pub grid_offset: f64,
    pub visible: bool,
    pub snap_to_grid: bool,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            grid_type: GridType::Baseline,
            grid_size: 12.0, // 12pt baseline grid
            grid_offset: 0.0,
            visible: false,
            snap_to_grid: true,
        }
    }
}

/// 版面平衡配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutBalanceConfig {
    pub avoid_widows: bool, // 避免孤行
    pub avoid_orphans: bool, // 避免寡行
    pub balance_columns: bool, // 栏平衡
    pub minimum_lines_at_bottom: usize,
    pub minimum_lines_at_top: usize,
}

impl Default for LayoutBalanceConfig {
    fn default() -> Self {
        Self {
            avoid_widows: true,
            avoid_orphans: true,
            balance_columns: true,
            minimum_lines_at_bottom: 2,
            minimum_lines_at_top: 2,
        }
    }
}

/// 栏宽配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnWidthConfig {
    pub column_count: usize,
    pub gutter_width: f64, // in points
    pub column_width: Option<f64>, // None = auto
    pub min_column_width: f64,
    pub max_column_width: f64,
    pub equal_width: bool,
}

impl Default for ColumnWidthConfig {
    fn default() -> Self {
        Self {
            column_count: 1,
            gutter_width: 12.0,
            column_width: None,
            min_column_width: 36.0, // minimum readable width
            max_column_width: 432.0, // 6 inches
            equal_width: true,
        }
    }
}

/// 页面边距配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMarginConfig {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
    pub inside: Option<f64>, // for facing pages
    pub outside: Option<f64>, // for facing pages
}

impl Default for PageMarginConfig {
    fn default() -> Self {
        Self {
            top: 72.0, // 1 inch
            bottom: 72.0,
            left: 72.0,
            right: 72.0,
            inside: None,
            outside: None,
        }
    }
}

/// 版面配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSystemConfig {
    pub grid: GridConfig,
    pub balance: LayoutBalanceConfig,
    pub columns: ColumnWidthConfig,
    pub margins: PageMarginConfig,
}

impl Default for LayoutSystemConfig {
    fn default() -> Self {
        Self {
            grid: GridConfig::default(),
            balance: LayoutBalanceConfig::default(),
            columns: ColumnWidthConfig::default(),
            margins: PageMarginConfig::default(),
        }
    }
}

/// 版面平衡引擎
pub struct LayoutBalanceEngine {
    config: LayoutBalanceConfig,
}

impl LayoutBalanceEngine {
    pub fn new(config: LayoutBalanceConfig) -> Self {
        Self { config }
    }

    /// 检测孤行（页面最后一行单独一行）
    pub fn detect_widow(&self, _line_count: usize, lines_on_last_page: usize) -> bool {
        if !self.config.avoid_widows {
            return false;
        }
        lines_on_last_page < self.config.minimum_lines_at_bottom
    }

    /// 检测寡行（页面第一行单独一行）
    pub fn detect_orphan(&self, lines_on_first_page: usize) -> bool {
        if !self.config.avoid_orphans {
            return false;
        }
        lines_on_first_page < self.config.minimum_lines_at_top
    }

    /// 计算栏平衡
    pub fn calculate_column_balance(
        &self,
        total_lines: usize,
        column_count: usize,
    ) -> Vec<usize> {
        if !self.config.balance_columns {
            return vec![total_lines; column_count];
        }

        let base_lines = total_lines / column_count;
        let remainder = total_lines % column_count;
        
        (0..column_count)
            .map(|i| base_lines + if i < remainder { 1 } else { 0 })
            .collect()
    }
}

/// 栏宽优化引擎
pub struct ColumnWidthOptimizer {
    config: ColumnWidthConfig,
}

impl ColumnWidthOptimizer {
    pub fn new(config: ColumnWidthConfig) -> Self {
        Self { config }
    }

    /// 计算最优栏宽
    pub fn calculate_optimal_widths(&self, page_width: f64) -> Vec<f64> {
        let available_width = page_width - self.config.gutter_width * (self.config.column_count - 1) as f64;
        
        if let Some(fixed_width) = self.config.column_width {
            if self.config.equal_width {
                vec![fixed_width; self.config.column_count]
            } else {
                // 第一栏固定，其余平均分配
                let remaining_width = available_width - fixed_width;
                let remaining_columns = self.config.column_count - 1;
                let other_width = if remaining_columns > 0 {
                    remaining_width / remaining_columns as f64
                } else {
                    0.0
                };
                let mut widths = vec![fixed_width];
                widths.extend(vec![other_width; remaining_columns]);
                widths
            }
        } else {
            // 自动计算
            if self.config.equal_width {
                let width = available_width / self.config.column_count as f64;
                vec![width; self.config.column_count]
            } else {
                // 黄金比例分配
                let golden_ratio = 1.618;
                let total_ratio = (1.0 + golden_ratio) * (self.config.column_count - 1) as f64 + 1.0;
                let unit = available_width / total_ratio;
                
                let mut widths = Vec::new();
                widths.push(unit);
                for _ in 1..self.config.column_count {
                    widths.push(unit * golden_ratio);
                }
                widths
            }
        }
    }

    /// 验证栏宽是否在可接受范围内
    pub fn validate_width(&self, width: f64) -> bool {
        width >= self.config.min_column_width && width <= self.config.max_column_width
    }
}

/// 网格系统
pub struct GridSystem {
    config: GridConfig,
}

impl GridSystem {
    pub fn new(config: GridConfig) -> Self {
        Self { config }
    }

    /// 对齐到网格
    pub fn snap_to_grid(&self, position: f64) -> f64 {
        if !self.config.snap_to_grid {
            return position;
        }
        
        let grid_size = self.config.grid_size;
        let offset = self.config.grid_offset;
        
        let adjusted = position - offset;
        let snapped = (adjusted / grid_size).round() * grid_size + offset;
        
        snapped
    }

    /// 获取最近的网格线位置
    pub fn get_nearest_grid_line(&self, position: f64) -> f64 {
        let grid_size = self.config.grid_size;
        let offset = self.config.grid_offset;
        
        let adjusted = position - offset;
        let grid_line = (adjusted / grid_size).floor() * grid_size + offset;
        
        grid_line
    }

    /// 获取网格线数量
    pub fn get_grid_line_count(&self, height: f64) -> usize {
        ((height - self.config.grid_offset) / self.config.grid_size).ceil() as usize
    }

    /// 生成网格 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();
        
        match self.config.grid_type {
            GridType::Baseline => {
                typst.push_str("#set grid(");
                typst.push_str(&format!("size: {}pt, ", self.config.grid_size));
                typst.push_str(&format!("offset: {}pt, ", self.config.grid_offset));
                if self.config.visible {
                    typst.push_str("stroke: 0.5pt + gray, ");
                }
                typst.push_str(")\n");
            }
            GridType::Document => {
                typst.push_str("#set page(");
                typst.push_str(&format!("grid: {}pt, ", self.config.grid_size));
                if self.config.visible {
                    typst.push_str("grid-stroke: 0.5pt + gray, ");
                }
                typst.push_str(")\n");
            }
            GridType::MultiLevel => {
                typst.push_str("#set grid(");
                typst.push_str(&format!("size: {}pt, ", self.config.grid_size));
                typst.push_str(&format!("offset: {}pt, ", self.config.grid_offset));
                if self.config.visible {
                    typst.push_str("stroke: 0.5pt + gray, ");
                }
                typst.push_str(")\n");
            }
        }
        
        typst
    }
}

/// 专业版面控制系统
pub struct LayoutSystem {
    config: LayoutSystemConfig,
    balance_engine: LayoutBalanceEngine,
    column_optimizer: ColumnWidthOptimizer,
    grid_system: GridSystem,
}

impl LayoutSystem {
    pub fn new(config: LayoutSystemConfig) -> Self {
        Self {
            balance_engine: LayoutBalanceEngine::new(config.balance.clone()),
            column_optimizer: ColumnWidthOptimizer::new(config.columns.clone()),
            grid_system: GridSystem::new(config.grid.clone()),
            config,
        }
    }

    /// 对齐位置到网格
    pub fn snap_position(&self, position: f64) -> f64 {
        self.grid_system.snap_to_grid(position)
    }

    /// 计算最优栏宽
    pub fn calculate_column_widths(&self, page_width: f64) -> Vec<f64> {
        self.column_optimizer.calculate_optimal_widths(page_width)
    }

    /// 计算栏平衡
    pub fn calculate_balance(&self, total_lines: usize) -> Vec<usize> {
        self.balance_engine.calculate_column_balance(total_lines, self.config.columns.column_count)
    }

    /// 检测版面问题
    pub fn detect_layout_issues(&self, line_count: usize, lines_on_last_page: usize, lines_on_first_page: usize) -> LayoutIssues {
        LayoutIssues {
            has_widow: self.balance_engine.detect_widow(line_count, lines_on_last_page),
            has_orphan: self.balance_engine.detect_orphan(lines_on_first_page),
            column_width_valid: self
                .column_optimizer
                .validate_width(self.config.columns.column_width.unwrap_or(100.0)),
        }
    }

    /// 生成完整的版面配置 Typst 代码
    pub fn to_typst(&self) -> String {
        let mut typst = String::new();
        
        // 页面边距
        typst.push_str("#set page(");
        typst.push_str(&format!("margin: (left: {}pt, right: {}pt, top: {}pt, bottom: {}pt), ",
            self.config.margins.left,
            self.config.margins.right,
            self.config.margins.top,
            self.config.margins.bottom
        ));
        typst.push_str(")\n");
        
        // 栏配置
        if self.config.columns.column_count > 1 {
            typst.push_str("#set columns(");
            typst.push_str(&format!("{}, ", self.config.columns.column_count));
            typst.push_str(&format!("gutter: {}pt, ", self.config.columns.gutter_width));
            typst.push_str(")\n");
        }
        
        // 网格配置
        typst.push_str(&self.grid_system.to_typst());
        
        typst
    }
}

impl Default for LayoutSystem {
    fn default() -> Self {
        Self::new(LayoutSystemConfig::default())
    }
}

/// 版面问题检测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutIssues {
    pub has_widow: bool,
    pub has_orphan: bool,
    pub column_width_valid: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_config_default() {
        let config = GridConfig::default();
        assert_eq!(config.grid_size, 12.0);
        assert!(!config.visible);
    }

    #[test]
    fn test_layout_balance_config_default() {
        let config = LayoutBalanceConfig::default();
        assert!(config.avoid_widows);
        assert!(config.avoid_orphans);
    }

    #[test]
    fn test_column_width_config_default() {
        let config = ColumnWidthConfig::default();
        assert_eq!(config.column_count, 1);
        assert_eq!(config.gutter_width, 12.0);
    }

    #[test]
    fn test_layout_balance_engine_detect_widow() {
        let config = LayoutBalanceConfig::default();
        let engine = LayoutBalanceEngine::new(config);
        
        assert!(engine.detect_widow(10, 1));
        assert!(!engine.detect_widow(10, 3));
    }

    #[test]
    fn test_layout_balance_engine_detect_orphan() {
        let config = LayoutBalanceConfig::default();
        let engine = LayoutBalanceEngine::new(config);
        
        assert!(engine.detect_orphan(1));
        assert!(!engine.detect_orphan(3));
    }

    #[test]
    fn test_layout_balance_engine_calculate_column_balance() {
        let config = LayoutBalanceConfig::default();
        let engine = LayoutBalanceEngine::new(config);
        
        let balance = engine.calculate_column_balance(10, 3);
        assert_eq!(balance.len(), 3);
        assert_eq!(balance.iter().sum::<usize>(), 10);
    }

    #[test]
    fn test_column_width_optimizer_calculate_optimal_widths() {
        let config = ColumnWidthConfig {
            column_count: 2,
            gutter_width: 12.0,
            column_width: None,
            min_column_width: 36.0,
            max_column_width: 432.0,
            equal_width: true,
        };
        let optimizer = ColumnWidthOptimizer::new(config);
        
        let widths = optimizer.calculate_optimal_widths(432.0);
        assert_eq!(widths.len(), 2);
        assert_eq!(widths[0], widths[1]);
    }

    #[test]
    fn test_column_width_optimizer_validate_width() {
        let config = ColumnWidthConfig::default();
        let optimizer = ColumnWidthOptimizer::new(config);
        
        assert!(optimizer.validate_width(100.0));
        assert!(!optimizer.validate_width(10.0));
        assert!(!optimizer.validate_width(500.0));
    }

    #[test]
    fn test_grid_system_snap_to_grid() {
        let config = GridConfig {
            grid_type: GridType::Baseline,
            grid_size: 12.0,
            grid_offset: 0.0,
            visible: false,
            snap_to_grid: true,
        };
        let grid = GridSystem::new(config);
        
        let snapped = grid.snap_to_grid(15.0);
        assert_eq!(snapped, 12.0);
    }

    #[test]
    fn test_grid_system_get_nearest_grid_line() {
        let config = GridConfig::default();
        let grid = GridSystem::new(config);
        
        let line = grid.get_nearest_grid_line(15.0);
        assert_eq!(line, 12.0);
    }

    #[test]
    fn test_grid_system_get_grid_line_count() {
        let config = GridConfig::default();
        let grid = GridSystem::new(config);
        
        let count = grid.get_grid_line_count(72.0);
        assert_eq!(count, 6);
    }

    #[test]
    fn test_layout_system_creation() {
        let system = LayoutSystem::default();
        assert_eq!(system.config.columns.column_count, 1);
    }

    #[test]
    fn test_layout_system_detect_layout_issues() {
        let system = LayoutSystem::default();
        let issues = system.detect_layout_issues(10, 1, 1);
        
        assert!(issues.has_widow);
        assert!(issues.has_orphan);
    }

    #[test]
    fn test_layout_system_to_typst() {
        let system = LayoutSystem::default();
        let typst = system.to_typst();
        
        assert!(typst.contains("#set page("));
        assert!(typst.contains("margin:"));
    }
}
