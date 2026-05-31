//! Aerospace-grade CSRF protection middleware
//! Implements double-submit cookie pattern for CSRF protection

use axum::{
    extract::{Request, State},
    http::{header, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use cookie::{Cookie, SameSite};
use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::Sha256;
use std::time::SystemTime;
use tracing::{debug, warn};

type HmacSha256 = Hmac<Sha256>;

/// CSRF configuration
#[derive(Clone)]
pub struct CsrfConfig {
    /// Secret key for HMAC signing
    pub secret: Vec<u8>,
    /// Cookie name for CSRF token
    pub cookie_name: String,
    /// Header name for CSRF token
    pub header_name: String,
    /// Token expiration in seconds
    pub token_ttl: u64,
    /// Cookie SameSite policy
    pub same_site: SameSite,
    /// Whether to use secure cookies (HTTPS only)
    pub secure: bool,
}

impl Default for CsrfConfig {
    fn default() -> Self {
        Self {
            secret: Self::generate_secret(),
            cookie_name: "csrf_token".to_string(),
            header_name: "X-CSRF-Token".to_string(),
            token_ttl: 3600, // 1 hour
            same_site: SameSite::Strict,
            secure: false, // Set to true in production with HTTPS
        }
    }
}

impl CsrfConfig {
    /// Generate a random secret key
    pub fn generate_secret() -> Vec<u8> {
        let mut secret = [0u8; 32];
        rand::thread_rng().fill(&mut secret);
        secret.to_vec()
    }

    /// Create config from environment variable
    pub fn from_env() -> Self {
        let secret = std::env::var("CSRF_SECRET")
            .ok()
            .and_then(|s| hex::decode(s).ok())
            .unwrap_or_else(Self::generate_secret);

        let secure = std::env::var("CSRF_SECURE")
            .ok()
            .map(|s| s == "true")
            .unwrap_or(false);

        Self {
            secret,
            secure,
            ..Default::default()
        }
    }
}

/// CSRF token structure
#[derive(Debug, Clone)]
pub struct CsrfToken {
    /// Random token value
    pub value: String,
    /// Timestamp when token was created
    pub timestamp: u64,
    /// HMAC signature
    pub signature: String,
}

impl CsrfToken {
    /// Generate a new CSRF token
    pub fn new(config: &CsrfConfig) -> Self {
        let value = Self::generate_random_token();
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let signature = Self::sign(&value, timestamp, &config.secret);
        
        Self {
            value,
            timestamp,
            signature,
        }
    }

    /// Generate a random token value
    fn generate_random_token() -> String {
        let mut rng = rand::thread_rng();
        let token: String = (0..32)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect();
        token
    }

    /// Sign the token with HMAC
    fn sign(value: &str, timestamp: u64, secret: &[u8]) -> String {
        let message = format!("{}:{}", value, timestamp);
        let mut mac = HmacSha256::new_from_slice(secret).unwrap();
        mac.update(message.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    /// Verify the token signature
    pub fn verify(&self, config: &CsrfConfig) -> bool {
        // Check token expiration
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if now - self.timestamp > config.token_ttl {
            debug!("CSRF token expired");
            return false;
        }

        // Verify signature
        let expected_signature = Self::sign(&self.value, self.timestamp, &config.secret);
        
        if self.signature != expected_signature {
            warn!("CSRF token signature mismatch");
            return false;
        }

        true
    }

    /// Serialize token for cookie storage
    pub fn serialize(&self) -> String {
        format!("{}:{}:{}", self.value, self.timestamp, self.signature)
    }

    /// Deserialize token from cookie storage
    pub fn deserialize(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return None;
        }

        let value = parts[0].to_string();
        let timestamp = parts[1].parse::<u64>().ok()?;
        let signature = parts[2].to_string();

        Some(Self {
            value,
            timestamp,
            signature,
        })
    }
}

/// CSRF protection middleware
#[derive(Clone)]
pub struct CsrfProtection {
    config: CsrfConfig,
}

impl CsrfProtection {
    /// Create new CSRF protection middleware
    pub fn new(config: CsrfConfig) -> Self {
        Self { config }
    }

    /// Create from environment variables
    pub fn from_env() -> Self {
        Self::new(CsrfConfig::from_env())
    }

    /// Generate CSRF token and add to response
    pub fn generate_token(&self) -> CsrfToken {
        CsrfToken::new(&self.config)
    }

    /// Set CSRF cookie on response
    pub fn set_cookie(&self, token: &CsrfToken, response: &mut Response) {
        let cookie = Cookie::build((&self.config.cookie_name, token.serialize()))
            .path("/")
            .max_age(cookie::time::Duration::seconds(self.config.token_ttl as i64))
            .same_site(self.config.same_site)
            .secure(self.config.secure)
            .http_only(true)
            .build();

        let cookie_header = cookie.to_string();
        response.headers_mut().insert(
            header::SET_COOKIE,
            HeaderValue::from_str(&cookie_header).unwrap(),
        );
    }

    /// Verify CSRF token from request
    pub fn verify_token(&self, request: &Request) -> Result<(), CsrfError> {
        // Get token from header
        let header_token = request
            .headers()
            .get(&self.config.header_name)
            .and_then(|h| h.to_str().ok())
            .ok_or(CsrfError::MissingToken)?;

        // Get token from cookie
        let cookie_token = request
            .headers()
            .get(header::COOKIE)
            .and_then(|h| h.to_str().ok())
            .and_then(|cookies| {
                cookies
                    .split(';')
                    .find_map(|c| {
                        let parts: Vec<&str> = c.trim().split('=').collect();
                        if parts.len() == 2 && parts[0] == self.config.cookie_name {
                            Some(parts[1])
                        } else {
                            None
                        }
                    })
            })
            .ok_or(CsrfError::MissingCookie)?;

        // Deserialize and verify cookie token
        let csrf_token = CsrfToken::deserialize(cookie_token)
            .ok_or(CsrfError::InvalidToken)?;

        if !csrf_token.verify(&self.config) {
            return Err(CsrfError::InvalidToken);
        }

        // Compare header token with cookie token
        if header_token != csrf_token.value {
            return Err(CsrfError::TokenMismatch);
        }

        Ok(())
    }
}

/// CSRF error types
#[derive(Debug)]
pub enum CsrfError {
    MissingToken,
    MissingCookie,
    InvalidToken,
    TokenMismatch,
}

impl axum::response::IntoResponse for CsrfError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            CsrfError::MissingToken => (StatusCode::FORBIDDEN, "CSRF token missing from header"),
            CsrfError::MissingCookie => (StatusCode::FORBIDDEN, "CSRF token missing from cookie"),
            CsrfError::InvalidToken => (StatusCode::FORBIDDEN, "Invalid CSRF token"),
            CsrfError::TokenMismatch => (StatusCode::FORBIDDEN, "CSRF token mismatch"),
        };

        let body = serde_json::json!({
            "error": message,
            "code": "CSRF_ERROR"
        });

        (status, axum::Json(body)).into_response()
    }
}

/// Axum middleware for CSRF protection
pub async fn csrf_middleware(
    State(csrf): State<CsrfProtection>,
    request: Request,
    next: Next,
) -> Result<Response, CsrfError> {
    // Skip CSRF verification for GET, HEAD, OPTIONS requests
    let method = request.method();
    if method == axum::http::Method::GET 
        || method == axum::http::Method::HEAD 
        || method == axum::http::Method::OPTIONS 
    {
        let mut response = next.run(request).await;
        
        // Generate and set CSRF token for safe methods
        let token = csrf.generate_token();
        csrf.set_cookie(&token, &mut response);
        
        // Also add token to response header for easy access
        response.headers_mut().insert(
            "X-CSRF-Token",
            HeaderValue::from_str(&token.value).unwrap(),
        );
        
        return Ok(response);
    }

    // Verify CSRF token for state-changing methods
    csrf.verify_token(&request)?;

    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csrf_token_generation() {
        let config = CsrfConfig::default();
        let token = CsrfToken::new(&config);
        
        assert!(!token.value.is_empty());
        assert!(token.timestamp > 0);
        assert!(!token.signature.is_empty());
    }

    #[test]
    fn test_csrf_token_verification() {
        let config = CsrfConfig::default();
        let token = CsrfToken::new(&config);
        
        assert!(token.verify(&config));
    }

    #[test]
    fn test_csrf_token_serialization() {
        let config = CsrfConfig::default();
        let token = CsrfToken::new(&config);
        
        let serialized = token.serialize();
        let deserialized = CsrfToken::deserialize(&serialized);
        
        assert!(deserialized.is_some());
        let deserialized = deserialized.unwrap();
        assert_eq!(deserialized.value, token.value);
        assert_eq!(deserialized.timestamp, token.timestamp);
        assert_eq!(deserialized.signature, token.signature);
    }

    #[test]
    fn test_csrf_token_expiration() {
        let mut config = CsrfConfig::default();
        config.token_ttl = 1; // 1 second
        
        let token = CsrfToken::new(&config);
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        assert!(!token.verify(&config));
    }

    #[test]
    fn test_csrf_secret_generation() {
        let secret1 = CsrfConfig::generate_secret();
        let secret2 = CsrfConfig::generate_secret();
        
        assert_ne!(secret1, secret2);
        assert_eq!(secret1.len(), 32);
        assert_eq!(secret2.len(), 32);
    }
}
