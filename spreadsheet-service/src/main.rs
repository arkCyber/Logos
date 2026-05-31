mod db;
mod models;
mod handlers;
mod services;
mod error;
mod validation;
mod config;
mod excel;
mod csrf;
mod secrets;
mod rate_limit;
mod auth;
mod conditional_formatting;
mod charts;
mod pivot_tables;
mod transaction;

pub use db::init_db_with_config;

use axum::{
    routing::{get, post, put},
    Router,
    response::Json,
    extract::State,
};
use serde_json::json;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber;
use crate::config::AppConfig;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize secrets manager
    let secrets_manager = secrets::init_secrets_manager()
        .map_err(|e| {
            error!("Failed to initialize secrets manager: {}", e);
            e
        })?;

    // Load configuration
    let config = AppConfig::load()
        .map_err(|e| {
            error!("Failed to load configuration: {}", e);
            e
        })?;

    // Use secrets manager for JWT secret if available
    let _jwt_secret = secrets_manager.get_or("jwt_secret", &config.security.jwt_secret);
    
    // Initialize tracing with configuration
    let log_level = config.logging.level.parse::<tracing::Level>()
        .unwrap_or(tracing::Level::INFO);
    
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(log_level.into())
        )
        .init();

    info!("Starting spreadsheet service...");

    // Initialize database with configuration
    let pool = db::init_db_with_config(&config).await
        .map_err(|e| {
            error!("Failed to initialize database: {}", e);
            e
        })?;

    // Build our application with routes
    let app = create_app(pool.clone());

    // Get server address from configuration
    let addr = config.server_address()?;
    let listener = TcpListener::bind(addr).await?;
    info!("Spreadsheet service listening on {}", addr);

    // Handle graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Spreadsheet service shutdown complete");
    Ok(())
}

/// Create application router (used for testing)
pub fn create_app(pool: SqlitePool) -> Router {
    let csrf_config = csrf::CsrfConfig::from_env();
    let csrf_protection = csrf::CsrfProtection::new(csrf_config);
    let rate_limiter = rate_limit::RateLimiter::from_env();

    Router::new()
        .route("/", get(health_check))
        .route("/api/health", get(health_check))
        .route("/api/health/detailed", get(detailed_health_check))
        .route("/api/auth/register", post(handlers::register))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/sheets", get(handlers::list_sheets).post(handlers::create_sheet))
        .route("/api/sheets/:id", get(handlers::get_sheet).put(handlers::update_sheet).delete(handlers::delete_sheet))
        .route("/api/sheets/:id/cells", get(handlers::list_cells).post(handlers::create_cell))
        .route("/api/sheets/:id/cells/batch", post(handlers::batch_create_cells))
        .route("/api/sheets/:id/cells/:row/:col", get(handlers::get_cell).put(handlers::update_cell).delete(handlers::delete_cell))
        .route("/api/sheets/:id/cells/batch/update", put(handlers::batch_update_cells))
        .route("/api/sheets/:id/cells/batch/delete", post(handlers::batch_delete_cells))
        .route("/api/sheets/:id/formula", post(handlers::calculate_formula))
        .route("/api/files/import", post(handlers::import_excel))
        .route("/api/files/export/:id", get(handlers::export_excel))
        // Conditional Formatting Routes
        .route("/api/sheets/:sheet_id/conditional-formats", get(handlers::list_conditional_formats).post(handlers::create_conditional_format))
        .route("/api/conditional-formats/:id", put(handlers::update_conditional_format).delete(handlers::delete_conditional_format))
        // Chart Routes
        .route("/api/sheets/:sheet_id/charts", get(handlers::list_charts).post(handlers::create_chart))
        .route("/api/charts/:id", put(handlers::update_chart).delete(handlers::delete_chart))
        // Pivot Table Routes
        .route("/api/sheets/:sheet_id/pivot-tables", get(handlers::list_pivot_tables).post(handlers::create_pivot_table))
        .route("/api/pivot-tables/:id", put(handlers::update_pivot_table).delete(handlers::delete_pivot_table))
        .layer(axum::middleware::from_fn_with_state(
            csrf_protection.clone(),
            csrf::csrf_middleware,
        ))
        .layer(axum::middleware::from_fn_with_state(
            rate_limiter.clone(),
            rate_limit::rate_limit_middleware,
        ))
        .with_state(pool)
        .with_state(csrf_protection)
        .with_state(rate_limiter)
}

/// Signal handler for graceful shutdown
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal");
        },
        _ = terminate => {
            info!("Received terminate signal");
        },
    }
}

async fn health_check(State(pool): State<SqlitePool>) -> Json<serde_json::Value> {
    // Check database health
    let db_healthy = sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await
        .is_ok();

    Json(json!({
        "status": if db_healthy { "healthy" } else { "degraded" },
        "service": "spreadsheet-service",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "checks": {
            "database": {
                "status": if db_healthy { "up" } else { "down" }
            }
        }
    }))
}

async fn detailed_health_check(State(pool): State<SqlitePool>) -> Json<serde_json::Value> {
    // Check database health with detailed info
    let db_healthy = sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await
        .is_ok();
    
    // Get connection pool stats
    let pool_size = pool.size();
    
    // Get sheet count
    let sheet_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sheets")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);
    
    // Get cell count
    let cell_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM cells")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    Json(json!({
        "status": if db_healthy { "healthy" } else { "degraded" },
        "service": "spreadsheet-service",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "checks": {
            "database": {
                "status": if db_healthy { "up" } else { "down" },
                "pool_size": pool_size,
                "active_connections": pool_size,
                "idle_connections": 0
            },
            "data": {
                "sheets": sheet_count,
                "cells": cell_count
            }
        },
        "uptime": {
            "seconds": 0
        }
    }))
}
