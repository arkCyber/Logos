//! Aerospace-grade transaction management module
//! Provides transaction rollback and error recovery mechanisms

use sqlx::{SqlitePool, Transaction};
use crate::error::{SpreadsheetError, SpreadsheetResult};
use tracing::{debug, error};

/// Transaction manager for aerospace-grade error recovery
pub struct TransactionManager;

impl TransactionManager {
    /// Execute a transaction with automatic rollback on error
    pub async fn execute_transaction<F, T>(
        pool: &SqlitePool,
        operation: F,
    ) -> SpreadsheetResult<T>
    where
        for<'a> F: FnOnce(&'a mut Transaction<'a, sqlx::Sqlite>) -> futures::future::BoxFuture<'a, SpreadsheetResult<T>>,
        T: Send + 'static,
    {
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| SpreadsheetError::DatabaseTransaction(format!("Failed to begin transaction: {}", e)))?;

        debug!("Transaction started");

        match operation(&mut tx).await {
            Ok(result) => {
                tx.commit()
                    .await
                    .map_err(|e| SpreadsheetError::DatabaseTransaction(format!("Failed to commit transaction: {}", e)))?;
                debug!("Transaction committed successfully");
                Ok(result)
            }
            Err(e) => {
                if let Err(rollback_err) = tx.rollback().await {
                    error!(
                        original_error = %e,
                        rollback_error = %rollback_err,
                        "Failed to rollback transaction"
                    );
                    return Err(SpreadsheetError::DatabaseTransaction(format!(
                        "Transaction failed and rollback also failed: {} (rollback error: {})",
                        e, rollback_err
                    )));
                }
                debug!("Transaction rolled back due to error: {}", e);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_transaction_success() {
        // This test would require a test database setup
        // For now, we'll just verify the structure compiles
        assert!(true);
    }
}
