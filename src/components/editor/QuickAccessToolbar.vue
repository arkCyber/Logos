<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { MessageCircleMore, HelpCircle } from 'lucide-vue-next';

interface Props {
  showFileBackstage: boolean;
  documentTitle?: string;
}

interface Emits {
  (e: 'toggle-file-backstage'): void;
  (e: 'save'): void;
  (e: 'undo'): void;
  (e: 'redo'): void;
  (e: 'toggle-search'): void;
  (e: 'update-title', title: string): void;
  (e: 'toggle-split-view'): void;
  (e: 'toggle-ai-sidebar'): void;
  (e: 'toggle-help'): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const isMacOS = ref(false);
const isEditingTitle = ref(false);
const editingTitle = ref('');
const autoSave = ref(true);

const startEditingTitle = () => {
  isEditingTitle.value = true;
  editingTitle.value = props.documentTitle || '';
};

const finishEditingTitle = () => {
  if (editingTitle.value.trim()) {
    emit('update-title', editingTitle.value.trim());
  }
  isEditingTitle.value = false;
};

const cancelEditingTitle = () => {
  isEditingTitle.value = false;
  editingTitle.value = '';
};

const handleTitleKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Enter') {
    finishEditingTitle();
  } else if (e.key === 'Escape') {
    cancelEditingTitle();
  }
};

const handleToggleAISidebar = () => {
  emit('toggle-ai-sidebar');
};

onMounted(() => {
  isMacOS.value =
    typeof navigator !== 'undefined' &&
    (navigator.platform.includes('Mac') || navigator.userAgent.includes('Macintosh'));
});
</script>

<template>
  <div
    class="quick-access-toolbar"
    :class="{ 'macos-titlebar': isMacOS }"
    role="toolbar"
    aria-label="快速访问工具栏"
    data-tauri-drag-region
  >
    <!-- 左侧区域：自动保存与快速访问按钮 -->
    <div class="qat-left">
      <div class="qat-autosave-container">
        <span class="autosave-text">自动保存</span>
        <div class="autosave-toggle" :class="{ active: autoSave }" @click="autoSave = !autoSave">
          <div class="autosave-slider"></div>
        </div>
      </div>

      <button class="qat-button" title="保存 (Ctrl+S)" aria-label="保存" @click="emit('save')">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
          <polyline points="17 21 17 13 7 13 7 21" />
          <polyline points="7 3 7 8 15 8" />
        </svg>
      </button>

      <button class="qat-button" title="撤销 (Ctrl+Z)" aria-label="撤销" @click="emit('undo')">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M3 7v6h6" />
          <path d="M21 17a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6 2.3L3 13" />
        </svg>
      </button>

      <button class="qat-button" title="重做 (Ctrl+Y)" aria-label="重做" @click="emit('redo')">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 7v6h-6" />
          <path d="M3 17a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6 2.3l3 3.7" />
        </svg>
      </button>

      <button class="qat-button dropdown-trigger" title="自定义快速访问工具栏">
        <svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
          <path d="m6 9 6 6 6-9"/>
        </svg>
      </button>
    </div>

    <!-- 中间左侧：文档标题 -->
    <div class="qat-title-container">
      <input
        v-if="isEditingTitle"
        ref="titleInput"
        v-model="editingTitle"
        class="qat-title-input"
        @blur="finishEditingTitle"
        @keydown="handleTitleKeydown"
        @click.stop
      />
      <span v-else class="qat-title-text" @dblclick="startEditingTitle">
        {{ props.documentTitle ? props.documentTitle : '未命名文档' }}
      </span>
      <span class="qat-app-suffix">- Logos</span>
    </div>

    <!-- 中间：大搜索框 -->
    <div class="qat-search-container">
      <div class="qat-search-box" @click="emit('toggle-search')">
        <svg class="search-icon" xmlns="http://www.w3.org/2000/svg" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>
        </svg>
        <span class="search-placeholder">搜索</span>
        <svg class="mic-icon" xmlns="http://www.w3.org/2000/svg" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z"/>
          <path d="M19 10v1a7 7 0 0 1-14 0v-1"/><line x1="12" y1="19" x2="12" y2="22"/>
        </svg>
      </div>
    </div>

    <!-- 右侧区域：高级功能按钮、用户头像、窗口控制 -->
    <div class="qat-right">
      <button class="qat-right-btn text-btn editing-btn" title="更改编辑模式">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 20h9"/><path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z"/>
        </svg>
        <span>正在编辑</span>
        <svg class="arrow-down" xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
          <path d="m6 9 6 6 6-9"/>
        </svg>
      </button>

      <button class="qat-right-btn share-btn" title="双开窗口" @click="emit('toggle-split-view')">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
          <line x1="12" y1="3" x2="12" y2="21" />
        </svg>
        <span>双窗</span>
      </button>

      <div class="qat-separator"></div>

      <!-- Help Button -->
      <button class="qat-button help-btn" title="帮助 (F1)" @click="emit('toggle-help')">
        <HelpCircle :size="18" />
      </button>

      <!-- 用户头像 -->
      <div class="qat-avatar" title="AI 助手" @click="handleToggleAISidebar">
        <MessageCircleMore :size="20" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.quick-access-toolbar {
  height: 48px;
  background: var(--word-ribbon-bg);
  border-bottom: 1px solid var(--word-border);
  display: flex;
  align-items: center;
  padding: 0 4px 0 8px;
  gap: 12px;
  flex-shrink: 0;
  position: relative;
  -webkit-app-region: drag;
  app-region: drag;
  user-select: none;
}

/* macOS traffic lights offset */
.quick-access-toolbar.macos-titlebar {
  padding-left: 80px;
}

/* All interactive children must opt out of drag region */
.quick-access-toolbar button,
.quick-access-toolbar input,
.quick-access-toolbar .autosave-toggle,
.quick-access-toolbar .qat-avatar {
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.qat-left {
  display: flex;
  align-items: center;
  gap: 4px;
}

/* AutoSave Switch Style */
.qat-autosave-container {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-right: 8px;
}

.autosave-text {
  font-family: var(--word-font-ui);
  font-size: 11px;
  color: var(--word-text-primary);
  font-weight: 500;
}

.autosave-toggle {
  width: 32px;
  height: 16px;
  background-color: #a19f9d;
  border-radius: 8px;
  position: relative;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.autosave-toggle.active {
  background-color: var(--word-accent);
}

.autosave-slider {
  width: 10px;
  height: 10px;
  background-color: white;
  border-radius: 50%;
  position: absolute;
  top: 3px;
  left: 3px;
  transition: transform 0.15s ease;
}

.autosave-toggle.active .autosave-slider {
  transform: translateX(16px);
}

.qat-button {
  height: 32px;
  width: 32px;
  background: transparent;
  border: none;
  border-radius: 3px;
  color: var(--word-text-primary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.1s ease;
}

.qat-button:hover {
  background: var(--word-button-hover);
}

.qat-button.dropdown-trigger {
  width: 14px;
}

.qat-button.help-btn {
  color: var(--word-text-secondary, #666);
}

.qat-button.help-btn:hover {
  color: var(--word-accent, #007bff);
  background-color: var(--word-button-hover, #f5f5f5);
}

/* Title container styling */
.qat-title-container {
  display: flex;
  align-items: center;
  gap: 4px;
  font-family: var(--word-font-ui);
  font-size: 12px;
  color: var(--word-text-primary);
  font-weight: 500;
  max-width: 250px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.qat-title-text {
  cursor: pointer;
  font-weight: 600;
}

.qat-app-suffix {
  color: var(--word-text-secondary);
}

.qat-title-input {
  font-family: var(--word-font-ui);
  font-size: 12px;
  font-weight: 600;
  color: var(--word-text-primary);
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 2px;
  padding: 1px 6px;
  width: 140px;
  outline: none;
}

/* Wide Search Box styling */
.qat-search-container {
  flex: 1;
  display: flex;
  justify-content: center;
  max-width: 322px;
  margin: 0 auto;
}

.qat-search-box {
  width: 100%;
  height: 28px;
  background: var(--word-input-bg, #f3f2f1);
  border: 1px solid var(--word-input-border, #edebe9);
  border-radius: 4px;
  display: flex;
  align-items: center;
  padding: 0 10px;
  gap: 8px;
  cursor: pointer;
  transition: all 0.1s ease;
}

.qat-search-box:hover {
  background: #edebe9;
  border-color: #d2d0ce;
}

.search-icon, .mic-icon {
  color: #605e5c;
  flex-shrink: 0;
}

.search-placeholder {
  font-family: var(--word-font-ui);
  font-size: 12px;
  color: #605e5c;
  flex: 1;
}

/* Right buttons section styling */
.qat-right {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-left: auto;
}

.qat-right-btn {
  height: 32px;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 0 10px;
  background: transparent;
  border: none;
  border-radius: 3px;
  font-family: var(--word-font-ui);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.1s ease;
  color: var(--word-text-primary);
}

.qat-right-btn.text-btn:hover {
  background: var(--word-button-hover);
}

.qat-right-btn .arrow-down {
  margin-left: -2px;
  color: var(--word-text-secondary);
}

/* Word Style Blue Share Button */
.qat-right-btn.share-btn {
  background-color: transparent;
  color: var(--word-text-primary);
  font-weight: 500;
  border-radius: 4px;
}

.qat-right-btn.share-btn:hover {
  background-color: var(--word-button-hover);
}

.qat-separator {
  width: 1px;
  height: 18px;
  background: var(--word-divider, #edebe9);
  margin: 0 4px;
}

/* Avatar styling */
.qat-avatar {
  width: 32px;
  height: 32px;
  background-color: transparent;
  color: var(--word-text-primary);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  border: 1px solid var(--word-button-border);
}

.qat-avatar:hover {
  background-color: var(--word-button-hover);
}

.qat-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
  margin-left: 8px;
}

.qat-avatar:hover {
  transform: scale(1.05);
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.4);
}
</style>
