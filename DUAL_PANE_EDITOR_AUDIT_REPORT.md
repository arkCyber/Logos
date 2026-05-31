# 双栏编辑器功能审计报告

## 审计目标
审计软件是否实现了以下核心功能：
- 左边：类似Word的Tiptap富文本编辑器
- 右边：Typst实时编译的PDF预览区
- 三个核心交互纽带：状态纽带、数据纽带、视觉纽带

---

## 审计结果总结

### ✅ 已实现但未集成
三个核心交互纽带的composable已经完整实现，但**未被任何组件实际使用**。

### ⚠️ 关键问题
1. **三个composable（useStateSync、useDataSync、useVisualSync）完全未被使用**
2. **现有组件（Editor.vue、DualPaneEditor.vue）各自独立实现，没有集成三个纽带**
3. **PDF预览使用PNG而非PDF字节流，未使用pdf.js**
4. **视觉纽带中的PDF元素扫描功能是TODO**

---

## 详细审计结果

### 1. 状态纽带 ✅ 已实现但未使用

**文件**: `src/composables/useStateSync.ts`

**实现情况**:
- ✅ 实时感知Tiptap编辑器光标位置和当前状态
- ✅ 根据编辑器状态动态控制Ribbon工具栏按钮的亮灭
- ✅ 根据编辑器状态动态显示右键菜单选项
- ✅ 支持表格、列表、标题等特殊状态的检测
- ✅ 监听编辑器选择变化（selectionUpdate、transaction事件）
- ✅ 提供完整的Ribbon按钮状态计算（ribbonButtons）
- ✅ 提供完整的右键菜单项计算（contextMenuItems）

**状态检测覆盖**:
- 文本格式：粗体、斜体、下划线、删除线、代码
- 标题：H1-H6级别检测
- 列表：无序、有序、任务列表
- 引用、代码块
- 表格：是否在表格中、表头、单元格、选中范围
- 对齐：左、中、右、两端对齐
- 链接状态
- 光标位置和选择范围
- 当前节点类型

**❌ 问题**: 
- 没有任何组件import或使用这个composable
- Editor.vue有自己的状态管理，未使用useStateSync
- RibbonToolbar.vue是静态的，未与useStateSync集成

---

### 2. 数据纽带 ✅ 已实现但未使用

**文件**: `src/composables/useDataSync.ts`

**实现情况**:
- ✅ Tiptap (JSON) → 防抖(500ms-1s) → 微服务 → Typst编译 → PDF刷新
- ✅ 避免每次键盘敲击都触发编译（scheduleCompile防抖）
- ✅ 支持Ctrl+S手动编译（manualCompile）
- ✅ 调用后端微服务进行Typst编译（TODO注释，目前使用模拟数据）
- ✅ 处理编译错误和加载状态
- ✅ 重试机制（maxRetries: 3, retryDelay: 1000ms）
- ✅ 内容变化检测（避免重复编译相同内容）
- ✅ PDF URL生成和下载功能

**配置参数**:
```typescript
{
  debounceDelay: 500,        // 防抖延迟（毫秒）
  autoCompile: true,         // 是否自动编译
  manualCompileShortcut: 'Ctrl+S',
  maxRetries: 3,             // 最大重试次数
  retryDelay: 1000           // 重试延迟（毫秒）
}
```

**❌ 问题**:
- 没有任何组件import或使用这个composable
- DualPaneEditor.vue有自己的防抖实现（scheduleCompile），未使用useDataSync
- 后端编译调用是TODO，使用模拟PDF数据

---

### 3. 视觉纽带 ✅ 已实现但未使用

**文件**: `src/composables/useVisualSync.ts`

**实现情况**:
- ✅ 正向同步（左动右动）：Tiptap编辑器滚动/点击 → PDF预览区自动滚动到对应位置
- ✅ 反向同步（右动左动）：PDF预览区双击 → Tiptap编辑器自动滚动并聚焦到对应元素
- ✅ 基于ID的元素位置映射（ElementPosition接口）
- ✅ 平滑滚动动画（smoothScroll配置）
- ✅ 同步延迟控制（syncDelay: 100ms）
- ✅ 元素ID生成和扫描（scanEditorElements）
- ✅ 滚动和点击事件监听

**元素位置映射**:
```typescript
interface ElementPosition {
  id: string;
  type: 'heading' | 'paragraph' | 'table' | 'image' | 'list' | 'code';
  editorOffset: number;      // 编辑器中的偏移量
  pdfPage: number;           // PDF页码
  pdfOffset: number;         // PDF页面中的偏移量
}
```

**❌ 问题**:
- 没有任何组件import或使用这个composable
- PDF元素扫描功能是TODO（需要使用pdf.js API）
- 没有实际的双向同步滚动实现

---

### 4. Tiptap编辑器实现 ✅ 已实现

**文件**: 
- `src/components/Editor.vue` (主编辑器)
- `src/components/DualPaneEditor.vue` (双栏编辑器)
- `src/components/EditorRefactored.vue` (重构版本)

**实现情况**:
- ✅ 使用@tiptap/vue-3和@tiptap/starter-kit
- ✅ 支持富文本编辑：粗体、斜体、下划线、删除线
- ✅ 支持标题（H1-H6）
- ✅ 支持列表（无序、有序、任务列表）
- ✅ 支持表格（Table、TableRow、TableCell、TableHeader）
- ✅ 支持对齐、链接、图片等
- ✅ 有RibbonToolbar组件（但未与useStateSync集成）

**❌ 问题**:
- Editor.vue没有使用useStateSync、useDataSync、useVisualSync
- RibbonToolbar.vue是静态的，按钮状态不随编辑器状态变化
- 没有右键菜单实现

---

### 5. Typst预览实现 ⚠️ 部分实现

**后端实现**:
- ✅ 文件: `src-tauri/src/lib.rs`
- ✅ 有`compile_typst`命令（返回PNG base64）
- ✅ 有`compile_typst_slide`命令（幻灯片编译）
- ✅ 使用TypstCompiler和TypstRenderer

**前端实现**:
- ✅ 文件: `src/utils/typstConverter.ts`（HTML到Typst转换）
- ✅ 文件: `src/utils/translator.ts`（HTML到Typst翻译）
- ✅ DualPaneEditor.vue有PDF预览UI（使用iframe显示PDF URL）

**❌ 问题**:
- 后端返回PNG而非PDF字节流
- 没有使用pdf.js进行PDF渲染
- package.json中没有pdfjs-dist依赖
- 技术分析文档建议使用PDF字节流+pdf.js，但未实现

---

## 组件集成情况

### 现有组件关系

```
Editor.vue (主编辑器)
├── RibbonToolbar.vue (静态Ribbon，未集成useStateSync)
├── 其他对话框和面板
└── 独立的状态管理

DualPaneEditor.vue (双栏编辑器)
├── Tiptap编辑器（左侧）
├── PDF预览（右侧，使用iframe）
└── 独立的防抖和编译逻辑

EditorRefactored.vue (重构版本)
├── RibbonToolbar.vue
└── 未完成的重构
```

### 三个composable的使用情况

| Composable | 定义位置 | 使用情况 | 状态 |
|-----------|---------|---------|------|
| useStateSync | src/composables/useStateSync.ts | ❌ 未被任何组件使用 | 已实现但未集成 |
| useDataSync | src/composables/useDataSync.ts | ❌ 未被任何组件使用 | 已实现但未集成 |
| useVisualSync | src/composables/useVisualSync.ts | ❌ 未被任何组件使用 | 已实现但未集成 |

---

## 技术债务和缺失功能

### 高优先级
1. **集成三个composable到实际组件**
   - 在Editor.vue或DualPaneEditor.vue中使用useStateSync
   - 在DualPaneEditor.vue中使用useDataSync替换现有防抖逻辑
   - 在DualPaneEditor.vue中使用useVisualSync实现双向同步

2. **实现PDF字节流渲染**
   - 修改后端compile_typst返回PDF字节流而非PNG
   - 安装pdfjs-dist依赖
   - 使用pdf.js渲染PDF到Canvas

3. **实现PDF元素扫描**
   - 完成useVisualSync.ts中的scanPdfElements TODO
   - 使用pdf.js API获取PDF中的元素位置
   - 建立编辑器元素和PDF元素的映射关系

### 中优先级
4. **RibbonToolbar动态状态**
   - 将RibbonToolbar.vue与useStateSync集成
   - 实现按钮状态随编辑器状态变化
   - 实现表格工具标签页的动态切换

5. **右键菜单实现**
   - 使用useStateSync的contextMenuItems
   - 实现右键菜单UI组件
   - 根据编辑器状态动态显示菜单项

6. **双向同步滚动测试**
   - 测试正向同步（编辑器→PDF）
   - 测试反向同步（PDF→编辑器）
   - 优化同步性能和准确性

---

## 推荐实施方案

### 方案A：集成到DualPaneEditor.vue（推荐）

DualPaneEditor.vue已经是双栏布局，最适合集成三个纽带：

1. **导入三个composable**
```typescript
import { useStateSync } from '../composables/useStateSync';
import { useDataSync } from '../composables/useDataSync';
import { useVisualSync } from '../composables/useVisualSync';
```

2. **在setup中使用**
```typescript
const editor = useEditor({...});

// 状态纽带
const { editorState, ribbonButtons, contextMenuItems } = useStateSync(editor.value);

// 数据纽带
const { compileState, scheduleCompile, manualCompile } = useDataSync(editor.value, {
  debounceDelay: 500
});

// 视觉纽带
const { syncEditorToPdf, syncPdfToEditor } = useVisualSync(
  editor.value,
  editorContainerRef.value,
  pdfContainerRef.value
);
```

3. **修改模板**
- 使用ribbonButtons动态控制Ribbon按钮状态
- 使用contextMenuItems显示右键菜单
- 监听编辑器滚动和点击事件触发syncEditorToPdf
- 监听PDF双击事件触发syncPdfToEditor

### 方案B：集成到Editor.vue

如果需要在主编辑器中添加Typst预览面板：

1. 添加右侧预览面板
2. 集成三个composable
3. 实现PDF渲染和双向同步

---

## 依赖缺失

### 需要安装的包
```json
{
  "pdfjs-dist": "^4.0.379"
}
```

### 需要修改的后端
- `src-tauri/src/lib.rs` 的 `compile_typst` 函数
- 返回PDF字节流而非PNG base64

---

## 结论

### 功能实现度评估

| 功能 | 实现度 | 说明 |
|-----|--------|------|
| 状态纽带 | 90% | composable完整实现，但未集成到组件 |
| 数据纽带 | 85% | composable完整实现，但未集成到组件，后端编译是TODO |
| 视觉纽带 | 70% | composable框架完整，但PDF元素扫描是TODO，未集成 |
| Tiptap编辑器 | 95% | 功能完整，但未与状态纽带集成 |
| Typst预览 | 60% | 有转换和编译，但返回PNG而非PDF，未使用pdf.js |

### 总体评估

**代码质量**: ⭐⭐⭐⭐⭐
- 三个composable设计优秀，架构清晰
- 类型定义完整，注释详细
- 符合Vue 3 Composition API最佳实践

**功能完成度**: ⭐⭐⭐☆☆
- 核心功能已实现但未集成
- 缺少关键依赖（pdf.js）
- 部分功能是TODO

**可用性**: ⭐⭐☆☆☆
- 现有DualPaneEditor.vue可以运行，但功能不完整
- 没有真正的双向同步滚动
- PDF预览使用PNG而非PDF

### 建议

1. **立即行动**: 集成三个composable到DualPaneEditor.vue
2. **短期目标**: 实现PDF字节流渲染和pdf.js集成
3. **中期目标**: 完成PDF元素扫描和双向同步滚动
4. **长期目标**: 优化性能，添加更多Typst特性

---

## 审计日期
2026-05-31

## 审计人
Cascade AI Assistant
