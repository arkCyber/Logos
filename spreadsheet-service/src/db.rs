use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use crate::config::AppConfig;
use crate::error::{SpreadsheetError, SpreadsheetResult};
use tracing::info;
use std::time::Duration;
use crate::auth;

/// Initialize database with aerospace-grade connection management
pub async fn init_db_with_config(config: &AppConfig) -> SpreadsheetResult<SqlitePool> {
    // Create database directory if it doesn't exist
    std::fs::create_dir_all("data")
        .map_err(|e| SpreadsheetError::DatabaseConnection(format!("Failed to create data directory: {}", e)))?;

    // Get current directory
    let current_dir = std::env::current_dir()
        .map_err(|e| SpreadsheetError::DatabaseConnection(format!("Failed to get current directory: {}", e)))?;

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| format!("sqlite://{}/data/spreadsheet.db?mode=rwc", current_dir.display()));

    info!(
        url = %database_url,
        max_connections = config.database.max_connections,
        min_connections = config.database.min_connections,
        "Initializing database connection pool"
    );

    // Configure connection pool with aerospace-grade settings
    let pool = SqlitePoolOptions::new()
        .max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .acquire_timeout(Duration::from_secs(config.database.connection_timeout))
        .idle_timeout(Duration::from_secs(config.database.idle_timeout))
        .max_lifetime(Duration::from_secs(config.database.max_lifetime))
        .test_before_acquire(true) // Test connections before use
        .connect(&database_url)
        .await
        .map_err(|e| SpreadsheetError::DatabaseConnection(format!("Failed to connect to database: {}", e)))?;

    // Run migrations with error handling
    run_migrations(&pool).await?;

    info!("Database initialized successfully");
    Ok(pool)
}

/// Run database migrations
async fn run_migrations(pool: &SqlitePool) -> SpreadsheetResult<()> {
    info!("Running database migrations");

    // Create users table
    auth::init_users_table(pool).await?;

    // Create sheets table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sheets (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create sheets table: {}", e)))?;

    // Create cells table with proper constraints
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS cells (
            id TEXT PRIMARY KEY,
            sheet_id TEXT NOT NULL,
            row INTEGER NOT NULL,
            col INTEGER NOT NULL,
            value TEXT,
            formula TEXT,
            style TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (sheet_id) REFERENCES sheets(id) ON DELETE CASCADE,
            UNIQUE(sheet_id, row, col)
        )
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create cells table: {}", e)))?;

    // Create indexes for performance
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_cells_sheet_id ON cells(sheet_id)
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create cells index: {}", e)))?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_cells_coordinates ON cells(sheet_id, row, col)
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create coordinates index: {}", e)))?;

    // Create conditional_formatting_rules table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS conditional_formatting_rules (
            id TEXT PRIMARY KEY,
            sheet_id TEXT NOT NULL,
            range TEXT NOT NULL,
            rule_type TEXT NOT NULL,
            rule_data TEXT NOT NULL,
            format_data TEXT NOT NULL,
            priority INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            FOREIGN KEY (sheet_id) REFERENCES sheets(id) ON DELETE CASCADE
        )
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create conditional_formatting_rules table: {}", e)))?;

    // Create charts table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS charts (
            id TEXT PRIMARY KEY,
            sheet_id TEXT NOT NULL,
            name TEXT NOT NULL,
            chart_type TEXT NOT NULL,
            data_range TEXT NOT NULL,
            title TEXT,
            x_axis_title TEXT,
            y_axis_title TEXT,
            legend_position TEXT,
            style_data TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (sheet_id) REFERENCES sheets(id) ON DELETE CASCADE
        )
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create charts table: {}", e)))?;

    // Create pivot_tables table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS pivot_tables (
            id TEXT PRIMARY KEY,
            sheet_id TEXT NOT NULL,
            name TEXT NOT NULL,
            source_range TEXT NOT NULL,
            row_fields TEXT NOT NULL,
            column_fields TEXT NOT NULL,
            value_fields TEXT NOT NULL,
            filter_fields TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (sheet_id) REFERENCES sheets(id) ON DELETE CASCADE
        )
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create pivot_tables table: {}", e)))?;

    // Create indexes for new tables
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_conditional_formatting_sheet_id ON conditional_formatting_rules(sheet_id)
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create conditional_formatting index: {}", e)))?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_charts_sheet_id ON charts(sheet_id)
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create charts index: {}", e)))?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_pivot_tables_sheet_id ON pivot_tables(sheet_id)
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| SpreadsheetError::DatabaseQuery(format!("Failed to create pivot_tables index: {}", e)))?;

    info!("Database migrations completed successfully");
    Ok(())
}

/// Health check for database connection
pub async fn health_check(pool: &SqlitePool) -> SpreadsheetResult<()> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map_err(|e| SpreadsheetError::DatabaseConnection(format!("Database health check failed: {}", e)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_initialization() {
        let config = AppConfig::default();
        let pool = init_db_with_config(&config).await;
        assert!(pool.is_ok());
    }

    #[tokio::test]
    async fn test_health_check() {
        let config = AppConfig::default();
        let pool = init_db_with_config(&config).await.unwrap();
        let health = health_check(&pool).await;
        assert!(health.is_ok());
    }
}
