//! Authentication module with JWT token management
//! Provides user registration, login, and token validation

use crate::error::{SpreadsheetError, SpreadsheetResult};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::{info, warn};
use uuid::Uuid;

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // User ID
    pub email: String,
    pub exp: i64,     // Expiration time
    pub iat: i64,     // Issued at
}

/// User model
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Register request
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

/// Auth response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

/// User response (without password)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Authentication service
pub struct AuthService {
    jwt_secret: String,
    token_expiration_hours: i64,
}

impl AuthService {
    pub fn new(jwt_secret: String, token_expiration_hours: i64) -> Self {
        Self {
            jwt_secret,
            token_expiration_hours,
        }
    }

    /// Register a new user
    pub async fn register(
        &self,
        req: RegisterRequest,
        pool: &SqlitePool,
    ) -> SpreadsheetResult<AuthResponse> {
        info!(email = %req.email, "Registering new user");

        // Check if user already exists
        let existing = sqlx::query_as::<_, (String,)>(
            "SELECT id FROM users WHERE email = ?"
        )
        .bind(&req.email)
        .fetch_optional(pool)
        .await?;

        if existing.is_some() {
            return Err(SpreadsheetError::Authentication(
                "User with this email already exists".to_string(),
            ));
        }

        // Hash password
        let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
            .map_err(|e| SpreadsheetError::Authentication(format!("Failed to hash password: {}", e)))?;

        // Create user
        let user_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO users (id, email, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&user_id)
        .bind(&req.email)
        .bind(&password_hash)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create user: {}", e)))?;

        // Generate token
        let token = self.generate_token(&user_id, &req.email)?;

        let user_response = UserResponse {
            id: user_id.clone(),
            email: req.email.clone(),
            created_at: now,
        };

        info!(user_id = %user_id, "User registered successfully");

        Ok(AuthResponse {
            token,
            user: user_response,
        })
    }

    /// Login user
    pub async fn login(
        &self,
        req: LoginRequest,
        pool: &SqlitePool,
    ) -> SpreadsheetResult<AuthResponse> {
        info!(email = %req.email, "User login attempt");

        // Find user by email
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE email = ?"
        )
        .bind(&req.email)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| SpreadsheetError::Authentication("Invalid credentials".to_string()))?;

        // Verify password
        let is_valid = bcrypt::verify(&req.password, &user.password_hash)
            .map_err(|e| SpreadsheetError::Authentication(format!("Password verification failed: {}", e)))?;

        if !is_valid {
            warn!(email = %req.email, "Invalid password");
            return Err(SpreadsheetError::Authentication("Invalid credentials".to_string()));
        }

        // Generate token
        let token = self.generate_token(&user.id, &user.email)?;

        let user_response = UserResponse {
            id: user.id.clone(),
            email: user.email.clone(),
            created_at: user.created_at,
        };

        info!(user_id = %user.id, "User logged in successfully");

        Ok(AuthResponse {
            token,
            user: user_response,
        })
    }

    /// Generate JWT token
    fn generate_token(&self, user_id: &str, email: &str) -> SpreadsheetResult<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.token_expiration_hours);

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|e| SpreadsheetError::Authentication(format!("Failed to generate token: {}", e)))
    }

    /// Validate JWT token
    pub fn validate_token(&self, token: &str) -> SpreadsheetResult<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map(|data| data.claims)
        .map_err(|e| SpreadsheetError::Authentication(format!("Invalid token: {}", e)))
    }
}

/// Initialize users table
pub async fn init_users_table(pool: &SqlitePool) -> SpreadsheetResult<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create users table: {}", e)))?;

    info!("Users table initialized");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token() {
        let service = AuthService::new("test_secret".to_string(), 24);
        let token = service.generate_token("user123", "test@example.com");
        assert!(token.is_ok());
    }

    #[test]
    fn test_validate_token() {
        let service = AuthService::new("test_secret".to_string(), 24);
        let token = service.generate_token("user123", "test@example.com").unwrap();
        let claims = service.validate_token(&token);
        assert!(claims.is_ok());
        assert_eq!(claims.unwrap().sub, "user123");
    }
}
