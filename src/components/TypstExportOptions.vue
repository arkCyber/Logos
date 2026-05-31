<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { logger, LogCategory } from '../utils/logger';
import { invoke } from '@tauri-apps/api/core';

interface TypstExportOptionsProps {
  show?: boolean;
  typstContent?: string;
}

const props = withDefaults(defineProps<TypstExportOptionsProps>(), {
  show: false,
  typstContent: ''
});

const emit = defineEmits<{
  'update:show': [value: boolean];
  'export': [options: ExportOptions];
  'cancel': [];
}>();

// 导出格式
export type ExportFormat = 'pdf' | 'png' | 'html' | 'json' | 'yaml' | 'toml';

// 导出质量
export type ExportQuality = 'low' | 'medium' | 'high' | 'print';

// 页面范围
export type PageRange = 'all' | 'current' | 'custom';

// 导出选项接口
interface ExportOptions {
  format: ExportFormat;
  quality: ExportQuality;
  pageRange: PageRange;
  customPages: string;
  dpi: number;
  embedFonts: boolean;
  compress: boolean;
  includeMetadata: boolean;
  preserveFormatting: boolean;
}

// 默认导出选项
const defaultOptions: ExportOptions = {
  format: 'pdf',
  quality: 'high',
  pageRange: 'all',
  customPages: '1-10',
  dpi: 300,
  embedFonts: true,
  compress: true,
  includeMetadata: true,
  preserveFormatting: true
};

// 状态
const options = ref<ExportOptions>({ ...defaultOptions });
const isExporting = ref(false);
const exportError = ref<string | null>(null);

// 预设配置
const presets = ref([
  {
    name: '高质量PDF',
    options: {
      format: 'pdf' as ExportFormat,
      quality: 'print' as ExportQuality,
      pageRange: 'all' as PageRange,
      dpi: 600,
      embedFonts: true,
      compress: false
    }
  },
  {
    name: '标准PDF',
    options: {
      format: 'pdf' as ExportFormat,
      quality: 'high' as ExportQuality,
      pageRange: 'all' as PageRange,
      dpi: 300,
      embedFonts: true,
      compress: true
    }
  },
  {
    name: '网页HTML',
    options: {
      format: 'html' as ExportFormat,
      quality: 'high' as ExportQuality,
      pageRange: 'all' as PageRange,
      dpi: 96,
      embedFonts: false,
      compress: false
    }
  },
  {
    name: '快速预览PNG',
    options: {
      format: 'png' as ExportFormat,
      quality: 'medium' as ExportQuality,
      pageRange: 'current' as PageRange,
      dpi: 150,
      embedFonts: true,
      compress: true
    }
  }
]);

// 计算属性
const canExport = computed(() => {
  return props.typstContent && props.typstContent.trim().length > 0;
});

const showCustomPages = computed(() => {
  return options.value.pageRange === 'custom';
});

const formatOptions = computed(() => [
  { value: 'pdf' as ExportFormat, label: 'PDF文档', icon: '📄' },
  { value: 'png' as ExportFormat, label: 'PNG图片', icon: '🖼️' },
  { value: 'html' as ExportFormat, label: 'HTML网页', icon: '🌐' },
  { value: 'json' as ExportFormat, label: 'JSON数据', icon: '📊' },
  { value: 'yaml' as ExportFormat, label: 'YAML配置', icon: '⚙️' },
  { value: 'toml' as ExportFormat, label: 'TOML配置', icon: '📝' }
]);

const qualityOptions = computed(() => [
  { value: 'low', label: '低质量 (快速)', description: '适合快速预览' },
  { value: 'medium', label: '中等质量', description: '平衡质量和速度' },
  { value: 'high', label: '高质量', description: '适合一般用途' },
  { value: 'print', label: '印刷质量', description: '适合专业打印' }
]);

const dpiOptions = computed(() => [
  { value: 72, label: '72 DPI (屏幕)' },
  { value: 96, label: '96 DPI (网页)' },
  { value: 150, label: '150 DPI (预览)' },
  { value: 300, label: '300 DPI (标准)' },
  { value: 600, label: '600 DPI (印刷)' }
]);

// 方法
const applyPreset = (preset: typeof presets.value[0]) => {
  options.value = { ...options.value, ...preset.options };
  logger.info('Applied export preset', { preset: preset.name }, LogCategory.UI);
};

const validateCustomPages = (value: string): boolean => {
  if (!value) {
return false;
}
  // 验证页面范围格式：1-10, 5, 1,3,5-7
  const pattern = /^(\d+(-\d+)?)(,\s*\d+(-\d+)?)*$/;
  return pattern.test(value);
};

const handleExport = async () => {
  if (!canExport.value) {
    exportError.value = '没有可导出的内容';
    return;
  }

  if (options.value.pageRange === 'custom' && !validateCustomPages(options.value.customPages)) {
    exportError.value = '无效的页面范围格式';
    return;
  }

  isExporting.value = true;
  exportError.value = null;

  try {
    logger.info('Starting export', { format: options.value.format, quality: options.value.quality }, LogCategory.BUSINESS);

    emit('export', options.value);

    // 调用后端导出服务
    if (options.value.format === 'png') {
      const pngBytes = await invoke<number[]>('export_to_png', {
        code: props.typstContent,
        dpi: options.value.dpi
      });
      logger.info('PNG export completed', { size: pngBytes.length }, LogCategory.BUSINESS);
    } else if (options.value.format === 'pdf') {
      const pdfBytes = await invoke<number[]>('export_to_pdf', {
        code: props.typstContent
      });
      logger.info('PDF export completed', { size: pdfBytes.length }, LogCategory.BUSINESS);
    } else {
      logger.info('Export format not yet implemented', { format: options.value.format }, LogCategory.BUSINESS);
      exportError.value = `导出格式 ${options.value.format} 尚未实现`;
      return;
    }

    logger.info('Export completed', { format: options.value.format }, LogCategory.BUSINESS);

    // 关闭对话框
    emit('update:show', false);
  } catch (error) {
    logger.error('Export failed', error, LogCategory.BUSINESS);
    exportError.value = error instanceof Error ? error.message : '导出失败';
  } finally {
    isExporting.value = false;
  }
};

const handleCancel = () => {
  emit('cancel');
  emit('update:show', false);
};

const resetOptions = () => {
  options.value = { ...defaultOptions };
  logger.info('Reset export options', {}, LogCategory.UI);
};

// 监听show变化，重置选项
watch(() => props.show, (newShow) => {
  if (newShow) {
    resetOptions();
  }
});

// 辅助方法
const isCurrentPreset = (preset: typeof presets.value[0]): boolean => {
  return (
    options.value.format === preset.options.format &&
    options.value.quality === preset.options.quality &&
    options.value.pageRange === preset.options.pageRange &&
    options.value.dpi === preset.options.dpi &&
    options.value.embedFonts === preset.options.embedFonts &&
    options.value.compress === preset.options.compress
  );
};

const getFormatIcon = (format: ExportFormat): string => {
  const icons: Record<ExportFormat, string> = {
    pdf: '📄',
    png: '🖼️',
    html: '🌐',
    json: '📊',
    yaml: '⚙️',
    toml: '📝'
  };
  return icons[format] || '📄';
};
</script>

<template>
  <div v-if="show" class="typst-export-overlay" @click.self="handleCancel">
    <div class="typst-export-dialog">
      <!-- 对话框标题 -->
      <div class="dialog-header">
        <h2>导出选项</h2>
        <button class="btn-close" @click="handleCancel">✕</button>
      </div>

      <!-- 对话框内容 -->
      <div class="dialog-content">
        <!-- 预设选择 -->
        <div class="section">
          <h3>快速预设</h3>
          <div class="presets-grid">
            <button
              v-for="preset in presets"
              :key="preset.name"
              class="preset-btn"
              :class="{ active: isCurrentPreset(preset) }"
              @click="applyPreset(preset)"
            >
              <span class="preset-icon">{{ getFormatIcon(preset.options.format) }}</span>
              <span class="preset-name">{{ preset.name }}</span>
            </button>
          </div>
        </div>

        <!-- 格式选择 -->
        <div class="section">
          <h3>导出格式</h3>
          <div class="format-grid">
            <button
              v-for="format in formatOptions"
              :key="format.value"
              class="format-btn"
              :class="{ active: options.format === format.value }"
              @click="options.format = format.value"
            >
              <span class="format-icon">{{ format.icon }}</span>
              <span class="format-label">{{ format.label }}</span>
            </button>
          </div>
        </div>

        <!-- 质量选择 -->
        <div class="section">
          <h3>导出质量</h3>
          <div class="quality-options">
            <label
              v-for="quality in qualityOptions"
              :key="quality.value"
              class="quality-option"
            >
              <input
                v-model="options.quality"
                type="radio"
                :value="quality.value"
              />
              <div class="quality-info">
                <span class="quality-label">{{ quality.label }}</span>
                <span class="quality-description">{{ quality.description }}</span>
              </div>
            </label>
          </div>
        </div>

        <!-- 页面范围 -->
        <div class="section">
          <h3>页面范围</h3>
          <div class="page-range-options">
            <label class="radio-option">
              <input v-model="options.pageRange" type="radio" value="all" />
              <span>全部页面</span>
            </label>
            <label class="radio-option">
              <input v-model="options.pageRange" type="radio" value="current" />
              <span>当前页面</span>
            </label>
            <label class="radio-option">
              <input v-model="options.pageRange" type="radio" value="custom" />
              <span>自定义范围</span>
            </label>
          </div>
          
          <div v-if="showCustomPages" class="custom-pages-input">
            <input
              v-model="options.customPages"
              type="text"
              placeholder="例如: 1-10, 5, 1,3,5-7"
              class="input-field"
            />
            <span class="input-hint">格式: 1-10, 5, 1,3,5-7</span>
          </div>
        </div>

        <!-- DPI设置 -->
        <div class="section">
          <h3>DPI设置</h3>
          <select v-model="options.dpi" class="select-field">
            <option v-for="dpi in dpiOptions" :key="dpi.value" :value="dpi.value">
              {{ dpi.label }}
            </option>
          </select>
        </div>

        <!-- 高级选项 -->
        <div class="section">
          <h3>高级选项</h3>
          <div class="checkbox-options">
            <label class="checkbox-option">
              <input v-model="options.embedFonts" type="checkbox" />
              <span>嵌入字体</span>
            </label>
            <label class="checkbox-option">
              <input v-model="options.compress" type="checkbox" />
              <span>压缩输出</span>
            </label>
            <label class="checkbox-option">
              <input v-model="options.includeMetadata" type="checkbox" />
              <span>包含元数据</span>
            </label>
            <label class="checkbox-option">
              <input v-model="options.preserveFormatting" type="checkbox" />
              <span>保留格式</span>
            </label>
          </div>
        </div>

        <!-- 错误显示 -->
        <div v-if="exportError" class="error-message">
          {{ exportError }}
        </div>
      </div>

      <!-- 对话框底部 -->
      <div class="dialog-footer">
        <button class="btn-secondary" @click="resetOptions">重置</button>
        <button class="btn-secondary" @click="handleCancel">取消</button>
        <button
          class="btn-primary"
          :disabled="!canExport || isExporting"
          @click="handleExport"
        >
          <span v-if="isExporting">导出中...</span>
          <span v-else>导出</span>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.typst-export-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.typst-export-dialog {
  background: var(--bg-primary, #ffffff);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px;
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.dialog-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary, #333333);
}

.btn-close {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--text-secondary, #666666);
  padding: 4px 8px;
  border-radius: 4px;
  transition: background 0.2s;
}

.btn-close:hover {
  background: var(--bg-secondary, #f5f5f5);
}

.dialog-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.section {
  margin-bottom: 24px;
}

.section h3 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary, #333333);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.presets-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 8px;
}

.preset-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  padding: 12px;
  border: 2px solid var(--border-color, #e0e0e0);
  border-radius: 8px;
  background: var(--bg-primary, #ffffff);
  cursor: pointer;
  transition: all 0.2s;
}

.preset-btn:hover {
  border-color: var(--primary-color, #007bff);
  background: var(--accent-color, #e3f2fd);
}

.preset-btn.active {
  border-color: var(--primary-color, #007bff);
  background: var(--accent-color, #e3f2fd);
}

.preset-icon {
  font-size: 24px;
}

.preset-name {
  font-size: 12px;
  color: var(--text-primary, #333333);
}

.format-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 8px;
}

.format-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  padding: 12px;
  border: 2px solid var(--border-color, #e0e0e0);
  border-radius: 8px;
  background: var(--bg-primary, #ffffff);
  cursor: pointer;
  transition: all 0.2s;
}

.format-btn:hover {
  border-color: var(--primary-color, #007bff);
  background: var(--accent-color, #e3f2fd);
}

.format-btn.active {
  border-color: var(--primary-color, #007bff);
  background: var(--accent-color, #e3f2fd);
}

.format-icon {
  font-size: 20px;
}

.format-label {
  font-size: 12px;
  color: var(--text-primary, #333333);
}

.quality-options {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.quality-option {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px;
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.2s;
}

.quality-option:hover {
  background: var(--bg-secondary, #f5f5f5);
}

.quality-info {
  display: flex;
  flex-direction: column;
}

.quality-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary, #333333);
}

.quality-description {
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.page-range-options {
  display: flex;
  gap: 16px;
  margin-bottom: 12px;
}

.radio-option {
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
}

.custom-pages-input {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.input-field,
.select-field {
  padding: 8px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 4px;
  font-size: 14px;
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #333333);
}

.input-field:focus,
.select-field:focus {
  outline: none;
  border-color: var(--primary-color, #007bff);
}

.input-hint {
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.checkbox-options {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.checkbox-option {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.error-message {
  padding: 12px;
  background: var(--error-bg, #ffebee);
  border: 1px solid var(--error-color, #f44336);
  border-radius: 4px;
  color: var(--error-color, #f44336);
  font-size: 14px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 20px;
  border-top: 1px solid var(--border-color, #e0e0e0);
}

.btn-primary,
.btn-secondary {
  padding: 8px 16px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: var(--primary-color, #007bff);
  color: white;
  border-color: var(--primary-color, #007bff);
}

.btn-primary:hover:not(:disabled) {
  background: var(--primary-dark, #0056b3);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #333333);
}

.btn-secondary:hover {
  background: var(--bg-secondary, #f5f5f5);
}
</style>
