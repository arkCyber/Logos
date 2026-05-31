<script setup lang="ts">
import { ref, computed } from 'vue';
import BaseDialog from './BaseDialog.vue';

interface Props {
  show: boolean;
  type: 'header' | 'footer';
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'apply', content: HeaderFooterContent): void;
}

interface HeaderFooterContent {
  type: 'header' | 'footer';
  differentFirst: boolean;
  differentOddEven: boolean;
  firstPageContent: string;
  oddPageContent: string;
  evenPageContent: string;
  position: 'left' | 'center' | 'right';
  distanceFromEdge: number;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// Tab state
const activeTab = ref<'first' | 'odd' | 'even'>('odd');

// Content state
const differentFirst = ref(false);
const differentOddEven = ref(false);
const firstPageContent = ref('');
const oddPageContent = ref('');
const evenPageContent = ref('');
const position = ref<'left' | 'center' | 'right'>('center');
const distanceFromEdge = ref(1.27);

// Computed
const dialogTitle = computed(() => {
  return props.type === 'header' ? '页眉' : '页脚';
});

const currentContent = computed(() => {
  switch (activeTab.value) {
    case 'first':
      return firstPageContent.value;
    case 'odd':
      return oddPageContent.value;
    case 'even':
      return evenPageContent.value;
    default:
      return '';
  }
});

const setCurrentContent = (value: string) => {
  switch (activeTab.value) {
    case 'first':
      firstPageContent.value = value;
      break;
    case 'odd':
      oddPageContent.value = value;
      break;
    case 'even':
      evenPageContent.value = value;
      break;
  }
};

// Insert field
const insertField = (field: string) => {
  const fieldCode = `{${field}}`;
  setCurrentContent(currentContent.value + fieldCode);
};

// Insert page number
const insertPageNumber = () => {
  insertField('PAGE');
};

// Insert total pages
const insertTotalPages = () => {
  insertField('NUMPAGES');
};

// Insert date
const insertDate = () => {
  insertField('DATE');
};

// Insert time
const insertTime = () => {
  insertField('TIME');
};

// Insert document title
const insertTitle = () => {
  insertField('TITLE');
};

// Insert author
const insertAuthor = () => {
  insertField('AUTHOR');
};

// Apply settings
const handleApply = () => {
  const content: HeaderFooterContent = {
    type: props.type,
    differentFirst: differentFirst.value,
    differentOddEven: differentOddEven.value,
    firstPageContent: firstPageContent.value,
    oddPageContent: oddPageContent.value,
    evenPageContent: evenPageContent.value,
    position: position.value,
    distanceFromEdge: distanceFromEdge.value
  };
  emit('apply', content);
  emit('update:show', false);
};

// Cancel
const handleCancel = () => {
  emit('update:show', false);
};

// Clear content
const clearContent = () => {
  setCurrentContent('');
};
</script>

<template>
  <BaseDialog
    :show="show"
    :title="dialogTitle"
    width="600px"
    @update:show="handleCancel"
  >
    <div class="header-footer-dialog">
      <!-- Options -->
      <div class="dialog-section">
        <h4 class="section-title">选项</h4>
        <div class="options-grid">
          <label class="checkbox-label">
            <input v-model="differentFirst" type="checkbox" />
            <span>首页不同</span>
          </label>
          <label class="checkbox-label">
            <input v-model="differentOddEven" type="checkbox" />
            <span>奇偶页不同</span>
          </label>
        </div>
      </div>

      <!-- Tabs -->
      <div class="dialog-section">
        <div class="content-tabs">
          <button
            class="content-tab"
            :class="{ active: activeTab === 'odd' }"
            type="button"
            @click="activeTab = 'odd'"
          >
            奇数页
          </button>
          <button
            v-if="differentOddEven"
            class="content-tab"
            :class="{ active: activeTab === 'even' }"
            type="button"
            @click="activeTab = 'even'"
          >
            偶数页
          </button>
          <button
            v-if="differentFirst"
            class="content-tab"
            :class="{ active: activeTab === 'first' }"
            type="button"
            @click="activeTab = 'first'"
          >
            首页
          </button>
        </div>
      </div>

      <!-- Content editor -->
      <div class="dialog-section">
        <div class="content-editor">
          <textarea
            v-model="currentContent"
            placeholder="输入页眉/页脚内容，可使用字段代码如 {PAGE}、{DATE} 等"
            class="content-textarea"
            @input="(e) => setCurrentContent((e.target as HTMLTextAreaElement).value)"
          ></textarea>
        </div>
      </div>

      <!-- Field buttons -->
      <div class="dialog-section">
        <h4 class="section-title">插入字段</h4>
        <div class="field-buttons">
          <button class="field-btn" type="button" @click="insertPageNumber">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
              <path d="M14 2v6h6" />
              <line x1="16" y1="13" x2="8" y2="13" />
              <line x1="16" y1="17" x2="8" y2="17" />
              <polyline points="10 9 9 9 8 9" />
            </svg>
            页码
          </button>
          <button class="field-btn" type="button" @click="insertTotalPages">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
              <path d="M14 2v6h6" />
              <line x1="16" y1="13" x2="8" y2="13" />
              <line x1="16" y1="17" x2="8" y2="17" />
              <line x1="10" y1="9" x2="14" y2="9" />
            </svg>
            总页数
          </button>
          <button class="field-btn" type="button" @click="insertDate">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <rect x="3" y="4" width="18" height="18" rx="2" ry="2" />
              <line x1="16" y1="2" x2="16" y2="6" />
              <line x1="8" y1="2" x2="8" y2="6" />
              <line x1="3" y1="10" x2="21" y2="10" />
            </svg>
            日期
          </button>
          <button class="field-btn" type="button" @click="insertTime">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <circle cx="12" cy="12" r="10" />
              <polyline points="12 6 12 12 16 14" />
            </svg>
            时间
          </button>
          <button class="field-btn" type="button" @click="insertTitle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
              <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" />
            </svg>
            标题
          </button>
          <button class="field-btn" type="button" @click="insertAuthor">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
              <circle cx="12" cy="7" r="4" />
            </svg>
            作者
          </button>
        </div>
      </div>

      <!-- Position -->
      <div class="dialog-section">
        <h4 class="section-title">位置</h4>
        <div class="position-options">
          <button
            class="position-btn"
            :class="{ active: position === 'left' }"
            type="button"
            @click="position = 'left'"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <line x1="17" y1="10" x2="3" y2="10" />
              <line x1="21" y1="6" x2="3" y2="6" />
              <line x1="21" y1="14" x2="3" y2="14" />
              <line x1="17" y1="18" x2="3" y2="18" />
            </svg>
            左对齐
          </button>
          <button
            class="position-btn"
            :class="{ active: position === 'center' }"
            type="button"
            @click="position = 'center'"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <line x1="21" y1="6" x2="3" y2="6" />
              <line x1="21" y1="12" x2="3" y2="12" />
              <line x1="21" y1="18" x2="3" y2="18" />
            </svg>
            居中
          </button>
          <button
            class="position-btn"
            :class="{ active: position === 'right' }"
            type="button"
            @click="position = 'right'"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <line x1="21" y1="10" x2="7" y2="10" />
              <line x1="21" y1="6" x2="3" y2="6" />
              <line x1="21" y1="14" x2="3" y2="14" />
              <line x1="21" y1="18" x2="7" y2="18" />
            </svg>
            右对齐
          </button>
        </div>
      </div>

      <!-- Distance from edge -->
      <div class="dialog-section">
        <h4 class="section-title">距离边缘</h4>
        <div class="distance-input">
          <input
            v-model.number="distanceFromEdge"
            type="number"
            step="0.1"
            min="0"
            max="10"
          />
          <span>cm</span>
        </div>
      </div>

      <!-- Preview -->
      <div class="dialog-section">
        <h4 class="section-title">预览</h4>
        <div class="preview-area">
          <div
            class="preview-content"
            :style="{
              textAlign: position,
              paddingTop: distanceFromEdge + 'cm',
              paddingBottom: distanceFromEdge + 'cm'
            }"
          >
            <span v-if="currentContent">{{ currentContent }}</span>
            <span v-else class="preview-placeholder">预览将在此显示</span>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <button class="dialog-btn secondary" type="button" @click="clearContent">
        清除
      </button>
      <button class="dialog-btn secondary" type="button" @click="handleCancel">
        取消
      </button>
      <button class="dialog-btn primary" type="button" @click="handleApply">
        确定
      </button>
    </template>
  </BaseDialog>
</template>

<style scoped>
.header-footer-dialog {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.dialog-section {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.section-title {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--word-text-primary);
}

/* Options */
.options-grid {
  display: flex;
  gap: 20px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 13px;
  color: var(--word-text-primary);
}

.checkbox-label input[type="checkbox"] {
  cursor: pointer;
}

/* Tabs */
.content-tabs {
  display: flex;
  border-bottom: 1px solid var(--word-border);
}

.content-tab {
  padding: 10px 20px;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  cursor: pointer;
  font-size: 13px;
  color: var(--word-text-secondary);
  transition: all 0.15s ease;
}

.content-tab:hover {
  color: var(--word-text-primary);
  background: var(--word-button-hover);
}

.content-tab.active {
  color: var(--word-text-primary);
  border-bottom-color: var(--word-button-pressed);
  font-weight: 600;
}

/* Content editor */
.content-editor {
  width: 100%;
}

.content-textarea {
  width: 100%;
  min-height: 80px;
  padding: 12px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 13px;
  font-family: var(--word-font-ui);
  resize: vertical;
}

.content-textarea:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

/* Field buttons */
.field-buttons {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.field-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  color: var(--word-text-primary);
  transition: all 0.15s ease;
}

.field-btn:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

/* Position */
.position-options {
  display: flex;
  gap: 8px;
}

.position-btn {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 12px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  color: var(--word-text-primary);
  transition: all 0.15s ease;
}

.position-btn:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

.position-btn.active {
  background: var(--word-button-active);
  border-color: var(--word-button-pressed);
}

/* Distance */
.distance-input {
  display: flex;
  align-items: center;
  gap: 8px;
}

.distance-input input {
  width: 80px;
  padding: 8px 12px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 13px;
}

.distance-input input:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

.distance-input span {
  font-size: 13px;
  color: var(--word-text-secondary);
}

/* Preview */
.preview-area {
  padding: 20px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-border);
  border-radius: 4px;
  min-height: 100px;
}

.preview-content {
  background: white;
  border: 1px solid var(--word-border);
  padding: 20px;
  min-height: 60px;
  color: var(--word-text-primary);
  font-size: 13px;
}

.preview-placeholder {
  color: var(--word-text-secondary);
  font-style: italic;
}

/* Dialog buttons */
.dialog-btn {
  padding: 8px 24px;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border: none;
}

.dialog-btn.primary {
  background: var(--word-button-active);
  color: var(--word-text-primary);
}

.dialog-btn.primary:hover {
  background: var(--word-button-pressed);
}

.dialog-btn.secondary {
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  color: var(--word-text-primary);
}

.dialog-btn.secondary:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

/* Dark mode */
:global(.dark) .preview-content {
  background: var(--word-bg-canvas);
}
</style>
