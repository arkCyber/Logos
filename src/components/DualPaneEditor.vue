<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue';
import { useEditor, EditorContent } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import { TextStyle } from '@tiptap/extension-text-style';
import { FontFamily } from '@tiptap/extension-font-family';
import { Underline } from '@tiptap/extension-underline';
import { Strike } from '@tiptap/extension-strike';
import { Subscript } from '@tiptap/extension-subscript';
import { Superscript } from '@tiptap/extension-superscript';
import { Table } from '@tiptap/extension-table';
import { TableRow } from '@tiptap/extension-table-row';
import { TableCell } from '@tiptap/extension-table-cell';
import { TableHeader } from '@tiptap/extension-table-header';
import { invoke } from '@tauri-apps/api/core';
import { logger, LogCategory } from '../utils/logger';
import { htmlToTypst } from '../utils/translator';
import { useStateSync } from '../composables/useStateSync';
import { useDataSync } from '../composables/useDataSync';
import { useVisualSync } from '../composables/useVisualSync';
// import PdfViewer from './PdfViewer.vue'; // 暂时禁用
import { 
  Bold, 
  Italic, 
  Underline as UnderlineIcon, 
  Strikethrough, 
  Heading1, 
  Heading2, 
  Heading3, 
  List, 
  ListOrdered, 
  Quote, 
  Table as TableIcon, 
  Plus, 
  Minus, 
  Undo, 
  Redo, 
  Play, 
  Loader2, 
  ZoomIn, 
  ZoomOut, 
  RotateCw, 
  RotateCcw, 
  Maximize2, 
  Minimize2 
} from 'lucide-vue-next';

interface DualPaneEditorProps {
  modelValue?: string;
  theme?: 'light' | 'dark';
  autoCompile?: boolean;
  compileDelay?: number;
}

const props = withDefaults(defineProps<DualPaneEditorProps>(), {
  modelValue: '',
  theme: 'light',
  autoCompile: true,
  compileDelay: 500
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
  'compile': [htmlContent: string];
  'compiled': [pdfData: Uint8Array];
  'error': [error: Error];
}>();

// PDF预览状态
const pdfData = ref<string | null>(null);
const pdfScale = ref(1.0);
const pdfRotation = ref(0);
const currentPage = ref(1);
const totalPages = ref(1);

// 容器引用（用于视觉同步）
const editorContainerRef = ref<HTMLDivElement | null>(null);
const pdfContainerRef = ref<HTMLDivElement | null>(null);

// Tiptap编辑器
const editor = useEditor({
  content: props.modelValue || '<p>开始输入...</p>',
  extensions: [
    StarterKit, // StarterKit already includes Underline and Strike
    TextStyle,
    FontFamily,
    Subscript,
    Superscript,
    Table.configure({
      resizable: true
    }),
    TableRow,
    TableHeader,
    TableCell
  ],
  onUpdate: ({ editor }) => {
    const html = editor.getHTML();
    emit('update:modelValue', html);
    
    if (props.autoCompile) {
      handleEditorUpdate();
    }
  },
  onCreate: ({ editor }) => {
    console.log('[DualPaneEditor] Editor created successfully');
  },
  editorProps: {
    attributes: {
      class: 'tiptap-editor'
    }
  }
});

// 集成状态纽带（useStateSync）
const {
  editorState,
  ribbonButtons,
  contextMenuItems,
  showContextMenu,
  contextMenuPosition,
  handleContextMenu,
  hideContextMenu,
  executeContextMenuItem
} = useStateSync(editor.value || null);

// 集成数据纽带（useDataSync）
const {
  compileState,
  scheduleCompile,
  manualCompile,
  getPdfUrl,
  downloadPdf
} = useDataSync(editor.value || null, {
  debounceDelay: props.compileDelay,
  autoCompile: props.autoCompile,
  manualCompileShortcut: 'Ctrl+S',
  maxRetries: 3,
  retryDelay: 1000
});

// 集成视觉纽带（useVisualSync）
const {
  syncState,
  elementMap,
  syncEditorToPdf,
  syncPdfToEditor,
  updateElementMap,
  enableSync,
  disableSync
} = useVisualSync(
  editor.value || null,
  editorContainerRef.value,
  pdfContainerRef.value,
  {
    enabled: false, // 暂时禁用，避免初始化问题
    syncDelay: 100,
    smoothScroll: true,
    scrollDuration: 300,
    syncOnScroll: true,
    syncOnClick: true
  }
);

// 监听编译状态变化
watch(() => compileState.value.pdfData, (newPdfData) => {
  // 清理旧的Blob URL
  if (pdfData.value && pdfData.value.startsWith('blob:')) {
    URL.revokeObjectURL(pdfData.value);
  }
  
  if (newPdfData && newPdfData.length > 0) {
    // 转换为base64 URL供PdfViewer使用
    const blob = new Blob([newPdfData as unknown as ArrayBuffer], { type: 'application/pdf' });
    pdfData.value = URL.createObjectURL(blob);
  } else {
    pdfData.value = null;
  }
});

// 监听编译错误
watch(() => compileState.value.compileError, (error) => {
  if (error) {
    logger.error('Compilation error', { error }, LogCategory.BUSINESS);
  }
});

// 编译Typst（调用后端服务）
const compileTypst = async () => {
  if (!editor.value) {
    return;
  }

  const html = editor.value.getHTML();
  if (!html || !html.trim()) {
    pdfData.value = null;
    return;
  }

  try {
    logger.info('Compiling HTML to Typst', { contentLength: html.length }, LogCategory.BUSINESS);
    
    // 转换HTML为Typst
    const typstCode = htmlToTypst(html);
    
    // 检查是否在Tauri环境中
    if (typeof window !== 'undefined' && (window as any).__TAURI__) {
      // 调用后端编译服务
      const pdfBase64 = await invoke<string>('compile_typst', { code: typstCode });
      logger.info('Typst compilation completed', { typstLength: typstCode.length }, LogCategory.BUSINESS);
      pdfData.value = pdfBase64;
    } else {
      // 非Tauri环境，使用模拟数据
      logger.warn('Not in Tauri environment, using mock PDF data', LogCategory.BUSINESS);
      const mockPdf = new Uint8Array([0x25, 0x50, 0x44, 0x46]); // PDF magic number
      const blob = new Blob([mockPdf], { type: 'application/pdf' });
      pdfData.value = URL.createObjectURL(blob);
    }
  } catch (error) {
    logger.error('Failed to compile Typst', error, LogCategory.BUSINESS);
    pdfData.value = null;
  }
};

// 监听props变化
watch(() => props.modelValue, (newValue) => {
  if (editor.value && newValue !== editor.value.getHTML()) {
    editor.value.commands.setContent(newValue);
  }
});

// 防抖编译（使用useDataSync的scheduleCompile）
const handleEditorUpdate = () => {
  if (props.autoCompile) {
    scheduleCompile();
  }
};

// 手动编译
const triggerManualCompile = () => {
  manualCompile();
  compileTypst();
};
const setBold = () => editor.value?.chain().focus().toggleBold().run();
const setItalic = () => editor.value?.chain().focus().toggleItalic().run();
const setUnderline = () => editor.value?.chain().focus().toggleUnderline().run();
const setStrike = () => editor.value?.chain().focus().toggleStrike().run();
const setHeading = (level: 1 | 2 | 3) => editor.value?.chain().focus().toggleHeading({ level }).run();
const setBulletList = () => editor.value?.chain().focus().toggleBulletList().run();
const setOrderedList = () => editor.value?.chain().focus().toggleOrderedList().run();
const setBlockquote = () => editor.value?.chain().focus().toggleBlockquote().run();
const insertTable = () => {
  editor.value?.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run();
};
const addRow = () => editor.value?.chain().focus().addRowAfter().run();
const deleteRow = () => editor.value?.chain().focus().deleteRow().run();
const addColumn = () => editor.value?.chain().focus().addColumnAfter().run();
const deleteColumn = () => editor.value?.chain().focus().deleteColumn().run();
const undo = () => editor.value?.chain().focus().undo().run();
const redo = () => editor.value?.chain().focus().redo().run();

// PDF控制（暂时禁用）
// const pdfViewerRef = ref<InstanceType<typeof PdfViewer> | null>(null);

const zoomIn = () => { /* pdfViewerRef.value?.zoomIn(); */ };
const zoomOut = () => { /* pdfViewerRef.value?.zoomOut(); */ };
const rotateClockwise = () => { /* pdfViewerRef.value?.rotateClockwise(); */ };
const rotateCounterClockwise = () => { /* pdfViewerRef.value?.rotateCounterClockwise(); */ };
const fitToWidth = () => { /* pdfViewerRef.value?.setScale(1.0); */ };
const fitToPage = () => { /* pdfViewerRef.value?.setScale(1.0); */ };

// 处理PDF元素点击（暂时禁用）
const handlePdfElementClicked = (elementId: string, position: { page: number; x: number; y: number }) => {
  logger.debug('PDF element clicked', { elementId, position }, LogCategory.UI);
  // syncPdfToEditor(elementId);
};

// 处理PDF页面变化（暂时禁用）
const handlePdfPageChanged = (pageNumber: number, total: number) => {
  currentPage.value = pageNumber;
  totalPages.value = total;
};

// 生命周期
onMounted(async () => {
  logger.info('Dual pane editor mounted', { theme: props.theme }, LogCategory.UI);
  
  // 初始化编辑器监听
  if (editor.value) {
    editor.value.on('update', handleEditorUpdate);
  }
  
  // 初始编译
  if (props.autoCompile && editor.value) {
    await compileTypst();
  }
  
  // 初始化元素映射
  await nextTick();
  updateElementMap();
});

onUnmounted(() => {
  // 清理编辑器监听
  if (editor.value) {
    editor.value.off('update', handleEditorUpdate);
  }
  
  // 清理PDF URL
  if (pdfData.value && pdfData.value.startsWith('blob:')) {
    URL.revokeObjectURL(pdfData.value);
  }
  
  logger.info('Dual pane editor unmounted', {}, LogCategory.UI);
});

// 暴露方法给父组件
defineExpose({
  compile: triggerManualCompile,
  getContent: () => editor.value?.getHTML() || '',
  setContent: (content: string) => {
    editor.value?.commands.setContent(content);
  },
  getEditor: () => editor.value,
  getEditorState: () => editorState.value,
  getCompileState: () => compileState.value,
  enableSync,
  disableSync
});
</script>

<template>
  <div class="dual-pane-editor" :class="`theme-${theme}`">
    <!-- 左侧：Tiptap编辑器 -->
    <div class="editor-pane">
      <div class="pane-header">
        <h3>编辑器</h3>
        <div class="header-actions">
          <button class="btn-icon" title="撤销" @click="undo">
            <Undo :size="16" />
          </button>
          <button class="btn-icon" title="重做" @click="redo">
            <Redo :size="16" />
          </button>
          <button class="btn-icon" :disabled="compileState.isCompiling" title="编译" @click="triggerManualCompile">
            <Loader2 v-if="compileState.isCompiling" :size="16" class="spin" />
            <Play v-else :size="16" />
          </button>
        </div>
      </div>
      
      <!-- 工具栏（使用useStateSync的ribbonButtons状态） -->
      <div class="editor-toolbar">
        <div class="toolbar-group">
          <button 
            class="toolbar-btn" 
            :class="{ active: editorState.isBold }" 
            title="粗体" 
            @click="setBold"
          >
            <Bold :size="16" />
          </button>
          <button 
            class="toolbar-btn" 
            :class="{ active: editorState.isItalic }" 
            title="斜体" 
            @click="setItalic"
          >
            <Italic :size="16" />
          </button>
          <button 
            class="toolbar-btn" 
            :class="{ active: editorState.isUnderline }" 
            title="下划线" 
            @click="setUnderline"
          >
            <UnderlineIcon :size="16" />
          </button>
          <button 
            class="toolbar-btn" 
            :class="{ active: editorState.isStrike }" 
            title="删除线" 
            @click="setStrike"
          >
            <Strikethrough :size="16" />
          </button>
        </div>
        
        <div class="toolbar-separator"></div>
        
        <div class="toolbar-group">
          <button 
            class="toolbar-btn" 
            :class="{ active: editorState.headingLevel === 1 }" 
            title="标题1" 
            @click="setHeading(1)"
          >
            <Heading1 :size="16" />
          </button>
          <button 
            class="toolbar-btn" 
            :class="{ active: editorState.headingLevel === 2 }" 
            title="标题2" 
            @click="setHeading(2)"
          >
            <Heading2 :size="16" />
          </button>
          <button 
            class="toolbar-btn" 
            :class="{ active: editorState.headingLevel === 3 }" 
            title="标题3" 
            @click="setHeading(3)"
          >
            <Heading3 :size="16" />
          </button>
        </div>
        
        <div class="toolbar-separator"></div>
        
        <div class="toolbar-group">
          <button 
            class="toolbar-btn" 
            :class="{ active: editorState.isBulletList }" 
            title="无序列表" 
            @click="setBulletList"
          >
            <List :size="16" />
          </button>
          <button 
            class="toolbar-btn" 
            :class="{ active: editorState.isOrderedList }" 
            title="有序列表" 
            @click="setOrderedList"
          >
            <ListOrdered :size="16" />
          </button>
          <button 
            class="toolbar-btn" 
            :class="{ active: editorState.isBlockquote }" 
            title="引用" 
            @click="setBlockquote"
          >
            <Quote :size="16" />
          </button>
        </div>
        
        <div class="toolbar-separator"></div>
        
        <div class="toolbar-group">
          <button class="toolbar-btn" title="插入表格" @click="insertTable">
            <TableIcon :size="16" />
          </button>
          <button 
            class="toolbar-btn" 
            title="添加行" 
            :disabled="!editorState.isInTable" 
            @click="addRow"
          >
            <Plus :size="16" />
          </button>
          <button 
            class="toolbar-btn" 
            title="删除行" 
            :disabled="!editorState.isInTable" 
            @click="deleteRow"
          >
            <Minus :size="16" />
          </button>
          <button 
            class="toolbar-btn" 
            title="添加列" 
            :disabled="!editorState.isInTable" 
            @click="addColumn"
          >
            <Plus :size="16" />
          </button>
          <button 
            class="toolbar-btn" 
            title="删除列" 
            :disabled="!editorState.isInTable" 
            @click="deleteColumn"
          >
            <Minus :size="16" />
          </button>
        </div>
      </div>
      
      <!-- 编辑器内容 -->
      <div ref="editorContainerRef" class="editor-content">
        <EditorContent v-if="editor" :editor="editor" />
      </div>
      
      <!-- 右键菜单 -->
      <div 
        v-if="showContextMenu" 
        class="context-menu"
        :style="{ left: contextMenuPosition.x + 'px', top: contextMenuPosition.y + 'px' }"
        @click="hideContextMenu"
      >
        <div 
          v-for="item in contextMenuItems" 
          :key="item.id"
          class="context-menu-item"
          :class="{ separator: item.separator }"
          :disabled="!item.enabled"
          @click="executeContextMenuItem(item)"
        >
          {{ item.label }}
        </div>
      </div>
    </div>
    
    <!-- 右侧：PDF预览 -->
    <div class="preview-pane" v-if="false">
      <div class="pane-header">
        <h3>PDF 预览</h3>
        <div class="header-actions">
          <button class="btn-icon" title="缩小" @click="zoomOut">
            <ZoomOut :size="16" />
          </button>
          <span class="zoom-level">{{ Math.round(pdfScale * 100) }}%</span>
          <button class="btn-icon" title="放大" @click="zoomIn">
            <ZoomIn :size="16" />
          </button>
          <button class="btn-icon" title="适应宽度" @click="fitToWidth">
            <Maximize2 :size="16" />
          </button>
          <button class="btn-icon" title="适应页面" @click="fitToPage">
            <Minimize2 :size="16" />
          </button>
          <button class="btn-icon" title="逆时针旋转" @click="rotateCounterClockwise">
            <RotateCcw :size="16" />
          </button>
          <button class="btn-icon" title="顺时针旋转" @click="rotateClockwise">
            <RotateCw :size="16" />
          </button>
        </div>
      </div>
      
      <!-- PDF预览内容 -->
      <div ref="pdfContainerRef" class="preview-content">
        <!-- PDF预览暂时禁用 -->
        <div class="empty-state">
          <p>PDF预览功能暂时禁用（PDF.js worker配置问题）</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dual-pane-editor {
  display: flex;
  height: 100%;
  background: var(--bg-primary, #ffffff);
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 8px;
  overflow: hidden;
}

.editor-pane,
.preview-pane {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.editor-pane {
  border-right: 1px solid var(--border-color, #e0e0e0);
}

.pane-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: var(--bg-secondary, #f5f5f5);
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.pane-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary, #333333);
}

.header-actions {
  display: flex;
  gap: 4px;
}

.btn-icon {
  padding: 6px 10px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 4px;
  background: var(--bg-primary, #ffffff);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.btn-icon:hover:not(:disabled) {
  background: var(--bg-secondary, #f5f5f5);
  border-color: var(--primary-color, #007bff);
}

.btn-icon:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-icon .spin {
  animation: spin 1s linear infinite;
}

.editor-toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: var(--bg-primary, #ffffff);
  border-bottom: 1px solid var(--border-color, #e0e0e0);
  flex-wrap: wrap;
}

.toolbar-group {
  display: flex;
  gap: 4px;
}

.toolbar-separator {
  width: 1px;
  height: 24px;
  background: var(--border-color, #d0d0d0);
  margin: 0 4px;
}

.toolbar-btn {
  padding: 6px 10px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 4px;
  background: var(--bg-primary, #ffffff);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.toolbar-btn:hover:not(:disabled) {
  background: var(--bg-secondary, #f5f5f5);
}

.toolbar-btn.active {
  background: var(--primary-color, #007bff);
  color: white;
  border-color: var(--primary-color, #007bff);
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.editor-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  position: relative;
}

/* 右键菜单 */
.context-menu {
  position: fixed;
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  z-index: 1000;
  min-width: 200px;
}

.context-menu-item {
  padding: 8px 16px;
  cursor: pointer;
  transition: background 0.2s;
}

.context-menu-item:hover:not(:disabled) {
  background: #f5f5f5;
}

.context-menu-item:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.context-menu-item.separator {
  border-top: 1px solid #e0e0e0;
  margin: 4px 0;
  padding: 0;
}

/* Tiptap编辑器样式 */
:deep(.tiptap-editor) {
  outline: none;
  min-height: 100%;
}

:deep(.tiptap-editor p) {
  margin: 0.5em 0;
  line-height: 1.6;
}

:deep(.tiptap-editor h1) {
  font-size: 2em;
  font-weight: bold;
  margin: 0.67em 0;
}

:deep(.tiptap-editor h2) {
  font-size: 1.5em;
  font-weight: bold;
  margin: 0.75em 0;
}

:deep(.tiptap-editor h3) {
  font-size: 1.17em;
  font-weight: bold;
  margin: 0.83em 0;
}

:deep(.tiptap-editor ul),
:deep(.tiptap-editor ol) {
  padding-left: 2em;
  margin: 0.5em 0;
}

:deep(.tiptap-editor blockquote) {
  border-left: 4px solid var(--border-color, #d0d0d0);
  padding-left: 1em;
  margin: 1em 0;
  color: var(--text-secondary, #666666);
}

:deep(.tiptap-editor table) {
  border-collapse: collapse;
  width: 100%;
  margin: 1em 0;
}

:deep(.tiptap-editor table td),
:deep(.tiptap-editor table th) {
  border: 1px solid var(--border-color, #d0d0d0);
  padding: 8px;
}

:deep(.tiptap-editor table th) {
  background: var(--bg-secondary, #f5f5f5);
  font-weight: bold;
}

.preview-content {
  flex: 1;
  overflow: auto;
  background: var(--bg-secondary, #f5f5f5);
  padding: 0;
  display: flex;
  align-items: stretch;
  justify-content: stretch;
}

.error-panel {
  background: var(--error-bg, #ffebee);
  border: 1px solid var(--error-color, #f44336);
  border-radius: 4px;
  padding: 16px;
  max-width: 600px;
  width: 100%;
}

.error-panel h4 {
  margin: 0 0 8px 0;
  color: var(--error-color, #f44336);
  font-size: 14px;
}

.error-panel pre {
  margin: 0;
  font-family: 'Fira Code', 'Consolas', monospace;
  font-size: 12px;
  color: var(--error-color, #f44336);
  white-space: pre-wrap;
  word-break: break-all;
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  width: 100%;
  color: var(--text-secondary, #666666);
  font-size: 14px;
}

.loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--border-color, #e0e0e0);
  border-top-color: var(--primary-color, #007bff);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.zoom-level {
  font-size: 12px;
  color: var(--text-secondary, #666666);
  min-width: 40px;
  text-align: center;
}

/* 主题样式 */
.theme-dark {
  --bg-primary: #1e1e1e;
  --bg-secondary: #2d2d2d;
  --text-primary: #ffffff;
  --text-secondary: #b0b0b0;
  --border-color: #404040;
}

.theme-dark .editor-toolbar,
.theme-dark .pane-header {
  background: var(--bg-secondary);
  border-color: var(--border-color);
}

.theme-dark .toolbar-btn,
.theme-dark .btn-icon {
  background: var(--bg-primary);
  color: var(--text-primary);
  border-color: var(--border-color);
}

.theme-dark .toolbar-btn:hover:not(:disabled),
.theme-dark .btn-icon:hover:not(:disabled) {
  background: var(--bg-secondary);
}

.theme-dark .preview-content {
  background: var(--bg-secondary);
}
</style>
