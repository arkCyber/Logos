import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { isTauri } from '@tauri-apps/api/core';

// TypeScript interfaces matching Rust structs
export interface DocumentStats {
  word_count: number;
  char_count: number;
  paragraph_count: number;
  line_count: number;
  reading_time: number;
  sentence_count: number;
  avg_word_length: number;
  avg_sentence_length: number;
}

export interface DocumentAnalysis {
  stats: DocumentStats;
  is_valid_html: boolean;
  validation_errors: string[];
  has_images: boolean;
  has_links: boolean;
  has_tables: boolean;
  has_code_blocks: boolean;
  content_detection?: {
    images: number;
    links: number;
    tables: number;
    code_blocks: number;
  };
}

export interface SpellingError {
  word: string;
  position: number;
  suggestions: string[];
}

export interface SpellCheckResult {
  errors: SpellingError[];
  total_words: number;
  error_count: number;
}

export interface SaveConfig {
  enabled: boolean;
  interval_ms: number;
  debounce_ms: number;
  max_versions: number;
}

export interface SaveResult {
  success: boolean;
  document_id: string;
  version: number;
  timestamp: number;
  error: string | null;
}

export interface SearchOptions {
  case_sensitive: boolean;
  whole_word: boolean;
  use_regex: boolean;
}

export interface MatchInfo {
  position: number;
  length: number;
  text: string;
}

export interface SearchResult {
  matches: MatchInfo[];
  total_count: number;
  current_index: number;
}

export interface ReplaceOptions {
  case_sensitive: boolean;
  whole_word: boolean;
  use_regex: boolean;
  replace_all: boolean;
}

export interface ReplaceResult {
  replaced_count: number;
  new_text: string;
  success: boolean;
}

export interface TocItem {
  id: string;
  level: number;
  text: string;
  children: TocItem[];
}

export interface TocResult {
  items: TocItem[];
  html: string;
}

export interface HeaderConfig {
  enabled: boolean;
  content: string;
  align: string;
  different_first_page: boolean;
}

export interface FooterConfig {
  enabled: boolean;
  content: string;
  align: string;
  different_first_page: boolean;
}

export interface PageNumberConfig {
  enabled: boolean;
  position: string;
  align: string;
  format: string;
}

export interface WatermarkConfig {
  enabled: boolean;
  text: string;
  opacity: number;
  rotation: number;
  color: string;
  font_size: number;
}

export enum ConversionType {
  FullToHalf = 'FullToHalf',
  HalfToFull = 'HalfToFull',
  Auto = 'Auto',
}

export interface ConversionConfig {
  conversion_type: ConversionType;
  preserve_newlines: boolean;
  preserve_spaces: boolean;
}

export interface CharStats {
  full_width: number;
  half_width: number;
  other: number;
  total: number;
}

// Performance monitoring interface
export interface PerformanceMetrics {
  operation: string;
  duration: number;
  timestamp: number;
  success: boolean;
}

// Validation constants
const MAX_HTML_SIZE = 10 * 1024 * 1024; // 10MB
const MAX_TEXT_LENGTH = 1 * 1024 * 1024; // 1MB
const MAX_WATERMARK_TEXT_LENGTH = 500;
const MAX_HEADER_FOOTER_CONTENT_LENGTH = 5000;

// Performance thresholds
const PERFORMANCE_WARNING_THRESHOLD_MS = 1000;
const PERFORMANCE_CRITICAL_THRESHOLD_MS = 5000;

/**
 * Validates HTML input size
 */
function validateHtmlSize(html: string): boolean {
  return html.length <= MAX_HTML_SIZE;
}

/**
 * Validates text input size
 */
function validateTextSize(text: string): boolean {
  return text.length <= MAX_TEXT_LENGTH;
}

/**
 * Validates watermark configuration
 */
function validateWatermarkConfig(config: WatermarkConfig): { valid: boolean; error: string | null } {
  if (config.text.length > MAX_WATERMARK_TEXT_LENGTH) {
    return { valid: false, error: `Watermark text exceeds maximum length of ${MAX_WATERMARK_TEXT_LENGTH} characters` };
  }
  if (config.opacity < 0 || config.opacity > 1) {
    return { valid: false, error: 'Opacity must be between 0 and 1' };
  }
  if (config.rotation < -180 || config.rotation > 180) {
    return { valid: false, error: 'Rotation must be between -180 and 180 degrees' };
  }
  if (config.font_size < 8 || config.font_size > 200) {
    return { valid: false, error: 'Font size must be between 8 and 200 pixels' };
  }
  if (!config.color.match(/^#[0-9A-Fa-f]{6}$/)) {
    return { valid: false, error: 'Color must be a valid hex color (e.g., #cccccc)' };
  }
  return { valid: true, error: null };
}

/**
 * Validates header/footer configuration
 */
function validateHeaderFooterConfig(config: HeaderConfig | FooterConfig): { valid: boolean; error: string | null } {
  if (config.content.length > MAX_HEADER_FOOTER_CONTENT_LENGTH) {
    return { valid: false, error: `Content exceeds maximum length of ${MAX_HEADER_FOOTER_CONTENT_LENGTH} characters` };
  }
  if (!['left', 'center', 'right'].includes(config.align)) {
    return { valid: false, error: 'Alignment must be one of: left, center, right' };
  }
  return { valid: true, error: null };
}

/**
 * Logs performance metrics
 */
function logPerformanceMetrics(operation: string, duration: number, success: boolean) {
  const metrics: PerformanceMetrics = {
    operation,
    duration,
    timestamp: Date.now(),
    success,
  };

  if (duration > PERFORMANCE_CRITICAL_THRESHOLD_MS) {
    console.error(`[PERFORMANCE CRITICAL] ${operation} took ${duration}ms`, metrics);
  } else if (duration > PERFORMANCE_WARNING_THRESHOLD_MS) {
    console.warn(`[PERFORMANCE WARNING] ${operation} took ${duration}ms`, metrics);
  } else {
    console.debug(`[PERFORMANCE] ${operation} took ${duration}ms`, metrics);
  }
}

export function useHybridServices() {
  const isTauriEnv = ref(false);
  const isLoading = ref(false);
  const error = ref<string | null>(null);
  const performanceMetrics = ref<PerformanceMetrics[]>([]);

  // Initialize Tauri environment check
  const init = async () => {
    isTauriEnv.value = await isTauri();
  };

  /**
   * Wrapper for Tauri invoke with performance monitoring
   */
  async function invokeWithPerformance<T>(
    command: string,
    args: Record<string, unknown>
  ): Promise<T | null> {
    const startTime = performance.now();
    
    try {
      const result = await invoke<T>(command, args);
      const duration = performance.now() - startTime;
      logPerformanceMetrics(command, duration, true);
      
      // Store metrics (keep last 100)
      performanceMetrics.value.push({
        operation: command,
        duration,
        timestamp: Date.now(),
        success: true,
      });
      if (performanceMetrics.value.length > 100) {
        performanceMetrics.value.shift();
      }
      
      return result;
    } catch (err) {
      const duration = performance.now() - startTime;
      logPerformanceMetrics(command, duration, false);
      
      // Store metrics
      performanceMetrics.value.push({
        operation: command,
        duration,
        timestamp: Date.now(),
        success: false,
      });
      if (performanceMetrics.value.length > 100) {
        performanceMetrics.value.shift();
      }
      
      throw err;
    }
  }

  // Document Analysis Service
  const analyzeDocument = async (html: string): Promise<DocumentAnalysis | null> => {
    if (!isTauriEnv.value) {
      console.warn('Document analysis only available in Tauri environment');
      return null;
    }

    if (!validateHtmlSize(html)) {
      error.value = `HTML input exceeds maximum size of ${MAX_HTML_SIZE} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<DocumentAnalysis>('analyze_document', { html });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Document analysis failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  // Spell Check Service
  const checkSpelling = async (text: string): Promise<SpellCheckResult | null> => {
    if (!isTauriEnv.value) {
      console.warn('Spell check only available in Tauri environment');
      return null;
    }

    if (!validateTextSize(text)) {
      error.value = `Text input exceeds maximum size of ${MAX_TEXT_LENGTH} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<SpellCheckResult>('check_spelling', { text });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Spell check failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  // Auto Save Service
  const autoSaveDocument = async (
    documentId: string,
    content: string
  ): Promise<SaveResult | null> => {
    if (!isTauriEnv.value) {
      console.warn('Auto save only available in Tauri environment');
      return null;
    }

    if (!documentId) {
      error.value = 'Document ID is required';
      console.error(error.value);
      return null;
    }

    if (!validateTextSize(content)) {
      error.value = `Content exceeds maximum size of ${MAX_TEXT_LENGTH} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<SaveResult>('auto_save_document', {
        documentId,
        content,
      });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Auto save failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  const getSavedDocument = async (documentId: string): Promise<string | null> => {
    if (!isTauriEnv.value) {
      console.warn('Get document only available in Tauri environment');
      return null;
    }

    if (!documentId) {
      error.value = 'Document ID is required';
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<string | null>('get_saved_document', { documentId });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Get document failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  const shouldAutoSave = async (documentId: string): Promise<boolean> => {
    if (!isTauriEnv.value) {
      return false;
    }

    try {
      const result = await invoke<boolean>('should_auto_save', { documentId });
      return result;
    } catch (err) {
      console.error('Should save check failed:', err);
      return false;
    }
  };

  const updateAutoSaveConfig = async (config: SaveConfig): Promise<boolean> => {
    if (!isTauriEnv.value) {
      return false;
    }

    try {
      await invoke('update_auto_save_config', { config });
      return true;
    } catch (err) {
      error.value = err as string;
      console.error('Update auto save config failed:', err);
      return false;
    }
  };

  const getAutoSaveConfig = async (): Promise<SaveConfig | null> => {
    if (!isTauriEnv.value) {
      return null;
    }

    try {
      const result = await invoke<SaveConfig>('get_auto_save_config');
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Get auto save config failed:', err);
      return null;
    }
  };

  // Search Text Service
  const searchText = async (
    text: string,
    pattern: string,
    options: SearchOptions,
    startPosition: number = 0
  ): Promise<SearchResult | null> => {
    if (!isTauriEnv.value) {
      console.warn('Search only available in Tauri environment');
      return null;
    }

    if (!pattern) {
      error.value = 'Search pattern is required';
      console.error(error.value);
      return null;
    }

    if (!validateTextSize(text)) {
      error.value = `Text input exceeds maximum size of ${MAX_TEXT_LENGTH} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<SearchResult>('search_text', {
        text,
        pattern,
        options,
        startPosition,
      });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Search failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  // Replace Text Service
  const replaceText = async (
    text: string,
    pattern: string,
    replacement: string,
    options: ReplaceOptions
  ): Promise<ReplaceResult | null> => {
    if (!isTauriEnv.value) {
      console.warn('Replace only available in Tauri environment');
      return null;
    }

    if (!pattern) {
      error.value = 'Search pattern is required';
      console.error(error.value);
      return null;
    }

    if (!validateTextSize(text)) {
      error.value = `Text input exceeds maximum size of ${MAX_TEXT_LENGTH} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<ReplaceResult>('replace_text', {
        text,
        pattern,
        replacement,
        options,
      });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Replace failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  // Generate TOC Service
  const generateToc = async (html: string): Promise<TocResult | null> => {
    if (!isTauriEnv.value) {
      console.warn('TOC generation only available in Tauri environment');
      return null;
    }

    if (!validateHtmlSize(html)) {
      error.value = `HTML input exceeds maximum size of ${MAX_HTML_SIZE} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<TocResult>('generate_toc', { html });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('TOC generation failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  // Insert TOC Service
  const insertToc = async (
    html: string,
    toc: TocResult,
    position: 'beginning' | 'after_first_heading' | 'end' = 'beginning'
  ): Promise<string | null> => {
    if (!isTauriEnv.value) {
      console.warn('TOC insertion only available in Tauri environment');
      return null;
    }

    if (!validateHtmlSize(html)) {
      error.value = `HTML input exceeds maximum size of ${MAX_HTML_SIZE} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<string>('insert_toc', { html, toc, position });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('TOC insertion failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  // Apply Header Footer Service
  const applyHeaderFooter = async (
    html: string,
    header: HeaderConfig,
    footer: FooterConfig
  ): Promise<string | null> => {
    if (!isTauriEnv.value) {
      console.warn('Header footer only available in Tauri environment');
      return null;
    }

    const headerValidation = validateHeaderFooterConfig(header);
    if (!headerValidation.valid) {
      error.value = headerValidation.error;
      console.error(error.value);
      return null;
    }

    const footerValidation = validateHeaderFooterConfig(footer);
    if (!footerValidation.valid) {
      error.value = footerValidation.error;
      console.error(error.value);
      return null;
    }

    if (!validateHtmlSize(html)) {
      error.value = `HTML input exceeds maximum size of ${MAX_HTML_SIZE} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<string>('apply_header_footer', { html, header, footer });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Apply header footer failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  // Remove Header Footer Service
  const removeHeaderFooter = async (html: string): Promise<string | null> => {
    if (!isTauriEnv.value) {
      console.warn('Header footer only available in Tauri environment');
      return null;
    }

    if (!validateHtmlSize(html)) {
      error.value = `HTML input exceeds maximum size of ${MAX_HTML_SIZE} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<string>('remove_header_footer', { html });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Remove header footer failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  // Apply Page Numbers Service
  const applyPageNumbers = async (
    html: string,
    config: PageNumberConfig
  ): Promise<string | null> => {
    if (!isTauriEnv.value) {
      console.warn('Page numbers only available in Tauri environment');
      return null;
    }

    if (!['header', 'footer'].includes(config.position)) {
      error.value = 'Position must be one of: header, footer';
      console.error(error.value);
      return null;
    }

    if (!['left', 'center', 'right'].includes(config.align)) {
      error.value = 'Alignment must be one of: left, center, right';
      console.error(error.value);
      return null;
    }

    if (!validateHtmlSize(html)) {
      error.value = `HTML input exceeds maximum size of ${MAX_HTML_SIZE} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<string>('apply_page_numbers', { html, config });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Apply page numbers failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  // Apply Watermark Service
  const applyWatermark = async (
    html: string,
    config: WatermarkConfig
  ): Promise<string | null> => {
    if (!isTauriEnv.value) {
      console.warn('Watermark only available in Tauri environment');
      return null;
    }

    const validation = validateWatermarkConfig(config);
    if (!validation.valid) {
      error.value = validation.error;
      console.error(error.value);
      return null;
    }

    if (!validateHtmlSize(html)) {
      error.value = `HTML input exceeds maximum size of ${MAX_HTML_SIZE} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<string>('apply_watermark', { html, config });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Apply watermark failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  // Remove Watermark Service
  const removeWatermark = async (html: string): Promise<string | null> => {
    if (!isTauriEnv.value) {
      console.warn('Watermark only available in Tauri environment');
      return null;
    }

    if (!validateHtmlSize(html)) {
      error.value = `HTML input exceeds maximum size of ${MAX_HTML_SIZE} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<string>('remove_watermark', { html });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Remove watermark failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  // Text Conversion Service
  const convertText = async (text: string, config: ConversionConfig): Promise<string | null> => {
    if (!isTauriEnv.value) {
      console.warn('Text conversion only available in Tauri environment');
      return null;
    }

    if (!validateTextSize(text)) {
      error.value = `Text input exceeds maximum size of ${MAX_TEXT_LENGTH} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<string>('convert_text', { text, config });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Text conversion failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  // Get Character Statistics Service
  const getCharStats = async (text: string): Promise<CharStats | null> => {
    if (!isTauriEnv.value) {
      console.warn('Character statistics only available in Tauri environment');
      return null;
    }

    if (!validateTextSize(text)) {
      error.value = `Text input exceeds maximum size of ${MAX_TEXT_LENGTH} bytes`;
      console.error(error.value);
      return null;
    }

    try {
      isLoading.value = true;
      error.value = null;
      const result = await invokeWithPerformance<CharStats>('get_char_stats', { text });
      return result;
    } catch (err) {
      error.value = err as string;
      console.error('Get character statistics failed:', err);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  /**
   * Gets performance metrics for monitoring
   */
  const getPerformanceMetrics = () => {
    return performanceMetrics.value;
  };

  /**
   * Clears performance metrics
   */
  const clearPerformanceMetrics = () => {
    performanceMetrics.value = [];
  };

  /**
   * Gets average performance for a specific operation
   */
  const getAveragePerformance = (operation: string): number | null => {
    const metrics = performanceMetrics.value.filter(m => m.operation === operation && m.success);
    if (metrics.length === 0) return null;
    const total = metrics.reduce((sum, m) => sum + m.duration, 0);
    return total / metrics.length;
  };

  return {
    isTauriEnv,
    isLoading,
    error,
    performanceMetrics,
    init,
    analyzeDocument,
    checkSpelling,
    autoSaveDocument,
    getSavedDocument,
    updateAutoSaveConfig,
    getAutoSaveConfig,
    searchText,
    replaceText,
    generateToc,
    insertToc,
    applyHeaderFooter,
    removeHeaderFooter,
    applyPageNumbers,
    applyWatermark,
    removeWatermark,
    convertText,
    getCharStats,
    getPerformanceMetrics,
    clearPerformanceMetrics,
    getAveragePerformance,
  };
}
