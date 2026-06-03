/*!
 * Aerospace-Grade SVG Service Module
 *
 * Provides validated SVG vector graphics generation, styling, element modeling,
 * HTML layout parsing, visual effects, and export pipelines.
 */

use std::sync::Once;

static SVG_SERVICE_INIT: Once = Once::new();

/// Log module initialization once with a Unix timestamp (milliseconds).
pub(crate) fn ensure_module_initialized() {
    SVG_SERVICE_INIT.call_once(|| {
        let timestamp_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap_or(0);
        eprintln!(
            "[svg_service][{}] Aerospace-grade SVG service module loaded",
            timestamp_ms
        );
    });
}

pub mod config;
pub mod effects;
pub mod element;
pub mod export;
pub mod html_parser;
pub mod sanitize;
pub mod style;

pub use config::{SvgConfig, SvgVersion};
pub use element::{SvgElement, SvgElementType};
pub use export::SvgExporter;
pub use style::SvgStyle;
