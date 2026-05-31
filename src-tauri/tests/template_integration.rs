#[cfg(test)]
mod integration_tests {
    use chrono::Utc;
    use logos_lib::typist_service::template::TemplateEngine;
    use logos_lib::typist_service::template::{
        Template, TemplateCategory, TemplateMetadata, TemplateVariable,
    };
    use std::collections::HashMap;

    fn create_test_template(name: &str, content: &str) -> Template {
        Template {
            name: name.to_string(),
            description: "Test template".to_string(),
            content: content.to_string(),
            variables: vec![],
            metadata: TemplateMetadata {
                category: TemplateCategory::Custom,
                author: None,
                version: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                thumbnail: None,
                previous_versions: Vec::new(),
            },
        }
    }

    #[test]
    fn test_full_template_lifecycle() {
        let mut engine = TemplateEngine::new();

        // 1. Register template
        let template = create_test_template("lifecycle-test", "Initial content");
        assert!(engine.register_template(template).is_ok());

        // 2. Get template
        let retrieved = engine.get_template("lifecycle-test");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().content, "Initial content");

        // 3. Update template with version change
        let updated = Template {
            name: "lifecycle-test".to_string(),
            description: "Updated description".to_string(),
            content: "Updated content".to_string(),
            variables: vec![],
            metadata: TemplateMetadata {
                category: TemplateCategory::Custom,
                author: None,
                version: Some("2.0.0".to_string()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                thumbnail: None,
                previous_versions: Vec::new(),
            },
        };
        assert!(engine.register_template(updated).is_ok());

        // 4. Check version history
        let final_template = engine.get_template("lifecycle-test").unwrap();
        assert_eq!(final_template.metadata.version, Some("2.0.0".to_string()));
        assert_eq!(final_template.metadata.previous_versions.len(), 1);

        // 5. Render template
        let values = HashMap::new();
        let rendered = engine.render("lifecycle-test", &values);
        assert!(rendered.is_ok());
        assert_eq!(rendered.unwrap(), "Updated content");

        // 6. Export template
        let exported = engine.export_template(
            "lifecycle-test",
            logos_lib::typist_service::template::TemplateExportFormat::Json,
        );
        assert!(exported.is_ok());
        assert!(exported.unwrap().contains("lifecycle-test"));

        // 7. Remove template
        assert!(engine.remove_template("lifecycle-test"));
        assert!(engine.get_template("lifecycle-test").is_none());
    }

    #[test]
    fn test_template_search_and_filter() {
        let mut engine = TemplateEngine::new();

        // Register multiple templates
        engine
            .register_template(create_test_template("academic-paper", "Academic content"))
            .unwrap();
        engine
            .register_template(create_test_template("business-report", "Business content"))
            .unwrap();
        engine
            .register_template(create_test_template("technical-doc", "Technical content"))
            .unwrap();

        // Set categories
        let mut academic = engine.get_template("academic-paper").unwrap().clone();
        academic.metadata.category = TemplateCategory::Academic;
        engine.register_template(academic).unwrap();

        let mut business = engine.get_template("business-report").unwrap().clone();
        business.metadata.category = TemplateCategory::Business;
        engine.register_template(business).unwrap();

        // Test category filter
        let academic_templates = engine.get_templates_by_category(TemplateCategory::Academic);
        assert_eq!(academic_templates.len(), 1);
        assert_eq!(academic_templates[0].name, "academic-paper");

        // Test search
        let search_results = engine.search_templates("business");
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].name, "business-report");

        // Test search with multiple matches
        let doc_results = engine.search_templates("doc");
        assert!(doc_results.len() >= 1);
    }

    #[test]
    fn test_template_rendering_with_variables() {
        let mut engine = TemplateEngine::new();

        let template = Template {
            name: "var-test".to_string(),
            description: "Template with variables".to_string(),
            content: "Hello {{name}}, your score is {{score}}".to_string(),
            variables: vec![
                TemplateVariable {
                    name: "name".to_string(),
                    default_value: Some("World".to_string()),
                    description: Some("User name".to_string()),
                    required: false, // Changed to false to allow rendering with defaults
                },
                TemplateVariable {
                    name: "score".to_string(),
                    default_value: Some("100".to_string()),
                    description: Some("Score value".to_string()),
                    required: false,
                },
            ],
            metadata: TemplateMetadata {
                category: TemplateCategory::Custom,
                author: None,
                version: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                thumbnail: None,
                previous_versions: Vec::new(),
            },
        };

        engine.register_template(template).unwrap();

        // Render with custom values
        let mut values = HashMap::new();
        values.insert("name".to_string(), "Alice".to_string());
        values.insert("score".to_string(), "95".to_string());

        let rendered = engine.render("var-test", &values).unwrap();
        assert_eq!(rendered, "Hello Alice, your score is 95");

        // Render with defaults
        let empty_values = HashMap::new();
        let rendered_defaults = engine.render("var-test", &empty_values).unwrap();
        assert_eq!(rendered_defaults, "Hello World, your score is 100");
    }

    #[test]
    fn test_metadata_update_workflow() {
        let mut engine = TemplateEngine::new();

        let template = create_test_template("meta-test", "Content");
        engine.register_template(template).unwrap();

        // Update metadata
        let updates = logos_lib::typist_service::template::TemplateMetadataUpdate {
            category: Some(TemplateCategory::Academic),
            author: Some("Test Author".to_string()),
            version: Some("1.0.0".to_string()),
            thumbnail: Some("thumb.png".to_string()),
        };

        assert!(engine
            .update_template_metadata("meta-test", updates)
            .is_ok());

        let updated = engine.get_template("meta-test").unwrap();
        assert_eq!(updated.metadata.category, TemplateCategory::Academic);
        assert_eq!(updated.metadata.author, Some("Test Author".to_string()));
        assert_eq!(updated.metadata.version, Some("1.0.0".to_string()));
        assert_eq!(updated.metadata.thumbnail, Some("thumb.png".to_string()));
    }

    #[test]
    fn test_export_import_roundtrip() {
        let mut engine = TemplateEngine::new();

        let template = create_test_template("export-test", "Export content");
        engine.register_template(template).unwrap();

        // Export to JSON
        let exported = engine
            .export_template(
                "export-test",
                logos_lib::typist_service::template::TemplateExportFormat::Json,
            )
            .unwrap();

        // Remove original
        engine.remove_template("export-test");

        // Import from JSON
        let imported_name = engine
            .import_template(
                &exported,
                logos_lib::typist_service::template::TemplateExportFormat::Json,
            )
            .unwrap();

        assert_eq!(imported_name, "export-test");

        let imported = engine.get_template("export-test").unwrap();
        assert_eq!(imported.content, "Export content");
    }

    #[test]
    fn test_cache_invalidation_workflow() {
        let mut engine = TemplateEngine::new();

        let template = Template {
            name: "cache-test".to_string(),
            description: "Test".to_string(),
            content: "Hello {{name}}".to_string(),
            variables: vec![TemplateVariable {
                name: "name".to_string(),
                default_value: Some("World".to_string()),
                description: None,
                required: false,
            }],
            metadata: TemplateMetadata {
                category: TemplateCategory::Custom,
                author: None,
                version: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                thumbnail: None,
                previous_versions: Vec::new(),
            },
        };

        engine.register_template(template).unwrap();

        // Generate preview (should cache)
        engine.generate_template_preview("cache-test").unwrap();
        assert_eq!(engine.preview_cache_size(), 1);

        // Update template (should invalidate cache)
        let updated = Template {
            name: "cache-test".to_string(),
            description: "Test".to_string(),
            content: "Updated content".to_string(),
            variables: vec![],
            metadata: TemplateMetadata {
                category: TemplateCategory::Custom,
                author: None,
                version: Some("2.0.0".to_string()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                thumbnail: None,
                previous_versions: Vec::new(),
            },
        };
        engine.register_template(updated).unwrap();
        assert_eq!(engine.preview_cache_size(), 0);

        // Verify version history
        let final_template = engine.get_template("cache-test").unwrap();
        assert_eq!(final_template.metadata.previous_versions.len(), 1);
    }
}
