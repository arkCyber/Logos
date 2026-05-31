/*!
 * 航空航天级路径操作系统
 * 实现 Typst 的高级路径操作功能
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PathCommand {
    MoveTo(f64, f64),
    LineTo(f64, f64),
    CubicBezierTo(f64, f64, f64, f64, f64, f64),
    QuadraticBezierTo(f64, f64, f64, f64),
    ArcTo(f64, f64, f64, bool, bool, f64, f64),
    ClosePath,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    commands: Vec<PathCommand>,
    closed: bool,
}

impl Path {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            closed: false,
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.commands.push(PathCommand::MoveTo(x, y));
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        self.commands.push(PathCommand::LineTo(x, y));
    }

    pub fn cubic_bezier_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.commands
            .push(PathCommand::CubicBezierTo(cp1x, cp1y, cp2x, cp2y, x, y));
    }

    pub fn quadratic_bezier_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.commands
            .push(PathCommand::QuadraticBezierTo(cpx, cpy, x, y));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn arc_to(
        &mut self,
        rx: f64,
        ry: f64,
        rotation: f64,
        large_arc: bool,
        sweep: bool,
        x: f64,
        y: f64,
    ) {
        self.commands
            .push(PathCommand::ArcTo(rx, ry, rotation, large_arc, sweep, x, y));
    }

    pub fn close(&mut self) {
        self.closed = true;
        self.commands.push(PathCommand::ClosePath);
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub fn commands(&self) -> &[PathCommand] {
        &self.commands
    }

    pub fn to_svg(&self) -> String {
        let mut d = String::new();

        for cmd in &self.commands {
            match cmd {
                PathCommand::MoveTo(x, y) => {
                    d.push_str(&format!("M {} {} ", x, y));
                }
                PathCommand::LineTo(x, y) => {
                    d.push_str(&format!("L {} {} ", x, y));
                }
                PathCommand::CubicBezierTo(cp1x, cp1y, cp2x, cp2y, x, y) => {
                    d.push_str(&format!(
                        "C {} {} {} {} {} {} ",
                        cp1x, cp1y, cp2x, cp2y, x, y
                    ));
                }
                PathCommand::QuadraticBezierTo(cpx, cpy, x, y) => {
                    d.push_str(&format!("Q {} {} {} {} ", cpx, cpy, x, y));
                }
                PathCommand::ArcTo(rx, ry, rotation, large_arc, sweep, x, y) => {
                    let large_arc_flag = if *large_arc { 1 } else { 0 };
                    let sweep_flag = if *sweep { 1 } else { 0 };
                    d.push_str(&format!(
                        "A {} {} {} {} {} {} {} ",
                        rx, ry, rotation, large_arc_flag, sweep_flag, x, y
                    ));
                }
                PathCommand::ClosePath => {
                    d.push_str("Z ");
                }
            }
        }

        format!("<path d=\"{}\" />", d.trim())
    }

    pub fn to_typst(&self) -> String {
        let mut typst = String::new();
        typst.push_str("path((");

        for cmd in &self.commands {
            match cmd {
                PathCommand::MoveTo(x, y) => {
                    typst.push_str(&format!("move({}, {}), ", x, y));
                }
                PathCommand::LineTo(x, y) => {
                    typst.push_str(&format!("line({}, {}), ", x, y));
                }
                PathCommand::CubicBezierTo(cp1x, cp1y, cp2x, cp2y, x, y) => {
                    typst.push_str(&format!(
                        "curve(({},{}, {},{}, {},{}), ",
                        cp1x, cp1y, cp2x, cp2y, x, y
                    ));
                }
                PathCommand::QuadraticBezierTo(cpx, cpy, x, y) => {
                    typst.push_str(&format!("quad(({},{}, {},{}), ", cpx, cpy, x, y));
                }
                PathCommand::ArcTo(rx, ry, rotation, large_arc, sweep, x, y) => {
                    typst.push_str(&format!(
                        "arc({},{}, {}, {}, {}, {}, {})",
                        rx, ry, rotation, large_arc, sweep, x, y
                    ));
                }
                PathCommand::ClosePath => {
                    typst.push_str("close, ");
                }
            }
        }

        typst.push_str("))");
        typst
    }

    pub fn to_canvas(&self) -> String {
        let mut canvas = String::new();
        canvas.push_str("ctx.beginPath();\n");

        for cmd in &self.commands {
            match cmd {
                PathCommand::MoveTo(x, y) => {
                    canvas.push_str(&format!("ctx.moveTo({}, {});\n", x, y));
                }
                PathCommand::LineTo(x, y) => {
                    canvas.push_str(&format!("ctx.lineTo({}, {});\n", x, y));
                }
                PathCommand::CubicBezierTo(cp1x, cp1y, cp2x, cp2y, x, y) => {
                    canvas.push_str(&format!(
                        "ctx.bezierCurveTo({}, {}, {}, {}, {}, {});\n",
                        cp1x, cp1y, cp2x, cp2y, x, y
                    ));
                }
                PathCommand::QuadraticBezierTo(cpx, cpy, x, y) => {
                    canvas.push_str(&format!(
                        "ctx.quadraticCurveTo({}, {}, {}, {});\n",
                        cpx, cpy, x, y
                    ));
                }
                PathCommand::ArcTo(rx, ry, rotation, large_arc, sweep, x, y) => {
                    canvas.push_str(&format!(
                        "ctx.arcTo({}, {}, {}, {}, {}, {}, {});\n",
                        rx, ry, rotation, large_arc, sweep, x, y
                    ));
                }
                PathCommand::ClosePath => {
                    canvas.push_str("ctx.closePath();\n");
                }
            }
        }

        canvas
    }

    /// 计算路径的边界框
    pub fn bounding_box(&self) -> Option<BoundingBox> {
        if self.commands.is_empty() {
            return None;
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for cmd in &self.commands {
            match cmd {
                PathCommand::MoveTo(x, y)
                | PathCommand::LineTo(x, y)
                | PathCommand::CubicBezierTo(_, _, _, _, x, y)
                | PathCommand::QuadraticBezierTo(_, _, x, y)
                | PathCommand::ArcTo(_, _, _, _, _, x, y) => {
                    min_x = min_x.min(*x);
                    min_y = min_y.min(*y);
                    max_x = max_x.max(*x);
                    max_y = max_y.max(*y);
                }
                PathCommand::ClosePath => {}
            }
        }

        Some(BoundingBox {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        })
    }

    /// 路径长度（近似）
    pub fn length(&self) -> f64 {
        let mut total = 0.0;
        let mut last_x = 0.0;
        let mut last_y = 0.0;

        for cmd in &self.commands {
            match cmd {
                PathCommand::MoveTo(x, y) => {
                    last_x = *x;
                    last_y = *y;
                }
                PathCommand::LineTo(x, y) => {
                    let dx = x - last_x;
                    let dy = y - last_y;
                    total += (dx * dx + dy * dy).sqrt();
                    last_x = *x;
                    last_y = *y;
                }
                PathCommand::CubicBezierTo(_cp1x, _cp1y, _cp2x, _cp2y, x, y) => {
                    // 简化：使用直线距离近似
                    let dx = x - last_x;
                    let dy = y - last_y;
                    total += (dx * dx + dy * dy).sqrt();
                    last_x = *x;
                    last_y = *y;
                }
                PathCommand::QuadraticBezierTo(_cpx, _cpy, x, y) => {
                    let dx = x - last_x;
                    let dy = y - last_y;
                    total += (dx * dx + dy * dy).sqrt();
                    last_x = *x;
                    last_y = *y;
                }
                PathCommand::ArcTo(_rx, _ry, _rotation, _large_arc, _sweep, x, y) => {
                    // 简化：使用直线距离近似
                    let dx = x - last_x;
                    let dy = y - last_y;
                    total += (dx * dx + dy * dy).sqrt();
                    last_x = *x;
                    last_y = *y;
                }
                PathCommand::ClosePath => {}
            }
        }

        total
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl BoundingBox {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains(&self, px: f64, py: f64) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathBuilder {
    path: Path,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self { path: Path::new() }
    }

    pub fn move_to(mut self, x: f64, y: f64) -> Self {
        self.path.move_to(x, y);
        self
    }

    pub fn line_to(mut self, x: f64, y: f64) -> Self {
        self.path.line_to(x, y);
        self
    }

    pub fn cubic_bezier_to(
        mut self,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    ) -> Self {
        self.path.cubic_bezier_to(cp1x, cp1y, cp2x, cp2y, x, y);
        self
    }

    pub fn quadratic_bezier_to(mut self, cpx: f64, cpy: f64, x: f64, y: f64) -> Self {
        self.path.quadratic_bezier_to(cpx, cpy, x, y);
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn arc_to(
        mut self,
        rx: f64,
        ry: f64,
        rotation: f64,
        large_arc: bool,
        sweep: bool,
        x: f64,
        y: f64,
    ) -> Self {
        self.path.arc_to(rx, ry, rotation, large_arc, sweep, x, y);
        self
    }

    pub fn close(mut self) -> Self {
        self.path.close();
        self
    }

    pub fn build(self) -> Path {
        self.path
    }
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 路径组合操作
pub struct PathOperations;

impl PathOperations {
    /// 合并两个路径
    pub fn union(path1: &Path, path2: &Path) -> Path {
        let mut result = Path::new();

        for cmd in path1.commands() {
            match cmd {
                PathCommand::MoveTo(x, y) => result.move_to(*x, *y),
                PathCommand::LineTo(x, y) => result.line_to(*x, *y),
                PathCommand::CubicBezierTo(cp1x, cp1y, cp2x, cp2y, x, y) => {
                    result.cubic_bezier_to(*cp1x, *cp1y, *cp2x, *cp2y, *x, *y);
                }
                PathCommand::QuadraticBezierTo(cpx, cpy, x, y) => {
                    result.quadratic_bezier_to(*cpx, *cpy, *x, *y);
                }
                PathCommand::ArcTo(rx, ry, rotation, large_arc, sweep, x, y) => {
                    result.arc_to(*rx, *ry, *rotation, *large_arc, *sweep, *x, *y);
                }
                PathCommand::ClosePath => result.close(),
            }
        }

        for cmd in path2.commands() {
            match cmd {
                PathCommand::MoveTo(x, y) => result.move_to(*x, *y),
                PathCommand::LineTo(x, y) => result.line_to(*x, *y),
                PathCommand::CubicBezierTo(cp1x, cp1y, cp2x, cp2y, x, y) => {
                    result.cubic_bezier_to(*cp1x, *cp1y, *cp2x, *cp2y, *x, *y);
                }
                PathCommand::QuadraticBezierTo(cpx, cpy, x, y) => {
                    result.quadratic_bezier_to(*cpx, *cpy, *x, *y);
                }
                PathCommand::ArcTo(rx, ry, rotation, large_arc, sweep, x, y) => {
                    result.arc_to(*rx, *ry, *rotation, *large_arc, *sweep, *x, *y);
                }
                PathCommand::ClosePath => result.close(),
            }
        }

        result
    }

    /// 路径偏移
    pub fn offset(path: &Path, dx: f64, dy: f64) -> Path {
        let mut result = Path::new();

        for cmd in path.commands() {
            match cmd {
                PathCommand::MoveTo(x, y) => result.move_to(x + dx, y + dy),
                PathCommand::LineTo(x, y) => result.line_to(x + dx, y + dy),
                PathCommand::CubicBezierTo(cp1x, cp1y, cp2x, cp2y, x, y) => {
                    result.cubic_bezier_to(
                        cp1x + dx,
                        cp1y + dy,
                        cp2x + dx,
                        cp2y + dy,
                        x + dx,
                        y + dy,
                    );
                }
                PathCommand::QuadraticBezierTo(cpx, cpy, x, y) => {
                    result.quadratic_bezier_to(cpx + dx, cpy + dy, x + dx, y + dy);
                }
                PathCommand::ArcTo(rx, ry, _rotation, _large_arc, _sweep, x, y) => {
                    result.arc_to(*rx, *ry, 0.0, false, true, x + dx, y + dy);
                }
                PathCommand::ClosePath => result.close(),
            }
        }

        result
    }

    /// 路径缩放
    pub fn scale(path: &Path, sx: f64, sy: f64) -> Path {
        let mut result = Path::new();

        for cmd in path.commands() {
            match cmd {
                PathCommand::MoveTo(x, y) => result.move_to(x * sx, y * sy),
                PathCommand::LineTo(x, y) => result.line_to(x * sx, y * sy),
                PathCommand::CubicBezierTo(cp1x, cp1y, cp2x, cp2y, x, y) => {
                    result.cubic_bezier_to(
                        cp1x * sx,
                        cp1y * sy,
                        cp2x * sx,
                        cp2y * sy,
                        x * sx,
                        y * sy,
                    );
                }
                PathCommand::QuadraticBezierTo(cpx, cpy, x, y) => {
                    result.quadratic_bezier_to(cpx * sx, cpy * sy, x * sx, y * sy);
                }
                PathCommand::ArcTo(rx, ry, _rotation, _large_arc, _sweep, x, y) => {
                    result.arc_to(rx * sx, ry * sy, 0.0, false, true, x * sx, y * sy);
                }
                PathCommand::ClosePath => result.close(),
            }
        }

        result
    }

    /// 路径旋转
    pub fn rotate(path: &Path, angle: f64, cx: f64, cy: f64) -> Path {
        let rad = angle * std::f64::consts::PI / 180.0;
        let cos = rad.cos();
        let sin = rad.sin();

        let mut result = Path::new();

        for cmd in path.commands() {
            match cmd {
                PathCommand::MoveTo(x, y) => {
                    let (rx, ry) = Self::rotate_point(*x, *y, cx, cy, cos, sin);
                    result.move_to(rx, ry);
                }
                PathCommand::LineTo(x, y) => {
                    let (rx, ry) = Self::rotate_point(*x, *y, cx, cy, cos, sin);
                    result.line_to(rx, ry);
                }
                PathCommand::CubicBezierTo(_cp1x, _cp1y, _cp2x, _cp2y, x, y) => {
                    let (rx, ry) = Self::rotate_point(*x, *y, cx, cy, cos, sin);
                    result.line_to(rx, ry);
                }
                PathCommand::QuadraticBezierTo(_cpx, _cpy, x, y) => {
                    let (rx, ry) = Self::rotate_point(*x, *y, cx, cy, cos, sin);
                    result.line_to(rx, ry);
                }
                PathCommand::ArcTo(_rx, _ry, _rotation, _large_arc, _sweep, x, y) => {
                    let (rx, ry) = Self::rotate_point(*x, *y, cx, cy, cos, sin);
                    result.line_to(rx, ry);
                }
                PathCommand::ClosePath => result.close(),
            }
        }

        result
    }

    fn rotate_point(x: f64, y: f64, cx: f64, cy: f64, cos: f64, sin: f64) -> (f64, f64) {
        let dx = x - cx;
        let dy = y - cy;
        let rx = dx * cos - dy * sin + cx;
        let ry = dx * sin + dy * cos + cy;
        (rx, ry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_creation() {
        let path = Path::new();
        assert!(path.commands().is_empty());
        assert!(!path.is_closed());
    }

    #[test]
    fn test_path_move_to() {
        let mut path = Path::new();
        path.move_to(10.0, 20.0);
        assert_eq!(path.commands().len(), 1);
    }

    #[test]
    fn test_path_line_to() {
        let mut path = Path::new();
        path.move_to(0.0, 0.0);
        path.line_to(10.0, 10.0);
        assert_eq!(path.commands().len(), 2);
    }

    #[test]
    fn test_path_close() {
        let mut path = Path::new();
        path.move_to(0.0, 0.0);
        path.line_to(10.0, 10.0);
        path.close();
        assert!(path.is_closed());
    }

    #[test]
    fn test_path_to_svg() {
        let mut path = Path::new();
        path.move_to(0.0, 0.0);
        path.line_to(10.0, 10.0);
        let svg = path.to_svg();
        assert!(svg.contains("<path"));
        assert!(svg.contains("M 0 0"));
        assert!(svg.contains("L 10 10"));
    }

    #[test]
    fn test_path_to_typst() {
        let mut path = Path::new();
        path.move_to(0.0, 0.0);
        path.line_to(10.0, 10.0);
        let typst = path.to_typst();
        assert!(typst.contains("path("));
        assert!(typst.contains("move"));
    }

    #[test]
    fn test_bounding_box() {
        let mut path = Path::new();
        path.move_to(0.0, 0.0);
        path.line_to(10.0, 20.0);
        let bbox = path.bounding_box();
        assert!(bbox.is_some());
        let bbox = bbox.unwrap();
        assert_eq!(bbox.x, 0.0);
        assert_eq!(bbox.y, 0.0);
        assert_eq!(bbox.width, 10.0);
        assert_eq!(bbox.height, 20.0);
    }

    #[test]
    fn test_path_length() {
        let mut path = Path::new();
        path.move_to(0.0, 0.0);
        path.line_to(3.0, 4.0);
        let length = path.length();
        assert!((length - 5.0).abs() < 0.01); // 3-4-5 triangle
    }

    #[test]
    fn test_path_builder() {
        let path = PathBuilder::new()
            .move_to(0.0, 0.0)
            .line_to(10.0, 10.0)
            .close()
            .build();
        assert!(path.is_closed());
        assert_eq!(path.commands().len(), 3);
    }

    #[test]
    fn test_path_union() {
        let mut path1 = Path::new();
        path1.move_to(0.0, 0.0);
        path1.line_to(10.0, 0.0);

        let mut path2 = Path::new();
        path2.move_to(0.0, 10.0);
        path2.line_to(10.0, 10.0);

        let result = PathOperations::union(&path1, &path2);
        assert_eq!(result.commands().len(), 4);
    }

    #[test]
    fn test_path_offset() {
        let mut path = Path::new();
        path.move_to(0.0, 0.0);
        path.line_to(10.0, 10.0);

        let result = PathOperations::offset(&path, 5.0, 5.0);
        assert_eq!(result.commands().len(), 2);
    }

    #[test]
    fn test_path_scale() {
        let mut path = Path::new();
        path.move_to(10.0, 10.0);
        path.line_to(20.0, 20.0);

        let result = PathOperations::scale(&path, 2.0, 2.0);
        assert_eq!(result.commands().len(), 2);
    }

    #[test]
    fn test_path_rotate() {
        let mut path = Path::new();
        path.move_to(10.0, 0.0);
        path.line_to(20.0, 0.0);

        let result = PathOperations::rotate(&path, 90.0, 0.0, 0.0);
        assert_eq!(result.commands().len(), 2);
    }

    #[test]
    fn test_bounding_box_contains() {
        let bbox = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        assert!(bbox.contains(5.0, 5.0));
        assert!(!bbox.contains(15.0, 15.0));
    }

    #[test]
    fn test_bounding_box_intersects() {
        let bbox1 = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        let bbox2 = BoundingBox::new(5.0, 5.0, 10.0, 10.0);
        assert!(bbox1.intersects(&bbox2));

        let bbox3 = BoundingBox::new(20.0, 20.0, 10.0, 10.0);
        assert!(!bbox1.intersects(&bbox3));
    }

    #[test]
    fn test_cubic_bezier() {
        let mut path = Path::new();
        path.move_to(0.0, 0.0);
        path.cubic_bezier_to(10.0, 10.0, 20.0, 20.0, 30.0, 30.0);
        assert_eq!(path.commands().len(), 2);
    }

    #[test]
    fn test_quadratic_bezier() {
        let mut path = Path::new();
        path.move_to(0.0, 0.0);
        path.quadratic_bezier_to(10.0, 10.0, 20.0, 20.0);
        assert_eq!(path.commands().len(), 2);
    }

    #[test]
    fn test_arc() {
        let mut path = Path::new();
        path.move_to(0.0, 0.0);
        path.arc_to(10.0, 10.0, 0.0, false, true, 20.0, 20.0);
        assert_eq!(path.commands().len(), 2);
    }
}
