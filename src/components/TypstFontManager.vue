<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { logger, LogCategory } from '../utils/logger';
import { debounce } from '../utils/debounce';
import { auditLogger, AuditAction } from '../utils/auditLogger';

interface TypstFont {
  name: string;
  family: string;
  style: string;
  weight: number;
  path: string;
  size: number;
  isSystem: boolean;
  previewText: string;
  categories: string[];
}

const fonts = ref<TypstFont[]>([]);
const searchQuery = ref('');
const selectedCategory = ref('');
const isLoading = ref(false);
const showUploadDialog = ref(false);
const showPreviewDialog = ref(false);
const selectedFont = ref<TypstFont | null>(null);
const previewText = ref('The quick brown fox jumps over the lazy dog. 1234567890');
const errorMessage = ref('');
const successMessage = ref('');
const isUploading = ref(false);
const isDeleting = ref(false);
const showContextMenu = ref(false);
const contextMenuPosition = ref({ x: 0, y: 0 });
const contextMenuFont = ref<TypstFont | null>(null);

const categories = computed(() => {
  const categorySet = new Set<string>();
  fonts.value.forEach(font => {
    font.categories.forEach(category => categorySet.add(category));
  });
  return Array.from(categorySet).sort();
});

const filteredFonts = computed(() => {
  let result = fonts.value;

  if (selectedCategory.value) {
    result = result.filter(font => 
      font.categories.includes(selectedCategory.value)
    );
  }

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    result = result.filter(font =>
      font.name.toLowerCase().includes(query) ||
      font.family.toLowerCase().includes(query)
    );
  }

  return result;
});

const systemFonts = computed(() => {
  return fonts.value.filter(font => font.isSystem);
});

const userFonts = computed(() => {
  return fonts.value.filter(font => !font.isSystem);
});

onMounted(async () => {
  await loadFonts();
  setupKeyboardShortcuts();
});

// 键盘快捷键
function setupKeyboardShortcuts() {
  const handleKeyDown = (e: KeyboardEvent) => {
    // Ctrl/Cmd + F: 聚焦搜索框
    if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
      e.preventDefault();
      const searchInput = document.querySelector('.search-input') as HTMLInputElement;
      if (searchInput) {
        searchInput.focus();
      }
    }
    
    // Ctrl/Cmd + R: 刷新字体列表
    if ((e.ctrlKey || e.metaKey) && e.key === 'r') {
      e.preventDefault();
      loadFonts();
    }
    
    // Ctrl/Cmd + U: 打开上传对话框
    if ((e.ctrlKey || e.metaKey) && e.key === 'u') {
      e.preventDefault();
      showUploadDialog.value = true;
    }
    
    // Escape: 关闭对话框
    if (e.key === 'Escape') {
      if (showUploadDialog.value) {
        showUploadDialog.value = false;
      }
      if (showPreviewDialog.value) {
        showPreviewDialog.value = false;
      }
      if (showContextMenu.value) {
        closeContextMenu();
      }
      if (errorMessage.value) {
        errorMessage.value = '';
      }
      if (successMessage.value) {
        successMessage.value = '';
      }
    }
  };
  
  window.addEventListener('keydown', handleKeyDown);
  window.addEventListener('click', () => {
    if (showContextMenu.value) {
      closeContextMenu();
    }
  });
  
  // 清理函数
  return () => {
    window.removeEventListener('keydown', handleKeyDown);
  };
}

// 防抖搜索
const debouncedSearch = debounce((query: string) => {
  searchQuery.value = query;
}, 300);

// 验证字体数据
function validateFont(font: any): font is TypstFont {
  return (
    typeof font.name === 'string' &&
    font.name.length > 0 &&
    font.name.length <= 100 &&
    typeof font.family === 'string' &&
    font.family.length > 0 &&
    font.family.length <= 100 &&
    typeof font.style === 'string' &&
    font.style.length <= 50 &&
    typeof font.weight === 'number' &&
    font.weight >= 100 &&
    font.weight <= 900 &&
    typeof font.path === 'string' &&
    font.path.length > 0 &&
    font.path.length <= 500 &&
    typeof font.size === 'number' &&
    font.size > 0 &&
    font.size <= 100 * 1024 * 1024 && // 最大100MB
    typeof font.isSystem === 'boolean' &&
    typeof font.previewText === 'string' &&
    font.previewText.length <= 500 &&
    Array.isArray(font.categories) &&
    font.categories.every((c: any) => typeof c === 'string' && c.length <= 50)
  );
}

// XSS防护：转义HTML
function escapeHtml(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

// 验证文件名
function isValidFileName(name: string): boolean {
  const invalidChars = /[<>:"/\\|?*]/g;
  return !invalidChars.test(name) && name.length > 0 && name.length <= 255;
}

async function loadFonts() {
  isLoading.value = true;
  errorMessage.value = '';
  try {
    // TODO: 调用后端API获取字体列表
    // const rawData = await invoke('get_typst_fonts');
    // fonts.value = rawData.filter(validateFont);
    
    // 临时：模拟数据
    const mockData = [
      {
        name: 'Arial',
        family: 'Arial',
        style: 'Regular',
        weight: 400,
        path: '/System/Library/Fonts/Arial.ttf',
        size: 256000,
        isSystem: true,
        previewText: 'The quick brown fox jumps over the lazy dog.',
        categories: ['sans-serif', 'system']
      },
      {
        name: 'Times New Roman',
        family: 'Times New Roman',
        style: 'Regular',
        weight: 400,
        path: '/System/Library/Fonts/Times.ttf',
        size: 289000,
        isSystem: true,
        previewText: 'The quick brown fox jumps over the lazy dog.',
        categories: ['serif', 'system']
      },
      {
        name: 'Roboto',
        family: 'Roboto',
        style: 'Regular',
        weight: 400,
        path: '/Users/arksong/.local/share/fonts/Roboto.ttf',
        size: 312000,
        isSystem: false,
        previewText: 'The quick brown fox jumps over the lazy dog.',
        categories: ['sans-serif', 'user']
      },
      {
        name: 'Noto Sans CJK',
        family: 'Noto Sans CJK',
        style: 'Regular',
        weight: 400,
        path: '/Users/arksong/.local/share/fonts/NotoSansCJK.ttf',
        size: 15000000,
        isSystem: false,
        previewText: 'The quick brown fox jumps over the lazy dog. 快速的棕色狐狸跳过懒惰的狗。',
        categories: ['sans-serif', 'user', 'cjk']
      }
    ];
    
    // 验证数据
    fonts.value = mockData.filter(validateFont);
    
    logger.info('Typst fonts loaded', { count: fonts.value.length }, LogCategory.BUSINESS);
    auditLogger.log(AuditAction.REFRESH, { count: fonts.value.length }, true);
  } catch (error) {
    const errorMsg = '加载字体列表失败';
    errorMessage.value = errorMsg;
    logger.error(errorMsg, error as Error, LogCategory.SYSTEM);
    auditLogger.log(AuditAction.REFRESH, {}, false, errorMsg);
  } finally {
    isLoading.value = false;
  }
}

async function uploadFont(file: File) {
  // 验证文件名
  if (!isValidFileName(file.name)) {
    errorMessage.value = '文件名包含非法字符';
    auditLogger.log(AuditAction.FONT_UPLOAD, { name: file.name }, false, '文件名包含非法字符');
    return;
  }
  
  // 验证文件类型
  const validExtensions = ['.ttf', '.otf', '.woff', '.woff2'];
  const fileExtension = '.' + file.name.split('.').pop()?.toLowerCase();
  
  if (!validExtensions.includes(fileExtension)) {
    errorMessage.value = '不支持的字体格式。支持的格式: TTF, OTF, WOFF, WOFF2';
    auditLogger.log(AuditAction.FONT_UPLOAD, { name: file.name, extension: fileExtension }, false, '不支持的字体格式');
    return;
  }
  
  // 验证文件大小（最大50MB）
  const maxSize = 50 * 1024 * 1024;
  if (file.size > maxSize) {
    errorMessage.value = '字体文件过大。最大支持50MB';
    auditLogger.log(AuditAction.FONT_UPLOAD, { name: file.name, size: file.size }, false, '字体文件过大');
    return;
  }
  
  isUploading.value = true;
  errorMessage.value = '';
  
  try {
    // TODO: 调用后端API上传字体
    // await invoke('upload_typst_font', { file });
    
    successMessage.value = `字体 ${escapeHtml(file.name)} 上传成功`;
    logger.info('Font uploaded', { name: file.name, size: file.size }, LogCategory.BUSINESS);
    auditLogger.log(AuditAction.FONT_UPLOAD, { name: file.name, size: file.size }, true);
    
    // 重新加载字体列表
    await loadFonts();
    
    // 3秒后清除成功消息
    setTimeout(() => {
      successMessage.value = '';
    }, 3000);
  } catch (error) {
    const errorMsg = `上传字体 ${escapeHtml(file.name)} 失败`;
    errorMessage.value = errorMsg;
    logger.error(errorMsg, error as Error, LogCategory.SYSTEM);
    auditLogger.log(AuditAction.FONT_UPLOAD, { name: file.name }, false, errorMsg);
  } finally {
    isUploading.value = false;
  }
}

async function deleteFont(font: TypstFont) {
  if (isDeleting.value) {
return;
}
  
  if (!confirm(`确定要删除字体 "${escapeHtml(font.name)}" 吗？`)) {
    return;
  }
  
  errorMessage.value = '';
  isDeleting.value = true;
  
  try {
    // TODO: 调用后端API删除字体
    // await invoke('delete_typst_font', { name: font.name });
    
    successMessage.value = `字体 ${escapeHtml(font.name)} 删除成功`;
    logger.info('Font deleted', { name: font.name }, LogCategory.BUSINESS);
    auditLogger.log(AuditAction.FONT_DELETE, { name: font.name, family: font.family }, true);
    
    // 重新加载字体列表
    await loadFonts();
    
    // 3秒后清除成功消息
    setTimeout(() => {
      successMessage.value = '';
    }, 3000);
  } catch (error) {
    const errorMsg = `删除字体 ${escapeHtml(font.name)} 失败`;
    errorMessage.value = errorMsg;
    logger.error(errorMsg, error as Error, LogCategory.SYSTEM);
    auditLogger.log(AuditAction.FONT_DELETE, { name: font.name }, false, errorMsg);
  } finally {
    isDeleting.value = false;
  }
}

function showFontPreview(font: TypstFont) {
  selectedFont.value = font;
  showPreviewDialog.value = true;
  auditLogger.log(AuditAction.FONT_VIEW, { name: font.name, family: font.family }, true);
}

// 右键菜单处理
function handleContextMenu(event: MouseEvent, font: TypstFont) {
  event.preventDefault();
  contextMenuFont.value = font;
  
  // 边界检测，防止菜单超出屏幕
  const menuWidth = 180;
  const menuHeight = 200; // 估计高度
  const padding = 8;
  
  let x = event.clientX;
  let y = event.clientY;
  
  // 检查右边界
  if (x + menuWidth > window.innerWidth - padding) {
    x = window.innerWidth - menuWidth - padding;
  }
  
  // 检查下边界
  if (y + menuHeight > window.innerHeight - padding) {
    y = window.innerHeight - menuHeight - padding;
  }
  
  // 确保不超出左边界和上边界
  x = Math.max(padding, x);
  y = Math.max(padding, y);
  
  contextMenuPosition.value = { x, y };
  showContextMenu.value = true;
}

function closeContextMenu() {
  showContextMenu.value = false;
  contextMenuFont.value = null;
}

function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text).then(() => {
    successMessage.value = '已复制到剪贴板';
    setTimeout(() => {
      successMessage.value = '';
    }, 2000);
  }).catch((error) => {
    errorMessage.value = '复制失败';
    logger.error('Failed to copy to clipboard', error as Error, LogCategory.SYSTEM);
  });
}

function copyFontName() {
  if (contextMenuFont.value) {
    copyToClipboard(contextMenuFont.value.name);
  }
}

function copyFontFamily() {
  if (contextMenuFont.value) {
    copyToClipboard(contextMenuFont.value.family);
  }
}

function copyFontPath() {
  if (contextMenuFont.value) {
    copyToClipboard(contextMenuFont.value.path);
  }
}

function contextMenuPreview() {
  if (contextMenuFont.value) {
    showFontPreview(contextMenuFont.value);
    closeContextMenu();
  }
}

function contextMenuDelete() {
  if (contextMenuFont.value && !contextMenuFont.value.isSystem) {
    deleteFont(contextMenuFont.value);
    closeContextMenu();
  }
}

function formatSize(bytes: number): string {
  if (bytes < 1024) {
return bytes + ' B';
}
  if (bytes < 1024 * 1024) {
return (bytes / 1024).toFixed(1) + ' KB';
}
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
}

function getWeightLabel(weight: number): string {
  if (weight < 300) {
return 'Light';
}
  if (weight < 400) {
return 'Regular';
}
  if (weight < 500) {
return 'Medium';
}
  if (weight < 600) {
return 'SemiBold';
}
  if (weight < 700) {
return 'Bold';
}
  return 'ExtraBold';
}
</script>

<template>
  <div class="typst-font-manager">
    <div class="manager-header">
      <h2>Typst 字体管理</h2>
      <div class="header-actions">
        <div class="search-box">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索字体... (Ctrl+F)"
            class="search-input"
            maxlength="100"
            aria-label="搜索字体"
            role="searchbox"
          />
        </div>
        <button class="btn-primary" :disabled="isUploading" aria-label="上传字体" title="快捷键: Ctrl+U" @click="showUploadDialog = true">
          {{ isUploading ? '上传中...' : '上传字体' }}
        </button>
        <button class="btn-refresh" :disabled="isLoading" aria-label="刷新字体列表" title="快捷键: Ctrl+R" @click="loadFonts">
          {{ isLoading ? '加载中...' : '刷新' }}
        </button>
      </div>
      
      <div v-if="errorMessage" class="error-message">
        {{ errorMessage }}
        <button class="btn-close-error" @click="errorMessage = ''">✕</button>
      </div>
      
      <div v-if="successMessage" class="success-message">
        {{ successMessage }}
        <button class="btn-close-success" @click="successMessage = ''">✕</button>
      </div>
    </div>

    <div class="manager-filters">
      <div class="filter-group">
        <label>分类:</label>
        <select v-model="selectedCategory" class="filter-select">
          <option value="">全部</option>
          <option v-for="category in categories" :key="category" :value="category">
            {{ category }}
          </option>
        </select>
      </div>
      
      <div class="filter-group">
        <label>来源:</label>
        <button class="filter-btn" :class="{ active: selectedCategory === '' }" @click="selectedCategory = ''">
          全部
        </button>
        <button class="filter-btn" :class="{ active: selectedCategory === 'system' }" @click="selectedCategory = 'system'">
          系统字体 ({{ systemFonts.length }})
        </button>
        <button class="filter-btn" :class="{ active: selectedCategory === 'user' }" @click="selectedCategory = 'user'">
          用户字体 ({{ userFonts.length }})
        </button>
      </div>
    </div>

    <div class="fonts-container">
      <div v-if="isLoading" class="loading-state">
        <div class="spinner"></div>
        <p>加载字体列表...</p>
      </div>
      
      <div v-else-if="filteredFonts.length === 0" class="empty-state">
        <p>没有找到匹配的字体</p>
      </div>
      
      <div v-else class="fonts-grid">
        <div
          v-for="font in filteredFonts"
          :key="font.name"
          class="font-card"
          @contextmenu="handleContextMenu($event, font)"
        >
          <div class="font-header">
            <h3>{{ font.name }}</h3>
            <span v-if="font.isSystem" class="badge system">系统</span>
            <span v-else class="badge user">用户</span>
          </div>
          
          <div class="font-preview" :style="{ fontFamily: font.family }">
            {{ font.previewText }}
          </div>
          
          <div class="font-meta">
            <span class="family">{{ font.family }}</span>
            <span class="style">{{ font.style }}</span>
            <span class="weight">{{ getWeightLabel(font.weight) }}</span>
          </div>
          
          <div class="font-info">
            <span class="size" :title="`文件大小: ${formatSize(font.size)}`">
              📦 {{ formatSize(font.size) }}
            </span>
            <span class="path" :title="font.path">
              📁 {{ font.path.split('/').pop() }}
            </span>
          </div>
          
          <div class="font-tags">
            <span v-for="category in font.categories" :key="category" class="tag">
              {{ category }}
            </span>
          </div>
          
          <div class="font-actions">
            <button class="btn-preview" @click="showFontPreview(font)">
              预览
            </button>
            <button
              v-if="!font.isSystem"
              class="btn-delete"
              @click="deleteFont(font)"
            >
              删除
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 右键菜单 -->
    <div
      v-if="showContextMenu && contextMenuFont"
      class="context-menu"
      :style="{ left: contextMenuPosition.x + 'px', top: contextMenuPosition.y + 'px' }"
      role="menu"
      aria-label="字体操作菜单"
      @click.stop
    >
      <div class="context-menu-item" role="menuitem" tabindex="0" @click="contextMenuPreview">
        <span class="menu-icon">👁️</span>
        预览字体
      </div>
      <div v-if="!contextMenuFont.isSystem" class="context-menu-item" role="menuitem" tabindex="0" @click="contextMenuDelete">
        <span class="menu-icon">🗑️</span>
        删除字体
      </div>
      <div class="context-menu-divider"></div>
      <div class="context-menu-item" role="menuitem" tabindex="0" @click="copyFontName">
        <span class="menu-icon">📝</span>
        复制字体名称
      </div>
      <div class="context-menu-item" role="menuitem" tabindex="0" @click="copyFontFamily">
        <span class="menu-icon">👨‍👩‍👧‍👦</span>
        复制字体族
      </div>
      <div class="context-menu-item" role="menuitem" tabindex="0" @click="copyFontPath">
        <span class="menu-icon">📁</span>
        复制字体路径
      </div>
    </div>

    <!-- 上传字体对话框 -->
    <div v-if="showUploadDialog" class="dialog-overlay" role="dialog" aria-modal="true" aria-labelledby="upload-dialog-title" @click.self="showUploadDialog = false">
      <div class="dialog">
        <div class="dialog-header">
          <h2 id="upload-dialog-title">上传字体</h2>
          <button class="btn-close" aria-label="关闭对话框" @click="showUploadDialog = false">✕</button>
        </div>
        
        <div class="dialog-content">
          <div class="upload-area">
            <input
              id="font-upload"
              type="file"
              accept=".ttf,.otf,.woff,.woff2"
              class="file-input"
              @change="(e) => {
                const file = (e.target as HTMLInputElement).files?.[0];
                if (file) uploadFont(file);
              }"
            />
            <label for="font-upload" class="upload-label">
              <div class="upload-icon">📁</div>
              <p>点击或拖拽字体文件到此处</p>
              <p class="upload-hint">支持格式: TTF, OTF, WOFF, WOFF2</p>
            </label>
          </div>
        </div>
        
        <div class="dialog-footer">
          <button class="btn-secondary" @click="showUploadDialog = false">取消</button>
        </div>
      </div>
    </div>

    <!-- 字体预览对话框 -->
    <div v-if="showPreviewDialog && selectedFont" class="dialog-overlay" role="dialog" aria-modal="true" :aria-labelledby="`preview-dialog-title-${selectedFont.name}`" @click.self="showPreviewDialog = false">
      <div class="dialog dialog-large">
        <div class="dialog-header">
          <h2 :id="`preview-dialog-title-${selectedFont.name}`">{{ selectedFont.name }} 预览</h2>
          <button class="btn-close" aria-label="关闭对话框" @click="showPreviewDialog = false">✕</button>
        </div>
        
        <div class="dialog-content">
          <div class="preview-controls">
            <label>预览文本:</label>
            <input
              v-model="previewText"
              type="text"
              class="preview-input"
              placeholder="输入预览文本..."
            />
          </div>
          
          <div class="preview-area" :style="{ fontFamily: selectedFont.family }">
            <div class="preview-sample" :style="{ fontSize: '48px' }">
              {{ previewText }}
            </div>
            <div class="preview-sample" :style="{ fontSize: '32px' }">
              {{ previewText }}
            </div>
            <div class="preview-sample" :style="{ fontSize: '24px' }">
              {{ previewText }}
            </div>
            <div class="preview-sample" :style="{ fontSize: '16px' }">
              {{ previewText }}
            </div>
            <div class="preview-sample" :style="{ fontSize: '12px' }">
              {{ previewText }}
            </div>
          </div>
          
          <div class="preview-info">
            <div class="info-item">
              <label>字体族:</label>
              <span>{{ selectedFont.family }}</span>
            </div>
            <div class="info-item">
              <label>样式:</label>
              <span>{{ selectedFont.style }}</span>
            </div>
            <div class="info-item">
              <label>字重:</label>
              <span>{{ selectedFont.weight }} ({{ getWeightLabel(selectedFont.weight) }})</span>
            </div>
            <div class="info-item">
              <label>文件大小:</label>
              <span>{{ formatSize(selectedFont.size) }}</span>
            </div>
            <div class="info-item">
              <label>路径:</label>
              <span>{{ selectedFont.path }}</span>
            </div>
          </div>
        </div>
        
        <div class="dialog-footer">
          <button class="btn-secondary" aria-label="关闭对话框" @click="showPreviewDialog = false">关闭</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.typst-font-manager {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-primary, #ffffff);
}

.manager-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px;
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.manager-header h2 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary, #333333);
}

.header-actions {
  display: flex;
  gap: 12px;
  align-items: center;
  flex-wrap: wrap;
}

.error-message,
.success-message {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-radius: 6px;
  margin-top: 12px;
  font-size: 14px;
}

.error-message {
  background: var(--error-bg, #ffebee);
  color: var(--error-color, #f44336);
  border: 1px solid var(--error-color, #f44336);
}

.success-message {
  background: var(--success-bg, #e8f5e9);
  color: var(--success-color, #4caf50);
  border: 1px solid var(--success-color, #4caf50);
}

.btn-close-error,
.btn-close-success {
  background: none;
  border: none;
  font-size: 18px;
  cursor: pointer;
  padding: 0 4px;
  margin-left: 8px;
}

.search-box {
  position: relative;
}

.search-input {
  padding: 8px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 6px;
  font-size: 14px;
  width: 250px;
}

.btn-primary,
.btn-refresh {
  padding: 8px 16px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: var(--primary-color, #007bff);
  color: white;
  border-color: var(--primary-color, #007bff);
}

.btn-primary:hover {
  background: var(--primary-dark, #0056b3);
}

.btn-refresh {
  background: var(--bg-primary, #ffffff);
}

.btn-refresh:hover:not(:disabled) {
  background: var(--bg-secondary, #f5f5f5);
}

.btn-refresh:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.manager-filters {
  display: flex;
  gap: 20px;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color, #e0e0e0);
  align-items: center;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.filter-group label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary, #666666);
}

.filter-select {
  padding: 6px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 4px;
  font-size: 14px;
  background: var(--bg-primary, #ffffff);
}

.filter-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 4px;
  background: var(--bg-primary, #ffffff);
  cursor: pointer;
  transition: all 0.2s;
}

.filter-btn:hover {
  background: var(--bg-secondary, #f5f5f5);
}

.filter-btn.active {
  background: var(--primary-color, #007bff);
  color: white;
  border-color: var(--primary-color, #007bff);
}

.fonts-container {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.loading-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-secondary, #666666);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--border-color, #e0e0e0);
  border-top-color: var(--primary-color, #007bff);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 16px;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.fonts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 16px;
}

.font-card {
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 8px;
  padding: 16px;
  transition: transform 0.2s, box-shadow 0.2s;
}

.font-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.font-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.font-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #333333);
}

.badge {
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 500;
}

.badge.system {
  background: var(--info-bg, #e3f2fd);
  color: var(--info-color, #2196f3);
}

.badge.user {
  background: var(--success-bg, #e8f5e9);
  color: var(--success-color, #4caf50);
}

.font-preview {
  padding: 16px;
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 6px;
  margin-bottom: 12px;
  font-size: 18px;
  line-height: 1.4;
  color: var(--text-primary, #333333);
  min-height: 60px;
}

.font-meta {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: var(--text-secondary, #666666);
  margin-bottom: 12px;
}

.font-info {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: var(--text-secondary, #666666);
  margin-bottom: 12px;
}

.font-tags {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
  margin-bottom: 12px;
}

.tag {
  padding: 2px 8px;
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 12px;
  font-size: 11px;
  color: var(--text-secondary, #666666);
}

.font-actions {
  display: flex;
  gap: 8px;
}

.btn-preview,
.btn-delete {
  flex: 1;
  padding: 6px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 4px;
  background: var(--bg-primary, #ffffff);
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}

.btn-preview:hover {
  background: var(--primary-color, #007bff);
  color: white;
  border-color: var(--primary-color, #007bff);
}

.btn-delete:hover {
  background: var(--error-color, #f44336);
  color: white;
  border-color: var(--error-color, #f44336);
}

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
  z-index: 1000;
}

.dialog {
  background: var(--bg-primary, #ffffff);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  width: 90%;
  max-width: 500px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
}

.dialog-large {
  max-width: 800px;
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
}

.dialog-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.upload-area {
  border: 2px dashed var(--border-color, #d0d0d0);
  border-radius: 8px;
  padding: 40px;
  text-align: center;
  transition: border-color 0.2s;
}

.upload-area:hover {
  border-color: var(--primary-color, #007bff);
}

.file-input {
  display: none;
}

.upload-label {
  cursor: pointer;
  display: block;
}

.upload-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.upload-label p {
  margin: 8px 0;
  color: var(--text-primary, #333333);
}

.upload-hint {
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.preview-controls {
  display: flex;
  gap: 12px;
  align-items: center;
  margin-bottom: 20px;
}

.preview-controls label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary, #666666);
}

.preview-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 6px;
  font-size: 14px;
}

.preview-area {
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 8px;
  padding: 24px;
  margin-bottom: 20px;
}

.preview-sample {
  margin-bottom: 16px;
  color: var(--text-primary, #333333);
}

.preview-info {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.info-item label {
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.info-item span {
  font-size: 14px;
  color: var(--text-primary, #333333);
  word-break: break-all;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 20px;
  border-top: 1px solid var(--border-color, #e0e0e0);
}

.btn-secondary {
  padding: 8px 16px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 6px;
  background: var(--bg-primary, #ffffff);
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background: var(--bg-secondary, #f5f5f5);
}

.context-menu {
  position: fixed;
  background: var(--bg-primary, #ffffff);
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
  z-index: 2000;
  min-width: 180px;
  padding: 4px 0;
  animation: contextMenuFadeIn 0.15s ease-out;
}

@keyframes contextMenuFadeIn {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  cursor: pointer;
  transition: background 0.2s;
  font-size: 14px;
  color: var(--text-primary, #333333);
}

.context-menu-item:hover {
  background: var(--bg-secondary, #f5f5f5);
}

.context-menu-item:focus {
  outline: 2px solid var(--primary-color, #007bff);
  outline-offset: -2px;
  background: var(--bg-secondary, #f5f5f5);
}

.context-menu-item:active {
  background: var(--primary-bg, #e3f2fd);
}

.menu-icon {
  font-size: 16px;
}

.context-menu-divider {
  height: 1px;
  background: var(--border-color, #e0e0e0);
  margin: 4px 0;
}
</style>
