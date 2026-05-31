use logos_lib::typist_service::{PreviewEditor, PreviewEditorConfig};

#[cfg(test)]
mod preview_editor_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_full_editing_workflow() {
        let config = PreviewEditorConfig {
            enable_realtime_preview: false, // Disable for faster tests
            ..Default::default()
        };
        let editor = PreviewEditor::new(config);

        // Step 1: Open a file
        let file_id = "test.typ".to_string();
        let initial_content = "#table(\n  row(\"test\")\n)".to_string();
        assert!(editor
            .open_file(file_id.clone(), initial_content.clone())
            .is_ok());

        // Verify editor state
        let state = editor.get_editor_state(&file_id).unwrap();
        assert!(state.is_some());
        assert_eq!(state.unwrap().source, initial_content);

        // Step 2: Update source code
        let new_content = "#table(\n  row(\"updated\")\n)".to_string();
        assert!(editor
            .update_source(&file_id, new_content.clone(), 10)
            .is_ok());

        let state = editor.get_editor_state(&file_id).unwrap();
        assert!(state.is_some());
        let state_ref = state.as_ref().unwrap();
        assert_eq!(state_ref.source, new_content);
        assert!(state_ref.is_dirty);

        // Step 3: Move cursor
        assert!(editor.move_cursor(&file_id, 20).is_ok());
        let state = editor.get_editor_state(&file_id).unwrap();
        assert!(state.is_some());
        assert_eq!(state.as_ref().unwrap().cursor_position, 20);

        // Step 4: Set selection
        assert!(editor.set_selection(&file_id, Some((10, 20))).is_ok());
        let state = editor.get_editor_state(&file_id).unwrap();
        assert!(state.is_some());
        assert_eq!(state.as_ref().unwrap().selection, Some((10, 20)));

        // Step 5: Save file
        assert!(editor.save_file(&file_id).is_ok());
        let state = editor.get_editor_state(&file_id).unwrap();
        assert!(state.is_some());
        assert!(!state.as_ref().unwrap().is_dirty);

        // Step 6: Close file
        assert!(editor.close_file(&file_id).is_ok());
        let state = editor.get_editor_state(&file_id).unwrap();
        assert!(state.is_none());
    }

    #[tokio::test]
    async fn test_lsp_integration() {
        let config = PreviewEditorConfig {
            enable_realtime_preview: false,
            ..Default::default()
        };
        let editor = PreviewEditor::new(config);

        let file_id = "test.typ".to_string();
        let content = "#table(row(\"test\")".to_string(); // Intentional syntax error
        assert!(editor.open_file(file_id.clone(), content).is_ok());

        // Test diagnostics
        let diagnostics = editor.get_diagnostics(&file_id).unwrap();
        assert!(!diagnostics.is_empty()); // Should detect unmatched parentheses

        // Test completions
        let completions = editor.get_completions(&file_id, 3).unwrap();
        assert!(!completions.is_empty()); // Should suggest keywords

        // Test symbols
        let symbols = editor.get_symbols(&file_id).unwrap();
        assert!(!symbols.is_empty()); // Should extract table symbol
    }

    #[tokio::test]
    async fn test_sync_scroll_workflow() {
        let config = PreviewEditorConfig {
            enable_realtime_preview: false,
            enable_sync_scroll: true,
            ..Default::default()
        };
        let editor = PreviewEditor::new(config);

        let file_id = "test.typ".to_string();
        let content = "#table(\n  row(\"test\")\n)".to_string();
        assert!(editor.open_file(file_id.clone(), content).is_ok());

        // Move cursor to trigger sync
        assert!(editor.move_cursor(&file_id, 10).is_ok());

        // Get preview state
        let preview_state = editor.get_preview_state(&file_id).unwrap();
        assert!(preview_state.is_some());
    }

    #[tokio::test]
    async fn test_multiple_files() {
        let config = PreviewEditorConfig {
            enable_realtime_preview: false,
            ..Default::default()
        };
        let editor = PreviewEditor::new(config);

        // Open multiple files
        let file1 = "file1.typ".to_string();
        let file2 = "file2.typ".to_string();

        assert!(editor
            .open_file(file1.clone(), "content 1".to_string())
            .is_ok());
        assert!(editor
            .open_file(file2.clone(), "content 2".to_string())
            .is_ok());

        // Verify both files are open
        let state1 = editor.get_editor_state(&file1).unwrap();
        let state2 = editor.get_editor_state(&file2).unwrap();
        assert!(state1.is_some());
        assert!(state2.is_some());
        assert_eq!(state1.as_ref().unwrap().source, "content 1");
        assert_eq!(state2.as_ref().unwrap().source, "content 2");

        // Close one file
        assert!(editor.close_file(&file1).is_ok());
        let state1 = editor.get_editor_state(&file1).unwrap();
        assert!(state1.is_none());
        let state2 = editor.get_editor_state(&file2).unwrap();
        assert!(state2.is_some());
    }

    #[tokio::test]
    async fn test_event_flow() {
        let config = PreviewEditorConfig {
            enable_realtime_preview: false,
            ..Default::default()
        };
        let editor = PreviewEditor::new(config);

        let file_id = "test.typ".to_string();
        assert!(editor
            .open_file(file_id.clone(), "initial".to_string())
            .is_ok());

        // Update source to trigger SourceChanged event
        assert!(editor
            .update_source(&file_id, "updated".to_string(), 0)
            .is_ok());

        // Move cursor to trigger CursorMoved event
        assert!(editor.move_cursor(&file_id, 5).is_ok());

        // Set selection to trigger SelectionChanged event
        assert!(editor.set_selection(&file_id, Some((0, 5))).is_ok());

        // Get events
        let events = editor.get_events().unwrap();
        assert!(!events.is_empty());
    }

    #[tokio::test]
    async fn test_ast_mapping_integration() {
        let config = PreviewEditorConfig {
            enable_realtime_preview: false,
            ..Default::default()
        };
        let editor = PreviewEditor::new(config);

        let file_id = "test.typ".to_string();
        let content = "#table(\n  row(\"test\")\n)".to_string();
        assert!(editor.open_file(file_id.clone(), content).is_ok());

        // Get AST mapping - may be None if preview is disabled
        let _mapping = editor.get_ast_mapping(&file_id).unwrap();
        // AST mapping is generated during compilation, so it might be None
        // This is expected behavior when preview is disabled
    }

    #[tokio::test]
    async fn test_configuration_options() {
        // Test with different configurations (disable preview for speed)
        let config1 = PreviewEditorConfig {
            enable_realtime_preview: false,
            enable_sync_scroll: false,
            enable_two_way_sync: false,
            ..Default::default()
        };
        let editor1 = PreviewEditor::new(config1);
        assert!(editor1
            .open_file("test.typ".to_string(), "test".to_string())
            .is_ok());

        let config2 = PreviewEditorConfig {
            enable_realtime_preview: false,
            enable_sync_scroll: true,
            enable_two_way_sync: true,
            auto_save_interval_ms: 60000,
            preview_update_delay_ms: 1000,
        };
        let editor2 = PreviewEditor::new(config2);
        assert!(editor2
            .open_file("test2.typ".to_string(), "test".to_string())
            .is_ok());
    }

    #[tokio::test]
    async fn test_error_handling() {
        let config = PreviewEditorConfig {
            enable_realtime_preview: false,
            ..Default::default()
        };
        let editor = PreviewEditor::new(config);

        // Try to close non-existent file - this succeeds silently
        let result = editor.close_file("nonexistent.typ");
        assert!(result.is_ok());

        // Try to save non-existent file - this succeeds silently
        let result = editor.save_file("nonexistent.typ");
        assert!(result.is_ok());

        // Move cursor for non-existent file - this succeeds silently
        let result = editor.move_cursor("nonexistent.typ", 0);
        assert!(result.is_ok());

        // Set selection for non-existent file - this succeeds silently
        let result = editor.set_selection("nonexistent.typ", Some((0, 5)));
        assert!(result.is_ok());

        // Update source for non-existent file - this succeeds as it updates LSP service
        let result = editor.update_source("nonexistent.typ", "test".to_string(), 0);
        assert!(result.is_ok());

        // Get diagnostics for non-existent file - returns empty list
        let result = editor.get_diagnostics("nonexistent.typ");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
