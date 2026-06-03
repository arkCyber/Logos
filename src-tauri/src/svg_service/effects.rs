/*!
 * Aerospace-Grade SVG Effects Model
 *
 * Defines gradients, filters, and SMIL animations rendered inside SVG `<defs>`.
 */

use serde::{Deserialize, Serialize};
use super::sanitize::escape_svg_attribute;

/// Maximum number of gradient stops per gradient
pub const MAX_GRADIENT_STOPS: usize = 32;

/// Maximum SMIL animation count per graphic
pub const MAX_ANIMATIONS: usize = 256;

/// Gradient color stop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgGradientStop {
    pub offset: f64,
    pub color: (u8, u8, u8),
    pub opacity: f64,
}

impl SvgGradientStop {
    /// Create a validated gradient stop
    pub fn new(offset: f64, color: (u8, u8, u8), opacity: f64) -> Result<Self, String> {
        if !offset.is_finite() || !(0.0..=1.0).contains(&offset) {
            return Err("Gradient stop offset must be between 0.0 and 1.0".to_string());
        }
        Ok(Self {
            offset,
            color,
            opacity: opacity.clamp(0.0, 1.0),
        })
    }
}

/// Linear gradient definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgLinearGradient {
    pub id: String,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub stops: Vec<SvgGradientStop>,
}

/// Radial gradient definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgRadialGradient {
    pub id: String,
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
    pub stops: Vec<SvgGradientStop>,
}

/// Basic SVG filter (Gaussian blur)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgFilter {
    pub id: String,
    pub blur_std_deviation: f64,
}

/// SMIL animation definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgAnimation {
    pub target_id: String,
    pub attribute_name: String,
    pub from_value: String,
    pub to_value: String,
    pub duration_secs: f64,
    pub repeat_count: String,
}

/// Collection of reusable SVG definitions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SvgDefs {
    pub linear_gradients: Vec<SvgLinearGradient>,
    pub radial_gradients: Vec<SvgRadialGradient>,
    pub filters: Vec<SvgFilter>,
    pub animations: Vec<SvgAnimation>,
}

impl SvgDefs {
    /// Returns true when no definitions are present
    pub fn is_empty(&self) -> bool {
        self.linear_gradients.is_empty()
            && self.radial_gradients.is_empty()
            && self.filters.is_empty()
            && self.animations.is_empty()
    }

    /// Validate all definition payloads
    pub fn validate(&self) -> Result<(), String> {
        if self.animations.len() > MAX_ANIMATIONS {
            return Err(format!(
                "Animation count exceeds maximum of {}",
                MAX_ANIMATIONS
            ));
        }

        for gradient in &self.linear_gradients {
            validate_gradient_id(&gradient.id)?;
            validate_stops(&gradient.stops)?;
        }
        for gradient in &self.radial_gradients {
            validate_gradient_id(&gradient.id)?;
            validate_stops(&gradient.stops)?;
            if gradient.r < 0.0 || !gradient.r.is_finite() {
                return Err("Radial gradient radius must be a non-negative finite number".to_string());
            }
        }
        for filter in &self.filters {
            validate_gradient_id(&filter.id)?;
            if filter.blur_std_deviation < 0.0 || !filter.blur_std_deviation.is_finite() {
                return Err("Filter blur deviation must be a non-negative finite number".to_string());
            }
        }
        for animation in &self.animations {
            if animation.target_id.is_empty() || animation.attribute_name.is_empty() {
                return Err("Animation target id and attribute name must not be empty".to_string());
            }
            if animation.duration_secs <= 0.0 || !animation.duration_secs.is_finite() {
                return Err("Animation duration must be a positive finite number".to_string());
            }
        }
        Ok(())
    }

    /// Render `<defs>` block content
    pub fn to_svg_string(&self, indent: &str) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut defs = format!("{indent}<defs>\n");
        for gradient in &self.linear_gradients {
            defs.push_str(&format!(
                "{indent}  <linearGradient id=\"{}\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\">\n",
                escape_svg_attribute(&gradient.id),
                gradient.x1,
                gradient.y1,
                gradient.x2,
                gradient.y2
            ));
            defs.push_str(&render_stops(gradient.stops.as_slice(), indent));
            defs.push_str(&format!("{indent}  </linearGradient>\n"));
        }
        for gradient in &self.radial_gradients {
            defs.push_str(&format!(
                "{indent}  <radialGradient id=\"{}\" cx=\"{}\" cy=\"{}\" r=\"{}\">\n",
                escape_svg_attribute(&gradient.id),
                gradient.cx,
                gradient.cy,
                gradient.r
            ));
            defs.push_str(&render_stops(gradient.stops.as_slice(), indent));
            defs.push_str(&format!("{indent}  </radialGradient>\n"));
        }
        for filter in &self.filters {
            defs.push_str(&format!(
                "{indent}  <filter id=\"{}\">\n{indent}    <feGaussianBlur stdDeviation=\"{}\"/>\n{indent}  </filter>\n",
                escape_svg_attribute(&filter.id),
                filter.blur_std_deviation
            ));
        }
        for animation in &self.animations {
            defs.push_str(&format!(
                "{indent}  <animate xlink:href=\"#{0}\" attributeName=\"{1}\" from=\"{2}\" to=\"{3}\" dur=\"{4}s\" repeatCount=\"{5}\"/>\n",
                escape_svg_attribute(&animation.target_id),
                escape_svg_attribute(&animation.attribute_name),
                escape_svg_attribute(&animation.from_value),
                escape_svg_attribute(&animation.to_value),
                animation.duration_secs,
                escape_svg_attribute(&animation.repeat_count)
            ));
        }
        defs.push_str(&format!("{indent}</defs>\n"));
        defs
    }
}

fn validate_gradient_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err("Definition id must not be empty".to_string());
    }
    if id.len() > 128 {
        return Err("Definition id exceeds maximum length of 128".to_string());
    }
    Ok(())
}

fn validate_stops(stops: &[SvgGradientStop]) -> Result<(), String> {
    if stops.is_empty() {
        return Err("Gradient must contain at least one stop".to_string());
    }
    if stops.len() > MAX_GRADIENT_STOPS {
        return Err(format!(
            "Gradient stop count exceeds maximum of {}",
            MAX_GRADIENT_STOPS
        ));
    }
    Ok(())
}

fn render_stops(stops: &[SvgGradientStop], indent: &str) -> String {
    let mut rendered = String::new();
    for stop in stops {
        rendered.push_str(&format!(
            "{indent}    <stop offset=\"{}\" stop-color=\"rgb({},{},{})\" stop-opacity=\"{}\"/>\n",
            stop.offset, stop.color.0, stop.color.1, stop.color.2, stop.opacity
        ));
    }
    rendered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_defs_render_linear_gradient() {
        let defs = SvgDefs {
            linear_gradients: vec![SvgLinearGradient {
                id: "grad1".to_string(),
                x1: 0.0,
                y1: 0.0,
                x2: 1.0,
                y2: 0.0,
                stops: vec![
                    SvgGradientStop::new(0.0, (255, 0, 0), 1.0).unwrap(),
                    SvgGradientStop::new(1.0, (0, 0, 255), 1.0).unwrap(),
                ],
            }],
            ..Default::default()
        };
        let svg = defs.to_svg_string("  ");
        assert!(svg.contains("<linearGradient"));
        assert!(svg.contains("grad1"));
    }

    #[test]
    fn test_svg_defs_validate_animation_limit() {
        let mut defs = SvgDefs::default();
        defs.animations = vec![SvgAnimation {
            target_id: "shape1".to_string(),
            attribute_name: "opacity".to_string(),
            from_value: "1".to_string(),
            to_value: "0".to_string(),
            duration_secs: 1.0,
            repeat_count: "indefinite".to_string(),
        }];
        assert!(defs.validate().is_ok());
    }
}
