#[cfg(test)]
mod tests {
    use super::super::service::{AutoSaveService, SaveConfig, SaveResult};

    #[test]
    fn test_auto_save_service_creation() {
        let service = AutoSaveService::default();
        assert!(service.get_config().enabled);
    }

    #[test]
    fn test_save_document() {
        let service = AutoSaveService::default();
        let result = service.save_document("doc1", "Hello world");
        assert!(result.success);
        assert_eq!(result.version, 1);
    }

    #[test]
    fn test_get_document() {
        let service = AutoSaveService::default();
        service.save_document("doc1", "Hello world");
        let content = service.get_document("doc1");
        assert_eq!(content, Some("Hello world".to_string()));
    }

    #[test]
    fn test_should_save_debounce() {
        let config = SaveConfig {
            enabled: true,
            interval_ms: 30000,
            debounce_ms: 1000,
            max_versions: 10,
        };
        let service = AutoSaveService::new(config);
        
        assert!(service.should_save("doc1"));
        service.save_document("doc1", "Hello");
        assert!(!service.should_save("doc1")); // Should not save immediately
    }

    #[test]
    fn test_delete_document() {
        let service = AutoSaveService::default();
        service.save_document("doc1", "Hello world");
        service.delete_document("doc1");
        assert_eq!(service.get_document("doc1"), None);
    }

    #[test]
    fn test_list_documents() {
        let service = AutoSaveService::default();
        service.save_document("doc1", "Hello");
        service.save_document("doc2", "World");
        let docs = service.list_documents();
        assert_eq!(docs.len(), 2);
    }

    #[test]
    fn test_duplicate_save_no_version_increment() {
        let service = AutoSaveService::default();
        service.save_document("doc1", "Hello");
        let result1 = service.save_document("doc1", "Hello");
        let result2 = service.save_document("doc1", "Hello");
        assert_eq!(result1.version, result2.version); // Same content, no increment
    }

    #[test]
    fn test_config_update() {
        let mut service = AutoSaveService::default();
        let new_config = SaveConfig {
            enabled: false,
            interval_ms: 60000,
            debounce_ms: 5000,
            max_versions: 20,
        };
        service.update_config(new_config.clone());
        assert_eq!(service.get_config().enabled, false);
    }

    #[test]
    fn test_max_versions_cleanup() {
        let config = SaveConfig {
            enabled: true,
            interval_ms: 30000,
            debounce_ms: 1000,
            max_versions: 3,
        };
        let service = AutoSaveService::new(config);
        
        // Save more documents than max_versions
        for i in 0..5 {
            let doc_id = format!("doc{}", i);
            let content = format!("Content {}", i);
            service.save_document(&doc_id, &content);
        }
        
        let docs = service.list_documents();
        assert!(docs.len() <= 3);
    }
}
