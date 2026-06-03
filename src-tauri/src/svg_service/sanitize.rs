/*!
 * Aerospace-Grade SVG Sanitization Utilities
 *
 * Shared XML escaping and color validation for svg_service, chart_service,
 * and typist_service SVG output paths.
 */

/// Maximum SVG color string length in bytes
pub const MAX_COLOR_STRING_LENGTH: usize = 64;

/// Escape text content for safe inclusion in SVG/XML text nodes.
pub fn escape_svg_text(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Escape attribute values for safe inclusion in SVG/XML attributes.
pub fn escape_svg_attribute(value: &str) -> String {
    escape_svg_text(value).replace('"', "&quot;")
}

/// Validate and normalize an SVG color value.
///
/// Accepts `#rgb`, `#rrggbb`, `rgb(r,g,b)`, and a small named-color whitelist.
pub fn validate_svg_color(color: &str) -> Result<String, String> {
    let trimmed = color.trim();
    if trimmed.is_empty() {
        return Err("Color must not be empty".to_string());
    }
    if trimmed.len() > MAX_COLOR_STRING_LENGTH {
        return Err(format!(
            "Color exceeds maximum length of {}",
            MAX_COLOR_STRING_LENGTH
        ));
    }
    if trimmed.contains('\0') {
        return Err("Color must not contain null bytes".to_string());
    }

    let lower = trimmed.to_ascii_lowercase();
    if is_named_color(&lower) {
        return Ok(lower);
    }

    if lower.starts_with('#') {
        let hex = &lower[1..];
        if hex.len() == 3 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(format!("#{}", hex));
        }
        if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(format!("#{}", hex));
        }
        return Err(format!("Invalid hex color: {}", trimmed));
    }

    if lower.starts_with("rgb(") && lower.ends_with(')') {
        let inner = &lower[4..lower.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(str::trim).collect();
        if parts.len() != 3 {
            return Err(format!("Invalid rgb color: {}", trimmed));
        }
        let mut values = [0u8; 3];
        for (idx, part) in parts.iter().enumerate() {
            let value: u16 = part
                .parse()
                .map_err(|_| format!("Invalid rgb component in color: {}", trimmed))?;
            if value > 255 {
                return Err(format!("RGB component out of range in color: {}", trimmed));
            }
            values[idx] = value as u8;
        }
        return Ok(format!("rgb({},{},{})", values[0], values[1], values[2]));
    }

    Err(format!("Unsupported SVG color format: {}", trimmed))
}

/// Check whether a lowercase color token is an allowed named color.
fn is_named_color(color: &str) -> bool {
    matches!(
        color,
        "black"
            | "white"
            | "red"
            | "green"
            | "blue"
            | "yellow"
            | "orange"
            | "purple"
            | "gray"
            | "grey"
            | "none"
            | "transparent"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_svg_text() {
        assert_eq!(
            escape_svg_text("<script>&</script>"),
            "&lt;script&gt;&amp;&lt;/script&gt;"
        );
    }

    #[test]
    fn test_escape_svg_attribute_quotes() {
        assert_eq!(escape_svg_attribute("\"x\""), "&quot;x&quot;");
    }

    #[test]
    fn test_validate_svg_color_hex() {
        assert_eq!(validate_svg_color("#3B82F6").unwrap(), "#3b82f6");
        assert_eq!(validate_svg_color("#abc").unwrap(), "#abc");
    }

    #[test]
    fn test_validate_svg_color_rgb() {
        assert_eq!(
            validate_svg_color("rgb(10, 20, 30)").unwrap(),
            "rgb(10,20,30)"
        );
    }

    #[test]
    fn test_validate_svg_color_named() {
        assert_eq!(validate_svg_color("White").unwrap(), "white");
    }

    #[test]
    fn test_validate_svg_color_rejects_script() {
        assert!(validate_svg_color("<script>").is_err());
    }

    #[test]
    fn test_validate_svg_color_rejects_empty() {
        assert!(validate_svg_color("").is_err());
    }
}
