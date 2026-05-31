# Editor.vue 重构计划

## 当前状态
- 原始 Editor.vue: 8210 行，功能完整
- 重构版本: 功能不完整，E2E 测试失败

## 问题分析
完全重构会导致功能缺失，需要采用渐进式拆分策略。

## 渐进式拆分计划

### 阶段 1: 提取样式和常量（低风险）
- 提取 CSS 到独立文件
- 提取常量配置
- 保持原有逻辑不变

### 阶段 2: 提取纯展示组件（中风险）
- QuickAccessToolbar（已完成）
- StatusBar（已完成）
- RibbonToolbar（已完成，但需要集成）
- 逐步替换，保持功能完整

### 阶段 3: 提取业务逻辑到 Composables（高风险）
- useEditorState（已完成）
- useDocumentOperations（已完成）
- 逐步迁移，保持向后兼容

### 阶段 4: 拆分对话框组件（高风险）
- 搜索对话框
- 数学公式对话框
- 表格样式对话框
- 等等...

### 阶段 5: 最终重构
- 完全替换主文件
- 全面测试

## 当前成果
✅ 创建了组件目录结构
✅ 拆分了 QuickAccessToolbar 组件
✅ 拆分了 StatusBar 组件  
✅ 拆分了 RibbonToolbar 组件（基础版本）
✅ 创建了 useEditorState composable
✅ 创建了 useDocumentOperations composable
✅ 构建测试通过
❌ E2E 测试失败（功能不完整）

## 下一步行动
1. 保持原始 Editor.vue 作为主文件
2. 将拆分的组件作为可选的优化模块
3. 逐步集成，每次集成后测试
4. 创建集成测试确保功能完整性
