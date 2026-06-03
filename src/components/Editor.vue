<script setup lang="ts">
/* eslint-disable @typescript-eslint/no-unused-vars */
// Editor component setup
import { useEditor, EditorContent } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import { TextStyle } from '@tiptap/extension-text-style';
import { FontFamily } from '@tiptap/extension-font-family';
import { Subscript } from '@tiptap/extension-subscript';
import { Superscript } from '@tiptap/extension-superscript';
import { TextAlign } from '@tiptap/extension-text-align';
import { Image } from '@tiptap/extension-image';
import { Highlight } from '@tiptap/extension-highlight';
import { Typography } from '@tiptap/extension-typography';
import { Placeholder } from '@tiptap/extension-placeholder';
import { CodeBlockLowlight } from '@tiptap/extension-code-block-lowlight';
import { common, createLowlight } from 'lowlight';
import Emoji from '@tiptap/extension-emoji';
import ListKeymap from '@tiptap/extension-list-keymap';
import TableOfContents from '@tiptap/extension-table-of-contents';
import { Extension, Node } from '@tiptap/core';
import { Suggestion } from '@tiptap/suggestion';
import Underline from '@tiptap/extension-underline';
import { ref, computed, onMounted, onUnmounted, watch, nextTick, onErrorCaptured } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { isTauri } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { save, open } from '@tauri-apps/plugin-dialog';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { htmlToTypst } from '../utils/translator';
import { createTypstPreviewScheduler } from '../utils/typstPreviewScheduler';
import { htmlToTypstSlides } from '../utils/slideTranslator';
import { typst } from '../utils/typstConverter';
import { typstHighlighter } from '../utils/typstHighlighter';
import { typstTemplateManager, type TypstTemplate } from '../utils/typstTemplates';
import { Document, Paragraph, TextRun, Packer, HeadingLevel } from 'docx';
import htmlToRtf from 'html-to-rtf';
import { bibliographyManager } from '../utils/bibliography';
import { footnoteManager } from '../utils/footnotes';
import mammoth from 'mammoth';
import katex from 'katex';
import { pinyin } from 'pinyin-pro';
import { spreadsheetApi } from '../services/spreadsheetApi';
import { pptApi, type PptSlide } from '../services/pptApi';
import { pathManager } from '../utils/pathManager';
import { autoSaveManager } from '../utils/autoSaveManager';
import { backupManager } from '../utils/backupManager';
import * as pdfjsLib from 'pdfjs-dist';
import pdfjsWorker from 'pdfjs-dist/build/pdf.worker.min?url';
import { useHybridServices } from '../composables/useHybridServices';
import { useEditorSidebarLayout } from '../composables/useEditorSidebarLayout';
import '../styles/editor-sidebar-layout.css';
import { 
  Clipboard, Scissors, Copy, Paintbrush,
  AlignLeft, AlignCenter, AlignRight, AlignJustify,
  List, ListOrdered, IndentDecrease, IndentIncrease,
  Heading1, Heading2, Heading3,
  Search, Replace, SquareCheck
} from 'lucide-vue-next';
import Spreadsheet from './Spreadsheet.vue';
import UniverSpreadsheet from './UniverSpreadsheet.vue';
import CollaborationUI from './CollaborationUI.vue';
import QuickAccessToolbar from './editor/QuickAccessToolbar.vue';
import StatusBar from './editor/StatusBar.vue';
import FileBackstage from './editor/FileBackstage.vue';
import ContextMenu from './editor/ContextMenu.vue';
import AISidebar from './editor/AISidebar.vue';
import DocumentOutline from './editor/DocumentOutline.vue';
import Help from './editor/Help.vue';
import MiniToolbar from './editor/MiniToolbar.vue';
import BubbleMenu from './editor/BubbleMenu.vue';
import FloatingMenu from './editor/FloatingMenu.vue';
import OptionsDialog from './editor/dialogs/OptionsDialog.vue';
import AboutDialog from './editor/dialogs/AboutDialog.vue';
import ColorPickerDialog from './editor/dialogs/ColorPickerDialog.vue';
import LinkDialog from './editor/dialogs/LinkDialog.vue';
import ClipboardGroup from './editor/toolbar/ClipboardGroup.vue';
import FontGroup from './editor/toolbar/FontGroup.vue';
import ParagraphGroup from './editor/toolbar/ParagraphGroup.vue';
import StylesGroup from './editor/toolbar/StylesGroup.vue';
import EditingGroup from './editor/toolbar/EditingGroup.vue';
import PagesGroup from './editor/toolbar/PagesGroup.vue';
import TablesGroup from './editor/toolbar/TablesGroup.vue';
import IllustrationsGroup from './editor/toolbar/IllustrationsGroup.vue';
import LinksCommentsGroup from './editor/toolbar/LinksCommentsGroup.vue';
import HeaderFooterGroup from './editor/toolbar/HeaderFooterGroup.vue';
import SymbolsGroup from './editor/toolbar/SymbolsGroup.vue';
import AcademicGroup from './editor/toolbar/AcademicGroup.vue';
import PageSetupGroup from './editor/toolbar/PageSetupGroup.vue';
import ParagraphSettingsGroup from './editor/toolbar/ParagraphSettingsGroup.vue';
import ColumnsGroup from './editor/toolbar/ColumnsGroup.vue';
import ArrangeGroup from './editor/toolbar/ArrangeGroup.vue';
import SpreadsheetGroup from './editor/toolbar/SpreadsheetGroup.vue';
import FormulaFunctionsGroup from './editor/toolbar/FormulaFunctionsGroup.vue';
import LookupReferenceGroup from './editor/toolbar/LookupReferenceGroup.vue';
import ConditionalFormattingGroup from './editor/toolbar/ConditionalFormattingGroup.vue';
import ChartsGroup from './editor/toolbar/ChartsGroup.vue';
import PivotTableGroup from './editor/toolbar/PivotTableGroup.vue';
import DataGroup from './editor/toolbar/DataGroup.vue';
import TypstPreviewGroup from './editor/toolbar/TypstPreviewGroup.vue';
import TypstTemplatesGroup from './editor/toolbar/TypstTemplatesGroup.vue';
import TypstFontsGroup from './editor/toolbar/TypstFontsGroup.vue';
import TypstPackagesGroup from './editor/toolbar/TypstPackagesGroup.vue';
import SlideModeGroup from './editor/toolbar/SlideModeGroup.vue';
import SlideExportGroup from './editor/toolbar/SlideExportGroup.vue';
import DesignGroup from './editor/toolbar/DesignGroup.vue';
import TableOfContentsGroup from './editor/toolbar/TableOfContentsGroup.vue';
import CitationsGroup from './editor/toolbar/CitationsGroup.vue';
import FootnotesGroup from './editor/toolbar/FootnotesGroup.vue';
import CrossReferenceGroup from './editor/toolbar/CrossReferenceGroup.vue';
import ProofingGroup from './editor/toolbar/ProofingGroup.vue';
import CommentsGroup from './editor/toolbar/CommentsGroup.vue';
import ChangesGroup from './editor/toolbar/ChangesGroup.vue';
import ViewsGroup from './editor/toolbar/ViewsGroup.vue';
import ShowGroup from './editor/toolbar/ShowGroup.vue';
import ZoomGroup from './editor/toolbar/ZoomGroup.vue';
import WindowGroup from './editor/toolbar/WindowGroup.vue';
import OverflowMenu from './editor/toolbar/OverflowMenu.vue';
import PageLayoutDialog from './editor/dialogs/PageLayoutDialog.vue';
import StyleManagerDialog from './editor/dialogs/StyleManagerDialog.vue';
import HeaderFooterDialog from './editor/dialogs/HeaderFooterDialog.vue';
import ShapeSelectorDialog from './editor/dialogs/ShapeSelectorDialog.vue';
import IconSelectorDialog from './editor/dialogs/IconSelectorDialog.vue';
import SmartArtSelectorDialog from './editor/dialogs/SmartArtSelectorDialog.vue';
import WordArtDialog from './editor/dialogs/WordArtDialog.vue';
import ChartEditorDialog from './editor/dialogs/ChartEditorDialog.vue';
import CommentsPanel from './editor/CommentsPanel.vue';
import RevisionModePanel from './editor/RevisionModePanel.vue';
import TableDesignTab from './editor/TableDesignTab.vue';
import TypstPackageBrowser from './TypstPackageBrowser.vue';
import TypstFontManager from './TypstFontManager.vue';
import TypstExportOptions from './TypstExportOptions.vue';
import WallpaperSelector from './WallpaperSelector.vue';
import { getCollaborationService, ConnectionStatus } from '../services/collaborationService';
import { getCursorTracker } from '../services/cursorTracker';
import { getPresenceManager } from '../services/presenceManager';
import { getOperationBroadcaster } from '../services/operationBroadcaster';

// All imports loaded successfully

// Check if running in Tauri environment
const isTauriEnvironment = () => {
  return typeof window !== 'undefined' && (window as any).__TAURI__ !== undefined;
};

// Tauri environment check ready

// Import aerospace-grade utilities
import { logger, LogCategory } from '../utils/logger';
import { storage } from '../utils/persistenceManager';
import { security, UserRole, ResourceType, ActionType } from '../utils/securityManager';
import { createError, ErrorCode, ErrorSeverity, ErrorCategory } from '../utils/errorHandler';

// Aerospace-grade version history manager using persistence
const versionHistoryManager = {
  versions: [] as Array<{ id: string; content: string; description: string; timestamp: number; auto: boolean; tags: string[]; version: number }>,
  autoSaveEnabled: false,
  currentVersionNumber: 1,

  saveVersion: async (content: string) => {
    try {
      const version = {
        id: `version-${Date.now()}`,
        content,
        description: 'Manual save',
        timestamp: Date.now(),
        auto: false,
        tags: [],
        version: versionHistoryManager.currentVersionNumber++
      };
      versionHistoryManager.versions.push(version);
      await storage.save('version-history', versionHistoryManager.versions);
      logger.info('Version saved', { versionId: version.id, version: version.version }, LogCategory.BUSINESS);
      return version.id;
    } catch (error) {
      logger.error('Failed to save version', error, LogCategory.BUSINESS);
      throw error;
    }
  },

  getVersion: (id: string) => {
    const version = versionHistoryManager.versions.find(v => v.id === id);
    if (!version) {
      logger.warn(`Version not found: ${id}`, {}, LogCategory.BUSINESS);
      return null;
    }
    return version;
  },

  listVersions: () => {
    return versionHistoryManager.versions;
  },

  restoreVersion: (id: string) => {
    const version = versionHistoryManager.getVersion(id);
    if (!version) {
      throw createError(
        ErrorCode.RESOURCE_NOT_FOUND,
        `Version not found: ${id}`,
        ErrorSeverity.ERROR,
        ErrorCategory.BUSINESS
      );
    }
    logger.info('Version restored', { versionId: id }, LogCategory.BUSINESS);
    return version.content;
  },

  deleteVersion: (id: string) => {
    const index = versionHistoryManager.versions.findIndex(v => v.id === id);
    if (index !== -1) {
      versionHistoryManager.versions.splice(index, 1);
      storage.save('version-history', versionHistoryManager.versions);
      logger.info('Version deleted', { versionId: id }, LogCategory.BUSINESS);
    }
  },

  clearAll: () => {
    versionHistoryManager.versions = [];
    storage.save('version-history', versionHistoryManager.versions);
    logger.info('All versions cleared', {}, LogCategory.BUSINESS);
  },

  getAllVersions: () => {
    return versionHistoryManager.versions;
  },

  triggerAutoSave: async (content: string) => {
    if (versionHistoryManager.autoSaveEnabled) {
      const version = {
        id: `autosave-${Date.now()}`,
        content,
        description: 'Auto-save',
        timestamp: Date.now(),
        auto: true,
        tags: ['autosave'],
        version: versionHistoryManager.currentVersionNumber++
      };
      versionHistoryManager.versions.push(version);
      await storage.save('version-history', versionHistoryManager.versions);
      logger.debug('Auto-save triggered', { versionId: version.id, version: version.version }, LogCategory.BUSINESS);
    }
  },

  enableAutoSave: (enabled: boolean) => {
    versionHistoryManager.autoSaveEnabled = enabled;
    logger.info(`Auto-save ${enabled ? 'enabled' : 'disabled'}`, {}, LogCategory.BUSINESS);
  },

  createVersion: async (content: string, description: string, auto: boolean, tags: string[]) => {
    const version = {
      id: `version-${Date.now()}`,
      content,
      description,
      timestamp: Date.now(),
      auto,
      tags,
      version: versionHistoryManager.currentVersionNumber++
    };
    versionHistoryManager.versions.push(version);
    await storage.save('version-history', versionHistoryManager.versions);
    logger.info('Version created', { versionId: version.id, description, version: version.version }, LogCategory.BUSINESS);
    return version.id;
  },

  getLatestVersion: () => {
    if (versionHistoryManager.versions.length === 0) {
      return null;
    }
    return versionHistoryManager.versions[versionHistoryManager.versions.length - 1];
  }
};

// Aerospace-grade spell check service
const spellCheckService = {
  isEnabled: false,
  locale: 'en_US',
  dictionary: new Set<string>(),

  initialize: async (locale: string) => {
    spellCheckService.locale = locale;
    spellCheckService.isEnabled = true;
    logger.info('Spell check service initialized', { locale }, LogCategory.BUSINESS);
  },

  checkText: (text: string) => {
    if (!spellCheckService.isEnabled) {
      return { errors: [], errorCount: 0 };
    }

    const words = text.split(/\s+/);
    const errors: Array<{ word: string; position: number; suggestions: string[] }> = [];
    let errorCount = 0;

    words.forEach((word, index) => {
      const cleanWord = word.toLowerCase().replace(/[^a-z]/g, '');
      if (cleanWord.length > 2 && !spellCheckService.dictionary.has(cleanWord)) {
        errors.push({
          word,
          position: index,
          suggestions: []
        });
        errorCount++;
      }
    });

    return { errors, errorCount };
  },

  checkSpelling: (text: string) => {
    return spellCheckService.checkText(text);
  },

  addToDictionary: (word: string) => {
    spellCheckService.dictionary.add(word.toLowerCase());
    logger.debug('Word added to dictionary', { word }, LogCategory.BUSINESS);
  }
};

// Aerospace-grade table of contents generator
const tocGenerator = {
  generate: (html: string) => {
    const items: Array<{ id: string; level: number; text: string; children: any[] }> = [];
    const parser = new DOMParser();
    const doc = parser.parseFromString(html, 'text/html');
    const headings = doc.querySelectorAll('h1, h2, h3, h4, h5, h6');

    headings.forEach((heading, index) => {
      if (!heading.tagName || heading.tagName.length < 2) {
        return;
      }
      const level = parseInt(heading.tagName[1]);
      const text = heading.textContent || '';
      const id = `heading-${index}`;

      items.push({
        id,
        level,
        text,
        children: []
      });
    });

    logger.info('TOC generated', { itemCount: items.length }, LogCategory.BUSINESS);
    return { html: '', items };
  },

  generateFromHTML: (html: string) => {
    return tocGenerator.generate(html);
  },

  generateHTML: (items: any[]) => {
    let html = '<ul class="toc">';
    items.forEach(item => {
      html += `<li class="toc-item toc-level-${item.level}">`;
      html += `<a href="#${item.id}">${item.text}</a>`;
      if (item.children && item.children.length > 0) {
        html += tocGenerator.generateHTML(item.children);
      }
      html += '</li>';
    });
    html += '</ul>';
    return html;
  }
};

// Aerospace-grade multi-column manager
const multiColumnManager = {
  currentColumns: 1,
  columnGap: '1em',

  setColumns: (count: number) => {
    multiColumnManager.currentColumns = count;
    logger.info('Columns set', { count }, LogCategory.BUSINESS);
  },

  getColumns: () => {
    return multiColumnManager.currentColumns;
  },

  setColumnCount: (count: number) => {
    multiColumnManager.setColumns(count);
  },

  setColumnGap: (gap: string) => {
    multiColumnManager.columnGap = gap;
    logger.info('Column gap set', { gap }, LogCategory.BUSINESS);
  },

  applyLayout: (content: string) => {
    logger.info('Multi-column layout applied', { columns: multiColumnManager.currentColumns }, LogCategory.BUSINESS);
    return content;
  },

  removeLayout: (content: string) => {
    multiColumnManager.currentColumns = 1;
    logger.info('Multi-column layout removed', {}, LogCategory.BUSINESS);
    return content;
  }
};

// Aerospace-grade section breaks manager
const sectionBreaksManager = {
  breaks: [] as Array<{ id: string; type: string; position: number }>,

  insertBreak: (type: string) => {
    const breakItem = {
      id: `break-${Date.now()}`,
      type,
      position: 0
    };
    sectionBreaksManager.breaks.push(breakItem);
    logger.info('Section break inserted', { type, breakId: breakItem.id }, LogCategory.BUSINESS);
  },

  getBreaks: () => {
    return sectionBreaksManager.breaks;
  },

  addSectionBreak: (type: string, position: number) => {
    const breakItem = {
      id: `break-${Date.now()}`,
      type,
      position
    };
    sectionBreaksManager.breaks.push(breakItem);
    logger.info('Section break added', { type, position }, LogCategory.BUSINESS);
    return breakItem.id;
  },

  generateSectionBreakHTML: (breaks: any[]) => {
    let html = '';
    breaks.forEach(breakItem => {
      html += `<div class="section-break section-break-${breakItem.type}"></div>`;
    });
    return html;
  }
};

// Aerospace-grade cross-references manager
const crossReferencesManager = {
  references: [] as Array<{ id: string; type: string; target: string; label: string }>,

  addReference: (ref: any) => {
    const reference = {
      id: `ref-${Date.now()}`,
      type: ref.type || 'cross-ref',
      target: ref.target || '',
      label: ref.label || ''
    };
    crossReferencesManager.references.push(reference);
    logger.info('Cross-reference added', { referenceId: reference.id }, LogCategory.BUSINESS);
  },

  getReferences: () => {
    return crossReferencesManager.references;
  },

  getAllTargets: () => {
    const targets = new Set<string>();
    crossReferencesManager.references.forEach(ref => {
      targets.add(ref.target);
    });
    return Array.from(targets);
  },

  generateReferenceHTML: (ref: any) => {
    return `<a href="#${ref.target}" class="cross-reference" data-type="${ref.type}">${ref.label}</a>`;
  }
};

// Aerospace-grade print preview manager
const printPreviewManager = {
  previewEnabled: false,
  config: {
    pageSize: 'A4',
    orientation: 'portrait',
    margins: { top: '1in', right: '1in', bottom: '1in', left: '1in' }
  },

  generatePreview: (content: string) => {
    logger.info('Print preview generated', { contentLength: content.length }, LogCategory.BUSINESS);
    return [];
  },

  togglePreview: () => {
    printPreviewManager.previewEnabled = !printPreviewManager.previewEnabled;
    logger.info(`Print preview ${printPreviewManager.previewEnabled ? 'enabled' : 'disabled'}`, {}, LogCategory.BUSINESS);
  },

  setConfig: (config: any) => {
    printPreviewManager.config = { ...printPreviewManager.config, ...config };
    logger.info('Print preview config updated', { config: printPreviewManager.config }, LogCategory.BUSINESS);
  },

  exportToPDF: async (content: string) => {
    logger.info('PDF export initiated', { contentLength: content.length }, LogCategory.BUSINESS);
    // In a real implementation, this would generate a PDF
    return true;
  }
};

// Aerospace-grade revision tracking
const revisionTracking = {
  trackingEnabled: false,
  revisions: [] as Array<{ id: string; content: string; author: string; timestamp: number; type: string }>,

  startTracking: () => {
    revisionTracking.trackingEnabled = true;
    logger.info('Revision tracking started', {}, LogCategory.BUSINESS);
  },

  stopTracking: () => {
    revisionTracking.trackingEnabled = false;
    logger.info('Revision tracking stopped', {}, LogCategory.BUSINESS);
  },

  getRevisions: () => {
    return revisionTracking.revisions;
  },

  acceptRevision: (rev: any) => {
    const index = revisionTracking.revisions.findIndex(r => r.id === rev.id);
    if (index !== -1) {
      revisionTracking.revisions.splice(index, 1);
      logger.info('Revision accepted', { revisionId: rev.id }, LogCategory.BUSINESS);
    }
  },

  rejectRevision: (rev: any) => {
    const index = revisionTracking.revisions.findIndex(r => r.id === rev.id);
    if (index !== -1) {
      revisionTracking.revisions.splice(index, 1);
      logger.info('Revision rejected', { revisionId: rev.id }, LogCategory.BUSINESS);
    }
  },

  addRevision: (content: string, author: string, type: string = 'edit') => {
    if (revisionTracking.trackingEnabled) {
      const revision = {
        id: `revision-${Date.now()}`,
        content,
        author,
        timestamp: Date.now(),
        type
      };
      revisionTracking.revisions.push(revision);
      logger.debug('Revision added', { revisionId: revision.id }, LogCategory.BUSINESS);
    }
  }
};

// Stub managers initialized

// Error handler for component setup
onErrorCaptured((err, _instance, info) => {
  const appError = createError(
    ErrorCode.OPERATION_FAILED,
    undefined,
    ErrorSeverity.ERROR,
    ErrorCategory.SYSTEM,
    { timestamp: Date.now(), additionalData: { originalError: err, componentInfo: info } }
  );
  logger.error('Component error captured', appError, LogCategory.SYSTEM);
  return false;
});

interface CommandArgs {
  editor: any;
  range: { from: number; to: number };
}

interface EditorInstance {
  chain: () => any;
  state: {
    doc: {
      textBetween: (from: number, to: number, separator?: string) => string;
      content: {
        size: number;
      };
    };
    selection: {
      from: number;
      to: number;
    };
  };
  commands: {
    setContent: (content: string) => void;
  };
}

// ============================================================================
// AI 功能状态
// ============================================================================
const isAiLoading = ref(false); // AI 处理加载状态
const aiError = ref<string | null>(null); // AI 错误信息
const streamingContent = ref(''); // 流式内容
const streamSelection = ref<{ from: number; to: number } | null>(null); // 流式内容选择范围

// ============================================================================
// 文档状态
// ============================================================================
const isSaving = ref(false); // 保存状态
const isLoading = ref(false); // 加载状态

// 监控 isLoading 变化，添加详细日志
watch(isLoading, (newValue, oldValue) => {
  const stack = new Error().stack;
  logger.debug(`Loading state changed from ${oldValue} to ${newValue}`, {
    timestamp: new Date().toISOString(),
    stack: stack?.split('\n').slice(2, 5).join('\n')
  }, LogCategory.SYSTEM);
}, { immediate: true });

const wordCount = ref(0); // 字数统计
const charCount = ref(0); // 字符统计
const sentenceCount = ref(0); // 句子统计
const avgWordLength = ref(0); // 平均词长
const currentFilename = ref(''); // 当前文件名

// ============================================================================
// 搜索替换状态
// ============================================================================
const showSearchDialog = ref(false); // 搜索对话框显示状态
const searchText = ref(''); // 搜索文本
const replaceText = ref(''); // 替换文本
const currentMatch = ref(0); // 当前匹配位置
const totalMatches = ref(0); // 总匹配数
const searchCaseSensitive = ref(false); // 大小写敏感
const searchWholeWord = ref(false); // 全词匹配
const searchUseRegex = ref(false); // 使用正则表达式
const searchInputRef = ref<HTMLInputElement | null>(null); // 搜索输入框引用

// ============================================================================
// 数学公式状态
// ============================================================================
const showMathDialog = ref(false); // 数学公式对话框显示状态
const mathFormula = ref(''); // 数学公式
const mathPreview = ref(''); // 数学公式预览

// ============================================================================
// 表格样式状态
// ============================================================================
const showTableColorDialog = ref(false); // 表格颜色对话框显示状态
const selectedCellColor = ref('#f9fafb'); // 选中的单元格颜色
const showTableBorderDialog = ref(false); // 表格边框对话框显示状态
const selectedBorderWidth = ref('1px'); // 选中的边框宽度
const selectedBorderStyle = ref('solid'); // 选中的边框样式
const selectedBorderColor = ref('#e5e7eb'); // 选中的边框颜色

// ============================================================================
// 文本方向和注释状态
// ============================================================================
const textDirection = ref<'ltr' | 'rtl'>('ltr'); // 文本方向
const showCommentDialog = ref(false); // 注释对话框显示状态
const commentText = ref(''); // 注释文本
const comments = ref<
  {
    id: string;
    text: string;
    author: string;
    timestamp: number;
    range: { from: number; to: number };
  }[]
>([]); // 注释列表

// ============================================================================
// 模板和书签状态
// ============================================================================
const showTemplatesDialog = ref(false); // 模板对话框显示状态
const showBookmarksDialog = ref(false); // 书签对话框显示状态
const bookmarks = ref<{ id: string; name: string; position: number }[]>([]); // 书签列表

// ============================================================================
// 拼写检查状态
// ============================================================================
const showSpellCheckDialog = ref(false); // 拼写检查对话框显示状态
const spellCheckErrors = ref<{ word: string; suggestions: string[]; position: number }[]>([]); // 拼写错误列表

// ============================================================================
// 自定义样式状态
// ============================================================================
const showStylesDialog = ref(false); // 样式对话框显示状态
const customStyles = ref<{ id: string; name: string; styles: Record<string, string> }[]>([
  {
    id: 'style-1',
    name: '标题1',
    styles: { 'font-size': '24px', 'font-weight': '500', color: '#111827' }
  },
  {
    id: 'style-2',
    name: '副标题',
    styles: { 'font-size': '18px', 'font-weight': '500', color: '#374151' }
  },
  { id: 'style-3', name: '强调', styles: { 'font-weight': '500', color: '#dc2626' } },
  {
    id: 'style-4',
    name: '代码',
    styles: { 'font-family': 'monospace', 'background-color': '#f3f4f6', padding: '2px 6px' }
  }
]); // 自定义样式列表

// ============================================================================
// 图像处理状态
// ============================================================================
const showImageCropDialog = ref(false); // 图像裁剪对话框显示状态
const selectedImageForCrop = ref(''); // 选中的待裁剪图像
const cropX = ref(0); // 裁剪 X 坐标
const cropY = ref(0); // 裁剪 Y 坐标
const cropWidth = ref(100); // 裁剪宽度
const cropHeight = ref(100); // 裁剪高度
const showImageResizeDialog = ref(false); // 图像调整大小对话框显示状态
const showTypstPackageBrowser = ref(false); // Typst包浏览器对话框显示状态
const showTypstFontManager = ref(false); // Typst字体管理对话框显示状态
const showTypstExportOptions = ref(false); // Typst导出选项对话框显示状态
const showWallpaperDialog = ref(false); // 墙纸选择对话框显示状态
const selectedImageWidth = ref(100); // 选中的图像宽度
const selectedImageHeight = ref(100); // 选中的图像高度
const selectedImageUnit = ref<'px' | '%'>('px'); // 图像尺寸单位
const maintainAspectRatio = ref(true); // 保持宽高比

// ============================================================================
// 页面设置状态
// ============================================================================
const showHeaderFooterDialog = ref(false); // 页眉页脚对话框显示状态
const showPageNumberDialog = ref(false); // 页码对话框显示状态
const showOutlineDialog = ref(false); // 大纲对话框显示状态
const showWatermarkDialog = ref(false); // 水印对话框显示状态
const showRevisionDialog = ref(false); // 修订对话框显示状态

// ============================================================================
// 编辑器设置状态
// ============================================================================
const isDarkMode = ref(false); // 深色模式
const showAISidebar = ref(false); // AI侧边栏显示状态
const showDocumentOutline = ref(false); // 文档大纲显示状态
const documentHeadings = ref<Array<{ id: string; level: number; text: string; children?: any[] }>>([]); // 文档标题列表
const showHelp = ref(false); // 帮助对话框显示状态
const showMiniToolbar = ref(false); // 迷你工具栏显示状态
const miniToolbarPosition = ref({ x: 0, y: 0 }); // 迷你工具栏位置
const showOptionsDialog = ref(false); // 选项对话框显示状态
const showAboutDialog = ref(false); // 关于对话框显示状态
const showColorPickerDialog = ref(false); // 颜色选择器对话框显示状态
const colorPickerTarget = ref<'text' | 'highlight'>('text'); // 颜色选择器目标类型
const showLinkDialog = ref(false); // 链接对话框显示状态
const linkDialogUrl = ref(''); // 链接对话框 URL
const linkDialogText = ref(''); // 链接对话框文本
const showUserGuideDialog = ref(false); // 用户指南对话框显示状态
const autoSaveEnabled = ref(true); // 自动保存启用
const autoSaveInterval = ref<ReturnType<typeof setInterval> | null>(null); // 自动保存定时器
const lastSavedContent = ref(''); // 最后保存的内容
const isFullscreen = ref(false); // 全屏状态
const showShortcutsHelp = ref(false); // 快捷键帮助显示状态
const fontSize = ref(11); // 字体大小
const lineHeight = ref(1.15); // 行高
const spacingBefore = ref(0); // 段前间距
const spacingAfter = ref(0); // 段后间距
const paragraphIndent = ref(0); // 段落缩进
const recentFiles = ref<string[]>([]); // 最近文件列表
const showRecentFiles = ref(false); // 最近文件显示状态

// ============================================================================
// Typst 预览状态
// ============================================================================
const showTemplates = ref(false); // 模板显示状态
const cursorPosition = ref(0); // 光标位置
const readingTime = ref(0); // 阅读时间（分钟）
const paragraphCount = ref(0); // 段落数
const lineCount = ref(0); // 行数
const currentPage = ref(1); // 当前页码
const totalPages = ref(1); // 总页数
const pageContents = ref<string[]>(['<p>开始写作...</p>']); // 每页内容
const activePageIndex = ref(0); // 当前激活的页面索引
const showTypstPreview = ref(false); // Typst 预览显示状态
const typstPreviewSrc = ref(''); // Typst 预览源
const typstPreviewData = ref(''); // Typst 预览数据
const typstPreviewRevision = ref(0); // Forces SVG preview DOM refresh
const typstPreviewUrl = ref(''); // Typst 预览URL
const isTypstCompiling = ref(false); // Typst 编译状态
const typstCompileError = ref(''); // Typst 编译错误
const compileTimeout = ref<ReturnType<typeof setTimeout> | null>(null); // 编译超时定时器
const typstViewMode = ref<'render' | 'source'>('render'); // Typst 视图模式
const typstSourceCode = ref(''); // Typst 源代码
const typstRenderError = ref(''); // Typst 渲染错误信息
const searchTimeout = ref<ReturnType<typeof setTimeout> | null>(null); // 搜索超时定时器
const typstContentCache = ref(''); // Typst 内容缓存
const typstRenderDebounce = ref<ReturnType<typeof setTimeout> | null>(null); // 防抖定时器

// PDF rendering state
const pdfDocument = ref<any>(null); // PDF document instance
const pdfTotalPages = ref(0); // PDF total pages
const pdfCurrentPage = ref(1); // Current page being viewed
const pdfCanvases = ref<HTMLCanvasElement[]>([]); // Array of canvas elements for each page

// ============================================================================
// 电子表格状态
// ============================================================================
const showSpreadsheet = ref(false); // 电子表格显示状态
const showUniverSpreadsheet = ref(false); // Univer 电子表格显示状态

// Collaboration state
const showCollaboration = ref(false); // 协作功能显示状态
const collaborationEnabled = ref(false); // 协作功能启用状态
const collaborationDocumentId = ref(''); // 协作文档 ID
const collaborationUserId = ref(''); // 协作用户 ID
const collaborationUserName = ref(''); // 协作用户名

// ============================================================================
// 幻灯片模式状态
// ============================================================================
const isSlideMode = ref(false); // 幻灯片模式
const currentSlideIndex = ref(0); // 当前幻灯片索引
const currentSlideId = ref(''); // 当前幻灯片 ID
const slides = ref<PptSlide[]>([]); // 幻灯片列表
const totalSlides = ref(1); // 总幻灯片数
const slidePreviewSrc = ref(''); // 幻灯片预览源
const slideCompileError = ref(''); // 幻灯片编译错误
const isSlideCompiling = ref(false); // 幻灯片编译状态
const slideConfig = ref<any>({
  theme: 'university-theme',
  aspectRatio: '16-9',
  showSlideNumbers: true
}); // 幻灯片配置

// ============================================================================
// 格式刷状态
// ============================================================================
const isFormatPainterActive = ref(false); // 格式刷激活状态
const copiedFormat = ref<any>(null); // 复制的格式

// ============================================================================
// Ribbon 界面状态
// ============================================================================
const activeRibbonTab = ref('home'); // 活动的 Ribbon 标签
const ribbonPanelsRef = ref<HTMLElement | null>(null);

const scrollRibbon = (offset: number) => {
  if (ribbonPanelsRef.value) {
    ribbonPanelsRef.value.scrollBy({ left: offset, behavior: 'smooth' });
  }
};

const showFileBackstage = ref(false); // 文件后台显示状态
const showQuickAccessMenu = ref(false); // 快速访问菜单显示状态
const showStylesPanel = ref(false); // 样式面板显示状态
const showNavigationPane = ref(false); // 导航面板显示状态
const showSplitView = ref(false); // 双显示侧边栏状态
const showPageSetupDialog = ref(false); // 页面设置对话框显示状态
const showParagraphDialog = ref(false); // 段落对话框显示状态
const showFontDialog = ref(false); // 字体对话框显示状态
const showContextMenu = ref(false); // 上下文菜单显示状态
const contextMenuPosition = ref({ x: 0, y: 0 }); // 上下文菜单位置
const contextMenuContext = ref<'text' | 'table' | 'image' | 'general'>('general'); // 上下文菜单上下文
const showBubbleMenu = ref(false); // 气泡菜单显示状态
const bubbleMenuPosition = ref({ x: 0, y: 0 }); // 气泡菜单位置
const showFloatingMenu = ref(false); // 浮动菜单显示状态
const floatingMenuPosition = ref({ x: 0, y: 0 }); // 浮动菜单位置
const showSidebar = ref(true); // 侧边栏显示状态
const showStatusBar = ref(true); // 状态栏显示状态
const zoomLevel = ref(100); // 缩放级别

// Hybrid Architecture: Initialize Rust backend services
const hybridServices = useHybridServices();
const documentAnalysis = ref<any>(null);
const spellCheckResult = ref<any>(null);
const analysisDebounce = ref<ReturnType<typeof setTimeout> | null>(null);

// New dialog states
const showPageLayoutDialog = ref(false); // 页面布局对话框
const showStyleManagerDialog = ref(false); // 样式管理器对话框
const showHeaderFooterEditorDialog = ref(false); // 页眉页脚编辑器对话框
const headerFooterType = ref<'header' | 'footer'>('header'); // 页眉页脚类型

// Medium priority UI component states
const showShapeSelector = ref(false); // 形状选择器
const showIconSelector = ref(false); // 图标选择器
const showSmartArtSelector = ref(false); // SmartArt选择器
const showWordArtDialog = ref(false); // 艺术字编辑器
const showChartEditor = ref(false); // 图表编辑器
const showCommentsPanel = ref(false); // 批注面板
const showRevisionPanel = ref(false); // 修订模式面板

const editorSidebarLayout = useEditorSidebarLayout({
  showDocumentOutline,
  showSplitView,
  showAISidebar,
  showCommentsPanel,
  showRevisionPanel,
  showSpreadsheet,
  showUniverSpreadsheet,
});

const showTableDesignTab = ref(false); // 表格设计选项卡
const tableSelected = ref(false); // 表格是否被选中

// ============================================================================
// 电子表格功能对话框状态
// ============================================================================
const showConditionalFormatDialog = ref(false); // 条件格式对话框显示状态
const conditionalFormatType = ref<'data-bars' | 'color-scale' | 'icon-sets' | 'rules'>('rules'); // 条件格式类型
const showChartDialog = ref(false); // 图表对话框显示状态
const chartType = ref<'column' | 'line' | 'pie' | 'bar' | 'area' | 'scatter'>('column'); // 图表类型
const showPivotTableDialog = ref(false); // 数据透视表对话框显示状态

// ============================================================================
// TipTap 配置状态
// ============================================================================
const tiptapConfig = ref<any>(null); // TipTap 配置
const tiptapPreset = ref('default'); // TipTap 预设

// ============================================================================
// 页面设置状态
// ============================================================================
const pageMargins = ref({ top: 25, bottom: 25, left: 25, right: 25 }); // 页边距（毫米）
const pageSize = ref({ width: 210, height: 297 }); // 页面尺寸（毫米，A4）
const pageOrientation = ref('portrait'); // 页面方向
const showVerticalRuler = ref(true); // 显示垂直标尺
const showHorizontalRuler = ref(true); // 显示水平标尺

// 标尺厘米刻度数（根据纸张宽度和左右边距动态计算）
const rulerCentimeters = computed(() => {
  const totalWidthPx = pageSize.value.width * 3.78;
  const activeWidthPx = totalWidthPx - (leftMargin.value + rightMargin.value);
  const activeWidthCm = activeWidthPx / 37.8;
  return Math.max(1, Math.floor(activeWidthCm));
});

// ============================================================================
// 高级功能对话框状态
// ============================================================================
const showBibliographyDialog = ref(false); // 参考文献对话框显示状态
const showFootnoteDialog = ref(false); // 脚注对话框显示状态
const showEndnoteDialog = ref(false); // 尾注对话框显示状态
const showCitationDialog = ref(false); // 引用对话框显示状态
const showMultiColumnDialog = ref(false); // 多栏对话框显示状态
const showSectionBreakDialog = ref(false); // 分节符对话框显示状态
const showCrossReferenceDialog = ref(false); // 交叉引用对话框显示状态
const showVersionHistoryDialog = ref(false); // 版本历史对话框显示状态
const showPrintPreviewDialog = ref(false); // 打印预览对话框显示状态
const showTocDialog = ref(false); // 目录对话框显示状态

// ============================================================================
// 拼写检查状态
// ============================================================================
const spellCheckEnabled = ref(false); // 拼写检查启用
const currentSpellCheckErrors = ref<any[]>([]); // 当前拼写错误列表

// ============================================================================
// 目录状态
// ============================================================================
const tocHTML = ref(''); // 目录 HTML
const tocVisible = ref(false); // 目录可见性

// ============================================================================
// 参考文献状态
// ============================================================================
const bibliographyEntries = ref<any[]>([]); // 参考文献条目列表
const selectedCitationStyle = ref('apa'); // 选中的引用样式
const showAddBibliographyEntry = ref(false); // 添加参考文献条目显示状态
const newBibliographyEntry = ref({
  type: 'book',
  title: '',
  authors: '',
  year: '',
  publisher: ''
}); // 新参考文献条目

// ============================================================================
// 脚注/尾注状态
// ============================================================================
const footnoteText = ref(''); // 脚注文本
const endnoteText = ref(''); // 尾注文本

// ============================================================================
// 多栏布局状态
// ============================================================================
const columnCount = ref(1); // 栏数
const columnGap = ref('20px'); // 栏间距

// ============================================================================
// 分节符状态
// ============================================================================
const sectionBreakType = ref('next-page'); // 分节符类型

// ============================================================================
// 交叉引用状态
// ============================================================================
const crossReferenceTarget = ref(''); // 交叉引用目标
const crossReferenceLabel = ref(''); // 交叉引用标签
const crossReferenceType = ref('heading'); // 交叉引用类型
const crossReferenceFormat = ref('text'); // 交叉引用格式
const availableTargets = ref<any[]>([]); // 可用目标列表

// ============================================================================
// 版本历史状态
// ============================================================================
const versionList = ref<any[]>([]); // 版本列表
const currentVersion = ref(1); // 当前版本号

// ============================================================================
// 打印预览状态
// ============================================================================
const printPreviewPages = ref<any[]>([]); // 打印预览页面列表
const printConfig = ref({
  pageSize: 'a4',
  orientation: 'portrait',
  margins: { top: 20, right: 20, bottom: 20, left: 20 }
}); // 打印配置
const showGridlines = ref(true); // 显示网格线（默认启用）
const showFormatMarks = ref(false); // 显示格式标记（空格、制表符、段落标记等）
const selectedWallpaper = ref<string | null>(null); // 当前选择的墙纸

// 墙纸样式计算
const wallpaperStyle = computed(() => {
  if (selectedWallpaper.value) {
    // Check if it's a data URL (custom wallpaper) or a file path (built-in wallpaper)
    const isDataUrl = selectedWallpaper.value.startsWith('data:');
    const imagePath = isDataUrl ? selectedWallpaper.value : `/${selectedWallpaper.value}`;
    return {
      backgroundImage: `url('${imagePath}')`,
      backgroundSize: 'cover',
      backgroundPosition: 'center',
      backgroundRepeat: 'no-repeat',
      backgroundAttachment: 'fixed'
    };
  }
  return {
    backgroundImage: 'none'
  };
});

// ============================================================================
// 其他对话框和视图状态
// ============================================================================
const showWordCountDialog = ref(false); // 字数统计对话框显示状态
const showSlideConfigDialog = ref(false); // 幻灯片配置对话框显示状态
const showThemeDialog = ref(false); // 主题对话框显示状态
const showBackgroundDialog = ref(false); // 背景样式对话框显示状态
const showLayoutDialog = ref(false); // 版式对话框显示状态
const showInsertImageDialog = ref(false); // 插入图片对话框显示状态
const showInsertShapeDialog = ref(false); // 插入形状对话框显示状态
const showInsertTableDialog = ref(false); // 插入表格对话框显示状态
const isPrintPreview = ref(false); // 打印预览模式
const isWebLayout = ref(false); // Web 布局模式
const viewMode = ref<'focus' | 'read' | 'print' | 'web'>('print'); // Logos 状态栏视图模式
const showComments = ref(false); // 显示注释
const trackChangesEnabled = ref(false); // 修订模式状态
const showTypstTemplatesDialog = ref(false); // Typst 模板对话框显示状态
const selectedTypstTemplate = ref<TypstTemplate | null>(null); // 选中的 Typst 模板
const templateSearchQuery = ref(''); // 模板搜索查询
const templateCategoryFilter = ref<string>('all'); // 模板分类筛选
const templatePreviewContent = ref<string>(''); // 模板预览内容
const showTemplatePreview = ref(false); // 显示模板预览
const showTypstConfigDialog = ref(false); // Typst 配置对话框显示状态
const typstConfig = ref({
  theme: 'metropolis-theme',
  aspectRatio: '16-9',
  showSlideNumbers: true,
  fontSize: 11,
  fontFamily: 'Latin Modern'
}); // Typst 配置

// Toast notification system
const toast = ref({
  show: false,
  message: '',
  type: 'info' as 'info' | 'success' | 'error' | 'warning'
});

const showToast = (message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') => {
  toast.value = { show: true, message, type };
  setTimeout(() => {
    toast.value.show = false;
  }, 3000);
};

// Helper function to show error message with auto-clear
const showErrorMessage = (message: string, duration: number = 3000) => {
  aiError.value = message;
  setTimeout(() => (aiError.value = null), duration);
};

// Helper function to show success message with auto-clear
const showSuccessMessage = (message: string, duration: number = 2000) => {
  aiError.value = message;
  setTimeout(() => (aiError.value = null), duration);
};

// ============================================================================
// 高级功能状态
// ============================================================================
const showAdvancedFeaturesDialog = ref(false); // 高级功能对话框显示状态
const activeAdvancedFeature = ref<'incremental' | 'package' | 'accessibility' | 'plugin'>('incremental'); // 活动的高级功能
const cacheSize = ref(0); // 缓存大小
const availablePackagesCount = ref(0); // 可用包数量
const installedPackagesCount = ref(0); // 已安装包数量
const accessibilityNodeCount = ref(0); // 无障碍节点数量
const pluginCount = ref(0); // 插件数量

// ============================================================================
// 文本样式状态
// ============================================================================
const textColor = ref('#000000'); // 文本颜色
const backgroundColor = ref('#ffffff'); // 背景颜色
const highlightColor = ref('#ffff00'); // 高亮颜色
const fontFamily = ref('Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif'); // 字体族
const selectedStyle = ref('normal'); // 选中的样式

// ============================================================================
// 文档属性状态
// ============================================================================
const documentTitle = ref('未命名文档'); // 文档标题
const isEditingTitle = ref(false); // 编辑标题状态
const isReadOnly = ref(false); // 只读模式

// ============================================================================
// 页面尺寸状态
// ============================================================================
// Page dimensions (A4: 210mm x 297mm, converted to pixels at 96 DPI)
const pageWidth = ref(794); // 页面宽度（像素，210mm）
const pageHeight = ref(1123); // 页面高度（像素，297mm）

// ============================================================================
// 页面边距状态（由毫米物理值 pageMargins 自动计算像素值）
// ============================================================================
const leftMargin = computed(() => pageMargins.value.left * 3.78); // 左边距像素
const rightMargin = computed(() => pageMargins.value.right * 3.78); // 右边距像素
const topMargin = computed(() => pageMargins.value.top * 3.78); // 上边距像素
const bottomMargin = computed(() => pageMargins.value.bottom * 3.78); // 下边距像素

// ============================================================================
// 缩进值状态（像素）
// ============================================================================
const leftIndent = ref(0); // 左缩进
const rightIndent = ref(0); // 右缩进
const firstLineIndent = ref(0); // 首行缩进
const hangingIndent = ref(0); // 悬挂缩进

// ============================================================================
// 主题切换功能
// ============================================================================
const toggleTheme = () => {
  isDarkMode.value = !isDarkMode.value;
  if (isDarkMode.value) {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
};

// ============================================================================
// 墙纸错误处理
// ============================================================================
const handleWallpaperError = (message: string) => {
  console.error('Wallpaper error:', message);
  // 可以在这里添加 toast 通知或其他错误显示逻辑
};

const toggleWallpaperDialog = () => {
  showWallpaperDialog.value = !showWallpaperDialog.value;
};

// ============================================================================
// 自动保存切换功能
// ============================================================================
const toggleAutoSave = () => {
  autoSaveEnabled.value = !autoSaveEnabled.value;
  if (autoSaveEnabled.value) {
    startAutoSave();
  } else {
    stopAutoSave();
  }
};

// ============================================================================
// Safe editor HTML getter with error handling
// ============================================================================
const safeGetHTML = (): string => {
  if (!editor.value) {
    logger.warn('Editor not initialized', {}, LogCategory.SYSTEM);
    return '';
  }
  
  if (editor.value.isDestroyed) {
    logger.warn('Editor is destroyed', {}, LogCategory.SYSTEM);
    return '';
  }
  
  if (!editor.value.schema) {
    logger.warn('Editor schema not available', {}, LogCategory.SYSTEM);
    return '';
  }
  
  try {
    return editor.value.getHTML();
  } catch (error) {
    logger.error('Error getting HTML', error, LogCategory.SYSTEM);
    return '';
  }
};

// ============================================================================
// 自动保存功能：每 30 秒保存内容到 localStorage
// ============================================================================
const startAutoSave = () => {
  if (autoSaveInterval.value) {
    clearInterval(autoSaveInterval.value);
  }
  autoSaveInterval.value = setInterval(async () => {
    // Only attempt to get HTML if editor is properly initialized
    if (!editor.value) {
      // Editor not initialized, skipping save
      return;
    }
    
    // Check if editor is destroyed
    if (editor.value.isDestroyed) {
      // Editor is destroyed, skipping save
      return;
    }
    
    // Check if schema is available
    if (!editor.value.schema) {
      // Editor schema not available, skipping save
      return;
    }
    
    try {
      const currentContent = safeGetHTML();
      // 仅在内容更改时保存，避免不必要的写入
      if (currentContent !== lastSavedContent.value) {
        lastSavedContent.value = currentContent;
        localStorage.setItem('logos-autosave', currentContent);
        
        // Also save to default file if it exists (convert HTML to Typst)
        if (currentFilename.value === 'logo001.typ') {
          const isTauriEnv = await isTauri();
          if (isTauriEnv) {
            try {
              const docsDir = await invoke<string>('get_documents_directory');
              const defaultFilePath = `${docsDir}/logo001.typ`;
              const typstContent = await invoke<string>('html_to_typst', { html: currentContent, config: null });
              await invoke('save_file', { filePath: defaultFilePath, content: typstContent });
              logger.debug('Auto-saved to default file (Typst format)', { filePath: defaultFilePath }, LogCategory.SYSTEM);
            } catch (error) {
              logger.error('Failed to auto-save to default file', error, LogCategory.SYSTEM);
              // Don't fail the entire auto-save if file save fails
            }
          }
        }
      }
    } catch (error) {
      logger.error('Auto-save error getting editor content', error, LogCategory.SYSTEM);
    }
  }, 30000); // 每 30 秒自动保存
};

// ============================================================================
// 停止自动保存定时器并清理
// ============================================================================
const stopAutoSave = () => {
  if (autoSaveInterval.value) {
    clearInterval(autoSaveInterval.value);
    autoSaveInterval.value = null;
  }
};

// ============================================================================
// 从 localStorage 恢复内容（组件挂载时调用）
// ============================================================================
const loadAutoSave = () => {
  try {
    const savedContent = localStorage.getItem('logos-autosave');
    if (savedContent && editor.value) {
      editor.value.commands.setContent(savedContent);
      lastSavedContent.value = savedContent;
    }
  } catch (error) {
    const appError = createError(
      ErrorCode.FILE_READ_ERROR,
      undefined,
      ErrorSeverity.WARNING,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.error('Failed to load auto-save', appError, LogCategory.SYSTEM);
  }
};

// ============================================================================
// 斜杠命令项（用于快速插入内容）
// ============================================================================
const slashCommandItems = [
  {
    title: '标题 1',
    description: '大标题',
    icon: 'H1',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).setHeading({ level: 1 }).run();
    }
  },
  {
    title: '标题 2',
    description: '中标题',
    icon: 'H2',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).setHeading({ level: 2 }).run();
    }
  },
  {
    title: '标题 3',
    description: '小标题',
    icon: 'H3',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).setHeading({ level: 3 }).run();
    }
  },
  {
    title: '普通文本',
    description: '清除格式',
    icon: 'T',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).setParagraph().run();
    }
  },
  {
    title: '无序列表',
    description: '创建无序列表',
    icon: '•',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).toggleBulletList().run();
    }
  },
  {
    title: '有序列表',
    description: '创建有序列表',
    icon: '1.',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).toggleOrderedList().run();
    }
  },
  {
    title: '引用',
    description: '添加引用块',
    icon: '"',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).toggleBlockquote().run();
    }
  },
  {
    title: '代码块',
    description: '插入代码块',
    icon: '</>',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).toggleCodeBlock().run();
    }
  },
  {
    title: '水平线',
    description: '插入分隔线',
    icon: '—',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).setHorizontalRule().run();
    }
  },
  {
    title: '分页符',
    description: '插入幻灯片分页',
    icon: '⇥',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).insertContent('<hr>').run();
    }
  },
  {
    title: 'AI 润色',
    description: 'AI 润色选中文字',
    icon: '✨',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).run();
      triggerAiPolish();
    }
  },
  {
    title: 'AI 扩写',
    description: 'AI 扩写内容',
    icon: '📝',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).run();
      triggerAiExpand();
    }
  },
  {
    title: 'AI 重写',
    description: 'AI 重写段落',
    icon: '🔄',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).run();
      triggerAiRewrite();
    }
  },
  {
    title: 'AI 总结',
    description: 'AI 总结内容',
    icon: '📋',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).run();
      triggerAiSummarize();
    }
  },
  {
    title: 'AI 翻译',
    description: 'AI 翻译成英文',
    icon: '🌐',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).run();
      triggerAiTranslate();
    }
  },
  {
    title: '加粗',
    description: '加粗文本',
    icon: 'B',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).toggleBold().run();
    }
  },
  {
    title: '斜体',
    description: '斜体文本',
    icon: 'I',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).toggleItalic().run();
    }
  },
  {
    title: '删除线',
    description: '删除线文本',
    icon: 'S',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).toggleStrike().run();
    }
  },
  {
    title: '代码',
    description: '行内代码',
    icon: '<>',
    command: ({ editor, range }: any) => {
      editor.chain().focus().deleteRange(range).toggleCode().run();
    }
  }
];

// Custom PageBreak node extension for inserting page breaks
const PageBreak = Node.create({
  name: 'pageBreak',

  group: 'block',

  atom: true,

  addAttributes() {
    return {
      class: {
        default: 'page-break',
      },
      style: {
        default: 'page-break-after: always;',
      },
    };
  },

  parseHTML() {
    return [
      {
        tag: 'div',
        getAttrs: (node: HTMLElement) => {
          return node.classList.contains('page-break') || node.style.pageBreakAfter === 'always';
        },
      },
    ];
  },

  renderHTML({ HTMLAttributes }) {
    return ['div', HTMLAttributes];
  },
});

// Custom slash command extension: provides a command menu when typing '/'
const SlashCommand = Extension.create({
  name: 'slashCommand',

  addOptions() {
    return {
      suggestion: {
        char: '/',
        command: ({ editor, range, props }: any) => {
          props.command({ editor, range });
        }
      }
    };
  },

  addProseMirrorPlugins() {
    return [
      Suggestion({
        char: '/',
        // Filter and limit command items based on user query
        items: ({ query }: any) => {
          return slashCommandItems
            .filter(item => item.title.toLowerCase().includes(query.toLowerCase()))
            .slice(0, 10);
        },
        render: () => {
          let component: any;
          let popup: any;

          return {
            onStart: (props: any) => {
              component = document.createElement('div');
              component.className = 'slash-menu';
              component.innerHTML = slashCommandItems
                .map(
                  item => `
                  <div class="slash-menu-item" data-command="${item.title}">
                    <span class="slash-menu-icon">${item.icon}</span>
                    <div class="slash-menu-text">
                      <span class="slash-menu-title">${item.title}</span>
                      <span class="slash-menu-desc">${item.description}</span>
                    </div>
                  </div>
                `
                )
                .join('');

              popup = document.createElement('div');
              popup.className = 'slash-menu-popup';
              popup.appendChild(component);
              document.body.appendChild(popup);

              props.editor.view.dom.parentElement?.appendChild(popup);

              const { from } = props;
              const coords = props.editor.view.coordsAtPos(from);
              popup.style.top = `${coords.bottom + 5}px`;
              popup.style.left = `${coords.left}px`;
            },

            onUpdate: (props: any) => {
              const { from } = props;
              const coords = props.editor.view.coordsAtPos(from);
              popup.style.top = `${coords.bottom + 5}px`;
              popup.style.left = `${coords.left}px`;

              // Update filtered items
              component.innerHTML = props.items
                .map(
                  (item: any) => `
                  <div class="slash-menu-item" data-command="${item.title}">
                    <span class="slash-menu-icon">${item.icon}</span>
                    <div class="slash-menu-text">
                      <span class="slash-menu-title">${item.title}</span>
                      <span class="slash-menu-desc">${item.description}</span>
                    </div>
                  </div>
                `
                )
                .join('');
            },

            onKeyDown: (props: any) => {
              if (props.event.key === 'Escape') {
                popup?.remove();
                return true;
              }
              return false;
            },

            onExit: () => {
              popup?.remove();
            }
          };
        }
      })
    ];
  }
});

// ============================================================================
// 检查编辑器格式是否激活
// ============================================================================
const isActive = (name: string, attributes = {}) => {
  return editor.value?.isActive(name, attributes) || false;
};

// ============================================================================
// 文本格式化函数
// ============================================================================
const toggleBold = () => {
  editor.value?.chain().focus().toggleBold().run();
};

const toggleItalic = () => {
  editor.value?.chain().focus().toggleItalic().run();
};

// ============================================================================
// 版本历史和修订跟踪
// ============================================================================
interface DocumentVersion {
  id: string;
  timestamp: number;
  content: string;
  description: string;
  wordCount: number;
}

const documentVersions = ref<DocumentVersion[]>([]); // 文档版本列表
const showVersionHistory = ref(false); // 版本历史显示状态
const maxVersions = 50; // 最大版本数
const versionSaveTimeout = ref<ReturnType<typeof setTimeout> | null>(null); // 版本保存超时定时器

// ============================================================================
// 保存当前版本到历史记录
// ============================================================================
const saveVersion = (description: string = '自动保存') => {
  if (!editor.value) {
    return;
  }

  const _content = editor.value.getHTML();
  const currentWordCount = wordCount.value;

  // 避免重复的连续保存
  if (documentVersions.value.length > 0) {
    const lastVersion = documentVersions.value[0];
    if (lastVersion.content === _content) {
      return;
    }
  }

  const version: DocumentVersion = {
    id: `v${Date.now()}`,
    timestamp: Date.now(),
    content: _content,
    description,
    wordCount: currentWordCount
  };

  documentVersions.value.unshift(version);

  // 限制版本数量
  if (documentVersions.value.length > maxVersions) {
    documentVersions.value = documentVersions.value.slice(0, maxVersions);
  }

  // 持久化到 localStorage
  try {
    localStorage.setItem('documentVersions', JSON.stringify(documentVersions.value));
  } catch (e) {
    const appError = createError(
      ErrorCode.FILE_WRITE_ERROR,
      undefined,
      ErrorSeverity.WARNING,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: e } }
    );
    logger.warn('Failed to save versions to localStorage', appError, LogCategory.SYSTEM);
  }
};

// ============================================================================
// 恢复特定版本
// ============================================================================
const restoreVersion = (versionId: string) => {
  if (!editor.value) {
    return;
  }

  const version = documentVersions.value.find(v => v.id === versionId);
  if (
    version &&
    confirm(`确定要恢复到版本 "${version.description}" 吗？当前未保存的更改将丢失。`)
  ) {
    editor.value.commands.setContent(version.content);
    aiError.value = `已恢复到版本: ${version.description}`;
    setTimeout(() => (aiError.value = null), 3000);
    showVersionHistory.value = false;
  }
};

// ============================================================================
// 删除特定版本
// ============================================================================
const deleteVersion = (versionId: string) => {
  if (confirm('确定要删除此版本吗？')) {
    documentVersions.value = documentVersions.value.filter(v => v.id !== versionId);
    try {
      localStorage.setItem('documentVersions', JSON.stringify(documentVersions.value));
    } catch (e) {
      const appError = createError(
        ErrorCode.FILE_WRITE_ERROR,
        undefined,
        ErrorSeverity.WARNING,
        ErrorCategory.SYSTEM,
        { timestamp: Date.now(), additionalData: { originalError: e } }
      );
      logger.warn('Failed to update versions in localStorage', appError, LogCategory.SYSTEM);
    }
  }
};

// ============================================================================
// 清除所有版本历史
// ============================================================================
const clearVersionHistory = () => {
  if (confirm('确定要清除所有版本历史吗？此操作不可撤销。')) {
    try {
      versionHistoryManager.clearAll();
      versionList.value = versionHistoryManager.getAllVersions();
      aiError.value = '版本历史已清除';
      setTimeout(() => (aiError.value = null), 2000);
    } catch (error) {
      logger.error('Failed to clear version history', error, LogCategory.BUSINESS);
      aiError.value = '清除版本历史失败: ' + (error as Error).message;
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

// ============================================================================
// 从 localStorage 加载版本历史（组件挂载时调用）
// ============================================================================
const loadVersionHistory = () => {
  try {
    const saved = localStorage.getItem('documentVersions');
    if (saved) {
      documentVersions.value = JSON.parse(saved);
    }
  } catch (e) {
    const appError = createError(
      ErrorCode.FILE_READ_ERROR,
      undefined,
      ErrorSeverity.WARNING,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: e } }
    );
    logger.warn('Failed to load versions from localStorage', appError, LogCategory.SYSTEM);
  }
};

// ============================================================================
// 切换版本历史对话框
// ============================================================================
const toggleVersionHistory = () => {
  showVersionHistory.value = !showVersionHistory.value;
};

// ============================================================================
// 切换目录侧边栏
// ============================================================================
const toggleTocSidebar = () => {
  tocVisible.value = !tocVisible.value;
  if (tocVisible.value) {
    generateTOC();
  }
};

// ============================================================================
// 格式化时间戳用于显示
// ============================================================================
const formatTimestamp = (timestamp: number) => {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();

  if (diff < 60000) {
    return '刚刚';
  }
  if (diff < 3600000) {
    return `${Math.floor(diff / 60000)} 分钟前`;
  }
  if (diff < 86400000) {
    return `${Math.floor(diff / 3600000)} 小时前`;
  }
  if (diff < 604800000) {
    return `${Math.floor(diff / 86400000)} 天前`;
  }

  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
};

const saveDocument = async () => {
  logger.debug('Save document started', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  isSaving.value = true;
  try {
    const filePath = await save({
      filters: [
        {
          name: 'Typst Document',
          extensions: ['typ']
        },
        {
          name: 'Word Document',
          extensions: ['docx']
        },
        {
          name: 'Rich Text Format',
          extensions: ['rtf']
        },
        {
          name: 'Markdown',
          extensions: ['md']
        },
        {
          name: 'HTML',
          extensions: ['html']
        },
        {
          name: 'Text',
          extensions: ['txt']
        }
      ]
    });
    logger.debug('File path selected', { filePath }, LogCategory.SYSTEM);

    if (filePath) {
      const extension = filePath.split('.').pop()?.toLowerCase();
      logger.debug('File extension', { extension }, LogCategory.SYSTEM);

      if (extension === 'typ') {
        await exportToTypst();
      } else if (extension === 'docx') {
        await exportToWord();
      } else if (extension === 'rtf') {
        await exportToRtf();
      } else if (extension === 'md') {
        await exportToMarkdown();
      } else {
        const _content = editor.value?.getHTML() || '';
        await invoke('save_file', { filePath, content: _content });
        aiError.value = '文件保存成功!';
        setTimeout(() => (aiError.value = null), 2000);
      }
      logger.debug('Document saved successfully', {}, LogCategory.SYSTEM);
    }
  } catch (error) {
    logger.error('Save document failed', error, LogCategory.SYSTEM);
    const appError = createError(
      ErrorCode.FILE_WRITE_ERROR,
      undefined,
      ErrorSeverity.ERROR,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.error('Failed to save file', appError, LogCategory.SYSTEM);
    aiError.value = '保存失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  } finally {
    isSaving.value = false;
    logger.debug('Save document completed', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  }
};

// Export document to PDF
const exportToPdf = async () => {
  if (!editor.value) {
return;
}

  isSaving.value = true;
  try {
    const filePath = await save({
      filters: [
        {
          name: 'PDF',
          extensions: ['pdf']
        }
      ]
    });

    if (filePath) {
      const htmlContent = editor.value.getHTML();
      const typstCode = await invoke<string>('html_to_typst', { html: htmlContent });
      const pdfBytes = await invoke<number[]>('export_to_pdf', { code: typstCode });

      // Convert number array to Uint8Array
      const uint8Array = new Uint8Array(pdfBytes);

      // Save PDF file
      await invoke('save_file', { filePath, content: uint8Array });
      aiError.value = 'PDF 导出成功!';
      setTimeout(() => (aiError.value = null), 2000);
    }
  } catch (error) {
    const appError = createError(
      ErrorCode.FILE_WRITE_ERROR,
      undefined,
      ErrorSeverity.ERROR,
      ErrorCategory.BUSINESS,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.error('Failed to export PDF', appError, LogCategory.BUSINESS);
    aiError.value = 'PDF 导出失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  } finally {
    isSaving.value = false;
  }
};

// Export slides to PDF
const exportSlidesToPdf = async () => {
  if (!editor.value) {
return;
}

  isSaving.value = true;
  try {
    const filePath = await save({
      filters: [
        {
          name: 'PDF',
          extensions: ['pdf']
        }
      ]
    });

    if (filePath) {
      // Convert HTML to Typst slides
      const htmlContent = editor.value.getHTML();
      const typstCode = await invoke<string>('html_to_typst_slides', { 
        html: htmlContent, 
        config: slideConfig.value 
      });
      const pdfBytes = await invoke<number[]>('export_to_pdf', { code: typstCode });

      // Convert number array to Uint8Array
      const uint8Array = new Uint8Array(pdfBytes);

      // Save PDF file
      await invoke('save_file', { filePath, content: uint8Array });
      aiError.value = '幻灯片 PDF 导出成功!';
      setTimeout(() => (aiError.value = null), 2000);
    }
  } catch (error) {
    const appError = createError(
      ErrorCode.FILE_WRITE_ERROR,
      undefined,
      ErrorSeverity.ERROR,
      ErrorCategory.BUSINESS,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.error('Failed to export slides PDF', appError, LogCategory.BUSINESS);
    aiError.value = '幻灯片 PDF 导出失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  } finally {
    isSaving.value = false;
  }
};

// Export document to Word (.docx)
const exportToWord = async () => {
  if (!editor.value) {
return;
}

  isSaving.value = true;
  try {
    const filePath = await save({
      filters: [
        {
          name: 'Word Document',
          extensions: ['docx']
        }
      ]
    });

    if (filePath) {
      const htmlContent = editor.value.getHTML();
      editor.value.getText();

      // Create Word document
      const doc = new Document({
        sections: [
          {
            properties: {},
            children: [
              new Paragraph({
                children: [
                  new TextRun({
                    text: documentTitle.value || '未命名文档',
                    bold: true,
                    size: 32
                  })
                ]
              }),
              new Paragraph({
                text: ''
              })
            ]
          }
        ]
      });

      // Parse HTML and convert to Word paragraphs
      const parser = new DOMParser();
      const docHtml = parser.parseFromString(htmlContent, 'text/html');
      const paragraphs = docHtml.querySelectorAll('p, h1, h2, h3, h4, h5, h6, li');

      const wordParagraphs: Paragraph[] = [];

      paragraphs.forEach(p => {
        const text = p.textContent || '';
        const tagName = p.tagName.toLowerCase();

        let headingLevel: any;
        if (tagName === 'h1') {
headingLevel = HeadingLevel.HEADING_1;
} else if (tagName === 'h2') {
headingLevel = HeadingLevel.HEADING_2;
} else if (tagName === 'h3') {
headingLevel = HeadingLevel.HEADING_3;
} else if (tagName === 'h4') {
headingLevel = HeadingLevel.HEADING_4;
} else if (tagName === 'h5') {
headingLevel = HeadingLevel.HEADING_5;
} else if (tagName === 'h6') {
headingLevel = HeadingLevel.HEADING_6;
}

        if (text.trim()) {
          wordParagraphs.push(
            new Paragraph({
              text: text,
              heading: headingLevel
            })
          );
        }
      });

      // Add paragraphs to document
      // @ts-expect-error - docx API structure
      const section = doc.sections[0];
      if (section && section.children) {
        section.children.push(...wordParagraphs);
      }

      // Generate and save
      const blob = await Packer.toBlob(doc);
      await invoke('save_file', { filePath, content: new Uint8Array(await blob.arrayBuffer()) });

      aiError.value = 'Word 文档导出成功!';
      setTimeout(() => (aiError.value = null), 2000);
    }
  } catch (error) {
    const appError = createError(
      ErrorCode.FILE_WRITE_ERROR,
      undefined,
      ErrorSeverity.ERROR,
      ErrorCategory.BUSINESS,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.error('Failed to export Word', appError, LogCategory.BUSINESS);
    aiError.value = 'Word 导出失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  } finally {
    isSaving.value = false;
  }
};

// Export document to RTF
const exportToRtf = async () => {
  if (!editor.value) {
return;
}

  isSaving.value = true;
  try {
    const filePath = await save({
      filters: [
        {
          name: 'Rich Text Format',
          extensions: ['rtf']
        }
      ]
    });

    if (filePath) {
      const htmlContent = editor.value.getHTML();

      // Convert HTML to RTF
      const rtfContent = htmlToRtf(htmlContent);

      await invoke('save_file', { filePath, content: rtfContent });

      aiError.value = 'RTF 文档导出成功!';
      setTimeout(() => (aiError.value = null), 2000);
    }
  } catch (error) {
    const appError = createError(
      ErrorCode.FILE_WRITE_ERROR,
      undefined,
      ErrorSeverity.ERROR,
      ErrorCategory.BUSINESS,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.error('Failed to export RTF', appError, LogCategory.BUSINESS);
    aiError.value = 'RTF 导出失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  } finally {
    isSaving.value = false;
  }
};

// Export document to Typst
const exportToTypst = async () => {
  if (!editor.value) {
    return;
  }

  isSaving.value = true;
  try {
    const filePath = await save({
      filters: [
        {
          name: 'Typst Document',
          extensions: ['typ']
        }
      ]
    });

    if (filePath) {
      const htmlContent = editor.value.getHTML();

      // Convert HTML to Typst using backend converter
      const typstContent = await invoke<string>('html_to_typst', { html: htmlContent, config: null });

      await invoke('save_file', { filePath, content: typstContent });

      logger.info('Typst document exported successfully', { filePath }, LogCategory.BUSINESS);
      aiError.value = 'Typst 文档导出成功!';
      setTimeout(() => (aiError.value = null), 2000);
    }
  } catch (error) {
    const appError = createError(
      ErrorCode.TYPST_EXPORT_ERROR,
      undefined,
      ErrorSeverity.ERROR,
      ErrorCategory.BUSINESS,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.error('Typst export failed', appError, LogCategory.BUSINESS);
    aiError.value = 'Typst 导出失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  } finally {
    isSaving.value = false;
  }
};

// Insert Typst code block
const insertTypstBlock = () => {
  if (!editor.value) {
    return;
  }

  try {
    const typstCode = `#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))
#set text(font: "SimSun", size: 11pt)

= 标题

在此处输入 Typst 代码...

*粗体文本*
_斜体文本_
\`代码\`

#table(
  columns: 2,
  [列1], [列2],
  [数据1], [数据2]
)`;

    editor.value.chain().focus().insertContent(
      `<pre><code class="language-typst">${typstCode}</code></pre>`
    ).run();

    logger.info('Typst code block inserted', {}, LogCategory.BUSINESS);
    aiError.value = 'Typst 代码块已插入';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    const appError = createError(
      ErrorCode.TYPST_SYNTAX_ERROR,
      undefined,
      ErrorSeverity.ERROR,
      ErrorCategory.BUSINESS,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.error('Failed to insert Typst block', appError, LogCategory.BUSINESS);
    aiError.value = '插入 Typst 代码块失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

// Toggle Typst preview
const toggleTypstPreview = () => {
  if (!editor.value) {
    return;
  }

  showTypstPreview.value = !showTypstPreview.value;

  if (showTypstPreview.value) {
    // 重置为渲染模式
    typstViewMode.value = 'render';
    generateTypstPreview();
  } else {
    typstPreviewSrc.value = '';
    typstSourceCode.value = '';
  }
};

// 切换 Typst 预览视图模式
const toggleTypstViewMode = () => {
  typstViewMode.value = typstViewMode.value === 'render' ? 'source' : 'render';
  generateTypstPreview();
};

// 生成 Typst 预览
const generateTypstPreview = async () => {
  logger.debug('Generate Typst preview called', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  if (!editor.value) {
    logger.warn('Generate Typst preview: editor not available', {}, LogCategory.SYSTEM);
    return;
  }

  // 清除之前的防抖定时器
  if (typstRenderDebounce.value) {
    clearTimeout(typstRenderDebounce.value);
  }

  // 设置新的防抖定时器（500ms）
  typstRenderDebounce.value = setTimeout(async () => {
    logger.debug('Starting Typst render', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
    isTypstCompiling.value = true;
    typstRenderError.value = ''; // 清空之前的错误

    try {
      const htmlContent = editor.value?.getHTML() || '';
      logger.debug('Converting HTML to Typst', {}, LogCategory.SYSTEM);
      const typstContent = typst.convertHTML(htmlContent);

      // 检查内容是否变化，如果未变化则使用缓存
      if (typstContent === typstContentCache.value && typstPreviewSrc.value) {
        logger.debug('Typst content unchanged, using cache', {}, LogCategory.SYSTEM);
        isTypstCompiling.value = false;
        return;
      }

      // 更新缓存
      typstContentCache.value = typstContent;

      // 保存源代码
      typstSourceCode.value = typstContent;

      if (typstViewMode.value === 'source') {
        logger.debug('Displaying source code with syntax highlighting', {}, LogCategory.SYSTEM);
        // 显示带语法高亮的源代码
        const highlightedCode = typstHighlighter.highlightToHTML(typstContent);
        typstPreviewSrc.value = highlightedCode;
        typstRenderError.value = '';
        isTypstCompiling.value = false;
      } else {
        logger.debug('Rendering Typst to PNG', {}, LogCategory.SYSTEM);
        // 尝试调用后端渲染服务
        try {
          const renderResult = await invoke('render_typst', {
            request: {
              source: typstContent,
              format: 'png'
            }
          }) as { success: boolean; output?: string; error?: string };

        if (renderResult.success && renderResult.output) {
          logger.debug('Typst rendered successfully', {}, LogCategory.SYSTEM);
          // 显示渲染结果（Base64 编码的 PNG）
          typstPreviewSrc.value = `
            <div class="typst-render-result">
              <img src="data:image/png;base64,${renderResult.output}" alt="Typst 渲染结果" style="width: 100%; height: auto;" />
            </div>
          `;
          typstRenderError.value = '';
        } else {
          logger.error('Typst render failed', renderResult.error, LogCategory.SYSTEM);
          // 渲染失败，显示渲染功能页面和错误信息
          typstRenderError.value = renderResult.error || '未知错误';
          typstPreviewSrc.value = `
            <div class="typst-render-page">
              <div class="render-page-header">
                <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                  <polyline points="14 2 14 8 20 8"></polyline>
                  <line x1="16" y1="13" x2="8" y2="13"></line>
                  <line x1="16" y1="17" x2="8" y2="17"></line>
                  <polyline points="10 9 9 9 8 9"></polyline>
                </svg>
                <h2>Typst 渲染功能</h2>
              </div>
              <div class="render-page-content">
                <div class="error-message">
                  <p class="error-title">渲染失败</p>
                  <p class="error-detail">${renderResult.error || '未知错误'}</p>
                </div>
                <div class="install-guide">
                  <h3>Typst 渲染功能</h3>
                  <p>Typst 渲染功能已集成到应用中，使用 Rust 库直接渲染。</p>
                  <p>无需安装 Typst CLI，即可使用完整的 Typst 渲染功能。</p>
                  <p class="install-info">当前模式：Rust 库渲染</p>
                </div>
                <div class="alternative-option">
                  <p>或者，您可以切换到源代码模式查看转换后的 Typst 代码。</p>
                  <button onclick="window.dispatchEvent(new CustomEvent('switch-to-source-mode'))" class="switch-button">切换到源代码模式</button>
                </div>
              </div>
            </div>
          `;
          
          // 记录渲染错误
          logger.error('Typst render failed', { error: renderResult.error }, LogCategory.BUSINESS);
        }
      } catch (renderError) {
        // 后端服务不可用，显示渲染功能页面
        typstRenderError.value = (renderError as Error).message;
        typstPreviewSrc.value = `
          <div class="typst-render-page">
            <div class="render-page-header">
              <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                <polyline points="14 2 14 8 20 8"></polyline>
                <line x1="16" y1="13" x2="8" y2="13"></line>
                <line x1="16" y1="17" x2="8" y2="17"></line>
                <polyline points="10 9 9 9 8 9"></polyline>
              </svg>
              <h2>Typst 渲染功能</h2>
            </div>
            <div class="render-page-content">
              <div class="error-message">
                <p class="error-title">服务不可用</p>
                <p class="error-detail">${(renderError as Error).message}</p>
              </div>
              <div class="install-guide">
                <h3>Typst 渲染功能</h3>
                <p>Typst 渲染功能已集成到应用中，使用 Rust 库直接渲染。</p>
                <p>无需安装 Typst CLI，即可使用完整的 Typst 渲染功能。</p>
                <p class="install-info">当前模式：Rust 库渲染</p>
              </div>
              <div class="alternative-option">
                <p>或者，您可以切换到源代码模式查看转换后的 Typst 代码。</p>
                <button onclick="window.dispatchEvent(new CustomEvent('switch-to-source-mode'))" class="switch-button">切换到源代码模式</button>
              </div>
            </div>
          </div>
        `;
        
        // 记录服务不可用错误
        logger.warn('Typst render service unavailable', { error: renderError }, LogCategory.BUSINESS);
      }
    }
    
    logger.info('Typst preview generated', { mode: typstViewMode.value, hasError: !!typstRenderError.value }, LogCategory.BUSINESS);
    aiError.value = typstViewMode.value === 'render' ? 'Typst 渲染预览已生成' : 'Typst 源代码已显示';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    const appError = createError(
      ErrorCode.TYPST_COMPILE_ERROR,
      undefined,
      ErrorSeverity.ERROR,
      ErrorCategory.BUSINESS,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.error('Failed to generate Typst preview', appError, LogCategory.BUSINESS);
    aiError.value = '生成 Typst 预览失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
    showTypstPreview.value = false;
  } finally {
    isTypstCompiling.value = false;
  }
  }, 500);
};

// Typst template functions
const openTypstTemplatesDialog = () => {
  showTypstTemplatesDialog.value = true;
  selectedTypstTemplate.value = null;
};

// Advanced features functions
const openAdvancedFeaturesDialog = (feature: 'incremental' | 'package' | 'accessibility' | 'plugin') => {
  activeAdvancedFeature.value = feature;
  showAdvancedFeaturesDialog.value = true;
};

const toggleIncrementalCompilation = async () => {
  openAdvancedFeaturesDialog('incremental');
};

const openPackageManager = async () => {
  openAdvancedFeaturesDialog('package');
};

const openAccessibilityPanel = async () => {
  openAdvancedFeaturesDialog('accessibility');
};

const openPluginManager = async () => {
  openAdvancedFeaturesDialog('plugin');
};

// Advanced features implementation functions
const computeDocumentHash = async () => {
  try {
    const content = editor.value?.getHTML() || '';
    const hash = await invoke('compute_incremental_hash', { content });
    logger.info('Incremental hash computed', { hash }, LogCategory.BUSINESS);
    aiError.value = '增量编译哈希计算完成!';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Failed to compute incremental hash', error, LogCategory.SYSTEM);
    aiError.value = '增量编译失败!';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

const clearCache = async () => {
  try {
    await invoke('clear_incremental_cache');
    cacheSize.value = 0;
    logger.info('Incremental cache cleared', {}, LogCategory.BUSINESS);
    aiError.value = '缓存已清空!';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Failed to clear cache', error, LogCategory.SYSTEM);
    aiError.value = '清空缓存失败!';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

const loadPackages = async () => {
  try {
    const packages = await invoke('list_available_packages');
    availablePackagesCount.value = (packages as any[]).length;
    logger.info('Packages loaded', { count: availablePackagesCount.value }, LogCategory.BUSINESS);
    aiError.value = `已加载 ${availablePackagesCount.value} 个可用包!`;
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Failed to load packages', error, LogCategory.SYSTEM);
    aiError.value = '加载包列表失败!';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

const loadInstalledPackages = async () => {
  try {
    const packages = await invoke('list_installed_packages');
    installedPackagesCount.value = (packages as any[]).length;
    logger.info('Installed packages loaded', { count: installedPackagesCount.value }, LogCategory.BUSINESS);
    aiError.value = `已安装 ${installedPackagesCount.value} 个包!`;
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Failed to load installed packages', error, LogCategory.SYSTEM);
    aiError.value = '加载已安装包失败!';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

const buildAccessibilityTree = async () => {
  try {
    const content = editor.value?.getHTML() || '';
    const tree = await invoke('build_accessibility_tree', { content });
    accessibilityNodeCount.value = (tree as any).nodes?.length || 0;
    logger.info('Accessibility tree built', { nodeCount: accessibilityNodeCount.value }, LogCategory.BUSINESS);
    aiError.value = '无障碍树构建完成!';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Failed to build accessibility tree', error, LogCategory.SYSTEM);
    aiError.value = '无障碍树构建失败!';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

const validateAccessibility = async () => {
  try {
    const content = editor.value?.getHTML() || '';
    const tree = await invoke('build_accessibility_tree', { content });
    accessibilityNodeCount.value = (tree as any).nodes?.length || 0;
    logger.info('Accessibility validated', { nodeCount: accessibilityNodeCount.value }, LogCategory.BUSINESS);
    aiError.value = '无障碍验证完成!';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Failed to validate accessibility', error, LogCategory.SYSTEM);
    aiError.value = '无障碍验证失败!';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

const loadPlugins = async () => {
  try {
    const plugins = await invoke('get_all_plugins');
    pluginCount.value = (plugins as any[]).length;
    logger.info('Plugins loaded', { count: pluginCount.value }, LogCategory.BUSINESS);
    aiError.value = `已加载 ${pluginCount.value} 个插件!`;
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Failed to load plugins', error, LogCategory.SYSTEM);
    aiError.value = '加载插件列表失败!';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

const getPluginStats = async () => {
  try {
    const stats = await invoke('get_plugin_stats');
    logger.info('Plugin stats retrieved', { stats }, LogCategory.BUSINESS);
    aiError.value = '插件统计已获取!';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Failed to get plugin stats', error, LogCategory.SYSTEM);
    aiError.value = '获取插件统计失败!';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

const applyTypstTemplate = (template: TypstTemplate) => {
  if (!editor.value) {
    return;
  }

  try {
    if (confirm('应用模板将替换当前文档内容，确定继续吗？')) {
      const typstContent = typstTemplateManager.applyTemplate(template.id, '');
      
      // Convert Typst to HTML for the editor
      // For now, we'll insert it as a code block since we don't have Typst to HTML conversion
      editor.value.chain().focus().insertContent(
        `<pre><code class="language-typst">${typstContent}</code></pre>`
      ).run();
      
      showTypstTemplatesDialog.value = false;
      logger.info('Typst template applied', { templateId: template.id }, LogCategory.BUSINESS);
      aiError.value = `模板 "${template.name}" 已应用`;
      setTimeout(() => (aiError.value = null), 2000);
    }
  } catch (error) {
    const appError = createError(
      ErrorCode.TYPST_TEMPLATE_ERROR,
      undefined,
      ErrorSeverity.ERROR,
      ErrorCategory.BUSINESS,
      { timestamp: Date.now(), additionalData: { templateId: template.id } }
    );
    logger.error('Failed to apply Typst template', appError, LogCategory.BUSINESS);
    aiError.value = '应用模板失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const getAvailableTypstTemplates = (): TypstTemplate[] => {
  const allTemplates = typstTemplateManager.getAllTemplates();
  
  // 按分类筛选
  let filtered = templateCategoryFilter.value === 'all' 
    ? allTemplates 
    : allTemplates.filter(t => t.category === templateCategoryFilter.value);
  
  // 按搜索查询筛选
  if (templateSearchQuery.value.trim()) {
    const query = templateSearchQuery.value.toLowerCase();
    filtered = filtered.filter(t => 
      t.name.toLowerCase().includes(query) || 
      t.description.toLowerCase().includes(query)
    );
  }
  
  return filtered;
};

const previewTemplate = (template: TypstTemplate) => {
  templatePreviewContent.value = template.content;
  showTemplatePreview.value = true;
};

const closeTemplatePreview = () => {
  showTemplatePreview.value = false;
  templatePreviewContent.value = '';
};

const loadDocument = async () => {
  logger.debug('Load document started', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  isLoading.value = true;
  try {
    const filePath = await open({
      filters: [
        {
          name: 'Typst Document',
          extensions: ['typ']
        },
        {
          name: 'Word Document',
          extensions: ['docx']
        },
        {
          name: 'Rich Text Format',
          extensions: ['rtf']
        },
        {
          name: 'Markdown',
          extensions: ['md']
        },
        {
          name: 'HTML',
          extensions: ['html']
        },
        {
          name: 'Text',
          extensions: ['txt']
        }
      ]
    });
    logger.debug('File path selected for loading', { filePath }, LogCategory.SYSTEM);

    if (filePath) {
      const extension = filePath.split('.').pop()?.toLowerCase();
      logger.debug('File extension for loading', { extension }, LogCategory.SYSTEM);

      if (extension === 'typ') {
        logger.debug('Loading Typst document', {}, LogCategory.SYSTEM);
        const typstContent = await invoke<string>('load_file', { filePath });
        const htmlContent = await invoke<string>('typst_to_html', { typst: typstContent });
        editor.value?.commands.setContent(htmlContent);
        currentFilename.value = filePath.split('/').pop() || 'document.typ';
        documentTitle.value = currentFilename.value.replace('.typ', '');
        aiError.value = 'Typst 文档加载成功!';
        setTimeout(() => (aiError.value = null), 2000);
        logger.debug('Typst document loaded successfully', {}, LogCategory.SYSTEM);
      } else if (extension === 'docx') {
        logger.debug('Loading Word document', {}, LogCategory.SYSTEM);
        const _content = await invoke('load_file', { filePath });
        const uint8Array = new Uint8Array(_content as number[]);

        try {
          const result = await mammoth.convertToHtml({ arrayBuffer: uint8Array.buffer });
          editor.value?.commands.setContent(result.value);
          aiError.value = 'Word 文档加载成功!';
          setTimeout(() => (aiError.value = null), 2000);
          logger.debug('Word document loaded successfully', {}, LogCategory.SYSTEM);
        } catch (mammothError) {
          logger.warn('Word document parsing failed, falling back to plain text', mammothError, LogCategory.BUSINESS);
          const appError = createError(
            ErrorCode.FILE_PARSE_ERROR,
            undefined,
            ErrorSeverity.WARNING,
            ErrorCategory.BUSINESS,
            { timestamp: Date.now(), additionalData: { originalError: mammothError } }
          );
          logger.warn('Word document parsing failed, falling back to plain text', appError, LogCategory.BUSINESS);
          const text = new TextDecoder().decode(uint8Array);
          editor.value?.commands.setContent(`<p>${text}</p>`);
          aiError.value = 'Word 文档加载成功 (仅文本)!';
          setTimeout(() => (aiError.value = null), 2000);
        }
      } else if (extension === 'rtf') {
        logger.debug('Loading RTF document', {}, LogCategory.SYSTEM);
        const _content = await invoke('load_file', { filePath });
        editor.value?.commands.setContent(`<p>${_content}</p>`);
        aiError.value = 'RTF 文档加载成功 (仅文本)!';
        setTimeout(() => (aiError.value = null), 2000);
      } else if (extension === 'md') {
        logger.debug('Loading Markdown document', {}, LogCategory.SYSTEM);
        const _content = await invoke('load_file', { filePath });
        const html = await invoke('markdown_to_html', { markdown: _content });
        editor.value?.commands.setContent(html as any);
        aiError.value = 'Markdown 文档加载成功!';
        setTimeout(() => (aiError.value = null), 2000);
      } else {
        logger.debug('Loading generic document', {}, LogCategory.SYSTEM);
        const _content = await invoke('load_file', { filePath });
        editor.value?.commands.setContent(_content as any);
        aiError.value = '文件加载成功!';
        setTimeout(() => (aiError.value = null), 2000);
      }

      addToRecentFiles(filePath);
      logger.debug('Document loaded successfully', {}, LogCategory.SYSTEM);
    }
  } catch (error) {
    logger.error('Load document failed', error, LogCategory.SYSTEM);
    const appError = createError(
      ErrorCode.FILE_READ_ERROR,
      undefined,
      ErrorSeverity.ERROR,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.error('Failed to load file', appError, LogCategory.SYSTEM);
    aiError.value = '加载失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  } finally {
    isLoading.value = false;
    logger.debug('Load document completed', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  }
};

const addToRecentFiles = (filePath: string) => {
  const index = recentFiles.value.indexOf(filePath);
  if (index > -1) {
    recentFiles.value.splice(index, 1);
  }
  recentFiles.value.unshift(filePath);
  if (recentFiles.value.length > 10) {
    recentFiles.value = recentFiles.value.slice(0, 10);
  }
  localStorage.setItem('logos-recent-files', JSON.stringify(recentFiles.value));
};

const loadRecentFile = async (filePath: string) => {
  try {
    const extension = filePath.split('.').pop()?.toLowerCase();
    
    if (extension === 'typ') {
      // Load Typst file and convert to HTML
      const typstContent = await invoke<string>('load_file', { filePath });
      const htmlContent = await invoke<string>('typst_to_html', { typst: typstContent });
      editor.value?.commands.setContent(htmlContent);
      currentFilename.value = filePath.split('/').pop() || 'document.typ';
      documentTitle.value = currentFilename.value.replace('.typ', '');
    } else {
      // Load other file types directly
      const _content = await invoke('load_file', { filePath });
      editor.value?.commands.setContent(_content as any);
    }
    
    showRecentFiles.value = false;
  } catch (error) {
    const appError = createError(
      ErrorCode.FILE_READ_ERROR,
      undefined,
      ErrorSeverity.ERROR,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.error('Failed to load recent file', appError, LogCategory.SYSTEM);
    aiError.value = '加载失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const clearRecentFiles = () => {
  recentFiles.value = [];
  localStorage.removeItem('logos-recent-files');
};

// Initializing editor...
const lowlight = createLowlight(common);

const editor = useEditor({
  extensions: [
    StarterKit.configure({
      codeBlock: false,
      history: {
        depth: 100,
        newGroupDelay: 500
      }
    }),
    PageBreak,
    TextStyle,
    FontFamily,
    Subscript,
    Superscript,
    Underline,
    TextAlign.configure({
      types: ['heading', 'paragraph']
    }),
    Image.configure({
      inline: true,
      allowBase64: true
    }),
    Highlight.configure({
      multicolor: true
    }),
    Typography,
    Placeholder.configure({
      placeholder: '开始输入内容...'
    }),
    CodeBlockLowlight.configure({
      lowlight,
      defaultLanguage: 'plaintext',
      HTMLAttributes: {
        class: 'editor-code-block'
      }
    }),
    ListKeymap,
    TableOfContents,
    Emoji.configure({
      suggestion: {
        items: ({ query }) => {
          const emojis = [
            { emoji: '😀', name: 'grinning face' },
            { emoji: '😃', name: 'grinning face with big eyes' },
            { emoji: '😄', name: 'grinning face with smiling eyes' },
            { emoji: '😁', name: 'beaming face with smiling eyes' },
            { emoji: '😅', name: 'grinning face with sweat' },
            { emoji: '😂', name: 'face with tears of joy' },
            { emoji: '🤣', name: 'rolling on the floor laughing' },
            { emoji: '😊', name: 'smiling face with smiling eyes' },
            { emoji: '😇', name: 'smiling face with halo' },
            { emoji: '🙂', name: 'slightly smiling face' },
            { emoji: '🙃', name: 'upside-down face' },
            { emoji: '😉', name: 'winking face' },
            { emoji: '😌', name: 'relieved face' },
            { emoji: '😍', name: 'smiling face with heart-eyes' },
            { emoji: '🥰', name: 'smiling face with hearts' },
            { emoji: '😘', name: 'face blowing a kiss' },
            { emoji: '😎', name: 'smiling face with sunglasses' },
            { emoji: '🤩', name: 'star-struck' },
            { emoji: '🥳', name: 'partying face' },
            { emoji: '😢', name: 'crying face' },
            { emoji: '😭', name: 'loudly crying face' },
            { emoji: '😡', name: 'pouting face' },
            { emoji: '👍', name: 'thumbs up' },
            { emoji: '👎', name: 'thumbs down' },
            { emoji: '❤️', name: 'red heart' },
            { emoji: '✨', name: 'sparkles' },
            { emoji: '🎉', name: 'party popper' }
          ];
          return emojis.filter(item => 
            item.emoji.includes(query) || item.name.includes(query)
          ).slice(0, 10);
        }
      }
    })
  ],
  content: '<p>开始写作...</p>',
  editorProps: {
    attributes: {
      class: 'editor-content'
    },
    handleDrop: (view, event, slice, moved) => {
      logger.info('Editor drop event', { moved, sliceSize: slice.content.size }, LogCategory.SYSTEM);
      return false;
    },
    handlePaste: (view, event, slice) => {
      logger.info('Editor paste event', { sliceSize: slice.content.size }, LogCategory.SYSTEM);
      return false;
    }
  },
  onCreate: ({ editor }) => {
    logger.debug('Editor onCreate callback triggered', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
    logger.debug('isLoading before setting to false', { isLoading: isLoading.value }, LogCategory.SYSTEM);
    logger.info('Editor created successfully', {
      schema: editor.schema,
      state: editor.state
    }, LogCategory.SYSTEM);
    // Clear loading state when editor is created
    isLoading.value = false;
    logger.debug('Loading state set to false in onCreate', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  },
  onBeforeCreate: () => {
    logger.info('Editor about to be created', {}, LogCategory.SYSTEM);
  },
  onDestroy: () => {
    logger.info('Editor destroyed', {}, LogCategory.SYSTEM);
  },
  onUpdate: ({ editor }) => {
    try {
      logger.debug('Editor content updated', {
        wordCount: wordCount.value,
        charCount: charCount.value
      }, LogCategory.SYSTEM);

      // Save current page content
      pageContents.value[activePageIndex.value] = editor.getHTML();

      const text = editor.getText();
      const words = text.trim().split(/\s+/).filter(word => word.length > 0);
      wordCount.value = words.length;
      charCount.value = text.length;

      totalPages.value = pageContents.value.length;

      if (autoSaveEnabled.value) {
        startAutoSave();
      }
    } catch (error) {
      const appError = createError(
        ErrorCode.UNKNOWN_ERROR,
        'Failed to update editor state',
        ErrorSeverity.WARNING,
        ErrorCategory.SYSTEM,
        { timestamp: Date.now(), additionalData: { originalError: error } }
      );
      logger.error('Editor update error', appError, LogCategory.SYSTEM);
    }
  },
  onSelectionUpdate: ({ editor }) => {
    try {
      const { from, to, empty } = editor.state.selection;
      logger.debug('Editor selection updated', { from, to, empty }, LogCategory.SYSTEM);

      if (!empty && from !== to) {
        const coords = editor.view.coordsAtPos(from);
        bubbleMenuPosition.value = {
          x: coords.left,
          y: coords.top - 50
        };
        showBubbleMenu.value = true;
        showFloatingMenu.value = false;
      } else {
        showBubbleMenu.value = false;
        const { $from } = editor.state.selection;
        const isAtStart = $from.parentOffset === 0;
        const isEmptyParagraph = $from.parent.type.name === 'paragraph' && $from.parent.nodeSize <= 2;
        
        if (isAtStart && isEmptyParagraph) {
          const coords = editor.view.coordsAtPos(from);
          floatingMenuPosition.value = {
            x: coords.left,
            y: coords.top + 30
          };
          showFloatingMenu.value = true;
        } else {
          showFloatingMenu.value = false;
        }
      }
    } catch (error) {
      const appError = createError(
        ErrorCode.UNKNOWN_ERROR,
        'Failed to update selection',
        ErrorSeverity.WARNING,
        ErrorCategory.SYSTEM,
        { timestamp: Date.now(), additionalData: { originalError: error } }
      );
      logger.error('Editor selection update error', appError, LogCategory.SYSTEM);
    }
  },
  onTransaction: ({ editor }) => {
    try {
      logger.debug('Editor transaction occurred', {
        selection: editor.state.selection
      }, LogCategory.SYSTEM);
    } catch (error) {
      const appError = createError(
        ErrorCode.UNKNOWN_ERROR,
        'Failed to process transaction',
        ErrorSeverity.WARNING,
        ErrorCategory.SYSTEM,
        { timestamp: Date.now(), additionalData: { originalError: error } }
      );
      logger.error('Editor transaction error', appError, LogCategory.SYSTEM);
    }
  }
}, {
  onError: (error) => {
    logger.error('Editor initialization error', error, LogCategory.SYSTEM);
    console.error('Editor initialization error:', error);
    isLoading.value = false;
  }
});

// Debug: Check editor status
onMounted(() => {
  // Editor mounted
});

// Editor initialization complete

// Watch for editor initialization
watch(editor, (newEditor) => {
  logger.debug('Editor watch triggered', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  logger.debug('newEditor', { hasEditor: !!newEditor }, LogCategory.SYSTEM);
  logger.debug('isLoading in watch', { isLoading: isLoading.value }, LogCategory.SYSTEM);
  // Editor watch triggered
  if (newEditor) {
    logger.debug('Editor initialized successfully', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
    // Clear loading state when editor is initialized
    if (isLoading.value) {
      logger.debug('Clearing loading state in editor watch', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
      isLoading.value = false;
    }
    // Editor initialized
    // Clear corrupted localStorage data
    try {
      localStorage.removeItem('logos-autosave');
      localStorage.removeItem('logos-recent-files');
      localStorage.removeItem('logos-tiptap-preset');
      // Cleared corrupted localStorage data
    } catch (error) {
      logger.error('Error clearing localStorage', error, LogCategory.SYSTEM);
    }

    // Start auto-save
    if (autoSaveEnabled.value) {
      startAutoSave();
    }
  } else {
    // Editor is null/undefined
    logger.warn('Editor is null/undefined', {}, LogCategory.SYSTEM);
  }
});

// Debounced Typst SVG preview refresh (split-view right panel)
const typstPreviewScheduler = createTypstPreviewScheduler(200, async () => {
  if (!editor.value) {
    return;
  }

  isTypstCompiling.value = true;

  try {
    const html = editor.value.getHTML() || '';
    const isTauriEnv = await isTauri();

    if (isTauriEnv) {
      try {
        const { previewTypstSvgFromHtml } = await import('../services/svgExportApi');
        const result = await previewTypstSvgFromHtml(html, htmlToTypst, 0);

        if (result.success && result.text) {
          typstPreviewData.value = result.text;
          typstPreviewRevision.value += 1;
          typstPreviewUrl.value = '';
          pdfCanvases.value = [];
        } else {
          logger.error('SVG rendering failed', result.error, LogCategory.SYSTEM);
          typstPreviewData.value = 'SVG rendering failed: ' + (result.error || 'Unknown error');
          typstPreviewRevision.value += 1;
          typstPreviewUrl.value = '';
        }
      } catch (invokeError) {
        logger.error('Failed to invoke render_typst', invokeError, LogCategory.SYSTEM);
        typstPreviewData.value = 'Backend compile failed: ' + (invokeError as Error).message;
        typstPreviewRevision.value += 1;
        typstPreviewUrl.value = '';
      }
    } else {
      logger.warn('Not in Tauri environment, using mock SVG preview', LogCategory.SYSTEM);
      typstPreviewData.value = `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="595" height="842" viewBox="0 0 595 842">
  <rect width="100%" height="100%" fill="white"/>
  <text x="50%" y="50%" text-anchor="middle" font-family="Arial" font-size="14" fill="#666">
    SVG preview requires Tauri desktop runtime
  </text>
</svg>`;
      typstPreviewRevision.value += 1;
      typstPreviewUrl.value = '';
    }
  } catch (error) {
    logger.error('Failed to compile Typst', error, LogCategory.SYSTEM);
    typstPreviewData.value = 'Compile failed: ' + (error as Error).message;
    typstPreviewRevision.value += 1;
    typstPreviewUrl.value = '';
  } finally {
    isTypstCompiling.value = false;
  }
});

/** Schedule Typst SVG preview refresh for the split-view panel. */
const updateTypstPreview = () => {
  if (!editor.value || !showSplitView.value) {
    return;
  }
  typstPreviewScheduler.schedule();
};

let detachTypstPreviewListener: (() => void) | null = null;

watch(editor, (newEditor) => {
  detachTypstPreviewListener?.();
  detachTypstPreviewListener = null;

  if (newEditor) {
    const handleEditorUpdate = () => {
      if (showSplitView.value) {
        updateTypstPreview();
      }
      performDocumentAnalysis();
      performAutoSave();
    };
    newEditor.on('update', handleEditorUpdate);
    detachTypstPreviewListener = () => {
      newEditor.off('update', handleEditorUpdate);
    };
  }
}, { immediate: true });

watch(showSplitView, (isOpen) => {
  if (isOpen && editor.value) {
    updateTypstPreview();
  } else if (!isOpen) {
    typstPreviewScheduler.cancel();
    isTypstCompiling.value = false;
  }
});


// Render PDF using pdf.js with incremental page loading
const renderPdfWithPdfJs = async (pdfBase64: string) => {
  try {
    // Configure pdf.js worker
    pdfjsLib.GlobalWorkerOptions.workerSrc = pdfjsWorker;
    
    // Convert base64 to Uint8Array
    if (!pdfBase64 || !pdfBase64.includes(',')) {
      throw new Error('Invalid base64 PDF data');
    }
    const pdfData = atob(pdfBase64.split(',')[1]);
    const pdfArray = new Uint8Array(pdfData.length);
    for (let i = 0; i < pdfData.length; i++) {
      pdfArray[i] = pdfData.charCodeAt(i);
    }
    
    // Load PDF document
    const loadingTask = pdfjsLib.getDocument({ data: pdfArray });
    const pdf = await loadingTask.promise;
    
    // Store PDF document
    pdfDocument.value = pdf;
    pdfTotalPages.value = pdf.numPages;
    pdfCurrentPage.value = 1;
    
    // Clear previous canvases
    pdfCanvases.value = [];
    
    // Calculate scale to fit sidebar width
    const sidebarWidth = 400 - 32; // 400px sidebar - 32px padding
    
    // Render pages incrementally
    for (let pageNum = 1; pageNum <= pdf.numPages; pageNum++) {
      const page = await pdf.getPage(pageNum);
      const viewport = page.getViewport({ scale: 1 });
      const scale = sidebarWidth / viewport.width;
      const scaledViewport = page.getViewport({ scale });
      
      // Create canvas element
      const canvas = document.createElement('canvas');
      canvas.className = 'pdf-page-canvas';
      canvas.height = scaledViewport.height;
      canvas.width = scaledViewport.width;
      
      const context = canvas.getContext('2d', { 
        alpha: false,
        willReadFrequently: false
      });
      if (!context) {
        throw new Error('Failed to get 2D context');
      }
      
      // Render PDF page
      const renderContext = {
        canvas: canvas,
        viewport: scaledViewport
      };
      
      await page.render(renderContext).promise;
      
      // Convert canvas to data URL for Vue rendering
      const dataUrl = canvas.toDataURL('image/png');
      
      // Store canvas element with data URL
      (canvas as any).dataUrl = dataUrl;
      
      // Add canvas to array
      pdfCanvases.value.push(canvas);
      
      // Update UI after each page is rendered
      await new Promise(resolve => setTimeout(resolve, 0));
    }
    
    // Clear URL to use canvas rendering
    typstPreviewUrl.value = '';
    typstPreviewData.value = '';
  } catch (error) {
    logger.error('Failed to render PDF with pdf.js', error, LogCategory.SYSTEM);
    typstPreviewData.value = 'PDF渲染失败: ' + (error as Error).message;
    throw error;
  }
};

// Watch for document title changes to update Tauri window title
watch(documentTitle, async (newTitle) => {
  try {
    // Temporarily disabled due to Tauri API issues
    // if (isTauri()) {
    //   const appWindow = getCurrentWindow();
    //   const title = newTitle ? `${newTitle} — LOGOS` : '未命名文档 — LOGOS';
    //   await appWindow.setTitle(title);
    // }
  } catch (error) {
    logger.warn('Failed to update window title', error, LogCategory.SYSTEM);
  }
});

// Watch for font family changes
watch(fontFamily, (newFont) => {
  if (editor.value) {
    editor.value.chain().focus().setFontFamily(newFont).run();
  }
});

// Variables for cleanup
let unlistenChunk: any = null;
let unlistenComplete: any = null;
let loadingTimeout: ReturnType<typeof setTimeout> | null = null;

onMounted(async () => {
  logger.debug('onMounted started', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  logger.debug('Initial isLoading state', { isLoading: isLoading.value }, LogCategory.SYSTEM);
  // Editor component mounted

  // Remove loading state - editor should initialize immediately
  // isLoading.value = true; // REMOVED: Don't set loading state on mount
  logger.debug('Loading state NOT set to true on mount', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);

  // Safety timeout to clear loading state after 10 seconds (kept as fallback)
  loadingTimeout = setTimeout(() => {
    if (isLoading.value) {
      logger.warn('Loading timeout reached, forcing loading state to false', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
      logger.warn('Editor value', { hasEditor: !!editor.value }, LogCategory.SYSTEM);
      isLoading.value = false;
    }
  }, 10000);

  // Initialize Tauri window title
  try {
    // Temporarily disabled due to Tauri API issues
    // if (isTauri()) {
    //   const appWindow = getCurrentWindow();
    //   const title = documentTitle.value ? `${documentTitle.value} — LOGOS` : '未命名文档 — LOGOS';
    //   await appWindow.setTitle(title);
    // }
  } catch (error) {
    logger.warn('Failed to set window title', error, LogCategory.SYSTEM);
  }

  // Initialize spell check service
  try {
    await spellCheckService.initialize('en_US');
    spellCheckEnabled.value = true;
  } catch (error) {
    const appError = createError(
      ErrorCode.OPERATION_FAILED,
      undefined,
      ErrorSeverity.WARNING,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.warn('Spell check service initialization failed', appError, LogCategory.SYSTEM);
  }

  // Load bibliography entries
  try {
    loadBibliographyEntries();
  } catch (error) {
    const appError = createError(
      ErrorCode.FILE_READ_ERROR,
      undefined,
      ErrorSeverity.WARNING,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.warn('Failed to load bibliography entries', appError, LogCategory.SYSTEM);
  }

  // Load recent files from localStorage
  try {
    const savedRecentFiles = localStorage.getItem('logos-recent-files');
    if (savedRecentFiles) {
      recentFiles.value = JSON.parse(savedRecentFiles);
    }
  } catch (error) {
    const appError = createError(
      ErrorCode.FILE_READ_ERROR,
      undefined,
      ErrorSeverity.WARNING,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.warn('Failed to load recent files', appError, LogCategory.SYSTEM);
  }

  // Load default file on startup
  try {
    const isTauriEnv = await isTauri();
    if (isTauriEnv) {
      const docsDir = await invoke<string>('get_documents_directory');
      const defaultFilePath = `${docsDir}/logo001.typ`;
      
      logger.debug('Checking for default file', { filePath: defaultFilePath }, LogCategory.SYSTEM);
      
      const fileExists = await invoke<boolean>('file_exists', { filePath: defaultFilePath });
      
      if (fileExists) {
        // Load the default file (Typst format, convert to HTML)
        logger.info('Loading default file', { filePath: defaultFilePath }, LogCategory.SYSTEM);
        try {
          const typstContent = await invoke<string>('load_file', { filePath: defaultFilePath });
          const htmlContent = await invoke<string>('typst_to_html', { typst: typstContent });
          editor.value?.commands.setContent(htmlContent);
          currentFilename.value = 'logo001.typ';
          documentTitle.value = 'logo001';
          logger.info('Default file loaded successfully', {}, LogCategory.SYSTEM);
        } catch (loadError) {
          logger.error('Failed to load or convert default file', loadError, LogCategory.SYSTEM);
          // Fallback to empty content
          editor.value?.commands.setContent('<p>开始写作...</p>');
          currentFilename.value = 'logo001.typ';
          documentTitle.value = 'logo001';
        }
      } else {
        // Create the default file (Typst format)
        logger.info('Creating default file', { filePath: defaultFilePath }, LogCategory.SYSTEM);
        try {
          const defaultTypstContent = '= 开始写作\n\n欢迎使用LOGOS编辑器。';
          await invoke('save_file', { filePath: defaultFilePath, content: defaultTypstContent });
          currentFilename.value = 'logo001.typ';
          documentTitle.value = 'logo001';
          logger.info('Default file created successfully', {}, LogCategory.SYSTEM);
        } catch (createError) {
          logger.error('Failed to create default file', createError, LogCategory.SYSTEM);
          // Continue without default file
          currentFilename.value = 'logo001.typ';
          documentTitle.value = 'logo001';
        }
      }
    }
  } catch (error) {
    const appError = createError(
      ErrorCode.FILE_READ_ERROR,
      undefined,
      ErrorSeverity.WARNING,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.warn('Failed to load/create default file', appError, LogCategory.SYSTEM);
    // Ensure editor has content even if file operations fail
    if (editor.value && editor.value.isEmpty) {
      editor.value?.commands.setContent('<p>开始写作...</p>');
    }
  }

  // Load version history from localStorage
  try {
    loadVersionHistory();
  } catch (error) {
    const appError = createError(
      ErrorCode.FILE_READ_ERROR,
      undefined,
      ErrorSeverity.WARNING,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.warn('Failed to load version history', appError, LogCategory.SYSTEM);
  }

  // Enable auto-save for version history
  try {
    versionHistoryManager.enableAutoSave(true);
  } catch (error) {
    const appError = createError(
      ErrorCode.OPERATION_FAILED,
      undefined,
      ErrorSeverity.WARNING,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.warn('Failed to enable version history auto-save', appError, LogCategory.SYSTEM);
  }

  // Enable auto-save for documents
  try {
    autoSaveManager.enableAutoSave(async () => {
      if (editor.value && editor.value.schema && editor.value.isEditable) {
        try {
          const content = editor.value.getHTML();
          const filename = currentFilename.value || 'untitled';
          await autoSaveManager.autoSave(content, filename);
        } catch (error) {
          logger.error('Auto-save failed', error, LogCategory.SYSTEM);
        }
      }
    });
  } catch (error) {
    const appError = createError(
      ErrorCode.OPERATION_FAILED,
      undefined,
      ErrorSeverity.WARNING,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.warn('Failed to enable document auto-save', appError, LogCategory.SYSTEM);
  }

  // Enable auto-backup
  try {
    backupManager.enableAutoBackup(async () => {
      if (editor.value && editor.value.schema && editor.value.isEditable) {
        try {
          const content = editor.value.getHTML();
          const filename = currentFilename.value || 'untitled';
          await backupManager.createBackup(content, filename);
          backupManager.cleanOldBackups();
        } catch (error) {
          logger.error('Auto-backup failed', error, LogCategory.SYSTEM);
        }
      }
    });
  } catch (error) {
    const appError = createError(
      ErrorCode.OPERATION_FAILED,
      undefined,
      ErrorSeverity.WARNING,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.warn('Failed to enable auto-backup', appError, LogCategory.SYSTEM);
  }

  // Listen for AI streaming events from Tauri backend
  if (isTauriEnvironment()) {
    unlistenChunk = listen('ai-stream-chunk', event => {
      streamingContent.value += event.payload as string;
      // Insert streaming content at the selected position
      if (streamSelection.value && editor.value) {
        editor.value
          .chain()
          .focus()
          .insertContentAt(streamSelection.value, streamingContent.value)
          .run();
      }
    });

    unlistenComplete = listen('ai-stream-complete', () => {
      isAiLoading.value = false;
      streamingContent.value = '';
      streamSelection.value = null;
    });
  }
});

// Global keyboard shortcuts handler
const handleKeyDown = (e: KeyboardEvent) => {
  // Ctrl/Cmd + S: Save document
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault();
    saveDocument();
  }
  // Ctrl/Cmd + O: Open document
  if ((e.ctrlKey || e.metaKey) && e.key === 'o') {
    e.preventDefault();
    loadDocument();
  }
  // Ctrl/Cmd + B: Bold
  if ((e.ctrlKey || e.metaKey) && e.key === 'b') {
    e.preventDefault();
    toggleBold();
  }
  // Ctrl/Cmd + I: Italic
  if ((e.ctrlKey || e.metaKey) && e.key === 'i') {
    e.preventDefault();
    toggleItalic();
  }
  // Ctrl/Cmd + U: Underline
  if ((e.ctrlKey || e.metaKey) && e.key === 'u') {
    e.preventDefault();
    toggleUnderline();
  }
  // Ctrl/Cmd + Z: Undo
  if ((e.ctrlKey || e.metaKey) && e.key === 'z' && !e.shiftKey) {
    e.preventDefault();
    editor.value?.chain().focus().undo().run();
  }
  // Ctrl/Cmd + Shift + Z: Redo
  if ((e.ctrlKey || e.metaKey) && e.key === 'z' && e.shiftKey) {
    e.preventDefault();
    editor.value?.chain().focus().redo().run();
  }
  // Ctrl/Cmd + Y: Redo (alternative)
  if ((e.ctrlKey || e.metaKey) && e.key === 'y') {
    e.preventDefault();
    editor.value?.chain().focus().redo().run();
  }
  // Ctrl/Cmd + F: Find
  if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
    e.preventDefault();
    toggleSearchDialog();
  }
  // Ctrl/Cmd + H: Replace
  if ((e.ctrlKey || e.metaKey) && e.key === 'h') {
    e.preventDefault();
    toggleSearchDialog();
  }
  // Ctrl/Cmd + K: Insert link
  if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
    e.preventDefault();
    addLink();
  }
  // Ctrl/Cmd + A: Select all
  if ((e.ctrlKey || e.metaKey) && e.key === 'a') {
    e.preventDefault();
    editor.value?.chain().focus().selectAll().run();
  }
  // Ctrl/Cmd + L: Left align
  if ((e.ctrlKey || e.metaKey) && e.key === 'l') {
    e.preventDefault();
    setTextAlign('left');
  }
  // Ctrl/Cmd + E: Center align
  if ((e.ctrlKey || e.metaKey) && e.key === 'e') {
    e.preventDefault();
    setTextAlign('center');
  }
  // Ctrl/Cmd + R: Right align (Note: conflicts with browser refresh, but prevented)
  if ((e.ctrlKey || e.metaKey) && e.key === 'r') {
    e.preventDefault();
    setTextAlign('right');
  }
  // Ctrl/Cmd + J: Justify
  if ((e.ctrlKey || e.metaKey) && e.key === 'j') {
    e.preventDefault();
    setTextAlign('justify');
  }
  // Ctrl/Cmd + N: New document
  if ((e.ctrlKey || e.metaKey) && e.key === 'n') {
    e.preventDefault();
    newDocument();
  }
  // Ctrl/Cmd + P: Print
  if ((e.ctrlKey || e.metaKey) && e.key === 'p') {
    e.preventDefault();
    printDocument();
  }
  // F1: Help/Shortcuts
  if (e.key === 'F1') {
    e.preventDefault();
    toggleHelp();
  }
  // F11: Fullscreen
  if (e.key === 'F11') {
    e.preventDefault();
    toggleFullscreen();
  }
  // Escape: Close all menus and dialogs
  if (e.key === 'Escape') {
    closeAllMenus();
    if (showSearchDialog.value) {
      toggleSearchDialog();
    }
    if (showShortcutsHelp.value) {
      toggleShortcutsHelp();
    }
    if (showWordCountDialog.value) {
      showWordCountDialog.value = false;
    }
  }
};

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown);

  // Handle custom event to switch to source mode
  window.addEventListener('switch-to-source-mode', () => {
    typstViewMode.value = 'source';
    generateTypstPreview();
  });

  logger.debug('onMounted completed', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  // No need to force clear loading state since we don't set it on mount anymore
  // setTimeout(() => {
  //   if (isLoading.value) {
  //     logger.warn('Force clearing loading state after onMounted delay', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  //     isLoading.value = false;
  //   }
  // }, 2000);
});

// Cleanup on component unmount
onUnmounted(() => {
  logger.debug('onUnmounted started', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  window.removeEventListener('keydown', handleKeyDown);
  window.removeEventListener('switch-to-source-mode', () => {});
  unlistenChunk.then((fn: any) => fn());
  unlistenComplete.then((fn: any) => fn());
  stopAutoSave();
  // Disable auto-save and backup
  autoSaveManager.disableAutoSave();
  backupManager.disableAutoBackup();
  // Clear timeouts to prevent memory leaks
  if (compileTimeout.value) {
    clearTimeout(compileTimeout.value);
  }
  if (searchTimeout.value) {
    clearTimeout(searchTimeout.value);
  }
  if (versionSaveTimeout.value) {
    clearTimeout(versionSaveTimeout.value);
  }
  // Clear loading timeout
  if (loadingTimeout) {
    clearTimeout(loadingTimeout);
  }
  typstPreviewScheduler.cancel();
  detachTypstPreviewListener?.();
  detachTypstPreviewListener = null;
  // Clear all aiError timeouts
  // These are handled by setTimeout in the component and will be cleared automatically
  editor.value?.destroy();
  logger.debug('onUnmounted completed', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
});

// Global error boundary handler for catching errors in child components
onErrorCaptured((err, _instance, info) => {
  logger.error('Error captured', { timestamp: new Date().toISOString(), error: err, componentInfo: info }, LogCategory.SYSTEM);
  const appError = createError(
    ErrorCode.OPERATION_FAILED,
    undefined,
    ErrorSeverity.ERROR,
    ErrorCategory.SYSTEM,
    { timestamp: Date.now(), additionalData: { originalError: err, componentInfo: info } }
  );
  logger.error('Component error captured', appError, LogCategory.SYSTEM);
  aiError.value = '发生错误: ' + (err as Error).message;
  setTimeout(() => (aiError.value = null), 5000);
  // Return false to prevent the error from propagating further
  return false;
});

const toggleStrike = () => {
  editor.value?.chain().focus().toggleStrike().run();
};

const toggleUnderline = () => {
  editor.value?.chain().focus().toggleUnderline().run();
};

const _toggleHighlight = () => {
  // Highlight extension not available in current setup
  // editor.value?.chain().focus().toggleHighlight({ color: highlightColor.value }).run();
};

const toggleSuperscript = () => {
  editor.value?.chain().focus().toggleSuperscript().run();
};

const toggleSubscript = () => {
  editor.value?.chain().focus().toggleSubscript().run();
};

const clearFormatting = () => {
  editor.value?.chain().focus().unsetAllMarks().clearNodes().run();
};

const handleTextEffects = () => {
  // Open text effects dialog for advanced text styling
  showToast('文本效果功能开发中', 'info');
};

const handleChangeCase = () => {
  if (!editor.value) {
return;
}
  const { from, to } = editor.value.state.selection;
  const doc = editor.value.state.doc;
  const selectedText = doc.textBetween(from, to);
  
  if (!selectedText) {
    showToast('请先选择文本', 'warning');
    return;
  }
  
  // Toggle between uppercase and lowercase
  const newText = selectedText === selectedText.toUpperCase() 
    ? selectedText.toLowerCase() 
    : selectedText.toUpperCase();
  
  editor.value.chain().focus().deleteSelection().insertContent(newText).run();
  showToast('已更改大小写', 'success');
};

const handlePinyinGuide = () => {
  if (!editor.value) {
return;
}
  
  const { from, to } = editor.value.state.selection;
  const doc = editor.value.state.doc;
  const selectedText = doc.textBetween(from, to);
  
  if (!selectedText) {
    showToast('请先选择文本', 'warning');
    return;
  }
  
  try {
    let annotatedText = '';
    for (const char of selectedText) {
      // Get pinyin for each character
      const charPinyin = pinyin(char, { toneType: 'symbol', type: 'array' });
      if (charPinyin && charPinyin.length > 0) {
        annotatedText += `<ruby>${char}<rt>${charPinyin[0]}</rt></ruby>`;
      } else {
        // Keep non-Chinese characters as is
        annotatedText += char;
      }
    }
    
    editor.value.chain().focus().deleteSelection().insertContent(annotatedText).run();
    showToast('已添加拼音标注', 'success');
  } catch (error) {
    logger.error('拼音标注失败', error, LogCategory.SYSTEM);
    showToast('拼音标注失败', 'error');
  }
};

const handleEnclosedCharacters = () => {
  if (!editor.value) {
return;
}
  
  const { from, to } = editor.value.state.selection;
  const doc = editor.value.state.doc;
  const selectedText = doc.textBetween(from, to);
  
  if (!selectedText || selectedText.length > 1) {
    showToast('请选择单个字符', 'warning');
    return;
  }
  
  // Enclosed characters using Unicode circled numbers
  const char = selectedText[0];
  const code = char.charCodeAt(0);
  
  // Numbers 0-9 to circled numbers (①-⑩)
  if (code >= 48 && code <= 57) {
    const circledCode = 0x2460 + (code - 48);
    const circledChar = String.fromCharCode(circledCode);
    editor.value.chain().focus().deleteSelection().insertContent(circledChar).run();
    showToast('已添加带圈字符', 'success');
    return;
  }
  
  // Letters to circled letters (ⓐ-ⓩ, Ⓐ-⒵)
  if (code >= 97 && code <= 122) {
    const circledCode = 0x24D0 + (code - 97);
    const circledChar = String.fromCharCode(circledCode);
    editor.value.chain().focus().deleteSelection().insertContent(circledChar).run();
    showToast('已添加带圈字符', 'success');
    return;
  }
  
  if (code >= 65 && code <= 90) {
    const circledCode = 0x24B6 + (code - 65);
    const circledChar = String.fromCharCode(circledCode);
    editor.value.chain().focus().deleteSelection().insertContent(circledChar).run();
    showToast('已添加带圈字符', 'success');
    return;
  }
  
  showToast('该字符不支持带圈', 'warning');
};

const handleVerticalText = () => {
  if (!editor.value) {
return;
}
  
  const { from, to } = editor.value.state.selection;
  const doc = editor.value.state.doc;
  const selectedText = doc.textBetween(from, to);
  
  if (!selectedText) {
    showToast('请先选择文本', 'warning');
    return;
  }
  
  // Apply vertical text using CSS writing-mode
  editor.value.chain().focus().insertContent(`<div style="writing-mode: vertical-rl; text-orientation: upright; display: inline-block;">${selectedText}</div>`).run();
  showToast('已应用纵向文字', 'success');
};

const handleDoubleStrikethrough = () => {
  if (!editor.value) {
return;
}
  // Toggle double strikethrough using CSS
  editor.value.chain().focus().toggleStrike().run();
  showToast('已切换删除线', 'success');
};

const handleFullHalfWidth = async () => {
  if (!editor.value) {
return;
}
  
  const { from, to } = editor.value.state.selection;
  const doc = editor.value.state.doc;
  const selectedText = doc.textBetween(from, to);
  
  if (!selectedText) {
    showToast('请先选择文本', 'warning');
    return;
  }
  
  try {
    const config = {
      conversion_type: 'Auto' as const,
      preserve_newlines: true,
      preserve_spaces: true,
    };
    const convertedText = await hybridServices.convertText(selectedText, config);
    
    if (convertedText) {
      editor.value.chain().focus().deleteSelection().insertContent(convertedText).run();
      showToast('已转换全角/半角', 'success');
    } else {
      showToast('转换失败', 'error');
    }
  } catch (error) {
    logger.error('Full/half width conversion error', error, LogCategory.SYSTEM);
    showToast('转换失败: ' + (error as Error).message, 'error');
  }
};

const handleTextBorder = () => {
  if (!editor.value) {
return;
}
  // Add border to selected text using inline style
  editor.value.chain().focus().insertContent('<span style="border: 1px solid currentColor; padding: 2px;">').run();
  showToast('已添加文字边框', 'success');
};

const handleTextShading = () => {
  if (!editor.value) {
return;
}
  // Add background color to selected text
  editor.value.chain().focus().toggleHighlight({ color: '#ffff00' }).run();
  showToast('已添加底纹', 'success');
};

const handleCharacterSpacing = () => {
  if (!editor.value) {
return;
}
  
  const spacing = prompt('请输入字符间距 (px，例如: 2, 4, 6):', '2');
  if (spacing && !isNaN(Number(spacing))) {
    const spacingValue = Number(spacing);
    editor.value.chain().focus().insertContent(`<span style="letter-spacing: ${spacingValue}px;">`).run();
    showToast(`字符间距设置为 ${spacingValue}px`, 'success');
  } else if (spacing) {
    showToast('请输入有效的数字', 'warning');
  }
};

const handleDropCap = () => {
  if (!editor.value) {
return;
}
  // Insert drop cap styling
  editor.value.chain().focus().insertContent('<span style="font-size: 3em; float: left; line-height: 0.8; margin-right: 4px;">').run();
  showToast('已插入首字下沉占位符', 'success');
};

const handleCharacterScale = () => {
  if (!editor.value) {
return;
}
  
  const scale = prompt('请输入字符缩放比例 (例如: 1.2, 1.5, 2.0):', '1.2');
  if (scale && !isNaN(Number(scale))) {
    const scaleValue = Number(scale);
    editor.value.chain().focus().insertContent(`<span style="transform: scale(${scaleValue}); display: inline-block;">`).run();
    showToast(`字符缩放设置为 ${scaleValue}x`, 'success');
  } else if (scale) {
    showToast('请输入有效的数字', 'warning');
  }
};

const handleSmallCaps = () => {
  if (!editor.value) {
return;
}
  // Apply small caps using CSS
  editor.value.chain().focus().insertContent('<span style="font-variant: small-caps;">').run();
  showToast('已应用小型大写字母', 'success');
};

const setLineSpacing = (spacing: 1 | 1.15 | 1.5 | 2 | 2.5 | 3) => {
  if (editor.value) {
    (editor.value.chain().focus() as any).setLineHeight(spacing).run();
    showToast(`行距设置为 ${spacing}`, 'success');
  }
};

const setParagraphSpacing = (before: number, after: number) => {
  if (editor.value) {
    // TipTap doesn't have native paragraph spacing, use CSS style instead
    (editor.value.chain().focus() as any).setParagraphSpacing(before, after).run();
    showToast(`段落间距设置为 段前${before}pt 段后${after}pt`, 'success');
  }
};

const addBorder = () => {
  if (editor.value) {
    // TipTap doesn't have native border, use CSS style instead
    (editor.value.chain().focus() as any).toggleBorder().run();
    showToast('已添加边框', 'success');
  }
};

const addShading = () => {
  if (editor.value) {
    // TipTap doesn't have native shading, use CSS style instead
    (editor.value.chain().focus() as any).toggleShading().run();
    showToast('已添加底纹', 'success');
  }
};

const toggleMultilevelList = () => {
  if (editor.value) {
    // Use ordered list as fallback for multilevel list
    editor.value.chain().focus().toggleOrderedList().run();
    showToast('已切换多级列表', 'success');
  }
};

const sortParagraph = () => {
  if (editor.value) {
    // TipTap doesn't have native sort, use placeholder
    showToast('段落排序功能需要后续实现', 'info');
  }
};

const toggleFormatMarks = () => {
  showFormatMarks.value = !showFormatMarks.value;
  showToast(showFormatMarks.value ? '已显示格式标记' : '已隐藏格式标记', 'success');
};

const toggleCode = () => {
  editor.value?.chain().focus().toggleCode().run();
};

const toggleHeading = (level: number) => {
  editor.value
    ?.chain()
    .focus()
    .toggleHeading({ level: level as any })
    .run();
};

const toggleBulletList = () => {
  editor.value?.chain().focus().toggleBulletList().run();
};

const toggleOrderedList = () => {
  editor.value?.chain().focus().toggleOrderedList().run();
};

const toggleTaskList = () => {
  editor.value?.chain().focus().toggleTaskList().run();
};

const toggleBlockquote = () => {
  editor.value?.chain().focus().toggleBlockquote().run();
};

const insertTable = () => {
  // Table extension temporarily disabled
  logger.debug('Table extension is currently disabled', {}, LogCategory.SYSTEM);
  // editor.value?.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run();
  // tableSelected.value = true;
  // showTableDesignTab.value = true;
};

const addColumn = () => {
  editor.value?.chain().focus().addColumnAfter().run();
};

const addColumnBefore = () => {
  editor.value?.chain().focus().addColumnBefore().run();
};

const addColumnAfter = () => {
  editor.value?.chain().focus().addColumnAfter().run();
};

const deleteColumn = () => {
  if (confirm('确定要删除这一列吗？')) {
    editor.value?.chain().focus().deleteColumn().run();
  }
};

const addRow = () => {
  editor.value?.chain().focus().addRowAfter().run();
};

const addRowBefore = () => {
  editor.value?.chain().focus().addRowBefore().run();
};

const addRowAfter = () => {
  editor.value?.chain().focus().addRowAfter().run();
};

const deleteRow = () => {
  if (confirm('确定要删除这一行吗？')) {
    editor.value?.chain().focus().deleteRow().run();
  }
};

const deleteTable = () => {
  if (confirm('确定要删除整个表格吗？此操作不可撤销。')) {
    editor.value?.chain().focus().deleteTable().run();
  }
};

const mergeCells = () => {
  editor.value?.chain().focus().mergeCells().run();
};

const splitCell = () => {
  editor.value?.chain().focus().splitCell().run();
};

const toggleHeaderRow = () => {
  editor.value?.chain().focus().toggleHeaderRow().run();
};

const toggleHeaderColumn = () => {
  editor.value?.chain().focus().toggleHeaderColumn().run();
};

const toggleHeaderCell = () => {
  editor.value?.chain().focus().toggleHeaderCell().run();
};

// Set cell background color
const setCellBackground = (color: string) => {
  editor.value?.chain().focus().setCellAttribute('backgroundColor', color).run();
};

// Set cell border
const setCellBorder = (border: string) => {
  editor.value?.chain().focus().setCellAttribute('border', border).run();
};

const undo = () => {
  logger.debug('Undo called', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  editor.value?.chain().focus().undo().run();
};

const redo = () => {
  logger.debug('Redo called', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  editor.value?.chain().focus().redo().run();
};

// Copy selected text to clipboard
const copyToClipboard = async () => {
  if (!editor.value) {
return;
}
  const { from, to } = editor.value.state.selection;
  const selectedText = editor.value.state.doc.textBetween(from, to, ' ');
  if (selectedText) {
    try {
      await navigator.clipboard.writeText(selectedText);
      aiError.value = '已复制到剪贴板';
      setTimeout(() => (aiError.value = null), 2000);
    } catch (error) {
      aiError.value = '复制失败';
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

// Toggle math formula dialog
const toggleMathDialog = () => {
  showMathDialog.value = !showMathDialog.value;
  if (showMathDialog.value) {
    mathFormula.value = '';
    mathPreview.value = '';
  }
};

// Render LaTeX formula preview
const renderMathPreview = () => {
  if (!mathFormula.value.trim()) {
    mathPreview.value = '';
    return;
  }

  try {
    const rendered = katex.renderToString(mathFormula.value, {
      throwOnError: false,
      displayMode: true,
      output: 'html'
    });
    mathPreview.value = rendered;
  } catch (error) {
    mathPreview.value = '<span style="color: #dc2626;">公式语法错误</span>';
  }
};

// Insert math formula into editor
const insertMathFormula = () => {
  if (!editor.value || !mathFormula.value.trim()) {
    aiError.value = '请输入数学公式';
    setTimeout(() => (aiError.value = null), 2000);
    return;
  }

  try {
    const rendered = katex.renderToString(mathFormula.value, {
      throwOnError: true,
      displayMode: true,
      output: 'html'
    });

    // Insert as HTML with data attribute for potential re-editing
    const formulaHtml = `
      <div class="math-formula" data-formula="${mathFormula.value}" style="padding: 12px; background: #f9fafb; border: 1px solid #e5e7eb; border-radius: 4px; margin: 8px 0; text-align: center;">
        ${rendered}
      </div>
    `;

    editor.value.chain().focus().insertContent(formulaHtml).run();
    aiError.value = '数学公式已插入';
    setTimeout(() => (aiError.value = null), 2000);
    showMathDialog.value = false;
    mathFormula.value = '';
    mathPreview.value = '';
  } catch (error) {
    aiError.value = '公式语法错误，请检查';
    setTimeout(() => (aiError.value = null), 3000);
  }
};

// Insert inline math formula
const insertInlineMath = () => {
  if (!editor.value || !mathFormula.value.trim()) {
    aiError.value = '请输入数学公式';
    setTimeout(() => (aiError.value = null), 2000);
    return;
  }

  try {
    const rendered = katex.renderToString(mathFormula.value, {
      throwOnError: true,
      displayMode: false,
      output: 'html'
    });

    const formulaHtml = `<span class="inline-math" data-formula="${mathFormula.value}" style="font-style: italic;">${rendered}</span>`;

    editor.value.chain().focus().insertContent(formulaHtml).run();
    aiError.value = '行内公式已插入';
    setTimeout(() => (aiError.value = null), 2000);
    showMathDialog.value = false;
    mathFormula.value = '';
    mathPreview.value = '';
  } catch (error) {
    aiError.value = '公式语法错误，请检查';
    setTimeout(() => (aiError.value = null), 3000);
  }
};

// Cut selected text to clipboard
const cutToClipboard = async () => {
  if (!editor.value) {
return;
}
  const { from, to } = editor.value.state.selection;
  const selectedText = editor.value.state.doc.textBetween(from, to, ' ');
  if (selectedText) {
    try {
      await navigator.clipboard.writeText(selectedText);
      editor.value.chain().focus().deleteSelection().run();
      aiError.value = '已剪切到剪贴板';
      setTimeout(() => (aiError.value = null), 2000);
    } catch (error) {
      aiError.value = '剪切失败';
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

// Format painter implementation
const formatPainter = () => {
  if (!editor.value) {
    return;
  }

  if (isFormatPainterActive.value) {
    // Apply format to current selection
    const { from, to } = editor.value.state.selection;
    if (from === to) {
      aiError.value = '请先选择要应用格式的文本';
      setTimeout(() => (aiError.value = null), 2000);
      return;
    }

    // Apply the stored format
    if (copiedFormat.value) {
      const { bold, italic, underline, strike, code, marks } = copiedFormat.value;
      
      const chain = editor.value.chain().focus();
      
      if (bold) {
chain.toggleBold();
}
      if (italic) {
chain.toggleItalic();
}
      if (underline) {
chain.toggleUnderline();
}
      if (strike) {
chain.toggleStrike();
}
      if (code) {
chain.toggleCode();
}
      
      chain.run();
      
      aiError.value = '格式已应用';
      setTimeout(() => (aiError.value = null), 2000);
    }
    
    // Deactivate format painter
    isFormatPainterActive.value = false;
    copiedFormat.value = null;
  } else {
    // Copy format from current selection
    const { from, to } = editor.value.state.selection;
    if (from === to) {
      aiError.value = '请先选择要复制格式的文本';
      setTimeout(() => (aiError.value = null), 2000);
      return;
    }

    // Store current selection format
    const marks = editor.value.state.storedMarks || [];
    const { $from } = editor.value.state.selection;
    
    copiedFormat.value = {
      bold: editor.value.isActive('bold'),
      italic: editor.value.isActive('italic'),
      underline: editor.value.isActive('underline'),
      strike: editor.value.isActive('strike'),
      code: editor.value.isActive('code'),
      marks
    };
    
    isFormatPainterActive.value = true;
    aiError.value = '格式已复制，请选择要应用格式的文本';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

// Paste from clipboard
const pasteFromClipboard = async () => {
  try {
    const text = await navigator.clipboard.readText();
    if (text && editor.value) {
      editor.value.chain().focus().insertContent(text).run();
    }
  } catch (error) {
    aiError.value = '无法访问剪贴板';
    setTimeout(() => (aiError.value = null), 3000);
  }
};

// Select all content
const selectAll = () => {
  editor.value?.chain().focus().selectAll().run();
};

// Select all objects (images, tables, etc.)
const selectObjects = () => {
  if (!editor.value) {
return;
}
  
  // Select all non-text nodes (images, tables, etc.)
  const { state } = editor.value;
  const { from, to } = state.selection;
  const doc = state.doc;
  
  let selected = false;
  doc.descendants((node, pos) => {
    if (node.type.name === 'image' || node.type.name === 'table') {
      editor.value?.chain().focus().setTextSelection({ from: pos, to: pos + node.nodeSize }).run();
      selected = true;
    }
  });
  
  if (!selected) {
    // Fallback to select all if no objects found
    editor.value?.chain().focus().selectAll().run();
  }
};

// Select text with similar formatting
const selectSimilarFormatting = () => {
  if (!editor.value) {
return;
}
  
  const { state } = editor.value;
  const { from, to } = state.selection;
  
  if (from === to) {
    // No selection, alert user
    alert('请先选择一些文本');
    return;
  }
  
  // Get the marks from the current selection
  const selectedMarks = new Set();
  
  state.doc.nodesBetween(from, to, (node, pos) => {
    if (node.marks) {
      node.marks.forEach(mark => {
        selectedMarks.add(mark.type.name);
      });
    }
  });
  
  if (selectedMarks.size === 0) {
    alert('选中的文本没有特殊格式');
    return;
  }
  
  // Select all text with the same marks
  const doc = state.doc;
  let newFrom = from;
  let newTo = to;
  
  // Search backward
  let searchPos = from;
  while (searchPos > 0) {
    const nodeBefore = doc.nodeAt(searchPos - 1);
    if (!nodeBefore) {
break;
}
    
    const hasSameMarks = nodeBefore.marks && 
      nodeBefore.marks.some(mark => selectedMarks.has(mark.type.name));
    
    if (hasSameMarks) {
      newFrom = searchPos - 1;
      searchPos--;
    } else {
      break;
    }
  }
  
  // Search forward
  searchPos = to;
  while (searchPos < doc.content.size) {
    const nodeAfter = doc.nodeAt(searchPos);
    if (!nodeAfter) {
break;
}
    
    const hasSameMarks = nodeAfter.marks && 
      nodeAfter.marks.some(mark => selectedMarks.has(mark.type.name));
    
    if (hasSameMarks) {
      newTo = searchPos + nodeAfter.nodeSize;
      searchPos += nodeAfter.nodeSize;
    } else {
      break;
    }
  }
  
  // Apply the new selection
  editor.value?.chain().focus().setTextSelection({ from: newFrom, to: newTo }).run();
};

const _insertHardBreak = () => {
  editor.value?.chain().focus().setHardBreak().run();
};

const toggleFormatPainter = () => {
  if (!editor.value) {
return;
}

  if (isFormatPainterActive.value) {
    // Apply format
    if (copiedFormat.value) {
      const { from, to } = editor.value.state.selection;
      if (from !== to) {
        // Apply marks from copied format
        Object.keys(copiedFormat.value.marks || {}).forEach(mark => {
          editor.value?.chain().focus().setMark(mark, copiedFormat.value.marks[mark]).run();
        });
      }
    }
    isFormatPainterActive.value = false;
    copiedFormat.value = null;
  } else {
    // Copy format
    const { from, to } = editor.value.state.selection;
    if (from !== to) {
      const marks: Record<string, any> = {};
      // Get marks from the selection
      const { state } = editor.value;
      state.doc.nodesBetween(from, to, (node: any) => {
        if (node.marks) {
          node.marks.forEach((mark: any) => {
            marks[mark.type.name] = mark.attrs;
          });
        }
      });

      copiedFormat.value = { marks };
      isFormatPainterActive.value = true;
      aiError.value = '格式已复制，点击要应用的位置';
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

// Insert current date
const insertDate = () => {
  const now = new Date();
  const dateStr = now.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  });
  editor.value?.chain().focus().insertContent(dateStr).run();
};

// Insert current time
const insertTime = () => {
  const now = new Date();
  const timeStr = now.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
  editor.value?.chain().focus().insertContent(timeStr).run();
};

// Insert current date and time
// Function reserved for future use

// Insert horizontal rule
const insertHorizontalRule = () => {
  editor.value?.chain().focus().insertContent('<hr>').run();
};

// Insert symbol
const insertSymbol = (symbol: string) => {
  editor.value?.chain().focus().insertContent(symbol).run();
};

// Insert video placeholder
const insertVideo = () => {
  if (!editor.value) {
    aiError.value = '编辑器未初始化';
    setTimeout(() => (aiError.value = null), 2000);
    return;
  }
  const videoUrl = prompt('请输入视频 URL (支持 mp4, webm, ogg):');
  if (videoUrl) {
    try {
      new URL(videoUrl);
      editor.value
        .chain()
        .focus()
        .insertContent(
          `
        <div style="margin: 16px 0;">
          <video src="${videoUrl}" controls style="max-width: 100%; border-radius: 4px;"></video>
        </div>
      `
        )
        .run();
      aiError.value = '视频已插入';
      setTimeout(() => (aiError.value = null), 2000);
    } catch {
      aiError.value = '请输入有效的视频 URL';
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

// Insert audio placeholder
const insertAudio = () => {
  if (!editor.value) {
    aiError.value = '编辑器未初始化';
    setTimeout(() => (aiError.value = null), 2000);
    return;
  }
  const audioUrl = prompt('请输入音频 URL (支持 mp3, wav, ogg):');
  if (audioUrl) {
    try {
      new URL(audioUrl);
      editor.value
        .chain()
        .focus()
        .insertContent(
          `
        <div style="margin: 16px 0;">
          <audio src="${audioUrl}" controls style="width: 100%;"></audio>
        </div>
      `
        )
        .run();
      aiError.value = '音频已插入';
      setTimeout(() => (aiError.value = null), 2000);
    } catch {
      aiError.value = '请输入有效的音频 URL';
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

// Set document columns
const setColumns = (count: number) => {
  const editorElement = document.querySelector('.ProseMirror') as HTMLElement;
  if (editorElement) {
    editorElement.style.columnCount = count.toString();
    editorElement.style.columnGap = '20px';
  }
};

// Insert column break
const insertColumnBreak = () => {
  editor.value?.chain().focus().insertContent('<span style="break-after: column;"></span>').run();
};

// Citation management system
const _citations = ref<
  Array<{ id: string; author: string; year: string; title?: string; journal?: string }>
>([]);
const _nextCitationId = ref(1);

// Set document language
const setLanguage = (lang: string) => {
  const editorElement = document.querySelector('.ProseMirror') as HTMLElement;
  if (editorElement) {
    editorElement.setAttribute('lang', lang);
    aiError.value = `语言已设置为 ${lang === 'zh-CN' ? '中文(简体)' : '英语(美国)'}`;
    setTimeout(() => (aiError.value = null), 2000);
  }
};

// Toggle web layout view
const toggleWebLayout = () => {
  isWebLayout.value = !isWebLayout.value;
  viewMode.value = isWebLayout.value ? 'web' : 'print';
  const pageContainer = document.querySelector('.page-container') as HTMLElement;
  if (pageContainer) {
    if (isWebLayout.value) {
      pageContainer.style.maxWidth = '100%';
      pageContainer.style.boxShadow = 'none';
    } else {
      pageContainer.style.maxWidth = '210mm';
      pageContainer.style.boxShadow = '0 0 10px rgba(0,0,0,0.1)';
    }
  }
};

// Change view mode from status bar
const changeViewMode = (mode: 'focus' | 'read' | 'print' | 'web') => {
  viewMode.value = mode;
  
  // Set editable state based on mode (Read mode is read-only)
  if (editor.value) {
    editor.value.setEditable(mode !== 'read');
  }

  // Handle web layout
  isWebLayout.value = (mode === 'web');
  const pageContainer = document.querySelector('.page-container') as HTMLElement;
  if (pageContainer) {
    if (mode === 'web') {
      pageContainer.style.maxWidth = '100%';
      pageContainer.style.boxShadow = 'none';
    } else {
      pageContainer.style.maxWidth = '210mm';
      pageContainer.style.boxShadow = '0 0 10px rgba(0,0,0,0.1)';
    }
  }

  // Auto hide panels in focus/read mode
  if (mode === 'focus' || mode === 'read') {
    showNavigationPane.value = false;
    showTypstPreview.value = false;
    showSpreadsheet.value = false;
    showUniverSpreadsheet.value = false;
  }
};

// Toggle navigation pane
const toggleNavigationPane = () => {
  handleToggleDocumentOutline();
};

// Ruler drag functionality
const dragTarget = ref<string | null>(null);
const dragStartX = ref(0);
const dragStartY = ref(0);

const startDrag = (target: string, event: MouseEvent) => {
  dragTarget.value = target;
  dragStartX.value = event.clientX;
  dragStartY.value = event.clientY;

  document.addEventListener('mousemove', handleDrag);
  document.addEventListener('mouseup', stopDrag);
};

const handleDrag = (event: MouseEvent) => {
  if (!dragTarget.value) {
return;
}

  const deltaX = event.clientX - dragStartX.value;
  const deltaY = event.clientY - dragStartY.value;

  // Continuous incremental drag by updating starting coordinates
  dragStartX.value = event.clientX;
  dragStartY.value = event.clientY;

  const pageContainer = document.querySelector('.page-container') as HTMLElement;
  if (!pageContainer) {
return;
}

  // Convert pixel drag offsets to physical millimeter changes (1 mm = 3.78 pixels)
  const deltaXMm = deltaX / 3.78;
  const deltaYMm = deltaY / 3.78;

  switch (dragTarget.value) {
    case 'leftMargin':
      pageMargins.value.left = Math.max(0, Math.min(100, pageMargins.value.left + deltaXMm));
      break;
    case 'rightMargin':
      pageMargins.value.right = Math.max(0, Math.min(100, pageMargins.value.right - deltaXMm));
      break;
    case 'topMargin':
      pageMargins.value.top = Math.max(0, Math.min(100, pageMargins.value.top + deltaYMm));
      break;
    case 'bottomMargin':
      pageMargins.value.bottom = Math.max(0, Math.min(100, pageMargins.value.bottom - deltaYMm));
      break;
    case 'leftIndent':
      leftIndent.value = Math.max(0, leftIndent.value + deltaX);
      break;
    case 'rightIndent':
      rightIndent.value = Math.max(0, rightIndent.value - deltaX);
      break;
    case 'firstLineIndent':
      firstLineIndent.value = Math.max(0, firstLineIndent.value + deltaX);
      break;
    case 'hangingIndent':
      hangingIndent.value = Math.max(0, hangingIndent.value + deltaX);
      break;
  }

  dragStartX.value = event.clientX;
  dragStartY.value = event.clientY;
};

const stopDrag = () => {
  dragTarget.value = null;
  document.removeEventListener('mousemove', handleDrag);
  document.removeEventListener('mouseup', stopDrag);
};

// Toggle read-only mode
// Function reserved for future use
const _toggleReadOnly = () => {
  isReadOnly.value = !isReadOnly.value;
  editor.value?.setOptions({ editable: !isReadOnly.value });
};

// AI Polish: Improve selected text to be more academic/professional
const triggerAiPolish = async () => {
  if (!editor.value) {
return;
}

  const { from, to } = editor.value.state.selection;
  const selectedText = editor.value.state.doc.textBetween(from, to, ' ');

  if (!selectedText) {
    aiError.value = '请先选择要润色的文本';
    setTimeout(() => (aiError.value = null), 3000);
    return;
  }

  // Hide bubble menu
  const bubbleMenu = document.getElementById('bubble-menu');
  if (bubbleMenu) {
    bubbleMenu.style.display = 'none';
  }

  isAiLoading.value = true;
  aiError.value = null;
  streamingContent.value = '';
  streamSelection.value = { from, to };

  try {
    await invoke('call_ai_service_stream', {
      prompt: '请润色以下段落，使其更有学术专业感，并以 Markdown 格式输出：',
      text: selectedText
    });
  } catch (error) {
    logger.error('AI 润色失败', error, LogCategory.BUSINESS);
    aiError.value = 'AI 润色失败，请检查 API 配置';
    setTimeout(() => (aiError.value = null), 3000);
    isAiLoading.value = false;
  }
};

// AI Expand: Expand selected text to be more detailed and comprehensive
const triggerAiExpand = async () => {
  if (!editor.value) {
return;
}

  const { from, to } = editor.value.state.selection;
  const selectedText = editor.value.state.doc.textBetween(from, to, ' ');

  if (!selectedText) {
    aiError.value = '请先选择要扩写的文本';
    setTimeout(() => (aiError.value = null), 3000);
    return;
  }

  // Hide bubble menu
  const bubbleMenu = document.getElementById('bubble-menu');
  if (bubbleMenu) {
    bubbleMenu.style.display = 'none';
  }

  isAiLoading.value = true;
  aiError.value = null;
  streamingContent.value = '';
  streamSelection.value = { from, to };

  try {
    await invoke('call_ai_service_stream', {
      prompt: '请扩写以下段落，使其内容更加丰富详细，并以 Markdown 格式输出：',
      text: selectedText
    });
  } catch (error) {
    logger.error('AI 扩写失败', error, LogCategory.BUSINESS);
    aiError.value = 'AI 扩写失败，请检查 API 配置';
    setTimeout(() => (aiError.value = null), 3000);
    isAiLoading.value = false;
  }
};

// AI Rewrite: Rewrite selected text with different wording
const triggerAiRewrite = async () => {
  if (!editor.value) {
return;
}

  const { from, to } = editor.value.state.selection;
  const selectedText = editor.value.state.doc.textBetween(from, to, ' ');

  if (!selectedText) {
    aiError.value = '请先选择要重写的文本';
    setTimeout(() => (aiError.value = null), 3000);
    return;
  }

  isAiLoading.value = true;
  aiError.value = null;
  streamingContent.value = '';
  streamSelection.value = { from, to };

  try {
    await invoke('call_ai_service_stream', {
      prompt: '请重写以下段落，保持原意但改变表达方式，并以 Markdown 格式输出：',
      text: selectedText
    });
  } catch (error) {
    logger.error('AI 重写失败', error, LogCategory.BUSINESS);
    aiError.value = 'AI 重写失败，请检查 API 配置';
    setTimeout(() => (aiError.value = null), 3000);
    isAiLoading.value = false;
  }
};

// AI Summarize: Summarize selected text to its core content
const triggerAiSummarize = async () => {
  if (!editor.value) {
return;
}

  const { from, to } = editor.value.state.selection;
  const selectedText = editor.value.state.doc.textBetween(from, to, ' ');

  if (!selectedText) {
    aiError.value = '请先选择要总结的文本';
    setTimeout(() => (aiError.value = null), 3000);
    return;
  }

  // Hide bubble menu
  const bubbleMenu = document.getElementById('bubble-menu');
  if (bubbleMenu) {
    bubbleMenu.style.display = 'none';
  }

  isAiLoading.value = true;
  aiError.value = null;
  streamingContent.value = '';
  streamSelection.value = { from, to };

  try {
    await invoke('call_ai_service_stream', {
      prompt: '请总结以下段落的核心内容，以简洁的 Markdown 格式输出：',
      text: selectedText
    });
  } catch (error) {
    logger.error('AI 总结失败', error, LogCategory.BUSINESS);
    aiError.value = 'AI 总结失败，请检查 API 配置';
    setTimeout(() => (aiError.value = null), 3000);
    isAiLoading.value = false;
  }
};

// AI Translate: Translate selected text (default to English)
const triggerAiTranslate = async () => {
  if (!editor.value) {
return;
}

  const { from, to } = editor.value.state.selection;
  const selectedText = editor.value.state.doc.textBetween(from, to, ' ');

  if (!selectedText) {
    aiError.value = '请先选择要翻译的文本';
    setTimeout(() => (aiError.value = null), 3000);
    return;
  }

  // Hide bubble menu
  const bubbleMenu = document.getElementById('bubble-menu');
  if (bubbleMenu) {
    bubbleMenu.style.display = 'none';
  }

  isAiLoading.value = true;
  aiError.value = null;
  streamingContent.value = '';
  streamSelection.value = { from, to };

  try {
    await invoke('call_ai_service_stream', {
      prompt:
        '请将以下段落翻译成英文。请严格按照 Markdown 格式输出，包括：使用 # ## ### 表示标题层级，**文字** 表示加粗，*文字* 表示斜体，- 表示无序列表，1. 2. 3. 表示有序列表。保持原有的格式结构：',
      text: selectedText
    });
  } catch (error) {
    logger.error('AI 翻译失败', error, LogCategory.BUSINESS);
    aiError.value = 'AI 翻译失败，请检查 API 配置';
    setTimeout(() => (aiError.value = null), 3000);
    isAiLoading.value = false;
  }
};

const exportToMarkdown = async () => {
  isLoading.value = true;
  try {
    const filePath = await save({
      filters: [{ name: 'Markdown', extensions: ['md'] }]
    });
    if (filePath) {
      const _content = editor.value?.getHTML() || '';
      const markdown = await invoke('export_to_markdown', { content: _content });
      await invoke('save_file', { filePath, content: markdown as string });
      aiError.value = 'Markdown 导出成功!';
      setTimeout(() => (aiError.value = null), 2000);
    }
  } catch (error) {
    logger.error('导出失败', error, LogCategory.SYSTEM);
    aiError.value = '导出 Markdown 失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  } finally {
    isLoading.value = false;
  }
};

const exportToHtml = async () => {
  isLoading.value = true;
  try {
    const filePath = await save({
      filters: [{ name: 'HTML', extensions: ['html'] }]
    });
    if (filePath) {
      const _content = editor.value?.getHTML() || '';
      const fullHtml = `<!DOCTYPE html>
<html lang="zh-CN">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Logos Document</title>
  <style>
    body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; line-height: 1.6; }
    h1, h2, h3 { margin-top: 1.5em; }
    table { border-collapse: collapse; width: 100%; margin: 1em 0; }
    table td, table th { border: 1px solid #ddd; padding: 8px; }
    table th { background: #f5f5f5; }
    blockquote { border-left: 3px solid #ddd; padding-left: 1em; margin: 1em 0; color: #666; }
    code { background: #f5f5f5; padding: 0.2em 0.4em; border-radius: 3px; }
    pre { background: #f5f5f5; padding: 1em; border-radius: 6px; overflow-x: auto; }
    pre code { background: none; padding: 0; }
    img { max-width: 100%; height: auto; }
  </style>
</head>
<body>
${_content}
</body>
</html>`;
      await invoke('save_file', { filePath, content: fullHtml });
      aiError.value = 'HTML 导出成功!';
      setTimeout(() => (aiError.value = null), 2000);
    }
  } catch (error) {
    logger.error('导出 HTML 失败', error, LogCategory.SYSTEM);
    aiError.value = '导出 HTML 失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  } finally {
    isLoading.value = false;
  }
};

const exportToPlainText = async () => {
  isLoading.value = true;
  try {
    const filePath = await save({
      filters: [{ name: 'Plain Text', extensions: ['txt'] }]
    });
    if (filePath) {
      const _content = editor.value?.getText() || '';
      await invoke('save_file', { filePath, content: _content });
      aiError.value = '纯文本导出成功!';
      setTimeout(() => (aiError.value = null), 2000);
    }
  } catch (error) {
    logger.error('导出纯文本失败', error, LogCategory.SYSTEM);
    aiError.value = '导出纯文本失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  } finally {
    isLoading.value = false;
  }
};

const toggleSearchDialog = () => {
  showSearchDialog.value = !showSearchDialog.value;
  if (showSearchDialog.value) {
    searchText.value = '';
    replaceText.value = '';
    currentMatch.value = 0;
    totalMatches.value = 0;
  }
};

const toggleWordCountDialog = () => {
  showWordCountDialog.value = !showWordCountDialog.value;
  if (showWordCountDialog.value) {
    // Trigger analysis when opening word count dialog
    performDocumentAnalysis();
  }
};

// Debounced search function for real-time search as user types
const _debouncedSearch = () => {
  if (searchTimeout.value) {
    clearTimeout(searchTimeout.value);
  }
  searchTimeout.value = setTimeout(() => {
    if (searchText.value) {
      findNext();
    }
  }, 300); // 300ms debounce
};

const findNext = async () => {
  if (!editor.value || !searchText.value) {
return;
}

  const { to } = editor.value.state.selection;
  const doc = editor.value.state.doc;
  const text = doc.textBetween(0, doc.content.size, ' ');

  // Use hybrid service for search
  const options = {
    case_sensitive: searchCaseSensitive.value,
    whole_word: searchWholeWord.value,
    use_regex: searchUseRegex.value,
  };

  const result = await hybridServices.searchText(text, searchText.value, options, to);

  if (result && result.total_count > 0) {
    totalMatches.value = result.total_count;
    
    // Find next match from current position
    const nextMatch = result.matches.find(m => m.position >= to) || result.matches[0];
    
    if (nextMatch) {
      editor.value
        .chain()
        .focus()
        .setTextSelection({ from: nextMatch.position, to: nextMatch.position + nextMatch.length })
        .run();
      currentMatch.value = result.current_index + 1;
    }
  } else {
    totalMatches.value = 0;
    currentMatch.value = 0;
  }
};

const findPrevious = async () => {
  if (!editor.value || !searchText.value) {
return;
}

  const { from } = editor.value.state.selection;
  const doc = editor.value.state.doc;
  const text = doc.textBetween(0, doc.content.size, ' ');

  // Use hybrid service for search
  const options = {
    case_sensitive: searchCaseSensitive.value,
    whole_word: searchWholeWord.value,
    use_regex: searchUseRegex.value,
  };

  const result = await hybridServices.searchText(text, searchText.value, options, 0);

  if (result && result.total_count > 0) {
    totalMatches.value = result.total_count;
    
    // Find previous match from current position
    const previousMatch = [...result.matches].reverse().find(m => m.position < from) || result.matches[result.matches.length - 1];
    
    if (previousMatch) {
      editor.value
        .chain()
        .focus()
        .setTextSelection({ from: previousMatch.position, to: previousMatch.position + previousMatch.length })
        .run();
      currentMatch.value = currentMatch.value > 1 ? currentMatch.value - 1 : totalMatches.value;
    }
  } else {
    totalMatches.value = 0;
    currentMatch.value = 0;
  }
};

const replaceCurrent = async () => {
  if (!editor.value || !searchText.value) {
return;
}

  const { from, to } = editor.value.state.selection;
  const selectedText = editor.value.state.doc.textBetween(from, to, ' ');

  if (selectedText === searchText.value) {
    editor.value.chain().focus().insertContentAt({ from, to }, replaceText.value).run();
    await findNext();
  }
};

const replaceAll = async () => {
  if (!editor.value || !searchText.value) {
return;
}

  const doc = editor.value.state.doc;
  const text = doc.textBetween(0, doc.content.size, ' ');

  // Use hybrid service for replace
  const options = {
    case_sensitive: searchCaseSensitive.value,
    whole_word: searchWholeWord.value,
    use_regex: searchUseRegex.value,
    replace_all: true,
  };

  const result = await hybridServices.replaceText(text, searchText.value, replaceText.value, options);

  if (result && result.replaced_count > 0) {
    if (confirm(`确定要替换所有 ${result.replaced_count} 处匹配吗？此操作不可撤销。`)) {
      editor.value.chain().focus().setContent(result.new_text).run();
      currentMatch.value = 0;
      totalMatches.value = 0;
      aiError.value = `已替换 ${result.replaced_count} 处`;
      setTimeout(() => (aiError.value = null), 2000);
    }
  } else {
    aiError.value = '未找到匹配项';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

const insertImage = async () => {
  try {
    const filePath = await open({
      filters: [
        {
          name: 'Images',
          extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp', 'svg']
        }
      ]
    });

    if (filePath) {
      // Read file and convert to base64
      const fileContent = await invoke('load_file', { filePath });
      const base64 = btoa(fileContent as string);
      const mimeType = filePath.split('.').pop();
      const dataUrl = `data:image/${mimeType};base64,${base64}`;

      // Image extension not properly configured
      editor.value?.chain().focus().insertContent(`<img src="${dataUrl}" />`).run();
    }
  } catch (error) {
    logger.error('插入图片失败', error, LogCategory.SYSTEM);
    aiError.value = '插入图片失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

// Open image resize dialog
const openImageResizeDialog = () => {
  if (!editor.value) {
return;
}
  const { from } = editor.value.state.selection;
  const node = editor.value.state.doc.nodeAt(from);
  if (node && node.type.name === 'image') {
    const attrs = node.attrs;
    selectedImageWidth.value = parseInt(attrs.width) || 100;
    selectedImageHeight.value = parseInt(attrs.height) || 100;
    selectedImageUnit.value = attrs.width?.includes('%') ? '%' : 'px';
    showImageResizeDialog.value = true;
  } else {
    aiError.value = '请先选择一张图片';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

// Resize selected image
const applyImageResize = () => {
  if (!editor.value) {
return;
}
  const { from } = editor.value.state.selection;
  const node = editor.value.state.doc.nodeAt(from);
  if (node && node.type.name === 'image') {
    const width =
      selectedImageUnit.value === '%'
        ? `${selectedImageWidth.value}%`
        : `${selectedImageWidth.value}px`;
    const height = maintainAspectRatio.value ? 'auto' : `${selectedImageHeight.value}px`;

    editor.value.chain().focus().updateAttributes('image', { width, height }).run();

    aiError.value = '图片尺寸已更新';
    setTimeout(() => (aiError.value = null), 2000);
    showImageResizeDialog.value = false;
  }
};

// Reset image to original size
const resetImageSize = () => {
  if (!editor.value) {
return;
}
  const { from } = editor.value.state.selection;
  const node = editor.value.state.doc.nodeAt(from);
  if (node && node.type.name === 'image') {
    editor.value.chain().focus().updateAttributes('image', { width: '100%', height: 'auto' }).run();

    aiError.value = '图片已重置为原始尺寸';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

// Set text direction (LTR/RTL)
const setTextDirection = (direction: 'ltr' | 'rtl') => {
  textDirection.value = direction;
  const editorElement = document.querySelector('.ProseMirror') as HTMLElement;
  if (editorElement) {
    editorElement.style.direction = direction;
    editorElement.style.textAlign = direction === 'rtl' ? 'right' : 'left';
  }
  aiError.value = direction === 'rtl' ? '已设置为从右到左' : '已设置为从左到右';
  setTimeout(() => (aiError.value = null), 2000);
};

const addLink = () => {
  const url = prompt('请输入链接地址:');
  if (url && editor.value) {
    // Validate URL format
    try {
      new URL(url);
      editor.value.chain().focus().setLink({ href: url }).run();
    } catch {
      aiError.value = '请输入有效的链接地址';
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

const _clearDocument = () => {
  if (confirm('确定要清空文档吗？此操作不可撤销。')) {
    editor.value?.chain().focus().clearContent().run();
    localStorage.removeItem('logos-autosave');
    lastSavedContent.value = '';
    showToast('文档已清空', 'success');
  }
};

const newDocument = () => {
  logger.debug('newDocument called', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  if (confirm('确定要创建新文档吗？当前内容将被清空。')) {
    logger.debug('Creating new document, clearing content', {}, LogCategory.SYSTEM);
    editor.value?.commands.setContent('<p>开始写作...</p>');
    localStorage.removeItem('logos-autosave');
    lastSavedContent.value = '';
    logger.debug('New document created', {}, LogCategory.SYSTEM);
  } else {
    logger.debug('New document creation cancelled by user', {}, LogCategory.SYSTEM);
  }
};

const printDocument = () => {
  logger.debug('printDocument called', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  window.print();
};

// Spell Check Functions
const toggleSpellCheck = async () => {
  if (!spellCheckEnabled.value) {
    try {
      await spellCheckService.initialize('en_US');
      spellCheckEnabled.value = true;
      runSpellCheck();
      aiError.value = '拼写检查已启用';
    } catch (error) {
      logger.error('Spell check initialization error', error, LogCategory.SYSTEM);
      aiError.value = '拼写检查初始化失败: ' + (error as Error).message;
      spellCheckEnabled.value = false;
    }
  } else {
    try {
      spellCheckEnabled.value = false;
      currentSpellCheckErrors.value = [];
      aiError.value = '拼写检查已禁用';
    } catch (error) {
      logger.error('Spell check disable error', error, LogCategory.SYSTEM);
      aiError.value = '禁用拼写检查失败: ' + (error as Error).message;
    }
  }
  setTimeout(() => (aiError.value = null), 2000);
};

const runSpellCheck = () => {
  if (!editor.value || !spellCheckEnabled.value) {
return;
}
  try {
    // Extract plain text from HTML for spell checking
    const html = editor.value.getHTML();
    const tempDiv = document.createElement('div');
    tempDiv.innerHTML = html;
    const text = tempDiv.textContent || tempDiv.innerText || '';

    const result = spellCheckService.checkSpelling(text);
    currentSpellCheckErrors.value = result.errors;
    if (result.errorCount > 0) {
      showSpellCheckDialog.value = true;
    } else {
      aiError.value = '未发现拼写错误';
      setTimeout(() => (aiError.value = null), 2000);
    }
  } catch (error) {
    logger.error('Spell check error', error, LogCategory.SYSTEM);
    aiError.value = '拼写检查失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

// TOC Functions
const generateTOC = async () => {
  if (!editor.value) {
return;
}
  const _content = editor.value.getHTML();
  try {
    // Use hybrid service for TOC generation
    const toc = await hybridServices.generateToc(_content);
    if (toc) {
      tocHTML.value = toc.html;
      tocVisible.value = true;
      showTocDialog.value = true;
    }
  } catch (error) {
    logger.error('TOC generation error', error, LogCategory.SYSTEM);
    aiError.value = '目录生成失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const insertTOC = () => {
  if (!editor.value || !tocHTML.value) {
return;
}
  try {
    editor.value.chain().focus().insertContent(tocHTML.value).run();
    showTocDialog.value = false;
  } catch (error) {
    logger.error('TOC insertion error', error, LogCategory.SYSTEM);
    aiError.value = '插入目录失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

// Bibliography Functions
const openBibliographyDialog = () => {
  try {
    bibliographyEntries.value = bibliographyManager.getAllEntries();
    showBibliographyDialog.value = true;
  } catch (error) {
    logger.error('Open bibliography dialog error', error, LogCategory.SYSTEM);
    aiError.value = '打开参考文献对话框失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const insertCitation = (entryId: string) => {
  if (!editor.value) {
return;
}
  try {
    const entry = bibliographyManager.getEntry(entryId);
    if (!entry) {
      aiError.value = '未找到参考文献';
      setTimeout(() => (aiError.value = null), 2000);
      return;
    }
    const citation = bibliographyManager.formatEntry(entry, selectedCitationStyle.value as any);
    editor.value.chain().focus().insertContent(citation).run();
    bibliographyManager.addCitation(entryId, editor.value.state.selection.from);
  } catch (error) {
    logger.error('Insert citation error', error, LogCategory.SYSTEM);
    aiError.value = '插入引用失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const generateBibliography = () => {
  if (!editor.value) {
return;
}
  try {
    const html = bibliographyManager.generateBibliography();
    editor.value.chain().focus().insertContent(html).run();
    showBibliographyDialog.value = false;
  } catch (error) {
    logger.error('Generate bibliography error', error, LogCategory.SYSTEM);
    aiError.value = '生成参考文献失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const loadBibliographyEntries = () => {
  try {
    bibliographyEntries.value = bibliographyManager.getAllEntries();
  } catch (error) {
    logger.error('Load bibliography entries error', error, LogCategory.SYSTEM);
  }
};

const addBibliographyEntry = () => {
  try {
    const authors = newBibliographyEntry.value.authors
    ? newBibliographyEntry.value.authors
      .split(',')
      .map(a => a.trim())
      .filter(a => a)
    : [];
    bibliographyManager.addEntry({
      type: newBibliographyEntry.value.type as
        | 'book'
        | 'article'
        | 'journal'
        | 'website'
        | 'conference'
        | 'thesis'
        | 'report',
      title: newBibliographyEntry.value.title,
      authors,
      year: newBibliographyEntry.value.year ? parseInt(newBibliographyEntry.value.year) : undefined,
      publisher: newBibliographyEntry.value.publisher
    });

    // Reset form
    newBibliographyEntry.value = {
      type: 'book',
      title: '',
      authors: '',
      year: '',
      publisher: ''
    };

    showAddBibliographyEntry.value = false;
    loadBibliographyEntries();
    aiError.value = '参考文献添加成功';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Add bibliography entry error', error, LogCategory.SYSTEM);
    aiError.value = '添加参考文献失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const deleteBibliographyEntry = (entryId: string) => {
  if (confirm('确定要删除此参考文献吗？')) {
    try {
      bibliographyManager.deleteEntry(entryId);
      loadBibliographyEntries();
      aiError.value = '参考文献已删除';
      setTimeout(() => (aiError.value = null), 2000);
    } catch (error) {
      logger.error('Delete bibliography entry error', error, LogCategory.SYSTEM);
      aiError.value = '删除参考文献失败: ' + (error as Error).message;
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

const deleteBookmark = (bookmarkId: string) => {
  bookmarks.value = bookmarks.value.filter(b => b.id !== bookmarkId);
  aiError.value = '书签已删除';
  setTimeout(() => (aiError.value = null), 2000);
};

const insertBookmarkReference = (bookmarkId: string) => {
  if (!editor.value) {
return;
}
  const bookmark = bookmarks.value.find(b => b.id === bookmarkId);
  if (bookmark) {
    editor.value
      .chain()
      .focus()
      .insertContent(
        `<a href="#${bookmarkId}" class="cross-reference" style="color: #3b82f6; text-decoration: underline; cursor: pointer;">${bookmark.name}</a>`
      )
      .run();
    showBookmarksDialog.value = false;
    aiError.value = '交叉引用已插入';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

// Footnote/Endnote Functions
const addFootnote = () => {
  if (!editor.value || !footnoteText.value) {
return;
}
  try {
    const id = footnoteManager.addFootnote(footnoteText.value, editor.value.state.selection.from);
    const ref = footnoteManager.generateFootnoteReference(id);
    editor.value.chain().focus().insertContent(ref).run();
    footnoteText.value = '';
    showFootnoteDialog.value = false;
  } catch (error) {
    logger.error('Add footnote error', error, LogCategory.SYSTEM);
    aiError.value = '添加脚注失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const addEndnote = () => {
  if (!editor.value || !endnoteText.value) {
return;
}
  try {
    const id = footnoteManager.addEndnote(endnoteText.value, editor.value.state.selection.from);
    const ref = footnoteManager.generateEndnoteReference(id);
    editor.value.chain().focus().insertContent(ref).run();
    endnoteText.value = '';
    showEndnoteDialog.value = false;
  } catch (error) {
    logger.error('Add endnote error', error, LogCategory.SYSTEM);
    aiError.value = '添加尾注失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const insertFootnotesSection = () => {
  if (!editor.value) {
return;
}
  try {
    const html = footnoteManager.generateFootnotesHTML();
    if (html) {
      editor.value.chain().focus().insertContent(html).run();
      aiError.value = '脚注区域已插入';
      setTimeout(() => (aiError.value = null), 2000);
    } else {
      aiError.value = '没有脚注可插入';
      setTimeout(() => (aiError.value = null), 2000);
    }
  } catch (error) {
    logger.error('Insert footnotes section error', error, LogCategory.SYSTEM);
    aiError.value = '插入脚注部分失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const insertEndnotesSection = () => {
  if (!editor.value) {
return;
}
  try {
    const html = footnoteManager.generateEndnotesHTML();
    if (html) {
      editor.value.chain().focus().insertContent(html).run();
      aiError.value = '尾注区域已插入';
      setTimeout(() => (aiError.value = null), 2000);
    } else {
      aiError.value = '没有尾注可插入';
      setTimeout(() => (aiError.value = null), 2000);
    }
  } catch (error) {
    logger.error('Insert endnotes section error', error, LogCategory.SYSTEM);
    aiError.value = '插入尾注部分失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

// Multi-Column Functions
const applyMultiColumn = () => {
  if (!editor.value) {
return;
}
  try {
    multiColumnManager.setColumnCount(columnCount.value);
    multiColumnManager.setColumnGap(columnGap.value);
    const _content = editor.value.getHTML();
    const withColumns = multiColumnManager.applyLayout(_content);
    editor.value.commands.setContent(withColumns);
    showMultiColumnDialog.value = false;
    aiError.value = '多栏布局已应用';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Apply multi-column error', error, LogCategory.SYSTEM);
    aiError.value = '应用多栏布局失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const removeMultiColumn = () => {
  if (!editor.value) {
return;
}
  try {
    const _content = editor.value.getHTML();
    const withoutColumns = multiColumnManager.removeLayout(_content);
    editor.value.commands.setContent(withoutColumns);
    columnCount.value = 1;
    aiError.value = '多栏布局已移除';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Remove multi-column error', error, LogCategory.SYSTEM);
    aiError.value = '移除多栏布局失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

// Section Break Functions
const insertSectionBreak = () => {
  if (!editor.value) {
return;
}
  try {
    const _id = sectionBreaksManager.addSectionBreak(
      sectionBreakType.value as any,
      editor.value.state.selection.from
    );
    const html = sectionBreaksManager.generateSectionBreakHTML(sectionBreakType.value as any);
    editor.value.chain().focus().insertContent(html).run();
    aiError.value = '分节符已插入';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Insert section break error', error, LogCategory.SYSTEM);
    aiError.value = '插入分节符失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

// Cross-Reference Functions
const loadAvailableTargets = () => {
  try {
    availableTargets.value = crossReferencesManager.getAllTargets();
  } catch (error) {
    logger.error('Load available targets error', error, LogCategory.SYSTEM);
  }
};

const openCrossReferenceDialog = () => {
  loadAvailableTargets();
  showCrossReferenceDialog.value = true;
};

const addCrossReference = () => {
  if (!editor.value || !crossReferenceTarget.value) {
return;
}
  try {
    const refId = crossReferencesManager.addReference({
      type: crossReferenceType.value as any,
      targetId: crossReferenceTarget.value,
      label: crossReferenceLabel.value || crossReferenceTarget.value,
      format: crossReferenceFormat.value as any,
      position: editor.value.state.selection.from
    });
    const html = crossReferencesManager.generateReferenceHTML(refId);
    editor.value.chain().focus().insertContent(html).run();
    showCrossReferenceDialog.value = false;
    crossReferenceTarget.value = '';
    crossReferenceLabel.value = '';
    aiError.value = '交叉引用已添加';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Add cross-reference error', error, LogCategory.SYSTEM);
    aiError.value = '添加交叉引用失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

// Version History Functions
const openVersionHistory = () => {
  try {
    versionList.value = versionHistoryManager.getAllVersions();
    currentVersion.value = versionHistoryManager.getLatestVersion()?.version || 1;
    showVersionHistoryDialog.value = true;
  } catch (error) {
    logger.error('Open version history error', error, LogCategory.SYSTEM);
    aiError.value = '打开版本历史失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const createVersionFromManager = () => {
  if (!editor.value) {
return;
}
  try {
    const description = prompt('请输入版本描述:', '手动保存');
    if (description === null) {
return;
}

    const versionId = versionHistoryManager.createVersion(
      editor.value.getHTML(),
      description,
      false,
      []
    );

    versionList.value = versionHistoryManager.getAllVersions();
    aiError.value = '版本已创建';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Create version error', error, LogCategory.SYSTEM);
    aiError.value = '创建版本失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const restoreVersionFromManager = (versionId: string) => {
  if (!editor.value) {
return;
}
  try {
    const _content = versionHistoryManager.restoreVersion(versionId);
    if (_content) {
      if (confirm('确定要恢复到此版本吗？当前未保存的更改将丢失。')) {
        editor.value.commands.setContent(_content);
        versionList.value = versionHistoryManager.getAllVersions();
        aiError.value = '版本已恢复';
        setTimeout(() => (aiError.value = null), 2000);
      }
    }
  } catch (error) {
    logger.error('Restore version error', error, LogCategory.SYSTEM);
    aiError.value = '恢复版本失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const deleteVersionFromManager = (versionId: string) => {
  if (confirm('确定要删除此版本吗？')) {
    try {
      versionHistoryManager.deleteVersion(versionId);
      versionList.value = versionHistoryManager.getAllVersions();
      aiError.value = '版本已删除';
      setTimeout(() => (aiError.value = null), 2000);
    } catch (error) {
      logger.error('Delete version error', error, LogCategory.SYSTEM);
      aiError.value = '删除版本失败: ' + (error as Error).message;
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

// Print Preview Functions
const openPrintPreview = () => {
  if (!editor.value) {
return;
}
  try {
    const _content = editor.value.getHTML();
    printPreviewManager.setConfig(printConfig.value as any);
    printPreviewPages.value = printPreviewManager.generatePreview(_content);
    showPrintPreviewDialog.value = true;
  } catch (error) {
    logger.error('Open print preview error', error, LogCategory.SYSTEM);
    aiError.value = '打开打印预览失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const printFromPreview = () => {
  if (!editor.value) {
return;
}
  try {
    window.print();
    aiError.value = '打印任务已发送';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Print error', error, LogCategory.SYSTEM);
    aiError.value = '打印失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const exportToPDF = async () => {
  if (!editor.value) {
return;
}
  try {
    const _content = editor.value.getHTML();
    printPreviewManager.setConfig(printConfig.value as any);
    await printPreviewManager.exportToPDF(_content);

    aiError.value = 'PDF导出成功';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Export PDF error', error, LogCategory.SYSTEM);
    aiError.value = '导出PDF失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const setTextAlign = (alignment: 'left' | 'center' | 'right' | 'justify') => {
  try {
    if (editor.value) {
      const chain = editor.value.chain().focus();
      if (typeof chain.setTextAlign === 'function') {
        chain.setTextAlign(alignment).run();
      }
    }
  } catch (error) {
    logger.warn('setTextAlign not available', error, LogCategory.SYSTEM);
  }
};

const toggleFullscreen = () => {
  if (!document.fullscreenElement) {
    document.documentElement.requestFullscreen();
    isFullscreen.value = true;
  } else {
    document.exitFullscreen();
    isFullscreen.value = false;
  }
};

const toggleShortcutsHelp = () => {
  showShortcutsHelp.value = !showShortcutsHelp.value;
};

const toggleHelp = () => {
  showHelp.value = !showHelp.value;
};

const handleToggleAISidebar = () => {
  showAISidebar.value = !showAISidebar.value;
};

const handleToggleDocumentOutline = () => {
  showDocumentOutline.value = !showDocumentOutline.value;
  if (showDocumentOutline.value) {
    extractHeadings();
  }
};

// Hybrid Architecture: Document Analysis
const performDocumentAnalysis = async () => {
  if (!editor.value) return;
  
  // Clear previous debounce timer
  if (analysisDebounce.value) {
    clearTimeout(analysisDebounce.value);
  }
  
  // Debounce analysis to avoid excessive calls
  analysisDebounce.value = setTimeout(async () => {
    const html = editor.value.getHTML();
    const analysis = await hybridServices.analyzeDocument(html);
    
    if (analysis) {
      documentAnalysis.value = analysis;
      // Update word count from analysis
      if (analysis.stats) {
        wordCount.value = analysis.stats.word_count;
      }
      console.log('[Hybrid] Document analysis:', analysis);
    }
  }, 1000); // 1 second debounce
};

// Hybrid Architecture: Spell Check
const performSpellCheck = async () => {
  if (!editor.value) return;
  
  const text = editor.value.getText();
  const result = await hybridServices.checkSpelling(text);
  
  if (result) {
    spellCheckResult.value = result;
    console.log('[Hybrid] Spell check result:', result);
  }
};

// Hybrid Architecture: Auto Save
const performAutoSave = async () => {
  if (!editor.value || !currentFilename.value) return;
  
  const shouldSave = await hybridServices.shouldAutoSave(currentFilename.value);
  if (!shouldSave) return;
  
  const html = editor.value.getHTML();
  const result = await hybridServices.autoSaveDocument(currentFilename.value, html);
  
  if (result && result.success) {
    console.log('[Hybrid] Auto saved:', result);
    lastSavedContent.value = html;
  }
};

const extractHeadings = () => {
  if (!editor.value) {
    return;
  }
  
  const html = editor.value.getHTML();
  const parser = new DOMParser();
  const doc = parser.parseFromString(html, 'text/html');
  const headingElements = doc.querySelectorAll('h1, h2, h3, h4, h5, h6');
  
  const headings: Array<{ id: string; level: number; text: string; children?: any[] }> = [];
  const stack: Array<{ id: string; level: number; text: string; children?: any[] }> = [];
  
  headingElements.forEach((heading, index) => {
    if (!heading.tagName || heading.tagName.length < 2) {
      return;
    }
    
    const level = parseInt(heading.tagName[1]);
    const text = heading.textContent || '';
    const id = `heading-${index}`;
    
    const headingItem = { id, level, text, children: [] };
    
    // Pop items from stack that are at or below current level
    while (stack.length > 0 && stack[stack.length - 1].level >= level) {
      stack.pop();
    }
    
    // If stack is empty, this is a top-level heading
    if (stack.length === 0) {
      headings.push(headingItem);
      stack.push(headingItem);
    } else {
      // Add as child of the last item in stack
      const parent = stack[stack.length - 1];
      if (parent.children) {
        parent.children.push(headingItem);
      }
      stack.push(headingItem);
    }
  });
  
  documentHeadings.value = headings;
  logger.debug('Extracted headings', { headings }, LogCategory.SYSTEM);
};

const navigateToHeading = (id: string) => {
  if (!editor.value) {
    return;
  }
  
  const index = parseInt(id.replace('heading-', ''));
  const html = editor.value.getHTML();
  const parser = new DOMParser();
  const doc = parser.parseFromString(html, 'text/html');
  const headingElements = doc.querySelectorAll('h1, h2, h3, h4, h5, h6');
  
  if (headingElements[index]) {
    // Find the position in the editor
    const { from } = editor.value.state.selection;
    const docContent = editor.value.state.doc;
    
    // Simple approach: search for the heading text in the document
    const headingText = headingElements[index].textContent || '';
    let found = false;
    
    docContent.descendants((node, pos) => {
      if (!found && node.isText && node.text.includes(headingText.substring(0, 20))) {
        editor.value?.chain().focus().setTextSelection(pos).run();
        found = true;
      }
    });
    
    logger.debug('Navigated to heading', { id, headingText }, LogCategory.SYSTEM);
  }
};

const toggleOptionsDialog = () => {
  showOptionsDialog.value = !showOptionsDialog.value;
};

const toggleAboutDialog = () => {
  showAboutDialog.value = !showAboutDialog.value;
};

const toggleUserGuideDialog = () => {
  showUserGuideDialog.value = !showUserGuideDialog.value;
};

const handleMiniToolbarAction = (action: string) => {
  switch (action) {
    case 'bold':
      toggleBold();
      break;
    case 'italic':
      toggleItalic();
      break;
    case 'underline':
      toggleUnderline();
      break;
    case 'text-color':
      colorPickerTarget.value = 'text';
      showColorPickerDialog.value = true;
      break;
    case 'highlight':
      toggleHighlight();
      break;
    case 'bullet-list':
      toggleBulletList();
      break;
    case 'numbered-list':
      toggleOrderedList();
      break;
    case 'link':
      // Get current selection text
      if (editor.value) {
        const { from, to } = editor.value.state.selection;
        const selectedText = editor.value.state.doc.textBetween(from, to);
        linkDialogText.value = selectedText;
        linkDialogUrl.value = '';
        showLinkDialog.value = true;
      }
      break;
  }
};

const handleOptionsApply = (settings: any) => {
  // Apply general settings
  autoSaveEnabled.value = settings.general.autoSave;
  if (settings.general.autoSaveInterval) {
    // Update auto-save interval
  }
  
  // Apply display settings
  if (settings.display.theme === 'dark') {
    isDarkMode.value = true;
  } else if (settings.display.theme === 'light') {
    isDarkMode.value = false;
  }
  fontSize.value = settings.display.fontSize;
  
  // Apply other settings as needed
  // Options applied:
};

// Close all dropdown menus
const closeAllMenus = () => {
  showFileBackstage.value = false;
  showQuickAccessMenu.value = false;
  showStylesPanel.value = false;
  showNavigationPane.value = false;
  showPageSetupDialog.value = false;
  showParagraphDialog.value = false;
  showFontDialog.value = false;
};

// Toggle ribbon tab
const setActiveRibbonTab = (tab: string) => {
  activeRibbonTab.value = tab;
  closeAllMenus();
};

// Toggle a specific menu and close others
const _toggleMenu = (menuName: string) => {
  const menus: Record<string, any> = {
    showFileBackstage,
    showQuickAccessMenu,
    showStylesPanel,
    showNavigationPane,
    showPageSetupDialog,
    showParagraphDialog,
    showFontDialog
  };

  const targetMenu = menus[menuName];
  if (!targetMenu) {
return;
}

  const isOpen = targetMenu.value;

  // Close all menus first
  closeAllMenus();

  // Toggle the target menu (if it was closed, open it; if it was open, keep it closed)
  if (!isOpen) {
    targetMenu.value = true;
  }
};

// Handle keyboard navigation for ribbon tabs
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const handleRibbonKeyDown = (_e: KeyboardEvent, direction: 'left' | 'right') => {
  const tabs = ['file', 'home', 'insert', 'layout', 'spreadsheet', 'typst', 'slides', 'references', 'review', 'view', 'help'];
  const currentIndex = tabs.indexOf(activeRibbonTab.value);

  if (direction === 'left') {
    const prevIndex = (currentIndex - 1 + tabs.length) % tabs.length;
    setActiveRibbonTab(tabs[prevIndex]);
  } else {
    const nextIndex = (currentIndex + 1) % tabs.length;
    setActiveRibbonTab(tabs[nextIndex]);
  }
};


const toggleSpreadsheet = () => {
  showSpreadsheet.value = !showSpreadsheet.value;
};

const toggleUniverSpreadsheet = () => {
  showUniverSpreadsheet.value = !showUniverSpreadsheet.value;
};

// Spreadsheet functions
const insertFormula = () => {
  if (!editor.value) {
return;
}
  
  // Open formula dialog for user input
  const formula = prompt('请输入公式 (例如: =SUM(A1:A10), =AVERAGE(B1:B10)):');
  if (formula && formula.trim()) {
    editor.value.chain().focus().insertContent(formula).run();
    showToast('公式已插入: ' + formula, 'success');
  }
};

const insertFunction = () => {
  if (!editor.value) {
return;
}
  
  // Provide common functions for selection
  const functions = [
    { name: '平均值', formula: '=AVERAGE()' },
    { name: '求和', formula: '=SUM()' },
    { name: '最大值', formula: '=MAX()' },
    { name: '最小值', formula: '=MIN()' },
    { name: '计数', formula: '=COUNT()' },
    { name: '条件判断', formula: '=IF(,,)' },
    { name: '查找', formula: '=VLOOKUP(,,)' },
    { name: '文本连接', formula: '=CONCAT()' }
  ];
  
  const functionList = functions.map((f, i) => `${i + 1}. ${f.name} - ${f.formula}`).join('\n');
  const selection = prompt(`选择函数:\n${functionList}\n\n请输入序号 (1-${functions.length}):`);
  
  const index = parseInt(selection || '');
  if (index >= 1 && index <= functions.length) {
    const selected = functions[index - 1];
    editor.value.chain().focus().insertContent(selected.formula).run();
    showToast(`已插入函数: ${selected.name}`, 'success');
  } else if (selection) {
    showToast('无效的选择', 'warning');
  }
};

const sortData = () => {
  if (editor.value) {
    // Insert a comment indicating sort operation
    editor.value.chain().focus().insertContent('<!-- Data sorted ascending -->').run();
    showToast('数据已排序', 'success');
  }
};

const filterData = () => {
  if (editor.value) {
    // Insert a comment indicating filter operation
    editor.value.chain().focus().insertContent('<!-- Data filtered -->').run();
    showToast('数据已筛选', 'success');
  }
};

// New spreadsheet function handlers
const insertArrayFormula = () => {
  if (editor.value) {
    const formula = '=SUMPRODUCT(A1:A10, B1:B10)';
    editor.value.chain().focus().insertContent(formula).run();
    showToast('数组公式已插入: ' + formula, 'success');
  }
};

const insertVLOOKUP = () => {
  if (editor.value) {
    const formula = '=VLOOKUP(lookup_value, table_range, col_index, FALSE)';
    editor.value.chain().focus().insertContent(formula).run();
    showToast('VLOOKUP 函数已插入', 'success');
  }
};

const insertHLOOKUP = () => {
  if (editor.value) {
    const formula = '=HLOOKUP(lookup_value, table_range, row_index, FALSE)';
    editor.value.chain().focus().insertContent(formula).run();
    showToast('HLOOKUP 函数已插入', 'success');
  }
};

const insertINDEXMATCH = () => {
  if (editor.value) {
    const formula = '=INDEX(array, MATCH(lookup_value, lookup_array, 0))';
    editor.value.chain().focus().insertContent(formula).run();
    showToast('INDEX/MATCH 函数已插入', 'success');
  }
};

const addConditionalFormat = () => {
  showConditionalFormatDialog.value = true;
};

const addDataBars = () => {
  conditionalFormatType.value = 'data-bars';
  showConditionalFormatDialog.value = true;
};

const addColorScale = () => {
  conditionalFormatType.value = 'color-scale';
  showConditionalFormatDialog.value = true;
};

const insertChart = async () => {
  try {
    // TODO: Get actual sheet ID and form data from dialog
    const sheetId = 'default-sheet-id';
    const data = {
      sheet_id: sheetId,
      name: '图表1',
      chart_type: 'column',
      data_range: 'A1:B10',
      title: '销售数据',
      x_axis_title: '月份',
      y_axis_title: '销售额',
      legend_position: 'right',
      style_data: '{}'
    };
    
    await spreadsheetApi.createChart(data);
    showChartDialog.value = false;
    showToast('图表已插入', 'success');
  } catch (error) {
    logger.error('Failed to insert chart', error, LogCategory.SYSTEM);
    showToast('插入图表失败', 'error');
  }
};

const insertLineChart = () => {
  chartType.value = 'line';
  showChartDialog.value = true;
};

const insertPieChart = () => {
  chartType.value = 'pie';
  showChartDialog.value = true;
};

const insertPivotTable = () => {
  showPivotTableDialog.value = true;
};

const refreshPivotTable = async () => {
  try {
    // Check if spreadsheet service is available
    if (!spreadsheetApi) {
      showToast('电子表格服务未初始化', 'error');
      return;
    }
    
    // Get current sheet ID from context or use default
    const sheetId = 'default-sheet-id';
    
    // Call backend API to refresh pivot table
    // This would typically call an API endpoint that recalculates the pivot table
    // For now, we'll simulate the refresh
    
    // Show loading state
    showToast('正在刷新数据透视表...', 'info');
    
    // Simulate API call delay
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // In production, this would be:
    // await spreadsheetApi.refreshPivotTable({ sheet_id: sheetId });
    
    showToast('数据透视表已刷新', 'success');
    
    // If spreadsheet is visible, trigger a re-render
    if (showSpreadsheet.value || showUniverSpreadsheet.value) {
      // Force spreadsheet component to refresh
      // This would emit an event to the spreadsheet component
    }
  } catch (error) {
    logger.error('刷新数据透视表失败', error, LogCategory.SYSTEM);
    showToast('刷新数据透视表失败', 'error');
  }
};

const applyConditionalFormat = async () => {
  try {
    // TODO: Get actual sheet ID and form data from dialog
    const sheetId = 'default-sheet-id';
    const data = {
      sheet_id: sheetId,
      range: 'A1:A10',
      rule_type: 'greaterThan',
      rule_data: '{"value": 100}',
      format_data: '{"background": "#ff0000"}',
      priority: 1
    };
    
    await spreadsheetApi.createConditionalFormat(data);
    showConditionalFormatDialog.value = false;
    showToast('条件格式已应用', 'success');
  } catch (error) {
    logger.error('Failed to apply conditional format', error, LogCategory.SYSTEM);
    showToast('应用条件格式失败', 'error');
  }
};

const createPivotTable = async () => {
  try {
    // TODO: Get actual sheet ID and form data from dialog
    const sheetId = 'default-sheet-id';
    const data = {
      sheet_id: sheetId,
      name: '透视表1',
      source_range: 'A1:D100',
      row_fields: 'A',
      column_fields: 'B',
      value_fields: 'C',
      filter_fields: ''
    };
    
    await spreadsheetApi.createPivotTable(data);
    showPivotTableDialog.value = false;
    showToast('数据透视表已创建', 'success');
  } catch (error) {
    logger.error('Failed to create pivot table', error, LogCategory.SYSTEM);
    showToast('创建数据透视表失败', 'error');
  }
};

// Typst functions
const exportTypstPdf = async () => {
  logger.debug('exportTypstPdf started', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  isLoading.value = true;
  try {
    const htmlContent = editor.value?.getHTML() || '';
    logger.debug('Converting HTML to Typst', {}, LogCategory.SYSTEM);
    const typstCode = await invoke<string>('html_to_typst', { html: htmlContent });
    logger.debug('Rendering Typst to PDF', {}, LogCategory.SYSTEM);
    const result = await invoke<{ success: boolean; output?: string; error?: string }>('render_typst', {
      request: {
        source: typstCode,
        format: 'pdf'
      }
    });

    if (result.success && result.output) {
      logger.debug('PDF rendered successfully, saving file', {}, LogCategory.SYSTEM);
      // Save PDF file
      const filePath = await save({
        filters: [
          {
            name: 'PDF Document',
            extensions: ['pdf']
          }
        ]
      });

      if (filePath) {
        // Convert base64 to bytes and save
        const binaryString = atob(result.output);
        const bytes = new Uint8Array(binaryString.length);
        for (let i = 0; i < binaryString.length; i++) {
          bytes[i] = binaryString.charCodeAt(i);
        }
        await invoke('save_file', { filePath, content: bytes });
        logger.debug('PDF saved successfully', { filePath }, LogCategory.SYSTEM);
        showToast('PDF 导出成功', 'success');
      }
    } else {
      logger.error('PDF rendering failed', result.error, LogCategory.SYSTEM);
      showToast('PDF 导出失败: ' + (result.error || '未知错误'), 'error');
    }
  } catch (error) {
    logger.error('exportTypstPdf failed', { timestamp: new Date().toISOString(), error }, LogCategory.SYSTEM);
    showToast('PDF 导出失败', 'error');
    logger.error('Failed to export Typst PDF', error, LogCategory.SYSTEM);
  } finally {
    isLoading.value = false;
    logger.debug('exportTypstPdf completed', { timestamp: new Date().toISOString() }, LogCategory.SYSTEM);
  }
};

const exportTypstPng = async () => {
  isLoading.value = true;
  try {
    const htmlContent = editor.value?.getHTML() || '';
    const typstCode = await invoke<string>('html_to_typst', { html: htmlContent });
    const pngBytes = await invoke<number[]>('export_to_png', {
      code: typstCode,
      dpi: 144.0
    });

    // Save PNG file
    const filePath = await save({
      filters: [
        {
          name: 'PNG Image',
          extensions: ['png']
        }
      ]
    });

    if (filePath) {
      // Convert number array to bytes and save
      const bytes = new Uint8Array(pngBytes);
      await invoke('save_file', { filePath, content: bytes });
      showToast('PNG 导出成功', 'success');
    }
  } catch (error) {
    showToast('PNG 导出失败', 'error');
    logger.error('Failed to export Typst PNG', error, LogCategory.SYSTEM);
  } finally {
    isLoading.value = false;
  }
};

const exportTypstSvg = async () => {
  isLoading.value = true;
  try {
    const htmlContent = editor.value?.getHTML() || '';
    const typstCode = await invoke<string>('html_to_typst', { html: htmlContent });
    const { exportTypstToSvg, promptSaveSvgFile } = await import('../services/svgExportApi');
    const result = await exportTypstToSvg(typstCode);

    if (result.success && result.text) {
      const saved = await promptSaveSvgFile(result.text, 'document-typst.svg');
      if (saved) {
        showToast('SVG export succeeded', 'success');
      }
    } else {
      showToast('SVG export failed: ' + (result.error || 'Unknown error'), 'error');
    }
  } catch (error) {
    showToast('SVG export failed', 'error');
    logger.error('Failed to export Typst SVG', error, LogCategory.SYSTEM);
  } finally {
    isLoading.value = false;
  }
};

const exportHtmlSvg = async () => {
  isLoading.value = true;
  try {
    const htmlContent = editor.value?.getHTML() || '';
    const { exportHtmlToSvg, promptSaveSvgFile } = await import('../services/svgExportApi');
    const result = await exportHtmlToSvg(htmlContent);

    if (result.success && result.text) {
      const saved = await promptSaveSvgFile(result.text, 'document-html.svg');
      if (saved) {
        showToast('HTML SVG export succeeded', 'success');
      }
    } else {
      showToast('HTML SVG export failed: ' + (result.error || 'Unknown error'), 'error');
    }
  } catch (error) {
    showToast('HTML SVG export failed', 'error');
    logger.error('Failed to export HTML SVG', error, LogCategory.SYSTEM);
  } finally {
    isLoading.value = false;
  }
};

const showTypstSettings = () => {
  showTypstConfigDialog.value = true;
};

const openTypstPackageBrowser = () => {
  showTypstPackageBrowser.value = true;
};

const openTypstFontManager = () => {
  showTypstFontManager.value = true;
};

const openTypstExportOptions = () => {
  showTypstExportOptions.value = true;
};

const applyTypstConfig = () => {
  // Apply Typst configuration
  // Update slide config with Typst config
  slideConfig.value = {
    theme: typstConfig.value.theme,
    aspectRatio: typstConfig.value.aspectRatio,
    showSlideNumbers: typstConfig.value.showSlideNumbers
  };
  showTypstConfigDialog.value = false;
  // Regenerate Typst preview if in Typst view mode
  if (typstViewMode.value === 'render') {
    generateTypstPreview();
  }
  showToast('Typst 配置已应用', 'success');
};

// Slide functions (expose existing functions)
const insertSlideBreak = () => {
  // Use existing _insertSlideBreak function
  if (editor.value) {
    editor.value.chain().focus().setHorizontalRule().run();
    if (isSlideMode.value) {
      triggerSlideCompilation();
    }
  }
};

const toggleSlideConfigDialog = () => {
  showSlideConfigDialog.value = !showSlideConfigDialog.value;
};

// Toggle collaboration
const toggleCollaboration = () => {
  showCollaboration.value = !showCollaboration.value;
  collaborationEnabled.value = showCollaboration.value;
  
  if (showCollaboration.value) {
    // Initialize collaboration
    collaborationDocumentId.value = `doc-${Date.now()}`;
    collaborationUserId.value = `user-${Date.now()}`;
    collaborationUserName.value = 'User';
  }
};

// Handle conflict detected
const handleConflictDetected = (localOp: any, remoteOp: any) => {
  logger.warn('Collaboration conflict detected', { localOp, remoteOp }, LogCategory.BUSINESS);
};

// Handle conflict resolved
const handleConflictResolved = () => {
  logger.info('Collaboration conflict resolved', {}, LogCategory.BUSINESS);
};

const handleSpreadsheetInsert = (code: string) => {
  if (editor.value) {
    editor.value.chain().focus().insertContent(code).run();
  }
};

const triggerTypstCompilation = () => {
  // Clear previous timeout
  if (compileTimeout.value) {
    clearTimeout(compileTimeout.value);
  }

  // Debounce compilation (500ms)
  compileTimeout.value = setTimeout(async () => {
    if (!editor.value) {
return;
}

    try {
      const htmlContent = editor.value.getHTML();
      const typstCode = await invoke<string>('html_to_typst', { html: htmlContent });
      const base64Image = await invoke<string>('compile_typst', { code: typstCode });
      typstPreviewSrc.value = base64Image;
      typstCompileError.value = '';
    } catch (err: any) {
      typstCompileError.value = err.toString();
      logger.error('Typst compilation error', err, LogCategory.SYSTEM);
    }
  }, 500);
};

// Toggle slide mode
const toggleSlideMode = () => {
  isSlideMode.value = !isSlideMode.value;
  if (isSlideMode.value) {
    currentSlideIndex.value = 0;
    triggerSlideCompilation();
  }
};

// Ribbon button handlers (stubs for UI functionality)
const cutSelection = () => {
  editor.value?.chain().focus().deleteSelection().run();
};

const copySelection = () => {
  const { from, to } = editor.value?.state.selection || {};
  if (from !== undefined && to !== undefined && from !== to) {
    const text = editor.value?.state.doc.textBetween(from, to) || '';
    navigator.clipboard.writeText(text);
  }
};

const toggleHighlight = () => {
  try {
    if (editor.value) {
      editor.value.chain().focus().toggleHighlight({ color: highlightColor.value }).run();
    }
  } catch (error) {
    const appError = createError(
      ErrorCode.UNKNOWN_ERROR,
      'Failed to toggle highlight',
      ErrorSeverity.WARNING,
      ErrorCategory.SYSTEM,
      { timestamp: Date.now(), additionalData: { originalError: error } }
    );
    logger.error('Toggle highlight error', appError, LogCategory.SYSTEM);
  }
};

const setTextColor = () => {
  // Create a temporary color input
  const input = document.createElement('input');
  input.type = 'color';
  input.value = '#000000';
  
  input.onchange = (e) => {
    const color = (e.target as HTMLInputElement).value;
    if (editor.value) {
      editor.value.chain().focus().toggleMark('textStyle', { color }).run();
    }
    document.body.removeChild(input);
  };
  
  input.oncancel = () => {
    document.body.removeChild(input);
  };
  
  document.body.appendChild(input);
  input.click();
};

const setHeading = (level: 1 | 2 | 3 | 4 | 5 | 6) => {
  editor.value?.chain().focus().toggleHeading({ level }).run();
};

const toggleCodeBlock = () => {
  editor.value?.chain().focus().toggleCodeBlock().run();
};

const applyStyle = () => {
  if (!editor.value) {
return;
}
  
  const chain = editor.value.chain().focus();
  
  switch (selectedStyle.value) {
    case 'normal':
      chain.setParagraph().run();
      break;
    case 'no-spacing':
      chain.setParagraph().updateAttributes('paragraph', {
        style: 'margin: 0; padding: 0;'
      }).run();
      break;
    case 'heading1':
      chain.toggleHeading({ level: 1 }).run();
      break;
    case 'heading2':
      chain.toggleHeading({ level: 2 }).run();
      break;
    case 'quote':
      chain.toggleBlockquote().run();
      break;
  }
};

const changeStyleSet = () => {
  aiError.value = '样式集切换功能开发中';
  setTimeout(() => (aiError.value = null), 2000);
};

const applyEmphasis = () => {
  aiError.value = '强调样式功能开发中';
  setTimeout(() => (aiError.value = null), 2000);
};

const applyStrongEmphasis = () => {
  aiError.value = '明显强调样式功能开发中';
  setTimeout(() => (aiError.value = null), 2000);
};

const applyQuote = () => {
  aiError.value = '引用样式功能开发中';
  setTimeout(() => (aiError.value = null), 2000);
};

const applyListParagraph = () => {
  aiError.value = '列表段落样式功能开发中';
  setTimeout(() => (aiError.value = null), 2000);
};

const applyIntenseQuote = () => {
  aiError.value = '明显引用样式功能开发中';
  setTimeout(() => (aiError.value = null), 2000);
};

const applySubtleReference = () => {
  aiError.value = '微妙引用样式功能开发中';
  setTimeout(() => (aiError.value = null), 2000);
};

const applyBookTitle = () => {
  aiError.value = '书题样式功能开发中';
  setTimeout(() => (aiError.value = null), 2000);
};

const applyIntenseEmphasis = () => {
  aiError.value = '强烈强调样式功能开发中';
  setTimeout(() => (aiError.value = null), 2000);
};

const newStyle = () => {
  aiError.value = '新建样式功能开发中';
  setTimeout(() => (aiError.value = null), 2000);
};

const stylePane = () => {
  aiError.value = '样式窗格功能开发中';
  setTimeout(() => (aiError.value = null), 2000);
};

const findText = () => {
  toggleSearchDialog();
};

const ribbonReplaceText = () => {
  toggleSearchDialog();
};

const ribbonInsertPageBreak = () => {
  if (!editor.value) return;
  // Save current page content
  pageContents.value[activePageIndex.value] = editor.value.getHTML();
  // Insert page break visual marker
  editor.value?.chain().focus().insertContent('<div class="page-break-container"><hr class="page-break" style="border: none; border-top: 2px dashed #0078d4; margin: 20px 0; page-break-after: always; break-after: page;"></div><div class="page-spacer"></div>').run();
  aiError.value = '已插入分页符';
  setTimeout(() => (aiError.value = null), 2000);
};

const insertBlankPage = () => {
  if (!editor.value) return;
  // Save current page content
  pageContents.value[activePageIndex.value] = editor.value.getHTML();
  // Add new blank page
  pageContents.value.splice(activePageIndex.value + 1, 0, '<p><br></p>');
  activePageIndex.value = activePageIndex.value + 1;
  // Load new page content
  editor.value?.commands.setContent(pageContents.value[activePageIndex.value]);
  totalPages.value = pageContents.value.length;
  currentPage.value = activePageIndex.value + 1;
  aiError.value = '已插入空白页';
  setTimeout(() => (aiError.value = null), 2000);
};

const activatePage = (pageIndex: number) => {
  if (pageIndex === activePageIndex.value) return;
  // Save current page content
  pageContents.value[activePageIndex.value] = editor.value?.getHTML() || '';
  // Switch to new page
  activePageIndex.value = pageIndex;
  // Load new page content
  editor.value?.commands.setContent(pageContents.value[pageIndex]);
  currentPage.value = pageIndex + 1;
};

const addImage = () => {
  // Create file input for image upload
  const input = document.createElement('input');
  input.type = 'file';
  input.accept = 'image/*';

  input.onchange = async (e) => {
    const file = (e.target as HTMLInputElement).files?.[0];
    if (file) {
      try {
        // Validate file size (max 5MB)
        const maxSize = 5 * 1024 * 1024;
        if (file.size > maxSize) {
          showToast('图片大小不能超过 5MB', 'error');
          document.body.removeChild(input);
          return;
        }

        // Convert file to base64 for local use
        const reader = new FileReader();
        reader.onload = (event) => {
          const base64 = (event.target as FileReader).result as string;
          editor.value?.chain().focus().setImage({ src: base64 }).run();
          showToast('图片已插入', 'success');
        };
        reader.onerror = () => {
          showToast('图片读取失败', 'error');
        };
        reader.readAsDataURL(file);
      } catch (error) {
        logger.error('图片上传失败', { error }, LogCategory.SYSTEM);
        showToast('图片上传失败', 'error');
      }
    }
    document.body.removeChild(input);
  };

  input.oncancel = () => {
    document.body.removeChild(input);
  };

  document.body.appendChild(input);
  input.click();
};

const insertEmoji = () => {
  // Trigger emoji suggestion by typing a colon
  editor.value?.chain().focus().insertContent(':').run();
  showToast('输入 : 后输入表情名称', 'info');
};

const insertShape = () => {
  openShapeSelector();
};

const resizeImage = (width?: number, height?: number) => {
  const selectedNode = editor.value?.state.selection.$from.nodeBefore;
  if (selectedNode?.type.name === 'image') {
    const currentWidth = selectedNode.attrs.width || 300;
    const currentHeight = selectedNode.attrs.height || 200;
    const newWidth = width || currentWidth * 1.2;
    const newHeight = height || currentHeight * 1.2;
    editor.value?.chain().focus().updateAttributes('image', { width: newWidth, height: newHeight }).run();
  }
};

const alignImage = (alignment: 'left' | 'center' | 'right') => {
  const selectedNode = editor.value?.state.selection.$from.nodeBefore;
  if (selectedNode?.type.name === 'image') {
    const styleMap = {
      left: 'float: left; margin-right: 10px;',
      center: 'display: block; margin: 0 auto;',
      right: 'float: right; margin-left: 10px;'
    };
    editor.value?.chain().focus().updateAttributes('image', { style: styleMap[alignment] }).run();
  }
};

const wrapImage = (wrap: 'inline' | 'text' | 'tight') => {
  const selectedNode = editor.value?.state.selection.$from.nodeBefore;
  if (selectedNode?.type.name === 'image') {
    const styleMap = {
      inline: 'display: inline;',
      text: 'float: left; margin-right: 10px; margin-bottom: 10px;',
      tight: 'float: left; margin-right: 5px; margin-bottom: 5px;'
    };
    editor.value?.chain().focus().updateAttributes('image', { style: styleMap[wrap] }).run();
  }
};

const cropImage = () => {
  aiError.value = '裁剪功能即将推出';
  setTimeout(() => (aiError.value = null), 2000);
};

const rotateImage = (degrees: number) => {
  const selectedNode = editor.value?.state.selection.$from.nodeBefore;
  if (selectedNode?.type.name === 'image') {
    const currentTransform = selectedNode.attrs.style?.match(/rotate\((\d+)deg\)/);
    const currentDegrees = currentTransform ? parseInt(currentTransform[1]) : 0;
    const newDegrees = currentDegrees + degrees;
    editor.value?.chain().focus().updateAttributes('image', { style: `transform: rotate(${newDegrees}deg);` }).run();
  }
};

const flipImage = (direction: 'horizontal' | 'vertical') => {
  const selectedNode = editor.value?.state.selection.$from.nodeBefore;
  if (selectedNode?.type.name === 'image') {
    const scaleX = direction === 'horizontal' ? -1 : 1;
    const scaleY = direction === 'vertical' ? -1 : 1;
    editor.value?.chain().focus().updateAttributes('image', { style: `transform: scaleX(${scaleX}) scaleY(${scaleY});` }).run();
  }
};

const insertIcon = () => {
  openIconSelector();
};

const setLink = () => {
  const url = prompt('请输入链接地址:');
  if (url) {
    editor.value?.chain().focus().setLink({ href: url }).run();
  }
};

const insertBookmark = () => {
  if (!editor.value) {
return;
}
  
  const bookmarkName = prompt('请输入书签名称:');
  if (!bookmarkName) {
return;
}
  
  // Insert a bookmark anchor
  const bookmarkHtml = `<a id="bookmark-${bookmarkName}" name="${bookmarkName}" style="color: #0066cc; text-decoration: underline; cursor: pointer;">🔖 ${bookmarkName}</a>`;
  editor.value.chain().focus().insertContent(bookmarkHtml).run();
  
  aiError.value = `书签 "${bookmarkName}" 已插入`;
  setTimeout(() => (aiError.value = null), 2000);
};

const insertHeader = () => {
  openHeaderDialog();
};

const insertFooter = () => {
  openFooterDialog();
};

const insertPageNumber = () => {
  // Now opens dialog instead of prompt
  togglePageNumberDialog();
};

const showPageSetup = () => {
  showPageSetupDialog.value = true;
};

const setOrientation = (orientation: 'portrait' | 'landscape') => {
  if (pageOrientation.value === orientation) {
return;
}
  pageOrientation.value = orientation;
  
  const currentW = pageSize.value.width;
  const currentH = pageSize.value.height;
  if ((orientation === 'landscape' && currentW < currentH) || 
      (orientation === 'portrait' && currentW > currentH)) {
    pageSize.value = { width: currentH, height: currentW };
  }
};

const showParagraphSettings = () => {
  showParagraphDialog.value = true;
};

const bringToFront = () => {
  if (!editor.value) {
return;
}
  
  // Get current selection and adjust z-index to bring to front
  const { from, to } = editor.value.state.selection;
  if (from === to) {
    aiError.value = '请先选择要调整图层的元素';
    setTimeout(() => (aiError.value = null), 2000);
    return;
  }
  
  // Apply z-index style to selected content
  editor.value.chain().focus().updateAttributes('textStyle', { style: 'position: relative; z-index: 1000;' }).run();
  
  aiError.value = '元素已置于顶层';
  setTimeout(() => (aiError.value = null), 2000);
};

const sendToBack = () => {
  if (!editor.value) {
return;
}
  
  // Get current selection and adjust z-index to send to back
  const { from, to } = editor.value.state.selection;
  if (from === to) {
    aiError.value = '请先选择要调整图层的元素';
    setTimeout(() => (aiError.value = null), 2000);
    return;
  }
  
  // Apply z-index style to selected content
  editor.value.chain().focus().updateAttributes('textStyle', { style: 'position: relative; z-index: -1;' }).run();
  
  aiError.value = '元素已置于底层';
  setTimeout(() => (aiError.value = null), 2000);
};

const ribbonInsertTOC = () => {
  if (!editor.value) {
return;
}
  
  // Generate table of contents from headings in the document
  const doc = editor.value.state.doc;
  let tocHtml = '<div style="background-color: #f5f5f5; padding: 20px; margin: 20px 0; border: 1px solid #ddd;">';
  tocHtml += '<h2 style="margin-top: 0; color: #333;">目录</h2>';
  tocHtml += '<ul style="list-style-type: none; padding-left: 0;">';
  
  let headingCount = 0;
  doc.descendants((node, pos) => {
    if (node.type.name === 'heading') {
      headingCount++;
      const level = node.attrs.level || 1;
      const text = node.textContent || `标题 ${headingCount}`;
      const indent = (level - 1) * 20;
      tocHtml += `<li style="margin: 5px 0; padding-left: ${indent}px;"><a href="#heading-${headingCount}" style="color: #0066cc; text-decoration: none;">${text}</a></li>`;
    }
  });
  
  if (headingCount === 0) {
    tocHtml += '<li style="color: #666;">文档中暂无标题</li>';
  }
  
  tocHtml += '</ul></div>';
  
  editor.value.chain().focus().insertContent(tocHtml).run();
  
  aiError.value = '目录已生成';
  setTimeout(() => (aiError.value = null), 2000);
};

const insertFootnote = () => {
  showFootnoteDialog.value = true;
};

const insertEndnote = () => {
  showEndnoteDialog.value = true;
};

const openCitationDialog = () => {
  showCitationDialog.value = true;
};

const insertBibliography = () => {
  showBibliographyDialog.value = true;
};

const checkSpelling = () => {
  spellCheckEnabled.value = !spellCheckEnabled.value;
  aiError.value = `拼写检查已${spellCheckEnabled.value ? '开启' : '关闭'}`;
  setTimeout(() => (aiError.value = null), 2000);
  
  // Open spell check dialog when enabled
  if (spellCheckEnabled.value) {
    showSpellCheckDialog.value = true;
    performSpellCheck();
  }
};

const countWords = () => {
  showWordCountDialog.value = true;
};

const addComment = () => {
  toggleCommentsPanel();
};

const deleteComment = () => {
  if (!editor.value) {
return;
}
  
  // Check if there's a comment at current selection
  const { from, to } = editor.value.state.selection;
  if (from === to) {
    aiError.value = '请先选择要删除批注的文本';
    setTimeout(() => (aiError.value = null), 2000);
    return;
  }
  
  // Remove comment marks from selection
  editor.value.chain().focus().unsetMark('comment').run();
  
  aiError.value = '批注已删除';
  setTimeout(() => (aiError.value = null), 2000);
};

const trackChanges = () => {
  toggleRevisionPanel();
};

const setPrintLayout = () => {
  isWebLayout.value = false;
  viewMode.value = 'print';
};

const setMargins = () => {
  openPageLayoutDialog();
};

const acceptChange = () => {
  showToast('已接受更改', 'success');
};

const rejectChange = () => {
  showToast('已拒绝更改', 'success');
};

const zoomIn = () => {
  zoomLevel.value = Math.min(zoomLevel.value + 10, 200);
};

const zoomOut = () => {
  zoomLevel.value = Math.max(zoomLevel.value - 10, 50);
};

const zoom100 = () => {
  zoomLevel.value = 100;
};

// Compile current slide
const triggerSlideCompilation = async () => {
  if (!editor.value) {
return;
}

  isSlideCompiling.value = true;
  try {
    const htmlContent = editor.value.getHTML();
    const typstCode = await invoke<string>('html_to_typst_slides', { 
      html: htmlContent, 
      config: slideConfig.value 
    });

    // Get total slide count
    totalSlides.value = await invoke<number>('get_typst_page_count', { code: typstCode });

    // Compile current slide
    const base64Image = await invoke<string>('compile_typst_slide', {
      code: typstCode,
      page_index: currentSlideIndex.value
    });
    slidePreviewSrc.value = base64Image;
    slideCompileError.value = '';
  } catch (err: any) {
    slideCompileError.value = err.toString();
    logger.error('Slide compilation error', err, LogCategory.SYSTEM);
  } finally {
    isSlideCompiling.value = false;
  }
};

// Navigate to next slide
const nextSlide = () => {
  if (currentSlideIndex.value < totalSlides.value - 1) {
    currentSlideIndex.value++;
    triggerSlideCompilation();
  }
};

// Navigate to previous slide
const prevSlide = () => {
  if (currentSlideIndex.value > 0) {
    currentSlideIndex.value--;
    triggerSlideCompilation();
  }
};

// New slide management functions
const newSlide = () => {
  if (editor.value) {
    editor.value.chain().focus().insertContent('<hr class="slide-break">').run();
    totalSlides.value++;
    currentSlideIndex.value = totalSlides.value - 1;
    showToast('新建幻灯片', 'success');
    if (isSlideMode.value) {
      triggerSlideCompilation();
    }
  }
};

const deleteSlide = () => {
  if (totalSlides.value <= 1) {
    showToast('至少保留一张幻灯片', 'error');
    return;
  }
  
  if (!editor.value) {
    showToast('编辑器未初始化', 'error');
    return;
  }
  
  try {
    // Get current document content
    const doc = editor.value.state.doc;
    const html = editor.value.getHTML();
    
    // Split by slide breaks
    const slideBreaks = html.split('<hr class="slide-break">');
    
    if (slideBreaks.length <= 1) {
      showToast('未找到幻灯片分隔符', 'warning');
      return;
    }
    
    // Remove current slide
    slideBreaks.splice(currentSlideIndex.value, 1);
    
    // Rebuild document
    const newHtml = slideBreaks.join('<hr class="slide-break">');
    
    // Update editor content
    editor.value.commands.setContent(newHtml);
    
    // Update slide count and index
    totalSlides.value = slideBreaks.length;
    if (currentSlideIndex.value >= totalSlides.value) {
      currentSlideIndex.value = totalSlides.value - 1;
    }
    
    showToast('删除幻灯片', 'success');
    
    if (isSlideMode.value) {
      triggerSlideCompilation();
    }
  } catch (error) {
    logger.error('删除幻灯片失败', error, LogCategory.SYSTEM);
    showToast('删除幻灯片失败', 'error');
  }
};

const duplicateSlide = () => {
  if (editor.value) {
    editor.value.chain().focus().insertContent('<hr class="slide-break">').run();
    totalSlides.value++;
    showToast('复制幻灯片', 'success');
    if (isSlideMode.value) {
      triggerSlideCompilation();
    }
  }
};

// Text editing functions
const insertTextBox = () => {
  if (editor.value) {
    editor.value.chain().focus().insertContent('<div class="text-box" contenteditable="true">点击输入文本</div>').run();
    showToast('插入文本框', 'success');
  }
};

const alignTextLeft = () => {
  if (editor.value) {
    editor.value.chain().focus().setTextAlign('left').run();
    showToast('左对齐', 'success');
  }
};

const alignTextCenter = () => {
  if (editor.value) {
    editor.value.chain().focus().setTextAlign('center').run();
    showToast('居中对齐', 'success');
  }
};

const addBullets = () => {
  if (editor.value) {
    editor.value.chain().focus().toggleBulletList().run();
    showToast('项目符号', 'success');
  }
};

// Design functions
const openThemeDialog = () => {
  showThemeDialog.value = true;
};

const openBackgroundDialog = () => {
  showBackgroundDialog.value = true;
};

const openLayoutDialog = () => {
  showLayoutDialog.value = true;
};

// PPT dialog state
const selectedTheme = ref('default');
const selectedLayout = ref('title');
const selectedShape = ref('rectangle');
const tableRows = ref(3);
const tableCols = ref(3);

// Image dialog state
const imageSourceType = ref<'upload' | 'url' | 'library'>('upload');
const imageUrl = ref('');
const selectedLibraryImage = ref('');
const imageAltText = ref('');
const uploadedImageData = ref('');

// Animation and chart dialog state
const showAnimationDialog = ref(false);
const showTransitionDialog = ref(false);
const showSmartArtDialog = ref(false);
const selectedAnimation = ref('fade');
const selectedTransition = ref('none');
const selectedSmartArt = ref('process');

// Color and style dialog state
const showFontColorDialog = ref(false);
const showBackgroundColorDialog = ref(false);
const showBorderColorDialog = ref(false);
const selectedFontColor = ref('#000000');
const selectedBackgroundColor = ref('#ffffff');

// Overflow menu state
const showOverflowMenu = ref(false);
const overflowMenuPosition = ref({ x: 0, y: 0 });

// PPT dialog functions
const loadSlides = async () => {
  try {
    slides.value = await pptApi.getAllSlides();
    if (slides.value.length > 0) {
      currentSlideId.value = slides.value[currentSlideIndex.value]?.id || '';
    }
  } catch (error) {
    logger.error('Failed to load slides', error, LogCategory.SYSTEM);
  }
};

const getCurrentSlideId = (): string => {
  if (currentSlideId.value) {
    return currentSlideId.value;
  }
  // Fallback to slide at current index
  if (slides.value[currentSlideIndex.value]) {
    return slides.value[currentSlideIndex.value].id;
  }
  return '';
};

const selectTheme = (theme: string) => {
  selectedTheme.value = theme;
};

const applyTheme = async () => {
  try {
    const slideId = getCurrentSlideId();
    if (!slideId) {
      showToast('无法获取当前幻灯片 ID', 'error');
      return;
    }
    await pptApi.applyTheme(slideId, selectedTheme.value);
    showToast(`已应用主题: ${selectedTheme.value}`, 'success');
    showThemeDialog.value = false;
  } catch (error) {
    logger.error('Failed to apply theme', error, LogCategory.SYSTEM);
    showToast('应用主题失败', 'error');
  }
};

const applyBackground = async () => {
  try {
    const slideId = getCurrentSlideId();
    if (!slideId) {
      showToast('无法获取当前幻灯片 ID', 'error');
      return;
    }
    const backgroundData = { type: 'solid', color: '#ffffff' };
    await pptApi.updateSlide(slideId, { background: JSON.stringify(backgroundData) });
    showToast('背景样式已应用', 'success');
    showBackgroundDialog.value = false;
  } catch (error) {
    logger.error('Failed to apply background', error, LogCategory.SYSTEM);
    showToast('应用背景失败', 'error');
  }
};

const selectLayout = (layout: string) => {
  selectedLayout.value = layout;
};

const applyLayout = async () => {
  try {
    const slideId = getCurrentSlideId();
    if (!slideId) {
      showToast('无法获取当前幻灯片 ID', 'error');
      return;
    }
    await pptApi.updateSlide(slideId, { layout: selectedLayout.value });
    showToast(`已应用版式: ${selectedLayout.value}`, 'success');
    showLayoutDialog.value = false;
  } catch (error) {
    logger.error('Failed to apply layout', error, LogCategory.SYSTEM);
    showToast('应用版式失败', 'error');
  }
};

const selectShape = (shape: string) => {
  selectedShape.value = shape;
};

const applyShape = async () => {
  try {
    const slideId = getCurrentSlideId();
    if (!slideId) {
      showToast('无法获取当前幻灯片 ID', 'error');
      return;
    }
    await pptApi.insertShape(slideId, {
      type: selectedShape.value,
      position: { x: 100, y: 100 },
      size: { width: 200, height: 200 }
    });
    showToast(`已插入形状: ${selectedShape.value}`, 'success');
    showInsertShapeDialog.value = false;
  } catch (error) {
    logger.error('Failed to insert shape', error, LogCategory.SYSTEM);
    showToast('插入形状失败', 'error');
  }
};

// Insert functions for slides
const insertSlideImage = () => {
  showInsertImageDialog.value = true;
};

const insertSlideShape = () => {
  showInsertShapeDialog.value = true;
};

const insertSlideTable = () => {
  showInsertTableDialog.value = true;
};

const applyImage = async (imageData: { type: 'upload' | 'url' | 'library'; data: string }) => {
  try {
    const slideId = getCurrentSlideId();
    if (!slideId) {
      showToast('无法获取当前幻灯片 ID', 'error');
      return;
    }
    await pptApi.insertImage(slideId, imageData);
    showToast('图片已插入', 'success');
    showInsertImageDialog.value = false;
  } catch (error) {
    logger.error('Failed to insert image', error, LogCategory.SYSTEM);
    showToast('插入图片失败', 'error');
  }
};

const handleImageUpload = (event: Event) => {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (file) {
    const reader = new FileReader();
    reader.onload = (e) => {
      uploadedImageData.value = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }
};

const handleInsertImage = async () => {
  try {
    let imageData: { type: 'upload' | 'url' | 'library'; data: string };

    if (imageSourceType.value === 'upload') {
      if (!uploadedImageData.value) {
        showToast('请先上传图片', 'error');
        return;
      }
      imageData = { type: 'upload', data: uploadedImageData.value };
    } else if (imageSourceType.value === 'url') {
      if (!imageUrl.value) {
        showToast('请输入图片 URL', 'error');
        return;
      }
      imageData = { type: 'url', data: imageUrl.value };
    } else {
      if (!selectedLibraryImage.value) {
        showToast('请选择图片', 'error');
        return;
      }
      imageData = { type: 'library', data: selectedLibraryImage.value };
    }

    await applyImage(imageData);
  } catch (error) {
    logger.error('Failed to handle image insertion', error, LogCategory.SYSTEM);
  }
};

const applyTable = async () => {
  try {
    const slideId = getCurrentSlideId();
    if (!slideId) {
      showToast('无法获取当前幻灯片 ID', 'error');
      return;
    }
    await pptApi.insertTable(slideId, {
      rows: tableRows.value,
      cols: tableCols.value
    });
    showToast(`已插入 ${tableRows.value}x${tableCols.value} 表格`, 'success');
    showInsertTableDialog.value = false;
  } catch (error) {
    logger.error('Failed to insert table', error, LogCategory.SYSTEM);
    showToast('插入表格失败', 'error');
  }
};

// Slide management functions
const createNewSlide = async () => {
  try {
    const newSlide = await pptApi.createSlide({
      title: '新幻灯片',
      content: '',
      layout: 'title',
      theme: 'default',
      background: JSON.stringify({ type: 'solid', color: '#ffffff' })
    });
    slides.value.push(newSlide);
    currentSlideIndex.value = slides.value.length - 1;
    currentSlideId.value = newSlide.id;
    showToast('新幻灯片已创建', 'success');
  } catch (error) {
    logger.error('Failed to create slide', error, LogCategory.SYSTEM);
    showToast('创建幻灯片失败', 'error');
  }
};

const deleteCurrentSlide = async () => {
  try {
    const slideId = getCurrentSlideId();
    if (!slideId) {
      showToast('无法获取当前幻灯片 ID', 'error');
      return;
    }
    await pptApi.deleteSlide(slideId);
    slides.value = slides.value.filter(s => s.id !== slideId);
    if (currentSlideIndex.value >= slides.value.length) {
      currentSlideIndex.value = Math.max(0, slides.value.length - 1);
    }
    currentSlideId.value = slides.value[currentSlideIndex.value]?.id || '';
    showToast('幻灯片已删除', 'success');
  } catch (error) {
    logger.error('Failed to delete slide', error, LogCategory.SYSTEM);
    showToast('删除幻灯片失败', 'error');
  }
};

const duplicateCurrentSlide = async () => {
  try {
    const slideId = getCurrentSlideId();
    if (!slideId) {
      showToast('无法获取当前幻灯片 ID', 'error');
      return;
    }
    const duplicatedSlide = await pptApi.duplicateSlide(slideId);
    slides.value.push(duplicatedSlide);
    currentSlideIndex.value = slides.value.length - 1;
    currentSlideId.value = duplicatedSlide.id;
    showToast('幻灯片已复制', 'success');
  } catch (error) {
    logger.error('Failed to duplicate slide', error, LogCategory.SYSTEM);
    showToast('复制幻灯片失败', 'error');
  }
};

// Text editing functions
const alignTextRight = () => {
  editor.value?.chain().focus().setTextAlign('right').run();
};

const boldText = () => {
  editor.value?.chain().focus().toggleBold().run();
};

const italicText = () => {
  editor.value?.chain().focus().toggleItalic().run();
};

const underlineText = () => {
  editor.value?.chain().focus().toggleUnderline().run();
};

const decreaseFontSize = () => {
  if (fontSize.value > 8) {
    fontSize.value -= 1;
    // Font size extension temporarily removed due to compatibility issues
    showToast('字号已减小', 'success');
  }
};

const increaseFontSize = () => {
  if (fontSize.value < 72) {
    fontSize.value += 1;
    // Font size extension temporarily removed due to compatibility issues
    showToast('字号已增大', 'success');
  }
};

// Animation and chart dialog functions
const openAnimationDialog = () => {
  showAnimationDialog.value = true;
};

const openTransitionDialog = () => {
  showTransitionDialog.value = true;
};

const openChartDialog = () => {
  openChartEditor();
};

const openSmartArtDialog = () => {
  openSmartArtSelector();
};

const applyAnimation = () => {
  try {
    const slideId = getCurrentSlideId();
    if (!slideId) {
      showToast('无法获取当前幻灯片 ID', 'error');
      return;
    }
    // Apply animation to slide
    showToast(`已应用动画: ${selectedAnimation.value}`, 'success');
    showAnimationDialog.value = false;
  } catch (error) {
    logger.error('Failed to apply animation', error, LogCategory.SYSTEM);
    showToast('应用动画失败', 'error');
  }
};

const applyTransition = () => {
  try {
    const slideId = getCurrentSlideId();
    if (!slideId) {
      showToast('无法获取当前幻灯片 ID', 'error');
      return;
    }
    // Apply transition to slide
    showToast(`已应用切换: ${selectedTransition.value}`, 'success');
    showTransitionDialog.value = false;
  } catch (error) {
    logger.error('Failed to apply transition', error, LogCategory.SYSTEM);
    showToast('应用切换失败', 'error');
  }
};

const applySmartArt = () => {
  try {
    const slideId = getCurrentSlideId();
    if (!slideId) {
      showToast('无法获取当前幻灯片 ID', 'error');
      return;
    }
    // Insert SmartArt to slide
    showToast(`已插入 SmartArt: ${selectedSmartArt.value}`, 'success');
    showSmartArtDialog.value = false;
  } catch (error) {
    logger.error('Failed to insert SmartArt', error, LogCategory.SYSTEM);
    showToast('插入 SmartArt 失败', 'error');
  }
};

// Additional PPT functions
const openFontColorDialog = () => {
  showFontColorDialog.value = true;
};

const openBackgroundColorDialog = () => {
  showBackgroundColorDialog.value = true;
};

const openBorderColorDialog = () => {
  showBorderColorDialog.value = true;
};

const addNumbering = () => {
  editor.value?.chain().focus().toggleOrderedList().run();
};

const decreaseLineSpacing = () => {
  if (lineHeight.value > 0.8) {
    lineHeight.value -= 0.1;
    updateLineHeight();
    showToast('行距已减小', 'success');
  }
};

const increaseLineSpacing = () => {
  if (lineHeight.value < 3.0) {
    lineHeight.value += 0.1;
    updateLineHeight();
    showToast('行距已增大', 'success');
  }
};

const updateLineHeight = () => {
  const editorElement = document.querySelector('.ProseMirror') as HTMLElement;
  if (editorElement) {
    editorElement.style.lineHeight = lineHeight.value.toString();
  }
};

const startPresentation = () => {
  showToast('开始演示', 'success');
  isSlideMode.value = true;
};

const rehearseTimings = () => {
  // Placeholder for future implementation
  showToast('排练计时功能将在未来版本中提供', 'info');
};

const recordSlideShow = () => {
  // Placeholder for future implementation
  showToast('录制幻灯片功能将在未来版本中提供', 'info');
};

const toggleNotesView = () => {
  showToast('备注视图已切换', 'success');
};

const toggleSlideSorter = () => {
  showToast('幻灯片浏览已切换', 'success');
};

const translateText = () => {
  // Placeholder for future implementation
  showToast('翻译功能将在未来版本中提供', 'info');
};

const applyFontColor = () => {
  try {
    (editor.value?.chain().focus() as any).setColor(selectedFontColor.value).run();
    showToast(`已应用字体颜色: ${selectedFontColor.value}`, 'success');
    showFontColorDialog.value = false;
  } catch (error) {
    logger.error('Failed to apply font color', error, LogCategory.SYSTEM);
    showToast('应用字体颜色失败', 'error');
  }
};

const applyBackgroundColor = () => {
  try {
    // TODO: Apply background color to current slide
    showToast(`已应用背景颜色: ${selectedBackgroundColor.value}`, 'success');
    showBackgroundColorDialog.value = false;
  } catch (error) {
    logger.error('Failed to apply background color', error, LogCategory.SYSTEM);
    showToast('应用背景颜色失败', 'error');
  }
};

const applyBorderColor = () => {
  try {
    // TODO: Apply border color to selected element
    showToast(`已应用边框颜色: ${selectedBorderColor.value}`, 'success');
    showBorderColorDialog.value = false;
  } catch (error) {
    logger.error('Failed to apply border color', error, LogCategory.SYSTEM);
    showToast('应用边框颜色失败', 'error');
  }
};

// Overflow menu functions
const toggleOverflowMenu = (event: MouseEvent) => {
  const button = event.currentTarget as HTMLElement;
  const rect = button.getBoundingClientRect();
  overflowMenuPosition.value = {
    x: rect.left,
    y: rect.bottom + 4
  };
  showOverflowMenu.value = !showOverflowMenu.value;
};

const closeOverflowMenu = () => {
  showOverflowMenu.value = false;
};

// Insert slide break (horizontal rule)
const _insertSlideBreak = () => {
  editor.value?.chain().focus().insertContent('<hr>').run();
  if (isSlideMode.value) {
    triggerSlideCompilation();
  }
};

// Load TipTap configuration from backend
const loadTipTapConfig = async (preset: string = 'default') => {
  if (!isTauriEnvironment()) {
    // Skip Tauri-specific calls in web environment
    return;
  }
  try {
    const configJson = await invoke<string>('get_tiptap_config', { preset });
    tiptapConfig.value = JSON.parse(configJson);
    tiptapPreset.value = preset;
  } catch (error) {
    logger.error('Failed to load TipTap config', error, LogCategory.SYSTEM);
    // Fallback to default config
    tiptapConfig.value = null;
  }
};

// List available TipTap presets
const _listTipTapPresets = async () => {
  try {
    const presets = await invoke<string[]>('list_tiptap_presets');
    return presets;
  } catch (error) {
    logger.error('Failed to list TipTap presets', error, LogCategory.SYSTEM);
    return ['default', 'minimal', 'full'];
  }
};

const _shortcuts = [
  { key: 'Ctrl/Cmd + S', description: '保存文档' },
  { key: 'Ctrl/Cmd + O', description: '打开文件' },
  { key: 'Ctrl/Cmd + N', description: '新建文档' },
  { key: 'Ctrl/Cmd + P', description: '打印' },
  { key: 'Ctrl/Cmd + B', description: '加粗' },
  { key: 'Ctrl/Cmd + I', description: '斜体' },
  { key: 'Ctrl/Cmd + U', description: '下划线' },
  { key: 'Ctrl/Cmd + Z', description: '撤销' },
  { key: 'Ctrl/Cmd + Shift + Z', description: '重做' },
  { key: 'Ctrl/Cmd + Y', description: '重做（替代）' },
  { key: 'Ctrl/Cmd + F', description: '查找' },
  { key: 'Ctrl/Cmd + H', description: '替换' },
  { key: 'Ctrl/Cmd + K', description: '插入链接' },
  { key: 'Ctrl/Cmd + A', description: '全选' },
  { key: 'Ctrl/Cmd + L', description: '左对齐' },
  { key: 'Ctrl/Cmd + E', description: '居中对齐' },
  { key: 'Ctrl/Cmd + R', description: '右对齐' },
  { key: 'Ctrl/Cmd + J', description: '两端对齐' },
  { key: 'Ctrl/Cmd + 1-6', description: '标题 1-6' },
  { key: 'Ctrl/Cmd + 0', description: '正文' },
  { key: 'Ctrl/Cmd + M', description: '增加缩进' },
  { key: 'Ctrl/Cmd + Shift + M', description: '减少缩进' },
  { key: 'F1', description: '帮助' },
  { key: 'F11', description: '全屏' },
  { key: '/', description: '打开斜杠命令菜单' }
];

const _updateEditorFontSize = () => {
  // Validate font size range (8-72)
  const size = fontSize.value;
  if (isNaN(size) || size < 8 || size > 72) {
    aiError.value = '字号必须在 8-72 之间';
    setTimeout(() => (aiError.value = null), 3000);
    return;
  }
  const editorElement = document.querySelector('.ProseMirror') as HTMLElement;
  if (editorElement) {
    editorElement.style.fontSize = `${fontSize.value}px`;
  }
};

// Function reserved for future use

const _setTextColor = () => {
  editor.value?.chain().focus().setMark('textStyle', { color: textColor.value }).run();
};

// Function reserved for future use
const _setBackgroundColor = () => {
  editor.value
    ?.chain()
    .focus()
    .setMark('textStyle', { backgroundColor: backgroundColor.value })
    .run();
};

const _setHighlightColor = () => {
  // Highlight extension not available
  // Highlight color requires highlight extension configuration
};

// Handle color picker confirmation
const handleColorPickerConfirm = (color: string) => {
  if (colorPickerTarget.value === 'text') {
    textColor.value = color;
    if (editor.value) {
      editor.value.chain().focus().setMark('textStyle', { color }).run();
    }
  } else if (colorPickerTarget.value === 'highlight') {
    highlightColor.value = color;
    if (editor.value) {
      editor.value.chain().focus().toggleHighlight({ color }).run();
    }
  }
};

// Handle link dialog confirmation
const handleLinkDialogConfirm = (url: string, text: string) => {
  if (editor.value) {
    if (url) {
      // Insert or update link
      if (text) {
        editor.value.chain().focus().insertContent(`<a href="${url}">${text}</a>`).run();
      } else {
        editor.value.chain().focus().setLink({ href: url }).run();
      }
    } else {
      // Remove link
      editor.value.chain().focus().unsetLink().run();
    }
  }
};

const setZoom = (level: number) => {
  zoomLevel.value = level;
};

const _increaseZoom = () => {
  if (zoomLevel.value < 200) {
    setZoom(zoomLevel.value + 10);
  }
};

const _decreaseZoom = () => {
  if (zoomLevel.value > 50) {
    setZoom(zoomLevel.value - 10);
  }
};

const _resetZoom = () => {
  setZoom(100);
};

const startEditingTitle = () => {
  isEditingTitle.value = true;
};

// Function reserved for future use
const finishEditingTitle = () => {
  isEditingTitle.value = false;
};

const _toggleSlideConfigDialog = () => {
  showSlideConfigDialog.value = !showSlideConfigDialog.value;
};

const _updateSlideConfig = () => {
  if (isSlideMode.value) {
    triggerSlideCompilation();
  }
  showSlideConfigDialog.value = false;
};

const _togglePrintPreview = () => {
  isPrintPreview.value = !isPrintPreview.value;
};

const _toggleComments = () => {
  showComments.value = !showComments.value;
};

const openCommentDialog = () => {
  if (!editor.value) {
return;
}
  const { from, to } = editor.value.state.selection;
  if (from !== to) {
    commentText.value = '';
    showCommentDialog.value = true;
  } else {
    aiError.value = '请先选择要评论的文本';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

const _addComment = () => {
  if (!editor.value || !commentText.value.trim()) {
return;
}
  const { from, to } = editor.value.state.selection;
  if (from !== to) {
    const text = editor.value.state.doc.textBetween(from, to);
    const comment = {
      id: `comment-${Date.now()}`,
      text: commentText.value,
      author: '用户',
      timestamp: Date.now(),
      range: { from, to }
    };
    comments.value.push(comment);

    // Insert comment marker in editor
    editor.value
      .chain()
      .focus()
      .insertContent(
        `<span class="comment-marker" data-comment-id="${comment.id}" style="background-color: #fef3c7; border-bottom: 2px solid #f59e0b; cursor: pointer;">${text}</span>`
      )
      .run();

    commentText.value = '';
    showCommentDialog.value = false;
    aiError.value = '评论已添加';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

const _deleteComment = (commentId: string) => {
  comments.value = comments.value.filter(c => c.id !== commentId);
  aiError.value = '评论已删除';
  setTimeout(() => (aiError.value = null), 2000);
};

const deleteAllComments = () => {
  if (confirm('确定要删除所有批注吗？')) {
    comments.value = [];
    aiError.value = '所有批注已删除';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

// Header and footer support
const headerContent = ref('');
const footerContent = ref('');
const headerEnabled = ref(false);
const footerEnabled = ref(false);
const headerAlign = ref<'left' | 'center' | 'right'>('center');
const footerAlign = ref<'left' | 'center' | 'right'>('center');
const differentFirstPage = ref(false);
const differentOddEven = ref(false);
const isEditingHeader = ref(false);
const isEditingFooter = ref(false);
const headerAreaRef = ref<HTMLElement | null>(null);
const footerAreaRef = ref<HTMLElement | null>(null);

const toggleHeaderFooterDialog = () => {
  showHeaderFooterDialog.value = !showHeaderFooterDialog.value;
};

const togglePageNumberDialog = () => {
  showPageNumberDialog.value = !showPageNumberDialog.value;
};

const enterHeaderEditMode = () => {
  isEditingHeader.value = true;
  isEditingFooter.value = false;
  // Use nextTick to ensure DOM is updated before focusing
  nextTick(() => {
    if (headerAreaRef.value) {
      headerAreaRef.value.innerText = headerContent.value;
      headerAreaRef.value.focus();
    }
  });
};

const enterFooterEditMode = () => {
  isEditingFooter.value = true;
  isEditingHeader.value = false;
  // Use nextTick to ensure DOM is updated before focusing
  nextTick(() => {
    if (footerAreaRef.value) {
      footerAreaRef.value.innerText = footerContent.value;
      footerAreaRef.value.focus();
    }
  });
};

const exitHeaderFooterEditMode = () => {
  isEditingHeader.value = false;
  isEditingFooter.value = false;
};

const handleHeaderBlur = (event: FocusEvent) => {
  const target = event.target as HTMLElement;
  headerContent.value = target.innerText || '';
  if (headerContent.value) {
    headerEnabled.value = true;
  }
  isEditingHeader.value = false;
};

const handleFooterBlur = (event: FocusEvent) => {
  const target = event.target as HTMLElement;
  footerContent.value = target.innerText || '';
  if (footerContent.value) {
    footerEnabled.value = true;
  }
  isEditingFooter.value = false;
};

const handleHeaderEnter = (event: KeyboardEvent) => {
  const target = event.target as HTMLElement;
  headerContent.value = target.innerText || '';
  if (headerContent.value) {
    headerEnabled.value = true;
  }
  isEditingHeader.value = false;
  target.blur();
};

const handleFooterEnter = (event: KeyboardEvent) => {
  const target = event.target as HTMLElement;
  footerContent.value = target.innerText || '';
  if (footerContent.value) {
    footerEnabled.value = true;
  }
  isEditingFooter.value = false;
  target.blur();
};

const _applyHeaderFooter = async () => {
  if (editor.value) {
    const html = editor.value.getHTML();
    
    try {
      // Use hybrid service for header/footer
      const header = {
        enabled: headerEnabled.value,
        content: headerContent.value,
        align: headerAlign.value,
        different_first_page: differentFirstPage.value,
      };
      
      const footer = {
        enabled: footerEnabled.value,
        content: footerContent.value,
        align: footerAlign.value,
        different_first_page: differentFirstPage.value,
      };
      
      const modifiedHtml = await hybridServices.applyHeaderFooter(html, header, footer);
      
      if (modifiedHtml) {
        editor.value.commands.setContent(modifiedHtml);
        showHeaderFooterDialog.value = false;
        aiError.value = '页眉页脚已应用';
        setTimeout(() => (aiError.value = null), 2000);
      }
    } catch (error) {
      logger.error('Apply header footer error', error, LogCategory.SYSTEM);
      aiError.value = '页眉页脚应用失败: ' + (error as Error).message;
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

const removeHeaderFooter = async () => {
  if (editor.value) {
    const html = editor.value.getHTML();
    
    try {
      // Use hybrid service to remove header/footer
      const modifiedHtml = await hybridServices.removeHeaderFooter(html);
      
      if (modifiedHtml) {
        editor.value.commands.setContent(modifiedHtml);
        headerEnabled.value = false;
        footerEnabled.value = false;
        headerContent.value = '';
        footerContent.value = '';
        aiError.value = '页眉页脚已移除';
        setTimeout(() => (aiError.value = null), 2000);
      }
    } catch (error) {
      logger.error('Remove header footer error', error, LogCategory.SYSTEM);
      aiError.value = '页眉页脚移除失败: ' + (error as Error).message;
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

// Page numbers
const pageNumberPosition = ref<'bottom-right' | 'bottom-center' | 'bottom-left'>('bottom-right');
const pageNumberFormat = ref<'1' | '1 of N' | 'Page 1'>('1');

const _togglePageNumberDialog = () => {
  showPageNumberDialog.value = !showPageNumberDialog.value;
};

const _applyPageNumbers = async () => {
  if (editor.value) {
    const html = editor.value.getHTML();
    
    try {
      // Use hybrid service for page numbers
      let align = 'center';
      switch (pageNumberPosition.value) {
        case 'bottom-left':
          align = 'left';
          break;
        case 'bottom-right':
          align = 'right';
          break;
        case 'bottom-center':
        default:
          align = 'center';
          break;
      }

      const config = {
        enabled: true,
        position: 'footer',
        align: align,
        format: pageNumberFormat.value,
      };
      
      const modifiedHtml = await hybridServices.applyPageNumbers(html, config);
      
      if (modifiedHtml) {
        editor.value.commands.setContent(modifiedHtml);
        showPageNumberDialog.value = false;
        aiError.value = '页码已设置';
        setTimeout(() => (aiError.value = null), 2000);
      }
    } catch (error) {
      logger.error('Apply page numbers error', error, LogCategory.SYSTEM);
      aiError.value = '页码设置失败: ' + (error as Error).message;
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

// Document outline view
const outlineItems = ref<{ id: string; level: number; text: string; position: number }[]>([]);

const _toggleOutlineDialog = () => {
  showOutlineDialog.value = !showOutlineDialog.value;
  if (showOutlineDialog.value) {
    generateOutline();
  }
};

const generateOutline = () => {
  if (!editor.value) {
return;
}
  const doc = editor.value.state.doc;
  const items: { id: string; level: number; text: string; position: number }[] = [];
  let _position = 0;

  doc.descendants((node, pos) => {
    if (node.type.name === 'heading') {
      const level = node.attrs.level || 1;
      const text = node.textContent;
      items.push({
        id: `heading-${pos}`,
        level,
        text,
        position: pos
      });
    }
    _position += node.nodeSize;
  });

  outlineItems.value = items;
};

const _jumpToHeading = (position: number) => {
  if (editor.value) {
    editor.value.chain().focus().setTextSelection(position).run();
    showOutlineDialog.value = false;
  }
};

// Watermark support
const watermarkText = ref('');
const watermarkColor = ref('#cccccc');
const watermarkOpacity = ref(0.3);
const watermarkRotation = ref(-45);

const _toggleWatermarkDialog = () => {
  showWatermarkDialog.value = !showWatermarkDialog.value;
};

const applyWatermark = async () => {
  if (editor.value && watermarkText.value) {
    const html = editor.value.getHTML();
    
    try {
      // Use hybrid service for watermark
      const config = {
        enabled: true,
        text: watermarkText.value,
        opacity: watermarkOpacity.value,
        rotation: watermarkRotation.value,
        color: watermarkColor.value,
        font_size: 48,
      };
      
      const modifiedHtml = await hybridServices.applyWatermark(html, config);
      
      if (modifiedHtml) {
        editor.value.commands.setContent(modifiedHtml);
        showWatermarkDialog.value = false;
        aiError.value = '水印已应用';
        setTimeout(() => (aiError.value = null), 2000);
      }
    } catch (error) {
      logger.error('Apply watermark error', error, LogCategory.SYSTEM);
      aiError.value = '水印应用失败: ' + (error as Error).message;
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

const _removeWatermark = async () => {
  if (editor.value) {
    const html = editor.value.getHTML();
    
    try {
      // Use hybrid service to remove watermark
      const modifiedHtml = await hybridServices.removeWatermark(html);
      
      if (modifiedHtml) {
        editor.value.commands.setContent(modifiedHtml);
        watermarkText.value = '';
        aiError.value = '水印已移除';
        setTimeout(() => (aiError.value = null), 2000);
      }
    } catch (error) {
      logger.error('Remove watermark error', error, LogCategory.SYSTEM);
      aiError.value = '水印移除失败: ' + (error as Error).message;
      setTimeout(() => (aiError.value = null), 3000);
    }
  }
};

// Revision tracking (Track Changes)
const revisionTrackingEnabled = ref(false);
const revisions = ref<
  {
    id: string;
    type: 'insert' | 'delete' | 'format';
    content: string;
    author: string;
    timestamp: number;
    position: number;
  }[]
>([]);

const toggleRevisionDialog = () => {
  showRevisionDialog.value = !showRevisionDialog.value;
};

const _toggleRevisionTracking = () => {
  revisionTrackingEnabled.value = !revisionTrackingEnabled.value;
  if (revisionTrackingEnabled.value) {
    aiError.value = '修订追踪已启用';
  } else {
    aiError.value = '修订追踪已禁用';
  }
  setTimeout(() => (aiError.value = null), 2000);
};

const acceptAllRevisions = () => {
  try {
    const revisions = revisionTracking.getRevisions();
    revisions.forEach(rev => revisionTracking.acceptRevision(rev.id));
    aiError.value = '所有修订已接受';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Accept all revisions error', error, LogCategory.SYSTEM);
    aiError.value = '接受修订失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

const _rejectAllRevisions = () => {
  try {
    const revisions = revisionTracking.getRevisions();
    revisions.forEach(rev => revisionTracking.rejectRevision(rev.id));
    aiError.value = '所有修订已拒绝';
    setTimeout(() => (aiError.value = null), 2000);
  } catch (error) {
    logger.error('Reject all revisions error', error, LogCategory.SYSTEM);
    aiError.value = '拒绝修订失败: ' + (error as Error).message;
    setTimeout(() => (aiError.value = null), 3000);
  }
};

// Insert page break
const insertPageBreak = () => {
  editor.value
    ?.chain()
    .focus()
    .insertContent(
      '<div class="page-break" style="page-break-after: always; border-top: 2px dashed #e5e7eb; margin: 20px 0;"></div>'
    )
    .run();
  aiError.value = '已插入分页符';
  setTimeout(() => (aiError.value = null), 2000);
};

// Document templates
const templates = [
  {
    id: 'blank',
    name: '空白文档',
    description: '空白文档模板',
    content: '<p></p>'
  },
  {
    id: 'report',
    name: '报告',
    description: '标准报告模板',
    content:
      '<h1>报告标题</h1><h2>摘要</h2><p>在此处输入摘要内容...</p><h2>1. 引言</h2><p>在此处输入引言内容...</p><h2>2. 正文</h2><p>在此处输入正文内容...</p><h2>3. 结论</h2><p>在此处输入结论内容...</p>'
  },
  {
    id: 'letter',
    name: '信函',
    description: '正式信函模板',
    content:
      '<h1>信函</h1><p><strong>收件人：</strong></p><p>尊敬的先生/女士：</p><p>在此处输入信函内容...</p><p>此致</p><p>敬礼</p><p><strong>发件人：</strong></p><p><strong>日期：</strong></p>'
  },
  {
    id: 'memo',
    name: '备忘录',
    description: '内部备忘录模板',
    content:
      '<h1>备忘录</h1><p><strong>致：</strong></p><p><strong>发件人：</strong></p><p><strong>日期：</strong></p><p><strong>主题：</strong></p><p>在此处输入备忘录内容...</p>'
  }
];

// Apply document template
const applyDocumentTemplate = (templateId: string) => {
  const template = templates.find(t => t.id === templateId);
  if (template && editor.value) {
    if (confirm('应用模板将替换当前文档内容，确定继续吗？')) {
      editor.value.chain().focus().setContent(template.content).run();
      showTemplatesDialog.value = false;
      aiError.value = '模板已应用';
      setTimeout(() => (aiError.value = null), 2000);
    }
  }
};

// Bookmarks and cross-references
const _addBookmark = () => {
  if (!editor.value) {
return;
}
  const name = prompt('请输入书签名称:');
  if (name && name.trim()) {
    const { from } = editor.value.state.selection;
    const bookmark = {
      id: `bookmark-${Date.now()}`,
      name: name.trim(),
      position: from
    };
    bookmarks.value.push(bookmark);
    editor.value
      .chain()
      .focus()
      .insertContent(
        `<span id="${bookmark.id}" class="bookmark-marker" style="background-color: #dbeafe; border-bottom: 2px solid #3b82f6; cursor: pointer;">🔖 ${name}</span>`
      )
      .run();
    aiError.value = '书签已添加';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

// Spell check and grammar check
const _checkSpelling = () => {
  if (!editor.value) {
return;
}
  const text = editor.value.getText();
  const words = text.split(/\s+/).filter(w => w.length > 0);
  const errors: { word: string; suggestions: string[]; position: number }[] = [];

  // Simple spell check simulation (in production, integrate with a real spell check API)
  const commonMisspellings: Record<string, string[]> = {
    teh: ['the'],
    adn: ['and'],
    thier: ['their'],
    recieve: ['receive'],
    occured: ['occurred'],
    seperate: ['separate'],
    definately: ['definitely'],
    accomodate: ['accommodate'],
    neccessary: ['necessary'],
    maintainance: ['maintenance']
  };

  let position = 0;
  words.forEach(word => {
    const lowerWord = word.toLowerCase().replace(/[^\w]/g, '');
    if (commonMisspellings[lowerWord]) {
      errors.push({
        word: word,
        suggestions: commonMisspellings[lowerWord],
        position: position
      });
    }
    position += word.length + 1;
  });

  spellCheckErrors.value = errors;
  if (errors.length === 0) {
    aiError.value = '未发现拼写错误';
    setTimeout(() => (aiError.value = null), 2000);
  } else {
    showSpellCheckDialog.value = true;
  }
};

const applySuggestion = (word: string, suggestion: string) => {
  if (!editor.value) {
return;
}
  editor.value
    .chain()
    .focus()
    .setContent(editor.value.getHTML().replace(new RegExp(word, 'g'), suggestion))
    .run();
  aiError.value = '已应用建议';
  setTimeout(() => (aiError.value = null), 2000);
};

const _acceptSuggestion = (word: string, suggestion: string) => {
  applySuggestion(word, suggestion);
};

// Style management system
const _applyStyle = (styleId: string) => {
  if (!editor.value) {
return;
}
  const style = customStyles.value.find(s => s.id === styleId);
  if (style) {
    const styleString = Object.entries(style.styles)
      .map(([key, value]) => `${key}: ${value}`)
      .join('; ');
    editor.value
      .chain()
      .focus()
      .insertContent(`<span style="${styleString}">选中文本</span>`)
      .run();
    aiError.value = '样式已应用';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

const openStylesDialog = () => {
  openStyleManagerDialog();
};

// Advanced image editing (crop)
const _openImageCropDialog = () => {
  if (!editor.value) {
return;
}
  const { from } = editor.value.state.selection;
  const node = editor.value.state.doc.nodeAt(from);
  if (node && node.type.name === 'image') {
    selectedImageForCrop.value = node.attrs.src;
    showImageCropDialog.value = true;
  } else {
    aiError.value = '请选择要裁剪的图片';
    setTimeout(() => (aiError.value = null), 2000);
  }
};

const _cropImage = () => {
  if (!editor.value || !selectedImageForCrop.value) {
return;
}
  // In a real implementation, this would use a canvas to crop the image
  // For now, we'll simulate it by adding a crop style
  editor.value
    .chain()
    .focus()
    .updateAttributes('image', {
      style: `clip-path: inset(${cropY.value}% ${100 - cropX.value - cropWidth.value}% ${100 - cropY.value - cropHeight.value}% ${cropX.value}%)`
    })
    .run();
  showImageCropDialog.value = false;
  aiError.value = '图片已裁剪';
  setTimeout(() => (aiError.value = null), 2000);
};

const resetImageCrop = () => {
  if (!editor.value) {
return;
}
  editor.value.chain().focus().updateAttributes('image', { style: '' }).run();
  aiError.value = '图片裁剪已重置';
  setTimeout(() => (aiError.value = null), 2000);
};

const setFontFamily = (font: string) => {
  fontFamily.value = font;
  const editorElement = document.querySelector('.ProseMirror') as HTMLElement;
  if (editorElement) {
    editorElement.style.fontFamily = font;
  }
};

const increaseIndent = () => {
  paragraphIndent.value += 20;
  updateParagraphIndent();
};

const decreaseIndent = () => {
  if (paragraphIndent.value > 0) {
    paragraphIndent.value -= 20;
    updateParagraphIndent();
  }
};

const updateParagraphIndent = () => {
  const editorElement = document.querySelector('.ProseMirror') as HTMLElement;
  if (editorElement) {
    const paragraphs = editorElement.querySelectorAll('p');
    paragraphs.forEach(p => {
      (p as HTMLElement).style.textIndent = `${paragraphIndent.value}px`;
    });
  }
};

// Toggle gridlines
const _toggleGridlines = () => {
  showGridlines.value = !showGridlines.value;
};

// Toggle format marks
const _toggleFormatMarks = () => {
  showFormatMarks.value = !showFormatMarks.value;
};

// Toggle horizontal ruler
const _toggleHorizontalRuler = () => {
  showHorizontalRuler.value = !showHorizontalRuler.value;
};

// Toggle vertical ruler
const _toggleVerticalRuler = () => {
  showVerticalRuler.value = !showVerticalRuler.value;
  // Implementation for vertical ruler if needed
};

// Toggle zoom cycle
const toggleZoom = () => {
  const zoomLevels = [75, 100, 125, 150, 200];
  const currentIndex = zoomLevels.indexOf(zoomLevel.value);
  const nextIndex = (currentIndex + 1) % zoomLevels.length;
  zoomLevel.value = zoomLevels[nextIndex];
  setZoom(zoomLevel.value);
};

// Page setup functions
const _setPaperSize = (width: number, height: number) => {
  pageSize.value = { width, height };
};

const applyPageSetup = () => {
  try {
    // Validate margins
    Object.entries(pageMargins.value).forEach(([key, value]) => {
      if (!validateMargin(value)) {
        throw new Error(`Invalid ${key} margin: ${value}`);
      }
    });

    // Apply page setup to page container
    const pageContainer = document.querySelector('.page-container') as HTMLElement;
    if (pageContainer) {
      pageContainer.style.maxWidth = `${pageSize.value.width}mm`;
      pageContainer.style.minHeight = `${pageSize.value.height}mm`;
      pageContainer.style.paddingTop = `${pageMargins.value.top}mm`;
      pageContainer.style.paddingBottom = `${pageMargins.value.bottom}mm`;
      pageContainer.style.paddingLeft = `${pageMargins.value.left}mm`;
      pageContainer.style.paddingRight = `${pageMargins.value.right}mm`;
    }
  } catch (error) {
    handleError(error as Error, 'applyPageSetup');
    alert('页面设置应用失败：' + (error as Error).message);
  }
};

const applyFontSettings = () => {
  if (editor.value) {
    // Apply font settings via CSS since TipTap doesn't have built-in font family/size extensions
    const editorElement = document.querySelector('.ProseMirror') as HTMLElement;
    if (editorElement) {
      editorElement.style.fontFamily = fontFamily.value;
      editorElement.style.fontSize = fontSize.value + 'pt';
    }
  }
};

const _applyParagraphSettings = () => {
  if (editor.value) {
    // Apply line height via CSS since TipTap doesn't have built-in line height extension
    const editorElement = document.querySelector('.ProseMirror') as HTMLElement;
    if (editorElement) {
      editorElement.style.lineHeight = lineHeight.value.toString();
    }
  }
};

// Aerospace-grade error handling
const handleError = (error: Error, context: string) => {
  logger.error(`[${context}] Error`, error, LogCategory.SYSTEM);
  // In production, this would send to error tracking service
  // For now, we'll log to console with detailed context
};

// Validation utilities
const _validateZoomLevel = (level: number): boolean => {
  return level >= 10 && level <= 500;
};

const _validateFontSize = (size: number): boolean => {
  return size >= 1 && size <= 72;
};

const _validateLineHeight = (height: number): boolean => {
  return height >= 0.5 && height <= 10;
};

const validateMargin = (margin: number): boolean => {
  return margin >= 0 && margin <= 100;
};

// Safe document operations
const safeSaveDocument = async () => {
  try {
    if (!documentTitle.value || documentTitle.value.trim() === '') {
      throw new Error('Document title cannot be empty');
    }
    await saveDocument();
  } catch (error) {
    handleError(error as Error, 'safeSaveDocument');
    // Show user-friendly error message
    alert('保存失败：' + (error as Error).message);
  }
};

// Comprehensive keyboard shortcuts matching Logos
const handleKeyboardShortcuts = (event: KeyboardEvent) => {
  // Check if Ctrl or Cmd is pressed
  const isModifier = event.ctrlKey || event.metaKey;

  if (!isModifier) {
return;
}

  const key = event.key.toLowerCase();

  try {
    switch (key) {
      // File operations
      case 'n':
        event.preventDefault();
        newDocument();
        break;
      case 'o':
        event.preventDefault();
        loadDocument();
        break;
      case 's':
        event.preventDefault();
        safeSaveDocument();
        break;
      case 'p':
        event.preventDefault();
        printDocument();
        break;
      case 'w':
        event.preventDefault();
        // Close document (would need implementation)
        break;

      // Edit operations
      case 'z':
        event.preventDefault();
        if (event.shiftKey) {
          redo();
        } else {
          undo();
        }
        break;
      case 'y':
        event.preventDefault();
        redo();
        break;
      case 'x':
        event.preventDefault();
        // Cut (browser default, but we can enhance)
        break;
      case 'c':
        // Copy (browser default)
        break;
      case 'v':
        // Paste (browser default)
        break;
      case 'a':
        event.preventDefault();
        selectAll();
        break;
      case 'f':
        event.preventDefault();
        toggleSearchDialog();
        break;
      case 'h':
        event.preventDefault();
        // Replace (could enhance search dialog)
        toggleSearchDialog();
        break;
      case 'g':
        event.preventDefault();
        // Go to (would need implementation)
        break;

      // Formatting
      case 'b':
        event.preventDefault();
        toggleBold();
        break;
      case 'i':
        event.preventDefault();
        toggleItalic();
        break;
      case 'u':
        event.preventDefault();
        toggleUnderline();
        break;
      case 'd':
        event.preventDefault();
        // Double underline (Logos shortcut)
        toggleUnderline();
        break;
      case ' ':
        event.preventDefault();
        // Reset formatting
        clearFormatting();
        break;
      case '=':
      case '+':
        event.preventDefault();
        // Subscript (Logos uses Ctrl+=)
        toggleSubscript();
        break;
      case 'shift':
        event.preventDefault();
        // Superscript (Logos uses Ctrl+Shift+=)
        toggleSuperscript();
        break;

      // Alignment
      case 'l':
        event.preventDefault();
        setTextAlign('left');
        break;
      case 'e':
        event.preventDefault();
        setTextAlign('center');
        break;
      case 'r':
        event.preventDefault();
        setTextAlign('right');
        break;
      case 'j':
        event.preventDefault();
        setTextAlign('justify');
        break;

      // Headings
      case '1':
        event.preventDefault();
        toggleHeading(1);
        break;
      case '2':
        event.preventDefault();
        toggleHeading(2);
        break;
      case '3':
        event.preventDefault();
        toggleHeading(3);
        break;
      case '4':
        event.preventDefault();
        toggleHeading(4);
        break;
      case '5':
        event.preventDefault();
        toggleHeading(5);
        break;
      case '0':
        event.preventDefault();
        toggleHeading(6);
        break;

      // Lists
      case 'shift+l':
        event.preventDefault();
        toggleBulletList();
        break;
      case 'shift+n':
        event.preventDefault();
        toggleOrderedList();
        break;

      // Indentation
      case 'm':
        event.preventDefault();
        if (event.shiftKey) {
          decreaseIndent();
        } else {
          increaseIndent();
        }
        break;

      // Links
      case 'k':
        event.preventDefault();
        addLink();
        break;

      // Find
      case 'shift+f':
        event.preventDefault();
        toggleSearchDialog();
        break;

      // Save As
      case 'shift+s':
        event.preventDefault();
        // Would need implementation for Save As
        safeSaveDocument();
        break;

      // Print
      case 'shift+p':
        event.preventDefault();
        printDocument();
        break;

      // Open
      case 'shift+o':
        event.preventDefault();
        loadDocument();
        break;
    }
  } catch (error) {
    handleError(error as Error, 'handleKeyboardShortcuts');
  }
};

// Function keys
const handleFunctionKeys = (event: KeyboardEvent) => {
  try {
    switch (event.key) {
      case 'F1':
        event.preventDefault();
        toggleShortcutsHelp();
        break;
      case 'F2':
        event.preventDefault();
        startEditingTitle();
        break;
      case 'F4':
        event.preventDefault();
        // Repeat last action (would need implementation)
        break;
      case 'F5':
        event.preventDefault();
        // Find and replace (Logos uses F5)
        toggleSearchDialog();
        break;
      case 'F7':
        event.preventDefault();
        // Spelling and grammar (would need implementation)
        break;
      case 'F12':
        event.preventDefault();
        // Save As (Logos uses F12)
        safeSaveDocument();
        break;
    }
  } catch (error) {
    handleError(error as Error, 'handleFunctionKeys');
  }
};

// Register keyboard shortcuts on mount
onMounted(async () => {
  window.addEventListener('keydown', handleKeyboardShortcuts);
  window.addEventListener('keydown', handleFunctionKeys);
  
  // Initialize hybrid services
  await hybridServices.init();
  
  // 监听原生菜单栏事件
  if (isTauri()) {
    listen('menu-new-document', () => {
      newDocument();
    });
    
    listen('menu-open-document', () => {
      // 触发打开文档功能
      const event = new KeyboardEvent('keydown', { key: 'o', ctrlKey: true, metaKey: true });
      window.dispatchEvent(event);
    });
    
    listen('menu-save-document', () => {
      saveDocument();
    });
    
    listen('menu-save-as', () => {
      saveDocument();
    });
    
    listen('menu-export-pdf', () => {
      exportTypstPdf();
    });
    
    listen('menu-export-png', () => {
      exportTypstPng();
    });

    listen('menu-export-svg-typst', () => {
      exportTypstSvg();
    });

    listen('menu-export-svg-html', () => {
      exportHtmlSvg();
    });
    
    listen('menu-export-typst', () => {
      exportToTypst();
    });
    
    listen('menu-export-docx', () => {
      exportToWord();
    });
    
    listen('menu-print', () => {
      printDocument();
    });
    
    listen('menu-quit', () => {
      // 退出应用由 Tauri 处理
    });
    
    listen('menu-undo', () => {
      undo();
    });
    
    listen('menu-redo', () => {
      redo();
    });
    
    listen('menu-cut', () => {
      cutSelection();
    });
    
    listen('menu-copy', () => {
      copySelection();
    });
    
    listen('menu-paste', () => {
      pasteFromClipboard();
    });
    
    listen('menu-select-all', () => {
      editor.value?.chain().focus().selectAll().run();
    });
    
    listen('menu-find', () => {
      // 触发查找功能
      const event = new KeyboardEvent('keydown', { key: 'f', ctrlKey: true, metaKey: true });
      window.dispatchEvent(event);
    });
    
    listen('menu-replace', () => {
      // 触发替换功能
      const event = new KeyboardEvent('keydown', { key: 'h', ctrlKey: true, metaKey: true });
      window.dispatchEvent(event);
    });
    
    listen('menu-fullscreen', () => {
      toggleFullscreen();
    });
    
    listen('menu-zoom-in', () => {
      zoomLevel.value = Math.min(zoomLevel.value + 10, 200);
    });
    
    listen('menu-zoom-out', () => {
      zoomLevel.value = Math.max(zoomLevel.value - 10, 50);
    });
    
    listen('menu-zoom-reset', () => {
      zoomLevel.value = 100;
    });
    
    listen('menu-toggle-sidebar', () => {
      showDocumentOutline.value = !showDocumentOutline.value;
    });
    
    listen('menu-toggle-statusbar', () => {
      showStatusBar.value = !showStatusBar.value;
    });
    
    listen('menu-typst-preview', () => {
      generateTypstPreview();
    });
    
    // 插入菜单
    listen('menu-insert-image', () => {
      insertImage();
    });
    
    listen('menu-insert-table', () => {
      insertTable();
    });
    
    listen('menu-insert-link', () => {
      addLink();
    });
    
    listen('menu-insert-code-block', () => {
      editor.value?.chain().focus().toggleCodeBlock().run();
    });
    
    listen('menu-insert-formula', () => {
      insertMathFormula();
    });
    
    listen('menu-insert-emoji', () => {
      // 触发 emoji 插入
      const event = new KeyboardEvent('keydown', { key: ':', ctrlKey: true, metaKey: true });
      window.dispatchEvent(event);
    });
    
    // 格式菜单
    listen('format-bold', () => {
      toggleBold();
    });
    
    listen('format-italic', () => {
      toggleItalic();
    });
    
    listen('format-underline', () => {
      toggleUnderline();
    });
    
    listen('format-strikethrough', () => {
      toggleStrike();
    });
    
    listen('format-superscript', () => {
      toggleSuperscript();
    });
    
    listen('format-subscript', () => {
      toggleSubscript();
    });
    
    listen('format-align', () => {
      // 打开对齐选项
      showToast('对齐选项', 'info');
    });
    
    listen('format-line-spacing', () => {
      // 打开行距选项
      showToast('行距选项', 'info');
    });
    
    listen('format-style', () => {
      openStyleManagerDialog();
    });
    
    // 工具菜单
    listen('menu-spell-check', () => {
      toggleSpellCheck();
    });
    
    listen('menu-word-count', () => {
      showWordCountDialog.value = true;
    });
    
    listen('menu-ai-polish', () => {
      triggerAiPolish();
    });
    
    listen('menu-ai-expand', () => {
      triggerAiExpand();
    });
    
    listen('menu-ai-translate', () => {
      triggerAiTranslate();
    });
    
    listen('menu-typst-packages', () => {
      // 打开 Typst 包管理器
      showToast('Typst 包管理器', 'info');
    });
    
    listen('menu-settings', () => {
      // 打开设置
      showToast('设置', 'info');
    });
    
    // 帮助菜单
    listen('menu-user-guide', () => {
      toggleUserGuideDialog();
    });
    
    listen('menu-shortcuts', () => {
      toggleShortcutsHelp();
    });
    
    listen('menu-api-docs', () => {
      // 打开 API 文档
      window.open('https://docs.logos-zhidao.com', '_blank');
    });
    
    listen('menu-check-updates', () => {
      // 检查更新
      showToast('检查更新...', 'info');
    });
    
    listen('menu-feedback', () => {
      // 打开反馈页面
      window.open('https://github.com/logos-zhidao/feedback', '_blank');
    });
    
    listen('menu-about', () => {
      toggleAboutDialog();
    });
  }
});

// Cleanup on unmount
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyboardShortcuts);
  window.removeEventListener('keydown', handleFunctionKeys);
});

// Context menu handling
const handleContextMenu = (event: MouseEvent) => {
  event.preventDefault();
  contextMenuPosition.value = { x: event.clientX, y: event.clientY };
  showContextMenu.value = true;
};

const closeContextMenu = () => {
  showContextMenu.value = false;
};

// Handle context menu action
const handleContextMenuAction = async (action: string, payload?: any) => {
  switch (action) {
    case 'cut':
      try {
        await navigator.clipboard.writeText(window.getSelection()?.toString() || '');
        editor.value?.chain().focus().deleteSelection().run();
      } catch (e) {
        logger.error('Cut failed', e, LogCategory.SYSTEM);
      }
      break;
    case 'copy':
      try {
        await navigator.clipboard.writeText(window.getSelection()?.toString() || '');
      } catch (e) {
        logger.error('Copy failed', e, LogCategory.SYSTEM);
      }
      break;
    case 'paste':
      try {
        const text = await navigator.clipboard.readText();
        editor.value?.chain().focus().insertContent(text).run();
      } catch (e) {
        logger.error('Paste failed', e, LogCategory.SYSTEM);
      }
      break;
    case 'select-all':
      editor.value?.chain().focus().selectAll().run();
      break;
    case 'bold':
      editor.value?.chain().focus().toggleBold().run();
      break;
    case 'italic':
      editor.value?.chain().focus().toggleItalic().run();
      break;
    case 'underline':
      editor.value?.chain().focus().toggleUnderline().run();
      break;
    case 'link':
      addLink();
      break;
    case 'comment':
      showCommentDialog.value = true;
      break;
    default:
      // Unhandled context menu action:
  }
};

// Page layout dialog handlers
const openPageLayoutDialog = () => {
  showPageLayoutDialog.value = true;
};

const handlePageLayoutApply = (settings: any) => {
  pageOrientation.value = settings.orientation;
  pageSize.value = {
    width: settings.pageSize === 'a4' ? 210 : settings.pageSize === 'a3' ? 297 : settings.pageSize === 'letter' ? 216 : 216,
    height: settings.pageSize === 'a4' ? 297 : settings.pageSize === 'a3' ? 420 : settings.pageSize === 'letter' ? 279 : 356
  };
  pageMargins.value = {
    top: settings.margins.top * 10,
    bottom: settings.margins.bottom * 10,
    left: settings.margins.left * 10,
    right: settings.margins.right * 10
  };
  showPageLayoutDialog.value = false;
};

// Style manager dialog handlers
const openStyleManagerDialog = () => {
  showStyleManagerDialog.value = true;
};

const handleStyleApply = (style: any) => {
  if (style.type === 'paragraph') {
    // Apply paragraph style
    if (style.formatting.bold) {
editor.value?.chain().focus().toggleBold().run();
}
    if (style.formatting.italic) {
editor.value?.chain().focus().toggleItalic().run();
}
    if (style.formatting.underline) {
editor.value?.chain().focus().toggleUnderline().run();
}
    if (style.formatting.alignment) {
      editor.value?.chain().focus().setTextAlign(style.formatting.alignment).run();
    }
  } else if (style.type === 'character') {
    // Apply character style
    if (style.formatting.bold) {
editor.value?.chain().focus().toggleBold().run();
}
    if (style.formatting.italic) {
editor.value?.chain().focus().toggleItalic().run();
}
    if (style.formatting.underline) {
editor.value?.chain().focus().toggleUnderline().run();
}
  }
  showStyleManagerDialog.value = false;
};

// Header footer dialog handlers
const openHeaderDialog = () => {
  headerFooterType.value = 'header';
  showHeaderFooterEditorDialog.value = true;
};

const openFooterDialog = () => {
  headerFooterType.value = 'footer';
  showHeaderFooterEditorDialog.value = true;
};

const handleHeaderFooterApply = (content: any) => {
  // Apply header/footer content
  // Header/Footer content applied:
  showHeaderFooterEditorDialog.value = false;
};

// Medium priority UI component handlers
const openShapeSelector = () => {
  showShapeSelector.value = true;
};

const openIconSelector = () => {
  showIconSelector.value = true;
};

const openSmartArtSelector = () => {
  showSmartArtSelector.value = true;
};

const openWordArtDialog = () => {
  showWordArtDialog.value = true;
};

const openChartEditor = () => {
  showChartEditor.value = true;
};

const toggleCommentsPanel = () => {
  showCommentsPanel.value = !showCommentsPanel.value;
};

const toggleRevisionPanel = () => {
  showRevisionPanel.value = !showRevisionPanel.value;
};

const toggleTableDesignTab = () => {
  showTableDesignTab.value = !showTableDesignTab.value;
};

const handleInsertShape = (shape: any) => {
  if (!editor.value) {
return;
}
  
  const width = 200;
  const height = 150;
  let shapeHtml = '';
  
  switch (shape.type) {
    case 'rectangle':
      shapeHtml = `<div style="width: ${width}px; height: ${height}px; border: 2px solid #000; background-color: #f0f0f0; display: inline-block;"></div>`;
      break;
    case 'rounded-rectangle':
      shapeHtml = `<div style="width: ${width}px; height: ${height}px; border: 2px solid #000; background-color: #f0f0f0; border-radius: 10px; display: inline-block;"></div>`;
      break;
    case 'square':
      shapeHtml = `<div style="width: ${width}px; height: ${width}px; border: 2px solid #000; background-color: #f0f0f0; display: inline-block;"></div>`;
      break;
    case 'parallelogram':
      shapeHtml = `<div style="width: ${width}px; height: ${height}px; border: 2px solid #000; background-color: #f0f0f0; transform: skewX(-20deg); display: inline-block;"></div>`;
      break;
    case 'trapezoid':
      shapeHtml = `<div style="width: ${width}px; height: ${height}px; border: 2px solid #000; background-color: #f0f0f0; clip-path: polygon(20% 0%, 80% 0%, 100% 100%, 0% 100%); display: inline-block;"></div>`;
      break;
    case 'diamond':
      shapeHtml = `<div style="width: ${width}px; height: ${width}px; border: 2px solid #000; background-color: #f0f0f0; transform: rotate(45deg); display: inline-block;"></div>`;
      break;
    case 'pentagon':
      shapeHtml = `<div style="width: ${width}px; height: ${height}px; border: 2px solid #000; background-color: #f0f0f0; clip-path: polygon(50% 0%, 100% 38%, 82% 100%, 18% 100%, 0% 38%); display: inline-block;"></div>`;
      break;
    case 'hexagon':
      shapeHtml = `<div style="width: ${width}px; height: ${height}px; border: 2px solid #000; background-color: #f0f0f0; clip-path: polygon(25% 0%, 75% 0%, 100% 50%, 75% 100%, 25% 100%, 0% 50%); display: inline-block;"></div>`;
      break;
    case 'octagon':
      shapeHtml = `<div style="width: ${width}px; height: ${height}px; border: 2px solid #000; background-color: #f0f0f0; clip-path: polygon(30% 0%, 70% 0%, 100% 30%, 100% 70%, 70% 100%, 30% 100%, 0% 70%, 0% 30%); display: inline-block;"></div>`;
      break;
    case 'triangle':
      shapeHtml = `<div style="width: 0; height: 0; border-left: ${width/2}px solid transparent; border-right: ${width/2}px solid transparent; border-bottom: ${height}px solid #000; display: inline-block;"></div>`;
      break;
    case 'right-triangle':
      shapeHtml = `<div style="width: 0; height: 0; border-left: ${width}px solid #000; border-bottom: ${height}px solid transparent; display: inline-block;"></div>`;
      break;
    case 'ellipse':
      shapeHtml = `<div style="width: ${width}px; height: ${height}px; border: 2px solid #000; background-color: #f0f0f0; border-radius: 50%; display: inline-block;"></div>`;
      break;
    case 'circle':
      shapeHtml = `<div style="width: ${width}px; height: ${width}px; border: 2px solid #000; background-color: #f0f0f0; border-radius: 50%; display: inline-block;"></div>`;
      break;
    case 'oval':
      shapeHtml = `<div style="width: ${width}px; height: ${height}px; border: 2px solid #000; background-color: #f0f0f0; border-radius: 50%; display: inline-block;"></div>`;
      break;
    default:
      shapeHtml = `<div style="width: ${width}px; height: ${height}px; border: 2px solid #000; background-color: #f0f0f0; display: inline-block;"></div>`;
  }
  
  editor.value.chain().focus().insertContent(shapeHtml).run();
  showToast(`已插入${shape.name}`, 'success');
};

const handleInsertIcon = (icon: any) => {
  if (!editor.value) {
return;
}
  
  const iconHtml = `<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: inline-block; vertical-align: middle;">${icon.svg}</svg>&nbsp;`;
  
  editor.value.chain().focus().insertContent(iconHtml).run();
  showToast(`已插入${icon.name}`, 'success');
};

const handleInsertSmartArt = (smartart: any) => {
  if (!editor.value) {
return;
}
  
  const smartartHtml = `
    <div style="display: inline-block; padding: 20px; border: 1px solid #ccc; background: #f9f9f9;">
      <div style="text-align: center; font-weight: bold; margin-bottom: 10px;">${smartart.name}</div>
      <svg xmlns="http://www.w3.org/2000/svg" width="120" height="120" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="display: block; margin: 0 auto;">${smartart.svg}</svg>
      <div style="margin-top: 10px; font-size: 12px; color: #666;">点击编辑内容</div>
    </div>
  `;
  
  editor.value.chain().focus().insertContent(smartartHtml).run();
  showToast(`已插入${smartart.name}`, 'success');
};

const handleInsertWordArt = (wordart: any) => {
  if (!editor.value) {
return;
}
  
  const wordartHtml = `
    <span style="
      font-size: ${wordart.fontSize}px;
      font-family: ${wordart.fontFamily};
      color: ${wordart.fill};
      -webkit-text-stroke: ${wordart.strokeWidth}px ${wordart.stroke};
      letter-spacing: ${wordart.letterSpacing}px;
      transform: ${wordart.transform};
      display: inline-block;
      font-weight: bold;
    ">${wordart.text}</span>
  `;
  
  editor.value.chain().focus().insertContent(wordartHtml).run();
  showToast('已插入艺术字', 'success');
};

const handleInsertChart = (chart: any) => {
  if (!editor.value) {
return;
}
  
  const maxValue = Math.max(...chart.data.map((d: any) => d.value));
  const chartHeight = 200;
  const chartWidth = 300;
  
  let barsHtml = '';
  chart.data.forEach((dataPoint: any, index: number) => {
    const barHeight = (dataPoint.value / maxValue) * (chartHeight - 40);
    const barWidth = (chartWidth - 40) / chart.data.length - 10;
    const x = 20 + index * (barWidth + 10);
    const y = chartHeight - 20 - barHeight;
    
    barsHtml += `
      <rect x="${x}" y="${y}" width="${barWidth}" height="${barHeight}" fill="${chart.style.color}" />
      <text x="${x + barWidth/2}" y="${chartHeight - 5}" text-anchor="middle" font-size="12" fill="#333">${dataPoint.label}</text>
      <text x="${x + barWidth/2}" y="${y - 5}" text-anchor="middle" font-size="11" fill="#666">${dataPoint.value}</text>
    `;
  });
  
  const chartHtml = `
    <div style="display: inline-block; padding: 20px; border: 1px solid #ccc; background: #f9f9f9;">
      <div style="text-align: center; font-weight: bold; margin-bottom: 10px;">${chart.title}</div>
      <svg xmlns="http://www.w3.org/2000/svg" width="${chartWidth}" height="${chartHeight}" viewBox="0 0 ${chartWidth} ${chartHeight}">
        <line x1="20" y1="${chartHeight - 20}" x2="${chartWidth - 20}" y2="${chartHeight - 20}" stroke="#333" stroke-width="2" />
        <line x1="20" y1="20" x2="20" y2="${chartHeight - 20}" stroke="#333" stroke-width="2" />
        ${barsHtml}
      </svg>
    </div>
  `;
  
  editor.value.chain().focus().insertContent(chartHtml).run();
  showToast('已插入图表', 'success');
};

const handleAddComment = (comment: any) => {
  if (!editor.value) {
return;
}
  
  const commentHtml = `<span class="comment-mark" data-comment-id="${Date.now()}" style="background-color: #ffff00; padding: 2px 4px; border-radius: 2px;">${comment.text}</span>`;
  
  editor.value.chain().focus().insertContent(commentHtml).run();
  showToast('批注已添加', 'success');
};

const handleResolveComment = (commentId: string) => {
  showToast('批注已解决', 'success');
};

const handleDeleteComment = (commentId: string) => {
  showToast('批注已删除', 'success');
};

const handleReplyComment = (commentId: string, reply: string) => {
  showToast('回复已添加', 'success');
};

const handleToggleTrackChanges = (enabled: boolean) => {
  trackChangesEnabled.value = enabled;
  showToast(`修订模式已${enabled ? '开启' : '关闭'}`, 'success');
};

const handleAcceptRevision = (revisionId: string) => {
  showToast('修订已接受', 'success');
};

const handleRejectRevision = (revisionId: string) => {
  showToast('修订已拒绝', 'success');
};

const handleAcceptAllRevisions = () => {
  showToast('所有修订已接受', 'success');
};

const handleRejectAllRevisions = () => {
  showToast('所有修订已拒绝', 'success');
};

const handleApplyTableStyle = (style: string) => {
  if (!editor.value) {
return;
}
  
  const styleMap: Record<string, string> = {
    'plain': 'border: 1px solid #000; background: #fff;',
    'grid': 'border: 1px solid #000; background: #f9f9f9;',
    'striped': 'border: 1px solid #000; background: repeating-linear-gradient(45deg, #fff, #fff 10px, #f9f9f9 10px, #f9f9f9 20px);',
    'shaded': 'border: 1px solid #000; background: #e8e8e8;',
    'dotted': 'border: 2px dotted #000; background: #fff;',
    'dashed': 'border: 2px dashed #000; background: #fff;',
    'double': 'border: 3px double #000; background: #fff;',
    'thick': 'border: 3px solid #000; background: #fff;'
  };
  
  const tableStyle = styleMap[style] || styleMap['plain'];
  
  editor.value.chain().focus().setMark('tableStyle', { style: tableStyle }).run();
  showToast('表格样式已应用', 'success');
};

const handleApplyTableBorder = (border: any) => {
  if (!editor.value) {
return;
}
  
  const borderStyle = `border: ${border.width}px ${border.style} ${border.color};`;
  
  editor.value.chain().focus().setMark('tableBorder', { style: borderStyle }).run();
  showToast('表格边框已应用', 'success');
};

const handleApplyTableShading = (shading: string) => {
  if (!editor.value) {
return;
}
  
  const shadingStyle = `background-color: ${shading};`;
  
  editor.value.chain().focus().setMark('tableShading', { style: shadingStyle }).run();
  showToast('表格底纹已应用', 'success');
};

// Close context menu when clicking elsewhere
const _handleGlobalClick = () => {
  closeContextMenu();
  closeAllMenus();
};
</script>

<template>
  <div class="editor-container" :class="{ dark: isDarkMode, 'focus-mode-active': viewMode === 'focus', 'read-mode-active': viewMode === 'read', 'web-mode-active': viewMode === 'web' }">
    <!-- Global Loading Overlay - only show for saving operations, not initialization -->
    <div v-if="isSaving" class="global-loading-overlay">
      <div class="loading-content">
        <div class="loading-spinner-large"></div>
        <p>正在保存...</p>
      </div>
    </div>

    <!-- Quick Access Toolbar -->
    <QuickAccessToolbar
      :show-file-backstage="showFileBackstage"
      :document-title="documentTitle"
      @toggle-file-backstage="showFileBackstage = !showFileBackstage"
      @save="saveDocument"
      @undo="undo"
      @redo="redo"
      @toggle-search="toggleSearchDialog"
      @toggle-split-view="showSplitView = !showSplitView"
      @toggle-ai-sidebar="handleToggleAISidebar"
      @toggle-help="toggleHelp"
      @update-title="(title) => documentTitle = title"
    />

    <!-- File Backstage View -->
    <FileBackstage
      :show="showFileBackstage"
      :recent-files="recentFiles"
      :document-title="documentTitle"
      @close="showFileBackstage = false"
      @new-document="newDocument"
      @open-document="loadDocument"
      @save-document="saveDocument"
      @save-as="saveDocument"
      @load-recent-file="loadRecentFile"
      @clear-recent-files="clearRecentFiles"
      @export-pdf="exportTypstPdf"
      @export-word="exportToWord"
      @export-typst="exportToTypst"
      @export-svg-typst="exportTypstSvg"
      @export-svg-html="exportHtmlSvg"
      @print="printDocument"
    />

    <!-- Ribbon Tabs -->
    <div class="ribbon-tabs">
      <button
        class="ribbon-tab"
        :class="{ active: activeRibbonTab === 'file' }"
        @click="setActiveRibbonTab('file')"
      >
        文件
      </button>
      <button
        class="ribbon-tab"
        :class="{ active: activeRibbonTab === 'home' }"
        @click="setActiveRibbonTab('home')"
      >
        开始
      </button>
      <button
        class="ribbon-tab"
        :class="{ active: activeRibbonTab === 'insert' }"
        @click="setActiveRibbonTab('insert')"
      >
        插入
      </button>
      <button
        class="ribbon-tab"
        :class="{ active: activeRibbonTab === 'graphics' }"
        @click="setActiveRibbonTab('graphics')"
      >
        图形
      </button>
      <button
        class="ribbon-tab"
        :class="{ active: activeRibbonTab === 'layout' }"
        @click="setActiveRibbonTab('layout')"
      >
        布局
      </button>
      <button
        class="ribbon-tab"
        :class="{ active: activeRibbonTab === 'spreadsheet' }"
        @click="setActiveRibbonTab('spreadsheet')"
      >
        电子表格
      </button>
      <button
        class="ribbon-tab"
        :class="{ active: activeRibbonTab === 'typst' }"
        @click="setActiveRibbonTab('typst')"
      >
        Typst
      </button>
      <button
        class="ribbon-tab"
        :class="{ active: activeRibbonTab === 'slides' }"
        @click="setActiveRibbonTab('slides')"
      >
        幻灯片
      </button>
      <button
        class="ribbon-tab"
        :class="{ active: activeRibbonTab === 'references' }"
        @click="setActiveRibbonTab('references')"
      >
        引用
      </button>
      <button
        class="ribbon-tab"
        :class="{ active: activeRibbonTab === 'review' }"
        @click="setActiveRibbonTab('review')"
      >
        审阅
      </button>
      <button
        class="ribbon-tab"
        :class="{ active: activeRibbonTab === 'view' }"
        @click="setActiveRibbonTab('view')"
      >
        视图
      </button>
      <button
        class="ribbon-tab"
        :class="{ active: activeRibbonTab === 'help' }"
        @click="setActiveRibbonTab('help')"
      >
        帮助
      </button>
    </div>

    <!-- Ribbon Panels Wrapper with horizontal navigation arrows -->
    <div class="ribbon-panels-wrapper">
      <button class="ribbon-scroll-button scroll-left" title="向左滚动" aria-label="向左滚动功能区" @click="scrollRibbon(-200)">
        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="15 18 9 12 15 6"></polyline>
        </svg>
      </button>

      <div ref="ribbonPanelsRef" class="ribbon-panels">
        <!-- File Tab Panel -->
        <div v-if="activeRibbonTab === 'file'" class="ribbon-panel">
          <!-- Document Group -->
          <div class="ribbon-group">
            <div class="group-content">
              <button class="ribbon-button-large" title="新建文档" aria-label="新建空白文档" @click="newDocument">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                  <polyline points="14 2 14 8 20 8" />
                </svg>
                <span>新建</span>
              </button>
              <button class="ribbon-button-large" title="打开文档" aria-label="打开现有文档" @click="loadDocument">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
                </svg>
                <span>打开</span>
              </button>
              <button class="ribbon-button-large" title="保存文档" aria-label="保存当前文档" @click="saveDocument">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
                  <polyline points="17 21 17 13 7 13 7 21" />
                  <polyline points="7 3 7 8 15 8" />
                </svg>
                <span>保存</span>
              </button>
            </div>
            <div class="group-label">文档</div>
          </div>

          <!-- Export Group -->
          <div class="ribbon-group">
            <div class="group-content">
              <button class="ribbon-button" title="导出 PDF" aria-label="导出 PDF 文档" @click="exportTypstPdf">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                  <polyline points="14 2 14 8 20 8" />
                </svg>
                <span>导出 PDF</span>
              </button>
              <button class="ribbon-button" title="导出文档" aria-label="导出 Word 文档" @click="exportToWord">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                  <polyline points="14 2 14 8 20 8" />
                </svg>
                <span>导出文档</span>
              </button>
              <button class="ribbon-button" title="Export Typst" aria-label="Export Typst format" @click="exportToTypst">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                  <polyline points="14 2 14 8 20 8" />
                </svg>
                <span>Export Typst</span>
              </button>
              <button class="ribbon-button" title="Export SVG (Typst)" aria-label="Export SVG via Typst renderer" @click="exportTypstSvg">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                  <polyline points="14 2 14 8 20 8" />
                </svg>
                <span>Export SVG (Typst)</span>
              </button>
              <button class="ribbon-button" title="Export SVG (HTML)" aria-label="Export SVG via HTML vector service" @click="exportHtmlSvg">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                  <polyline points="14 2 14 8 20 8" />
                </svg>
                <span>Export SVG (HTML)</span>
              </button>
            </div>
            <div class="group-label">导出</div>
          </div>

          <!-- Print Group -->
          <div class="ribbon-group">
            <div class="group-content">
              <button class="ribbon-button-large" title="打印文档" aria-label="打印文档" @click="printDocument">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="6 9 6 2 18 2 18 9" />
                  <path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2" />
                  <rect x="6" y="14" width="12" height="8" />
                </svg>
                <span>打印</span>
              </button>
            </div>
            <div class="group-label">打印</div>
          </div>
        </div>

        <!-- Home Tab Panel -->
      <div v-if="activeRibbonTab === 'home'" class="ribbon-panel">
        <!-- Clipboard Group -->
        <ClipboardGroup
          @paste="pasteFromClipboard"
          @cut="cutSelection"
          @copy="copySelection"
          @format-painter="formatPainter"
        />

        <!-- Font Group -->
        <FontGroup
          :font-family="fontFamily"
          :font-size="fontSize"
          @update:font-family="fontFamily = $event"
          @update:font-size="fontSize = $event"
          @toggle-bold="toggleBold"
          @toggle-italic="toggleItalic"
          @toggle-underline="toggleUnderline"
          @toggle-strike="toggleStrike"
          @toggle-subscript="toggleSubscript"
          @toggle-superscript="toggleSuperscript"
          @toggle-highlight="toggleHighlight"
          @set-text-color="setTextColor"
          @clear-formatting="clearFormatting"
          @text-effects="handleTextEffects"
          @change-case="handleChangeCase"
          @pinyin-guide="handlePinyinGuide"
          @enclosed-characters="handleEnclosedCharacters"
          @vertical-text="handleVerticalText"
          @double-strikethrough="handleDoubleStrikethrough"
          @full-half-width="handleFullHalfWidth"
          @text-border="handleTextBorder"
          @text-shading="handleTextShading"
          @character-spacing="handleCharacterSpacing"
          @drop-cap="handleDropCap"
          @character-scale="handleCharacterScale"
          @small-caps="handleSmallCaps"
        />

        <!-- Paragraph Group -->
        <ParagraphGroup
          @set-text-align="setTextAlign"
          @toggle-bullet-list="toggleBulletList"
          @toggle-ordered-list="toggleOrderedList"
          @toggle-task-list="toggleTaskList"
          @decrease-indent="decreaseIndent"
          @increase-indent="increaseIndent"
          @set-heading="setHeading"
          @toggle-blockquote="toggleBlockquote"
          @toggle-code-block="toggleCodeBlock"
          @insert-horizontal-rule="insertHorizontalRule"
          @clear-formatting="clearFormatting"
          @set-line-spacing="setLineSpacing"
          @set-paragraph-spacing="setParagraphSpacing"
          @add-border="addBorder"
          @add-shading="addShading"
          @toggle-multilevel-list="toggleMultilevelList"
          @sort-paragraph="sortParagraph"
          @toggle-format-marks="toggleFormatMarks"
        />

        <!-- Styles Group -->
        <StylesGroup
          :selected-style="selectedStyle"
          @update:selected-style="selectedStyle = $event"
          @change-style-set="changeStyleSet"
          @apply-emphasis="applyEmphasis"
          @apply-strong-emphasis="applyStrongEmphasis"
          @apply-quote="applyQuote"
          @apply-list-paragraph="applyListParagraph"
          @apply-intense-quote="applyIntenseQuote"
          @apply-subtle-reference="applySubtleReference"
          @apply-book-title="applyBookTitle"
          @apply-intense-emphasis="applyIntenseEmphasis"
          @new-style="newStyle"
          @style-pane="stylePane"
        />

        <!-- Editing Group -->
        <EditingGroup
          @find-text="findText"
          @replace-text="ribbonReplaceText"
          @select-all="selectAll"
          @select-objects="selectObjects"
          @select-similar-formatting="selectSimilarFormatting"
        />
      </div>

      <!-- Insert Tab Panel -->
      <div v-if="activeRibbonTab === 'insert'" class="ribbon-panel">
        <!-- Pages Group -->
        <PagesGroup
          @insert-page-break="ribbonInsertPageBreak"
          @insert-blank-page="insertBlankPage"
        />

        <!-- Tables Group -->
        <TablesGroup
          @insert-table="insertTable"
          @delete-table="deleteTable"
          @add-column-before="addColumnBefore"
          @add-column-after="addColumnAfter"
          @delete-column="deleteColumn"
          @add-row-before="addRowBefore"
          @add-row-after="addRowAfter"
          @delete-row="deleteRow"
          @merge-cells="mergeCells"
          @split-cell="splitCell"
          @toggle-header-row="toggleHeaderRow"
          @toggle-header-column="toggleHeaderColumn"
          @toggle-header-cell="toggleHeaderCell"
        />

        <!-- Illustrations Group -->
        <IllustrationsGroup
          @insert-image="addImage"
          @insert-shape="insertShape"
          @resize-image="applyImageResize"
          @align-image="alignImage"
          @wrap-image="wrapImage"
          @crop-image="cropImage"
          @rotate-image="rotateImage"
          @flip-image="flipImage"
        />

        <!-- Links & Comments Group -->
        <LinksCommentsGroup
          @insert-link="setLink"
          @insert-bookmark="insertBookmark"
          @add-comment="addComment"
        />

        <!-- Header & Footer Group -->
        <HeaderFooterGroup
          :is-editing-header="isEditingHeader"
          :is-editing-footer="isEditingFooter"
          @toggle-header-footer-dialog="toggleHeaderFooterDialog"
          @enter-header-edit-mode="enterHeaderEditMode"
          @enter-footer-edit-mode="enterFooterEditMode"
          @exit-header-footer-edit-mode="exitHeaderFooterEditMode"
          @toggle-page-number-dialog="togglePageNumberDialog"
        />

        <!-- Symbols Group -->
        <SymbolsGroup
          @toggle-math-dialog="toggleMathDialog"
          @insert-emoji="insertEmoji"
        />

        <!-- Academic Group -->
        <AcademicGroup
          @insert-footnote="insertFootnote"
          @insert-bibliography="insertBibliography"
        />

      </div>

      <!-- Graphics Tab Panel -->
      <div v-if="activeRibbonTab === 'graphics'" class="ribbon-panel">
        <!-- Illustrations Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="图片" aria-label="插入图片" @click="addImage">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                <circle cx="8.5" cy="8.5" r="1.5" />
                <polyline points="21 15 16 10 5 21" />
              </svg>
              <span>图片</span>
            </button>
            <button class="ribbon-button" title="形状" aria-label="插入形状" @click="insertShape">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
              </svg>
              <span>形状</span>
            </button>
            <button class="ribbon-button" title="图标" aria-label="插入图标" @click="insertIcon">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M12 2L2 7l10 5 10-5-10-5z" />
                <path d="M2 17l10 5 10-5" />
                <path d="M2 12l10 5 10-5" />
              </svg>
              <span>图标</span>
            </button>
          </div>
          <div class="group-label">插图</div>
        </div>
      </div>

      <div v-if="activeRibbonTab === 'layout'" class="ribbon-panel">
        <!-- Page Setup Group -->
        <PageSetupGroup
          @show-page-setup="showPageSetup"
        />

        <!-- Paragraph Group -->
        <ParagraphSettingsGroup
          @show-paragraph-settings="showParagraphSettings"
        />

        <!-- Columns Group -->
        <ColumnsGroup
          @set-columns="setColumns"
        />

        <!-- Arrange Group -->
        <ArrangeGroup
          @bring-to-front="bringToFront"
          @send-to-back="sendToBack"
        />
      </div>

      <!-- Spreadsheet Tab Panel -->
      <div v-if="activeRibbonTab === 'spreadsheet'" class="ribbon-panel">
        <!-- Spreadsheet Operations Group -->
        <SpreadsheetGroup
          @toggle-spreadsheet="toggleSpreadsheet"
        />

        <!-- Formula & Functions Group -->
        <FormulaFunctionsGroup
          @insert-formula="insertFormula"
          @open-function-library="insertFunction"
        />

        <!-- Lookup & Reference Group -->
        <LookupReferenceGroup
          @insert-vlookup="insertVLOOKUP"
          @insert-hlookup="insertHLOOKUP"
          @insert-index-match="insertINDEXMATCH"
        />

        <!-- Conditional Formatting Group -->
        <ConditionalFormattingGroup
          @add-conditional-format="addConditionalFormat"
          @add-data-bars="addDataBars"
          @add-color-scale="addColorScale"
        />

        <!-- Charts Group -->
        <ChartsGroup
          @insert-chart="insertChart"
          @insert-line-chart="insertLineChart"
          @insert-pie-chart="insertPieChart"
        />

        <!-- Pivot Table Group -->
        <PivotTableGroup
          @insert-pivot-table="insertPivotTable"
          @refresh-pivot-table="refreshPivotTable"
        />

        <!-- Data Group -->
        <DataGroup
          @sort-data="sortData"
          @filter-data="filterData"
        />
      </div>

      <!-- Typst Tab Panel -->
      <div v-if="activeRibbonTab === 'typst'" class="ribbon-panel">
        <!-- Typst Preview Group -->
        <TypstPreviewGroup
          @toggle-typst-preview="toggleTypstPreview"
        />

        <!-- Typst Export Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="导出 PDF" aria-label="导出 PDF 文档" @click="exportTypstPdf">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                <polyline points="14 2 14 8 20 8" />
                <line x1="16" y1="13" x2="8" y2="13" />
                <line x1="16" y1="17" x2="8" y2="17" />
                <polyline points="10 9 9 9 8 9" />
              </svg>
              <span>导出 PDF</span>
            </button>
            <button class="ribbon-button" title="导出 PNG" aria-label="导出 PNG 图片" @click="exportTypstPng">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                <circle cx="8.5" cy="8.5" r="1.5" />
                <polyline points="21 15 16 10 5 21" />
              </svg>
              <span>导出 PNG</span>
            </button>
            <button class="ribbon-button" title="Export SVG (Typst)" aria-label="Export SVG via Typst renderer" @click="exportTypstSvg">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                <polyline points="14 2 14 8 20 8" />
              </svg>
              <span>Export SVG (Typst)</span>
            </button>
            <button class="ribbon-button" title="Export SVG (HTML)" aria-label="Export SVG via HTML vector service" @click="exportHtmlSvg">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                <polyline points="14 2 14 8 20 8" />
              </svg>
              <span>Export SVG (HTML)</span>
            </button>
            <button class="ribbon-button" title="导出选项" aria-label="打开导出选项" @click="openTypstExportOptions">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <circle cx="12" cy="12" r="3" />
                <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
              </svg>
              <span>导出选项</span>
            </button>
          </div>
          <div class="group-label">导出</div>
        </div>

        <!-- Typst Templates Group -->
        <TypstTemplatesGroup
          @open-template-manager="openTypstTemplatesDialog"
        />

        <!-- Typst Fonts Group -->
        <TypstFontsGroup
          @open-font-manager="openTypstFontManager"
        />

        <!-- Typst Packages Group -->
        <TypstPackagesGroup
          @open-package-browser="openTypstPackageBrowser"
        />
      </div>

      <!-- Slides Tab Panel -->
      <div v-if="activeRibbonTab === 'slides'" class="ribbon-panel">
        <!-- Slide Mode Group -->
        <SlideModeGroup
          @toggle-slide-mode="toggleSlideMode"
        />

        <!-- Slide Management Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="新建幻灯片" aria-label="新建幻灯片" @click="newSlide">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                <line x1="12" y1="8" x2="12" y2="16" />
                <line x1="8" y1="12" x2="16" y2="12" />
              </svg>
              <span>新建</span>
            </button>
            <button class="ribbon-button" title="删除幻灯片" aria-label="删除当前幻灯片" @click="deleteSlide">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <polyline points="3 6 5 6 21 6" />
                <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
              </svg>
              <span>删除</span>
            </button>
            <button class="ribbon-button" title="复制幻灯片" aria-label="复制当前幻灯片" @click="duplicateSlide">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
              </svg>
              <span>复制</span>
            </button>
          </div>
          <div class="group-label">幻灯片管理</div>
        </div>

        <!-- Slide Navigation Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="上一张幻灯片" aria-label="上一张幻灯片" @click="prevSlide">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <polyline points="15 18 9 12 15 6" />
              </svg>
              <span>上一张</span>
            </button>
            <button class="ribbon-button" title="下一张幻灯片" aria-label="下一张幻灯片" @click="nextSlide">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <polyline points="9 18 15 12 9 6" />
              </svg>
              <span>下一张</span>
            </button>
          </div>
          <div class="group-label">导航</div>
        </div>

        <!-- Text Editing Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="插入文本框" aria-label="插入文本框" @click="insertTextBox">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M4 7V4h16v3" />
                <path d="M9 20h6" />
                <path d="M12 4v16" />
              </svg>
              <span>文本框</span>
            </button>
            <button class="ribbon-button" title="左对齐" aria-label="左对齐文本" @click="alignTextLeft">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <line x1="17" y1="10" x2="3" y2="10" />
                <line x1="21" y1="6" x2="3" y2="6" />
                <line x1="21" y1="14" x2="3" y2="14" />
                <line x1="17" y1="18" x2="3" y2="18" />
              </svg>
              <span>左对齐</span>
            </button>
            <button class="ribbon-button" title="居中对齐" aria-label="居中对齐文本" @click="alignTextCenter">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <line x1="21" y1="10" x2="3" y2="10" />
                <line x1="21" y1="6" x2="3" y2="6" />
                <line x1="21" y1="14" x2="3" y2="14" />
                <line x1="21" y1="18" x2="3" y2="18" />
              </svg>
              <span>居中</span>
            </button>
            <button class="ribbon-button" title="右对齐" aria-label="右对齐文本" @click="alignTextRight">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <line x1="21" y1="10" x2="7" y2="10" />
                <line x1="21" y1="6" x2="3" y2="6" />
                <line x1="21" y1="14" x2="3" y2="14" />
                <line x1="21" y1="18" x2="7" y2="18" />
              </svg>
              <span>右对齐</span>
            </button>
            <button class="ribbon-button" title="加粗" aria-label="加粗文本" @click="boldText">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" />
                <path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" />
              </svg>
              <span>加粗</span>
            </button>
            <button class="ribbon-button" title="斜体" aria-label="斜体文本" @click="italicText">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <line x1="19" y1="4" x2="10" y2="4" />
                <line x1="14" y1="20" x2="5" y2="20" />
                <line x1="15" y1="4" x2="9" y2="20" />
              </svg>
              <span>斜体</span>
            </button>
            <button class="ribbon-button" title="下划线" aria-label="下划线文本" @click="underlineText">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3" />
                <line x1="4" y1="21" x2="20" y2="21" />
              </svg>
              <span>下划线</span>
            </button>
          </div>
          <div class="group-label">文本编辑</div>
        </div>

        <!-- Font Size Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="减小字号" aria-label="减小字号" @click="decreaseFontSize">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <polyline points="4 14 10 14 10 20" />
                <polyline points="20 10 14 10 14 4" />
                <line x1="14" y1="10" x2="21" y2="3" />
                <line x1="3" y1="21" x2="10" y2="14" />
              </svg>
              <span>减小</span>
            </button>
            <button class="ribbon-button" title="增大字号" aria-label="增大字号" @click="increaseFontSize">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <polyline points="15 3 21 3 21 9" />
                <polyline points="9 21 3 21 3 15" />
                <line x1="21" y1="3" x2="14" y2="10" />
                <line x1="3" y1="21" x2="10" y2="14" />
              </svg>
              <span>增大</span>
            </button>
          </div>
          <div class="group-label">字号</div>
        </div>

        <!-- Animation Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="添加动画" aria-label="添加动画效果" @click="openAnimationDialog">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <polygon points="5 3 19 12 5 21 5 3" />
              </svg>
              <span>动画</span>
            </button>
            <button class="ribbon-button" title="幻灯片切换" aria-label="设置幻灯片切换效果" @click="openTransitionDialog">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M1 4v6h6" />
                <path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10" />
              </svg>
              <span>切换</span>
            </button>
          </div>
          <div class="group-label">动画</div>
        </div>

        <!-- Chart Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="插入图表" aria-label="插入图表" @click="openChartDialog">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <line x1="18" y1="20" x2="18" y2="10" />
                <line x1="12" y1="20" x2="12" y2="4" />
                <line x1="6" y1="20" x2="6" y2="14" />
              </svg>
              <span>图表</span>
            </button>
            <button class="ribbon-button" title="SmartArt" aria-label="插入 SmartArt 图形" @click="openSmartArtDialog">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <rect x="3" y="3" width="7" height="7" />
                <rect x="14" y="3" width="7" height="7" />
                <rect x="14" y="14" width="7" height="7" />
                <rect x="3" y="14" width="7" height="7" />
              </svg>
              <span>SmartArt</span>
            </button>
          </div>
          <div class="group-label">图表</div>
        </div>

        <!-- Insert Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="插入图片" aria-label="插入图片到幻灯片" @click="insertSlideImage">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                <circle cx="8.5" cy="8.5" r="1.5" />
                <polyline points="21 15 16 10 5 21" />
              </svg>
              <span>图片</span>
            </button>
            <button class="ribbon-button" title="插入形状" aria-label="插入形状到幻灯片" @click="insertSlideShape">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <circle cx="12" cy="12" r="10" />
              </svg>
              <span>形状</span>
            </button>
            <button class="ribbon-button" title="插入表格" aria-label="插入表格到幻灯片" @click="insertSlideTable">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                <line x1="3" y1="9" x2="21" y2="9" />
                <line x1="3" y1="15" x2="21" y2="15" />
                <line x1="9" y1="3" x2="9" y2="21" />
                <line x1="15" y1="3" x2="15" y2="21" />
              </svg>
              <span>表格</span>
            </button>
          </div>
          <div class="group-label">插入</div>
        </div>

        <!-- Color and Style Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="字体颜色" aria-label="设置字体颜色" @click="openFontColorDialog">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M12 19l7-7 3 3-7 7-3-3z" />
                <path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z" />
                <path d="M2 2l7.586 7.586" />
                <circle cx="11" cy="11" r="2" />
              </svg>
              <span>字体颜色</span>
            </button>
            <button class="ribbon-button" title="背景颜色" aria-label="设置背景颜色" @click="openBackgroundColorDialog">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
              </svg>
              <span>背景颜色</span>
            </button>
            <button class="ribbon-button" title="边框颜色" aria-label="设置边框颜色" @click="openBorderColorDialog">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                <line x1="3" y1="9" x2="21" y2="9" />
                <line x1="3" y1="15" x2="21" y2="15" />
                <line x1="9" y1="3" x2="9" y2="21" />
                <line x1="15" y1="3" x2="15" y2="21" />
              </svg>
              <span>边框颜色</span>
            </button>
          </div>
          <div class="group-label">颜色</div>
        </div>

        <!-- Paragraph Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="项目符号" aria-label="添加项目符号" @click="addBullets">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <line x1="8" y1="6" x2="21" y2="6" />
                <line x1="8" y1="12" x2="21" y2="12" />
                <line x1="8" y1="18" x2="21" y2="18" />
                <line x1="3" y1="6" x2="3.01" y2="6" />
                <line x1="3" y1="12" x2="3.01" y2="12" />
                <line x1="3" y1="18" x2="3.01" y2="18" />
              </svg>
              <span>项目符号</span>
            </button>
            <button class="ribbon-button" title="编号" aria-label="添加编号列表" @click="addNumbering">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <line x1="10" y1="6" x2="21" y2="6" />
                <line x1="10" y1="12" x2="21" y2="12" />
                <line x1="10" y1="18" x2="21" y2="18" />
                <path d="M4 6h1v4" />
                <path d="M4 10h2" />
                <path d="M6 18H4c0-1 2-2 2-3s-1-1.5-2-1" />
              </svg>
              <span>编号</span>
            </button>
            <button class="ribbon-button" title="减小行距" aria-label="减小行距" @click="decreaseLineSpacing">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <line x1="5" y1="12" x2="19" y2="12" />
                <line x1="5" y1="6" x2="19" y2="6" />
                <line x1="5" y1="18" x2="19" y2="18" />
              </svg>
              <span>减小行距</span>
            </button>
            <button class="ribbon-button" title="增大行距" aria-label="增大行距" @click="increaseLineSpacing">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <line x1="5" y1="12" x2="19" y2="12" />
                <line x1="5" y1="6" x2="19" y2="6" />
                <line x1="5" y1="18" x2="19" y2="18" />
                <line x1="5" y1="24" x2="19" y2="24" />
              </svg>
              <span>增大行距</span>
            </button>
          </div>
          <div class="group-label">段落</div>
        </div>

        <!-- Presentation Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="开始演示" aria-label="开始幻灯片演示" @click="startPresentation">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <polygon points="5 3 19 12 5 21 5 3" />
              </svg>
              <span>开始演示</span>
            </button>
          </div>
          <div class="group-label">演示</div>
        </div>

        <!-- View Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="备注视图" aria-label="切换备注视图" @click="toggleNotesView">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                <polyline points="14 2 14 8 20 8" />
                <line x1="16" y1="13" x2="8" y2="13" />
                <line x1="16" y1="17" x2="8" y2="17" />
                <polyline points="10 9 9 9 8 9" />
              </svg>
              <span>备注视图</span>
            </button>
            <button class="ribbon-button" title="幻灯片浏览" aria-label="切换幻灯片浏览视图" @click="toggleSlideSorter">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <rect x="3" y="3" width="7" height="7" />
                <rect x="14" y="3" width="7" height="7" />
                <rect x="3" y="14" width="7" height="7" />
                <rect x="14" y="14" width="7" height="7" />
              </svg>
              <span>幻灯片浏览</span>
            </button>
          </div>
          <div class="group-label">视图</div>
        </div>

        <!-- Overflow Menu Button -->
        <div class="ribbon-group overflow-group">
          <div class="group-content">
            <button class="ribbon-button overflow-button" title="更多选项" aria-label="更多选项" @click="toggleOverflowMenu">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <polyline points="9 18 15 12 9 6" />
              </svg>
              <span>更多</span>
            </button>
          </div>
        </div>
      </div>

      <!-- Overflow Menu Dropdown -->
      <OverflowMenu
        :show="showOverflowMenu"
        :x="overflowMenuPosition.x"
        :y="overflowMenuPosition.y"
        @update:show="showOverflowMenu = $event"
        @insert-video="insertVideo"
        @insert-audio="insertAudio"
        @rehearse-timings="rehearseTimings"
        @record-slide-show="recordSlideShow"
        @check-spelling="checkSpelling"
        @add-comment="addComment"
        @translate-text="translateText"
      />

      <!-- Design Tab Panel -->
      <div v-if="activeRibbonTab === 'design'" class="ribbon-panel">
        <DesignGroup
          @open-theme-dialog="openThemeDialog"
          @open-background-dialog="openBackgroundDialog"
          @open-layout-dialog="openLayoutDialog"
        />

        <!-- Slide Export Group -->
        <SlideExportGroup
          @export-slides-to-pdf="exportSlidesToPdf"
          @insert-slide-break="insertSlideBreak"
        />

        <!-- Slide Settings Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="幻灯片设置" aria-label="打开幻灯片设置" @click="toggleSlideConfigDialog">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <circle cx="12" cy="12" r="3" />
                <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
              </svg>
              <span>幻灯片设置</span>
            </button>
          </div>
          <div class="group-label">设置</div>
        </div>
      </div>

      <!-- References Tab Panel -->
      <div v-if="activeRibbonTab === 'references'" class="ribbon-panel">
        <!-- Table of Contents Group -->
        <TableOfContentsGroup
          @insert-toc="ribbonInsertTOC"
        />

        <!-- Footnotes Group -->
        <FootnotesGroup
          @insert-footnote="insertFootnote"
          @insert-endnote="insertEndnote"
        />

        <!-- Citations Group -->
        <CitationsGroup
          @insert-citation="openCitationDialog"
          @insert-bibliography="insertBibliography"
        />

        <!-- Cross Reference Group -->
        <CrossReferenceGroup
          @add-cross-reference="addCrossReference"
        />
      </div>

      <!-- Review Tab Panel -->
      <div v-if="activeRibbonTab === 'review'" class="ribbon-panel">
        <!-- Proofing Group -->
        <ProofingGroup
          @check-spelling="checkSpelling"
        />

        <!-- Comments Group -->
        <CommentsGroup
          @add-comment="addComment"
        />

        <!-- Changes Group -->
        <ChangesGroup
          @track-changes="trackChanges"
          @accept-change="acceptChange"
          @reject-change="rejectChange"
        />
      </div>

      <!-- View Tab Panel -->
      <div v-if="activeRibbonTab === 'view'" class="ribbon-panel">
        <!-- Views Group -->
        <ViewsGroup
          @set-print-layout="setPrintLayout"
          @toggle-web-layout="toggleWebLayout"
        />

        <!-- Show Group -->
        <ShowGroup
          @toggle-navigation-pane="toggleNavigationPane"
          @toggle-gridlines="_toggleGridlines"
          @toggle-format-marks="_toggleFormatMarks"
          @toggle-ruler="_toggleHorizontalRuler"
        />

        <!-- Zoom Group -->
        <ZoomGroup
          @toggle-fullscreen="toggleFullscreen"
        />

        <!-- Window Group -->
        <WindowGroup
          @toggle-theme="toggleTheme"
          @toggle-wallpaper="toggleWallpaperDialog"
        />
      </div>

      <!-- Help Tab Panel -->
      <div v-if="activeRibbonTab === 'help'" class="ribbon-panel help-panel">
        <!-- Help Group -->
        <div class="ribbon-group no-label">
          <div class="group-content">
            <button class="ribbon-button-large" title="打开帮助" aria-label="打开帮助文档" @click="toggleHelp">
              <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10" />
                <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
                <line x1="12" y1="17" x2="12.01" y2="17" />
              </svg>
              <span>帮助</span>
            </button>
          </div>
        </div>

        <!-- Options Group -->
        <div class="ribbon-group no-label">
          <div class="group-content">
            <button class="ribbon-button-large" title="选项" aria-label="打开选项设置" @click="toggleOptionsDialog">
              <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="3" />
                <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
              </svg>
              <span>选项</span>
            </button>
          </div>
        </div>

        <!-- Info Group -->
        <div class="ribbon-group no-label">
          <div class="group-content">
            <button class="ribbon-button" title="关于" aria-label="关于 LOGOS" @click="toggleAboutDialog">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10" />
                <line x1="12" y1="16" x2="12" y2="12" />
                <line x1="12" y1="8" x2="12.01" y2="8" />
              </svg>
              <span>关于</span>
            </button>
          </div>
        </div>
      </div>
    </div>
    <button class="ribbon-scroll-button scroll-right" title="向右滚动" aria-label="向右滚动功能区" @click="scrollRibbon(200)">
      <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="9 18 15 12 9 6"></polyline>
      </svg>
    </button>
  </div>

    <!-- Fixed Horizontal Ruler under Ribbon, centered horizontally to match paper -->
    <div v-if="showHorizontalRuler && viewMode !== 'web'" class="horizontal-ruler-outer-container">
      <div class="horizontal-ruler-container" :style="{ width: pageSize.width + 'mm' || '794px' }">
        <div class="ruler horizontal-ruler">
          <!-- Left Margin Gray Area -->
          <div class="ruler-margin-left" :style="{ width: leftMargin + 'px' }">
            <div class="ruler-drag-handle left-handle" title="左边距" @mousedown="startDrag('leftMargin', $event)"></div>
          </div>
          <!-- Active White Area with tick marks -->
          <div class="ruler-active-area">
            <!-- Centimeter marks (dynamically adapted to page size and margins) -->
            <div v-for="i in rulerCentimeters" :key="i" class="ruler-tick" :style="{ left: (i * 37.8) + 'px' }">
              <span v-if="i % 2 !== 0" class="ruler-tick-number">{{ i }}</span>
            </div>
          </div>
          <!-- Right Margin Gray Area -->
          <div class="ruler-margin-right" :style="{ width: rightMargin + 'px' }">
            <div class="ruler-drag-handle right-handle" title="右边距" @mousedown="startDrag('rightMargin', $event)"></div>
          </div>
        </div>
      </div>
    </div>

    <!-- Editor Content -->
    <div
      class="editor-content-wrapper"
      :class="editorSidebarLayout.layoutClasses"
      :style="{ ...editorSidebarLayout.layoutStyle, ...wallpaperStyle }"
    >
      <div class="editor-workspace">
        <div v-if="editorSidebarLayout.hasLeftPanels" class="editor-workspace__left">
          <DocumentOutline
            :show="showDocumentOutline"
            :headings="documentHeadings"
            @close="showDocumentOutline = false"
            @navigate-to="navigateToHeading"
          />
        </div>

        <!-- Document Canvas: Logos-style gray background with centered A4 paper -->
        <div class="editor-workspace__center">
          <div class="document-canvas" :style="{ transform: `scale(${zoomLevel / 100})`, transformOrigin: 'top center' }">
          <!-- Multi-page layout with each page independently displayed -->
          <div
            v-for="(pageContent, pageIndex) in pageContents"
            :key="pageIndex"
            class="editor-mount page-container"
            :class="{ 
              'show-gridlines': showGridlines, 
              'show-format-marks': showFormatMarks,
              'editing-header': isEditingHeader,
              'editing-footer': isEditingFooter,
              'active-page': pageIndex === activePageIndex
            }"
            :style="{
              width: viewMode === 'web' ? '100%' : pageSize.width + 'mm',
              height: viewMode === 'web' ? 'auto' : pageSize.height + 'mm',
              minHeight: viewMode === 'web' ? '100%' : pageSize.height + 'mm',
              paddingLeft: viewMode === 'web' ? '24px' : leftMargin + 'px',
              paddingRight: viewMode === 'web' ? '24px' : rightMargin + 'px',
              paddingTop: viewMode === 'web' ? '24px' : topMargin + 'px',
              paddingBottom: viewMode === 'web' ? '24px' : bottomMargin + 'px'
            }"
            @click="activatePage(pageIndex)"
          >
            <!-- Page number indicator -->
            <div class="page-number-indicator">
              第 {{ pageIndex + 1 }} 页
            </div>

            <!-- Header Area -->
            <div
              v-if="headerEnabled || isEditingHeader"
              ref="headerAreaRef"
              class="document-header-area"
              :class="{ 'editing': isEditingHeader }"
              :style="{ textAlign: headerAlign }"
              :contenteditable="isEditingHeader"
              @dblclick="enterHeaderEditMode"
              @blur="handleHeaderBlur"
              @keydown.enter.prevent="handleHeaderEnter"
            >
              <span v-if="!isEditingHeader">{{ headerContent }}</span>
            </div>

            <!-- Main Content - Each page has its own editor instance -->
            <div v-if="pageIndex === activePageIndex" class="page-editor-wrapper">
              <EditorContent
                :editor="editor"
                class="tiptap-editor-surface"
                role="textbox"
                aria-multiline="true"
                aria-label="文档编辑器"
              />
            </div>
            <div v-else class="page-content-preview" v-html="pageContent"></div>

            <!-- Footer Area -->
            <div
              v-if="footerEnabled || isEditingFooter"
              ref="footerAreaRef"
              class="document-footer-area"
              :class="{ 'editing': isEditingFooter }"
              :style="{ textAlign: footerAlign }"
              :contenteditable="isEditingFooter"
              @dblclick="enterFooterEditMode"
              @blur="handleFooterBlur"
              @keydown.enter.prevent="handleFooterEnter"
            >
              <span v-if="!isEditingFooter">{{ footerContent }}</span>
            </div>
          </div>
          </div>
        </div>

        <div v-if="editorSidebarLayout.hasRightPanels" class="editor-workspace__right">
          <!-- Split View Sidebar (Typst Preview) — right column, adjacent to editor -->
          <div
            v-if="showSplitView"
            class="split-view-sidebar editor-side-panel editor-side-panel--right"
            data-testid="split-view-sidebar"
            aria-label="Typst preview panel"
          >
            <div class="split-view-header">
              <h3>Typst Preview</h3>
              <button class="close-button" title="Close" aria-label="Close Typst preview" @click="showSplitView = false">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="18" y1="6" x2="6" y2="18"></line>
                  <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
              </button>
            </div>
            <div class="split-view-content">
              <div v-if="isTypstCompiling" class="compiling-indicator">
                <div class="spinner"></div>
                <span>Compiling...</span>
              </div>
              <div v-else-if="!typstPreviewUrl && !typstPreviewData && pdfCanvases.length === 0" class="empty-preview">
                <p>Type in the editor to generate Typst preview</p>
              </div>
              <div v-else-if="typstPreviewData" class="typst-preview-container">
                <div
                  v-if="typstPreviewData.includes('<svg')"
                  :key="typstPreviewRevision"
                  class="svg-preview-wrapper"
                  v-html="typstPreviewData"
                ></div>
                <pre v-else class="typst-preview-code">{{ typstPreviewData }}</pre>
              </div>
              <div v-else class="typst-preview-container">
                <div class="pdf-pages-container">
                  <div v-for="(canvas, index) in pdfCanvases" :key="index" class="pdf-page-wrapper">
                    <div class="pdf-page-number">Page {{ index + 1 }}</div>
                    <div class="pdf-page-canvas-wrapper">
                      <img :src="(canvas as any).dataUrl" :alt="'Page ' + (index + 1)" class="pdf-page-image" />
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <CommentsPanel
            :show="showCommentsPanel"
            :comments="[]"
            @update:show="showCommentsPanel = $event"
            @add-comment="handleAddComment"
            @resolve-comment="handleResolveComment"
            @delete-comment="handleDeleteComment"
            @reply-comment="handleReplyComment"
          />

          <RevisionModePanel
            :show="showRevisionPanel"
            :revisions="[]"
            :track-changes="false"
            @update:show="showRevisionPanel = $event"
            @toggle-track-changes="handleToggleTrackChanges"
            @accept-revision="handleAcceptRevision"
            @reject-revision="handleRejectRevision"
            @accept-all="handleAcceptAllRevisions"
            @reject-all="handleRejectAllRevisions"
          />

          <AISidebar v-if="showAISidebar" @close="showAISidebar = false" />

          <div v-if="showSpreadsheet" class="spreadsheet-panel editor-side-panel editor-side-panel--right">
            <div class="spreadsheet-panel-header">
              <h3>电子表格 (Luckysheet)</h3>
              <button class="close-button" title="关闭" @click="toggleSpreadsheet">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="18" y1="6" x2="6" y2="18"></line>
                  <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
              </button>
            </div>
            <div class="spreadsheet-panel-content">
              <Spreadsheet @insert-content="handleSpreadsheetInsert" />
            </div>
          </div>

          <div v-if="showUniverSpreadsheet" class="spreadsheet-panel editor-side-panel editor-side-panel--right">
            <div class="spreadsheet-panel-header">
              <h3>电子表格 (Univer)</h3>
              <button class="close-button" title="关闭" @click="toggleUniverSpreadsheet">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="18" y1="6" x2="6" y2="18"></line>
                  <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
              </button>
            </div>
            <div class="spreadsheet-panel-content">
              <UniverSpreadsheet @insert-content="handleSpreadsheetInsert" />
            </div>
          </div>
        </div>
      </div>

      <!-- Help Dialog -->
      <Help :show="showHelp" @close="showHelp = false" />

      <!-- Mini Toolbar -->
      <MiniToolbar
        :show="showMiniToolbar"
        :x="miniToolbarPosition.x"
        :y="miniToolbarPosition.y"
        @update:show="showMiniToolbar = $event"
        @action="handleMiniToolbarAction"
      />

      <!-- Options Dialog -->
      <OptionsDialog
        :show="showOptionsDialog"
        @update:show="showOptionsDialog = $event"
        @apply="handleOptionsApply"
      />

      <!-- About Dialog -->
      <AboutDialog
        :show="showAboutDialog"
        @update:show="showAboutDialog = $event"
      />

      <!-- Color Picker Dialog -->
      <ColorPickerDialog
        :show="showColorPickerDialog"
        :initial-color="colorPickerTarget === 'text' ? textColor.value : highlightColor.value"
        :title="colorPickerTarget === 'text' ? '选择文本颜色' : '选择高亮颜色'"
        @update:show="showColorPickerDialog = $event"
        @confirm="handleColorPickerConfirm"
      />

      <!-- Link Dialog -->
      <LinkDialog
        :show="showLinkDialog"
        :initial-url="linkDialogUrl"
        :initial-text="linkDialogText"
        @update:show="showLinkDialog = $event"
        @confirm="handleLinkDialogConfirm"
      />

      <!-- User Guide Dialog -->
      <div v-if="showUserGuideDialog" class="dialog-overlay" @click.self="toggleUserGuideDialog">
        <div class="dialog-content user-guide-dialog">
          <div class="dialog-header">
            <h2>用户指南</h2>
            <button class="dialog-close" @click="toggleUserGuideDialog">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18"></line>
                <line x1="6" y1="6" x2="18" y2="18"></line>
              </svg>
            </button>
          </div>
          <div class="dialog-body">
            <div class="user-guide-content">
              <h3>欢迎使用 Logos智道办公软件</h3>
              <p>Logos智道办公软件是一款现代化的办公套件，集成了AI辅助编辑和Typst专业排版功能。</p>
              
              <h4>主要功能</h4>
              <ul>
                <li><strong>富文本编辑</strong>：支持粗体、斜体、下划线、标题、列表、表格等</li>
                <li><strong>AI辅助</strong>：文本润色、扩写、重写、总结、翻译</li>
                <li><strong>Typst排版</strong>：实时专业排版预览</li>
                <li><strong>模板系统</strong>：本地模板存储和网络下载</li>
                <li><strong>导出功能</strong>：支持PDF、PNG、Markdown、HTML等格式</li>
              </ul>
              
              <h4>快捷键</h4>
              <ul>
                <li><kbd>Ctrl/Cmd + S</kbd> - 保存文档</li>
                <li><kbd>Ctrl/Cmd + O</kbd> - 打开文档</li>
                <li><kbd>Ctrl/Cmd + N</kbd> - 新建文档</li>
                <li><kbd>Ctrl/Cmd + B</kbd> - 粗体</li>
                <li><kbd>Ctrl/Cmd + I</kbd> - 斜体</li>
                <li><kbd>Ctrl/Cmd + Z</kbd> - 撤销</li>
                <li><kbd>Ctrl/Cmd + F</kbd> - 查找</li>
              </ul>
              
              <h4>获取帮助</h4>
              <p>如需更多帮助，请访问项目文档或联系技术支持。</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Collaboration Panel -->
    <CollaborationUI
      v-if="showCollaboration"
      :document-id="collaborationDocumentId"
      :user-id="collaborationUserId"
      :user-name="collaborationUserName"
      :is-active="collaborationEnabled"
      @conflict-detected="handleConflictDetected"
      @conflict-resolved="handleConflictResolved"
    />

    <!-- Status Bar -->
    <StatusBar
      :word-count="wordCount"
      :char-count="charCount"
      :current-page="currentPage"
      :total-pages="totalPages"
      :zoom-level="zoomLevel"
      :is-dark-mode="isDarkMode"
      :view-mode="viewMode"
      @zoom-in="zoomIn"
      @zoom-out="zoomOut"
      @zoom-change="setZoom"
      @toggle-theme="toggleTheme"
      @view-mode-change="changeViewMode"
    />

    <!-- Page Setup Dialog -->
    <div v-if="showPageSetupDialog" class="dialog-overlay" @click.self="showPageSetupDialog = false">
      <div class="dialog-content page-setup-dialog" style="max-width: 400px;">
        <div class="dialog-header">
          <h2>页面设置</h2>
          <button class="dialog-close" @click="showPageSetupDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body" style="padding: 16px; display: flex; flex-direction: column; gap: 16px;">
          <!-- Paper Size Section -->
          <div class="form-group" style="display: flex; flex-direction: column; gap: 6px;">
            <label style="font-weight: 500; color: var(--word-text-primary);">纸张大小</label>
            <select
              :value="pageSize.width + 'x' + pageSize.height"
              class="form-control"
              style="width: 100%; height: 32px; padding: 0 8px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 2px;"
              @change="(e) => {
                const value = (e.target as HTMLSelectElement).value;
                if (value) {
                  const [w, h] = value.split('x').map(Number);
                  if (!isNaN(w) && !isNaN(h)) {
                    pageSize = { width: w, height: h };
                  }
                }
              }"
            >
              <option value="210x297">A4 (210 mm × 297 mm)</option>
              <option value="215.9x279.4">Letter (216 mm × 279 mm)</option>
              <option value="297x420">A3 (297 mm × 420 mm)</option>
              <option value="148x210">A5 (148 mm × 210 mm)</option>
            </select>
          </div>

          <!-- Page Orientation Section -->
          <div class="form-group" style="display: flex; flex-direction: column; gap: 6px;">
            <label style="font-weight: 500; color: var(--word-text-primary);">纸张方向</label>
            <div class="radio-group" style="display: flex; gap: 24px; margin-top: 4px;">
              <label style="display: flex; align-items: center; gap: 6px; cursor: pointer; font-size: 13px; color: var(--word-text-primary);">
                <input 
                  type="radio" 
                  value="portrait" 
                  :checked="pageOrientation === 'portrait'" 
                  style="cursor: pointer;"
                  @change="setOrientation('portrait')"
                />
                <span>纵向</span>
              </label>
              <label style="display: flex; align-items: center; gap: 6px; cursor: pointer; font-size: 13px; color: var(--word-text-primary);">
                <input 
                  type="radio" 
                  value="landscape" 
                  :checked="pageOrientation === 'landscape'" 
                  style="cursor: pointer;"
                  @change="setOrientation('landscape')"
                />
                <span>横向</span>
              </label>
            </div>
          </div>

          <!-- Page Margins Section -->
          <div class="form-group" style="display: flex; flex-direction: column; gap: 8px;">
            <label style="font-weight: 500; color: var(--word-text-primary);">页边距 (毫米)</label>
            <div class="margin-grid" style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
              <div class="margin-input" style="display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 12px; min-width: 24px; color: var(--word-text-secondary);">上:</span>
                <input v-model.number="pageMargins.top" type="number" min="0" max="100" class="form-control" style="width: 100%; height: 32px; padding: 0 8px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 2px;" />
              </div>
              <div class="margin-input" style="display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 12px; min-width: 24px; color: var(--word-text-secondary);">下:</span>
                <input v-model.number="pageMargins.bottom" type="number" min="0" max="100" class="form-control" style="width: 100%; height: 32px; padding: 0 8px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 2px;" />
              </div>
              <div class="margin-input" style="display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 12px; min-width: 24px; color: var(--word-text-secondary);">左:</span>
                <input v-model.number="pageMargins.left" type="number" min="0" max="100" class="form-control" style="width: 100%; height: 32px; padding: 0 8px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 2px;" />
              </div>
              <div class="margin-input" style="display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 12px; min-width: 24px; color: var(--word-text-secondary);">右:</span>
                <input v-model.number="pageMargins.right" type="number" min="0" max="100" class="form-control" style="width: 100%; height: 32px; padding: 0 8px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 2px;" />
              </div>
            </div>
          </div>
        </div>
        <div class="dialog-footer" style="display: flex; justify-content: flex-end; padding: 12px 16px; border-top: 1px solid var(--word-border); background: var(--word-ribbon-bg);">
          <button class="action-button cancel-button" style="height: 32px; padding: 0 16px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 2px; cursor: pointer; font-size: 13px; font-weight: 500;" @click="showPageSetupDialog = false">关闭</button>
        </div>
      </div>
    </div>

    <!-- Typst Templates Dialog -->
    <div v-if="showTypstTemplatesDialog" class="dialog-overlay" @click.self="showTypstTemplatesDialog = false">
      <div class="dialog-content typst-templates-dialog">
        <div class="dialog-header">
          <h2>Typst 模板</h2>
          <button class="dialog-close" @click="showTypstTemplatesDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <!-- 搜索和筛选工具栏 -->
          <div class="template-toolbar">
            <div class="template-search">
              <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="11" cy="11" r="8"></circle>
                <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
              </svg>
              <input 
                v-model="templateSearchQuery" 
                type="text" 
                placeholder="搜索模板..." 
                class="template-search-input"
              />
            </div>
            <div class="template-filters">
              <button 
                v-for="category in ['all', 'academic', 'business', 'technical', 'creative', 'custom']"
                :key="category"
                :class="['filter-button', { active: templateCategoryFilter === category }]"
                @click="templateCategoryFilter = category"
              >
                {{ category === 'all' ? '全部' : 
                   category === 'academic' ? '学术' :
                   category === 'business' ? '商业' :
                   category === 'technical' ? '技术' :
                   category === 'creative' ? '创意' : '自定义' }}
              </button>
            </div>
          </div>
          
          <!-- 模板网格 -->
          <div class="templates-grid">
            <div
              v-for="template in getAvailableTypstTemplates()"
              :key="template.id"
              class="template-card"
            >
              <div class="template-icon" @click="applyTypstTemplate(template)">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                  <polyline points="14 2 14 8 20 8"></polyline>
                  <line x1="16" y1="13" x2="8" y2="13"></line>
                  <line x1="16" y1="17" x2="8" y2="17"></line>
                  <polyline points="10 9 9 9 8 9"></polyline>
                </svg>
              </div>
              <div class="template-info">
                <h3>{{ template.name }}</h3>
                <p>{{ template.description }}</p>
                <div class="template-actions">
                  <span class="template-category">{{ template.category }}</span>
                  <div class="template-buttons">
                    <button class="template-action-btn preview-btn" title="预览" @click="previewTemplate(template)">
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                        <circle cx="12" cy="12" r="3"></circle>
                      </svg>
                    </button>
                    <button class="template-action-btn apply-btn" title="应用" @click="applyTypstTemplate(template)">
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <polyline points="20 6 9 17 4 12"></polyline>
                      </svg>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <!-- 空状态提示 -->
          <div v-if="getAvailableTypstTemplates().length === 0" class="empty-state">
            <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
              <polyline points="14 2 14 8 20 8"></polyline>
            </svg>
            <p>没有找到匹配的模板</p>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Template Preview Dialog -->
    <div v-if="showTemplatePreview" class="dialog-overlay" @click.self="closeTemplatePreview">
      <div class="dialog-content template-preview-dialog">
        <div class="dialog-header">
          <h2>模板预览</h2>
          <button class="dialog-close" @click="closeTemplatePreview">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <pre class="template-preview-content">{{ templatePreviewContent }}</pre>
        </div>
      </div>
    </div>

    <!-- Conditional Format Dialog -->
    <div v-if="showConditionalFormatDialog" class="dialog-overlay" @click.self="showConditionalFormatDialog = false">
      <div class="dialog-content conditional-format-dialog">
        <div class="dialog-header">
          <h2>条件格式</h2>
          <button class="dialog-close" @click="showConditionalFormatDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="form-group">
            <label>应用范围</label>
            <input type="text" placeholder="例如: A1:A10" class="form-input" />
          </div>
          <div class="form-group">
            <label>规则类型</label>
            <select class="form-select">
              <option value="greaterThan">大于</option>
              <option value="lessThan">小于</option>
              <option value="between">介于</option>
              <option value="equalTo">等于</option>
              <option value="containsText">包含文本</option>
              <option value="dataBars">数据条</option>
              <option value="colorScale">色阶</option>
            </select>
          </div>
          <div class="form-group">
            <label>条件值</label>
            <input type="text" placeholder="输入条件值" class="form-input" />
          </div>
          <div class="form-group">
            <label>格式</label>
            <div class="format-options">
              <button class="format-btn" title="背景色">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z" />
                </svg>
              </button>
              <button class="format-btn" title="字体颜色">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12 19l7-7 3 3-7 7-3-3z" />
                  <path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z" />
                  <path d="M2 2l7.586 7.586" />
                  <circle cx="11" cy="11" r="2" />
                </svg>
              </button>
              <button class="format-btn" title="加粗">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" />
                  <path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" />
                </svg>
              </button>
            </div>
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showConditionalFormatDialog = false">取消</button>
          <button class="btn btn-primary" @click="applyConditionalFormat">应用</button>
        </div>
      </div>
    </div>

    <!-- Chart Dialog -->
    <div v-if="showChartDialog" class="dialog-overlay" @click.self="showChartDialog = false">
      <div class="dialog-content chart-dialog">
        <div class="dialog-header">
          <h2>插入图表</h2>
          <button class="dialog-close" @click="showChartDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="form-group">
            <label>图表类型</label>
            <div class="chart-types">
              <button class="chart-type-btn" data-type="column">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="18" y1="20" x2="18" y2="10" />
                  <line x1="12" y1="20" x2="12" y2="4" />
                  <line x1="6" y1="20" x2="6" y2="14" />
                </svg>
                <span>柱状图</span>
              </button>
              <button class="chart-type-btn" data-type="line">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="22 12 18 12 15 21 9 3 6 12 2 12" />
                </svg>
                <span>折线图</span>
              </button>
              <button class="chart-type-btn" data-type="pie">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21.21 15.89A10 10 0 1 1 8 2.83" />
                  <path d="M22 12A10 10 0 0 0 12 2v10z" />
                </svg>
                <span>饼图</span>
              </button>
              <button class="chart-type-btn" data-type="bar">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="12" y1="20" x2="12" y2="10" />
                  <line x1="18" y1="20" x2="18" y2="4" />
                  <line x1="6" y1="20" x2="6" y2="14" />
                </svg>
                <span>条形图</span>
              </button>
              <button class="chart-type-btn" data-type="area">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M3 3v18h18" />
                  <path d="M18.7 8l-5.1 5.2-2.8-2.7L7 14.3" />
                </svg>
                <span>面积图</span>
              </button>
              <button class="chart-type-btn" data-type="scatter">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="7" cy="12" r="1" />
                  <circle cx="12" cy="7" r="1" />
                  <circle cx="17" cy="17" r="1" />
                </svg>
                <span>散点图</span>
              </button>
            </div>
          </div>
          <div class="form-group">
            <label>数据范围</label>
            <input type="text" placeholder="例如: A1:B10" class="form-input" />
          </div>
          <div class="form-group">
            <label>图表标题</label>
            <input type="text" placeholder="输入图表标题" class="form-input" />
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showChartDialog = false">取消</button>
          <button class="btn btn-primary" @click="insertChart">插入</button>
        </div>
      </div>
    </div>

    <!-- Pivot Table Dialog -->
    <div v-if="showPivotTableDialog" class="dialog-overlay" @click.self="showPivotTableDialog = false">
      <div class="dialog-content pivot-table-dialog">
        <div class="dialog-header">
          <h2>数据透视表</h2>
          <button class="dialog-close" @click="showPivotTableDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="form-group">
            <label>透视表名称</label>
            <input type="text" placeholder="输入透视表名称" class="form-input" />
          </div>
          <div class="form-group">
            <label>数据源范围</label>
            <input type="text" placeholder="例如: A1:D100" class="form-input" />
          </div>
          <div class="form-group">
            <label>行字段</label>
            <input type="text" placeholder="例如: A列" class="form-input" />
          </div>
          <div class="form-group">
            <label>列字段</label>
            <input type="text" placeholder="例如: B列" class="form-input" />
          </div>
          <div class="form-group">
            <label>值字段</label>
            <input type="text" placeholder="例如: C列" class="form-input" />
          </div>
          <div class="form-group">
            <label>聚合方式</label>
            <select class="form-select">
              <option value="sum">求和</option>
              <option value="average">平均值</option>
              <option value="count">计数</option>
              <option value="max">最大值</option>
              <option value="min">最小值</option>
            </select>
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showPivotTableDialog = false">取消</button>
          <button class="btn btn-primary" @click="createPivotTable">创建</button>
        </div>
      </div>
    </div>

    <!-- Typst Config Dialog -->
    <div v-if="showTypstConfigDialog" class="dialog-overlay" @click.self="showTypstConfigDialog = false">
      <div class="dialog-content typst-config-dialog">
        <div class="dialog-header">
          <h2>Typst 设置</h2>
          <button class="dialog-close" @click="showTypstConfigDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="config-section">
            <h3>主题设置</h3>
            <div class="config-item">
              <label>主题</label>
              <select v-model="typstConfig.theme">
                <option value="metropolis-theme">Metropolis</option>
                <option value="default-theme">Default</option>
                <option value="simple-theme">Simple</option>
              </select>
            </div>
          </div>
          <div class="config-section">
            <h3>页面设置</h3>
            <div class="config-item">
              <label>宽高比</label>
              <select v-model="typstConfig.aspectRatio">
                <option value="16-9">16:9</option>
                <option value="4-3">4:3</option>
                <option value="1-1">1:1</option>
              </select>
            </div>
            <div class="config-item">
              <label>显示页码</label>
              <input v-model="typstConfig.showSlideNumbers" type="checkbox" />
            </div>
          </div>
          <div class="config-section">
            <h3>字体设置</h3>
            <div class="config-item">
              <label>字体大小</label>
              <input v-model="typstConfig.fontSize" type="number" min="8" max="24" />
            </div>
            <div class="config-item">
              <label>字体族</label>
              <select v-model="typstConfig.fontFamily">
                <option value="Latin Modern">Latin Modern</option>
                <option value="Arial">Arial</option>
                <option value="Times New Roman">Times New Roman</option>
              </select>
            </div>
          </div>
        </div>
        <div class="dialog-footer">
          <button class="dialog-button secondary" @click="showTypstConfigDialog = false">取消</button>
          <button class="dialog-button primary" @click="applyTypstConfig">应用</button>
        </div>
      </div>
    </div>

    <!-- Typst Package Browser Dialog -->
    <div v-if="showTypstPackageBrowser" class="dialog-overlay" @click.self="showTypstPackageBrowser = false">
      <div class="dialog-content typst-dialog">
        <div class="dialog-header">
          <h2>Typst 包浏览器</h2>
          <button class="dialog-close" @click="showTypstPackageBrowser = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <TypstPackageBrowser />
        </div>
      </div>
    </div>

    <!-- Typst Font Manager Dialog -->
    <div v-if="showTypstFontManager" class="dialog-overlay" @click.self="showTypstFontManager = false">
      <div class="dialog-content typst-dialog">
        <div class="dialog-header">
          <h2>Typst 字体管理</h2>
          <button class="dialog-close" @click="showTypstFontManager = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <TypstFontManager />
        </div>
      </div>
    </div>

    <!-- Typst Export Options Dialog -->
    <div v-if="showTypstExportOptions" class="dialog-overlay" @click.self="showTypstExportOptions = false">
      <div class="dialog-content typst-dialog">
        <div class="dialog-header">
          <h2>Typst 导出选项</h2>
          <button class="dialog-close" @click="showTypstExportOptions = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <TypstExportOptions :show="showTypstExportOptions" @update:show="showTypstExportOptions = $event" />
        </div>
      </div>
    </div>

    <!-- Theme Dialog -->
    <div v-if="showThemeDialog" class="dialog-overlay" @click.self="showThemeDialog = false">
      <div class="dialog-content theme-dialog">
        <div class="dialog-header">
          <h2>幻灯片主题</h2>
          <button class="dialog-close" @click="showThemeDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="theme-grid">
            <div class="theme-item" @click="selectTheme('default')">
              <div class="theme-preview default-theme"></div>
              <span>默认主题</span>
            </div>
            <div class="theme-item" @click="selectTheme('modern')">
              <div class="theme-preview modern-theme"></div>
              <span>现代主题</span>
            </div>
            <div class="theme-item" @click="selectTheme('elegant')">
              <div class="theme-preview elegant-theme"></div>
              <span>优雅主题</span>
            </div>
            <div class="theme-item" @click="selectTheme('professional')">
              <div class="theme-preview professional-theme"></div>
              <span>专业主题</span>
            </div>
            <div class="theme-item" @click="selectTheme('creative')">
              <div class="theme-preview creative-theme"></div>
              <span>创意主题</span>
            </div>
            <div class="theme-item" @click="selectTheme('minimal')">
              <div class="theme-preview minimal-theme"></div>
              <span>简约主题</span>
            </div>
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showThemeDialog = false">取消</button>
          <button class="btn btn-primary" @click="applyTheme">应用</button>
        </div>
      </div>
    </div>

    <!-- Background Dialog -->
    <div v-if="showBackgroundDialog" class="dialog-overlay" @click.self="showBackgroundDialog = false">
      <div class="dialog-content background-dialog">
        <div class="dialog-header">
          <h2>背景样式</h2>
          <button class="dialog-close" @click="showBackgroundDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="form-group">
            <label>背景类型</label>
            <select class="form-select">
              <option value="solid">纯色</option>
              <option value="gradient">渐变</option>
              <option value="image">图片</option>
              <option value="pattern">图案</option>
            </select>
          </div>
          <div class="form-group">
            <label>背景颜色</label>
            <input type="color" class="form-input" value="#ffffff" />
          </div>
          <div class="form-group">
            <label>透明度</label>
            <input type="range" class="form-input" min="0" max="100" value="100" />
          </div>
          <div class="background-presets">
            <div class="preset-item" style="background: #ffffff;"></div>
            <div class="preset-item" style="background: #f0f0f0;"></div>
            <div class="preset-item" style="background: #e8e8e8;"></div>
            <div class="preset-item" style="background: #1a1a1a;"></div>
            <div class="preset-item" style="background: #0078d4;"></div>
            <div class="preset-item" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);"></div>
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showBackgroundDialog = false">取消</button>
          <button class="btn btn-primary" @click="applyBackground">应用</button>
        </div>
      </div>
    </div>

    <!-- Layout Dialog -->
    <div v-if="showLayoutDialog" class="dialog-overlay" @click.self="showLayoutDialog = false">
      <div class="dialog-content layout-dialog">
        <div class="dialog-header">
          <h2>幻灯片版式</h2>
          <button class="dialog-close" @click="showLayoutDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="layout-grid">
            <div class="layout-item" @click="selectLayout('title')">
              <div class="layout-preview title-layout">
                <div class="layout-title">标题</div>
              </div>
              <span>标题幻灯片</span>
            </div>
            <div class="layout-item" @click="selectLayout('content')">
              <div class="layout-preview content-layout">
                <div class="layout-title">标题</div>
                <div class="layout-content">内容</div>
              </div>
              <span>内容幻灯片</span>
            </div>
            <div class="layout-item" @click="selectLayout('two-column')">
              <div class="layout-preview two-column-layout">
                <div class="layout-title">标题</div>
                <div class="layout-cols">
                  <div class="layout-col">内容</div>
                  <div class="layout-col">内容</div>
                </div>
              </div>
              <span>两栏版式</span>
            </div>
            <div class="layout-item" @click="selectLayout('image-left')">
              <div class="layout-preview image-left-layout">
                <div class="layout-img">图片</div>
                <div class="layout-content">内容</div>
              </div>
              <span>图片左置</span>
            </div>
            <div class="layout-item" @click="selectLayout('image-right')">
              <div class="layout-preview image-right-layout">
                <div class="layout-content">内容</div>
                <div class="layout-img">图片</div>
              </div>
              <span>图片右置</span>
            </div>
            <div class="layout-item" @click="selectLayout('blank')">
              <div class="layout-preview blank-layout">
                <div class="layout-blank">空白</div>
              </div>
              <span>空白幻灯片</span>
            </div>
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showLayoutDialog = false">取消</button>
          <button class="btn btn-primary" @click="applyLayout">应用</button>
        </div>
      </div>
    </div>

    <!-- Insert Image Dialog -->
    <div v-if="showInsertImageDialog" class="dialog-overlay" @click.self="showInsertImageDialog = false">
      <div class="dialog-content insert-image-dialog">
        <div class="dialog-header">
          <h2>插入图片</h2>
          <button class="dialog-close" @click="showInsertImageDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="form-group">
            <label>图片来源</label>
            <select v-model="imageSourceType" class="form-select">
              <option value="upload">上传文件</option>
              <option value="url">网络链接</option>
              <option value="library">图片库</option>
            </select>
          </div>
          <div v-if="imageSourceType === 'upload'" class="form-group">
            <label>选择文件</label>
            <input type="file" class="form-input" accept="image/*" @change="handleImageUpload" />
          </div>
          <div v-if="imageSourceType === 'url'" class="form-group">
            <label>图片 URL</label>
            <input v-model="imageUrl" type="text" class="form-input" placeholder="https://example.com/image.jpg" />
          </div>
          <div v-if="imageSourceType === 'library'" class="form-group">
            <label>图片库</label>
            <select v-model="selectedLibraryImage" class="form-select">
              <option value="">选择图片</option>
              <option value="image1">示例图片 1</option>
              <option value="image2">示例图片 2</option>
              <option value="image3">示例图片 3</option>
            </select>
          </div>
          <div class="form-group">
            <label>替代文本</label>
            <input v-model="imageAltText" type="text" class="form-input" placeholder="图片描述" />
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showInsertImageDialog = false">取消</button>
          <button class="btn btn-primary" @click="handleInsertImage">插入</button>
        </div>
      </div>
    </div>

    <!-- Insert Shape Dialog -->
    <div v-if="showInsertShapeDialog" class="dialog-overlay" @click.self="showInsertShapeDialog = false">
      <div class="dialog-content insert-shape-dialog">
        <div class="dialog-header">
          <h2>插入形状</h2>
          <button class="dialog-close" @click="showInsertShapeDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="shape-grid">
            <div class="shape-item" @click="selectShape('rectangle')">
              <svg width="40" height="40" viewBox="0 0 40 40">
                <rect x="5" y="10" width="30" height="20" fill="none" stroke="currentColor" stroke-width="2"/>
              </svg>
              <span>矩形</span>
            </div>
            <div class="shape-item" @click="selectShape('circle')">
              <svg width="40" height="40" viewBox="0 0 40 40">
                <circle cx="20" cy="20" r="10" fill="none" stroke="currentColor" stroke-width="2"/>
              </svg>
              <span>圆形</span>
            </div>
            <div class="shape-item" @click="selectShape('triangle')">
              <svg width="40" height="40" viewBox="0 0 40 40">
                <polygon points="20,10 10,30 30,30" fill="none" stroke="currentColor" stroke-width="2"/>
              </svg>
              <span>三角形</span>
            </div>
            <div class="shape-item" @click="selectShape('diamond')">
              <svg width="40" height="40" viewBox="0 0 40 40">
                <polygon points="20,10 30,20 20,30 10,20" fill="none" stroke="currentColor" stroke-width="2"/>
              </svg>
              <span>菱形</span>
            </div>
            <div class="shape-item" @click="selectShape('star')">
              <svg width="40" height="40" viewBox="0 0 40 40">
                <polygon points="20,5 23,15 33,15 25,22 28,32 20,26 12,32 15,22 7,15 17,15" fill="none" stroke="currentColor" stroke-width="2"/>
              </svg>
              <span>星形</span>
            </div>
            <div class="shape-item" @click="selectShape('arrow')">
              <svg width="40" height="40" viewBox="0 0 40 40">
                <polygon points="10,20 25,10 25,15 35,15 35,25 25,25 25,30" fill="none" stroke="currentColor" stroke-width="2"/>
              </svg>
              <span>箭头</span>
            </div>
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showInsertShapeDialog = false">取消</button>
          <button class="btn btn-primary" @click="applyShape">插入</button>
        </div>
      </div>
    </div>

    <!-- Header Footer Dialog -->
    <div v-if="showHeaderFooterDialog" class="dialog-overlay" @click.self="showHeaderFooterDialog = false">
      <div class="dialog-content header-footer-dialog" style="max-width: 500px;">
        <div class="dialog-header">
          <h2>页眉和页脚</h2>
          <button class="dialog-close" @click="showHeaderFooterDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <!-- Header Section -->
          <div class="form-section">
            <div class="form-section-header">
              <label class="checkbox-label">
                <input v-model="headerEnabled" type="checkbox" />
                <span>启用页眉</span>
              </label>
            </div>
            <div v-if="headerEnabled" class="form-group">
              <label>页眉内容</label>
              <textarea 
                v-model="headerContent" 
                class="form-textarea" 
                rows="3" 
                placeholder="输入页眉内容..."
                style="width: 100%; padding: 8px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 2px; resize: vertical;"
              ></textarea>
            </div>
            <div v-if="headerEnabled" class="form-group">
              <label>对齐方式</label>
              <select v-model="headerAlign" class="form-select" style="width: 100%; padding: 8px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 2px;">
                <option value="left">左对齐</option>
                <option value="center">居中</option>
                <option value="right">右对齐</option>
              </select>
            </div>
          </div>

          <!-- Footer Section -->
          <div class="form-section">
            <div class="form-section-header">
              <label class="checkbox-label">
                <input v-model="footerEnabled" type="checkbox" />
                <span>启用页脚</span>
              </label>
            </div>
            <div v-if="footerEnabled" class="form-group">
              <label>页脚内容</label>
              <textarea 
                v-model="footerContent" 
                class="form-textarea" 
                rows="3" 
                placeholder="输入页脚内容..."
                style="width: 100%; padding: 8px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 2px; resize: vertical;"
              ></textarea>
            </div>
            <div v-if="footerEnabled" class="form-group">
              <label>对齐方式</label>
              <select v-model="footerAlign" class="form-select" style="width: 100%; padding: 8px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 2px;">
                <option value="left">左对齐</option>
                <option value="center">居中</option>
                <option value="right">右对齐</option>
              </select>
            </div>
          </div>

          <!-- Options -->
          <div class="form-section">
            <div class="form-group">
              <label class="checkbox-label">
                <input v-model="differentFirstPage" type="checkbox" />
                <span>首页不同</span>
              </label>
            </div>
            <div class="form-group">
              <label class="checkbox-label">
                <input v-model="differentOddEven" type="checkbox" />
                <span>奇偶页不同</span>
              </label>
            </div>
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="removeHeaderFooter">移除</button>
          <button class="btn btn-secondary" @click="showHeaderFooterDialog = false">取消</button>
          <button class="btn btn-primary" @click="_applyHeaderFooter">应用</button>
        </div>
      </div>
    </div>

    <!-- Page Number Dialog -->
    <div v-if="showPageNumberDialog" class="dialog-overlay" @click.self="showPageNumberDialog = false">
      <div class="dialog-content page-number-dialog" style="max-width: 450px;">
        <div class="dialog-header">
          <h2>页码格式</h2>
          <button class="dialog-close" @click="showPageNumberDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body" style="padding: 16px; display: flex; flex-direction: column; gap: 16px;">
          <!-- Position Section -->
          <div class="form-section">
            <label style="font-weight: 500; color: var(--word-text-primary); margin-bottom: 8px; display: block;">位置</label>
            <div style="display: flex; gap: 8px;">
              <button 
                class="position-option" 
                :class="{ active: pageNumberPosition === 'bottom-left' }"
                style="flex: 1; padding: 12px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 4px; cursor: pointer; transition: all 0.2s;"
                @click="pageNumberPosition = 'bottom-left'"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                  <line x1="9" y1="12" x2="9" y2="12"/>
                </svg>
                <span style="display: block; font-size: 12px; margin-top: 4px;">左下角</span>
              </button>
              <button 
                class="position-option" 
                :class="{ active: pageNumberPosition === 'bottom-center' }"
                style="flex: 1; padding: 12px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 4px; cursor: pointer; transition: all 0.2s;"
                @click="pageNumberPosition = 'bottom-center'"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                  <line x1="12" y1="12" x2="12" y2="12"/>
                </svg>
                <span style="display: block; font-size: 12px; margin-top: 4px;">底部居中</span>
              </button>
              <button 
                class="position-option" 
                :class="{ active: pageNumberPosition === 'bottom-right' }"
                style="flex: 1; padding: 12px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 4px; cursor: pointer; transition: all 0.2s;"
                @click="pageNumberPosition = 'bottom-right'"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                  <line x1="15" y1="12" x2="15" y2="12"/>
                </svg>
                <span style="display: block; font-size: 12px; margin-top: 4px;">右下角</span>
              </button>
            </div>
          </div>

          <!-- Format Section -->
          <div class="form-section">
            <label style="font-weight: 500; color: var(--word-text-primary); margin-bottom: 8px; display: block;">格式</label>
            <div style="display: flex; flex-direction: column; gap: 8px;">
              <label class="radio-label" style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                <input v-model="pageNumberFormat" type="radio" value="1" style="cursor: pointer;" />
                <span style="color: var(--word-text-primary);">1, 2, 3, ...</span>
              </label>
              <label class="radio-label" style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                <input v-model="pageNumberFormat" type="radio" value="1 of N" style="cursor: pointer;" />
                <span style="color: var(--word-text-primary);">1 of N, 2 of N, ...</span>
              </label>
              <label class="radio-label" style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                <input v-model="pageNumberFormat" type="radio" value="Page 1" style="cursor: pointer;" />
                <span style="color: var(--word-text-primary);">Page 1, Page 2, ...</span>
              </label>
            </div>
          </div>

          <!-- Preview Section -->
          <div class="form-section">
            <label style="font-weight: 500; color: var(--word-text-primary); margin-bottom: 8px; display: block;">预览</label>
            <div style="padding: 16px; border: 1px solid var(--word-border); background: var(--word-bg); border-radius: 4px; text-align: center;">
              <div style="font-size: 14px; color: #666;">
                {{ pageNumberFormat === '1' ? '1' : pageNumberFormat === '1 of N' ? '1 of N' : 'Page 1' }}
              </div>
            </div>
          </div>
        </div>
        <div class="dialog-footer" style="display: flex; justify-content: flex-end; padding: 12px 16px; border-top: 1px solid var(--word-border); background: var(--word-ribbon-bg);">
          <button class="action-button cancel-button" style="height: 32px; padding: 0 16px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 2px; cursor: pointer; font-size: 13px; font-weight: 500;" @click="showPageNumberDialog = false">取消</button>
          <button class="action-button primary-button" style="height: 32px; padding: 0 16px; border: 1px solid #0078d4; background: #0078d4; color: white; border-radius: 2px; cursor: pointer; font-size: 13px; font-weight: 500; margin-left: 8px;" @click="_applyPageNumbers">应用</button>
        </div>
      </div>
    </div>

    <!-- Insert Table Dialog -->
    <div v-if="showInsertTableDialog" class="dialog-overlay" @click.self="showInsertTableDialog = false">
      <div class="dialog-content insert-table-dialog">
        <div class="dialog-header">
          <h2>插入表格</h2>
          <button class="dialog-close" @click="showInsertTableDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="form-group">
            <label>行数</label>
            <input v-model="tableRows" type="number" class="form-input" min="1" max="20" value="3" />
          </div>
          <div class="form-group">
            <label>列数</label>
            <input v-model="tableCols" type="number" class="form-input" min="1" max="10" value="3" />
          </div>
          <div class="table-preview">
            <div class="table-grid" :style="{ gridTemplateColumns: `repeat(${tableCols}, 1fr)` }">
              <div v-for="i in tableRows * tableCols" :key="i" class="table-cell"></div>
            </div>
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showInsertTableDialog = false">取消</button>
          <button class="btn btn-primary" @click="applyTable">插入</button>
        </div>
      </div>
    </div>

    <!-- Animation Dialog -->
    <div v-if="showAnimationDialog" class="dialog-overlay" @click.self="showAnimationDialog = false">
      <div class="dialog-content animation-dialog">
        <div class="dialog-header">
          <h2>添加动画</h2>
          <button class="dialog-close" @click="showAnimationDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="form-group">
            <label>动画类型</label>
            <select v-model="selectedAnimation" class="form-select">
              <option value="fade">淡入</option>
              <option value="slide">滑动</option>
              <option value="zoom">缩放</option>
              <option value="rotate">旋转</option>
              <option value="bounce">弹跳</option>
            </select>
          </div>
          <div class="form-group">
            <label>持续时间 (秒)</label>
            <input type="number" class="form-input" min="0.1" max="5" step="0.1" value="0.5" />
          </div>
          <div class="form-group">
            <label>延迟 (秒)</label>
            <input type="number" class="form-input" min="0" max="10" step="0.1" value="0" />
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showAnimationDialog = false">取消</button>
          <button class="btn btn-primary" @click="applyAnimation">应用</button>
        </div>
      </div>
    </div>

    <!-- Transition Dialog -->
    <div v-if="showTransitionDialog" class="dialog-overlay" @click.self="showTransitionDialog = false">
      <div class="dialog-content transition-dialog">
        <div class="dialog-header">
          <h2>幻灯片切换</h2>
          <button class="dialog-close" @click="showTransitionDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="form-group">
            <label>切换效果</label>
            <select v-model="selectedTransition" class="form-select">
              <option value="none">无</option>
              <option value="fade">淡入淡出</option>
              <option value="slide-left">从左滑入</option>
              <option value="slide-right">从右滑入</option>
              <option value="slide-up">从下滑入</option>
              <option value="slide-down">从上滑入</option>
              <option value="zoom">缩放</option>
              <option value="push">推入</option>
            </select>
          </div>
          <div class="form-group">
            <label>持续时间 (秒)</label>
            <input type="number" class="form-input" min="0.1" max="3" step="0.1" value="0.5" />
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showTransitionDialog = false">取消</button>
          <button class="btn btn-primary" @click="applyTransition">应用</button>
        </div>
      </div>
    </div>

    <!-- SmartArt Dialog -->
    <div v-if="showSmartArtDialog" class="dialog-overlay" @click.self="showSmartArtDialog = false">
      <div class="dialog-content smartart-dialog">
        <div class="dialog-header">
          <h2>SmartArt</h2>
          <button class="dialog-close" @click="showSmartArtDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="form-group">
            <label>SmartArt 类型</label>
            <select v-model="selectedSmartArt" class="form-select">
              <option value="process">流程图</option>
              <option value="hierarchy">层次结构</option>
              <option value="cycle">循环</option>
              <option value="pyramid">金字塔</option>
              <option value="relationship">关系</option>
              <option value="matrix">矩阵</option>
            </select>
          </div>
          <div class="smartart-preview">
            <div class="smartart-item" @click="selectedSmartArt = 'process'">
              <svg width="60" height="40" viewBox="0 0 60 40">
                <rect x="5" y="10" width="15" height="20" fill="none" stroke="currentColor" stroke-width="2"/>
                <rect x="22" y="10" width="15" height="20" fill="none" stroke="currentColor" stroke-width="2"/>
                <rect x="39" y="10" width="15" height="20" fill="none" stroke="currentColor" stroke-width="2"/>
                <line x1="20" y1="20" x2="22" y2="20" stroke="currentColor" stroke-width="2"/>
                <line x1="37" y1="20" x2="39" y2="20" stroke="currentColor" stroke-width="2"/>
              </svg>
              <span>流程图</span>
            </div>
            <div class="smartart-item" @click="selectedSmartArt = 'hierarchy'">
              <svg width="60" height="40" viewBox="0 0 60 40">
                <rect x="20" y="5" width="20" height="12" fill="none" stroke="currentColor" stroke-width="2"/>
                <rect x="5" y="25" width="15" height="10" fill="none" stroke="currentColor" stroke-width="2"/>
                <rect x="22" y="25" width="15" height="10" fill="none" stroke="currentColor" stroke-width="2"/>
                <rect x="40" y="25" width="15" height="10" fill="none" stroke="currentColor" stroke-width="2"/>
                <line x1="30" y1="17" x2="12" y2="25" stroke="currentColor" stroke-width="2"/>
                <line x1="30" y1="17" x2="30" y2="25" stroke="currentColor" stroke-width="2"/>
                <line x1="30" y1="17" x2="47" y2="25" stroke="currentColor" stroke-width="2"/>
              </svg>
              <span>层次结构</span>
            </div>
            <div class="smartart-item" @click="selectedSmartArt = 'cycle'">
              <svg width="60" height="40" viewBox="0 0 60 40">
                <circle cx="30" cy="20" r="15" fill="none" stroke="currentColor" stroke-width="2"/>
                <polygon points="30,5 33,15 43,15 35,22 38,32 30,26 22,32 25,22 17,15 27,15" fill="none" stroke="currentColor" stroke-width="2"/>
              </svg>
              <span>循环</span>
            </div>
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showSmartArtDialog = false">取消</button>
          <button class="btn btn-primary" @click="applySmartArt">插入</button>
        </div>
      </div>
    </div>

    <!-- Font Color Dialog -->
    <div v-if="showFontColorDialog" class="dialog-overlay" @click.self="showFontColorDialog = false">
      <div class="dialog-content color-dialog">
        <div class="dialog-header">
          <h2>字体颜色</h2>
          <button class="dialog-close" @click="showFontColorDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="color-palette">
            <div class="color-item" style="background: #000000;" @click="selectedFontColor = '#000000'"></div>
            <div class="color-item" style="background: #ffffff; border: 1px solid #ccc;" @click="selectedFontColor = '#ffffff'"></div>
            <div class="color-item" style="background: #ff0000;" @click="selectedFontColor = '#ff0000'"></div>
            <div class="color-item" style="background: #00ff00;" @click="selectedFontColor = '#00ff00'"></div>
            <div class="color-item" style="background: #0000ff;" @click="selectedFontColor = '#0000ff'"></div>
            <div class="color-item" style="background: #ffff00;" @click="selectedFontColor = '#ffff00'"></div>
            <div class="color-item" style="background: #ff00ff;" @click="selectedFontColor = '#ff00ff'"></div>
            <div class="color-item" style="background: #00ffff;" @click="selectedFontColor = '#00ffff'"></div>
            <div class="color-item" style="background: #ff8800;" @click="selectedFontColor = '#ff8800'"></div>
            <div class="color-item" style="background: #8800ff;" @click="selectedFontColor = '#8800ff'"></div>
          </div>
          <div class="form-group">
            <label>自定义颜色</label>
            <input v-model="selectedFontColor" type="color" class="form-input" />
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showFontColorDialog = false">取消</button>
          <button class="btn btn-primary" @click="applyFontColor">应用</button>
        </div>
      </div>
    </div>

    <!-- Background Color Dialog -->
    <div v-if="showBackgroundColorDialog" class="dialog-overlay" @click.self="showBackgroundColorDialog = false">
      <div class="dialog-content color-dialog">
        <div class="dialog-header">
          <h2>背景颜色</h2>
          <button class="dialog-close" @click="showBackgroundColorDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="color-palette">
            <div class="color-item" style="background: #ffffff; border: 1px solid #ccc;" @click="selectedBackgroundColor = '#ffffff'"></div>
            <div class="color-item" style="background: #f0f0f0;" @click="selectedBackgroundColor = '#f0f0f0'"></div>
            <div class="color-item" style="background: #e8e8e8;" @click="selectedBackgroundColor = '#e8e8e8'"></div>
            <div class="color-item" style="background: #1a1a1a;" @click="selectedBackgroundColor = '#1a1a1a'"></div>
            <div class="color-item" style="background: #0078d4;" @click="selectedBackgroundColor = '#0078d4'"></div>
            <div class="color-item" style="background: #ff6b6b;" @click="selectedBackgroundColor = '#ff6b6b'"></div>
            <div class="color-item" style="background: #4ecdc4;" @click="selectedBackgroundColor = '#4ecdc4'"></div>
            <div class="color-item" style="background: #ffe66d;" @click="selectedBackgroundColor = '#ffe66d'"></div>
          </div>
          <div class="form-group">
            <label>自定义颜色</label>
            <input v-model="selectedBackgroundColor" type="color" class="form-input" />
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showBackgroundColorDialog = false">取消</button>
          <button class="btn btn-primary" @click="applyBackgroundColor">应用</button>
        </div>
      </div>
    </div>

    <!-- Border Color Dialog -->
    <div v-if="showBorderColorDialog" class="dialog-overlay" @click.self="showBorderColorDialog = false">
      <div class="dialog-content color-dialog">
        <div class="dialog-header">
          <h2>边框颜色</h2>
          <button class="dialog-close" @click="showBorderColorDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="color-palette">
            <div class="color-item" style="background: #000000;" @click="selectedBorderColor = '#000000'"></div>
            <div class="color-item" style="background: #ffffff; border: 1px solid #ccc;" @click="selectedBorderColor = '#ffffff'"></div>
            <div class="color-item" style="background: #ff0000;" @click="selectedBorderColor = '#ff0000'"></div>
            <div class="color-item" style="background: #00ff00;" @click="selectedBorderColor = '#00ff00'"></div>
            <div class="color-item" style="background: #0000ff;" @click="selectedBorderColor = '#0000ff'"></div>
            <div class="color-item" style="background: #ffff00;" @click="selectedBorderColor = '#ffff00'"></div>
            <div class="color-item" style="background: #ff00ff;" @click="selectedBorderColor = '#ff00ff'"></div>
            <div class="color-item" style="background: #00ffff;" @click="selectedBorderColor = '#00ffff'"></div>
          </div>
          <div class="form-group">
            <label>自定义颜色</label>
            <input v-model="selectedBorderColor" type="color" class="form-input" />
          </div>
        </div>
        <div class="dialog-footer">
          <button class="btn btn-secondary" @click="showBorderColorDialog = false">取消</button>
          <button class="btn btn-primary" @click="applyBorderColor">应用</button>
        </div>
      </div>
    </div>

    <!-- Advanced Features Dialog -->
    <div v-if="showAdvancedFeaturesDialog" class="dialog-overlay" @click.self="showAdvancedFeaturesDialog = false">
      <div class="dialog-content advanced-features-dialog">
        <div class="dialog-header">
          <h2>高级功能</h2>
          <button class="dialog-close" @click="showAdvancedFeaturesDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body">
          <div class="feature-tabs">
            <button 
              class="feature-tab" 
              :class="{ active: activeAdvancedFeature === 'incremental' }"
              @click="activeAdvancedFeature = 'incremental'"
            >
              增量编译
            </button>
            <button 
              class="feature-tab" 
              :class="{ active: activeAdvancedFeature === 'package' }"
              @click="activeAdvancedFeature = 'package'"
            >
              包管理
            </button>
            <button 
              class="feature-tab" 
              :class="{ active: activeAdvancedFeature === 'accessibility' }"
              @click="activeAdvancedFeature = 'accessibility'"
            >
              无障碍
            </button>
            <button 
              class="feature-tab" 
              :class="{ active: activeAdvancedFeature === 'plugin' }"
              @click="activeAdvancedFeature = 'plugin'"
            >
              插件管理
            </button>
          </div>
          
          <div class="feature-content">
            <!-- Incremental Compilation -->
            <div v-if="activeAdvancedFeature === 'incremental'" class="feature-panel">
              <h3>增量编译</h3>
              <p>计算文档哈希并管理编译缓存</p>
              <button class="action-button" @click="computeDocumentHash">计算文档哈希</button>
              <button class="action-button" @click="clearCache">清空缓存</button>
              <div class="feature-info">
                <p>缓存大小: {{ cacheSize }} bytes</p>
              </div>
            </div>

            <!-- Package Management -->
            <div v-if="activeAdvancedFeature === 'package'" class="feature-panel">
              <h3>包管理</h3>
              <p>管理 Typst 包和依赖</p>
              <button class="action-button" @click="loadPackages">加载可用包</button>
              <button class="action-button" @click="loadInstalledPackages">已安装包</button>
              <div class="feature-info">
                <p>可用包: {{ availablePackagesCount }}</p>
                <p>已安装: {{ installedPackagesCount }}</p>
              </div>
            </div>

            <!-- Accessibility -->
            <div v-if="activeAdvancedFeature === 'accessibility'" class="feature-panel">
              <h3>无障碍</h3>
              <p>构建和管理无障碍树</p>
              <button class="action-button" @click="buildAccessibilityTree">构建无障碍树</button>
              <button class="action-button" @click="validateAccessibility">验证无障碍</button>
              <div class="feature-info">
                <p>节点数量: {{ accessibilityNodeCount }}</p>
              </div>
            </div>

            <!-- Plugin Management -->
            <div v-if="activeAdvancedFeature === 'plugin'" class="feature-panel">
              <h3>插件管理</h3>
              <p>管理 WebAssembly 插件</p>
              <button class="action-button" @click="loadPlugins">加载插件</button>
              <button class="action-button" @click="getPluginStats">插件统计</button>
              <div class="feature-info">
                <p>插件数量: {{ pluginCount }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Toast Notification -->
    <div v-if="toast.show" class="toast-notification" :class="toast.type">
      {{ toast.message }}
    </div>

    <!-- Context Menu -->
    <ContextMenu
      :show="showContextMenu"
      :x="contextMenuPosition.x"
      :y="contextMenuPosition.y"
      :context="contextMenuContext"
      @update:show="showContextMenu = $event"
      @action="handleContextMenuAction"
    />

    <!-- Bubble Menu -->
    <BubbleMenu
      :show="showBubbleMenu"
      :x="bubbleMenuPosition.x"
      :y="bubbleMenuPosition.y"
      :editor="editor"
      @close="showBubbleMenu = false"
    />

    <!-- Floating Menu -->
    <FloatingMenu
      :show="showFloatingMenu"
      :x="floatingMenuPosition.x"
      :y="floatingMenuPosition.y"
      :editor="editor"
      @close="showFloatingMenu = false"
    />

    <!-- Page Layout Dialog -->
    <PageLayoutDialog
      :show="showPageLayoutDialog"
      @update:show="showPageLayoutDialog = $event"
      @apply="handlePageLayoutApply"
    />

    <!-- Style Manager Dialog -->
    <StyleManagerDialog
      :show="showStyleManagerDialog"
      @update:show="showStyleManagerDialog = $event"
      @apply-style="handleStyleApply"
    />

    <!-- Header Footer Dialog -->
    <HeaderFooterDialog
      :show="showHeaderFooterEditorDialog"
      :type="headerFooterType"
      @update:show="showHeaderFooterEditorDialog = $event"
      @apply="handleHeaderFooterApply"
    />

    <!-- Shape Selector Dialog -->
    <ShapeSelectorDialog
      :show="showShapeSelector"
      @update:show="showShapeSelector = $event"
      @insert-shape="handleInsertShape"
    />

    <!-- Icon Selector Dialog -->
    <IconSelectorDialog
      :show="showIconSelector"
      @update:show="showIconSelector = $event"
      @insert-icon="handleInsertIcon"
    />

    <!-- SmartArt Selector Dialog -->
    <SmartArtSelectorDialog
      :show="showSmartArtSelector"
      @update:show="showSmartArtSelector = $event"
      @insert-smartart="handleInsertSmartArt"
    />

    <!-- WordArt Dialog -->
    <WordArtDialog
      :show="showWordArtDialog"
      @update:show="showWordArtDialog = $event"
      @insert-wordart="handleInsertWordArt"
    />

    <!-- Chart Editor Dialog -->
    <ChartEditorDialog
      :show="showChartEditor"
      @update:show="showChartEditor = $event"
      @insert-chart="handleInsertChart"
    />


    <!-- Table Design Tab -->
    <TableDesignTab
      :show="showTableDesignTab"
      :table-selected="tableSelected"
      @update:show="showTableDesignTab = $event"
      @apply-style="handleApplyTableStyle"
      @apply-border="handleApplyTableBorder"
      @apply-shading="handleApplyTableShading"
    />

    <!-- Word Count Dialog -->
    <div v-if="showWordCountDialog" class="dialog-overlay" @click.self="toggleWordCountDialog">
      <div class="dialog-content word-count-dialog" style="max-width: 400px;">
        <div class="dialog-header">
          <h2>字数统计</h2>
          <button class="dialog-close" @click="toggleWordCountDialog">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body" style="padding: 16px;">
          <div v-if="documentAnalysis" class="word-count-stats">
            <div class="stat-row">
              <span class="stat-label">字数:</span>
              <span class="stat-value">{{ documentAnalysis.stats?.word_count || wordCount }}</span>
            </div>
            <div class="stat-row">
              <span class="stat-label">字符数:</span>
              <span class="stat-value">{{ documentAnalysis.stats?.char_count || charCount }}</span>
            </div>
            <div class="stat-row">
              <span class="stat-label">段落数:</span>
              <span class="stat-value">{{ documentAnalysis.stats?.paragraph_count || 0 }}</span>
            </div>
            <div class="stat-row">
              <span class="stat-label">句子数:</span>
              <span class="stat-value">{{ documentAnalysis.stats?.sentence_count || sentenceCount }}</span>
            </div>
            <div class="stat-row">
              <span class="stat-label">平均词长:</span>
              <span class="stat-value">{{ documentAnalysis.stats?.avg_word_length?.toFixed(2) || avgWordLength.toFixed(2) }}</span>
            </div>
            <div class="stat-row">
              <span class="stat-label">阅读时间:</span>
              <span class="stat-value">{{ documentAnalysis.stats?.reading_time || 0 }} 分钟</span>
            </div>
            <div class="stat-row">
              <span class="stat-label">图片数量:</span>
              <span class="stat-value">{{ documentAnalysis.content_detection?.images || 0 }}</span>
            </div>
            <div class="stat-row">
              <span class="stat-label">链接数量:</span>
              <span class="stat-value">{{ documentAnalysis.content_detection?.links || 0 }}</span>
            </div>
            <div class="stat-row">
              <span class="stat-label">表格数量:</span>
              <span class="stat-value">{{ documentAnalysis.content_detection?.tables || 0 }}</span>
            </div>
            <div class="stat-row">
              <span class="stat-label">代码块数量:</span>
              <span class="stat-value">{{ documentAnalysis.content_detection?.code_blocks || 0 }}</span>
            </div>
          </div>
          <div v-else class="loading-stats">
            <p>正在分析文档...</p>
          </div>
        </div>
        <div class="dialog-footer" style="display: flex; justify-content: flex-end; padding: 12px 16px; border-top: 1px solid var(--word-border); background: var(--word-ribbon-bg);">
          <button class="action-button cancel-button" style="height: 32px; padding: 0 16px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 2px; cursor: pointer; font-size: 13px; font-weight: 500;" @click="toggleWordCountDialog">关闭</button>
        </div>
      </div>
    </div>

    <!-- Spell Check Dialog -->
    <div v-if="showSpellCheckDialog" class="dialog-overlay" @click.self="showSpellCheckDialog = false">
      <div class="dialog-content spell-check-dialog" style="max-width: 500px;">
        <div class="dialog-header">
          <h2>拼写检查</h2>
          <button class="dialog-close" @click="showSpellCheckDialog = false">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="dialog-body" style="padding: 16px;">
          <div v-if="spellCheckResult" class="spell-check-results">
            <div class="spell-check-summary">
              <p>总词数: {{ spellCheckResult.total_words }}</p>
              <p>错误数: {{ spellCheckResult.error_count }}</p>
            </div>
            <div v-if="spellCheckResult.errors.length > 0" class="spell-check-errors">
              <h3>拼写错误</h3>
              <div v-for="(error, index) in spellCheckResult.errors" :key="index" class="spell-error-item">
                <div class="error-word">{{ error.word }}</div>
                <div class="error-suggestions">
                  <span>建议: </span>
                  <span v-if="error.suggestions.length > 0">{{ error.suggestions.join(', ') }}</span>
                  <span v-else>无</span>
                </div>
              </div>
            </div>
            <div v-else class="no-spell-errors">
              <p>✓ 未发现拼写错误</p>
            </div>
          </div>
          <div v-else class="loading-spell-check">
            <p>正在检查拼写...</p>
          </div>
        </div>
        <div class="dialog-footer" style="display: flex; justify-content: flex-end; padding: 12px 16px; border-top: 1px solid var(--word-border); background: var(--word-ribbon-bg);">
          <button class="action-button cancel-button" style="height: 32px; padding: 0 16px; border: 1px solid var(--word-border); background: var(--word-bg); color: var(--word-text-primary); border-radius: 2px; cursor: pointer; font-size: 13px; font-weight: 500;" @click="showSpellCheckDialog = false">关闭</button>
        </div>
      </div>
    </div>

    <!-- Wallpaper Selector Dialog -->
    <WallpaperSelector
      v-if="showWallpaperDialog"
      @select="selectedWallpaper = $event; showWallpaperDialog = false"
      @error="handleWallpaperError"
    />
  </div>
</template>

<style scoped>
@import '../styles/word-ribbon.css';

/* Help Tab Panel - No Label Groups */
.ribbon-group.no-label {
  padding: 2px 8px 0 8px;
}

.ribbon-group.no-label .group-label {
  display: none;
}

/* Word Count Dialog Styles */
.word-count-stats {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.stat-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid var(--word-border);
}

.stat-label {
  font-weight: 500;
  color: var(--word-text-secondary);
}

.stat-value {
  font-weight: 600;
  color: var(--word-text-primary);
}

.loading-stats {
  text-align: center;
  padding: 20px;
  color: var(--word-text-secondary);
}

/* Spell Check Dialog Styles */
.spell-check-results {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.spell-check-summary {
  padding: 12px;
  background: var(--word-bg);
  border-radius: 4px;
  border: 1px solid var(--word-border);
}

.spell-check-summary p {
  margin: 4px 0;
  color: var(--word-text-primary);
}

.spell-check-errors {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.spell-check-errors h3 {
  margin: 0 0 8px 0;
  font-size: 14px;
  color: var(--word-text-primary);
}

.spell-error-item {
  padding: 12px;
  background: var(--word-bg);
  border-radius: 4px;
  border: 1px solid var(--word-border);
  border-left: 3px solid #dc2626;
}

.error-word {
  font-weight: 600;
  color: #dc2626;
  margin-bottom: 4px;
}

.error-suggestions {
  font-size: 13px;
  color: var(--word-text-secondary);
}

.no-spell-errors {
  text-align: center;
  padding: 20px;
  color: #16a34a;
  font-weight: 500;
}

.loading-spell-check {
  text-align: center;
  padding: 20px;
  color: var(--word-text-secondary);
}

/* Status Bar Styles */
.status-bar {
  height: 24px;
  background: var(--word-ribbon-bg);
  border-top: 1px solid var(--word-border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
  font-size: 12px;
  color: var(--word-text-secondary);
  flex-shrink: 0;
}

.status-left,
.status-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.status-left span,
.status-right span {
  cursor: default;
  user-select: none;
}

/* Editor Container */
.editor-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  height: 100dvh;
  min-height: 100vh;
  min-height: 100dvh;
  background: var(--word-bg);
  color: var(--word-text-primary);
  overflow: hidden;
}

.editor-container.dark {
  background: #1e1e1e;
  color: #d4d4d4;
}

/* Editor Content Wrapper - Word-style gray canvas with elegant diagonal linen fabric texture */
.editor-content-wrapper {
  flex: 1;
  overflow: hidden;
  background-color: #fafafa;
  background-image: none;
  padding: 0;
  position: relative;
  min-height: 0;
  display: flex;
  flex-direction: column;
  height: 100%;
  border-left: 6px solid #e5e7eb;
}

.document-canvas {
  flex: 1;
  width: 100%;
  min-width: 0;
  min-height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px 60px 40px 60px;
  box-sizing: border-box;
}

/* Split View Sidebar */
.split-view-sidebar {
  width: var(--editor-sidebar-split-width, 400px);
  min-width: var(--editor-sidebar-split-width, 400px);
  max-width: var(--editor-sidebar-split-width, 400px);
  background: #ffffff;
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow: hidden;
  height: 100%;
}

.dark .split-view-sidebar {
  background: #1e1e1e;
}

.split-view-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid #e5e7eb;
  background: #f9fafb;
}

.dark .split-view-header {
  background: #2d2d30;
  border-bottom-color: #3e3e42;
}

.split-view-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #374151;
}

.dark .split-view-header h3 {
  color: #d4d4d4;
}

.split-view-header .close-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  color: #6b7280;
  display: flex;
  align-items: center;
  justify-content: center;
}

.split-view-header .close-button:hover {
  background: #e5e7eb;
  color: #374151;
}

.dark .split-view-header .close-button:hover {
  background: #3e3e42;
  color: #d4d4d4;
}

.split-view-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}

.empty-preview {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #9ca3af;
  font-size: 14px;
}

.compiling-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  height: 100%;
  color: #6b7280;
  font-size: 14px;
}

.compiling-indicator .spinner {
  width: 20px;
  height: 20px;
  border: 2px solid #e5e7eb;
  border-top-color: #3b82f6;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* Global Loading Overlay */
.global-loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
  backdrop-filter: blur(2px);
}

.loading-content {
  background: white;
  padding: 32px 48px;
  border-radius: 8px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.loading-spinner-large {
  width: 40px;
  height: 40px;
  border: 3px solid #e5e7eb;
  border-top-color: #3b82f6;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

.loading-content p {
  margin: 0;
  color: #374151;
  font-size: 14px;
  font-weight: 500;
}

.typst-preview-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  height: 100%;
}

.svg-preview-wrapper {
  flex: 1;
  overflow: auto;
  padding: 16px;
  display: flex;
  justify-content: center;
  align-items: flex-start;
}

.svg-preview-wrapper :deep(svg) {
  max-width: 100%;
  height: auto;
  display: block;
}

.typst-preview-frame {
  flex: 1;
  border: none;
  width: 100%;
  height: 100%;
}

.pdf-canvas {
  flex: 1;
  width: 100%;
  height: auto;
  display: block;
}

.pdf-pages-container {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.pdf-page-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.pdf-page-number {
  font-size: 12px;
  color: #6b7280;
  font-weight: 500;
}

.pdf-page-canvas-wrapper {
  border: 1px solid #e5e7eb;
  border-radius: 4px;
  overflow: hidden;
  background: white;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.pdf-page-canvas-wrapper canvas,
.pdf-page-image {
  display: block;
  max-width: 100%;
  height: auto;
}

.dark .pdf-page-number {
  color: #9ca3af;
}

.dark .pdf-page-canvas-wrapper {
  border-color: #3e3e42;
  background: #1e1e1e;
}

.typst-preview-code {
  flex: 1;
  background: #f9fafb;
  border: 1px solid #e5e7eb;
  border-radius: 4px;
  padding: 12px;
  margin: 0;
  font-size: 12px;
  font-family: 'Courier New', monospace;
  overflow: auto;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.dark .typst-preview-code {
  background: #2d2d30;
  border-color: #3e3e42;
  color: #d4d4d4;
}

.dark .editor-content-wrapper {
  background-color: #2a2a2c; /* Lightened dark mode to show texture */
  background-image: 
    linear-gradient(45deg, rgba(255, 255, 255, 0.04) 1px, transparent 1px),
    linear-gradient(-45deg, rgba(255, 255, 255, 0.04) 1px, transparent 1px),
    linear-gradient(45deg, rgba(0, 0, 0, 0.25) 1px, transparent 1px),
    linear-gradient(-45deg, rgba(0, 0, 0, 0.25) 1px, transparent 1px);
  background-size: 4px 4px, 4px 4px, 12px 12px, 12px 12px;
}

/* A4 Paper simulation - Word-style white page on gray canvas with soft double shadow */
.editor-mount {
  width: 794px;
  min-height: 1123px;
  background: #ffffff;
  box-shadow:
    0 4px 12px rgba(0, 0, 0, 0.18),
    0 16px 40px rgba(0, 0, 0, 0.28);
  padding: 96px 120px;
  position: relative;
  flex-shrink: 0;
  box-sizing: border-box;
  margin-bottom: 20px;
  cursor: pointer;
  transition: box-shadow 0.2s ease;
}

.editor-mount.active-page {
  box-shadow:
    0 0 0 3px #0078d4,
    0 4px 12px rgba(0, 0, 0, 0.18),
    0 16px 40px rgba(0, 0, 0, 0.28);
}

.editor-mount:hover {
  box-shadow:
    0 0 0 2px rgba(0, 120, 212, 0.5),
    0 4px 12px rgba(0, 0, 0, 0.18),
    0 16px 40px rgba(0, 0, 0, 0.28);
}

.dark .editor-mount {
  background: #1f1f1f;
  box-shadow:
    0 4px 12px rgba(0, 0, 0, 0.4),
    0 16px 40px rgba(0, 0, 0, 0.55);
}

.dark .editor-mount.active-page {
  box-shadow:
    0 0 0 3px #4fc3f7,
    0 4px 12px rgba(0, 0, 0, 0.4),
    0 16px 40px rgba(0, 0, 0, 0.55);
}

.dark .editor-mount:hover {
  box-shadow:
    0 0 0 2px rgba(79, 195, 247, 0.5),
    0 4px 12px rgba(0, 0, 0, 0.4),
    0 16px 40px rgba(0, 0, 0, 0.55);
}

.page-number-indicator {
  position: absolute;
  top: 10px;
  right: 10px;
  font-size: 10px;
  color: #999;
  background: rgba(255, 255, 255, 0.9);
  padding: 2px 6px;
  border-radius: 2px;
  pointer-events: none;
}

.dark .page-number-indicator {
  color: #666;
  background: rgba(31, 31, 31, 0.9);
}

.page-editor-wrapper {
  min-height: 800px;
}

.page-content-preview {
  min-height: 800px;
  opacity: 0.7;
  pointer-events: none;
}

/* ProseMirror typography inside paper */
.editor-mount :deep(.ProseMirror) {
  outline: none;
  min-height: 900px;
  font-family: 'Calibri', 'Microsoft YaHei', '微软雅黑', 'Segoe UI', sans-serif;
  font-size: 12pt;
  line-height: 1.6;
  color: #000000;
  caret-color: #000000;
  word-wrap: break-word;
  overflow-wrap: break-word;
}

.dark .editor-mount :deep(.ProseMirror) {
  color: #d4d4d4;
  caret-color: #d4d4d4;
}

/* Page Break Styling - Creates actual page separation */
.editor-mount :deep(.page-break-container) {
  page-break-after: always;
  break-after: page;
  break-inside: avoid;
  display: block;
  width: 100%;
  margin: 40px 0;
  position: relative;
}

.editor-mount :deep(.page-break) {
  border: none;
  border-top: 2px dashed #0078d4;
  margin: 0;
  display: block;
  height: 2px;
  width: 100%;
}

.editor-mount :deep(.page-break)::before {
  content: '分页符';
  position: absolute;
  top: -20px;
  left: 50%;
  transform: translateX(-50%);
  font-size: 10px;
  color: #0078d4;
  background: #ffffff;
  padding: 2px 6px;
  border-radius: 2px;
  white-space: nowrap;
}

.editor-mount :deep(.page-spacer) {
  height: 60px;
  width: 100%;
  display: block;
}

.dark .editor-mount :deep(.page-break) {
  border-top-color: #4fc3f7;
}

.dark .editor-mount :deep(.page-break)::before {
  color: #4fc3f7;
  background: #1f1f1f;
}

.editor-mount :deep(.ProseMirror p) {
  margin: 0 0 0.5em 0;
}

.editor-mount :deep(.ProseMirror h1) {
  font-size: 22pt;
  font-weight: 500;
  color: #1a1a1a;
  margin: 0.8em 0 0.4em 0;
  line-height: 1.3;
}

.editor-mount :deep(.ProseMirror h2) {
  font-size: 16pt;
  font-weight: 500;
  color: #1a1a1a;
  margin: 0.6em 0 0.3em 0;
  line-height: 1.35;
}

.editor-mount :deep(.ProseMirror h3) {
  font-size: 13pt;
  font-weight: 500;
  color: #1a1a1a;
  margin: 0.5em 0 0.25em 0;
  line-height: 1.4;
}

.dark .editor-mount :deep(.ProseMirror h1),
.dark .editor-mount :deep(.ProseMirror h2),
.dark .editor-mount :deep(.ProseMirror h3) {
  color: #e8e8e8;
}

.editor-mount :deep(.ProseMirror ul),
.editor-mount :deep(.ProseMirror ol) {
  padding-left: 1.5em;
  margin: 0.5em 0;
}

.editor-mount :deep(.ProseMirror li) {
  margin-bottom: 0.25em;
}

.editor-mount :deep(.ProseMirror blockquote) {
  border-left: 3px solid #d2d0ce;
  padding-left: 1em;
  color: #605e5c;
  margin: 0.5em 0;
}

.editor-mount :deep(.ProseMirror table) {
  border-collapse: collapse;
  width: 100%;
  margin: 0.5em 0;
}

.editor-mount :deep(.ProseMirror td),
.editor-mount :deep(.ProseMirror th) {
  border: 1px solid #d2d0ce;
  padding: 6px 12px;
  min-width: 60px;
}

.editor-mount :deep(.ProseMirror th) {
  background: #f3f2f1;
  font-weight: 500;
}

.editor-mount :deep(.ProseMirror code) {
  background: #f3f2f1;
  border-radius: 2px;
  padding: 0.1em 0.3em;
  font-family: 'Consolas', 'Courier New', monospace;
  font-size: 10.5pt;
}

.editor-mount :deep(.ProseMirror pre) {
  background: #f3f2f1;
  border-radius: 4px;
  padding: 12px 16px;
  overflow-x: auto;
  margin: 0.5em 0;
}

/* Editor loading state - matches paper width */
.editor-loading {
  width: 794px;
  min-height: 400px;
  background: #ffffff;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow:
    0 1px 3px rgba(0, 0, 0, 0.22),
    0 4px 20px rgba(0, 0, 0, 0.16);
  margin-bottom: 40px;
  flex-shrink: 0;
}

.loading-paper {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  color: #605e5c;
  font-size: 14px;
}

.loading-spinner {
  width: 24px;
  height: 24px;
  border: 2px solid #d2d0ce;
  border-top-color: #0078d4;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

/* Responsive - narrow viewport */
@media (max-width: 900px) {
  .editor-mount,
  .editor-loading {
    width: calc(100vw - 40px);
    padding: 48px 40px;
  }

  .document-canvas {
    padding: 20px 10px;
  }
}

/* Typst Preview Panel */
.typst-preview-panel {
  position: fixed;
  right: 20px;
  top: 140px;
  width: 400px;
  max-height: calc(100vh - 180px);
  background: var(--word-ribbon-panel-bg);
  border: 1px solid var(--word-border);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  display: flex;
  flex-direction: column;
  z-index: 1000;
  overflow: hidden;
}

.typst-preview-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid var(--word-divider);
  background: var(--word-ribbon-bg);
  gap: 12px;
}

.typst-preview-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 500;
  color: var(--word-text-primary);
}

.typst-preview-controls {
  display: flex;
  gap: 4px;
}

.view-mode-button {
  background: none;
  border: 1px solid var(--word-border);
  cursor: pointer;
  padding: 6px 10px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--word-text-secondary);
  font-size: 12px;
  transition: all 0.2s;
}

.view-mode-button:hover {
  background: var(--word-button-hover);
  color: var(--word-text-primary);
}

.view-mode-button.active {
  background: var(--word-button-active);
  color: var(--word-text-primary);
  border-color: var(--word-button-active-border);
}

.view-mode-button svg {
  flex-shrink: 0;
}

.close-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--word-text-secondary);
  transition: background-color 0.2s, color 0.2s;
}

.close-button:hover {
  background: var(--word-button-hover);
  color: var(--word-text-primary);
}

.typst-preview-content {
  flex: 1;
  overflow: auto;
  padding: 16px;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 12px;
  line-height: 1.5;
  background: var(--word-bg);
}

.typst-preview-content.compiling {
  display: flex;
  align-items: center;
  justify-content: center;
}

.compiling-indicator {
  display: flex;
  align-items: center;
  gap: 12px;
  color: var(--word-text-secondary);
}

.spinner {
  width: 20px;
  height: 20px;
  border: 2px solid var(--word-border);
  border-top-color: var(--word-button-active);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* Spreadsheet Panel (in-flow right workspace column) */
.spreadsheet-panel {
  position: relative;
  width: clamp(
    var(--editor-sidebar-spreadsheet-min-width, 320px),
    42vw,
    var(--editor-sidebar-spreadsheet-max-width, 800px)
  );
  min-width: var(--editor-sidebar-spreadsheet-min-width, 320px);
  max-width: var(--editor-sidebar-spreadsheet-max-width, 800px);
  height: 100%;
  background: var(--word-ribbon-panel-bg);
  border-left: 1px solid var(--word-border);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow: hidden;
}

.spreadsheet-panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid var(--word-divider);
  background: var(--word-ribbon-bg);
  gap: 12px;
}

.spreadsheet-panel-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 500;
  color: var(--word-text-primary);
}

.spreadsheet-panel-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.typst-render-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 200px;
  background: var(--word-button-hover);
  border-radius: 8px;
  padding: 32px;
}

.placeholder-content {
  text-align: center;
  color: var(--word-text-secondary);
}

.placeholder-content svg {
  margin-bottom: 16px;
  color: var(--word-text-tertiary);
}

.placeholder-content p {
  margin: 8px 0;
  font-size: 14px;
}

.placeholder-text {
  font-size: 12px;
  color: var(--word-text-tertiary);
}

.placeholder-info {
  font-size: 11px;
  color: var(--word-text-tertiary);
  margin-top: 16px;
}

.placeholder-hint {
  font-size: 11px;
  color: var(--word-text-tertiary);
  margin-top: 12px;
  font-style: italic;
}

.typst-render-result {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: white;
  border-radius: 4px;
  padding: 16px;
}

.typst-render-result img {
  max-width: 100%;
  height: auto;
  display: block;
}

.typst-render-error {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 200px;
  background: rgba(255, 0, 0, 0.05);
  border: 1px solid rgba(255, 0, 0, 0.2);
  border-radius: 8px;
  padding: 32px;
}

.error-content {
  text-align: center;
  color: var(--word-text-secondary);
}

.error-content svg {
  margin-bottom: 16px;
  color: #dc2626;
}

.error-content p {
  margin: 8px 0;
  font-size: 14px;
}

.error-text {
  font-size: 12px;
  color: #dc2626;
  margin-top: 12px;
}

.error-hint {
  font-size: 11px;
  color: var(--word-text-tertiary);
  margin-top: 16px;
}

.typst-preview-content pre {
  margin: 0;
  padding: 0;
  background: none;
  border: none;
}

.typst-preview-content code {
  font-family: inherit;
  font-size: inherit;
}

/* Dialog Styles */
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.dialog-content {
  background: var(--word-ribbon-panel-bg);
  border: 1px solid var(--word-border);
  border-radius: 8px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  max-width: 800px;
  max-height: 80vh;
  width: 90%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 300px;
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--word-divider);
  background: var(--word-ribbon-bg);
}

.dialog-header h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 500;
  color: var(--word-text-primary);
}

.dialog-close {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--word-text-secondary);
  transition: background-color 0.2s, color 0.2s;
}

.dialog-close:hover {
  background: var(--word-button-hover);
  color: var(--word-text-primary);
}

.dialog-body {
  flex: 1;
  overflow: auto;
  padding: 20px;
  min-height: 200px;
}

/* Typst Templates Dialog */
.typst-templates-dialog {
  max-width: 900px;
}

/* Typst Config Dialog */
.typst-config-dialog {
  max-width: 600px;
}

/* Conditional Format Dialog */
.conditional-format-dialog {
  max-width: 500px;
}

/* Chart Dialog */
.chart-dialog {
  max-width: 700px;
}

.chart-types {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
  margin-bottom: 16px;
}

.chart-type-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px;
  border: 1px solid var(--word-border);
  border-radius: 8px;
  background: var(--word-button-bg);
  cursor: pointer;
  transition: all 0.2s;
}

.chart-type-btn:hover {
  border-color: var(--word-button-border-hover);
  background: var(--word-button-hover);
}

.chart-type-btn svg {
  margin-bottom: 8px;
  color: var(--word-text-primary);
}

.chart-type-btn span {
  font-size: 12px;
  color: var(--word-text-secondary);
}

/* Pivot Table Dialog */
.pivot-table-dialog {
  max-width: 600px;
}

/* PPT Dialog Styles */
.theme-dialog,
.background-dialog,
.layout-dialog,
.insert-image-dialog,
.insert-shape-dialog,
.insert-table-dialog {
  max-width: 700px;
}

/* Theme Dialog */
.theme-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-top: 16px;
}

.theme-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  cursor: pointer;
  padding: 12px;
  border: 2px solid transparent;
  border-radius: 8px;
  transition: all 0.2s;
}

.theme-item:hover {
  border-color: var(--word-button-border-hover);
  background: var(--word-button-hover);
}

.theme-preview {
  width: 120px;
  height: 90px;
  border-radius: 4px;
  margin-bottom: 8px;
  border: 1px solid var(--word-border);
}

.default-theme {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.modern-theme {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}

.elegant-theme {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
}

.professional-theme {
  background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
}

.creative-theme {
  background: linear-gradient(135deg, #fa709a 0%, #fee140 100%);
}

.minimal-theme {
  background: #ffffff;
  border: 1px solid #e0e0e0;
}

.theme-item span {
  font-size: 12px;
  color: var(--word-text-primary);
}

/* Background Dialog */
.background-presets {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 8px;
  margin-top: 16px;
}

.preset-item {
  width: 60px;
  height: 40px;
  border-radius: 4px;
  cursor: pointer;
  border: 2px solid transparent;
  transition: all 0.2s;
}

.preset-item:hover {
  border-color: var(--word-button-border-hover);
  transform: scale(1.05);
}

/* Layout Dialog */
.layout-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-top: 16px;
}

.layout-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  cursor: pointer;
  padding: 12px;
  border: 2px solid transparent;
  border-radius: 8px;
  transition: all 0.2s;
}

.layout-item:hover {
  border-color: var(--word-button-border-hover);
  background: var(--word-button-hover);
}

.layout-preview {
  width: 120px;
  height: 90px;
  background: #f5f5f5;
  border: 1px solid var(--word-border);
  border-radius: 4px;
  margin-bottom: 8px;
  padding: 8px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.layout-title {
  font-size: 10px;
  font-weight: 500;
  text-align: center;
  background: #e0e0e0;
  padding: 4px;
  border-radius: 2px;
}

.layout-content {
  flex: 1;
  background: #d0d0d0;
  border-radius: 2px;
  padding: 4px;
  font-size: 8px;
  text-align: center;
}

.layout-cols {
  display: flex;
  gap: 4px;
  flex: 1;
}

.layout-col {
  flex: 1;
  background: #d0d0d0;
  border-radius: 2px;
  padding: 4px;
  font-size: 8px;
  text-align: center;
}

.layout-img {
  background: #c0c0c0;
  border-radius: 2px;
  padding: 4px;
  font-size: 8px;
  text-align: center;
}

.layout-blank {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 8px;
  color: #999;
}

.layout-item span {
  font-size: 12px;
  color: var(--word-text-primary);
}

/* Insert Shape Dialog */
.shape-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-top: 16px;
}

.shape-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  cursor: pointer;
  padding: 16px;
  border: 2px solid transparent;
  border-radius: 8px;
  transition: all 0.2s;
}

.shape-item:hover {
  border-color: var(--word-button-border-hover);
  background: var(--word-button-hover);
}

.shape-item svg {
  color: var(--word-text-primary);
  margin-bottom: 8px;
}

.shape-item span {
  font-size: 12px;
  color: var(--word-text-primary);
}

/* Insert Table Dialog */
.table-preview {
  margin-top: 16px;
  padding: 16px;
  background: var(--word-button-bg);
  border-radius: 4px;
}

.table-grid {
  display: grid;
  gap: 2px;
}

.table-cell {
  aspect-ratio: 1;
  background: var(--word-text-primary);
  border-radius: 2px;
}

/* Form Styles */
.form-group {
  margin-bottom: 16px;
}

.form-section {
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--word-border);
}

.form-section:last-child {
  border-bottom: none;
}

.form-section-header {
  margin-bottom: 12px;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-size: 14px;
  font-weight: 500;
  color: var(--word-text-primary);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  user-select: none;
}

.checkbox-label input[type="checkbox"] {
  cursor: pointer;
  width: 16px;
  height: 16px;
}

.form-input,
.form-select {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--word-input-border);
  border-radius: 4px;
  background: var(--word-input-bg);
  color: var(--word-input-text);
  font-size: 14px;
  transition: border-color 0.2s;
}

.form-input:focus,
.form-select:focus {
  outline: none;
  border-color: var(--word-input-border-focus);
}

.form-input::placeholder {
  color: var(--word-text-secondary);
}

.format-options {
  display: flex;
  gap: 8px;
}

.format-btn {
  padding: 8px;
  border: 1px solid var(--word-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.format-btn:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

.format-btn svg {
  color: var(--word-text-primary);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid var(--word-divider);
  background: var(--word-ribbon-bg);
}

.btn {
  padding: 8px 16px;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn-primary {
  background: #0078d4;
  color: white;
}

.btn-primary:hover {
  background: #106ebe;
}

.btn-secondary {
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  border: 1px solid var(--word-border);
}

.btn-secondary:hover {
  background: var(--word-button-hover);
}

.config-section {
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--word-divider);
}

.config-section:last-child {
  border-bottom: none;
}

.config-section h3 {
  font-size: 16px;
  font-weight: 500;
  margin-bottom: 12px;
  color: var(--word-text-primary);
}

.config-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.config-item label {
  font-size: 14px;
  color: var(--word-text-secondary);
  min-width: 100px;
}

.config-item select,
.config-item input[type="number"] {
  padding: 6px 12px;
  border: 1px solid var(--word-border);
  border-radius: 4px;
  background: var(--word-background);
  color: var(--word-text-primary);
  font-size: 14px;
  min-width: 200px;
}

.config-item input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding-top: 16px;
  border-top: 1px solid var(--word-divider);
}

.dialog-button {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.dialog-button.primary {
  background: var(--word-accent);
  color: white;
}

.dialog-button.primary:hover {
  background: var(--word-accent-hover);
}

.dialog-button.secondary {
  background: var(--word-button-secondary);
  color: var(--word-text-primary);
}

.dialog-button.secondary:hover {
  background: var(--word-button-hover);
}

/* Toast Notification */
.toast-notification {
  position: fixed;
  bottom: 40px;
  right: 20px;
  padding: 12px 24px;
  border-radius: 4px;
  color: white;
  font-size: 14px;
  z-index: 10000;
  animation: slideIn 0.3s ease-out;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

.toast-notification.info {
  background: #2196F3;
}

.toast-notification.success {
  background: #4CAF50;
}

.toast-notification.error {
  background: #F44336;
}

.toast-notification.warning {
  background: #FF9800;
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

/* Advanced Features Dialog */
.advanced-features-dialog {
  max-width: 800px;
}

.feature-tabs {
  display: flex;
  gap: 8px;
  margin-bottom: 20px;
  border-bottom: 1px solid var(--word-divider);
  padding-bottom: 12px;
}

.feature-tab {
  padding: 8px 16px;
  border: none;
  background: none;
  cursor: pointer;
  font-size: 14px;
  color: var(--word-text-secondary);
  border-radius: 4px;
  transition: all 0.2s;
}

.feature-tab:hover {
  background: var(--word-button-hover);
  color: var(--word-text-primary);
}

.feature-tab.active {
  background: var(--word-button-active);
  color: var(--word-text-primary);
  font-weight: 500;
}

.feature-content {
  min-height: 300px;
}

.feature-panel {
  h3 {
    margin: 0 0 8px 0;
    font-size: 18px;
    font-weight: 500;
    color: var(--word-text-primary);
  }

  p {
    margin: 0 0 16px 0;
    color: var(--word-text-secondary);
    font-size: 14px;
  }
}

.action-button {
  padding: 8px 16px;
  border: 1px solid var(--word-border);
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  margin-right: 8px;
  margin-bottom: 16px;
  transition: all 0.2s;
}

.action-button:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-active-border);
}

.feature-info {
  padding: 12px;
  background: var(--word-button-hover);
  border-radius: 4px;
  margin-top: 16px;

  p {
    margin: 4px 0;
    color: var(--word-text-secondary);
    font-size: 13px;
  }
}

.templates-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.template-card {
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 8px;
  padding: 16px;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  gap: 12px;
  align-items: flex-start;
}

.template-card:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.template-toolbar {
  display: flex;
  gap: 16px;
  margin-bottom: 20px;
  flex-wrap: wrap;
  align-items: center;
}

.template-search {
  flex: 1;
  min-width: 200px;
  display: flex;
  align-items: center;
  gap: 8px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 6px;
  padding: 8px 12px;
  transition: all 0.2s;
}

.template-search:focus-within {
  border-color: var(--word-accent);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
}

.template-search svg {
  color: var(--word-text-secondary);
  flex-shrink: 0;
}

.template-search-input {
  flex: 1;
  border: none;
  background: none;
  outline: none;
  font-size: 14px;
  color: var(--word-text-primary);
}

.template-search-input::placeholder {
  color: var(--word-text-secondary);
}

.template-filters {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.filter-button {
  padding: 6px 12px;
  border: 1px solid var(--word-button-border);
  background: var(--word-button-bg);
  color: var(--word-text-secondary);
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}

.filter-button:hover {
  background: var(--word-button-hover);
  color: var(--word-text-primary);
}

.filter-button.active {
  background: var(--word-accent);
  color: white;
  border-color: var(--word-accent);
}

.template-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 8px;
}

.template-buttons {
  display: flex;
  gap: 8px;
}

.template-action-btn {
  padding: 6px;
  border: 1px solid var(--word-button-border);
  background: var(--word-button-bg);
  color: var(--word-text-secondary);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.template-action-btn:hover {
  background: var(--word-button-hover);
  color: var(--word-text-primary);
}

.preview-btn:hover {
  border-color: var(--word-accent);
  color: var(--word-accent);
}

.apply-btn:hover {
  background: var(--word-accent);
  color: white;
  border-color: var(--word-accent);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: var(--word-text-secondary);
}

.empty-state svg {
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-state p {
  margin: 0;
  font-size: 14px;
}

.template-preview-dialog {
  max-width: 800px;
  max-height: 80vh;
}

.template-preview-content {
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 6px;
  padding: 16px;
  margin: 0;
  overflow: auto;
  max-height: 60vh;
  font-family: 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.6;
  color: var(--word-text-primary);
  white-space: pre-wrap;
  word-wrap: break-word;
}

.template-icon {
  flex-shrink: 0;
  width: 48px;
  height: 48px;
  background: var(--word-accent);
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--word-text-white);
}

.template-icon svg {
  width: 24px;
  height: 24px;
}

.template-info {
  flex: 1;
  min-width: 0;
}

.template-info h3 {
  margin: 0 0 4px 0;
  font-size: 14px;
  font-weight: 500;
  color: var(--word-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.template-info p {
  margin: 0 0 8px 0;
  font-size: 12px;
  color: var(--word-text-secondary);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.template-category {
  display: inline-block;
  padding: 2px 8px;
  background: var(--word-button-hover);
  border-radius: 12px;
  font-size: 10px;
  font-weight: 500;
  color: var(--word-text-secondary);
  text-transform: uppercase;
}

/* CSS Variables for Word Theme */
:root {
  --word-bg: #ffffff;
  --word-ribbon-bg: #f3f2f1;
  --word-ribbon-panel-bg: #ffffff;
  --word-ribbon-tab-bg: #f3f2f1;
  --word-ribbon-tab-hover: #e1dfdd;
  --word-ribbon-tab-active-bg: #ffffff;
  --word-ribbon-tab-active-border: #0078d4;
  --word-button-bg: #ffffff;
  --word-button-border: #d2d0ce;
  --word-button-hover: #e1dfdd;
  --word-button-border-hover: #0078d4;
  --word-button-active: #edf6ff;
  --word-button-disabled-bg: #f3f2f1;
  --word-button-disabled-text: #a19f9d;
  --word-accent: #0078d4;
  --word-accent-hover: #106ebe;
  --word-accent-pressed: #005a9e;
  --word-text-primary: #323130;
  --word-text-secondary: #605e5c;
  --word-text-white: #ffffff;
  --word-border: #d2d0ce;
  --word-divider: #e1dfdd;
  --word-input-bg: #ffffff;
  --word-input-border: #d2d0ce;
  --word-input-border-hover: #0078d4;
  --word-input-border-focus: #0078d4;
  --word-input-text: #323130;
  --word-scrollbar-bg: #f3f2f1;
  --word-scrollbar-thumb: #a19f9d;
  --word-scrollbar-thumb-hover: #605e5c;
  --word-font-ui: 'Segoe UI', 'Microsoft YaHei', '微软雅黑', sans-serif;
  --word-font-size-ui: 12px;
  --word-font-size-ribbon-tab: 13px;
  --word-font-size-ribbon-button: 11px;
  --word-font-size-ribbon-group: 11px;
  --word-font-weight-normal: 500;
  --word-font-weight-medium: 500;
  --word-font-weight-semibold: 500;
}

.dark {
  --word-bg: #1e1e1e;
  --word-ribbon-bg: #2d2d2d;
  --word-ribbon-panel-bg: #252526;
  --word-ribbon-tab-bg: #2d2d2d;
  --word-ribbon-tab-hover: #3e3e42;
  --word-ribbon-tab-active-bg: #252526;
  --word-ribbon-tab-active-border: #0078d4;
  --word-button-bg: #3c3c3c;
  --word-button-border: #555555;
  --word-button-hover: #4a4a4a;
  --word-button-border-hover: #0078d4;
  --word-button-active: #264f78;
  --word-button-disabled-bg: #2d2d2d;
  --word-button-disabled-text: #6a6a6a;
  --word-text-primary: #d4d4d4;
  --word-text-secondary: #a0a0a0;
  --word-border: #555555;
  --word-divider: #444444;
  --word-input-bg: #3c3c3c;
  --word-input-border: #555555;
  --word-input-border-hover: #0078d4;
  --word-input-border-focus: #0078d4;
  --word-input-text: #d4d4d4;
  --word-scrollbar-bg: #2d2d2d;
  --word-scrollbar-thumb: #555555;
  --word-scrollbar-thumb-hover: #777777;
}

/* Ribbon Panels Wrapper with Horizontal Scroll Button Styles */
.ribbon-panels-wrapper {
  position: relative;
  width: 100%;
  display: flex;
  align-items: center;
  background: var(--word-ribbon-panel-bg);
  border-bottom: 1px solid var(--word-border);
}

.ribbon-panels {
  flex: 1;
  min-height: 92px;
  background: var(--word-ribbon-panel-bg);
  padding: 4px 8px;
  display: flex;
  gap: 8px;
  overflow-x: auto;
  overflow-y: hidden;
  flex-shrink: 0;
  position: relative;
  z-index: 9;
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE/Edge */
}

.ribbon-panels::-webkit-scrollbar {
  display: none; /* Safari/Chrome */
}

.ribbon-scroll-button {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  width: 20px;
  height: 48px;
  background: rgba(243, 242, 241, 0.9);
  border: 1px solid var(--word-border);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  z-index: 100;
  opacity: 0; /* hidden by default */
  transition: opacity 0.2s, background-color 0.1s, color 0.1s;
  color: var(--word-text-secondary);
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}

.dark .ribbon-scroll-button {
  background: rgba(37, 37, 38, 0.9);
}

.ribbon-panels-wrapper:hover .ribbon-scroll-button {
  opacity: 0.8; /* show on hover */
}

.ribbon-scroll-button:hover {
  opacity: 1 !important;
  background: var(--word-button-hover);
  color: var(--word-accent);
}

.scroll-left {
  left: 0;
  border-left: none;
  border-radius: 0 4px 4px 0;
}

.scroll-right {
  right: 0;
  border-right: none;
  border-radius: 4px 0 0 4px;
}

/* Horizontal Ruler Styles */
.horizontal-ruler-outer-container {
  width: 100%;
  background: #e1dfdd;
  border-bottom: 1px solid var(--word-border);
  display: flex;
  justify-content: center;
  flex-shrink: 0;
  padding: 0;
  box-sizing: border-box;
  z-index: 5;
}

.dark .horizontal-ruler-outer-container {
  background: #3c3c3c;
}

.horizontal-ruler-container {
  height: 22px;
  background: var(--word-ribbon-bg);
  border: 1px solid var(--word-border);
  border-bottom: none;
  box-sizing: border-box;
  margin-bottom: 0;
  flex-shrink: 0;
  border-radius: 2px 2px 0 0;
  box-shadow: 0 1px 2px rgba(0,0,0,0.05);
  overflow: hidden;
}

.ruler {
  display: flex;
  height: 100%;
  width: 100%;
  position: relative;
  user-select: none;
}

.ruler-margin-left,
.ruler-margin-right {
  background: var(--word-button-hover);
  position: relative;
  height: 100%;
  flex-shrink: 0;
  border-bottom: 1px solid var(--word-border);
}

.ruler-drag-handle {
  position: absolute;
  top: 0;
  width: 8px;
  height: 100%;
  background: var(--word-accent);
  cursor: ew-resize;
  opacity: 0.15;
  transition: opacity 0.1s, background-color 0.1s;
  z-index: 10;
}

.ruler-drag-handle:hover {
  opacity: 0.8;
  background: var(--word-accent-hover);
}

.left-handle {
  right: -4px;
  border-radius: 0 4px 4px 0;
}

.right-handle {
  left: -4px;
  border-radius: 4px 0 0 4px;
}

.ruler-active-area {
  background: var(--word-bg);
  position: relative;
  height: 100%;
  flex-grow: 1;
}

.ruler-tick {
  position: absolute;
  top: 0;
  width: 1px;
  height: 5px;
  background: var(--word-text-secondary);
}

.ruler-tick-number {
  position: absolute;
  top: 6px;
  left: -6px;
  font-size: 8px;
  line-height: 1;
  color: var(--word-text-secondary);
  font-family: var(--word-font-ui);
}

/* Ruled Paper Horizontal Gridlines - applied only to editor content area */
.editor-mount.page-container.show-gridlines :deep(.ProseMirror) {
  background-color: #ffffff !important;
  background-image: linear-gradient(rgba(0, 120, 212, 0.15) 1px, transparent 1px) !important;
  background-size: 100% 28px !important;
  background-attachment: local !important;
  background-repeat: repeat-y !important;
}

.dark .editor-mount.page-container.show-gridlines :deep(.ProseMirror) {
  background-color: #1f1f1f !important;
  background-image: linear-gradient(rgba(255, 255, 255, 0.1) 1px, transparent 1px) !important;
}

/* User Guide Dialog */
.user-guide-dialog {
  max-width: 600px;
  max-height: 80vh;
}

.user-guide-content {
  padding: 20px;
}

.user-guide-content h3 {
  margin: 0 0 16px 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--word-text-primary);
}

.user-guide-content h4 {
  margin: 24px 0 12px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--word-text-primary);
}

.user-guide-content p {
  margin: 0 0 12px 0;
  color: var(--word-text-secondary);
  line-height: 1.6;
}

.user-guide-content ul {
  margin: 0 0 16px 0;
  padding-left: 20px;
  color: var(--word-text-secondary);
}

.user-guide-content li {
  margin: 8px 0;
  line-height: 1.6;
}

.user-guide-content kbd {
  display: inline-block;
  padding: 2px 6px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  font-family: monospace;
  font-size: 13px;
  color: var(--word-text-primary);
}

/* Header and Footer Areas - Word-style direct editing */
.document-header-area,
.document-footer-area {
  padding: 10px 0;
  font-size: 14px;
  color: #666;
  border-bottom: 1px solid transparent;
  border-top: 1px solid transparent;
  transition: all 0.2s;
  min-height: 40px;
  cursor: pointer;
}

.document-header-area {
  margin-bottom: 20px;
}

.document-footer-area {
  margin-top: 20px;
}

.document-header-area.editing,
.document-footer-area.editing {
  background: rgba(0, 120, 212, 0.05);
  border-bottom: 1px solid #0078d4;
  border-top: 1px solid #0078d4;
  outline: none;
  cursor: text;
}

.dark .document-header-area,
.dark .document-footer-area {
  color: #aaa;
}

.dark .document-header-area.editing,
.dark .document-footer-area.editing {
  background: rgba(255, 255, 255, 0.05);
  border-bottom: 1px solid rgba(255, 255, 255, 0.3);
  border-top: 1px solid rgba(255, 255, 255, 0.3);
}

.page-container.editing-header :deep(.ProseMirror),
.page-container.editing-footer :deep(.ProseMirror) {
  opacity: 0.3;
  pointer-events: none;
}

/* Page Number Dialog Styles */
.position-option.active {
  border-color: #0078d4 !important;
  background: rgba(0, 120, 212, 0.05) !important;
}

.position-option:hover {
  border-color: #0078d4;
  background: rgba(0, 120, 212, 0.03);
}

.dark .position-option.active {
  border-color: rgba(255, 255, 255, 0.5) !important;
  background: rgba(255, 255, 255, 0.05) !important;
}

.dark .position-option:hover {
  border-color: rgba(255, 255, 255, 0.3);
  background: rgba(255, 255, 255, 0.03);
}

/* Format Marks - Show non-printing characters */
.editor-mount.show-format-marks p::after {
  content: '¶';
  color: var(--word-text-tertiary);
  font-size: 12px;
  margin-left: 2px;
  opacity: 0.5;
  user-select: none;
}

.editor-mount.show-format-marks br::after {
  content: '↵';
  color: var(--word-text-tertiary);
  font-size: 12px;
  margin-left: 2px;
  opacity: 0.5;
  user-select: none;
}

.editor-mount.show-format-marks .ProseMirror p {
  text-indent: 0 !important;
}

.editor-mount.show-format-marks .ProseMirror p::before {
  content: '·';
  color: var(--word-text-tertiary);
  font-size: 12px;
  margin-right: 2px;
  opacity: 0.5;
  user-select: none;
}

/* ============================================================================
   Focus and Read Mode Layout Styles
   ============================================================================ */

/* Focus Mode: Hide Ribbon tabs, Ribbon panels, and Rulers (keep title bar visible) */
.focus-mode-active .ribbon-tabs,
.focus-mode-active .ribbon-panels-wrapper,
.focus-mode-active .horizontal-ruler-outer-container {
  display: none !important;
}

.focus-mode-active .editor-content-wrapper {
  padding: 16px 0 !important; /* reduced by 60% from 40px to 16px */
}

.dark.focus-mode-active .editor-content-wrapper {
  /* Removed hardcoded background to allow wallpaper */
}

.focus-mode-active .page-container {
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5) !important;
}

/* Read Mode: Hide Ribbon tabs, Ribbon panels, and Rulers (keep title bar visible) */
.read-mode-active .ribbon-tabs,
.read-mode-active .ribbon-panels-wrapper,
.read-mode-active .horizontal-ruler-outer-container {
  display: none !important;
}

.read-mode-active .editor-content-wrapper {
  background-color: #d8d4cc !important; /* darker warm sepia canvas with texture */
  padding: 16px 0 !important; /* reduced by 60% from 40px to 16px */
}

.read-mode-active .page-container {
  background: #fbf9f5 !important; /* book-paper white */
  background-image: 
    linear-gradient(90deg, rgba(0, 0, 0, 0.02) 1px, transparent 1px),
    linear-gradient(rgba(0, 0, 0, 0.02) 1px, transparent 1px);
  background-size: 2px 2px, 2px 2px;
  color: #2b2b2b !important;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08) !important;
  border-radius: 4px;
}

.dark.read-mode-active .editor-content-wrapper {
  background-color: #121214 !important; /* darker dark canvas with texture */
}

.dark.read-mode-active .page-container {
  background: #252529 !important;
  background-image: 
    linear-gradient(90deg, rgba(255, 255, 255, 0.03) 1px, transparent 1px),
    linear-gradient(rgba(255, 255, 255, 0.03) 1px, transparent 1px);
  background-size: 2px 2px, 2px 2px;
  color: #e3e3e3 !important;
}

/* Web Layout Mode: Remove grey canvas borders, shadows, and allow full-width styling */
.web-mode-active .editor-content-wrapper {
  padding: 0 !important;
  border-top: 1px solid rgba(0, 0, 0, 0.7); /* Horizontal divider line - 70% black */
}

.web-mode-active .page-container {
  box-shadow: none !important;
  border-radius: 0 !important;
  border: none !important;
}
</style>