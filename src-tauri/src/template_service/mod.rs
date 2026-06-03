/**
 * 模板服务模块
 * 提供模板文件系统存储和管理功能
 * 支持从网络下载模板
 */

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use reqwest;
use crate::error_handling::CircuitBreaker;
use crate::config_service::ExportConfigService;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub content: String,
    pub preview: Option<String>,
}

pub struct TemplateService {
    templates_dir: PathBuf,
    config_service: Arc<ExportConfigService>,
    circuit_breaker: CircuitBreaker,
}

impl TemplateService {
    pub fn new() -> Result<Self, String> {
        // 获取应用数据目录
        let templates_dir = Self::get_templates_directory()?;
        
        // 创建目录如果不存在
        if !templates_dir.exists() {
            fs::create_dir_all(&templates_dir)
                .map_err(|e| format!("Failed to create templates directory: {}", e))?;
        }
        
        let config_service = Arc::new(ExportConfigService::new());
        let circuit_breaker = CircuitBreaker::new(config_service.clone());
        
        Ok(Self { 
            templates_dir,
            config_service,
            circuit_breaker,
        })
    }
    
    pub fn get_templates_directory() -> Result<PathBuf, String> {
        // 获取用户文档目录
        let home_dir = dirs::home_dir()
            .ok_or("Failed to get home directory")?;
        
        let templates_dir = home_dir.join("Documents").join("LOGOS").join("templates");
        Ok(templates_dir)
    }
    
    pub fn save_template(&self, template: &Template) -> Result<(), String> {
        let template_file = self.templates_dir.join(format!("{}.json", template.id));
        
        let json = serde_json::to_string_pretty(template)
            .map_err(|e| format!("Failed to serialize template: {}", e))?;
        
        fs::write(&template_file, json)
            .map_err(|e| format!("Failed to write template file: {}", e))?;
        
        Ok(())
    }
    
    pub fn load_template(&self, id: &str) -> Result<Template, String> {
        let template_file = self.templates_dir.join(format!("{}.json", id));
        
        if !template_file.exists() {
            return Err(format!("Template with id '{}' not found", id));
        }
        
        let json = fs::read_to_string(&template_file)
            .map_err(|e| format!("Failed to read template file: {}", e))?;
        
        let template: Template = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to deserialize template: {}", e))?;
        
        Ok(template)
    }
    
    pub fn list_templates(&self) -> Result<Vec<Template>, String> {
        let mut templates = Vec::new();
        
        if !self.templates_dir.exists() {
            return Ok(templates);
        }
        
        let entries = fs::read_dir(&self.templates_dir)
            .map_err(|e| format!("Failed to read templates directory: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let json = fs::read_to_string(&path)
                    .map_err(|e| format!("Failed to read template file: {}", e))?;
                
                let template: Template = serde_json::from_str(&json)
                    .map_err(|e| format!("Failed to deserialize template: {}", e))?;
                
                templates.push(template);
            }
        }
        
        Ok(templates)
    }
    
    pub fn delete_template(&self, id: &str) -> Result<(), String> {
        let template_file = self.templates_dir.join(format!("{}.json", id));
        
        if !template_file.exists() {
            return Err(format!("Template with id '{}' not found", id));
        }
        
        fs::remove_file(&template_file)
            .map_err(|e| format!("Failed to delete template file: {}", e))?;
        
        Ok(())
    }
    
    /**
     * 从URL下载模板
     */
    pub async fn download_template_from_url(&self, url: &str) -> Result<Template, String> {
        // Check circuit breaker
        if !self.circuit_breaker.allow_operation() {
            return Err("Circuit breaker is open, blocking template downloads".to_string());
        }

        let response = reqwest::get(url)
            .await
            .map_err(|e| {
                self.circuit_breaker.record_failure();
                format!("Failed to fetch template from URL: {}", e)
            })?;
        
        if !response.status().is_success() {
            self.circuit_breaker.record_failure();
            return Err(format!("HTTP error: {}", response.status()));
        }
        
        let content = response.text()
            .await
            .map_err(|e| {
                self.circuit_breaker.record_failure();
                format!("Failed to read response body: {}", e)
            })?;
        
        // 从URL或内容中提取模板信息
        let id = url.split('/').last().unwrap_or("downloaded-template");
        let name = format!("Downloaded Template ({})", id);
        let description = format!("Template downloaded from {}", url);
        
        self.circuit_breaker.record_success();
        
        Ok(Template {
            id: id.to_string(),
            name,
            description,
            category: "custom".to_string(),
            content,
            preview: None,
        })
    }
}
