<script setup lang="ts">
import { ref, onMounted } from 'vue';

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
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const isMacOS = ref(false);
const isEditingTitle = ref(false);
const editingTitle = ref('');

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
    <div class="qat-left">
      <button
        class="qat-button file-button"
        title="文件"
        aria-label="文件"
        @click="emit('toggle-file-backstage')"
      >
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
        <span>文件</span>
      </button>
    </div>
    <div class="qat-title" aria-label="文档标题">
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
        {{ props.documentTitle ? props.documentTitle + ' — LOGOS' : '未命名文档 — LOGOS' }}
      </span>
    </div>
    <div class="qat-center">
      <button class="qat-button" title="保存 (Ctrl+S)" aria-label="保存" @click="emit('save')">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
          <polyline points="17 21 17 13 7 13 7 21" />
          <polyline points="7 3 7 8 15 8" />
        </svg>
      </button>
      <button class="qat-button" title="撤销 (Ctrl+Z)" aria-label="撤销" @click="emit('undo')">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="M3 7v6h6" />
          <path d="M21 17a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6 2.3L3 13" />
        </svg>
      </button>
      <button class="qat-button" title="重做 (Ctrl+Y)" aria-label="重做" @click="emit('redo')">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="M21 7v6h-6" />
          <path d="M3 17a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6 2.3l3 3.7" />
        </svg>
      </button>
      <div class="qat-separator"></div>
      <button
        class="qat-button"
        title="双显示 (Ctrl+D)"
        aria-label="双显示"
        @click="emit('toggle-split-view')"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
          <line x1="12" y1="3" x2="12" y2="21" />
        </svg>
      </button>
      <div class="qat-separator"></div>
      <button
        class="qat-button"
        title="查找 (Ctrl+F)"
        aria-label="查找"
        @click="emit('toggle-search')"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <circle cx="11" cy="11" r="8" />
          <path d="m21 21-4.3-4.3" />
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.quick-access-toolbar {
  height: 36px;
  background: var(--word-ribbon-bg);
  border-bottom: 1px solid var(--word-border);
  display: flex;
  align-items: center;
  padding: 0 8px;
  gap: 4px;
  flex-shrink: 0;
  position: relative;
  /* Allow the bar to be used as Tauri drag region */
  -webkit-app-region: drag;
  app-region: drag;
  user-select: none;
}

/* macOS: shift content right to clear traffic-light buttons (≈78 px) */
.quick-access-toolbar.macos-titlebar {
  padding-left: 80px;
}

/* All interactive children must opt out of drag region */
.quick-access-toolbar button,
.quick-access-toolbar input,
.quick-access-toolbar select,
.quick-access-toolbar a {
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.qat-left {
  display: flex;
  align-items: center;
  gap: 2px;
  z-index: 1;
}

.qat-center {
  display: flex;
  align-items: center;
  gap: 2px;
  margin-left: auto;
  z-index: 1;
}

.qat-title {
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
  font-family: var(--word-font-ui);
  font-size: 12px;
  font-weight: 500;
  color: var(--word-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 400px;
  pointer-events: none;
  user-select: none;
}

/* On macOS, shift title right to account for traffic lights */
.macos-titlebar .qat-title {
  left: calc(50% + 40px);
}

.qat-title-text {
  cursor: pointer;
  user-select: none;
}

.qat-title-input {
  font-family: var(--word-font-ui);
  font-size: 12px;
  font-weight: 500;
  color: var(--word-text-primary);
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 2px;
  padding: 2px 6px;
  width: 300px;
  max-width: 400px;
  outline: none;
  text-align: center;
}

.qat-title-input:focus {
  border-color: var(--word-button-border-hover);
  background: var(--word-button-hover);
}

.qat-button {
  height: 24px;
  min-width: 24px;
  padding: 0 6px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 2px;
  color: var(--word-text-primary);
  font-family: var(--word-font-ui);
  font-size: 12px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  transition: all 0.1s ease;
}

.qat-button:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

.qat-button:active {
  background: var(--word-button-active);
  border-color: var(--word-button-pressed);
}

.qat-button:disabled {
  background: var(--word-button-disabled-bg);
  color: var(--word-button-disabled-text);
  cursor: not-allowed;
}

.file-button {
  min-width: 60px;
  font-weight: var(--word-font-weight-semibold);
}

.file-button svg {
  width: 18px;
  height: 18px;
}

.qat-separator {
  width: 1px;
  height: 20px;
  background: var(--word-divider);
  margin: 0 4px;
}

.qat-button svg {
  flex-shrink: 0;
}
</style>
