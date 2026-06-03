pub mod service;

#[cfg(test)]
mod tests;

pub use service::{AutoSaveService, SaveResult, SaveConfig};
