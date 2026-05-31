use super::ast_mapping::{AstMapping, AstNodeMapping, AstNodeType, SourceLocation};
use super::compiler::TypstCompiler;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use typst::syntax::{FileId, Source, VirtualPath};

/// LSP 诊断信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspDiagnostic {
    /// 诊断级别
    pub severity: DiagnosticSeverity,
    /// 起始位置
    pub start: usize,
    /// 结束位置
    pub end: usize,
    /// 错误消息
    pub message: String,
    /// 错误代码
    pub code: Option<String>,
    /// 来源
    pub source: String,
}

/// 诊断严重程度
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

/// LSP 代码补全项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspCompletionItem {
    /// 标签
    pub label: String,
    /// 类型
    pub kind: CompletionItemKind,
    /// 详情
    pub detail: Option<String>,
    /// 文档
    pub documentation: Option<String>,
    /// 插入文本
    pub insert_text: String,
}

/// 补全项类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletionItemKind {
    Function,
    Variable,
    Keyword,
    Type,
    Module,
    Operator,
    Parameter,
}

/// LSP 符号信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspSymbol {
    /// 名称
    pub name: String,
    /// 类型
    pub kind: SymbolKind,
    /// 起始位置
    pub start: usize,
    /// 结束位置
    pub end: usize,
    /// 子符号
    pub children: Vec<LspSymbol>,
}

/// 符号类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolKind {
    Function,
    Variable,
    Constant,
    Module,
    Type,
    Heading,
}

/// LSP 服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspConfig {
    /// 是否启用实时诊断
    pub enable_diagnostics: bool,
    /// 是否启用代码补全
    pub enable_completion: bool,
    /// 是否启用符号解析
    pub enable_symbols: bool,
    /// 诊断延迟（毫秒）
    pub diagnostic_delay_ms: u64,
}

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            enable_diagnostics: true,
            enable_completion: true,
            enable_symbols: true,
            diagnostic_delay_ms: 500,
        }
    }
}

/// LSP 服务
pub struct LspService {
    config: LspConfig,
    #[allow(dead_code)]
    compiler: TypstCompiler,
    source_cache: Arc<Mutex<HashMap<String, Source>>>,
    ast_mapping_cache: Arc<Mutex<HashMap<String, AstMapping>>>,
    last_update: Arc<Mutex<DateTime<Utc>>>,
}

impl LspService {
    pub fn new(config: LspConfig) -> Self {
        Self {
            config,
            compiler: TypstCompiler::new(),
            source_cache: Arc::new(Mutex::new(HashMap::new())),
            ast_mapping_cache: Arc::new(Mutex::new(HashMap::new())),
            last_update: Arc::new(Mutex::new(Utc::now())),
        }
    }

    pub fn with_compiler(config: LspConfig, compiler: TypstCompiler) -> Self {
        Self {
            config,
            compiler,
            source_cache: Arc::new(Mutex::new(HashMap::new())),
            ast_mapping_cache: Arc::new(Mutex::new(HashMap::new())),
            last_update: Arc::new(Mutex::new(Utc::now())),
        }
    }

    /// 更新源码
    pub fn update_source(&self, file_id: String, content: String) -> Result<(), String> {
        let source = Source::new(
            FileId::new(None, VirtualPath::new(&file_id)),
            content.clone(),
        );

        let mut cache = self
            .source_cache
            .lock()
            .map_err(|e| format!("Failed to lock source cache: {}", e))?;
        cache.insert(file_id.clone(), source);

        // 重新生成 AST 映射
        let mapping = self.generate_ast_mapping(&file_id, &content)?;

        let mut ast_cache = self
            .ast_mapping_cache
            .lock()
            .map_err(|e| format!("Failed to lock AST cache: {}", e))?;
        ast_cache.insert(file_id, mapping);

        let mut last_update = self
            .last_update
            .lock()
            .map_err(|e| format!("Failed to lock last update: {}", e))?;
        *last_update = Utc::now();

        Ok(())
    }

    /// 生成 AST 映射
    fn generate_ast_mapping(&self, file_id: &str, content: &str) -> Result<AstMapping, String> {
        let mut mapping = AstMapping::new(file_id.to_string());
        let source = Source::new(
            FileId::new(None, VirtualPath::new(file_id)),
            content.to_string(),
        );

        // 简化的 AST 解析 - 实际实现需要使用 Typst 的 AST
        // 这里提供基础框架
        self.parse_table_nodes(&mut mapping, content, &source)?;
        self.parse_heading_nodes(&mut mapping, content, &source)?;
        self.parse_code_blocks(&mut mapping, content, &source)?;

        Ok(mapping)
    }

    /// 解析表格节点
    fn parse_table_nodes(
        &self,
        mapping: &mut AstMapping,
        content: &str,
        source: &Source,
    ) -> Result<(), String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut current_pos = 0;

        for (line_num, line) in lines.iter().enumerate() {
            if line.trim().starts_with("#table(") {
                let start = current_pos;
                let end = current_pos + line.len();

                let node_id = format!("table_{}", line_num);
                let node = AstNodeMapping::new(
                    node_id.clone(),
                    AstNodeType::Table,
                    SourceLocation::new(start, end, source),
                );
                mapping.add_node(node);

                // 尝试解析表格行
                self.parse_table_rows(mapping, line, line_num, current_pos, source)?;
            }
            current_pos += line.len() + 1; // +1 for newline
        }

        Ok(())
    }

    /// 解析表格行
    fn parse_table_rows(
        &self,
        mapping: &mut AstMapping,
        line: &str,
        line_num: usize,
        line_start: usize,
        source: &Source,
    ) -> Result<(), String> {
        if let Some(start) = line.find("row(") {
            let row_start = line_start + start;
            let row_end =
                line_start + start + line[start..].find(')').unwrap_or(line.len() - start);

            let row_id = format!("row_{}", line_num);
            let row_node = AstNodeMapping::new(
                row_id.clone(),
                AstNodeType::TableRow,
                SourceLocation::new(row_start, row_end, source),
            );
            mapping.add_node(row_node);
        }

        Ok(())
    }

    /// 解析标题节点
    fn parse_heading_nodes(
        &self,
        mapping: &mut AstMapping,
        content: &str,
        source: &Source,
    ) -> Result<(), String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut current_pos = 0;

        for (line_num, line) in lines.iter().enumerate() {
            if line.trim().starts_with("=") {
                let start = current_pos;
                let end = current_pos + line.len();

                let node_id = format!("heading_{}", line_num);
                let node = AstNodeMapping::new(
                    node_id.clone(),
                    AstNodeType::Heading,
                    SourceLocation::new(start, end, source),
                );
                mapping.add_node(node);
            }
            current_pos += line.len() + 1;
        }

        Ok(())
    }

    /// 解析代码块
    fn parse_code_blocks(
        &self,
        mapping: &mut AstMapping,
        content: &str,
        source: &Source,
    ) -> Result<(), String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut current_pos = 0;
        let mut in_code_block = false;
        let mut code_block_start = 0;

        for (line_num, line) in lines.iter().enumerate() {
            if line.trim().starts_with("```") {
                if !in_code_block {
                    in_code_block = true;
                    code_block_start = current_pos;
                } else {
                    let end = current_pos + line.len();
                    let node_id = format!("codeblock_{}", line_num);
                    let node = AstNodeMapping::new(
                        node_id.clone(),
                        AstNodeType::CodeBlock,
                        SourceLocation::new(code_block_start, end, source),
                    );
                    mapping.add_node(node);
                    in_code_block = false;
                }
            }
            current_pos += line.len() + 1;
        }

        Ok(())
    }

    /// 获取诊断信息
    pub fn get_diagnostics(&self, file_id: &str) -> Result<Vec<LspDiagnostic>, String> {
        if !self.config.enable_diagnostics {
            return Ok(Vec::new());
        }

        let cache = self
            .source_cache
            .lock()
            .map_err(|e| format!("Failed to lock source cache: {}", e))?;

        if let Some(source) = cache.get(file_id) {
            let diagnostics = self.analyze_diagnostics(source);
            Ok(diagnostics)
        } else {
            Ok(Vec::new())
        }
    }

    /// 分析诊断信息
    fn analyze_diagnostics(&self, source: &Source) -> Vec<LspDiagnostic> {
        let mut diagnostics = Vec::new();
        let content = source.text();

        // 简化的语法检查
        let lines: Vec<&str> = content.lines().collect();
        let mut current_pos = 0;

        for (line_num, line) in lines.iter().enumerate() {
            // 检查未闭合的括号
            let open_parens = line.matches('(').count();
            let close_parens = line.matches(')').count();
            if open_parens != close_parens {
                diagnostics.push(LspDiagnostic {
                    severity: DiagnosticSeverity::Warning,
                    start: current_pos,
                    end: current_pos + line.len(),
                    message: format!("Unmatched parentheses on line {}", line_num + 1),
                    code: Some("unmatched-parens".to_string()),
                    source: "typst-lsp".to_string(),
                });
            }

            // 检查未闭合的括号
            let open_braces = line.matches('{').count();
            let close_braces = line.matches('}').count();
            if open_braces != close_braces {
                diagnostics.push(LspDiagnostic {
                    severity: DiagnosticSeverity::Warning,
                    start: current_pos,
                    end: current_pos + line.len(),
                    message: format!("Unmatched braces on line {}", line_num + 1),
                    code: Some("unmatched-braces".to_string()),
                    source: "typst-lsp".to_string(),
                });
            }

            current_pos += line.len() + 1;
        }

        diagnostics
    }

    /// 获取代码补全
    pub fn get_completions(
        &self,
        file_id: &str,
        position: usize,
    ) -> Result<Vec<LspCompletionItem>, String> {
        if !self.config.enable_completion {
            return Ok(Vec::new());
        }

        let cache = self
            .source_cache
            .lock()
            .map_err(|e| format!("Failed to lock source cache: {}", e))?;

        if let Some(source) = cache.get(file_id) {
            let completions = self.suggest_completions(source, position);
            Ok(completions)
        } else {
            Ok(Vec::new())
        }
    }

    /// 建议补全项
    fn suggest_completions(&self, source: &Source, position: usize) -> Vec<LspCompletionItem> {
        let mut completions = Vec::new();
        let content = source.text();

        // 获取当前位置的上下文
        let before_cursor = &content[..position.min(content.len())];

        // Typst 关键字补全
        let keywords = vec![
            ("let", CompletionItemKind::Keyword, "Define a variable"),
            ("set", CompletionItemKind::Keyword, "Set style rules"),
            ("show", CompletionItemKind::Keyword, "Transform content"),
            ("if", CompletionItemKind::Keyword, "Conditional"),
            ("else", CompletionItemKind::Keyword, "Else branch"),
            ("for", CompletionItemKind::Keyword, "Loop"),
            ("while", CompletionItemKind::Keyword, "While loop"),
            ("return", CompletionItemKind::Keyword, "Return value"),
            ("import", CompletionItemKind::Keyword, "Import module"),
            ("include", CompletionItemKind::Keyword, "Include file"),
            ("#table", CompletionItemKind::Function, "Create a table"),
            ("#image", CompletionItemKind::Function, "Insert an image"),
            ("#heading", CompletionItemKind::Function, "Create a heading"),
            ("#link", CompletionItemKind::Function, "Create a link"),
            (
                "#math",
                CompletionItemKind::Function,
                "Mathematical expression",
            ),
        ];

        for (keyword, kind, doc) in keywords {
            if before_cursor.ends_with(keyword.chars().take(3).collect::<String>().as_str()) {
                completions.push(LspCompletionItem {
                    label: keyword.to_string(),
                    kind: kind.clone(),
                    detail: Some(keyword.to_string()),
                    documentation: Some(doc.to_string()),
                    insert_text: keyword.to_string(),
                });
            }
        }

        completions
    }

    /// 获取符号信息
    pub fn get_symbols(&self, file_id: &str) -> Result<Vec<LspSymbol>, String> {
        if !self.config.enable_symbols {
            return Ok(Vec::new());
        }

        let cache = self
            .ast_mapping_cache
            .lock()
            .map_err(|e| format!("Failed to lock AST cache: {}", e))?;

        if let Some(mapping) = cache.get(file_id) {
            let symbols = self.extract_symbols(mapping);
            Ok(symbols)
        } else {
            Ok(Vec::new())
        }
    }

    /// 提取符号信息
    fn extract_symbols(&self, mapping: &AstMapping) -> Vec<LspSymbol> {
        let mut symbols = Vec::new();

        for node in mapping.nodes.values() {
            let kind = match node.node_type {
                AstNodeType::Heading => SymbolKind::Heading,
                AstNodeType::FunctionCall => SymbolKind::Function,
                AstNodeType::Table => SymbolKind::Type,
                _ => SymbolKind::Variable,
            };

            symbols.push(LspSymbol {
                name: node.id.clone(),
                kind,
                start: node.source_location.start,
                end: node.source_location.end,
                children: Vec::new(),
            });
        }

        symbols
    }

    /// 获取 AST 映射
    pub fn get_ast_mapping(&self, file_id: &str) -> Result<Option<AstMapping>, String> {
        let cache = self
            .ast_mapping_cache
            .lock()
            .map_err(|e| format!("Failed to lock AST cache: {}", e))?;
        Ok(cache.get(file_id).cloned())
    }

    /// 获取最后更新时间
    pub fn get_last_update(&self) -> Result<DateTime<Utc>, String> {
        let last_update = self
            .last_update
            .lock()
            .map_err(|e| format!("Failed to lock last update: {}", e))?;
        Ok(*last_update)
    }

    /// 清除缓存
    pub fn clear_cache(&self) -> Result<(), String> {
        let mut source_cache = self
            .source_cache
            .lock()
            .map_err(|e| format!("Failed to lock source cache: {}", e))?;
        source_cache.clear();

        let mut ast_cache = self
            .ast_mapping_cache
            .lock()
            .map_err(|e| format!("Failed to lock AST cache: {}", e))?;
        ast_cache.clear();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_service_creation() {
        let config = LspConfig::default();
        let service = LspService::new(config);
        assert!(service.get_last_update().is_ok());
    }

    #[test]
    fn test_update_source() {
        let config = LspConfig::default();
        let service = LspService::new(config);
        let result = service.update_source(
            "test.typ".to_string(),
            "#table(\n  row(\"test\")\n)".to_string(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_diagnostics() {
        let config = LspConfig::default();
        let service = LspService::new(config);
        service
            .update_source("test.typ".to_string(), "#table(row(\"test\")".to_string())
            .unwrap();

        let diagnostics = service.get_diagnostics("test.typ").unwrap();
        // 应该检测到未闭合的括号
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_get_completions() {
        let config = LspConfig::default();
        let service = LspService::new(config);
        service
            .update_source("test.typ".to_string(), "let".to_string())
            .unwrap();

        let completions = service.get_completions("test.typ", 3).unwrap();
        // 应该建议 "let"
        assert!(!completions.is_empty());
    }

    #[test]
    fn test_get_symbols() {
        let config = LspConfig::default();
        let service = LspService::new(config);
        service
            .update_source(
                "test.typ".to_string(),
                "#table(\n  row(\"test\")\n)".to_string(),
            )
            .unwrap();

        let symbols = service.get_symbols("test.typ").unwrap();
        // 应该包含表格符号
        assert!(!symbols.is_empty());
    }

    #[test]
    fn test_get_ast_mapping() {
        let config = LspConfig::default();
        let service = LspService::new(config);
        service
            .update_source(
                "test.typ".to_string(),
                "#table(\n  row(\"test\")\n)".to_string(),
            )
            .unwrap();

        let mapping = service.get_ast_mapping("test.typ").unwrap();
        assert!(mapping.is_some());
    }

    #[test]
    fn test_clear_cache() {
        let config = LspConfig::default();
        let service = LspService::new(config);
        service
            .update_source("test.typ".to_string(), "test".to_string())
            .unwrap();

        let result = service.clear_cache();
        assert!(result.is_ok());

        let mapping = service.get_ast_mapping("test.typ").unwrap();
        assert!(mapping.is_none());
    }

    #[test]
    fn test_lsp_config_default() {
        let config = LspConfig::default();
        assert!(config.enable_diagnostics);
        assert!(config.enable_completion);
        assert!(config.enable_symbols);
        assert_eq!(config.diagnostic_delay_ms, 500);
    }
}
