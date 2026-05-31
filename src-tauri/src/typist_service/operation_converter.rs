use super::ast_mapping::{AstMapping, AstNodeType};
use serde::{Deserialize, Serialize};

/// 视觉操作类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VisualOperation {
    /// 删除节点
    DeleteNode { node_id: String },
    /// 插入文本
    InsertText {
        node_id: String,
        position: usize,
        text: String,
    },
    /// 删除文本
    DeleteText {
        node_id: String,
        start: usize,
        end: usize,
    },
    /// 修改文本
    ReplaceText {
        node_id: String,
        start: usize,
        end: usize,
        new_text: String,
    },
    /// 调整大小
    Resize {
        node_id: String,
        new_width: f64,
        new_height: f64,
    },
    /// 移动位置
    Move {
        node_id: String,
        new_x: f64,
        new_y: f64,
    },
    /// 修改属性
    UpdateAttribute {
        node_id: String,
        attribute: String,
        value: String,
    },
    /// 插入节点
    InsertNode {
        parent_id: String,
        position: usize,
        node_type: AstNodeType,
        content: String,
    },
}

/// 代码操作类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeOperation {
    /// 删除范围
    DeleteRange { start: usize, end: usize },
    /// 插入文本
    InsertText { position: usize, text: String },
    /// 替换范围
    ReplaceRange {
        start: usize,
        end: usize,
        new_text: String,
    },
    /// 修改属性
    ModifyAttribute {
        node_id: String,
        attribute: String,
        value: String,
    },
    /// 无操作
    NoOp,
}

/// 操作转换错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversionError {
    NodeNotFound(String),
    InvalidPosition(String),
    InvalidOperation(String),
    MappingNotAvailable,
}

/// 操作转换器
pub struct OperationConverter {
    ast_mapping: AstMapping,
}

impl OperationConverter {
    pub fn new(ast_mapping: AstMapping) -> Self {
        Self { ast_mapping }
    }

    /// 将视觉操作转换为代码操作
    pub fn visual_to_code(
        &self,
        visual_op: &VisualOperation,
    ) -> Result<CodeOperation, ConversionError> {
        match visual_op {
            VisualOperation::DeleteNode { node_id } => self.convert_delete_node(node_id),
            VisualOperation::InsertText {
                node_id,
                position,
                text,
            } => self.convert_insert_text(node_id, *position, text),
            VisualOperation::DeleteText {
                node_id,
                start,
                end,
            } => self.convert_delete_text(node_id, *start, *end),
            VisualOperation::ReplaceText {
                node_id,
                start,
                end,
                new_text,
            } => self.convert_replace_text(node_id, *start, *end, new_text),
            VisualOperation::Resize {
                node_id,
                new_width,
                new_height,
            } => self.convert_resize(node_id, *new_width, *new_height),
            VisualOperation::Move {
                node_id,
                new_x,
                new_y,
            } => self.convert_move(node_id, *new_x, *new_y),
            VisualOperation::UpdateAttribute {
                node_id,
                attribute,
                value,
            } => self.convert_update_attribute(node_id, attribute, value),
            VisualOperation::InsertNode {
                parent_id,
                position,
                node_type,
                content,
            } => self.convert_insert_node(parent_id, *position, node_type, content),
        }
    }

    /// 转换删除节点操作
    fn convert_delete_node(&self, node_id: &str) -> Result<CodeOperation, ConversionError> {
        let node = self
            .ast_mapping
            .get_node(node_id)
            .ok_or_else(|| ConversionError::NodeNotFound(node_id.to_string()))?;

        Ok(CodeOperation::DeleteRange {
            start: node.source_location.start,
            end: node.source_location.end,
        })
    }

    /// 转换插入文本操作
    fn convert_insert_text(
        &self,
        node_id: &str,
        position: usize,
        text: &str,
    ) -> Result<CodeOperation, ConversionError> {
        let node = self
            .ast_mapping
            .get_node(node_id)
            .ok_or_else(|| ConversionError::NodeNotFound(node_id.to_string()))?;

        let insert_position = node.source_location.start + position;
        if insert_position > node.source_location.end {
            return Err(ConversionError::InvalidPosition(format!(
                "Position {} exceeds node end {}",
                position, node.source_location.end
            )));
        }

        Ok(CodeOperation::InsertText {
            position: insert_position,
            text: text.to_string(),
        })
    }

    /// 转换删除文本操作
    fn convert_delete_text(
        &self,
        node_id: &str,
        start: usize,
        end: usize,
    ) -> Result<CodeOperation, ConversionError> {
        let node = self
            .ast_mapping
            .get_node(node_id)
            .ok_or_else(|| ConversionError::NodeNotFound(node_id.to_string()))?;

        let delete_start = node.source_location.start + start;
        let delete_end = node.source_location.start + end;

        if delete_end > node.source_location.end {
            return Err(ConversionError::InvalidPosition(format!(
                "End position {} exceeds node end {}",
                end, node.source_location.end
            )));
        }

        Ok(CodeOperation::DeleteRange {
            start: delete_start,
            end: delete_end,
        })
    }

    /// 转换替换文本操作
    fn convert_replace_text(
        &self,
        node_id: &str,
        start: usize,
        end: usize,
        new_text: &str,
    ) -> Result<CodeOperation, ConversionError> {
        let node = self
            .ast_mapping
            .get_node(node_id)
            .ok_or_else(|| ConversionError::NodeNotFound(node_id.to_string()))?;

        let replace_start = node.source_location.start + start;
        let replace_end = node.source_location.start + end;

        if replace_end > node.source_location.end {
            return Err(ConversionError::InvalidPosition(format!(
                "End position {} exceeds node end {}",
                end, node.source_location.end
            )));
        }

        Ok(CodeOperation::ReplaceRange {
            start: replace_start,
            end: replace_end,
            new_text: new_text.to_string(),
        })
    }

    /// 转换调整大小操作
    fn convert_resize(
        &self,
        node_id: &str,
        new_width: f64,
        new_height: f64,
    ) -> Result<CodeOperation, ConversionError> {
        let node = self
            .ast_mapping
            .get_node(node_id)
            .ok_or_else(|| ConversionError::NodeNotFound(node_id.to_string()))?;

        // 根据节点类型生成相应的属性修改
        match node.node_type {
            AstNodeType::Image => Ok(CodeOperation::ModifyAttribute {
                node_id: node_id.to_string(),
                attribute: "size".to_string(),
                value: format!("({}, {})", new_width, new_height),
            }),
            AstNodeType::Table => Ok(CodeOperation::ModifyAttribute {
                node_id: node_id.to_string(),
                attribute: "width".to_string(),
                value: new_width.to_string(),
            }),
            _ => Ok(CodeOperation::ModifyAttribute {
                node_id: node_id.to_string(),
                attribute: "width".to_string(),
                value: new_width.to_string(),
            }),
        }
    }

    /// 转换移动操作
    fn convert_move(
        &self,
        node_id: &str,
        new_x: f64,
        new_y: f64,
    ) -> Result<CodeOperation, ConversionError> {
        let _node = self
            .ast_mapping
            .get_node(node_id)
            .ok_or_else(|| ConversionError::NodeNotFound(node_id.to_string()))?;

        Ok(CodeOperation::ModifyAttribute {
            node_id: node_id.to_string(),
            attribute: "position".to_string(),
            value: format!("({}, {})", new_x, new_y),
        })
    }

    /// 转换更新属性操作
    fn convert_update_attribute(
        &self,
        node_id: &str,
        attribute: &str,
        value: &str,
    ) -> Result<CodeOperation, ConversionError> {
        let _node = self
            .ast_mapping
            .get_node(node_id)
            .ok_or_else(|| ConversionError::NodeNotFound(node_id.to_string()))?;

        Ok(CodeOperation::ModifyAttribute {
            node_id: node_id.to_string(),
            attribute: attribute.to_string(),
            value: value.to_string(),
        })
    }

    /// 转换插入节点操作
    fn convert_insert_node(
        &self,
        parent_id: &str,
        position: usize,
        node_type: &AstNodeType,
        content: &str,
    ) -> Result<CodeOperation, ConversionError> {
        let parent = self
            .ast_mapping
            .get_node(parent_id)
            .ok_or_else(|| ConversionError::NodeNotFound(parent_id.to_string()))?;

        let insert_position = if position < parent.child_ids.len() {
            if let Some(child_id) = parent.child_ids.get(position) {
                if let Some(child) = self.ast_mapping.get_node(child_id) {
                    child.source_location.start
                } else {
                    parent.source_location.end
                }
            } else {
                parent.source_location.end
            }
        } else {
            parent.source_location.end
        };

        // 根据节点类型生成相应的代码
        let code = match node_type {
            AstNodeType::TableRow => format!("  row(\"{}\")", content),
            AstNodeType::TableCell => format!("    cell(\"{}\")", content),
            AstNodeType::Text => content.to_string(),
            _ => format!(
                "{}(\"{}\")",
                format!("{:?}", node_type).to_lowercase(),
                content
            ),
        };

        Ok(CodeOperation::InsertText {
            position: insert_position,
            text: code,
        })
    }

    /// 批量转换视觉操作
    pub fn batch_visual_to_code(
        &self,
        operations: &[VisualOperation],
    ) -> Vec<Result<CodeOperation, ConversionError>> {
        operations
            .iter()
            .map(|op| self.visual_to_code(op))
            .collect()
    }

    /// 更新 AST 映射
    pub fn update_mapping(&mut self, new_mapping: AstMapping) {
        self.ast_mapping = new_mapping;
    }

    /// 获取当前 AST 映射
    pub fn get_mapping(&self) -> &AstMapping {
        &self.ast_mapping
    }
}

/// 代码操作应用器
pub struct CodeOperationApplier {
    source: String,
}

impl CodeOperationApplier {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    /// 应用代码操作到源码
    pub fn apply(&mut self, operation: &CodeOperation) -> Result<String, ConversionError> {
        match operation {
            CodeOperation::DeleteRange { start, end } => self.apply_delete_range(*start, *end),
            CodeOperation::InsertText { position, text } => self.apply_insert_text(*position, text),
            CodeOperation::ReplaceRange {
                start,
                end,
                new_text,
            } => self.apply_replace_range(*start, *end, new_text),
            CodeOperation::ModifyAttribute { .. } => {
                // 属性修改需要重新解析 AST，这里返回 NoOp
                Ok(self.source.clone())
            }
            CodeOperation::NoOp => Ok(self.source.clone()),
        }
    }

    fn apply_delete_range(&mut self, start: usize, end: usize) -> Result<String, ConversionError> {
        if start > end || end > self.source.len() {
            return Err(ConversionError::InvalidPosition(format!(
                "Invalid range {}-{} for source length {}",
                start,
                end,
                self.source.len()
            )));
        }

        self.source.replace_range(start..end, "");
        Ok(self.source.clone())
    }

    fn apply_insert_text(
        &mut self,
        position: usize,
        text: &str,
    ) -> Result<String, ConversionError> {
        if position > self.source.len() {
            return Err(ConversionError::InvalidPosition(format!(
                "Position {} exceeds source length {}",
                position,
                self.source.len()
            )));
        }

        self.source.insert_str(position, text);
        Ok(self.source.clone())
    }

    fn apply_replace_range(
        &mut self,
        start: usize,
        end: usize,
        new_text: &str,
    ) -> Result<String, ConversionError> {
        if start > end || end > self.source.len() {
            return Err(ConversionError::InvalidPosition(format!(
                "Invalid range {}-{} for source length {}",
                start,
                end,
                self.source.len()
            )));
        }

        self.source.replace_range(start..end, new_text);
        Ok(self.source.clone())
    }

    /// 批量应用操作
    pub fn apply_batch(&mut self, operations: &[CodeOperation]) -> Result<String, ConversionError> {
        for operation in operations {
            self.apply(operation)?;
        }
        Ok(self.source.clone())
    }

    /// 获取当前源码
    pub fn get_source(&self) -> &str {
        &self.source
    }
}

#[cfg(test)]
mod tests {
    use super::super::ast_mapping::SourceLocation;
    use super::*;
    use typst::syntax::{FileId, Source, VirtualPath};

    fn create_test_mapping() -> AstMapping {
        let mut mapping = AstMapping::new("test".to_string());
        let source = Source::new(
            FileId::new(None, VirtualPath::new("test.typ")),
            "#table(\n  row(\"test\")\n)".to_string(),
        );

        let table_node = crate::typist_service::ast_mapping::AstNodeMapping::new(
            "table1".to_string(),
            AstNodeType::Table,
            SourceLocation::new(0, 25, &source),
        );
        mapping.add_node(table_node);

        let row_node = crate::typist_service::ast_mapping::AstNodeMapping::new(
            "row1".to_string(),
            AstNodeType::TableRow,
            SourceLocation::new(9, 22, &source),
        )
        .with_parent("table1".to_string());
        mapping.add_node(row_node);

        mapping
    }

    #[test]
    fn test_convert_delete_node() {
        let mapping = create_test_mapping();
        let converter = OperationConverter::new(mapping);

        let visual_op = VisualOperation::DeleteNode {
            node_id: "row1".to_string(),
        };
        let code_op = converter.visual_to_code(&visual_op).unwrap();

        match code_op {
            CodeOperation::DeleteRange { start, end } => {
                assert_eq!(start, 9);
                assert_eq!(end, 22);
            }
            _ => panic!("Expected DeleteRange operation"),
        }
    }

    #[test]
    fn test_convert_insert_text() {
        let mapping = create_test_mapping();
        let converter = OperationConverter::new(mapping);

        let visual_op = VisualOperation::InsertText {
            node_id: "row1".to_string(),
            position: 5,
            text: "new".to_string(),
        };
        let code_op = converter.visual_to_code(&visual_op).unwrap();

        match code_op {
            CodeOperation::InsertText { position, text } => {
                assert_eq!(position, 14);
                assert_eq!(text, "new");
            }
            _ => panic!("Expected InsertText operation"),
        }
    }

    #[test]
    fn test_convert_delete_text() {
        let mapping = create_test_mapping();
        let converter = OperationConverter::new(mapping);

        let visual_op = VisualOperation::DeleteText {
            node_id: "row1".to_string(),
            start: 0,
            end: 4,
        };
        let code_op = converter.visual_to_code(&visual_op).unwrap();

        match code_op {
            CodeOperation::DeleteRange { start, end } => {
                assert_eq!(start, 9);
                assert_eq!(end, 13);
            }
            _ => panic!("Expected DeleteRange operation"),
        }
    }

    #[test]
    fn test_convert_replace_text() {
        let mapping = create_test_mapping();
        let converter = OperationConverter::new(mapping);

        let visual_op = VisualOperation::ReplaceText {
            node_id: "row1".to_string(),
            start: 0,
            end: 4,
            new_text: "updated".to_string(),
        };
        let code_op = converter.visual_to_code(&visual_op).unwrap();

        match code_op {
            CodeOperation::ReplaceRange {
                start,
                end,
                new_text,
            } => {
                assert_eq!(start, 9);
                assert_eq!(end, 13);
                assert_eq!(new_text, "updated");
            }
            _ => panic!("Expected ReplaceRange operation"),
        }
    }

    #[test]
    fn test_convert_update_attribute() {
        let mapping = create_test_mapping();
        let converter = OperationConverter::new(mapping);

        let visual_op = VisualOperation::UpdateAttribute {
            node_id: "table1".to_string(),
            attribute: "stroke".to_string(),
            value: "1pt".to_string(),
        };
        let code_op = converter.visual_to_code(&visual_op).unwrap();

        match code_op {
            CodeOperation::ModifyAttribute {
                node_id,
                attribute,
                value,
            } => {
                assert_eq!(node_id, "table1");
                assert_eq!(attribute, "stroke");
                assert_eq!(value, "1pt");
            }
            _ => panic!("Expected ModifyAttribute operation"),
        }
    }

    #[test]
    fn test_apply_delete_range() {
        let mut applier = CodeOperationApplier::new("Hello World".to_string());
        let operation = CodeOperation::DeleteRange { start: 6, end: 11 };
        let result = applier.apply(&operation).unwrap();
        assert_eq!(result, "Hello ");
    }

    #[test]
    fn test_apply_insert_text() {
        let mut applier = CodeOperationApplier::new("Hello World".to_string());
        let operation = CodeOperation::InsertText {
            position: 6,
            text: "Beautiful ".to_string(),
        };
        let result = applier.apply(&operation).unwrap();
        assert_eq!(result, "Hello Beautiful World");
    }

    #[test]
    fn test_apply_replace_range() {
        let mut applier = CodeOperationApplier::new("Hello World".to_string());
        let operation = CodeOperation::ReplaceRange {
            start: 6,
            end: 11,
            new_text: "Rust".to_string(),
        };
        let result = applier.apply(&operation).unwrap();
        assert_eq!(result, "Hello Rust");
    }

    #[test]
    fn test_apply_batch_operations() {
        let mut applier = CodeOperationApplier::new("Hello World".to_string());
        let operations = vec![
            CodeOperation::DeleteRange { start: 6, end: 11 },
            CodeOperation::InsertText {
                position: 6,
                text: "Typst".to_string(),
            },
        ];
        let result = applier.apply_batch(&operations).unwrap();
        assert_eq!(result, "Hello Typst");
    }

    #[test]
    fn test_node_not_found_error() {
        let mapping = create_test_mapping();
        let converter = OperationConverter::new(mapping);

        let visual_op = VisualOperation::DeleteNode {
            node_id: "nonexistent".to_string(),
        };
        let result = converter.visual_to_code(&visual_op);
        assert!(result.is_err());
        match result.unwrap_err() {
            ConversionError::NodeNotFound(id) => assert_eq!(id, "nonexistent"),
            _ => panic!("Expected NodeNotFound error"),
        }
    }

    #[test]
    fn test_invalid_position_error() {
        let mapping = create_test_mapping();
        let converter = OperationConverter::new(mapping);

        let visual_op = VisualOperation::InsertText {
            node_id: "row1".to_string(),
            position: 100, // 超出范围
            text: "test".to_string(),
        };
        let result = converter.visual_to_code(&visual_op);
        assert!(result.is_err());
    }
}
