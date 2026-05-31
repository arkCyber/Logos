pub mod data_processor;
pub mod template_engine;

pub use data_processor::{DataProcessor, DataSource, MergeBatchResult, MergeConfig};
pub use template_engine::{MergeResult, TemplateEngine, TemplateVariable};
