//! Aerospace-grade unit tests for security modules
//! Tests CSRF protection and rate limiting

#[cfg(test)]
mod tests {
    use spreadsheet_service::csrf::{CsrfToken, CsrfConfig, CsrfProtection};
    use spreadsheet_service::rate_limit::{RateLimiter, RateLimitConfig, TokenBucket};
    use std::time::Duration;

    #[test]
    fn test_csrf_token_generation() {
        let config = CsrfConfig::default();
        let protection = CsrfProtection::new(config);
        let token = protection.generate_token();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_csrf_token_verification() {
        let config = CsrfConfig::default();
        let protection = CsrfProtection::new(config);
        let token = protection.generate_token();
        assert!(protection.verify_token(&token).is_ok());
    }

    #[test]
    fn test_csrf_token_verification_invalid() {
        let config = CsrfConfig::default();
        let protection = CsrfProtection::new(config);
        let result = protection.verify_token("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    fn test_csrf_token_expiration() {
        let config = CsrfConfig {
            token_expiration_secs: 1,
            ..Default::default()
        };
        let protection = CsrfProtection::new(config);
        let token = protection.generate_token();
        
        // Wait for token to expire
        std::thread::sleep(Duration::from_secs(2));
        
        let result = protection.verify_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_rate_limit_token_bucket() {
        let config = RateLimitConfig {
            requests_per_second: 10,
            burst: 20,
            ..Default::default()
        };
        let mut bucket = TokenBucket::new(config);
        
        // Should allow burst requests
        for _ in 0..20 {
            assert!(bucket.try_consume().is_ok());
        }
        
        // Should exceed burst
        assert!(bucket.try_consume().is_err());
    }

    #[test]
    fn test_rate_limit_refill() {
        let config = RateLimitConfig {
            requests_per_second: 10,
            burst: 10,
            ..Default::default()
        };
        let mut bucket = TokenBucket::new(config);
        
        // Consume all tokens
        for _ in 0..10 {
            bucket.try_consume().ok();
        }
        
        assert!(bucket.try_consume().is_err());
        
        // Wait for refill
        std::thread::sleep(Duration::from_millis(150));
        
        // Should have refilled
        assert!(bucket.try_consume().is_ok());
    }

    #[test]
    fn test_rate_limiter_different_ips() {
        let config = RateLimitConfig::default();
        let mut limiter = RateLimiter::new(config);
        
        let ip1 = "192.168.1.1";
        let ip2 = "192.168.1.2";
        
        // Both IPs should be able to make requests
        for _ in 0..10 {
            assert!(limiter.check_rate_limit(ip1).is_ok());
            assert!(limiter.check_rate_limit(ip2).is_ok());
        }
    }

    #[test]
    fn test_rate_limiter_exceeds_limit() {
        let config = RateLimitConfig {
            requests_per_second: 5,
            burst: 5,
            ..Default::default()
        };
        let mut limiter = RateLimiter::new(config);
        let ip = "192.168.1.1";
        
        // Consume all tokens
        for _ in 0..5 {
            limiter.check_rate_limit(ip).ok();
        }
        
        // Should exceed limit
        assert!(limiter.check_rate_limit(ip).is_err());
    }
}
