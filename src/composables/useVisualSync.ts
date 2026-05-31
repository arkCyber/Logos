/**
 * 视觉纽带 - SyncTeX双向同步滚动
 * 
 * 功能：
 * 1. 正向同步（左动右动）：Tiptap编辑器滚动/点击 → PDF预览区自动滚动到对应位置
 * 2. 反向同步（右动左动）：PDF预览区双击 → Tiptap编辑器自动滚动并聚焦到对应元素
 * 3. 基于ID的元素位置映射
 * 4. 平滑滚动动画
 */

import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue';
import { Editor } from '@tiptap/vue-3';
import { logger, LogCategory } from '../utils/logger';

// 元素位置映射
export interface ElementPosition {
  id: string;
  type: 'heading' | 'paragraph' | 'table' | 'image' | 'list' | 'code';
  editorOffset: number;      // 编辑器中的偏移量
  pdfPage: number;           // PDF页码
  pdfOffset: number;         // PDF页面中的偏移量
}

// 同步配置
export interface VisualSyncConfig {
  enabled: boolean;
  syncDelay: number;         // 同步延迟（毫秒）
  smoothScroll: boolean;     // 平滑滚动
  scrollDuration: number;    // 滚动动画时长（毫秒）
  syncOnScroll: boolean;     // 滚动时同步
  syncOnClick: boolean;      // 点击时同步
}

// 同步状态
export interface SyncState {
  isSyncing: boolean;
  lastSyncTime: number;
  syncDirection: 'editor-to-pdf' | 'pdf-to-editor' | null;
  currentElement: ElementPosition | null;
}

export function useVisualSync(
  editor: Editor | null,
  editorContainer: HTMLElement | null,
  pdfContainer: HTMLElement | null,
  config: Partial<VisualSyncConfig> = {}
) {
  // 默认配置
  const defaultConfig: VisualSyncConfig = {
    enabled: true,
    syncDelay: 100,
    smoothScroll: true,
    scrollDuration: 300,
    syncOnScroll: true,
    syncOnClick: true
  };

  const mergedConfig = { ...defaultConfig, ...config };

  // 状态
  const syncState = ref<SyncState>({
    isSyncing: false,
    lastSyncTime: 0,
    syncDirection: null,
    currentElement: null
  });

  // 元素位置映射表
  const elementMap = ref<Map<string, ElementPosition>>(new Map());

  // 同步定时器
  let syncTimer: number | null = null;

  /**
   * 生成元素ID
   */
  const generateElementId = (type: string, index: number): string => {
    return `${type}-${Date.now()}-${index}`;
  };

  /**
   * 扫描编辑器元素并建立映射
   */
  const scanEditorElements = (): ElementPosition[] => {
    if (!editor || !editorContainer) {
return [];
}

    const positions: ElementPosition[] = [];
    const elements = editorContainer.querySelectorAll('[data-element-id]');

    elements.forEach((element, index) => {
      const id = element.getAttribute('data-element-id') || generateElementId('element', index);
      const type = element.getAttribute('data-element-type') as ElementPosition['type'] || 'paragraph';
      const offset = (element as HTMLElement).offsetTop;

      positions.push({
        id,
        type,
        editorOffset: offset,
        pdfPage: 1, // TODO: 从PDF获取对应页码
        pdfOffset: 0 // TODO: 从PDF获取对应偏移
      });
    });

    // 更新映射表
    elementMap.value.clear();
    positions.forEach(pos => {
      elementMap.value.set(pos.id, pos);
    });

    logger.debug('Editor elements scanned', { count: positions.length }, LogCategory.UI);
    return positions;
  };

  /**
   * 扫描PDF元素并建立映射
   */
  const scanPdfElements = (): ElementPosition[] => {
    if (!pdfContainer) {
return [];
}

    const positions: ElementPosition[] = [];
    
    // 从PdfViewer组件获取元素位置
    // PdfViewer组件会通过事件或暴露方法提供元素位置信息
    const pdfViewer = pdfContainer.querySelector('.pdf-viewer') as any;
    if (pdfViewer && pdfViewer.getAllElementPositions) {
      const pdfPositions = pdfViewer.getAllElementPositions();
      pdfPositions.forEach(([elementId, position]: [string, any]) => {
        // 将PDF元素位置映射到ElementPosition格式
        positions.push({
          id: elementId,
          type: 'paragraph', // 简化处理，实际应根据PDF内容判断
          editorOffset: 0, // 需要与编辑器元素匹配
          pdfPage: position.page,
          pdfOffset: position.y
        });
      });
    }
    
    logger.debug('PDF elements scanned', { count: positions.length }, LogCategory.UI);
    return positions;
  };

  /**
   * 更新元素映射
   */
  const updateElementMap = () => {
    const editorElements = scanEditorElements();
    const pdfElements = scanPdfElements();

    // 合并映射
    const mergedMap = new Map<string, ElementPosition>();
    
    editorElements.forEach(editorPos => {
      const pdfPos = pdfElements.find(p => p.id === editorPos.id);
      if (pdfPos) {
        mergedMap.set(editorPos.id, {
          ...editorPos,
          pdfPage: pdfPos.pdfPage,
          pdfOffset: pdfPos.pdfOffset
        });
      } else {
        mergedMap.set(editorPos.id, editorPos);
      }
    });

    elementMap.value = mergedMap;
  };

  /**
   * 正向同步：编辑器 → PDF
   */
  const syncEditorToPdf = (elementId?: string) => {
    if (!mergedConfig.enabled || !pdfContainer) {
return;
}

    syncState.value.isSyncing = true;
    syncState.value.syncDirection = 'editor-to-pdf';

    try {
      let targetPosition: ElementPosition | null = null;

      if (elementId) {
        // 根据元素ID同步
        targetPosition = elementMap.value.get(elementId) || null;
      } else {
        // 根据编辑器滚动位置同步
        const scrollTop = editorContainer?.scrollTop || 0;
        const positions = Array.from(elementMap.value.values());
        
        // 找到最接近当前滚动位置的元素
        targetPosition = positions.reduce((closest, pos) => {
          if (Math.abs(pos.editorOffset - scrollTop) < Math.abs(closest.editorOffset - scrollTop)) {
            return pos;
          }
          return closest;
        }, positions[0] || null);
      }

      if (targetPosition) {
        syncState.value.currentElement = targetPosition;
        
        // 计算PDF滚动位置
        const pdfScrollTop = (targetPosition.pdfPage - 1) * pdfContainer.clientHeight + targetPosition.pdfOffset;
        
        // 滚动PDF
        scrollToPdf(pdfScrollTop);
        
        logger.debug('Synced editor to PDF', { elementId: targetPosition.id, pdfPage: targetPosition.pdfPage }, LogCategory.UI);
      }
    } catch (error) {
      logger.error('Failed to sync editor to PDF', error, LogCategory.UI);
    } finally {
      syncState.value.isSyncing = false;
      syncState.value.lastSyncTime = Date.now();
    }
  };

  /**
   * 反向同步：PDF → 编辑器
   */
  const syncPdfToEditor = (elementId?: string) => {
    if (!mergedConfig.enabled || !editorContainer) {
return;
}

    syncState.value.isSyncing = true;
    syncState.value.syncDirection = 'pdf-to-editor';

    try {
      let targetPosition: ElementPosition | null = null;

      if (elementId) {
        // 根据元素ID同步
        targetPosition = elementMap.value.get(elementId) || null;
      } else {
        // 根据PDF滚动位置同步
        const scrollTop = pdfContainer?.scrollTop || 0;
        const positions = Array.from(elementMap.value.values());
        
        // 找到最接近当前PDF滚动位置的元素
        targetPosition = positions.reduce((closest, pos) => {
          const pdfScrollTop = (pos.pdfPage - 1) * (pdfContainer?.clientHeight || 0) + pos.pdfOffset;
          if (Math.abs(pdfScrollTop - scrollTop) < Math.abs((closest.pdfPage - 1) * (pdfContainer?.clientHeight || 0) + closest.pdfOffset - scrollTop)) {
            return pos;
          }
          return closest;
        }, positions[0] || null);
      }

      if (targetPosition) {
        syncState.value.currentElement = targetPosition;
        
        // 滚动编辑器
        scrollToEditor(targetPosition.editorOffset);
        
        // 聚焦到对应元素
        focusEditorElement(targetPosition.id);
        
        logger.debug('Synced PDF to editor', { elementId: targetPosition.id, editorOffset: targetPosition.editorOffset }, LogCategory.UI);
      }
    } catch (error) {
      logger.error('Failed to sync PDF to editor', error, LogCategory.UI);
    } finally {
      syncState.value.isSyncing = false;
      syncState.value.lastSyncTime = Date.now();
    }
  };

  /**
   * 平滑滚动到PDF指定位置
   */
  const scrollToPdf = (scrollTop: number) => {
    if (!pdfContainer) {
return;
}

    if (mergedConfig.smoothScroll) {
      pdfContainer.scrollTo({
        top: scrollTop,
        behavior: 'smooth'
      });
    } else {
      pdfContainer.scrollTop = scrollTop;
    }
  };

  /**
   * 平滑滚动到编辑器指定位置
   */
  const scrollToEditor = (scrollTop: number) => {
    if (!editorContainer) {
return;
}

    if (mergedConfig.smoothScroll) {
      editorContainer.scrollTo({
        top: scrollTop,
        behavior: 'smooth'
      });
    } else {
      editorContainer.scrollTop = scrollTop;
    }
  };

  /**
   * 聚焦到编辑器元素
   */
  const focusEditorElement = (elementId: string) => {
    if (!editor || !editorContainer) {
return;
}

    const element = editorContainer.querySelector(`[data-element-id="${elementId}"]`);
    if (element) {
      // 高亮元素
      element.classList.add('sync-highlight');
      
      // 移除高亮
      setTimeout(() => {
        element.classList.remove('sync-highlight');
      }, 1000);
      
      // 设置光标位置
      const offset = (element as HTMLElement).offsetTop;
      editor.commands.focus();
    }
  };

  /**
   * 处理编辑器滚动
   */
  const handleEditorScroll = () => {
    if (!mergedConfig.syncOnScroll || syncState.value.isSyncing) {
return;
}

    if (syncTimer) {
      clearTimeout(syncTimer);
    }

    syncTimer = window.setTimeout(() => {
      syncEditorToPdf();
    }, mergedConfig.syncDelay);
  };

  /**
   * 处理PDF滚动
   */
  const handlePdfScroll = () => {
    if (!mergedConfig.syncOnScroll || syncState.value.isSyncing) {
return;
}

    if (syncTimer) {
      clearTimeout(syncTimer);
    }

    syncTimer = window.setTimeout(() => {
      syncPdfToEditor();
    }, mergedConfig.syncDelay);
  };

  /**
   * 处理编辑器点击
   */
  const handleEditorClick = (event: MouseEvent) => {
    if (!mergedConfig.syncOnClick) {
return;
}

    const target = event.target as HTMLElement;
    const elementId = target.closest('[data-element-id]')?.getAttribute('data-element-id');
    
    if (elementId) {
      syncEditorToPdf(elementId);
    }
  };

  /**
   * 处理PDF双击
   */
  const handlePdfDoubleClick = (event: MouseEvent) => {
    if (!mergedConfig.syncOnClick) {
return;
}

    const target = event.target as HTMLElement;
    const elementId = target.closest('[data-element-id]')?.getAttribute('data-element-id');
    
    if (elementId) {
      syncPdfToEditor(elementId);
    }
  };

  /**
   * 启用同步
   */
  const enableSync = () => {
    mergedConfig.enabled = true;
    logger.info('Visual sync enabled', {}, LogCategory.UI);
  };

  /**
   * 禁用同步
   */
  const disableSync = () => {
    mergedConfig.enabled = false;
    logger.info('Visual sync disabled', {}, LogCategory.UI);
  };

  /**
   * 手动触发同步
   */
  const triggerSync = (direction: 'editor-to-pdf' | 'pdf-to-editor') => {
    if (direction === 'editor-to-pdf') {
      syncEditorToPdf();
    } else {
      syncPdfToEditor();
    }
  };

  // 生命周期
  onMounted(() => {
    if (editorContainer) {
      editorContainer.addEventListener('scroll', handleEditorScroll);
      editorContainer.addEventListener('click', handleEditorClick);
    }

    if (pdfContainer) {
      pdfContainer.addEventListener('scroll', handlePdfScroll);
      pdfContainer.addEventListener('dblclick', handlePdfDoubleClick);
    }

    // 初始化元素映射
    nextTick(() => {
      updateElementMap();
    });

    logger.info('Visual sync mounted', { config: mergedConfig }, LogCategory.UI);
  });

  onUnmounted(() => {
    if (editorContainer) {
      editorContainer.removeEventListener('scroll', handleEditorScroll);
      editorContainer.removeEventListener('click', handleEditorClick);
    }

    if (pdfContainer) {
      pdfContainer.removeEventListener('scroll', handlePdfScroll);
      pdfContainer.removeEventListener('dblclick', handlePdfDoubleClick);
    }

    if (syncTimer) {
      clearTimeout(syncTimer);
    }

    logger.info('Visual sync unmounted', {}, LogCategory.UI);
  });

  return {
    // 状态
    syncState,
    elementMap: computed(() => Array.from(elementMap.value.values())),
    
    // 方法
    updateElementMap,
    syncEditorToPdf,
    syncPdfToEditor,
    scrollToPdf,
    scrollToEditor,
    focusEditorElement,
    enableSync,
    disableSync,
    triggerSync
  };
}
