use super::ast_mapping::AstMapping;
use super::compiler::TypstCompiler;
use super::lsp_service::LspService;
use super::operation_converter::{CodeOperation, OperationConverter, VisualOperation};
use super::renderer::TypstRenderer;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 预览编辑器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewEditorConfig {
    /// 是否启用实时预览
    pub enable_realtime_preview: bool,
    /// 自动保存间隔（毫秒）
    pub auto_save_interval_ms: u64,
    /// 预览更新延迟（毫秒）
    pub preview_update_delay_ms: u64,
    /// 是否启用同步滚动
    pub enable_sync_scroll: bool,
    /// 是否启用双向同步
    pub enable_two_way_sync: bool,
}

impl Default for PreviewEditorConfig {
    fn default() -> Self {
        Self {
            enable_realtime_preview: true,
            auto_save_interval_ms: 30000,
            preview_update_delay_ms: 500,
            enable_sync_scroll: true,
            enable_two_way_sync: true,
        }
    }
}

/// 编辑器状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorState {
    /// 当前文件 ID
    pub file_id: String,
    /// 源码内容
    pub source: String,
    /// 光标位置
    pub cursor_position: usize,
    /// 选择范围
    pub selection: Option<(usize, usize)>,
    /// 最后修改时间
    pub last_modified: DateTime<Utc>,
    /// 是否已修改
    pub is_dirty: bool,
}

/// 预览状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewState {
    /// 渲染的页面数量
    pub page_count: usize,
    /// 当前页面
    pub current_page: usize,
    /// 缩放比例
    pub zoom: f64,
    /// 最后渲染时间
    pub last_rendered: DateTime<Utc>,
    /// 是否正在渲染
    pub is_rendering: bool,
}

/// 编辑器事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditorEvent {
    /// 源码变更
    SourceChanged {
        file_id: String,
        new_content: String,
        position: usize,
    },
    /// 光标移动
    CursorMoved { file_id: String, position: usize },
    /// 选择变更
    SelectionChanged {
        file_id: String,
        selection: Option<(usize, usize)>,
    },
    /// 视觉操作
    VisualOperation {
        file_id: String,
        operation: VisualOperation,
    },
    /// 预览更新
    PreviewUpdated { file_id: String, page_count: usize },
    /// 错误
    Error { file_id: String, message: String },
}

/// 实时预览编辑器
pub struct PreviewEditor {
    config: PreviewEditorConfig,
    compiler: TypstCompiler,
    #[allow(dead_code)]
    renderer: TypstRenderer,
    lsp_service: LspService,
    operation_converter: Arc<Mutex<OperationConverter>>,
    editor_states: Arc<Mutex<HashMap<String, EditorState>>>,
    preview_states: Arc<Mutex<HashMap<String, PreviewState>>>,
    ast_mappings: Arc<Mutex<HashMap<String, AstMapping>>>,
    event_channel: Arc<Mutex<Vec<EditorEvent>>>,
}

impl PreviewEditor {
    pub fn new(config: PreviewEditorConfig) -> Self {
        let lsp_config = super::lsp_service::LspConfig::default();
        let lsp_service = LspService::new(lsp_config);

        Self {
            config,
            compiler: TypstCompiler::new(),
            renderer: TypstRenderer,
            lsp_service,
            operation_converter: Arc::new(Mutex::new(OperationConverter::new(AstMapping::new(
                "".to_string(),
            )))),
            editor_states: Arc::new(Mutex::new(HashMap::new())),
            preview_states: Arc::new(Mutex::new(HashMap::new())),
            ast_mappings: Arc::new(Mutex::new(HashMap::new())),
            event_channel: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 打开文件
    pub fn open_file(&self, file_id: String, content: String) -> Result<(), String> {
        let now = Utc::now();

        // 初始化编辑器状态
        let editor_state = EditorState {
            file_id: file_id.clone(),
            source: content.clone(),
            cursor_position: 0,
            selection: None,
            last_modified: now,
            is_dirty: false,
        };

        let mut states = self
            .editor_states
            .lock()
            .map_err(|e| format!("Failed to lock editor states: {}", e))?;
        states.insert(file_id.clone(), editor_state);

        // 初始化预览状态
        let preview_state = PreviewState {
            page_count: 0,
            current_page: 0,
            zoom: 1.0,
            last_rendered: now,
            is_rendering: false,
        };

        let mut previews = self
            .preview_states
            .lock()
            .map_err(|e| format!("Failed to lock preview states: {}", e))?;
        previews.insert(file_id.clone(), preview_state);

        // 更新 LSP 服务
        self.lsp_service.update_source(file_id.clone(), content)?;

        // 初始渲染（如果启用）
        if self.config.enable_realtime_preview {
            self.render_preview(&file_id)?;
        }

        Ok(())
    }

    /// 更新源码
    pub fn update_source(
        &self,
        file_id: &str,
        new_content: String,
        position: usize,
    ) -> Result<(), String> {
        let mut states = self
            .editor_states
            .lock()
            .map_err(|e| format!("Failed to lock editor states: {}", e))?;

        if let Some(state) = states.get_mut(file_id) {
            state.source = new_content.clone();
            state.cursor_position = position;
            state.last_modified = Utc::now();
            state.is_dirty = true;
        }

        // 更新 LSP 服务
        self.lsp_service
            .update_source(file_id.to_string(), new_content.clone())?;

        // 更新 AST 映射
        if let Ok(Some(ast_mapping)) = self.lsp_service.get_ast_mapping(file_id) {
            let mut mappings = self
                .ast_mappings
                .lock()
                .map_err(|e| format!("Failed to lock AST mappings: {}", e))?;
            mappings.insert(file_id.to_string(), ast_mapping.clone());

            // 更新操作转换器
            let mut converter = self
                .operation_converter
                .lock()
                .map_err(|e| format!("Failed to lock operation converter: {}", e))?;
            *converter = OperationConverter::new(ast_mapping);
        }

        // 触发预览更新（如果启用）
        if self.config.enable_realtime_preview {
            self.render_preview(file_id)?;
        }

        // 记录事件
        self.emit_event(EditorEvent::SourceChanged {
            file_id: file_id.to_string(),
            new_content,
            position,
        });

        Ok(())
    }

    /// 应用视觉操作
    pub fn apply_visual_operation(
        &self,
        file_id: &str,
        operation: VisualOperation,
    ) -> Result<(), String> {
        if !self.config.enable_two_way_sync {
            return Err("Two-way sync is disabled".to_string());
        }

        let converter = self
            .operation_converter
            .lock()
            .map_err(|e| format!("Failed to lock operation converter: {}", e))?;

        let code_operation = converter
            .visual_to_code(&operation)
            .map_err(|e| format!("{:?}", e))?;

        // 应用代码操作到源码
        let mut states = self
            .editor_states
            .lock()
            .map_err(|e| format!("Failed to lock editor states: {}", e))?;

        if let Some(state) = states.get_mut(file_id) {
            match code_operation {
                CodeOperation::DeleteRange { start, end } => {
                    if start <= end && end <= state.source.len() {
                        state.source.replace_range(start..end, "");
                        state.is_dirty = true;
                    }
                }
                CodeOperation::InsertText { position, text } => {
                    if position <= state.source.len() {
                        state.source.insert_str(position, &text);
                        state.is_dirty = true;
                    }
                }
                CodeOperation::ReplaceRange {
                    start,
                    end,
                    new_text,
                } => {
                    if start <= end && end <= state.source.len() {
                        state.source.replace_range(start..end, &new_text);
                        state.is_dirty = true;
                    }
                }
                CodeOperation::ModifyAttribute { .. } => {
                    // 属性修改需要重新解析，这里简化处理
                    state.is_dirty = true;
                }
                CodeOperation::NoOp => {}
            }

            state.last_modified = Utc::now();
        }

        // 重新渲染预览
        self.render_preview(file_id)?;

        // 记录事件
        self.emit_event(EditorEvent::VisualOperation {
            file_id: file_id.to_string(),
            operation,
        });

        Ok(())
    }

    /// 渲染预览
    fn render_preview(&self, file_id: &str) -> Result<(), String> {
        let mut previews = self
            .preview_states
            .lock()
            .map_err(|e| format!("Failed to lock preview states: {}", e))?;

        if let Some(preview) = previews.get_mut(file_id) {
            preview.is_rendering = true;
        }

        let states = self
            .editor_states
            .lock()
            .map_err(|e| format!("Failed to lock editor states: {}", e))?;

        if let Some(state) = states.get(file_id) {
            // 使用编译器编译源码
            match self.compiler.compile(state.source.clone()) {
                Ok(document) => {
                    // 使用渲染器渲染文档
                    // 这里简化处理，实际需要调用渲染器
                    let page_count = document.pages.len();

                    if let Some(preview) = previews.get_mut(file_id) {
                        preview.page_count = page_count;
                        preview.last_rendered = Utc::now();
                        preview.is_rendering = false;
                    }

                    // 记录事件
                    self.emit_event(EditorEvent::PreviewUpdated {
                        file_id: file_id.to_string(),
                        page_count,
                    });
                }
                Err(e) => {
                    if let Some(preview) = previews.get_mut(file_id) {
                        preview.is_rendering = false;
                    }

                    self.emit_event(EditorEvent::Error {
                        file_id: file_id.to_string(),
                        message: e,
                    });
                }
            }
        }

        Ok(())
    }

    /// 移动光标
    pub fn move_cursor(&self, file_id: &str, position: usize) -> Result<(), String> {
        let mut states = self
            .editor_states
            .lock()
            .map_err(|e| format!("Failed to lock editor states: {}", e))?;

        if let Some(state) = states.get_mut(file_id) {
            state.cursor_position = position;
        }

        // 如果启用了同步滚动，更新预览位置
        if self.config.enable_sync_scroll {
            self.sync_preview_to_cursor(file_id, position)?;
        }

        self.emit_event(EditorEvent::CursorMoved {
            file_id: file_id.to_string(),
            position,
        });

        Ok(())
    }

    /// 设置选择范围
    pub fn set_selection(
        &self,
        file_id: &str,
        selection: Option<(usize, usize)>,
    ) -> Result<(), String> {
        let mut states = self
            .editor_states
            .lock()
            .map_err(|e| format!("Failed to lock editor states: {}", e))?;

        if let Some(state) = states.get_mut(file_id) {
            state.selection = selection;
        }

        self.emit_event(EditorEvent::SelectionChanged {
            file_id: file_id.to_string(),
            selection,
        });

        Ok(())
    }

    /// 同步预览到光标位置
    fn sync_preview_to_cursor(&self, file_id: &str, cursor_position: usize) -> Result<(), String> {
        let mappings = self
            .ast_mappings
            .lock()
            .map_err(|e| format!("Failed to lock AST mappings: {}", e))?;

        if let Some(mapping) = mappings.get(file_id) {
            // 查找光标位置对应的节点
            if let Some(node) = mapping.find_by_source_offset(cursor_position) {
                if let Some(visual_location) = &node.visual_location {
                    let mut previews = self
                        .preview_states
                        .lock()
                        .map_err(|e| format!("Failed to lock preview states: {}", e))?;

                    if let Some(preview) = previews.get_mut(file_id) {
                        preview.current_page = visual_location.page;
                    }
                }
            }
        }

        Ok(())
    }

    /// 同步光标到预览位置
    pub fn sync_cursor_to_preview(
        &self,
        file_id: &str,
        page: usize,
        x: f64,
        y: f64,
    ) -> Result<(), String> {
        let mappings = self
            .ast_mappings
            .lock()
            .map_err(|e| format!("Failed to lock AST mappings: {}", e))?;

        if let Some(mapping) = mappings.get(file_id) {
            // 查找预览位置对应的节点
            if let Some(node) = mapping.find_by_visual_position(page, x, y) {
                let mut states = self
                    .editor_states
                    .lock()
                    .map_err(|e| format!("Failed to lock editor states: {}", e))?;

                if let Some(state) = states.get_mut(file_id) {
                    state.cursor_position = node.source_location.start;
                }
            }
        }

        Ok(())
    }

    /// 获取编辑器状态
    pub fn get_editor_state(&self, file_id: &str) -> Result<Option<EditorState>, String> {
        let states = self
            .editor_states
            .lock()
            .map_err(|e| format!("Failed to lock editor states: {}", e))?;
        Ok(states.get(file_id).cloned())
    }

    /// 获取预览状态
    pub fn get_preview_state(&self, file_id: &str) -> Result<Option<PreviewState>, String> {
        let previews = self
            .preview_states
            .lock()
            .map_err(|e| format!("Failed to lock preview states: {}", e))?;
        Ok(previews.get(file_id).cloned())
    }

    /// 获取 AST 映射
    pub fn get_ast_mapping(&self, file_id: &str) -> Result<Option<AstMapping>, String> {
        let mappings = self
            .ast_mappings
            .lock()
            .map_err(|e| format!("Failed to lock AST mappings: {}", e))?;
        Ok(mappings.get(file_id).cloned())
    }

    /// 获取事件
    pub fn get_events(&self) -> Result<Vec<EditorEvent>, String> {
        let mut events = self
            .event_channel
            .lock()
            .map_err(|e| format!("Failed to lock event channel: {}", e))?;
        let result = events.clone();
        events.clear();
        Ok(result)
    }

    /// 发送事件
    fn emit_event(&self, event: EditorEvent) {
        let mut events = self.event_channel.lock().unwrap();
        events.push(event);
    }

    /// 保存文件
    pub fn save_file(&self, file_id: &str) -> Result<(), String> {
        let mut states = self
            .editor_states
            .lock()
            .map_err(|e| format!("Failed to lock editor states: {}", e))?;

        if let Some(state) = states.get_mut(file_id) {
            state.is_dirty = false;
        }

        Ok(())
    }

    /// 关闭文件
    pub fn close_file(&self, file_id: &str) -> Result<(), String> {
        let mut states = self
            .editor_states
            .lock()
            .map_err(|e| format!("Failed to lock editor states: {}", e))?;
        states.remove(file_id);

        let mut previews = self
            .preview_states
            .lock()
            .map_err(|e| format!("Failed to lock preview states: {}", e))?;
        previews.remove(file_id);

        let mut mappings = self
            .ast_mappings
            .lock()
            .map_err(|e| format!("Failed to lock AST mappings: {}", e))?;
        mappings.remove(file_id);

        Ok(())
    }

    /// 获取 LSP 诊断
    pub fn get_diagnostics(
        &self,
        file_id: &str,
    ) -> Result<Vec<super::lsp_service::LspDiagnostic>, String> {
        self.lsp_service.get_diagnostics(file_id)
    }

    /// 获取代码补全
    pub fn get_completions(
        &self,
        file_id: &str,
        position: usize,
    ) -> Result<Vec<super::lsp_service::LspCompletionItem>, String> {
        self.lsp_service.get_completions(file_id, position)
    }

    /// 获取符号
    pub fn get_symbols(&self, file_id: &str) -> Result<Vec<super::lsp_service::LspSymbol>, String> {
        self.lsp_service.get_symbols(file_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preview_editor_creation() {
        let config = PreviewEditorConfig::default();
        let editor = PreviewEditor::new(config);
        assert!(editor.get_events().unwrap().is_empty());
    }

    #[test]
    fn test_open_file() {
        let config = PreviewEditorConfig {
            enable_realtime_preview: false,
            ..Default::default()
        };
        let editor = PreviewEditor::new(config);
        let result = editor.open_file("test.typ".to_string(), "#table()".to_string());
        assert!(result.is_ok());

        let state = editor.get_editor_state("test.typ").unwrap();
        assert!(state.is_some());
        assert_eq!(state.unwrap().source, "#table()");
    }

    #[test]
    fn test_move_cursor() {
        let config = PreviewEditorConfig {
            enable_realtime_preview: false,
            ..Default::default()
        };
        let editor = PreviewEditor::new(config);
        editor
            .open_file("test.typ".to_string(), "test content".to_string())
            .unwrap();

        let result = editor.move_cursor("test.typ", 5);
        assert!(result.is_ok());

        let state = editor.get_editor_state("test.typ").unwrap();
        assert!(state.is_some());
        assert_eq!(state.unwrap().cursor_position, 5);
    }

    #[test]
    fn test_set_selection() {
        let config = PreviewEditorConfig {
            enable_realtime_preview: false,
            ..Default::default()
        };
        let editor = PreviewEditor::new(config);
        editor
            .open_file("test.typ".to_string(), "test content".to_string())
            .unwrap();

        let result = editor.set_selection("test.typ", Some((0, 4)));
        assert!(result.is_ok());

        let state = editor.get_editor_state("test.typ").unwrap();
        assert!(state.is_some());
        assert_eq!(state.unwrap().selection, Some((0, 4)));
    }

    #[test]
    fn test_close_file() {
        let config = PreviewEditorConfig {
            enable_realtime_preview: false,
            ..Default::default()
        };
        let editor = PreviewEditor::new(config);
        editor
            .open_file("test.typ".to_string(), "test".to_string())
            .unwrap();

        let result = editor.close_file("test.typ");
        assert!(result.is_ok());

        let state = editor.get_editor_state("test.typ").unwrap();
        assert!(state.is_none());
    }

    #[test]
    fn test_get_diagnostics() {
        let config = PreviewEditorConfig {
            enable_realtime_preview: false,
            ..Default::default()
        };
        let editor = PreviewEditor::new(config);
        editor
            .open_file("test.typ".to_string(), "#table(row(\"test\")".to_string())
            .unwrap();

        let diagnostics = editor.get_diagnostics("test.typ").unwrap();
        // Should detect unmatched parentheses
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_preview_config_default() {
        let config = PreviewEditorConfig::default();
        assert!(config.enable_realtime_preview);
        assert!(config.enable_sync_scroll);
        assert!(config.enable_two_way_sync);
        assert_eq!(config.auto_save_interval_ms, 30000);
        assert_eq!(config.preview_update_delay_ms, 500);
    }
}
