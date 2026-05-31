# Typst双栏编辑器技术方案分析

**日期**: 2026-05-31  
**分析目标**: 评估右侧预览区使用PDF字节流（pdf.js渲染）vs SVG格式的技术选择

---

## 执行摘要

基于用户需求的双栏编辑器架构（左侧Tiptap富文本编辑器 + 右侧Typst实时预览），本文档详细分析PDF和SVG两种预览方案的技术优劣，并推荐最优技术路径。

### 核心需求回顾

**三个核心交互纽带**:
1. **状态纽带**: Tiptap驱动Ribbon和右键菜单
2. **数据纽带**: 单向内容流 + 防抖动机制（500ms~1s）
3. **视觉纽带**: 基于ID的双向同步滚动（SyncTeX平替版）

---

## 技术方案对比

### 方案A: PDF字节流 + pdf.js渲染

#### 技术架构

```
Tiptap (JSON) → 防抖(500ms) → 微服务 → Typst编译 → PDF字节流 → pdf.js渲染 → Canvas显示
```

#### 优势

1. **渲染质量**
   - Typst原生输出PDF，无中间转换损失
   - 保持Typst的工业级排版精度
   - 支持所有Typst特性（字体、布局、图形）

2. **性能**
   - pdf.js成熟稳定，Mozilla维护
   - 支持虚拟化渲染（只渲染可见页面）
   - 支持缓存机制

3. **功能完整性**
   - 支持PDF原生功能（缩放、旋转、打印）
   - 支持文本选择和复制
   - 支持PDF内部链接跳转

4. **SyncTeX支持**
   - Typst原生支持SyncTeX
   - 可实现精确的位置同步
   - 双向滚动体验最佳

#### 劣势

1. **性能开销**
   - PDF解析和渲染较慢
   - 大型文档内存占用高
   - 首次渲染有明显延迟

2. **技术复杂度**
   - 需要集成pdf.js库
   - 需要处理跨域和CORS
   - Canvas渲染需要额外处理

3. **文件大小**
   - PDF文件通常比SVG大
   - 网络传输时间较长
   - 需要优化压缩

4. **交互限制**
   - Canvas无法直接操作DOM
   - 文本选择体验不如HTML
   - 无障碍支持较弱

#### 实现示例

```typescript
// PDF预览组件
import * as pdfjsLib from 'pdfjs-dist';

class TypstPdfPreview {
  private pdfDoc: pdfjsLib.PDFDocumentProxy | null = null;
  private pageRenderingQueue: Map<number, Promise<void>> = new Map();

  async loadPdf(pdfData: Uint8Array) {
    const loadingTask = pdfjsLib.getDocument({ data: pdfData });
    this.pdfDoc = await loadingTask.promise;
  }

  async renderPage(pageNum: number, canvas: HTMLCanvasElement) {
    if (!this.pdfDoc) return;

    const page = await this.pdfDoc.getPage(pageNum);
    const viewport = page.getViewport({ scale: 1.5 });

    const context = canvas.getContext('2d');
    if (!context) return;

    canvas.height = viewport.height;
    canvas.width = viewport.width;

    const renderContext = {
      canvasContext: context,
      viewport: viewport
    };

    await page.render(renderContext).promise;
  }

  // SyncTeX集成
  async scrollToPosition(syncTeXData: SyncTeXData) {
    const page = await this.pdfDoc?.getPage(syncTeXData.page);
    // 实现精确滚动
  }
}
```

---

### 方案B: SVG格式直接插入

#### 技术架构

```
Tiptap (JSON) → 防抖(500ms) → 微服务 → Typst编译 → SVG → 直接插入DOM
```

#### 优势

1. **渲染性能**
   - SVG是原生DOM元素，渲染速度快
   - 浏览器原生支持，无需额外库
   - 内存占用相对较小

2. **交互体验**
   - 可直接操作DOM（CSS样式、事件监听）
   - 文本选择体验接近HTML
   - 无障碍支持良好

3. **开发复杂度**
   - 无需额外渲染库
   - CSS样式控制简单
   - 调试方便

4. **文件大小**
   - SVG文件通常较小
   - 可压缩传输
   - 支持增量更新

#### 劣势

1. **渲染质量**
   - Typst不直接输出SVG，需要转换
   - 转换可能损失精度
   - 某些Typst特性可能不支持

2. **功能限制**
   - 缺少SyncTeX原生支持
   - 双向同步需要自定义实现
   - 复杂布局可能渲染不正确

3. **性能问题**
   - 大型SVG可能导致DOM卡顿
   - 缺少虚拟化支持
   - 内存占用随文档大小线性增长

4. **兼容性**
   - Typst到SVG转换不成熟
   - 需要维护转换逻辑
   - 可能存在渲染差异

#### 实现示例

```typescript
// SVG预览组件
class TypstSvgPreview {
  private svgContainer: HTMLElement;

  async loadSvg(svgString: string) {
    this.svgContainer.innerHTML = svgString;
    
    // 添加ID用于同步
    this.addSyncIds();
  }

  private addSyncIds() {
    // 为每个元素添加唯一ID
    const elements = this.svgContainer.querySelectorAll('*');
    elements.forEach((el, index) => {
      el.setAttribute('data-sync-id', `sync-${index}`);
    });
  }

  scrollToElement(syncId: string) {
    const element = this.svgContainer.querySelector(`[data-sync-id="${syncId}"]`);
    if (element) {
      element.scrollIntoView({ behavior: 'smooth' });
    }
  }
}
```

---

## 技术方案推荐

### 推荐方案: PDF字节流 + pdf.js渲染

**推荐理由**:

1. **质量优先**: Typst的核心价值是工业级排版，PDF原生输出能完整保留这一优势
2. **SyncTeX支持**: Typst原生支持SyncTeX，双向同步体验最佳
3. **成熟稳定**: pdf.js经过多年发展，稳定性和性能有保障
4. **未来兼容**: PDF是行业标准，长期维护支持

### 实施策略

#### 阶段1: MVP实现（2-3周）

**目标**: 基础双栏编辑器 + PDF预览

```typescript
// 核心组件结构
<template>
  <div class="dual-pane-editor">
    <!-- 左侧：Tiptap编辑器 -->
    <div class="editor-pane">
      <RibbonToolbar :active-nodes="activeNodes" />
      <TiptapEditor 
        @update="handleEditorUpdate"
        @selection-change="handleSelectionChange"
      />
    </div>
    
    <!-- 右侧：PDF预览 -->
    <div class="preview-pane">
      <PdfPreview 
        :pdf-data="pdfData"
        @page-click="handlePdfClick"
      />
    </div>
  </div>
</template>
```

**关键实现**:
1. Tiptap编辑器集成
2. pdf.js预览组件
3. 防抖机制（500ms）
4. 基础单向同步

#### 阶段2: 状态纽带（1-2周）

**目标**: Ribbon和右键菜单联动

```typescript
// 状态管理
interface EditorState {
  activeNodes: Set<string>;
  currentBlock: Block | null;
  selection: Selection | null;
}

// Ribbon工具栏
class RibbonToolbar {
  updateToolbar(state: EditorState) {
    // 根据activeNodes显示/隐藏按钮
    if (state.activeNodes.has('table')) {
      this.showTableTools();
    }
  }
}

// 右键菜单
class ContextMenu {
  showMenu(state: EditorState, position: Point) {
    if (state.activeNodes.has('table-cell')) {
      this.showTableMenu(position);
    }
  }
}
```

#### 阶段3: 数据纽带（1周）

**目标**: 防抖 + 微服务集成

```typescript
// 防抖服务
class DebounceService {
  private debounceTimer: NodeJS.Timeout | null = null;
  
  update(content: string) {
    clearTimeout(this.debounceTimer);
    this.debounceTimer = setTimeout(() => {
      this.sendToService(content);
    }, 500);
  }
  
  private async sendToService(content: string) {
    const json = this.tiptapToJSON(content);
    const pdfData = await invoke('compile_typst', { json });
    this.updatePreview(pdfData);
  }
}
```

#### 阶段4: 视觉纽带（2-3周）

**目标**: 双向同步滚动（SyncTeX）

```typescript
// SyncTeX集成
interface SyncTeXData {
  page: number;
  x: number;
  y: number;
  nodeId: string;
}

// 正向同步（左→右）
class ForwardSync {
  scrollToPdf(nodeId: string) {
    const syncData = this.getSyncTeXData(nodeId);
    this.pdfPreview.scrollToPage(syncData.page);
    this.pdfPreview.scrollToPosition(syncData.x, syncData.y);
  }
}

// 反向同步（右→左）
class ReverseSync {
  focusEditor(syncData: SyncTeXData) {
    const node = this.findNodeById(syncData.nodeId);
    this.tiptapEditor.focusNode(node);
    this.tiptapEditor.scrollToNode(node);
  }
}
```

---

## 性能优化策略

### 1. 预览缓存

```typescript
class PreviewCache {
  private cache: Map<string, Uint8Array> = new Map();
  
  get(contentHash: string): Uint8Array | undefined {
    return this.cache.get(contentHash);
  }
  
  set(contentHash: string, pdfData: Uint8Array) {
    this.cache.set(contentHash, pdfData);
  }
}
```

### 2. 虚拟化渲染

```typescript
// 只渲染可见页面
class VirtualizedPdfPreview {
  private visiblePages: Set<number> = new Set();
  
  updateVisiblePages(scrollTop: number) {
    const startPage = Math.floor(scrollTop / this.pageHeight);
    const endPage = startPage + this.visiblePageCount;
    
    // 只渲染可见范围内的页面
    for (let i = startPage; i <= endPage; i++) {
      if (!this.visiblePages.has(i)) {
        this.renderPage(i);
        this.visiblePages.add(i);
      }
    }
  }
}
```

### 3. 增量编译

```typescript
// 只编译修改的部分
class IncrementalCompiler {
  private lastCompiled: string = '';
  
  async compileIncremental(content: string) {
    const diff = this.computeDiff(this.lastCompiled, content);
    if (diff.isEmpty()) {
      return this.lastPdf;
    }
    
    const pdf = await this.compile(diff);
    this.lastCompiled = content;
    return pdf;
  }
}
```

---

## 技术栈建议

### 前端

```json
{
  "dependencies": {
    "@tiptap/vue-3": "^3.23.6",
    "@tiptap/starter-kit": "^3.23.6",
    "@tiptap/extension-table": "^3.23.6",
    "pdfjs-dist": "^4.0.379",
    "vue": "^3.5.13",
    "pinia": "^2.1.7"
  }
}
```

### 后端（Rust + Tauri）

```rust
// typst_service.rs
#[tauri::command]
async fn compile_typst_to_pdf(json: String) -> Result<Vec<u8>, String> {
    let content = convert_json_to_typst(json)?;
    let pdf = typst::compile(&content)?;
    Ok(pdf)
}

#[tauri::command]
async fn get_synctex_data(content: String) -> Result<SyncTeXData, String> {
    let data = typst::get_synctex(&content)?;
    Ok(data)
}
```

---

## 风险评估

### 技术风险

1. **pdf.js性能**
   - 风险：大型文档渲染慢
   - 缓解：虚拟化渲染 + 缓存

2. **SyncTeX精度**
   - 风险：同步位置不准确
   - 缓解：使用Typst原生SyncTeX

3. **防抖延迟**
   - 风险：用户感觉延迟
   - 缓解：显示"编译中"状态 + Ctrl+S强制刷新

### 依赖风险

1. **Typst版本**
   - 风险：API变化
   - 缓解：使用稳定版本 + 版本锁定

2. **pdf.js兼容性**
   - 风险：浏览器兼容性
   - 缓解：polyfill + 降级方案

---

## 实施时间表

### 阶段1: MVP（2-3周）
- Tiptap编辑器集成
- pdf.js预览组件
- 基础防抖机制

### 阶段2: 状态纽带（1-2周）
- Ribbon工具栏
- 右键菜单
- 状态感知

### 阶段3: 数据纽带（1周）
- 微服务集成
- 优化防抖
- 错误处理

### 阶段4: 视觉纽带（2-3周）
- SyncTeX集成
- 双向同步
- 性能优化

### 阶段5: 完善优化（1-2周）
- 性能调优
- 用户体验改进
- 测试和文档

**总计**: 7-11周完成完整双栏编辑器

---

## 结论

### 最终推荐

**使用PDF字节流 + pdf.js渲染方案**

**核心理由**:
1. Typst原生输出PDF，保证排版质量
2. SyncTeX原生支持，双向同步体验最佳
3. pdf.js成熟稳定，长期维护
4. 符合"工业级、无可挑剔的排版视觉"目标

### 关键成功因素

1. **防抖机制**: 500ms~1s延迟，避免卡顿
2. **状态感知**: 实时感知光标位置，驱动UI
3. **双向同步**: SyncTeX实现精确位置同步
4. **性能优化**: 虚拟化渲染 + 缓存机制

### 预期效果

- 左侧：傻瓜式、可视化的富文本编辑
- 右侧：工业级、无可挑剔的Typst排版
- 体验：接近Office的高水准双栏联动
