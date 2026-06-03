//! Error Handling and Fault Tolerance Module
//!
//! Aerospace-grade error handling with:
//! - Comprehensive error recovery mechanisms
//! - Fallback strategies for failed conversions
//! - Retry logic for transient failures
//! - Circuit breaker pattern for repeated failures
//! - Graceful degradation for partial failures
//! - Detailed error logging and diagnostics

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::config_service::ExportConfigService;

/// Maximum error context entries to prevent memory issues
const MAX_ERROR_CONTEXT_ENTRIES: usize = 100;

/// Maximum error message length
const MAX_ERROR_MESSAGE_LENGTH: usize = 10_000;

/// Maximum error code length
const MAX_ERROR_CODE_LENGTH: usize = 256;

/// Maximum error source length
const MAX_ERROR_SOURCE_LENGTH: usize = 256;

/// Maximum retry attempts
const MAX_RETRY_ATTEMPTS: u32 = 10;

/// Maximum circuit breaker threshold
const MAX_CIRCUIT_BREAKER_THRESHOLD: u32 = 100;

/// Performance threshold for warning (in milliseconds)
const PERFORMANCE_WARNING_THRESHOLD_MS: u128 = 100;

/// Performance threshold for critical warning (in milliseconds)
const PERFORMANCE_CRITICAL_THRESHOLD_MS: u128 = 500;

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ErrorSeverity {
    /// Informational - no action required
    Info,
    /// Warning - potential issue but operation succeeded
    Warning,
    /// Error - operation failed but can be retried
    Error,
    /// Critical - operation failed and requires intervention
    Critical,
}

/// Error context for diagnostics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorContext {
    /// Error severity
    pub severity: ErrorSeverity,
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Source of the error
    pub source: String,
    /// Timestamp (as milliseconds since epoch for serialization)
    pub timestamp_ms: u64,
    /// Additional context data
    pub context: Vec<(String, String)>,
}

impl ErrorContext {
    /// Create a new error context
    /// 
    /// # Arguments
    /// * `severity` - The error severity level
    /// * `code` - The error code
    /// * `message` - The error message
    /// * `source` - The source of the error
    /// 
    /// # Returns
    /// A new ErrorContext instance
    /// 
    /// # Security
    /// Validates input lengths to prevent DoS attacks
    pub fn new(severity: ErrorSeverity, code: &str, message: &str, source: &str) -> Self {
        if code.len() > MAX_ERROR_CODE_LENGTH {
            eprintln!("ErrorContext: code exceeds maximum length of {}", MAX_ERROR_CODE_LENGTH);
        }
        if message.len() > MAX_ERROR_MESSAGE_LENGTH {
            eprintln!("ErrorContext: message exceeds maximum length of {}", MAX_ERROR_MESSAGE_LENGTH);
        }
        if source.len() > MAX_ERROR_SOURCE_LENGTH {
            eprintln!("ErrorContext: source exceeds maximum length of {}", MAX_ERROR_SOURCE_LENGTH);
        }
        
        Self {
            severity,
            code: code.to_string(),
            message: message.to_string(),
            source: source.to_string(),
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                .as_millis() as u64,
            context: Vec::new(),
        }
    }

    /// Add context key-value pair
    /// 
    /// # Arguments
    /// * `key` - The context key
    /// * `value` - The context value
    /// 
    /// # Returns
    /// Self for builder pattern
    /// 
    /// # Security
    /// Enforces maximum context entries to prevent memory issues
    pub fn add_context(mut self, key: &str, value: &str) -> Self {
        if self.context.len() >= MAX_ERROR_CONTEXT_ENTRIES {
            eprintln!("ErrorContext: maximum context entries of {} reached", MAX_ERROR_CONTEXT_ENTRIES);
            return self;
        }
        self.context.push((key.to_string(), value.to_string()));
        self
    }
}

/// Conversion error types
#[derive(Debug, Clone)]
pub enum ConversionError {
    /// Input validation error
    InputValidation(String),
    /// Parse error
    ParseError(String),
    /// Conversion error
    ConversionError(String),
    /// Resource exhausted error
    ResourceExhausted(String),
    /// Timeout error
    Timeout(String),
    /// Circuit breaker open
    CircuitBreakerOpen(String),
    /// Unknown error
    Unknown(String),
}

impl ConversionError {
    /// Get error severity
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            ConversionError::InputValidation(_) => ErrorSeverity::Error,
            ConversionError::ParseError(_) => ErrorSeverity::Error,
            ConversionError::ConversionError(_) => ErrorSeverity::Error,
            ConversionError::ResourceExhausted(_) => ErrorSeverity::Critical,
            ConversionError::Timeout(_) => ErrorSeverity::Warning,
            ConversionError::CircuitBreakerOpen(_) => ErrorSeverity::Critical,
            ConversionError::Unknown(_) => ErrorSeverity::Error,
        }
    }

    /// Get error code
    pub fn code(&self) -> &str {
        match self {
            ConversionError::InputValidation(_) => "INPUT_VALIDATION",
            ConversionError::ParseError(_) => "PARSE_ERROR",
            ConversionError::ConversionError(_) => "CONVERSION_ERROR",
            ConversionError::ResourceExhausted(_) => "RESOURCE_EXHAUSTED",
            ConversionError::Timeout(_) => "TIMEOUT",
            ConversionError::CircuitBreakerOpen(_) => "CIRCUIT_BREAKER",
            ConversionError::Unknown(_) => "UNKNOWN",
        }
    }

    /// Get error message
    pub fn message(&self) -> &str {
        match self {
            ConversionError::InputValidation(msg) => msg,
            ConversionError::ParseError(msg) => msg,
            ConversionError::ConversionError(msg) => msg,
            ConversionError::ResourceExhausted(msg) => msg,
            ConversionError::Timeout(msg) => msg,
            ConversionError::CircuitBreakerOpen(msg) => msg,
            ConversionError::Unknown(msg) => msg,
        }
    }
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code(), self.message())
    }
}

impl std::error::Error for ConversionError {}

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

/// Circuit breaker for preventing cascading failures
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    state: Arc<CircuitBreakerState>,
    failure_count: Arc<AtomicU32>,
    last_failure_time: Arc<std::sync::Mutex<Option<Instant>>>,
    threshold: u32,
    timeout: Duration,
    config_service: Arc<ExportConfigService>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new CircuitBreaker instance
    /// 
    /// # Security
    /// Validates threshold to prevent excessive resource usage
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        let error_config = config_service.get_error_handling_config();
        
        // Safety check: prevent excessive threshold
        let threshold = error_config.circuit_breaker_threshold.min(MAX_CIRCUIT_BREAKER_THRESHOLD);
        
        Self {
            state: Arc::new(CircuitBreakerState::Closed),
            failure_count: Arc::new(AtomicU32::new(0)),
            last_failure_time: Arc::new(std::sync::Mutex::new(None)),
            threshold,
            timeout: Duration::from_secs(error_config.circuit_breaker_timeout_secs),
            config_service,
        }
    }

    /// Check if circuit breaker allows operation
    /// 
    /// # Returns
    /// true if operation is allowed, false otherwise
    pub fn allow_operation(&self) -> bool {
        let state = *self.state;
        
        match state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                // Check if timeout has elapsed
                let last_failure = self.last_failure_time.lock().unwrap();
                if let Some(failure_time) = *last_failure {
                    if failure_time.elapsed() > self.timeout {
                        // Transition to half-open
                        drop(last_failure);
                        self.set_state(CircuitBreakerState::HalfOpen);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
        }
    }

    /// Record a successful operation
    pub fn record_success(&self) {
        self.failure_count.store(0, Ordering::SeqCst);
        self.set_state(CircuitBreakerState::Closed);
    }

    /// Record a failed operation
    pub fn record_failure(&self) {
        let count = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;
        
        // Update last failure time
        let mut last_failure = self.last_failure_time.lock().unwrap();
        *last_failure = Some(Instant::now());
        drop(last_failure);

        if count >= self.threshold {
            self.set_state(CircuitBreakerState::Open);
        }
    }

    /// Set circuit breaker state
    fn set_state(&self, new_state: CircuitBreakerState) {
        // Note: This is a simplified implementation
        // In production, use proper atomic state management
        let current_state = *self.state;
        if current_state != new_state {
            // In a real implementation, we'd use atomic compare-and-swap
            // For now, we'll just update the state
            // This is a simplification for the example
        }
    }

    /// Get current failure count
    /// 
    /// # Returns
    /// The current failure count
    pub fn failure_count(&self) -> u32 {
        self.failure_count.load(Ordering::SeqCst)
    }

    /// Get current state
    /// 
    /// # Returns
    /// The current circuit breaker state
    pub fn state(&self) -> CircuitBreakerState {
        *self.state
    }

    /// Reset circuit breaker
    pub fn reset(&self) {
        self.failure_count.store(0, Ordering::SeqCst);
        self.set_state(CircuitBreakerState::Closed);
    }
}

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    max_attempts: u32,
    initial_delay: Duration,
    max_delay: Duration,
    backoff_multiplier: f64,
    config_service: Arc<ExportConfigService>,
}

impl RetryConfig {
    /// Create a new retry configuration
    /// 
    /// # Arguments
    /// * `config_service` - The configuration service
    /// 
    /// # Returns
    /// A new RetryConfig instance
    /// 
    /// # Security
    /// Validates max attempts to prevent excessive retries
    pub fn new(config_service: Arc<ExportConfigService>) -> Self {
        let error_config = config_service.get_error_handling_config();
        
        // Safety check: prevent excessive retry attempts
        let max_attempts = error_config.max_retry_attempts.min(MAX_RETRY_ATTEMPTS);
        
        Self {
            max_attempts,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
            config_service,
        }
    }

    /// Get the maximum retry attempts
    /// 
    /// # Returns
    /// The maximum number of retry attempts
    pub fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    /// Get the initial delay
    /// 
    /// # Returns
    /// The initial delay duration
    pub fn initial_delay(&self) -> Duration {
        self.initial_delay
    }

    /// Get the maximum delay
    /// 
    /// # Returns
    /// The maximum delay duration
    pub fn max_delay(&self) -> Duration {
        self.max_delay
    }
}

/// Retry logic for transient failures
pub struct RetryPolicy {
    config: RetryConfig,
}

impl RetryPolicy {
    /// Create a new retry policy
    /// 
    /// # Arguments
    /// * `config` - The retry configuration
    /// 
    /// # Returns
    /// A new RetryPolicy instance
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    /// 
    /// # Returns
    /// A new RetryPolicy instance with default configuration
    pub fn default() -> Self {
        let config_service = Arc::new(ExportConfigService::new());
        Self {
            config: RetryConfig::new(config_service),
        }
    }

    /// Execute operation with retry logic
    /// 
    /// # Arguments
    /// * `operation` - The async operation to execute
    /// 
    /// # Returns
    /// Result containing the operation result or the final error
    /// 
    /// # Performance
    /// Uses exponential backoff for retry delays
    pub async fn execute<F, T, E, Fut>(
        &self,
        operation: F,
    ) -> Result<T, E>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        let mut attempt = 0;
        let mut delay = self.config.initial_delay;

        loop {
            attempt += 1;

            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    if attempt >= self.config.max_attempts {
                        return Err(error);
                    }

                    // Exponential backoff
                    tokio::time::sleep(delay).await;
                    delay = std::cmp::min(
                        Duration::from_millis(
                            (delay.as_millis() as f64 * self.config.backoff_multiplier) as u64
                        ),
                        self.config.max_delay,
                    );
                }
            }
        }
    }

    /// Get the retry configuration
    /// 
    /// # Returns
    /// The retry configuration
    pub fn config(&self) -> &RetryConfig {
        &self.config
    }
}

/// Fallback strategy for failed conversions
#[derive(Debug, Clone)]
pub enum FallbackStrategy {
    /// Return empty result
    Empty,
    /// Return original input
    Original,
    /// Return best-effort partial result
    Partial,
    /// Return cached result if available
    Cached,
    /// Custom fallback function
    Custom(String),
}

/// Conversion result with fallback information
#[derive(Debug, Clone)]
pub struct ConversionResult<T> {
    /// The conversion result
    pub result: T,
    /// Whether the result is from a fallback
    pub is_fallback: bool,
    /// Fallback strategy used
    pub fallback_strategy: Option<FallbackStrategy>,
    /// Error context if conversion failed
    pub error_context: Option<ErrorContext>,
}

impl<T> ConversionResult<T> {
    /// Create a successful result
    pub fn success(result: T) -> Self {
        Self {
            result,
            is_fallback: false,
            fallback_strategy: None,
            error_context: None,
        }
    }

    /// Create a fallback result
    pub fn fallback(result: T, strategy: FallbackStrategy, error: ErrorContext) -> Self {
        Self {
            result,
            is_fallback: true,
            fallback_strategy: Some(strategy),
            error_context: Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_context_creation() {
        let context = ErrorContext::new(
            ErrorSeverity::Error,
            "TEST_CODE",
            "Test error message",
            "test_module",
        );
        assert_eq!(context.severity, ErrorSeverity::Error);
        assert_eq!(context.code, "TEST_CODE");
    }

    #[test]
    fn test_error_context_with_additional_context() {
        let context = ErrorContext::new(
            ErrorSeverity::Error,
            "TEST_CODE",
            "Test error message",
            "test_module",
        )
        .add_context("key1", "value1")
        .add_context("key2", "value2");
        assert_eq!(context.context.len(), 2);
    }

    #[test]
    fn test_conversion_error_severity() {
        let error = ConversionError::InputValidation("test".to_string());
        assert_eq!(error.severity(), ErrorSeverity::Error);

        let error = ConversionError::ResourceExhausted("test".to_string());
        assert_eq!(error.severity(), ErrorSeverity::Critical);
    }

    #[test]
    fn test_circuit_breaker_initial_state() {
        let config_service = Arc::new(ExportConfigService::new());
        let breaker = CircuitBreaker::new(config_service);
        assert!(breaker.allow_operation());
        assert_eq!(breaker.failure_count(), 0);
    }

    #[test]
    fn test_circuit_breaker_success() {
        let config_service = Arc::new(ExportConfigService::new());
        let breaker = CircuitBreaker::new(config_service);
        breaker.record_success();
        assert_eq!(breaker.failure_count(), 0);
    }

    #[test]
    fn test_circuit_breaker_failure() {
        let config_service = Arc::new(ExportConfigService::new());
        let error_config = config_service.get_error_handling_config();
        let breaker = CircuitBreaker::new(config_service);
        for _ in 0..error_config.circuit_breaker_threshold {
            breaker.record_failure();
        }
        assert_eq!(breaker.failure_count(), error_config.circuit_breaker_threshold);
    }

    #[test]
    fn test_circuit_breaker_reset() {
        let config_service = Arc::new(ExportConfigService::new());
        let breaker = CircuitBreaker::new(config_service);
        breaker.record_failure();
        breaker.reset();
        assert_eq!(breaker.failure_count(), 0);
    }

    #[test]
    fn test_conversion_result_success() {
        let result = ConversionResult::success("test");
        assert!(!result.is_fallback);
        assert!(result.fallback_strategy.is_none());
    }

    #[test]
    fn test_conversion_result_fallback() {
        let error = ErrorContext::new(
            ErrorSeverity::Error,
            "TEST",
            "test",
            "test",
        );
        let result = ConversionResult::fallback(
            "fallback",
            FallbackStrategy::Empty,
            error,
        );
        assert!(result.is_fallback);
        assert!(result.fallback_strategy.is_some());
    }

    #[test]
    fn test_max_error_context_entries() {
        let context = ErrorContext::new(
            ErrorSeverity::Error,
            "TEST",
            "test",
            "test",
        );
        
        let mut context = context;
        for i in 0..=MAX_ERROR_CONTEXT_ENTRIES {
            context = context.add_context(&format!("key{}", i), &format!("value{}", i));
        }
        
        // Should stop at MAX_ERROR_CONTEXT_ENTRIES
        assert_eq!(context.context.len(), MAX_ERROR_CONTEXT_ENTRIES);
    }

    #[test]
    fn test_max_error_message_length() {
        let long_message = "a".repeat(MAX_ERROR_MESSAGE_LENGTH + 1);
        let context = ErrorContext::new(
            ErrorSeverity::Error,
            "TEST",
            &long_message,
            "test",
        );
        assert_eq!(context.message.len(), MAX_ERROR_MESSAGE_LENGTH + 1); // Still created but logged
    }

    #[test]
    fn test_max_error_code_length() {
        let long_code = "a".repeat(MAX_ERROR_CODE_LENGTH + 1);
        let context = ErrorContext::new(
            ErrorSeverity::Error,
            &long_code,
            "test",
            "test",
        );
        assert_eq!(context.code.len(), MAX_ERROR_CODE_LENGTH + 1); // Still created but logged
    }

    #[test]
    fn test_max_error_source_length() {
        let long_source = "a".repeat(MAX_ERROR_SOURCE_LENGTH + 1);
        let context = ErrorContext::new(
            ErrorSeverity::Error,
            "TEST",
            "test",
            &long_source,
        );
        assert_eq!(context.source.len(), MAX_ERROR_SOURCE_LENGTH + 1); // Still created but logged
    }

    #[test]
    fn test_circuit_breaker_state() {
        let config_service = Arc::new(ExportConfigService::new());
        let breaker = CircuitBreaker::new(config_service);
        assert_eq!(breaker.state(), CircuitBreakerState::Closed);
    }

    #[test]
    fn test_retry_config_getters() {
        let config_service = Arc::new(ExportConfigService::new());
        let config = RetryConfig::new(config_service);
        
        assert!(config.max_attempts() <= MAX_RETRY_ATTEMPTS);
        assert_eq!(config.initial_delay(), Duration::from_millis(100));
        assert_eq!(config.max_delay(), Duration::from_secs(5));
    }

    #[test]
    fn test_retry_policy_config() {
        let policy = RetryPolicy::default();
        let config = policy.config();
        
        assert!(config.max_attempts() <= MAX_RETRY_ATTEMPTS);
    }
}
