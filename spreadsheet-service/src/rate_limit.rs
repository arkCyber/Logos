//! Aerospace-grade rate limiting middleware
//! Implements token bucket algorithm for API rate limiting

use axum::{
    extract::{Request, State},
    http::{header::HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::debug;

/// Rate limit configuration
#[derive(Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per second
    pub requests_per_second: u32,
    /// Burst size (maximum concurrent requests)
    pub burst_size: u32,
    /// Time window for rate limiting
    pub window_duration: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 100,
            burst_size: 200,
            window_duration: Duration::from_secs(60),
        }
    }
}

impl RateLimitConfig {
    /// Create from environment variables
    pub fn from_env() -> Self {
        let requests_per_second = std::env::var("RATE_LIMIT_RPS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(100);
        
        let burst_size = std::env::var("RATE_LIMIT_BURST")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(200);
        
        let window_secs = std::env::var("RATE_LIMIT_WINDOW")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(60);
        
        Self {
            requests_per_second,
            burst_size,
            window_duration: Duration::from_secs(window_secs),
        }
    }
}

/// Token bucket for rate limiting
#[derive(Clone)]
struct TokenBucket {
    capacity: u32,
    tokens: f64,
    last_refill: Instant,
    refill_rate: f64,
}

impl TokenBucket {
    fn new(capacity: u32, refill_rate: f64) -> Self {
        Self {
            capacity,
            tokens: capacity as f64,
            last_refill: Instant::now(),
            refill_rate,
        }
    }

    fn try_consume(&mut self, tokens: u32) -> bool {
        self.refill();
        
        if self.tokens >= tokens as f64 {
            self.tokens -= tokens as f64;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        
        let tokens_to_add = elapsed * self.refill_rate;
        self.tokens = (self.tokens + tokens_to_add).min(self.capacity as f64);
        self.last_refill = now;
    }
}

/// Rate limiter state
#[derive(Clone)]
pub struct RateLimiter {
    buckets: Arc<Mutex<HashMap<IpAddr, TokenBucket>>>,
    config: RateLimitConfig,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    /// Create from environment variables
    pub fn from_env() -> Self {
        Self::new(RateLimitConfig::from_env())
    }

    /// Check if a request should be rate limited
    pub fn check_rate_limit(&self, ip: IpAddr) -> Result<(), RateLimitError> {
        let mut buckets = self.buckets.lock().unwrap();
        
        let bucket = buckets.entry(ip).or_insert_with(|| {
            TokenBucket::new(
                self.config.burst_size,
                self.config.requests_per_second as f64,
            )
        });

        if bucket.try_consume(1) {
            Ok(())
        } else {
            Err(RateLimitError::RateLimitExceeded)
        }
    }

    /// Clean up old buckets to prevent memory leaks
    pub fn cleanup_old_buckets(&self) {
        let mut buckets = self.buckets.lock().unwrap();
        let now = Instant::now();
        
        buckets.retain(|_, bucket| {
            now.duration_since(bucket.last_refill) < Duration::from_secs(300)
        });
    }
}

/// Rate limit error types
#[derive(Debug)]
pub enum RateLimitError {
    RateLimitExceeded,
    InvalidIp,
}

impl axum::response::IntoResponse for RateLimitError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            RateLimitError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded"),
            RateLimitError::InvalidIp => (StatusCode::BAD_REQUEST, "Invalid IP address"),
        };

        let body = serde_json::json!({
            "error": message,
            "code": "RATE_LIMIT_ERROR"
        });

        (status, axum::Json(body)).into_response()
    }
}

/// Extract client IP from request
fn extract_client_ip(headers: &HeaderMap) -> Result<IpAddr, RateLimitError> {
    // Check for forwarded headers (behind proxy)
    if let Some(forwarded) = headers.get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            // Take the first IP in the list
            if let Some(ip_str) = forwarded_str.split(',').next() {
                if let Ok(ip) = ip_str.trim().parse::<IpAddr>() {
                    return Ok(ip);
                }
            }
        }
    }

    // Check for real IP header
    if let Some(real_ip) = headers.get("X-Real-IP") {
        if let Ok(ip_str) = real_ip.to_str() {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                return Ok(ip);
            }
        }
    }

    // Fallback - in a real application, you'd get this from the connection info
    // For now, return localhost as a fallback
    Ok(IpAddr::from([127, 0, 0, 1]))
}

/// Axum middleware for rate limiting
pub async fn rate_limit_middleware(
    State(limiter): State<RateLimiter>,
    request: Request,
    next: Next,
) -> Result<Response, RateLimitError> {
    // Extract client IP
    let ip = extract_client_ip(request.headers())?;

    // Check rate limit
    limiter.check_rate_limit(ip)?;

    // Allow request to proceed
    Ok(next.run(request).await)
}

/// Periodic cleanup task for rate limiter
pub async fn cleanup_task(limiter: RateLimiter) {
    let mut interval = tokio::time::interval(Duration::from_secs(300)); // Every 5 minutes
    
    loop {
        interval.tick().await;
        limiter.cleanup_old_buckets();
        debug!("Rate limiter cleanup completed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_bucket_consume() {
        let mut bucket = TokenBucket::new(10, 1.0); // 10 capacity, 1 refill/sec
        
        // Should be able to consume up to capacity
        for _ in 0..10 {
            assert!(bucket.try_consume(1));
        }
        
        // Should fail when empty
        assert!(!bucket.try_consume(1));
    }

    #[test]
    fn test_token_bucket_refill() {
        let mut bucket = TokenBucket::new(10, 10.0); // 10 capacity, 10 refill/sec
        
        // Consume all tokens
        for _ in 0..10 {
            assert!(bucket.try_consume(1));
        }
        
        // Wait for refill
        std::thread::sleep(Duration::from_millis(110));
        
        // Should be able to consume again
        assert!(bucket.try_consume(1));
    }

    #[test]
    fn test_rate_limiter_check() {
        let limiter = RateLimiter::new(RateLimitConfig {
            requests_per_second: 10,
            burst_size: 5,
            window_duration: Duration::from_secs(60),
        });
        
        let ip = IpAddr::from([127, 0, 0, 1]);
        
        // Should allow up to burst size
        for _ in 0..5 {
            assert!(limiter.check_rate_limit(ip).is_ok());
        }
        
        // Should fail after burst
        assert!(limiter.check_rate_limit(ip).is_err());
    }

    #[test]
    fn test_rate_limit_config_from_env() {
        std::env::set_var("RATE_LIMIT_RPS", "50");
        std::env::set_var("RATE_LIMIT_BURST", "100");
        std::env::set_var("RATE_LIMIT_WINDOW", "30");
        
        let config = RateLimitConfig::from_env();
        
        assert_eq!(config.requests_per_second, 50);
        assert_eq!(config.burst_size, 100);
        assert_eq!(config.window_duration, Duration::from_secs(30));
        
        std::env::remove_var("RATE_LIMIT_RPS");
        std::env::remove_var("RATE_LIMIT_BURST");
        std::env::remove_var("RATE_LIMIT_WINDOW");
    }

    #[test]
    fn test_rate_limit_config_default() {
        let config = RateLimitConfig::default();
        
        assert_eq!(config.requests_per_second, 100);
        assert_eq!(config.burst_size, 200);
        assert_eq!(config.window_duration, Duration::from_secs(60));
    }
}
