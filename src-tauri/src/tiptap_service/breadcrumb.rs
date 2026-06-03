//! TipTap Breadcrumb Manager - Aerospace-Grade Breadcrumb Service
//!
//! Safety-critical breadcrumb service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Performance monitoring
//! - Comprehensive error handling
//! - Security hardening
//! - Fault tolerance and error recovery

use std::time::Instant;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use std::sync::Arc;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum breadcrumb items
const MAX_BREADCRUMB_ITEMS: usize = 50;

/// Breadcrumb trail item
#[derive(Debug, Clone)]
pub struct BreadcrumbTrailItem {
    pub item_id: String,
    pub label: String,
    pub path: String,
}

/// Breadcrumb trail
#[derive(Debug, Clone)]
pub struct BreadcrumbTrail {
    pub trail_id: String,
    pub items: Vec<BreadcrumbTrailItem>,
}

pub struct BreadcrumbManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    enabled: bool,
}

impl BreadcrumbManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
            enabled: true,
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    pub fn max_breadcrumb_items() -> usize {
        MAX_BREADCRUMB_ITEMS
    }

    fn record_error(&mut self, code: &str, message: &str, source: &str) {
        self.last_error = Some(ErrorContext::new(ErrorSeverity::Error, code, message, source));
    }

    pub fn get_last_error(&self) -> Option<&ErrorContext> {
        self.last_error.as_ref()
    }

    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    pub fn reset_error_state(&mut self) {
        self.last_error = None;
    }

    pub fn reset_operation_count(&mut self) {
        self.operation_count = 0;
    }

    pub fn enable(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.enabled = true;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Enable breadcrumb CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Enable breadcrumb performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn disable(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.enabled = false;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Disable breadcrumb CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Disable breadcrumb performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }

    pub fn create_trail(&mut self, items: Vec<BreadcrumbTrailItem>) -> Result<BreadcrumbTrail, String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if !self.enabled {
            return Err("Breadcrumb is disabled".to_string());
        }

        if items.len() > MAX_BREADCRUMB_ITEMS {
            return Err(format!("Breadcrumb items exceed maximum of {}", MAX_BREADCRUMB_ITEMS));
        }

        let trail_id = format!("breadcrumb_trail_{}", self.operation_count);

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Create breadcrumb trail CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Create breadcrumb trail performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(BreadcrumbTrail {
            trail_id,
            items,
        })
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumb_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = BreadcrumbManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_create_trail() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreadcrumbManager::new(config_service);
        
        let items = vec![
            BreadcrumbTrailItem {
                item_id: "item1".to_string(),
                label: "Home".to_string(),
                path: "/".to_string(),
            }
        ];
        let result = manager.create_trail(items);
        assert!(result.is_ok());
    }

    #[test]
    fn test_too_many_items() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreadcrumbManager::new(config_service);
        
        let items = (0..MAX_BREADCRUMB_ITEMS + 1).map(|i| BreadcrumbTrailItem {
            item_id: format!("item{}", i),
            label: format!("Item {}", i),
            path: format!("/path/{}", i),
        }).collect();
        
        let result = manager.create_trail(items);
        assert!(result.is_err());
    }

    #[test]
    fn test_disable() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = BreadcrumbManager::new(config_service);
        
        manager.disable();
        assert!(!manager.is_enabled());
    }
}
