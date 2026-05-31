<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue';
import { logger, LogCategory } from '../utils/logger';
import { createError, ErrorCode, ErrorSeverity, ErrorCategory } from '../utils/errorHandler';
import { typst } from '../utils/typstConverter';
import { typstHighlighter } from '../utils/typstHighlighter';

interface TypstPreviewEditorProps {
  modelValue?: string;
  theme?: 'light' | 'dark';
  fontSize?: number;
  showLineNumbers?: boolean;
  autoCompile?: boolean;
  compileDelay?: number;
}

const props = withDefaults(defineProps<TypstPreviewEditorProps>(), {
  modelValue: '',
  theme: 'light',
  fontSize: 14,
  showLineNumbers: true,
  autoCompile: true,
  compileDelay: 500
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
  'compile': [typstCode: string];
  'error': [error: Error];
  'compiled': [pdfData: Uint8Array];
}>();

// 编辑器状态
const editorContent = ref(props.modelValue);
const highlightedCode = ref('');
const compiledPdf = ref<Uint8Array | null>(null);
const isCompiling = ref(false);
const compileError = ref<string | null>(null);
const compileTimer = ref<number | null>(null);

// 编辑器引用
const editorRef = ref<HTMLTextAreaElement | null>(null);
const previewRef = ref<HTMLDivElement | null>(null);

// Typst配置
const typstConfig = ref({
  paper: 'a4',
  margin: { top: '25mm', bottom: '25mm', left: '20mm', right: '20mm' },
  font: 'SimSun',
  fontSize: '11pt',
  lineHeight: '17pt',
  paragraphSpacing: '6pt',
  headingFont: 'SimHei',
  headingBaseSize: '16pt',
  headingScale: 1
});

// 计算属性
const editorClass = computed(() => [
  'typst-editor',
  `theme-${props.theme}`,
  `font-size-${props.fontSize}`,
  { 'show-line-numbers': props.showLineNumbers }
]);

const previewClass = computed(() => [
  'typst-preview',
  `theme-${props.theme}`,
  { 'has-error': !!compileError.value }
]);

// PDF URL计算属性
const pdfUrl = computed(() => {
  if (!compiledPdf.value || compiledPdf.value.length === 0) {
return '';
}
  const blob = new Blob([compiledPdf.value as unknown as ArrayBuffer], { type: 'application/pdf' });
  return URL.createObjectURL(blob);
});

// 监听props变化
watch(() => props.modelValue, (newValue) => {
  editorContent.value = newValue;
  if (props.autoCompile) {
    scheduleCompile();
  }
});

watch(editorContent, (newValue) => {
  emit('update:modelValue', newValue);
  if (props.autoCompile) {
    scheduleCompile();
  }
});

// 语法高亮
const highlightCode = () => {
  try {
    highlightedCode.value = typstHighlighter.highlight(editorContent.value);
  } catch (error) {
    logger.error('Failed to highlight code', error, LogCategory.UI);
    highlightedCode.value = editorContent.value;
  }
};

// 编译Typst
const compileTypst = async () => {
  if (!editorContent.value.trim()) {
    compiledPdf.value = null;
    compileError.value = null;
    return;
  }

  isCompiling.value = true;
  compileError.value = null;

  try {
    logger.info('Compiling Typst code', { contentLength: editorContent.value.length }, LogCategory.BUSINESS);
    
    emit('compile', editorContent.value);
    
    // 更新Typst配置
    typst.updateConfig(typstConfig.value);
    
    // 转换为Typst（这里暂时只是转换，实际编译需要后端支持）
    const typstCode = typst.convertMarkdown(editorContent.value);
    
    // TODO: 调用后端编译服务生成PDF
    // const pdfData = await invoke('compile_typst', { code: typstCode });
    // compiledPdf.value = new Uint8Array(pdfData);
    
    // 临时：显示编译成功状态
    compiledPdf.value = new Uint8Array();
    
    logger.info('Typst compilation completed', { typstLength: typstCode.length }, LogCategory.BUSINESS);
    emit('compiled', compiledPdf.value);
  } catch (error) {
    logger.error('Failed to compile Typst', error, LogCategory.BUSINESS);
    compileError.value = error instanceof Error ? error.message : 'Unknown error';
    emit('error', error instanceof Error ? error : new Error(String(error)));
  } finally {
    isCompiling.value = false;
  }
};

// 防抖编译
const scheduleCompile = () => {
  if (compileTimer.value) {
    clearTimeout(compileTimer.value);
  }
  
  compileTimer.value = window.setTimeout(() => {
    compileTypst();
  }, props.compileDelay);
};

// 手动编译
const manualCompile = () => {
  if (compileTimer.value) {
    clearTimeout(compileTimer.value);
  }
  compileTypst();
};

// 格式化代码
const formatCode = () => {
  try {
    // TODO: 实现Typst代码格式化
    logger.info('Formatting Typst code', {}, LogCategory.UI);
  } catch (error) {
    logger.error('Failed to format code', error, LogCategory.UI);
  }
};

// 插入模板
const insertTemplate = (template: string) => {
  const cursorPosition = editorRef.value?.selectionStart || 0;
  const before = editorContent.value.substring(0, cursorPosition);
  const after = editorContent.value.substring(cursorPosition);
  editorContent.value = before + template + after;
  
  // 设置光标位置
  nextTick(() => {
    if (editorRef.value) {
      const newPosition = cursorPosition + template.length;
      editorRef.value.setSelectionRange(newPosition, newPosition);
      editorRef.value.focus();
    }
  });
};

// 应用配置
const applyConfig = (config: Partial<typeof typstConfig.value>) => {
  typstConfig.value = { ...typstConfig.value, ...config };
  typst.updateConfig(typstConfig.value);
  scheduleCompile();
};

// 下载PDF
const downloadPdf = () => {
  if (!compiledPdf.value || compiledPdf.value.length === 0) {
    logger.warn('No PDF to download', {}, LogCategory.UI);
    return;
  }

  try {
    const blob = new Blob([compiledPdf.value as unknown as ArrayBuffer], { type: 'application/pdf' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `document-${Date.now()}.pdf`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
    
    logger.info('PDF downloaded', {}, LogCategory.UI);
  } catch (error) {
    logger.error('Failed to download PDF', error, LogCategory.UI);
  }
};

// 生命周期
onMounted(() => {
  logger.info('Typst preview editor mounted', { theme: props.theme }, LogCategory.UI);
  highlightCode();
  
  if (props.autoCompile && editorContent.value) {
    scheduleCompile();
  }
});

onUnmounted(() => {
  if (compileTimer.value) {
    clearTimeout(compileTimer.value);
  }
  logger.info('Typst preview editor unmounted', {}, LogCategory.UI);
});

// 暴露方法给父组件
defineExpose({
  compile: manualCompile,
  format: formatCode,
  insertTemplate,
  applyConfig,
  getContent: () => editorContent.value,
  setContent: (content: string) => {
    editorContent.value = content;
  }
});
</script>

<template>
  <div class="typst-preview-editor">
    <!-- 工具栏 -->
    <div class="editor-toolbar">
      <button :disabled="isCompiling" class="btn-primary" @click="manualCompile">
        <span v-if="isCompiling">编译中...</span>
        <span v-else>编译</span>
      </button>
      <button class="btn-secondary" @click="formatCode">格式化</button>
      
      <div class="toolbar-separator"></div>
      
      <select v-model="typstConfig.paper" class="config-select" @change="applyConfig({ paper: typstConfig.paper })">
        <option value="a4">A4</option>
        <option value="letter">Letter</option>
        <option value="legal">Legal</option>
      </select>
      
      <select v-model="typstConfig.font" class="config-select" @change="applyConfig({ font: typstConfig.font })">
        <option value="SimSun">宋体</option>
        <option value="SimHei">黑体</option>
        <option value="KaiTi">楷体</option>
        <option value="FangSong">仿宋</option>
      </select>
      
      <div class="toolbar-separator"></div>
      
      <button class="btn-template" @click="insertTemplate('= 标题')">标题</button>
      <button class="btn-template" @click="insertTemplate('== 副标题')">副标题</button>
      <button class="btn-template" @click="insertTemplate('- 列表项')">列表</button>
      <button class="btn-template" @click="insertTemplate('``` 代码块\n```')">代码</button>
    </div>
    
    <!-- 主编辑区域 -->
    <div class="editor-container">
      <!-- 左侧：代码编辑器 -->
      <div class="editor-pane">
        <div class="pane-header">
          <h3>Typst 代码</h3>
          <span class="status-indicator" :class="{ compiling: isCompiling, error: !!compileError }">
            {{ isCompiling ? '编译中' : (compileError ? '错误' : '就绪') }}
          </span>
        </div>
        
        <div class="editor-wrapper">
          <textarea
            ref="editorRef"
            v-model="editorContent"
            :class="editorClass"
            :style="{ fontSize: `${fontSize}px` }"
            placeholder="在此输入Typst代码..."
            spellcheck="false"
            @input="highlightCode"
          ></textarea>
          
          <!-- 语法高亮预览 -->
          <div class="syntax-highlight" v-html="highlightedCode"></div>
        </div>
      </div>
      
      <!-- 右侧：PDF预览 -->
      <div class="preview-pane">
        <div class="pane-header">
          <h3>PDF 预览</h3>
          <div class="preview-actions">
            <button class="btn-icon" title="刷新预览" @click="manualCompile">🔄</button>
            <button v-if="compiledPdf" class="btn-icon" title="下载PDF" @click="downloadPdf">⬇️</button>
          </div>
        </div>
        
        <div ref="previewRef" :class="previewClass">
          <!-- 错误显示 -->
          <div v-if="compileError" class="error-panel">
            <h4>编译错误</h4>
            <pre>{{ compileError }}</pre>
          </div>
          
          <!-- PDF预览区域 -->
          <div v-else-if="compiledPdf" class="pdf-preview">
            <iframe :src="pdfUrl" class="pdf-iframe"></iframe>
          </div>
          
          <!-- 空状态 -->
          <div v-else class="empty-state">
            <p>输入Typst代码后点击"编译"按钮生成PDF预览</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.typst-preview-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-primary, #ffffff);
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 8px;
  overflow: hidden;
}

.editor-toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: var(--bg-secondary, #f5f5f5);
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.btn-primary,
.btn-secondary,
.btn-template,
.btn-icon,
.config-select {
  padding: 6px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 4px;
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #333333);
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}

.btn-primary:hover {
  background: var(--primary-color, #007bff);
  color: white;
  border-color: var(--primary-color, #007bff);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary:hover {
  background: var(--bg-secondary, #f0f0f0);
}

.btn-template {
  background: var(--accent-color, #e3f2fd);
  border-color: var(--accent-color, #2196f3);
}

.btn-icon {
  padding: 6px;
  font-size: 16px;
}

.config-select {
  min-width: 100px;
}

.toolbar-separator {
  width: 1px;
  height: 24px;
  background: var(--border-color, #d0d0d0);
  margin: 0 4px;
}

.editor-container {
  display: flex;
  flex: 1;
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
  padding: 8px 12px;
  background: var(--bg-secondary, #f5f5f5);
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.pane-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary, #333333);
}

.status-indicator {
  font-size: 12px;
  padding: 2px 8px;
  border-radius: 4px;
  background: var(--success-color, #4caf50);
  color: white;
}

.status-indicator.compiling {
  background: var(--warning-color, #ff9800);
}

.status-indicator.error {
  background: var(--error-color, #f44336);
}

.preview-actions {
  display: flex;
  gap: 4px;
}

.editor-wrapper {
  position: relative;
  flex: 1;
  overflow: hidden;
}

.typst-editor {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  padding: 16px;
  border: none;
  resize: none;
  font-family: 'Fira Code', 'Consolas', 'Monaco', monospace;
  line-height: 1.6;
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #333333);
  white-space: pre;
  overflow: auto;
  z-index: 2;
}

.typst-editor:focus {
  outline: none;
}

.syntax-highlight {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  padding: 16px;
  font-family: 'Fira Code', 'Consolas', 'Monaco', monospace;
  line-height: 1.6;
  background: var(--bg-primary, #ffffff);
  color: transparent;
  white-space: pre;
  overflow: auto;
  pointer-events: none;
  z-index: 1;
}

.typst-preview {
  flex: 1;
  overflow: auto;
  background: var(--bg-secondary, #f5f5f5);
  padding: 16px;
}

.typst-preview.has-error {
  background: var(--error-bg, #ffebee);
}

.error-panel {
  background: var(--error-bg, #ffebee);
  border: 1px solid var(--error-color, #f44336);
  border-radius: 4px;
  padding: 12px;
  margin-bottom: 16px;
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

.pdf-preview {
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: flex-start;
}

.pdf-iframe {
  width: 100%;
  height: 100%;
  border: none;
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-secondary, #666666);
  font-size: 14px;
}

/* 主题样式 */
.theme-dark {
  --bg-primary: #1e1e1e;
  --bg-secondary: #2d2d2d;
  --text-primary: #ffffff;
  --text-secondary: #b0b0b0;
  --border-color: #404040;
}

.theme-dark .typst-editor,
.theme-dark .syntax-highlight {
  background: var(--bg-primary);
  color: var(--text-primary);
}

.theme-dark .editor-toolbar,
.theme-dark .pane-header {
  background: var(--bg-secondary);
  border-color: var(--border-color);
}
</style>
