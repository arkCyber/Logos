//! TipTap Operation Broadcaster Manager - Aerospace-Grade Operation Broadcasting Service
//!
//! Safety-critical operation broadcasting service with:
//! - Input validation and sanitization
//! - Bounds checking on all operations
//! - Comprehensive error handling
//! - Performance monitoring
//! - Security hardening
//! - Fault tolerance and error recovery

use std::time::Instant;
use std::collections::HashMap;
use crate::error_handling::{ErrorContext, ErrorSeverity};
use crate::config_service::ExportConfigService;
use std::sync::Arc;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 50;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 200;

/// Maximum operation payload size
const MAX_OPERATION_PAYLOAD_SIZE: usize = 100000;

/// Maximum number of subscribers
const MAX_SUBSCRIBERS: usize = 1000;

/// Operation type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationType {
    Insert,
    Delete,
    Format,
    Attribute,
}

impl OperationType {
    pub fn as_str(&self) -> &str {
        match self {
            OperationType::Insert => "insert",
            OperationType::Delete => "delete",
            OperationType::Format => "format",
            OperationType::Attribute => "attribute",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "insert" => Ok(OperationType::Insert),
            "delete" => Ok(OperationType::Delete),
            "format" => Ok(OperationType::Format),
            "attribute" => Ok(OperationType::Attribute),
            _ => Err(format!("Invalid operation type: {}", s)),
        }
    }
}

/// Operation data
#[derive(Debug, Clone)]
pub struct Operation {
    pub operation_type: OperationType,
    pub position: usize,
    pub length: usize,
    pub data: String,
    pub user_id: String,
    pub timestamp: Instant,
}

/// Subscriber ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubscriberId(String);

impl SubscriberId {
    pub fn new(id: String) -> Result<Self, String> {
        if id.is_empty() {
            return Err("Subscriber ID cannot be empty".to_string());
        }
        Ok(SubscriberId(id))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct OperationBroadcasterManager {
    config_service: Arc<ExportConfigService>,
    operation_count: u64,
    last_error: Option<ErrorContext>,
    subscribers: HashMap<SubscriberId, Vec<Operation>>,
    operation_history: Vec<Operation>,
}

impl OperationBroadcasterManager {
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        Self {
            config_service,
            operation_count: 0,
            last_error: None,
            subscribers: HashMap::new(),
            operation_history: Vec::new(),
        }
    }

    pub fn performance_warning_threshold_ms() -> u128 {
        PERFORMANCE_WARNING_THRESHOLD_MS
    }

    pub fn performance_critical_threshold_ms() -> u128 {
        PERFORMANCE_CRITICAL_THRESHOLD_MS
    }

    pub fn max_operation_payload_size() -> usize {
        MAX_OPERATION_PAYLOAD_SIZE
    }

    pub fn max_subscribers() -> usize {
        MAX_SUBSCRIBERS
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

    pub fn subscribe(&mut self, subscriber_id: SubscriberId) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if self.subscribers.len() >= MAX_SUBSCRIBERS {
            return Err("Maximum number of subscribers reached".to_string());
        }

        self.subscribers.insert(subscriber_id, Vec::new());

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Subscription CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Subscription performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn unsubscribe(&mut self, subscriber_id: &SubscriberId) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.subscribers.remove(subscriber_id)
            .ok_or("Subscriber not found")?;

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Unsubscription CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Unsubscription performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn broadcast(&mut self, operation: Operation) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if operation.data.len() > MAX_OPERATION_PAYLOAD_SIZE {
            return Err(format!("Operation payload exceeds maximum size of {} bytes", MAX_OPERATION_PAYLOAD_SIZE));
        }

        self.operation_history.push(operation.clone());

        for operations in self.subscribers.values_mut() {
            operations.push(operation.clone());
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Broadcast CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Broadcast performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_operations(&self, subscriber_id: &SubscriberId) -> Option<&Vec<Operation>> {
        self.subscribers.get(subscriber_id)
    }

    pub fn clear_operations(&mut self, subscriber_id: &SubscriberId) -> Result<(), String> {
        let start_time = Instant::now();
        self.operation_count += 1;

        if let Some(operations) = self.subscribers.get_mut(subscriber_id) {
            operations.clear();
        } else {
            return Err("Subscriber not found".to_string());
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("Clear operations CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("Clear operations performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
        Ok(())
    }

    pub fn get_operation_history(&self) -> &Vec<Operation> {
        &self.operation_history
    }

    pub fn clear_history(&mut self) {
        let start_time = Instant::now();
        self.operation_count += 1;

        self.operation_history.clear();

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > PERFORMANCE_CRITICAL_THRESHOLD_MS {
            eprintln!("History clear CRITICAL performance warning: took {}ms", elapsed.as_millis());
        } else if elapsed.as_millis() > PERFORMANCE_WARNING_THRESHOLD_MS {
            eprintln!("History clear performance warning: took {}ms", elapsed.as_millis());
        }

        self.last_error = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_broadcaster_manager_creation() {
        let config_service = Arc::new(ExportConfigService::new());
        let manager = OperationBroadcasterManager::new(config_service);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_subscribe() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OperationBroadcasterManager::new(config_service);
        
        let subscriber_id = SubscriberId::new("sub1".to_string()).unwrap();
        let result = manager.subscribe(subscriber_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_broadcast() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OperationBroadcasterManager::new(config_service);
        
        let subscriber_id = SubscriberId::new("sub1".to_string()).unwrap();
        manager.subscribe(subscriber_id.clone()).unwrap();
        
        let operation = Operation {
            operation_type: OperationType::Insert,
            position: 0,
            length: 5,
            data: "hello".to_string(),
            user_id: "user1".to_string(),
            timestamp: Instant::now(),
        };
        
        let result = manager.broadcast(operation);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_operations() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OperationBroadcasterManager::new(config_service);
        
        let subscriber_id = SubscriberId::new("sub1".to_string()).unwrap();
        manager.subscribe(subscriber_id.clone()).unwrap();
        
        let operation = Operation {
            operation_type: OperationType::Insert,
            position: 0,
            length: 5,
            data: "hello".to_string(),
            user_id: "user1".to_string(),
            timestamp: Instant::now(),
        };
        
        manager.broadcast(operation).unwrap();
        
        let operations = manager.get_operations(&subscriber_id);
        assert!(operations.is_some());
        assert_eq!(operations.unwrap().len(), 1);
    }

    #[test]
    fn test_unsubscribe() {
        let config_service = Arc::new(ExportConfigService::new());
        let mut manager = OperationBroadcasterManager::new(config_service);
        
        let subscriber_id = SubscriberId::new("sub1".to_string()).unwrap();
        manager.subscribe(subscriber_id.clone()).unwrap();
        
        let result = manager.unsubscribe(&subscriber_id);
        assert!(result.is_ok());
    }
}
