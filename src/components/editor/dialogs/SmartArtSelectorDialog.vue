<script setup lang="ts">
import { ref, computed } from 'vue';
import BaseDialog from './BaseDialog.vue';

interface Props {
  show: boolean;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'insert-smartart', smartart: SmartArtDefinition): void;
}

interface SmartArtDefinition {
  type: string;
  name: string;
  category: string;
  svg: string;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// SmartArt categories
const activeCategory = ref('list');

const smartartCategories = [
  { id: 'list', name: '列表' },
  { id: 'process', name: '流程' },
  { id: 'cycle', name: '循环' },
  { id: 'hierarchy', name: '层次结构' },
  { id: 'relationship', name: '关系' },
  { id: 'matrix', name: '矩阵' },
  { id: 'pyramid', name: '棱锥图' },
  { id: 'picture', name: '图片' }
];

// SmartArt definitions
const smartarts = ref<Record<string, SmartArtDefinition[]>>({
  list: [
    { type: 'basic-list', name: '基本列表', category: 'list', svg: '<rect x="2" y="2" width="20" height="4" rx="1" /><rect x="2" y="8" width="20" height="4" rx="1" /><rect x="2" y="14" width="20" height="4" rx="1" />' },
    { type: 'numbered-list', name: '编号列表', category: 'list', svg: '<circle cx="4" cy="4" r="2" /><rect x="8" y="2" width="14" height="4" rx="1" /><circle cx="4" cy="10" r="2" /><rect x="8" y="8" width="14" height="4" rx="1" /><circle cx="4" cy="16" r="2" /><rect x="8" y="14" width="14" height="4" rx="1" />' },
    { type: 'bulleted-list', name: '项目符号列表', category: 'list', svg: '<circle cx="4" cy="4" r="1.5" /><rect x="8" y="2" width="14" height="4" rx="1" /><circle cx="4" cy="10" r="1.5" /><rect x="8" y="8" width="14" height="4" rx="1" /><circle cx="4" cy="16" r="1.5" /><rect x="8" y="14" width="14" height="4" rx="1" />' },
    { type: 'vertical-list', name: '垂直列表', category: 'list', svg: '<rect x="2" y="2" width="20" height="4" rx="1" /><rect x="2" y="8" width="20" height="4" rx="1" /><rect x="2" y="14" width="20" height="4" rx="1" />' },
    { type: 'grouped-list', name: '分组列表', category: 'list', svg: '<rect x="2" y="2" width="20" height="4" rx="1" /><rect x="2" y="8" width="9" height="4" rx="1" /><rect x="13" y="8" width="9" height="4" rx="1" /><rect x="2" y="14" width="9" height="4" rx="1" /><rect x="13" y="14" width="9" height="4" rx="1" />' }
  ],
  process: [
    { type: 'basic-process', name: '基本流程', category: 'process', svg: '<rect x="2" y="8" width="5" height="8" rx="1" /><rect x="9.5" y="8" width="5" height="8" rx="1" /><rect x="17" y="8" width="5" height="8" rx="1" /><line x1="7" y1="12" x2="9.5" y2="12" /><line x1="14.5" y1="12" x2="17" y2="12" />' },
    { type: 'arrow-process', name: '箭头流程', category: 'process', svg: '<polygon points="2,8 7,8 7,2 17,12 7,22 7,16 2,16" /><polygon points="17,8 22,8 22,2 32,12 22,22 22,16 17,16" />' },
    { type: 'chevron-process', name: 'V形流程', category: 'process', svg: '<polygon points="2,2 12,12 2,22" /><polygon points="12,2 22,12 12,22" />' },
    { type: 'step-down-process', name: '步骤向下流程', category: 'process', svg: '<rect x="2" y="2" width="20" height="5" rx="1" /><rect x="2" y="9" width="20" height="5" rx="1" /><rect x="2" y="16" width="20" height="5" rx="1" /><line x1="12" y1="7" x2="12" y2="9" /><line x1="12" y1="14" x2="12" y2="16" />' },
    { type: 'accented-process', name: '强调流程', category: 'process', svg: '<rect x="2" y="8" width="5" height="8" rx="1" fill="currentColor" /><rect x="9.5" y="8" width="5" height="8" rx="1" /><rect x="17" y="8" width="5" height="8" rx="1" /><line x1="7" y1="12" x2="9.5" y2="12" /><line x1="14.5" y1="12" x2="17" y2="12" />' }
  ],
  cycle: [
    { type: 'basic-cycle', name: '基本循环', category: 'cycle', svg: '<circle cx="12" cy="12" r="8" fill="none" /><circle cx="12" cy="4" r="2" /><circle cx="20" cy="12" r="2" /><circle cx="12" cy="20" r="2" /><circle cx="4" cy="12" r="2" />' },
    { type: 'radial-cycle', name: '辐射循环', category: 'cycle', svg: '<circle cx="12" cy="12" r="3" /><circle cx="12" cy="4" r="2" /><circle cx="20" cy="12" r="2" /><circle cx="12" cy="20" r="2" /><circle cx="4" cy="12" r="2" /><line x1="12" y1="9" x2="12" y2="6" /><line x1="15" y1="12" x2="18" y2="12" /><line x1="12" y1="15" x2="12" y2="18" /><line x1="9" y1="12" x2="6" y2="12" />' },
    { type: 'segmented-cycle', name: '分段循环', category: 'cycle', svg: '<path d="M12 4 A8 8 0 0 1 20 12" /><path d="M20 12 A8 8 0 0 1 12 20" /><path d="M12 20 A8 8 0 0 1 4 12" /><path d="M4 12 A8 8 0 0 1 12 4" />' },
    { type: 'hexagon-radial', name: '六边形辐射', category: 'cycle', svg: '<polygon points="12,2 18,6 18,18 12,22 6,18 6,6" /><circle cx="12" cy="12" r="3" />' }
  ],
  hierarchy: [
    { type: 'organization-chart', name: '组织结构图', category: 'hierarchy', svg: '<rect x="8" y="2" width="8" height="4" rx="1" /><rect x="2" y="10" width="6" height="4" rx="1" /><rect x="9" y="10" width="6" height="4" rx="1" /><rect x="16" y="10" width="6" height="4" rx="1" /><line x1="12" y1="6" x2="12" y2="10" /><line x1="12" y1="8" x2="5" y2="8" /><line x1="5" y1="8" x2="5" y2="10" /><line x1="12" y1="8" x2="19" y2="8" /><line x1="19" y1="8" x2="19" y2="10" />' },
    { type: 'hierarchy', name: '层次结构', category: 'hierarchy', svg: '<rect x="9" y="2" width="6" height="4" rx="1" /><rect x="5" y="10" width="6" height="4" rx="1" /><rect x="13" y="10" width="6" height="4" rx="1" /><rect x="5" y="18" width="6" height="4" rx="1" /><rect x="13" y="18" width="6" height="4" rx="1" /><line x1="12" y1="6" x2="12" y2="10" /><line x1="12" y1="8" x2="8" y2="8" /><line x1="8" y1="8" x2="8" y2="10" /><line x1="12" y1="8" x2="16" y2="8" /><line x1="16" y1="8" x2="16" y2="10" /><line x1="8" y1="14" x2="8" y2="18" /><line x1="16" y1="14" x2="16" y2="18" />' },
    { type: 'labeled-hierarchy', name: '标记层次结构', category: 'hierarchy', svg: '<rect x="8" y="2" width="8" height="4" rx="1" /><rect x="2" y="10" width="6" height="4" rx="1" /><rect x="9" y="10" width="6" height="4" rx="1" /><rect x="16" y="10" width="6" height="4" rx="1" /><circle cx="4" cy="8" r="1.5" /><circle cx="12" cy="8" r="1.5" /><circle cx="20" cy="8" r="1.5" />' }
  ],
  relationship: [
    { type: 'balance', name: '平衡', category: 'relationship', svg: '<rect x="2" y="8" width="9" height="8" rx="1" /><rect x="13" y="8" width="9" height="8" rx="1" /><line x1="12" y1="12" x2="12" y2="12" />' },
    { type: 'venn', name: '维恩图', category: 'relationship', svg: '<circle cx="8" cy="12" r="6" fill="none" /><circle cx="16" cy="12" r="6" fill="none" />' },
    { type: 'stacked-venn', name: '堆叠维恩图', category: 'relationship', svg: '<circle cx="8" cy="10" r="5" fill="none" /><circle cx="16" cy="10" r="5" fill="none" /><circle cx="12" cy="18" r="5" fill="none" />' },
    { type: 'converging-arrows', name: '汇聚箭头', category: 'relationship', svg: '<polygon points="2,2 7,2 7,8 12,12 7,16 7,22 2,22" /><polygon points="22,2 17,2 17,8 12,12 17,16 17,22 22,22" />' }
  ],
  matrix: [
    { type: 'basic-matrix', name: '基本矩阵', category: 'matrix', svg: '<rect x="2" y="2" width="9" height="9" rx="1" /><rect x="13" y="2" width="9" height="9" rx="1" /><rect x="2" y="13" width="9" height="9" rx="1" /><rect x="13" y="13" width="9" height="9" rx="1" />' },
    { type: 'titled-matrix', name: '标题矩阵', category: 'matrix', svg: '<rect x="2" y="2" width="20" height="4" rx="1" /><rect x="2" y="8" width="9" height="14" rx="1" /><rect x="13" y="8" width="9" height="14" rx="1" />' },
    { type: 'grid-matrix', name: '网格矩阵', category: 'matrix', svg: '<rect x="2" y="2" width="6" height="6" rx="1" /><rect x="9" y="2" width="6" height="6" rx="1" /><rect x="16" y="2" width="6" height="6" rx="1" /><rect x="2" y="9" width="6" height="6" rx="1" /><rect x="9" y="9" width="6" height="6" rx="1" /><rect x="16" y="9" width="6" height="6" rx="1" /><rect x="2" y="16" width="6" height="6" rx="1" /><rect x="9" y="16" width="6" height="6" rx="1" /><rect x="16" y="16" width="6" height="6" rx="1" />' }
  ],
  pyramid: [
    { type: 'basic-pyramid', name: '基本棱锥图', category: 'pyramid', svg: '<polygon points="12,2 22,22 2,22" />' },
    { type: 'inverted-pyramid', name: '倒棱锥图', category: 'pyramid', svg: '<polygon points="2,2 22,2 12,22" />' },
    { type: 'segmented-pyramid', name: '分段棱锥图', category: 'pyramid', svg: '<polygon points="12,2 22,22 2,22" fill="none" /><line x1="12" y1="8" x2="18" y2="22" /><line x1="12" y1="8" x2="6" y2="22" /><line x1="12" y1="14" x2="16" y2="22" /><line x1="12" y1="14" x2="8" y2="22" />' }
  ],
  picture: [
    { type: 'picture-list', name: '图片列表', category: 'picture', svg: '<rect x="2" y="2" width="6" height="6" rx="1" /><rect x="10" y="2" width="12" height="6" rx="1" /><rect x="2" y="10" width="6" height="6" rx="1" /><rect x="10" y="10" width="12" height="6" rx="1" /><rect x="2" y="18" width="6" height="6" rx="1" /><rect x="10" y="18" width="12" height="6" rx="1" />' },
    { type: 'picture-process', name: '图片流程', category: 'picture', svg: '<rect x="2" y="6" width="6" height="12" rx="1" /><rect x="10" y="6" width="6" height="12" rx="1" /><rect x="18" y="6" width="6" height="12" rx="1" /><line x1="8" y1="12" x2="10" y2="12" /><line x1="16" y1="12" x2="18" y2="12" />' },
    { type: 'chevron-picture', name: 'V形图片', category: 'picture', svg: '<polygon points="2,2 12,12 2,22" fill="none" /><rect x="14" y="2" width="8" height="8" rx="1" /><rect x="14" y="14" width="8" height="8" rx="1" />' }
  ]
});

// Selected SmartArt
const selectedSmartArt = ref<SmartArtDefinition | null>(null);

// Search query
const searchQuery = ref('');

// Computed
const currentSmartArts = computed(() => {
  const categorySmartArts = smartarts.value[activeCategory.value] || [];
  if (!searchQuery.value) {
    return categorySmartArts;
  }
  return categorySmartArts.filter(smartart =>
    smartart.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

// Select SmartArt
const selectSmartArt = (smartart: SmartArtDefinition) => {
  selectedSmartArt.value = smartart;
};

// Insert SmartArt
const insertSmartArt = () => {
  if (selectedSmartArt.value) {
    emit('insert-smartart', selectedSmartArt.value);
    emit('update:show', false);
  }
};

// Cancel
const cancel = () => {
  emit('update:show', false);
};
</script>

<template>
  <BaseDialog
    :show="show"
    title="插入SmartArt"
    width="700px"
    height="500px"
    @update:show="cancel"
  >
    <div class="smartart-selector-dialog">
      <!-- Categories -->
      <div class="categories">
        <button
          v-for="category in smartartCategories"
          :key="category.id"
          class="category-btn"
          :class="{ active: activeCategory === category.id }"
          type="button"
          @click="activeCategory = category.id"
        >
          {{ category.name }}
        </button>
      </div>

      <!-- Search -->
      <div class="search-bar">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="搜索SmartArt..."
          class="search-input"
        />
      </div>

      <!-- SmartArt grid -->
      <div class="smartarts-grid">
        <div
          v-for="smartart in currentSmartArts"
          :key="smartart.type"
          class="smartart-item"
          :class="{ selected: selectedSmartArt?.type === smartart.type }"
          @click="selectSmartArt(smartart)"
        >
          <div class="smartart-preview">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="64"
              height="64"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="1.5"
              stroke-linecap="round"
              stroke-linejoin="round"
              v-html="smartart.svg"
            />
          </div>
          <span class="smartart-name">{{ smartart.name }}</span>
        </div>
      </div>

      <!-- Selected SmartArt preview -->
      <div v-if="selectedSmartArt" class="selected-preview">
        <h4 class="preview-title">预览</h4>
        <div class="preview-content">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="120"
            height="120"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            v-html="selectedSmartArt.svg"
          />
        </div>
        <p class="preview-name">{{ selectedSmartArt.name }}</p>
      </div>
    </div>

    <template #footer>
      <button class="dialog-btn secondary" type="button" @click="cancel">
        取消
      </button>
      <button
        class="dialog-btn primary"
        :disabled="!selectedSmartArt"
        type="button"
        @click="insertSmartArt"
      >
        插入
      </button>
    </template>
  </BaseDialog>
</template>

<style scoped>
.smartart-selector-dialog {
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: 100%;
}

/* Categories */
.categories {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
  border-bottom: 1px solid var(--word-border);
  padding-bottom: 12px;
}

.category-btn {
  padding: 8px 16px;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  cursor: pointer;
  font-size: 13px;
  color: var(--word-text-secondary);
  transition: all 0.15s ease;
}

.category-btn:hover {
  color: var(--word-text-primary);
  background: var(--word-button-hover);
}

.category-btn.active {
  color: var(--word-text-primary);
  border-bottom-color: var(--word-button-pressed);
  font-weight: 600;
}

/* Search */
.search-bar {
  display: flex;
}

.search-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 13px;
}

.search-input:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

/* SmartArt grid */
.smartarts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 12px;
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

.smartart-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.smartart-item:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

.smartart-item.selected {
  background: var(--word-button-active);
  border-color: var(--word-button-pressed);
}

.smartart-preview {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 64px;
  height: 64px;
  color: var(--word-text-primary);
}

.smartart-name {
  font-size: 11px;
  color: var(--word-text-secondary);
  text-align: center;
}

/* Selected preview */
.selected-preview {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 16px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-border);
  border-radius: 4px;
}

.preview-title {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--word-text-primary);
}

.preview-content {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 120px;
  height: 120px;
  background: white;
  border: 1px solid var(--word-border);
  border-radius: 4px;
  color: var(--word-text-primary);
}

.preview-name {
  margin: 0;
  font-size: 13px;
  color: var(--word-text-secondary);
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

.dialog-btn.primary:hover:not(:disabled) {
  background: var(--word-button-pressed);
}

.dialog-btn.primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
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
