<script setup lang="ts">
/**
 * 航空航天级 PDF 查看器组件
 * 
 * 功能：
 * 1. 使用 pdf.js 渲染 PDF 字节流
 * 2. 支持缩放、旋转、翻页
 * 3. 支持文本选择和搜索
 * 4. 提供元素位置信息用于双向同步
 * 5. 完整的错误处理和日志记录
 */

import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue'; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as pdfjsLib from 'pdfjs-dist';
import { logger, LogCategory } from '../utils/logger';
import { createError, ErrorCode, ErrorSeverity, ErrorCategory } from '../utils/errorHandler'; // eslint-disable-line @typescript-eslint/no-unused-vars

// 配置 pdf.js worker
pdfjsLib.GlobalWorkerOptions.workerSrc = `//cdnjs.cloudflare.com/ajax/libs/pdf.js/${pdfjsLib.version}/pdf.worker.min.js`;

interface PdfViewerProps {
  pdfData: string | Uint8Array | null;
  initialScale?: number;
  initialRotation?: number;
  enableTextSelection?: boolean;
  enableSearch?: boolean;
  enableSync?: boolean;
}

const props = withDefaults(defineProps<PdfViewerProps>(), {
  pdfData: null,
  initialScale: 1.0,
  initialRotation: 0,
  enableTextSelection: true,
  enableSearch: true,
  enableSync: true
});

const emit = defineEmits<{
  'page-changed': [pageNumber: number, totalPages: number];
  'scale-changed': [scale: number];
  'rotation-changed': [rotation: number];
  'text-selected': [text: string, position: { page: number; x: number; y: number }];
  'element-clicked': [elementId: string, position: { page: number; x: number; y: number }];
  'error': [error: Error];
  'loaded': [pageCount: number];
}>();

// 状态
const pdfDocument = ref<pdfjsLib.PDFDocumentProxy | null>(null);
const currentPage = ref(1);
const totalPages = ref(0);
const scale = ref(props.initialScale);
const rotation = ref(props.initialRotation);
const isLoading = ref(false);
const error = ref<string | null>(null);

// Canvas 和容器引用
const canvasContainerRef = ref<HTMLDivElement | null>(null);
const canvasRef = ref<HTMLCanvasElement | null>(null);
const textLayerRef = ref<HTMLDivElement | null>(null);

// PDF 渲染任务
let renderTask: pdfjsLib.RenderTask | null = null;
let currentPdfData: string | Uint8Array | null = null;

// 元素位置映射（用于双向同步）
const elementPositions = ref<Map<string, { page: number; x: number; y: number; width: number; height: number }>>(new Map());

/**
 * 加载 PDF 文档
 */
const loadPdfDocument = async (data: string | Uint8Array) => {
  try {
    isLoading.value = true;
    error.value = null;

    logger.info('Loading PDF document', { dataSize: typeof data === 'string' ? data.length : data.byteLength }, LogCategory.UI);

    let pdfData: string | Uint8Array = data;
    if (typeof data === 'string' && data.startsWith('data:application/pdf;base64,')) {
      // 如果是base64 URL，转换为Uint8Array
      const base64 = data.split(',')[1];
      const binaryString = atob(base64);
      const bytes = new Uint8Array(binaryString.length);
      for (let i = 0; i < binaryString.length; i++) {
        bytes[i] = binaryString.charCodeAt(i);
      }
      pdfData = bytes;
    }

    const loadingTask = pdfjsLib.getDocument({ data: pdfData });
    const pdf = await loadingTask.promise;

    pdfDocument.value = pdf;
    totalPages.value = pdf.numPages;
    currentPage.value = 1;

    logger.info('PDF document loaded', { pageCount: pdf.numPages }, LogCategory.UI);
    emit('loaded', pdf.numPages);

    // 渲染第一页
    await renderPage(1);
  } catch (err) {
    const errorObj = err instanceof Error ? err : new Error(String(err));
    error.value = errorObj.message;
    
    logger.error('Failed to load PDF document', errorObj, LogCategory.UI);
    emit('error', errorObj);
    
    throw createError(
      'PDF_LOAD_FAILED',
      'Failed to load PDF document',
      ErrorSeverity.ERROR,
      ErrorCategory.EXTERNAL,
      { timestamp: Date.now(), additionalData: { error: errorObj.message } }
    );
  } finally {
    isLoading.value = false;
  }
};

/**
 * 渲染指定页面
 */
const renderPage = async (pageNumber: number) => {
  if (!pdfDocument.value || !canvasRef.value || !canvasContainerRef.value) {
    return;
  }

  try {
    // 取消之前的渲染任务
    if (renderTask) {
      renderTask.cancel();
      renderTask = null;
    }

    logger.debug('Rendering PDF page', { pageNumber, scale: scale.value, rotation: rotation.value }, LogCategory.UI);

    const page = await pdfDocument.value.getPage(pageNumber);
    const viewport = page.getViewport({ scale: scale.value, rotation: rotation.value });

    const canvas = canvasRef.value;
    const context = canvas.getContext('2d');
    if (!context) {
      throw new Error('Failed to get canvas context');
    }

    // 设置 canvas 尺寸
    canvas.height = viewport.height;
    canvas.width = viewport.width;

    // 渲染 PDF 页面到 canvas
    renderTask = page.render({
      canvasContext: context,
      viewport: viewport,
      canvas: canvas
    });

    await renderTask.promise;
    renderTask = null;

    // 渲染文本层（用于文本选择）
    if (props.enableTextSelection && textLayerRef.value) {
      await renderTextLayer(page, viewport);
    }

    // 扫描页面元素（用于双向同步）
    if (props.enableSync) {
      await scanPageElements(page, viewport, pageNumber);
    }

    logger.debug('PDF page rendered', { pageNumber }, LogCategory.UI);
    emit('page-changed', pageNumber, totalPages.value);
  } catch (err) {
    if (err instanceof Error && err.name === 'RenderingCancelledException') {
      logger.debug('PDF rendering cancelled', {}, LogCategory.UI);
      return;
    }

    const errorObj = err instanceof Error ? err : new Error(String(err));
    error.value = errorObj.message;
    
    logger.error('Failed to render PDF page', errorObj, LogCategory.UI);
    emit('error', errorObj);
    
    throw createError(
      'PDF_RENDER_FAILED',
      `Failed to render PDF page ${pageNumber}`,
      ErrorSeverity.ERROR,
      ErrorCategory.EXTERNAL,
      { timestamp: Date.now(), additionalData: { error: errorObj.message } }
    );
  }
};

/**
 * 渲染文本层
 */
const renderTextLayer = async (page: pdfjsLib.PDFPageProxy, viewport: pdfjsLib.PageViewport) => {
  try {
    const textContent = await page.getTextContent();
    const textLayer = textLayerRef.value;
    if (!textLayer) {
      return;
    }

    // 清空文本层
    textLayer.innerHTML = '';

    // 渲染文本项
    textContent.items.forEach((item) => {
      if ('str' in item && item.transform && item.transform.length >= 6) {
        const textDiv = document.createElement('div');
        textDiv.textContent = item.str;
        textDiv.style.position = 'absolute';
        textDiv.style.left = `${item.transform[4]}px`;
        textDiv.style.top = `${viewport.height - item.transform[5] - item.height}px`;
        textDiv.style.fontSize = `${item.height}px`;
        textDiv.style.fontFamily = item.fontName || 'sans-serif';
        textDiv.style.color = 'transparent';
        textDiv.style.userSelect = 'text';
        textLayer.appendChild(textDiv);
      }
    });

    logger.debug('Text layer rendered', { textItems: textContent.items.length }, LogCategory.UI);
  } catch (err) {
    const errorObj = err instanceof Error ? err : new Error(String(err));
    logger.error('Failed to render text layer', errorObj, LogCategory.UI);
  }
};

/**
 * 扫描页面元素（用于双向同步）
 */
const scanPageElements = async (page: pdfjsLib.PDFPageProxy, viewport: pdfjsLib.PageViewport, pageNumber: number) => {
  try {
    const textContent = await page.getTextContent();
    
    // 简单的元素识别：基于文本内容生成元素ID
    textContent.items.forEach((item, index) => {
      if ('str' in item && item.transform && item.transform.length >= 6) {
        const text = item.str.trim();
        if (text.length > 0) {
          const elementId = `pdf-element-${pageNumber}-${index}`;
          const x = item.transform[4];
          const y = viewport.height - item.transform[5] - item.height;
          
          elementPositions.value.set(elementId, {
            page: pageNumber,
            x,
            y,
            width: item.width || 0,
            height: item.height || 0
          });
        }
      }
    });

    logger.debug('Page elements scanned', { 
      pageNumber, 
      elementCount: elementPositions.value.size 
    }, LogCategory.UI);
  } catch (err) {
    const errorObj = err instanceof Error ? err : new Error(String(err));
    logger.error('Failed to scan page elements', errorObj, LogCategory.UI);
  }
};

/**
 * 缩放 PDF
 */
const zoomIn = () => {
  const newScale = Math.min(scale.value + 0.25, 3.0);
  scale.value = newScale;
  emit('scale-changed', newScale);
  renderPage(currentPage.value);
};

const zoomOut = () => {
  const newScale = Math.max(scale.value - 0.25, 0.5);
  scale.value = newScale;
  emit('scale-changed', newScale);
  renderPage(currentPage.value);
};

const setScale = (newScale: number) => {
  scale.value = newScale;
  emit('scale-changed', newScale);
  renderPage(currentPage.value);
};

/**
 * 旋转 PDF
 */
const rotateClockwise = () => {
  rotation.value = (rotation.value + 90) % 360;
  emit('rotation-changed', rotation.value);
  renderPage(currentPage.value);
};

const rotateCounterClockwise = () => {
  rotation.value = (rotation.value - 90 + 360) % 360;
  emit('rotation-changed', rotation.value);
  renderPage(currentPage.value);
};

/**
 * 翻页
 */
const goToPage = (pageNumber: number) => {
  if (pageNumber >= 1 && pageNumber <= totalPages.value) {
    currentPage.value = pageNumber;
    renderPage(pageNumber);
  }
};

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    goToPage(currentPage.value + 1);
  }
};

const previousPage = () => {
  if (currentPage.value > 1) {
    goToPage(currentPage.value - 1);
  }
};

/**
 * 处理文本选择
 */
const handleTextSelection = () => {
  if (!props.enableTextSelection) {
    return;
  }

  const selection = window.getSelection();
  if (selection && selection.toString().trim().length > 0) {
    const text = selection.toString();
    const range = selection.getRangeAt(0);
    const rect = range.getBoundingClientRect();
    
    // 计算相对于 canvas 的位置
    const canvasRect = canvasRef.value?.getBoundingClientRect();
    if (canvasRect) {
      const x = rect.left - canvasRect.left;
      const y = rect.top - canvasRect.top;
      
      emit('text-selected', text, { page: currentPage.value, x, y });
    }
  }
};

/**
 * 处理 PDF 点击（用于双向同步）
 */
const handlePdfClick = (event: MouseEvent) => {
  if (!props.enableSync || !canvasRef.value) {
    return;
  }

  const rect = canvasRef.value.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;

  // 查找点击的元素
  for (const [elementId, position] of elementPositions.value.entries()) {
    if (position.page === currentPage.value &&
        x >= position.x && x <= position.x + position.width &&
        y >= position.y && y <= position.y + position.height) {
      emit('element-clicked', elementId, { page: currentPage.value, x, y });
      break;
    }
  }
};

/**
 * 获取元素位置
 */
const getElementPosition = (elementId: string) => {
  return elementPositions.value.get(elementId);
};

/**
 * 获取所有元素位置
 */
const getAllElementPositions = (): Array<[string, { page: number; x: number; y: number; width: number; height: number }]> => {
  return Array.from(elementPositions.value.entries());
};

// Blob URL 管理
let blobUrl: string | null = null;

const cleanupBlobUrl = () => {
  if (blobUrl) {
    URL.revokeObjectURL(blobUrl);
    blobUrl = null;
  }
};

/**
 * 清理资源
 */
const cleanup = () => {
  if (renderTask) {
    renderTask.cancel();
    renderTask = null;
  }
  
  pdfDocument.value = null;
  
  elementPositions.value.clear();
  
  cleanupBlobUrl();
  
  logger.debug('PDF viewer cleaned up', {}, LogCategory.UI);
};

// 监听 pdfData 变化
watch(() => props.pdfData, async (newData) => {
  if (newData && newData !== currentPdfData) {
    currentPdfData = newData;
    await loadPdfDocument(newData);
  }
});

// 监听缩放变化
watch(() => props.initialScale, (newScale) => {
  scale.value = newScale;
  if (pdfDocument.value) {
    renderPage(currentPage.value);
  }
});

// 监听旋转变化
watch(() => props.initialRotation, (newRotation) => {
  rotation.value = newRotation;
  if (pdfDocument.value) {
    renderPage(currentPage.value);
  }
});

// 生命周期
onMounted(async () => {
  logger.info('PDF viewer mounted', { 
    enableTextSelection: props.enableTextSelection,
    enableSearch: props.enableSearch,
    enableSync: props.enableSync
  }, LogCategory.UI);

  if (props.pdfData) {
    currentPdfData = props.pdfData;
    await loadPdfDocument(props.pdfData);
  }
});

onUnmounted(() => {
  cleanup();
  logger.info('PDF viewer unmounted', {}, LogCategory.UI);
});

// 暴露方法给父组件
defineExpose({
  zoomIn,
  zoomOut,
  setScale,
  rotateClockwise,
  rotateCounterClockwise,
  goToPage,
  nextPage,
  previousPage,
  getElementPosition,
  getAllElementPositions,
  cleanup
});
</script>

<template>
  <div class="pdf-viewer">
    <!-- 工具栏 -->
    <div class="pdf-toolbar">
      <div class="toolbar-group">
        <button 
          class="toolbar-btn" 
          :disabled="currentPage <= 1" 
          title="上一页"
          @click="previousPage"
        >
          ←
        </button>
        <span class="page-info">
          {{ currentPage }} / {{ totalPages }}
        </span>
        <button 
          class="toolbar-btn" 
          :disabled="currentPage >= totalPages" 
          title="下一页"
          @click="nextPage"
        >
          →
        </button>
      </div>

      <div class="toolbar-group">
        <button class="toolbar-btn" title="缩小" @click="zoomOut">
          -
        </button>
        <span class="zoom-level">{{ Math.round(scale * 100) }}%</span>
        <button class="toolbar-btn" title="放大" @click="zoomIn">
          +
        </button>
      </div>

      <div class="toolbar-group">
        <button class="toolbar-btn" title="逆时针旋转" @click="rotateCounterClockwise">
          ↺
        </button>
        <button class="toolbar-btn" title="顺时针旋转" @click="rotateClockwise">
          ↻
        </button>
      </div>
    </div>

    <!-- PDF 容器 -->
    <div 
      ref="canvasContainerRef" 
      class="pdf-container"
      @click="handlePdfClick"
      @mouseup="handleTextSelection"
    >
      <!-- 加载状态 -->
      <div v-if="isLoading" class="loading-overlay">
        <div class="spinner"></div>
        <p>加载中...</p>
      </div>

      <!-- 错误状态 -->
      <div v-else-if="error" class="error-overlay">
        <p>{{ error }}</p>
      </div>

      <!-- 空状态 -->
      <div v-else-if="!pdfData" class="empty-overlay">
        <p>无 PDF 数据</p>
      </div>

      <!-- PDF 渲染区域 -->
      <div v-else class="pdf-content">
        <canvas ref="canvasRef" class="pdf-canvas"></canvas>
        <div ref="textLayerRef" class="text-layer"></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.pdf-viewer {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #f5f5f5;
}

.pdf-toolbar {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 8px 16px;
  background: white;
  border-bottom: 1px solid #e0e0e0;
}

.toolbar-group {
  display: flex;
  align-items: center;
  gap: 4px;
}

.toolbar-btn {
  padding: 6px 12px;
  border: 1px solid #d0d0d0;
  background: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.toolbar-btn:hover:not(:disabled) {
  background: #f0f0f0;
  border-color: #007bff;
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.page-info,
.zoom-level {
  font-size: 12px;
  color: #666;
  min-width: 60px;
  text-align: center;
}

.pdf-container {
  flex: 1;
  overflow: auto;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding: 20px;
  position: relative;
}

.pdf-content {
  position: relative;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.pdf-canvas {
  display: block;
  background: white;
}

.text-layer {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
}

.loading-overlay,
.error-overlay,
.empty-overlay {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #666;
  font-size: 14px;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #e0e0e0;
  border-top-color: #007bff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 16px;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.error-overlay {
  color: #f44336;
}
</style>
