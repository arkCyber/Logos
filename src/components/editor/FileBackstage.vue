<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { save, open } from '@tauri-apps/plugin-dialog';
import { logger, LogCategory } from '../../utils/logger';
import {
  Clock, FilePlus, Info, Search,
  FileText, Download, Printer, X,
  Settings, HelpCircle, User, Palette,
  Keyboard, BookOpen, Globe, Github
} from 'lucide-vue-next';

interface Props {
  show: boolean;
  recentFiles: string[];
  documentTitle: string;
}

interface Emits {
  (e: 'close'): void;
  (e: 'new-document'): void;
  (e: 'open-document'): void;
  (e: 'save-document'): void;
  (e: 'save-as'): void;
  (e: 'load-recent-file', filePath: string): void;
  (e: 'clear-recent-files'): void;
  (e: 'export-pdf'): void;
  (e: 'export-word'): void;
  (e: 'export-typst'): void;
  (e: 'export-svg-typst'): void;
  (e: 'export-svg-html'): void;
  (e: 'print'): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const activeTab = ref<'recent' | 'new' | 'info' | 'settings' | 'help' | 'account'>('recent');
const searchQuery = ref('');

const filteredRecentFiles = computed(() => {
  if (!searchQuery.value) {
    return props.recentFiles;
  }
  return props.recentFiles.filter(file => 
    file.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

const getFileName = (filePath: string): string => {
  const parts = filePath.split(/[/\\]/);
  return parts[parts.length - 1] || filePath;
};

const getFileDate = (_filePath: string): string => {
  // Simplified file date display - in production this would fetch actual file stats
  // For now, return a placeholder to avoid async complexity in template
  return '最近';
};

const handleNewDocument = () => {
  emit('new-document');
  emit('close');
};

const handleTemplateDocument = () => {
  // Template selection feature to be implemented in future
  // For now, show a message indicating this feature is coming soon
  alert('模板功能即将推出，敬请期待！');
};

const handleOpenDocument = async () => {
  try {
    const filePath = await open({
      multiple: false,
      filters: [
        {
          name: '所有文件',
          extensions: ['*']
        },
        {
          name: 'HTML',
          extensions: ['html', 'htm']
        },
        {
          name: 'Markdown',
          extensions: ['md']
        },
        {
          name: '纯文本',
          extensions: ['txt']
        }
      ]
    });
    
    if (filePath && typeof filePath === 'string') {
      emit('load-recent-file', filePath);
      emit('close');
    }
  } catch (error) {
    logger.error('Failed to open file dialog', error as Error, LogCategory.SYSTEM);
  }
};

const handleSaveDocument = () => {
  emit('save-document');
  emit('close');
};

const handleSaveAs = async () => {
  try {
    const filePath = await save({
      defaultPath: `${props.documentTitle || '未命名文档'}.html`,
      filters: [
        {
          name: 'HTML',
          extensions: ['html']
        },
        {
          name: 'Markdown',
          extensions: ['md']
        },
        {
          name: '纯文本',
          extensions: ['txt']
        }
      ]
    });
    
    if (filePath) {
      emit('save-as');
      emit('close');
    }
  } catch (error) {
    logger.error('Failed to open save dialog', error as Error, LogCategory.SYSTEM);
  }
};

const handleLoadRecentFile = (filePath: string) => {
  emit('load-recent-file', filePath);
  emit('close');
};

const handleClearRecentFiles = () => {
  if (confirm('确定要清除所有最近文件记录吗？')) {
    emit('clear-recent-files');
  }
};

const handleExportPdf = () => {
  emit('export-pdf');
  emit('close');
};

const handleExportWord = () => {
  emit('export-word');
  emit('close');
};

const handleExportTypst = () => {
  emit('export-typst');
  emit('close');
};

const handleExportSvgTypst = () => {
  emit('export-svg-typst');
  emit('close');
};

const handleExportSvgHtml = () => {
  emit('export-svg-html');
  emit('close');
};

const handlePrint = () => {
  emit('print');
  emit('close');
};

// Settings handlers
const themeOptions = ref(['light', 'dark', 'auto']);
const selectedTheme = ref('auto');
const fontSizeOptions = ref(['12px', '14px', '16px', '18px']);
const selectedFontSize = ref('14px');

const handleThemeChange = (theme: string) => {
  selectedTheme.value = theme;
  // Apply theme logic here
  if (theme === 'dark') {
    document.documentElement.classList.add('dark');
  } else if (theme === 'light') {
    document.documentElement.classList.remove('dark');
  } else {
    // Auto - follow system preference
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }
};

const handleFontSizeChange = (size: string) => {
  selectedFontSize.value = size;
  document.documentElement.style.fontSize = size;
};

// Help handlers
const shortcuts = [
  { key: 'Ctrl+S', action: '保存文档' },
  { key: 'Ctrl+O', action: '打开文件' },
  { key: 'Ctrl+N', action: '新建文档' },
  { key: 'Ctrl+P', action: '打印文档' },
  { key: 'Ctrl+Z', action: '撤销' },
  { key: 'Ctrl+Y', action: '重做' },
  { key: 'Ctrl+F', action: '查找' },
  { key: 'Ctrl+H', action: '替换' },
  { key: 'F1', action: '打开帮助' },
  { key: 'Escape', action: '关闭对话框' }
];

const handleOpenDocumentation = () => {
  window.open('https://github.com/your-org/logos-zhidao-office/blob/main/docs/USER_GUIDE.md', '_blank');
};

const handleOpenGitHub = () => {
  window.open('https://github.com/your-org/logos-zhidao-office', '_blank');
};

const handleReportIssue = () => {
  window.open('https://github.com/your-org/logos-zhidao-office/issues', '_blank');
};

const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    emit('close');
  }
};

onMounted(() => {
  document.addEventListener('keydown', handleKeyDown);
});

onBeforeUnmount(() => {
  document.removeEventListener('keydown', handleKeyDown);
});
</script>

<template>
  <Transition name="backstage">
    <div v-if="show" class="file-backstage" @click.self="emit('close')">
      <div class="backstage-content" @click.stop>
        <!-- Left Sidebar -->
        <div class="backstage-sidebar">
          <button
            class="sidebar-item"
            :class="{ active: activeTab === 'recent' }"
            aria-label="最近文件"
            @click="activeTab = 'recent'"
          >
            <Clock :size="20" />
            <span>最近</span>
          </button>
          
          <button
            class="sidebar-item"
            :class="{ active: activeTab === 'new' }"
            aria-label="新建文档"
            @click="activeTab = 'new'"
          >
            <FilePlus :size="20" />
            <span>新建</span>
          </button>
          
          <button
            class="sidebar-item"
            :class="{ active: activeTab === 'info' }"
            aria-label="文档信息"
            @click="activeTab = 'info'"
          >
            <Info :size="20" />
            <span>信息</span>
          </button>

          <button
            class="sidebar-item"
            :class="{ active: activeTab === 'settings' }"
            aria-label="设置"
            @click="activeTab = 'settings'"
          >
            <Settings :size="20" />
            <span>设置</span>
          </button>

          <button
            class="sidebar-item"
            :class="{ active: activeTab === 'help' }"
            aria-label="帮助"
            @click="activeTab = 'help'"
          >
            <HelpCircle :size="20" />
            <span>帮助</span>
          </button>

          <button
            class="sidebar-item"
            :class="{ active: activeTab === 'account' }"
            aria-label="账户"
            @click="activeTab = 'account'"
          >
            <User :size="20" />
            <span>账户</span>
          </button>
        </div>

        <!-- Main Content Area -->
        <div class="backstage-main">
          <!-- Recent Files Tab -->
          <div v-if="activeTab === 'recent'" class="tab-content">
            <div class="tab-header">
              <h2>最近文件</h2>
              <div class="search-box">
                <input
                  v-model="searchQuery"
                  type="text"
                  placeholder="搜索最近文件..."
                  aria-label="搜索最近文件"
                />
                <Search :size="16" />
              </div>
            </div>

            <div v-if="filteredRecentFiles.length > 0" class="recent-files-list">
              <button
                v-for="file in filteredRecentFiles"
                :key="file"
                class="recent-file-item"
                :title="file"
                @click="handleLoadRecentFile(file)"
              >
                <div class="file-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
                    <polyline points="14 2 14 8 20 8" />
                    <path d="M16 13H8" />
                    <path d="M16 17H8" />
                    <path d="M10 9H8" />
                  </svg>
                </div>
                <div class="file-info">
                  <div class="file-name">{{ getFileName(file) }}</div>
                  <div class="file-path">{{ file }}</div>
                  <div class="file-date">{{ getFileDate(file) }}</div>
                </div>
              </button>
              
              <button
                v-if="recentFiles.length > 0"
                class="clear-recent-button"
                aria-label="清除最近文件"
                @click="handleClearRecentFiles"
              >
                <X :size="16" />
                清除最近文件
              </button>
            </div>

            <div v-else class="empty-state">
              <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round">
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                <polyline points="14 2 14 8 20 8" />
              </svg>
              <p>没有最近文件</p>
              <button class="btn-primary" @click="handleOpenDocument">
                打开文件
              </button>
            </div>
          </div>

          <!-- New Document Tab -->
          <div v-if="activeTab === 'new'" class="tab-content">
            <div class="tab-header">
              <h2>新建文档</h2>
            </div>

            <div class="new-document-options">
              <button class="document-option" aria-label="创建空白文档" @click="handleNewDocument">
                <div class="option-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                    <polyline points="14 2 14 8 20 8" />
                  </svg>
                </div>
                <div class="option-info">
                  <h3>空白文档</h3>
                  <p>创建一个新的空白文档</p>
                </div>
              </button>

              <button class="document-option" aria-label="从模板创建文档" @click="handleTemplateDocument">
                <div class="option-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M3 3v18h18" />
                    <path d="M18.7 8l-5.1 5.2-2.8-2.7L7 14.3" />
                  </svg>
                </div>
                <div class="option-info">
                  <h3>从模板</h3>
                  <p>使用模板创建文档</p>
                </div>
              </button>
            </div>
          </div>

          <!-- Info Tab -->
          <div v-if="activeTab === 'info'" class="tab-content">
            <div class="tab-header">
              <h2>文档信息</h2>
            </div>

            <div class="document-info">
              <div class="info-section">
                <h3>基本信息</h3>
                <div class="info-row">
                  <span class="label">文件名:</span>
                  <span class="value">{{ documentTitle || '未命名文档' }}</span>
                </div>
                <div class="info-row">
                  <span class="label">类型:</span>
                  <span class="value">HTML 文档</span>
                </div>
                <div class="info-row">
                  <span class="label">位置:</span>
                  <span class="value">未保存</span>
                </div>
              </div>

              <div class="info-section">
                <h3>操作</h3>
                <div class="action-buttons">
                  <button class="action-button" aria-label="保存文档" @click="handleSaveDocument">
                    <FileText :size="16" />
                    保存
                  </button>
                  <button class="action-button" aria-label="另存为" @click="handleSaveAs">
                    <Download :size="16" />
                    另存为
                  </button>
                  <button class="action-button" aria-label="打印文档" @click="handlePrint">
                    <Printer :size="16" />
                    打印
                  </button>
                </div>
              </div>

              <div class="info-section">
                <h3>导出</h3>
                <div class="action-buttons">
                  <button class="action-button" aria-label="导出PDF" @click="handleExportPdf">
                    <FileText :size="16" />
                    导出 PDF
                  </button>
                  <button class="action-button" aria-label="导出Word" @click="handleExportWord">
                    <FileText :size="16" />
                    导出 Word
                  </button>
                  <button class="action-button" aria-label="Export Typst" @click="handleExportTypst">
                    <FileText :size="16" />
                    Export Typst
                  </button>
                  <button class="action-button" aria-label="Export SVG (Typst)" @click="handleExportSvgTypst">
                    <FileText :size="16" />
                    Export SVG (Typst)
                  </button>
                  <button class="action-button" aria-label="Export SVG (HTML)" @click="handleExportSvgHtml">
                    <FileText :size="16" />
                    Export SVG (HTML)
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- Settings Tab -->
          <div v-if="activeTab === 'settings'" class="tab-content">
            <div class="tab-header">
              <h2>设置</h2>
            </div>

            <div class="settings-content">
              <div class="settings-section">
                <h3>
                  <Palette :size="18" />
                  外观
                </h3>
                <div class="setting-item">
                  <label class="setting-label">主题</label>
                  <div class="setting-options">
                    <button
                      v-for="theme in themeOptions"
                      :key="theme"
                      class="theme-option"
                      :class="{ active: selectedTheme === theme }"
                      @click="handleThemeChange(theme)"
                    >
                      {{ theme === 'light' ? '浅色' : theme === 'dark' ? '深色' : '自动' }}
                    </button>
                  </div>
                </div>
                <div class="setting-item">
                  <label class="setting-label">字体大小</label>
                  <div class="setting-options">
                    <button
                      v-for="size in fontSizeOptions"
                      :key="size"
                      class="font-option"
                      :class="{ active: selectedFontSize === size }"
                      @click="handleFontSizeChange(size)"
                    >
                      {{ size }}
                    </button>
                  </div>
                </div>
              </div>

              <div class="settings-section">
                <h3>
                  <Settings :size="18" />
                  编辑器
                </h3>
                <div class="setting-item">
                  <label class="setting-label">自动保存</label>
                  <div class="setting-toggle">
                    <input id="auto-save" type="checkbox" checked />
                    <label for="auto-save">启用</label>
                  </div>
                </div>
                <div class="setting-item">
                  <label class="setting-label">拼写检查</label>
                  <div class="setting-toggle">
                    <input id="spell-check" type="checkbox" checked />
                    <label for="spell-check">启用</label>
                  </div>
                </div>
                <div class="setting-item">
                  <label class="setting-label">显示格式标记</label>
                  <div class="setting-toggle">
                    <input id="format-marks" type="checkbox" />
                    <label for="format-marks">启用</label>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Help Tab -->
          <div v-if="activeTab === 'help'" class="tab-content">
            <div class="tab-header">
              <h2>帮助</h2>
            </div>

            <div class="help-content">
              <div class="help-section">
                <h3>
                  <Keyboard :size="18" />
                  快捷键
                </h3>
                <div class="shortcuts-list">
                  <div v-for="shortcut in shortcuts" :key="shortcut.key" class="shortcut-item">
                    <kbd class="shortcut-key">{{ shortcut.key }}</kbd>
                    <span class="shortcut-action">{{ shortcut.action }}</span>
                  </div>
                </div>
              </div>

              <div class="help-section">
                <h3>
                  <BookOpen :size="18" />
                  文档
                </h3>
                <div class="help-links">
                  <button class="help-link" @click="handleOpenDocumentation">
                    <BookOpen :size="16" />
                    <span>用户指南</span>
                    <Globe :size="14" />
                  </button>
                  <button class="help-link" @click="handleOpenDocumentation">
                    <BookOpen :size="16" />
                    <span>开发者指南</span>
                    <Globe :size="14" />
                  </button>
                </div>
              </div>

              <div class="help-section">
                <h3>
                  <Github :size="18" />
                  社区
                </h3>
                <div class="help-links">
                  <button class="help-link" @click="handleOpenGitHub">
                    <Github :size="16" />
                    <span>GitHub 仓库</span>
                    <Globe :size="14" />
                  </button>
                  <button class="help-link" @click="handleReportIssue">
                    <Github :size="16" />
                    <span>报告问题</span>
                    <Globe :size="14" />
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- Account Tab -->
          <div v-if="activeTab === 'account'" class="tab-content">
            <div class="tab-header">
              <h2>账户</h2>
            </div>

            <div class="account-content">
              <div class="account-info">
                <div class="account-avatar">
                  <User :size="48" />
                </div>
                <div class="account-details">
                  <h3>未登录</h3>
                  <p>登录以同步您的文档和设置</p>
                </div>
              </div>

              <div class="account-actions">
                <button class="btn-primary">
                  登录
                </button>
                <button class="btn-secondary">
                  注册账户
                </button>
              </div>

              <div class="account-features">
                <h4>账户功能</h4>
                <ul>
                  <li>云端同步文档</li>
                  <li>跨设备访问</li>
                  <li>版本历史</li>
                  <li>协作编辑</li>
                  <li>更多存储空间</li>
                </ul>
              </div>
            </div>
          </div>
        </div>

        <!-- Close Button -->
        <button class="close-button" aria-label="关闭" @click="emit('close')">
          <X :size="20" />
        </button>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.file-backstage {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
}

.backstage-content {
  background-color: var(--word-bg, #ffffff);
  border-radius: 8px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  width: 90%;
  max-width: 1000px;
  height: 80vh;
  max-height: 700px;
  display: flex;
  position: relative;
  overflow: hidden;
}

.backstage-sidebar {
  width: 200px;
  background-color: var(--word-sidebar-bg, #f5f5f5);
  border-right: 1px solid var(--word-border, #e0e0e0);
  display: flex;
  flex-direction: column;
  padding: 16px 0;
}

.sidebar-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 20px;
  border: none;
  background: none;
  color: var(--word-text-primary, #333);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

.sidebar-item:hover {
  background-color: var(--word-button-hover, #e8e8e8);
}

.sidebar-item.active {
  background-color: var(--word-button-active, #d0d0d0);
  border-left: 3px solid var(--word-accent, #007bff);
}

.sidebar-item svg {
  flex-shrink: 0;
}

.backstage-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.tab-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.tab-header {
  padding: 20px 24px;
  border-bottom: 1px solid var(--word-border, #e0e0e0);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.tab-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--word-text-primary, #333);
}

.search-box {
  position: relative;
  width: 300px;
}

.search-box input {
  width: 100%;
  padding: 8px 12px 8px 36px;
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 4px;
  font-size: 14px;
  outline: none;
}

.search-box input:focus {
  border-color: var(--word-accent, #007bff);
}

.search-box svg {
  position: absolute;
  left: 10px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--word-text-secondary, #666);
}

.recent-files-list {
  flex: 1;
  overflow-y: auto;
  padding: 16px 24px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.recent-file-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px 16px;
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 6px;
  background: var(--word-bg, #ffffff);
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

.recent-file-item:hover {
  background-color: var(--word-button-hover, #f5f5f5);
  border-color: var(--word-accent, #007bff);
  transform: translateX(4px);
}

.file-icon {
  flex-shrink: 0;
  color: var(--word-text-secondary, #666);
}

.file-info {
  flex: 1;
  min-width: 0;
}

.file-name {
  font-weight: 500;
  color: var(--word-text-primary, #333);
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-path {
  font-size: 12px;
  color: var(--word-text-secondary, #666);
  margin-bottom: 2px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-date {
  font-size: 11px;
  color: var(--word-text-tertiary, #999);
}

.clear-recent-button {
  margin-top: 8px;
  padding: 8px 16px;
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 4px;
  background: var(--word-bg, #ffffff);
  color: var(--word-text-secondary, #666);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 8px;
  align-self: flex-start;
}

.clear-recent-button:hover {
  background-color: var(--word-error-bg, #fee);
  color: var(--word-error, #dc3545);
  border-color: var(--word-error, #dc3545);
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  color: var(--word-text-secondary, #666);
}

.empty-state svg {
  color: var(--word-text-tertiary, #ccc);
}

.empty-state p {
  margin: 0;
  font-size: 16px;
}

.btn-primary {
  padding: 10px 24px;
  background-color: var(--word-accent, #007bff);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.btn-primary:hover {
  background-color: var(--word-accent-dark, #0056b3);
}

.new-document-options {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 24px;
  overflow-y: auto;
}

.document-option {
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 24px;
  border: 2px solid var(--word-border, #e0e0e0);
  border-radius: 8px;
  background: var(--word-bg, #ffffff);
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

.document-option:hover {
  border-color: var(--word-accent, #007bff);
  background-color: var(--word-button-hover, #f5f5f5);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.option-icon {
  flex-shrink: 0;
  color: var(--word-accent, #007bff);
}

.option-info h3 {
  margin: 0 0 4px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--word-text-primary, #333);
}

.option-info p {
  margin: 0;
  font-size: 13px;
  color: var(--word-text-secondary, #666);
}

.document-info {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
}

.info-section {
  margin-bottom: 32px;
}

.info-section h3 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--word-text-primary, #333);
}

.info-row {
  display: flex;
  padding: 8px 0;
  border-bottom: 1px solid var(--word-border-light, #f0f0f0);
}

.info-row .label {
  width: 120px;
  color: var(--word-text-secondary, #666);
  font-weight: 500;
}

.info-row .value {
  flex: 1;
  color: var(--word-text-primary, #333);
}

.action-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.action-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 4px;
  background: var(--word-bg, #ffffff);
  color: var(--word-text-primary, #333);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.action-button:hover {
  background-color: var(--word-button-hover, #f5f5f5);
  border-color: var(--word-accent, #007bff);
}

.close-button {
  position: absolute;
  top: 16px;
  right: 16px;
  width: 32px;
  height: 32px;
  border: none;
  background: var(--word-bg, #ffffff);
  border-radius: 4px;
  color: var(--word-text-secondary, #666);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  z-index: 10;
}

.close-button:hover {
  background-color: var(--word-error-bg, #fee);
  color: var(--word-error, #dc3545);
}

/* Settings Styles */
.settings-content {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
}

.settings-section {
  margin-bottom: 32px;
}

.settings-section h3 {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--word-text-primary, #333);
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 0;
  border-bottom: 1px solid var(--word-border-light, #f0f0f0);
}

.setting-label {
  font-weight: 500;
  color: var(--word-text-primary, #333);
}

.setting-options {
  display: flex;
  gap: 8px;
}

.theme-option,
.font-option {
  padding: 6px 16px;
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 4px;
  background: var(--word-bg, #ffffff);
  color: var(--word-text-primary, #333);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.theme-option:hover,
.font-option:hover {
  border-color: var(--word-accent, #007bff);
}

.theme-option.active,
.font-option.active {
  background-color: var(--word-accent, #007bff);
  color: white;
  border-color: var(--word-accent, #007bff);
}

.setting-toggle {
  display: flex;
  align-items: center;
  gap: 8px;
}

.setting-toggle input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.setting-toggle label {
  font-size: 13px;
  color: var(--word-text-primary, #333);
  cursor: pointer;
}

/* Help Styles */
.help-content {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
}

.help-section {
  margin-bottom: 32px;
}

.help-section h3 {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--word-text-primary, #333);
}

.shortcuts-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.shortcut-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 8px 0;
}

.shortcut-key {
  background-color: var(--word-button-bg, #f5f5f5);
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 4px;
  padding: 4px 10px;
  font-family: var(--word-font-ui, 'Segoe UI', system-ui);
  font-size: 13px;
  font-weight: 500;
  color: var(--word-text-primary, #333);
  min-width: 100px;
  text-align: center;
}

.shortcut-action {
  font-size: 14px;
  color: var(--word-text-secondary, #666);
}

.help-links {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.help-link {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 6px;
  background: var(--word-bg, #ffffff);
  color: var(--word-text-primary, #333);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

.help-link:hover {
  background-color: var(--word-button-hover, #f5f5f5);
  border-color: var(--word-accent, #007bff);
}

.help-link span {
  flex: 1;
}

/* Account Styles */
.account-content {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.account-info {
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 24px;
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 8px;
  background: var(--word-bg, #ffffff);
}

.account-avatar {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  background: var(--word-button-bg, #f5f5f5);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--word-text-secondary, #666);
}

.account-details h3 {
  margin: 0 0 4px 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--word-text-primary, #333);
}

.account-details p {
  margin: 0;
  font-size: 14px;
  color: var(--word-text-secondary, #666);
}

.account-actions {
  display: flex;
  gap: 12px;
}

.btn-secondary {
  padding: 10px 24px;
  background-color: var(--word-bg, #ffffff);
  color: var(--word-text-primary, #333);
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background-color: var(--word-button-hover, #f5f5f5);
  border-color: var(--word-accent, #007bff);
}

.account-features {
  padding: 20px;
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 8px;
  background: var(--word-button-bg, #f5f5f5);
}

.account-features h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--word-text-primary, #333);
}

.account-features ul {
  margin: 0;
  padding-left: 20px;
  list-style: disc;
}

.account-features li {
  font-size: 13px;
  color: var(--word-text-secondary, #666);
  margin: 4px 0;
}

/* Transition animations */
.backstage-enter-active,
.backstage-leave-active {
  transition: opacity 0.2s ease;
}

.backstage-enter-from,
.backstage-leave-to {
  opacity: 0;
}

.backstage-enter-active .backstage-content,
.backstage-leave-active .backstage-content {
  transition: transform 0.2s ease, opacity 0.2s ease;
}

.backstage-enter-from .backstage-content,
.backstage-leave-to .backstage-content {
  transform: scale(0.95);
  opacity: 0;
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  .file-backstage {
    background-color: rgba(0, 0, 0, 0.7);
  }
  
  .backstage-content {
    background-color: #1e1e1e;
  }
  
  .backstage-sidebar {
    background-color: #252526;
    border-right-color: #3e3e42;
  }
  
  .sidebar-item {
    color: #cccccc;
  }
  
  .sidebar-item:hover {
    background-color: #2d2d30;
  }
  
  .sidebar-item.active {
    background-color: #37373d;
    border-left-color: #0078d4;
  }
  
  .tab-header {
    border-bottom-color: #3e3e42;
  }
  
  .tab-header h2 {
    color: #ffffff;
  }
  
  .recent-file-item {
    background-color: #252526;
    border-color: #3e3e42;
  }
  
  .recent-file-item:hover {
    background-color: #2d2d30;
    border-color: #0078d4;
  }
  
  .file-name {
    color: #ffffff;
  }
  
  .file-path {
    color: #cccccc;
  }
  
  .document-option {
    background-color: #252526;
    border-color: #3e3e42;
  }
  
  .document-option:hover {
    background-color: #2d2d30;
    border-color: #0078d4;
  }
  
  .option-info h3 {
    color: #ffffff;
  }
  
  .info-section h3 {
    color: #ffffff;
  }
  
  .info-row {
    border-bottom-color: #3e3e42;
  }
  
  .info-row .label {
    color: #cccccc;
  }
  
  .info-row .value {
    color: #ffffff;
  }
  
  .action-button {
    background-color: #252526;
    border-color: #3e3e42;
    color: #ffffff;
  }
  
  .action-button:hover {
    background-color: #2d2d30;
    border-color: #0078d4;
  }
  
  .close-button {
    background-color: #252526;
    color: #cccccc;
  }
  
  .close-button:hover {
    background-color: #3e3e42;
    color: #ffffff;
  }
}
</style>
