use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub id: String,
    pub role: ConversationRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub tokens_used: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub messages: Vec<ConversationMessage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub template: String,
    pub variables: Vec<String>,
    pub category: String,
    pub is_system: bool,
}

pub struct ConversationManager {
    conversations: Arc<Mutex<HashMap<String, Conversation>>>,
    templates: Arc<Mutex<HashMap<String, PromptTemplate>>>,
    max_conversations: usize,
    max_messages_per_conversation: usize,
}

impl ConversationManager {
    pub fn new() -> Self {
        let mut templates = HashMap::new();

        // Add default system templates
        templates.insert(
            "polish".to_string(),
            PromptTemplate {
                id: "polish".to_string(),
                name: "Polish Text".to_string(),
                description: "Improve text for academic/professional tone".to_string(),
                template: "Please polish the following text to make it more professional and academic:\n\n{{text}}".to_string(),
                variables: vec!["text".to_string()],
                category: "editing".to_string(),
                is_system: true,
            },
        );

        templates.insert(
            "expand".to_string(),
            PromptTemplate {
                id: "expand".to_string(),
                name: "Expand Content".to_string(),
                description: "Add more details and context to the text".to_string(),
                template:
                    "Please expand the following text with more details and context:\n\n{{text}}"
                        .to_string(),
                variables: vec!["text".to_string()],
                category: "editing".to_string(),
                is_system: true,
            },
        );

        templates.insert(
            "rewrite".to_string(),
            PromptTemplate {
                id: "rewrite".to_string(),
                name: "Rewrite Text".to_string(),
                description: "Rewrite text while maintaining meaning".to_string(),
                template: "Please rewrite the following text while maintaining its original meaning:\n\n{{text}}".to_string(),
                variables: vec!["text".to_string()],
                category: "editing".to_string(),
                is_system: true,
            },
        );

        templates.insert(
            "summarize".to_string(),
            PromptTemplate {
                id: "summarize".to_string(),
                name: "Summarize".to_string(),
                description: "Create a concise summary of the text".to_string(),
                template: "Please provide a concise summary of the following text:\n\n{{text}}"
                    .to_string(),
                variables: vec!["text".to_string()],
                category: "editing".to_string(),
                is_system: true,
            },
        );

        templates.insert(
            "translate".to_string(),
            PromptTemplate {
                id: "translate".to_string(),
                name: "Translate to English".to_string(),
                description: "Translate text to English".to_string(),
                template: "Please translate the following text to English:\n\n{{text}}".to_string(),
                variables: vec!["text".to_string()],
                category: "translation".to_string(),
                is_system: true,
            },
        );

        Self {
            conversations: Arc::new(Mutex::new(HashMap::new())),
            templates: Arc::new(Mutex::new(templates)),
            max_conversations: 100,
            max_messages_per_conversation: 1000,
        }
    }

    /// Create a new conversation
    pub fn create_conversation(&self, title: String) -> Result<String, String> {
        let id = self.generate_id();
        let now = Utc::now();

        let conversation = Conversation {
            id: id.clone(),
            title,
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
        };

        let mut conversations = self
            .conversations
            .lock()
            .map_err(|e| format!("Failed to lock conversations: {}", e))?;

        conversations.insert(id.clone(), conversation);

        // Enforce max conversations limit
        if conversations.len() > self.max_conversations {
            let oldest = conversations.keys().next().cloned();
            if let Some(oldest_id) = oldest {
                conversations.remove(&oldest_id);
            }
        }

        Ok(id)
    }

    /// Add a message to a conversation
    pub fn add_message(
        &self,
        conversation_id: &str,
        role: ConversationRole,
        content: String,
        tokens_used: Option<u32>,
    ) -> Result<(), String> {
        let mut conversations = self
            .conversations
            .lock()
            .map_err(|e| format!("Failed to lock conversations: {}", e))?;

        let conversation = conversations
            .get_mut(conversation_id)
            .ok_or_else(|| format!("Conversation {} not found", conversation_id))?;

        let message = ConversationMessage {
            id: self.generate_id(),
            role,
            content,
            timestamp: Utc::now(),
            tokens_used,
        };

        conversation.messages.push(message);
        conversation.updated_at = Utc::now();

        // Enforce max messages limit
        if conversation.messages.len() > self.max_messages_per_conversation {
            conversation.messages.remove(0);
        }

        Ok(())
    }

    /// Get a conversation by ID
    pub fn get_conversation(&self, conversation_id: &str) -> Result<Conversation, String> {
        let conversations = self
            .conversations
            .lock()
            .map_err(|e| format!("Failed to lock conversations: {}", e))?;

        conversations
            .get(conversation_id)
            .cloned()
            .ok_or_else(|| format!("Conversation {} not found", conversation_id))
    }

    /// Get all conversations
    pub fn get_all_conversations(&self) -> Result<Vec<Conversation>, String> {
        let conversations = self
            .conversations
            .lock()
            .map_err(|e| format!("Failed to lock conversations: {}", e))?;

        let mut convs: Vec<_> = conversations.values().cloned().collect();
        convs.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(convs)
    }

    /// Delete a conversation
    pub fn delete_conversation(&self, conversation_id: &str) -> Result<(), String> {
        let mut conversations = self
            .conversations
            .lock()
            .map_err(|e| format!("Failed to lock conversations: {}", e))?;

        conversations
            .remove(conversation_id)
            .ok_or_else(|| format!("Conversation {} not found", conversation_id))?;

        Ok(())
    }

    /// Update conversation title
    #[allow(dead_code)]
    pub fn update_conversation_title(
        &self,
        conversation_id: &str,
        title: String,
    ) -> Result<(), String> {
        let mut conversations = self
            .conversations
            .lock()
            .map_err(|e| format!("Failed to lock conversations: {}", e))?;

        let conversation = conversations
            .get_mut(conversation_id)
            .ok_or_else(|| format!("Conversation {} not found", conversation_id))?;

        conversation.title = title;
        conversation.updated_at = Utc::now();

        Ok(())
    }

    /// Get conversation context for AI (last N messages)
    pub fn get_conversation_context(
        &self,
        conversation_id: &str,
        limit: usize,
    ) -> Result<Vec<ConversationMessage>, String> {
        let conversations = self
            .conversations
            .lock()
            .map_err(|e| format!("Failed to lock conversations: {}", e))?;

        let conversation = conversations
            .get(conversation_id)
            .ok_or_else(|| format!("Conversation {} not found", conversation_id))?;

        let messages = conversation
            .messages
            .iter()
            .rev()
            .take(limit)
            .cloned()
            .collect::<Vec<_>>();

        let context: Vec<_> = messages.into_iter().rev().collect();
        Ok(context)
    }

    /// Create a custom prompt template
    pub fn create_template(&self, template: PromptTemplate) -> Result<String, String> {
        let id = if template.id.is_empty() {
            self.generate_id()
        } else {
            template.id.clone()
        };

        let mut templates = self
            .templates
            .lock()
            .map_err(|e| format!("Failed to lock templates: {}", e))?;

        let new_template = PromptTemplate {
            id: id.clone(),
            ..template
        };

        templates.insert(id.clone(), new_template);
        Ok(id)
    }

    /// Get a template by ID
    pub fn get_template(&self, template_id: &str) -> Result<PromptTemplate, String> {
        let templates = self
            .templates
            .lock()
            .map_err(|e| format!("Failed to lock templates: {}", e))?;

        templates
            .get(template_id)
            .cloned()
            .ok_or_else(|| format!("Template {} not found", template_id))
    }

    /// Get all templates
    pub fn get_all_templates(&self) -> Result<Vec<PromptTemplate>, String> {
        let templates = self
            .templates
            .lock()
            .map_err(|e| format!("Failed to lock templates: {}", e))?;

        let mut temps: Vec<_> = templates.values().cloned().collect();
        temps.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(temps)
    }

    /// Get templates by category
    #[allow(dead_code)]
    pub fn get_templates_by_category(&self, category: &str) -> Result<Vec<PromptTemplate>, String> {
        let templates = self
            .templates
            .lock()
            .map_err(|e| format!("Failed to lock templates: {}", e))?;

        let temps: Vec<_> = templates
            .values()
            .filter(|t| t.category == category)
            .cloned()
            .collect();

        Ok(temps)
    }

    /// Delete a template
    pub fn delete_template(&self, template_id: &str) -> Result<(), String> {
        let mut templates = self
            .templates
            .lock()
            .map_err(|e| format!("Failed to lock templates: {}", e))?;

        let template = templates
            .get(template_id)
            .ok_or_else(|| format!("Template {} not found", template_id))?;

        if template.is_system {
            return Err("Cannot delete system templates".to_string());
        }

        templates.remove(template_id);
        Ok(())
    }

    /// Apply template with variables
    pub fn apply_template(
        &self,
        template_id: &str,
        variables: &HashMap<String, String>,
    ) -> Result<String, String> {
        let template = self.get_template(template_id)?;

        let mut result = template.template.clone();

        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        Ok(result)
    }

    /// Get conversation statistics
    pub fn get_statistics(&self) -> Result<ConversationStats, String> {
        let conversations = self
            .conversations
            .lock()
            .map_err(|e| format!("Failed to lock conversations: {}", e))?;

        let templates = self
            .templates
            .lock()
            .map_err(|e| format!("Failed to lock templates: {}", e))?;

        let total_messages: usize = conversations.values().map(|c| c.messages.len()).sum();

        let total_tokens: u32 = conversations
            .values()
            .flat_map(|c| c.messages.iter())
            .filter_map(|m| m.tokens_used)
            .sum();

        Ok(ConversationStats {
            total_conversations: conversations.len(),
            total_messages,
            total_tokens,
            total_templates: templates.len(),
            system_templates: templates.values().filter(|t| t.is_system).count(),
            custom_templates: templates.values().filter(|t| !t.is_system).count(),
        })
    }

    fn generate_id(&self) -> String {
        format!(
            "{}-{}",
            chrono::Utc::now().timestamp_millis(),
            rand::random::<u32>()
        )
    }
}

impl Default for ConversationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationStats {
    pub total_conversations: usize,
    pub total_messages: usize,
    pub total_tokens: u32,
    pub total_templates: usize,
    pub system_templates: usize,
    pub custom_templates: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_conversation() {
        let manager = ConversationManager::new();
        let id = manager.create_conversation("Test".to_string()).unwrap();
        assert!(!id.is_empty());
    }

    #[test]
    fn test_add_message() {
        let manager = ConversationManager::new();
        let id = manager.create_conversation("Test".to_string()).unwrap();
        manager
            .add_message(&id, ConversationRole::User, "Hello".to_string(), None)
            .unwrap();

        let conv = manager.get_conversation(&id).unwrap();
        assert_eq!(conv.messages.len(), 1);
    }

    #[test]
    fn test_get_template() {
        let manager = ConversationManager::new();
        let template = manager.get_template("polish").unwrap();
        assert_eq!(template.name, "Polish Text");
    }

    #[test]
    fn test_apply_template() {
        let manager = ConversationManager::new();
        let mut variables = HashMap::new();
        variables.insert("text".to_string(), "Hello world".to_string());

        let result = manager.apply_template("polish", &variables).unwrap();
        assert!(result.contains("Hello world"));
    }

    #[test]
    fn test_conversation_role_variants() {
        let user_role = ConversationRole::User;
        let assistant_role = ConversationRole::Assistant;
        let system_role = ConversationRole::System;

        // Test that all variants can be created
        let _ = (user_role, assistant_role, system_role);
    }

    #[test]
    fn test_conversation_role_serialization() {
        let role = ConversationRole::User;
        let json = serde_json::to_string(&role);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), "\"user\"");
    }

    #[test]
    fn test_conversation_role_deserialization() {
        let role: ConversationRole = serde_json::from_str("\"user\"").unwrap();
        assert!(matches!(role, ConversationRole::User));
    }

    #[test]
    fn test_conversation_message_creation() {
        let message = ConversationMessage {
            id: "msg_123".to_string(),
            role: ConversationRole::User,
            content: "Test message".to_string(),
            timestamp: Utc::now(),
            tokens_used: Some(100),
        };
        assert_eq!(message.id, "msg_123");
        assert_eq!(message.content, "Test message");
        assert_eq!(message.tokens_used, Some(100));
    }

    #[test]
    fn test_conversation_message_without_tokens() {
        let message = ConversationMessage {
            id: "msg_123".to_string(),
            role: ConversationRole::User,
            content: "Test message".to_string(),
            timestamp: Utc::now(),
            tokens_used: None,
        };
        assert!(message.tokens_used.is_none());
    }

    #[test]
    fn test_conversation_message_serialization() {
        let message = ConversationMessage {
            id: "msg_123".to_string(),
            role: ConversationRole::User,
            content: "Test message".to_string(),
            timestamp: Utc::now(),
            tokens_used: Some(100),
        };
        let json = serde_json::to_string(&message);
        assert!(json.is_ok());
    }

    #[test]
    fn test_conversation_message_deserialization() {
        let json = r#"{
            "id": "msg_123",
            "role": "user",
            "content": "Test message",
            "timestamp": "2024-01-01T00:00:00Z",
            "tokens_used": 100
        }"#;
        let message: ConversationMessage = serde_json::from_str(json).unwrap();
        assert_eq!(message.id, "msg_123");
        assert_eq!(message.content, "Test message");
    }

    #[test]
    fn test_conversation_creation() {
        let conversation = Conversation {
            id: "conv_123".to_string(),
            title: "Test Conversation".to_string(),
            messages: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        };
        assert_eq!(conversation.id, "conv_123");
        assert_eq!(conversation.title, "Test Conversation");
        assert!(conversation.messages.is_empty());
    }

    #[test]
    fn test_conversation_with_messages() {
        let messages = vec![ConversationMessage {
            id: "msg_1".to_string(),
            role: ConversationRole::User,
            content: "Hello".to_string(),
            timestamp: Utc::now(),
            tokens_used: None,
        }];
        let conversation = Conversation {
            id: "conv_123".to_string(),
            title: "Test Conversation".to_string(),
            messages,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        };
        assert_eq!(conversation.messages.len(), 1);
    }

    #[test]
    fn test_conversation_serialization() {
        let conversation = Conversation {
            id: "conv_123".to_string(),
            title: "Test Conversation".to_string(),
            messages: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        };
        let json = serde_json::to_string(&conversation);
        assert!(json.is_ok());
    }

    #[test]
    fn test_conversation_deserialization() {
        let json = r#"{
            "id": "conv_123",
            "title": "Test Conversation",
            "messages": [],
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "metadata": {}
        }"#;
        let conversation: Conversation = serde_json::from_str(json).unwrap();
        assert_eq!(conversation.id, "conv_123");
    }

    #[test]
    fn test_prompt_template_creation() {
        let template = PromptTemplate {
            id: "tpl_123".to_string(),
            name: "Test Template".to_string(),
            description: "Test description".to_string(),
            template: "Test template {{var}}".to_string(),
            variables: vec!["var".to_string()],
            category: "test".to_string(),
            is_system: false,
        };
        assert_eq!(template.id, "tpl_123");
        assert_eq!(template.name, "Test Template");
        assert!(!template.is_system);
    }

    #[test]
    fn test_prompt_template_serialization() {
        let template = PromptTemplate {
            id: "tpl_123".to_string(),
            name: "Test Template".to_string(),
            description: "Test description".to_string(),
            template: "Test template {{var}}".to_string(),
            variables: vec!["var".to_string()],
            category: "test".to_string(),
            is_system: false,
        };
        let json = serde_json::to_string(&template);
        assert!(json.is_ok());
    }

    #[test]
    fn test_prompt_template_deserialization() {
        let json = r#"{
            "id": "tpl_123",
            "name": "Test Template",
            "description": "Test description",
            "template": "Test template {{var}}",
            "variables": ["var"],
            "category": "test",
            "is_system": false
        }"#;
        let template: PromptTemplate = serde_json::from_str(json).unwrap();
        assert_eq!(template.id, "tpl_123");
    }

    #[test]
    fn test_conversation_manager_default() {
        let manager = ConversationManager::default();
        assert!(!manager.get_all_templates().unwrap().is_empty());
    }

    #[test]
    fn test_get_conversation_not_found() {
        let manager = ConversationManager::new();
        let result = manager.get_conversation("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_conversation() {
        let manager = ConversationManager::new();
        let id = manager.create_conversation("Test".to_string()).unwrap();
        manager.delete_conversation(&id).unwrap();

        let result = manager.get_conversation(&id);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_conversation_not_found() {
        let manager = ConversationManager::new();
        let result = manager.delete_conversation("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_conversation_title() {
        let manager = ConversationManager::new();
        let id = manager
            .create_conversation("Old Title".to_string())
            .unwrap();
        manager
            .update_conversation_title(&id, "New Title".to_string())
            .unwrap();

        let conv = manager.get_conversation(&id).unwrap();
        assert_eq!(conv.title, "New Title");
    }

    #[test]
    fn test_update_conversation_title_not_found() {
        let manager = ConversationManager::new();
        let result = manager.update_conversation_title("nonexistent", "New Title".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_conversations() {
        let manager = ConversationManager::new();
        manager.create_conversation("Test 1".to_string()).unwrap();
        manager.create_conversation("Test 2".to_string()).unwrap();

        let conversations = manager.get_all_conversations().unwrap();
        assert_eq!(conversations.len(), 2);
    }

    #[test]
    fn test_get_conversation_context() {
        let manager = ConversationManager::new();
        let id = manager.create_conversation("Test".to_string()).unwrap();
        manager
            .add_message(&id, ConversationRole::User, "Message 1".to_string(), None)
            .unwrap();
        manager
            .add_message(
                &id,
                ConversationRole::Assistant,
                "Response 1".to_string(),
                None,
            )
            .unwrap();
        manager
            .add_message(&id, ConversationRole::User, "Message 2".to_string(), None)
            .unwrap();

        let context = manager.get_conversation_context(&id, 2).unwrap();
        assert_eq!(context.len(), 2);
    }

    #[test]
    fn test_get_conversation_context_not_found() {
        let manager = ConversationManager::new();
        let result = manager.get_conversation_context("nonexistent", 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_template() {
        let manager = ConversationManager::new();
        let template = PromptTemplate {
            id: "".to_string(),
            name: "Custom Template".to_string(),
            description: "Custom description".to_string(),
            template: "Custom {{var}}".to_string(),
            variables: vec!["var".to_string()],
            category: "custom".to_string(),
            is_system: false,
        };
        let id = manager.create_template(template).unwrap();
        assert!(!id.is_empty());
    }

    #[test]
    fn test_create_template_with_id() {
        let manager = ConversationManager::new();
        let template = PromptTemplate {
            id: "custom_id".to_string(),
            name: "Custom Template".to_string(),
            description: "Custom description".to_string(),
            template: "Custom {{var}}".to_string(),
            variables: vec!["var".to_string()],
            category: "custom".to_string(),
            is_system: false,
        };
        let id = manager.create_template(template).unwrap();
        assert_eq!(id, "custom_id");
    }

    #[test]
    fn test_get_template_not_found() {
        let manager = ConversationManager::new();
        let result = manager.get_template("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_templates() {
        let manager = ConversationManager::new();
        let templates = manager.get_all_templates().unwrap();
        assert!(!templates.is_empty());
    }

    #[test]
    fn test_get_templates_by_category() {
        let manager = ConversationManager::new();
        let templates = manager.get_templates_by_category("editing").unwrap();
        assert!(!templates.is_empty());
    }

    #[test]
    fn test_get_templates_by_category_not_found() {
        let manager = ConversationManager::new();
        let templates = manager.get_templates_by_category("nonexistent").unwrap();
        assert!(templates.is_empty());
    }

    #[test]
    fn test_delete_template() {
        let manager = ConversationManager::new();
        let template = PromptTemplate {
            id: "".to_string(),
            name: "Custom Template".to_string(),
            description: "Custom description".to_string(),
            template: "Custom {{var}}".to_string(),
            variables: vec!["var".to_string()],
            category: "custom".to_string(),
            is_system: false,
        };
        let id = manager.create_template(template).unwrap();
        manager.delete_template(&id).unwrap();

        let result = manager.get_template(&id);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_system_template() {
        let manager = ConversationManager::new();
        let result = manager.delete_template("polish");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Cannot delete system templates"));
    }

    #[test]
    fn test_delete_template_not_found() {
        let manager = ConversationManager::new();
        let result = manager.delete_template("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_template_not_found() {
        let manager = ConversationManager::new();
        let mut variables = HashMap::new();
        variables.insert("text".to_string(), "Hello".to_string());

        let result = manager.apply_template("nonexistent", &variables);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_template_with_missing_variable() {
        let manager = ConversationManager::new();
        let mut variables = HashMap::new();
        variables.insert("other".to_string(), "Hello".to_string());

        let result = manager.apply_template("polish", &variables);
        assert!(result.is_ok()); // Should still work, just not replace the variable
    }

    #[test]
    fn test_get_statistics() {
        let manager = ConversationManager::new();
        let id = manager.create_conversation("Test".to_string()).unwrap();
        manager
            .add_message(&id, ConversationRole::User, "Hello".to_string(), Some(10))
            .unwrap();

        let stats = manager.get_statistics().unwrap();
        assert_eq!(stats.total_conversations, 1);
        assert_eq!(stats.total_messages, 1);
        assert_eq!(stats.total_tokens, 10);
        assert!(stats.total_templates > 0);
    }

    #[test]
    fn test_add_message_with_tokens() {
        let manager = ConversationManager::new();
        let id = manager.create_conversation("Test".to_string()).unwrap();
        manager
            .add_message(&id, ConversationRole::User, "Hello".to_string(), Some(100))
            .unwrap();

        let conv = manager.get_conversation(&id).unwrap();
        assert_eq!(conv.messages[0].tokens_used, Some(100));
    }

    #[test]
    fn test_add_message_to_nonexistent_conversation() {
        let manager = ConversationManager::new();
        let result = manager.add_message(
            "nonexistent",
            ConversationRole::User,
            "Hello".to_string(),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_conversation_stats_serialization() {
        let stats = ConversationStats {
            total_conversations: 10,
            total_messages: 100,
            total_tokens: 1000,
            total_templates: 5,
            system_templates: 3,
            custom_templates: 2,
        };
        let json = serde_json::to_string(&stats);
        assert!(json.is_ok());
    }

    #[test]
    fn test_conversation_stats_deserialization() {
        let json = r#"{
            "total_conversations": 10,
            "total_messages": 100,
            "total_tokens": 1000,
            "total_templates": 5,
            "system_templates": 3,
            "custom_templates": 2
        }"#;
        let stats: ConversationStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.total_conversations, 10);
    }

    #[test]
    fn test_conversation_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), "value".to_string());

        let conversation = Conversation {
            id: "conv_123".to_string(),
            title: "Test".to_string(),
            messages: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata,
        };
        assert_eq!(conversation.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_add_message_with_all_roles() {
        let manager = ConversationManager::new();
        let id = manager.create_conversation("Test".to_string()).unwrap();

        manager
            .add_message(
                &id,
                ConversationRole::User,
                "User message".to_string(),
                None,
            )
            .unwrap();
        manager
            .add_message(
                &id,
                ConversationRole::Assistant,
                "Assistant message".to_string(),
                None,
            )
            .unwrap();
        manager
            .add_message(
                &id,
                ConversationRole::System,
                "System message".to_string(),
                None,
            )
            .unwrap();

        let conv = manager.get_conversation(&id).unwrap();
        assert_eq!(conv.messages.len(), 3);
    }

    #[test]
    fn test_all_default_templates() {
        let manager = ConversationManager::new();
        let templates = manager.get_all_templates().unwrap();

        // Check that all default templates exist
        let template_ids: Vec<_> = templates.iter().map(|t| t.id.clone()).collect();
        assert!(template_ids.contains(&"polish".to_string()));
        assert!(template_ids.contains(&"expand".to_string()));
        assert!(template_ids.contains(&"rewrite".to_string()));
        assert!(template_ids.contains(&"summarize".to_string()));
        assert!(template_ids.contains(&"translate".to_string()));
    }

    #[test]
    fn test_conversation_timestamps() {
        let now = Utc::now();
        let conversation = Conversation {
            id: "conv_123".to_string(),
            title: "Test".to_string(),
            messages: vec![],
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
        };
        assert_eq!(conversation.created_at, now);
        assert_eq!(conversation.updated_at, now);
    }

    #[test]
    fn test_prompt_template_with_multiple_variables() {
        let template = PromptTemplate {
            id: "tpl_123".to_string(),
            name: "Multi Var Template".to_string(),
            description: "Test".to_string(),
            template: "{{var1}} and {{var2}}".to_string(),
            variables: vec!["var1".to_string(), "var2".to_string()],
            category: "test".to_string(),
            is_system: false,
        };
        assert_eq!(template.variables.len(), 2);
    }

    #[test]
    fn test_apply_template_multiple_variables() {
        let manager = ConversationManager::new();
        let template = PromptTemplate {
            id: "".to_string(),
            name: "Multi Var Template".to_string(),
            description: "Test".to_string(),
            template: "{{var1}} and {{var2}}".to_string(),
            variables: vec!["var1".to_string(), "var2".to_string()],
            category: "test".to_string(),
            is_system: false,
        };
        let id = manager.create_template(template).unwrap();

        let mut variables = HashMap::new();
        variables.insert("var1".to_string(), "Value 1".to_string());
        variables.insert("var2".to_string(), "Value 2".to_string());

        let result = manager.apply_template(&id, &variables).unwrap();
        assert!(result.contains("Value 1"));
        assert!(result.contains("Value 2"));
    }
}
