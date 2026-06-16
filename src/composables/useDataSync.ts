/**
 * 数据纽带 - 单向内容流 + 防抖机制 + 微服务集成
 * 
 * 功能：
 * 1. Tiptap (JSON) → 防抖(500ms-1s) → 微服务 → Typst编译 → PDF刷新
 * 2. 避免每次键盘敲击都触发编译，必须等用户停顿或按 Ctrl+S 时再刷新
 * 3. 调用后端微服务进行Typst编译
 * 4. 处理编译错误和加载状态
 */

import { ref, watch, onMounted, onUnmounted } from 'vue'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { invoke } from '@tauri-apps/api/core'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { Editor } from '@tiptap/vue-3';
import { logger, LogCategory } from '../utils/logger';
import { htmlToTypst } from '../utils/translator';

// 数据同步配置
export interface DataSyncConfig {
  debounceDelay: number;        // 防抖延迟（毫秒）
  autoCompile: boolean;         // 是否自动编译
  manualCompileShortcut: string; // 手动编译快捷键
  maxRetries: number;           // 最大重试次数
  retryDelay: number;           // 重试延迟（毫秒）
}

// 编译状态
export interface CompileState {
  isCompiling: boolean;
  lastCompiledAt: number | null;
  compileError: string | null;
  retryCount: number;
  pdfData: Uint8Array | null;
}

// 编译结果
export interface CompileResult {
  success: boolean;
  pdfData?: Uint8Array;
  error?: string;
  typstCode?: string;
  compileTime?: number;
}

export function useDataSync(
  editor: Editor | null,
  config: Partial<DataSyncConfig> = {}
) {
  // 默认配置
  const defaultConfig: DataSyncConfig = {
    debounceDelay: 500,
    autoCompile: true,
    manualCompileShortcut: 'Ctrl+S',
    maxRetries: 3,
    retryDelay: 1000
  };

  const mergedConfig = { ...defaultConfig, ...config };

  // 状态
  const compileState = ref<CompileState>({
    isCompiling: false,
    lastCompiledAt: null,
    compileError: null,
    retryCount: 0,
    pdfData: null
  });

  // 防抖定时器
  let debounceTimer: number | null = null;

  // 当前内容（用于比较是否发生变化）
  let lastContent = '';

  /**
   * 获取编辑器内容
   */
  const getEditorContent = (): string => {
    if (!editor) {
return '';
}
    return editor.getHTML();
  };

  /**
   * 获取编辑器JSON
   */
  const getEditorJSON = (): object => {
    if (!editor) {
return {};
}
    return editor.getJSON();
  };

  /**
   * 防抖编译
   */
  const scheduleCompile = () => {
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }

    debounceTimer = window.setTimeout(() => {
      compile();
    }, mergedConfig.debounceDelay);

    logger.debug('Compile scheduled', { delay: mergedConfig.debounceDelay }, LogCategory.UI);
  };

  /**
   * 手动编译（立即执行，不防抖）
   */
  const manualCompile = () => {
    if (debounceTimer) {
      clearTimeout(debounceTimer);
      debounceTimer = null;
    }
    compile();
  };

  /**
   * 编译内容
   */
  const compile = async (retryCount = 0): Promise<CompileResult> => {
    const content = getEditorContent();
    
    // 检查内容是否变化
    if (content === lastContent && compileState.value.pdfData) {
      logger.debug('Content unchanged, skipping compile', {}, LogCategory.UI);
      return {
        success: true,
        pdfData: compileState.value.pdfData
      };
    }

    // 检查内容是否为空
    if (!content || content.trim() === '') {
      compileState.value.pdfData = null;
      compileState.value.compileError = null;
      lastContent = content;
      
      return {
        success: true
      };
    }

    compileState.value.isCompiling = true;
    compileState.value.compileError = null;
    compileState.value.retryCount = retryCount;

    const startTime = Date.now();

    try {
      logger.info('Starting compilation', { contentLength: content.length, retryCount }, LogCategory.BUSINESS);
      
      // Note: emit events removed as composables cannot use defineEmits
      // emit('compile', content);

      // 转换HTML为Typst
      const typstCode = htmlToTypst(content);

      // 调用后端编译
      const b64Pdf: string = await invoke<string>('compile_typst', { code: typstCode });

      // 解析 data URL
      const pdfBase64 = b64Pdf.replace(/^data:application\/pdf;base64,/, '');
      const binaryStr = atob(pdfBase64);
      const pdfData = new Uint8Array(binaryStr.length);
      for (let i = 0; i < binaryStr.length; i++) {
        pdfData[i] = binaryStr.charCodeAt(i);
      }
      
      const compileTime = Date.now() - startTime;
      
      // 更新状态
      compileState.value.pdfData = pdfData;
      compileState.value.lastCompiledAt = Date.now();
      compileState.value.compileError = null;
      compileState.value.retryCount = 0;
      lastContent = content;
      
      const result: CompileResult = {
        success: true,
        pdfData,
        typstCode,
        compileTime
      };
      
      logger.info('Compilation completed', { compileTime }, LogCategory.BUSINESS);
      // Note: emit events removed as composables cannot use defineEmits
      // emit('compiled', result);
      
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      
      compileState.value.compileError = errorMessage;
      
      logger.error('Compilation failed', error, LogCategory.BUSINESS);
      // Note: emit events removed as composables cannot use defineEmits
      // emit('error', errorMessage);
      
      // 重试逻辑
      if (retryCount < mergedConfig.maxRetries) {
        logger.info('Retrying compilation', { retryCount: retryCount + 1 }, LogCategory.BUSINESS);
        
        await new Promise(resolve => setTimeout(resolve, mergedConfig.retryDelay));
        return compile(retryCount + 1);
      }
      
      return {
        success: false,
        error: errorMessage
      };
    } finally {
      compileState.value.isCompiling = false;
    }
  };

  /**
   * 取消编译
   */
  const cancelCompile = () => {
    if (debounceTimer) {
      clearTimeout(debounceTimer);
      debounceTimer = null;
    }
    compileState.value.isCompiling = false;
    
    logger.debug('Compilation cancelled', {}, LogCategory.UI);
  };

  /**
   * 重置编译状态
   */
  const resetCompileState = () => {
    compileState.value = {
      isCompiling: false,
      lastCompiledAt: null,
      compileError: null,
      retryCount: 0,
      pdfData: null
    };
    lastContent = '';
    
    logger.debug('Compile state reset', {}, LogCategory.UI);
  };

  /**
   * 获取PDF URL
   */
  const getPdfUrl = (): string => {
    if (!compileState.value.pdfData || compileState.value.pdfData.length === 0) {
      return '';
    }
    
    const blob = new Blob([compileState.value.pdfData as unknown as ArrayBuffer], { type: 'application/pdf' });
    return URL.createObjectURL(blob);
  };

  /**
   * 下载PDF
   */
  const downloadPdf = (filename = `document-${Date.now()}.pdf`) => {
    if (!compileState.value.pdfData || compileState.value.pdfData.length === 0) {
      logger.warn('No PDF to download', {}, LogCategory.UI);
      return;
    }

    try {
      const blob = new Blob([compileState.value.pdfData as unknown as ArrayBuffer], { type: 'application/pdf' });
      const url = URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = filename;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      URL.revokeObjectURL(url);
      
      logger.info('PDF downloaded', { filename }, LogCategory.UI);
    } catch (error) {
      logger.error('Failed to download PDF', error, LogCategory.UI);
    }
  };

  /**
   * 处理键盘快捷键
   */
  const handleKeyboardShortcut = (event: KeyboardEvent) => {
    // Ctrl+S 手动编译
    if (event.ctrlKey && event.key === 's') {
      event.preventDefault();
      manualCompile();
    }
  };

  // 监听编辑器内容变化
  const handleEditorUpdate = () => {
    if (mergedConfig.autoCompile) {
      scheduleCompile();
    }
  };

  // 生命周期
  onMounted(() => {
    if (editor) {
      editor.on('update', handleEditorUpdate);
      
      // 监听键盘快捷键
      document.addEventListener('keydown', handleKeyboardShortcut);
      
      logger.info('Data sync mounted', { config: mergedConfig }, LogCategory.UI);
    }
  });

  onUnmounted(() => {
    if (editor) {
      editor.off('update', handleEditorUpdate);
      
      // 移除键盘快捷键监听
      document.removeEventListener('keydown', handleKeyboardShortcut);
      
      // 清理定时器
      if (debounceTimer) {
        clearTimeout(debounceTimer);
      }
      
      logger.info('Data sync unmounted', {}, LogCategory.UI);
    }
  });

  return {
    // 状态
    compileState,
    
    // 方法
    getEditorContent,
    getEditorJSON,
    scheduleCompile,
    manualCompile,
    cancelCompile,
    resetCompileState,
    getPdfUrl,
    downloadPdf,
    handleKeyboardShortcut
  };
}
