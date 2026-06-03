use std::path::Path;
use crate::error_handling::{ErrorContext, ErrorSeverity, CircuitBreaker};
use crate::config_service::ExportConfigService;
use std::sync::Arc;

pub struct FileManager {
    config_service: Arc<ExportConfigService>,
    circuit_breaker: CircuitBreaker,
}

impl FileManager {
    pub fn new() -> Self {
        let config_service = Arc::new(ExportConfigService::new());
        let circuit_breaker = CircuitBreaker::new(config_service.clone());
        Self {
            config_service,
            circuit_breaker,
        }
    }

    pub fn save_file(file_path: &str, content: &str) -> Result<(), String> {
        Self::new().save_file_impl(file_path, content)
    }

    fn save_file_impl(&self, file_path: &str, content: &str) -> Result<(), String> {
        // Check circuit breaker
        if !self.circuit_breaker.allow_operation() {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "CIRCUIT_BREAKER_OPEN",
                "Circuit breaker is open, blocking file operations",
                "file_manager",
            );
            return Err(context.message);
        }

        if file_path.is_empty() {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "EMPTY_FILE_PATH",
                "File path cannot be empty",
                "file_manager",
            );
            self.circuit_breaker.record_failure();
            return Err(context.message);
        }

        eprintln!(
            "[File Manager] Saving file: {} ({} bytes)",
            file_path,
            content.len()
        );
        let path = Path::new(file_path);

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    let context = ErrorContext::new(
                        ErrorSeverity::Error,
                        "DIRECTORY_CREATE_FAILED",
                        &format!("Failed to create directory: {}", e),
                        "file_manager",
                    );
                    eprintln!("[File Manager] Error: {}", context.message);
                    self.circuit_breaker.record_failure();
                    context.message
                })?;
            }
        }

        std::fs::write(path, content).map_err(|e| {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "FILE_WRITE_FAILED",
                &format!("Failed to write file: {}", e),
                "file_manager",
            );
            eprintln!("[File Manager] Error: {}", context.message);
            self.circuit_breaker.record_failure();
            context.message
        })?;
        eprintln!("[File Manager] File saved successfully");
        self.circuit_breaker.record_success();
        Ok(())
    }

    pub fn load_file(file_path: &str) -> Result<String, String> {
        Self::new().load_file_impl(file_path)
    }

    fn load_file_impl(&self, file_path: &str) -> Result<String, String> {
        // Check circuit breaker
        if !self.circuit_breaker.allow_operation() {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "CIRCUIT_BREAKER_OPEN",
                "Circuit breaker is open, blocking file operations",
                "file_manager",
            );
            return Err(context.message);
        }

        if file_path.is_empty() {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "EMPTY_FILE_PATH",
                "File path cannot be empty",
                "file_manager",
            );
            self.circuit_breaker.record_failure();
            return Err(context.message);
        }

        eprintln!("[File Manager] Loading file: {}", file_path);
        let path = Path::new(file_path);

        if !path.exists() {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "FILE_NOT_FOUND",
                &format!("File does not exist: {}", file_path),
                "file_manager",
            );
            eprintln!("[File Manager] Error: {}", context.message);
            self.circuit_breaker.record_failure();
            return Err(context.message);
        }

        let content = std::fs::read_to_string(path).map_err(|e| {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "FILE_READ_FAILED",
                &format!("Failed to read file: {}", e),
                "file_manager",
            );
            eprintln!("[File Manager] Error: {}", context.message);
            self.circuit_breaker.record_failure();
            context.message
        })?;
        eprintln!(
            "[File Manager] File loaded successfully ({} bytes)",
            content.len()
        );
        self.circuit_breaker.record_success();
        Ok(content)
    }

    #[allow(dead_code)]
    pub fn file_exists(file_path: &str) -> bool {
        if file_path.is_empty() {
            return false;
        }
        Path::new(file_path).exists()
    }

    #[allow(dead_code)]
    pub fn get_file_size(file_path: &str) -> Result<u64, String> {
        if file_path.is_empty() {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "EMPTY_FILE_PATH",
                "File path cannot be empty",
                "file_manager",
            );
            return Err(context.message);
        }

        let path = Path::new(file_path);
        if !path.exists() {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "FILE_NOT_FOUND",
                &format!("File does not exist: {}", file_path),
                "file_manager",
            );
            return Err(context.message);
        }

        path.metadata()
            .map(|m| m.len())
            .map_err(|e| {
                let context = ErrorContext::new(
                    ErrorSeverity::Error,
                    "METADATA_READ_FAILED",
                    &format!("Failed to get file metadata: {}", e),
                    "file_manager",
                );
                context.message
            })
    }

    #[allow(dead_code)]
    pub fn delete_file(file_path: &str) -> Result<(), String> {
        if file_path.is_empty() {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "EMPTY_FILE_PATH",
                "File path cannot be empty",
                "file_manager",
            );
            return Err(context.message);
        }

        let path = Path::new(file_path);
        if !path.exists() {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "FILE_NOT_FOUND",
                &format!("File does not exist: {}", file_path),
                "file_manager",
            );
            return Err(context.message);
        }

        std::fs::remove_file(path).map_err(|e| {
            let context = ErrorContext::new(
                ErrorSeverity::Error,
                "FILE_DELETE_FAILED",
                &format!("Failed to delete file: {}", e),
                "file_manager",
            );
            context.message
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_save_file() {
        let temp_dir = std::env::temp_dir();
        let file_binding = temp_dir.join("test_save.txt");
        let file_path = file_binding.to_str().unwrap();
        let content = "Hello, World!";

        let result = FileManager::save_file(file_path, content);
        assert!(result.is_ok());

        // Verify file was created
        assert!(Path::new(file_path).exists());

        // Clean up
        let _ = fs::remove_file(file_binding);
    }

    #[test]
    fn test_save_file_with_nested_directory() {
        let temp_dir = std::env::temp_dir();
        let nested_dir = temp_dir.join("nested").join("dir");
        let file_binding = nested_dir.join("test.txt");
        let file_path = file_binding.to_str().unwrap();
        let content = "Nested content";

        let result = FileManager::save_file(file_path, content);
        assert!(result.is_ok());

        // Verify file was created
        assert!(Path::new(file_path).exists());

        // Clean up
        let _ = fs::remove_file(file_binding);
        let _ = fs::remove_dir_all(nested_dir);
    }

    #[test]
    fn test_save_file_empty_content() {
        let temp_dir = std::env::temp_dir();
        let file_binding = temp_dir.join("test_empty.txt");
        let file_path = file_binding.to_str().unwrap();
        let content = "";

        let result = FileManager::save_file(file_path, content);
        assert!(result.is_ok());

        // Clean up
        let _ = fs::remove_file(file_binding);
    }

    #[test]
    fn test_load_file() {
        let temp_dir = std::env::temp_dir();
        let file_binding = temp_dir.join("test_load.txt");
        let file_path = file_binding.to_str().unwrap();
        let content = "Content to load";

        // First create the file
        fs::write(&file_binding, content).unwrap();

        let result = FileManager::load_file(file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);

        // Clean up
        let _ = fs::remove_file(file_binding);
    }

    #[test]
    fn test_load_file_not_found() {
        let temp_dir = std::env::temp_dir();
        let file_binding = temp_dir.join("nonexistent.txt");
        let file_path = file_binding.to_str().unwrap();

        let result = FileManager::load_file(file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_file_exists_true() {
        let temp_dir = std::env::temp_dir();
        let file_binding = temp_dir.join("test_exists.txt");
        let file_path = file_binding.to_str().unwrap();

        // Create the file
        fs::write(&file_binding, "test").unwrap();

        let exists = FileManager::file_exists(file_path);
        assert!(exists);

        // Clean up
        let _ = fs::remove_file(file_binding);
    }

    #[test]
    fn test_file_exists_false() {
        let temp_dir = std::env::temp_dir();
        let file_binding = temp_dir.join("nonexistent.txt");
        let file_path = file_binding.to_str().unwrap();

        let exists = FileManager::file_exists(file_path);
        assert!(!exists);
    }

    #[test]
    fn test_get_file_size() {
        let temp_dir = std::env::temp_dir();
        let file_binding = temp_dir.join("test_size.txt");
        let file_path = file_binding.to_str().unwrap();
        let content = "Hello, World!";

        // Create the file
        fs::write(&file_binding, content).unwrap();

        let result = FileManager::get_file_size(file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content.len() as u64);

        // Clean up
        let _ = fs::remove_file(file_binding);
    }

    #[test]
    fn test_get_file_size_not_found() {
        let temp_dir = std::env::temp_dir();
        let file_binding = temp_dir.join("nonexistent.txt");
        let file_path = file_binding.to_str().unwrap();

        let result = FileManager::get_file_size(file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_file() {
        let temp_dir = std::env::temp_dir();
        let file_binding = temp_dir.join("test_delete.txt");
        let file_path = file_binding.to_str().unwrap();

        // Create the file
        fs::write(&file_binding, "test").unwrap();
        assert!(Path::new(file_path).exists());

        let result = FileManager::delete_file(file_path);
        assert!(result.is_ok());
        assert!(!Path::new(file_path).exists());
    }

    #[test]
    fn test_delete_file_not_found() {
        let temp_dir = std::env::temp_dir();
        let file_binding = temp_dir.join("nonexistent.txt");
        let file_path = file_binding.to_str().unwrap();

        let result = FileManager::delete_file(file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let temp_dir = std::env::temp_dir();
        let file_binding = temp_dir.join("test_roundtrip.txt");
        let file_path = file_binding.to_str().unwrap();
        let content = "Roundtrip test content with special chars: <>&\"'";

        FileManager::save_file(file_path, content).unwrap();
        let loaded = FileManager::load_file(file_path).unwrap();

        assert_eq!(loaded, content);

        // Clean up
        let _ = fs::remove_file(file_binding);
    }

    #[test]
    fn test_save_file_large_content() {
        let temp_dir = std::env::temp_dir();
        let file_binding = temp_dir.join("test_large.txt");
        let file_path = file_binding.to_str().unwrap();
        let content = "x".repeat(10000);

        let result = FileManager::save_file(file_path, &content);
        assert!(result.is_ok());

        let loaded = FileManager::load_file(file_path).unwrap();
        assert_eq!(loaded.len(), 10000);

        // Clean up
        let _ = fs::remove_file(file_binding);
    }

    #[test]
    fn test_save_file_unicode() {
        let temp_dir = std::env::temp_dir();
        let file_binding = temp_dir.join("test_unicode.txt");
        let file_path = file_binding.to_str().unwrap();
        let content = "Hello 世界 🌍";

        let result = FileManager::save_file(file_path, content);
        assert!(result.is_ok());

        let loaded = FileManager::load_file(file_path).unwrap();
        assert_eq!(loaded, content);

        // Clean up
        let _ = fs::remove_file(file_binding);
    }
}
