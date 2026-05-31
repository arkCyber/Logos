//! Aerospace-grade unit tests for authentication module
//! Tests JWT token generation, validation, and password hashing

#[cfg(test)]
mod tests {
    use spreadsheet_service::auth::{AuthService, Claims, RegisterRequest, LoginRequest};
    use chrono::{Utc, Duration};
    use sqlx::SqlitePool;

    async fn create_test_pool() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to create test database");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create users table");

        pool
    }

    #[tokio::test]
    async fn test_register_user_success() {
        let pool = create_test_pool().await;
        let auth_service = AuthService::new(pool, "test-secret".to_string(), 12);

        let request = RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "SecurePassword123!".to_string(),
        };

        let result = auth_service.register(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_duplicate_username() {
        let pool = create_test_pool().await;
        let auth_service = AuthService::new(pool, "test-secret".to_string(), 12);

        let request = RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "SecurePassword123!".to_string(),
        };

        // First registration
        auth_service.register(request.clone()).await.unwrap();

        // Duplicate registration should fail
        let result = auth_service.register(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_success() {
        let pool = create_test_pool().await;
        let auth_service = AuthService::new(pool, "test-secret".to_string(), 12);

        let register_request = RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "SecurePassword123!".to_string(),
        };

        auth_service.register(register_request).await.unwrap();

        let login_request = LoginRequest {
            username: "testuser".to_string(),
            password: "SecurePassword123!".to_string(),
        };

        let result = auth_service.login(login_request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_invalid_password() {
        let pool = create_test_pool().await;
        let auth_service = AuthService::new(pool, "test-secret".to_string(), 12);

        let register_request = RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "SecurePassword123!".to_string(),
        };

        auth_service.register(register_request).await.unwrap();

        let login_request = LoginRequest {
            username: "testuser".to_string(),
            password: "WrongPassword".to_string(),
        };

        let result = auth_service.login(login_request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_nonexistent_user() {
        let pool = create_test_pool().await;
        let auth_service = AuthService::new(pool, "test-secret".to_string(), 12);

        let login_request = LoginRequest {
            username: "nonexistent".to_string(),
            password: "Password123!".to_string(),
        };

        let result = auth_service.login(login_request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_valid() {
        let pool = create_test_pool().await;
        let auth_service = AuthService::new(pool, "test-secret".to_string(), 12);

        let register_request = RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "SecurePassword123!".to_string(),
        };

        auth_service.register(register_request).await.unwrap();

        let login_request = LoginRequest {
            username: "testuser".to_string(),
            password: "SecurePassword123!".to_string(),
        };

        let response = auth_service.login(login_request).await.unwrap();
        let result = auth_service.validate_token(&response.token);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_token_invalid() {
        let pool = create_test_pool().await;
        let auth_service = AuthService::new(pool, "test-secret".to_string(), 12);

        let result = auth_service.validate_token("invalid_token");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_expired() {
        let pool = create_test_pool().await;
        let short_secret = "short-secret".to_string();
        let auth_service = AuthService::new(pool, short_secret, 12);

        let register_request = RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "SecurePassword123!".to_string(),
        };

        auth_service.register(register_request).await.unwrap();

        let login_request = LoginRequest {
            username: "testuser".to_string(),
            password: "SecurePassword123!".to_string(),
        };

        let response = auth_service.login(login_request).await.unwrap();

        // Wait for token to expire (assuming short expiration)
        tokio::time::sleep(Duration::seconds(3601).to_std().unwrap()).await;

        let result = auth_service.validate_token(&response.token);
        assert!(result.is_err());
    }
}
