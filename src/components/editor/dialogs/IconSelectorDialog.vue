<script setup lang="ts">
import { ref, computed } from 'vue';
import BaseDialog from './BaseDialog.vue';

interface Props {
  show: boolean;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'insert-icon', icon: IconDefinition): void;
}

interface IconDefinition {
  name: string;
  category: string;
  svg: string;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// Icon categories
const activeCategory = ref('common');

const iconCategories = [
  { id: 'common', name: '常用' },
  { id: 'arrows', name: '箭头' },
  { id: 'math', name: '数学' },
  { id: 'currency', name: '货币' },
  { id: 'symbols', name: '符号' },
  { id: 'ui', name: 'UI' },
  { id: 'weather', name: '天气' },
  { id: 'social', name: '社交' }
];

// Icon definitions
const icons = ref<Record<string, IconDefinition[]>>({
  common: [
    { name: 'check', category: 'common', svg: '<polyline points="20 6 9 17 4 12" />' },
    { name: 'x', category: 'common', svg: '<line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" />' },
    { name: 'plus', category: 'common', svg: '<line x1="12" y1="5" x2="12" y2="19" /><line x1="5" y1="12" x2="19" y2="12" />' },
    { name: 'minus', category: 'common', svg: '<line x1="5" y1="12" x2="19" y2="12" />' },
    { name: 'star', category: 'common', svg: '<polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />' },
    { name: 'heart', category: 'common', svg: '<path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />' },
    { name: 'flag', category: 'common', svg: '<path d="M4 15s1-1 4-1 5 2 8 2 4-2 5-2 8-2 4 1 4 1V4s-1 1-4 1-5-2-8-2-4 2-5 2-8 2-4-1-4-1z" />' },
    { name: 'bookmark', category: 'common', svg: '<path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" />' },
    { name: 'bell', category: 'common', svg: '<path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" /><path d="M13.73 21a2 2 0 0 1-3.46 0" />' },
    { name: 'info', category: 'common', svg: '<circle cx="12" cy="12" r="10" /><line x1="12" y1="16" x2="12" y2="12" /><line x1="12" y1="8" x2="12.01" y2="8" />' },
    { name: 'warning', category: 'common', svg: '<path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" /><line x1="12" y1="9" x2="12" y2="13" /><line x1="12" y1="17" x2="12.01" y2="17" />' },
    { name: 'alert-circle', category: 'common', svg: '<circle cx="12" cy="12" r="10" /><line x1="12" y1="8" x2="12" y2="12" /><line x1="12" y1="16" x2="12.01" y2="16" />' }
  ],
  arrows: [
    { name: 'arrow-up', category: 'arrows', svg: '<line x1="12" y1="19" x2="12" y2="5" /><polyline points="5 12 12 5 19 12" />' },
    { name: 'arrow-down', category: 'arrows', svg: '<line x1="12" y1="5" x2="12" y2="19" /><polyline points="19 12 12 19 5 12" />' },
    { name: 'arrow-left', category: 'arrows', svg: '<line x1="19" y1="12" x2="5" y2="12" /><polyline points="12 19 5 12 12 5" />' },
    { name: 'arrow-right', category: 'arrows', svg: '<line x1="5" y1="12" x2="19" y2="12" /><polyline points="12 5 19 12 12 19" />' },
    { name: 'arrow-up-right', category: 'arrows', svg: '<line x1="7" y1="17" x2="17" y2="7" /><polyline points="7 7 17 7 17 17" />' },
    { name: 'arrow-up-left', category: 'arrows', svg: '<line x1="17" y1="17" x2="7" y2="7" /><polyline points="17 7 7 7 7 17" />' },
    { name: 'arrow-down-right', category: 'arrows', svg: '<line x1="7" y1="7" x2="17" y2="17" /><polyline points="7 17 17 17 17 7" />' },
    { name: 'arrow-down-left', category: 'arrows', svg: '<line x1="17" y1="7" x2="7" y2="17" /><polyline points="17 17 7 17 7 7" />' }
  ],
  math: [
    { name: 'divide', category: 'math', svg: '<circle cx="12" cy="6" r="2" /><line x1="5" y1="12" x2="19" y2="12" /><circle cx="12" cy="18" r="2" />' },
    { name: 'equal', category: 'math', svg: '<line x1="5" y1="9" x2="19" y2="9" /><line x1="5" y1="15" x2="19" y2="15" />' },
    { name: 'percent', category: 'math', svg: '<line x1="19" y1="5" x2="5" y2="19" /><circle cx="6.5" cy="6.5" r="2.5" /><circle cx="17.5" cy="17.5" r="2.5" />' },
    { name: 'infinity', category: 'math', svg: '<path d="M12 12c-2-2.67-4-4-6-4a4 4 0 1 0 0 8c2 0 4-1.33 6-4Zm0 0c2 2.67 4 4 6 4a4 4 0 1 0 0-8c-2 0-4 1.33-6 4Z" />' },
    { name: 'pi', category: 'math', svg: '<path d="M18 13c0 3-2 5-4 5s-4-2-4-5V9c0-3 2-5 4-5s4 2 4 5" />' },
    { name: 'function', category: 'math', svg: '<path d="M5 12h14" /><path d="M5 12a7 7 0 0 1 7-7a7 7 0 0 1 7 7" /><path d="M5 12a7 7 0 0 0 7 7a7 7 0 0 0 7-7" />' }
  ],
  currency: [
    { name: 'dollar-sign', category: 'currency', svg: '<line x1="12" y1="1" x2="12" y2="23" /><path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />' },
    { name: 'euro', category: 'currency', svg: '<path d="M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm0 18a8 8 0 1 1 8-8 8 8 0 0 1-8 8z" /><path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8" />' },
    { name: 'pound', category: 'currency', svg: '<line x1="4" y1="6" x2="20" y2="6" /><path d="M4 12h16" /><path d="M4 18h16" /><path d="M10 6v12" />' },
    { name: 'yen', category: 'currency', svg: '<path d="M12 2v20" /><path d="M5 6h14" /><path d="M5 12h14" /><path d="M5 18h14" />' },
    { name: 'bitcoin', category: 'currency', svg: '<circle cx="12" cy="12" r="10" /><path d="M12 6v12" /><path d="M8 12h8" /><path d="M9 6h6" /><path d="M9 18h6" />' }
  ],
  symbols: [
    { name: 'hash', category: 'symbols', svg: '<line x1="4" y1="9" x2="20" y2="9" /><line x1="4" y1="15" x2="20" y2="15" /><line x1="10" y1="3" x2="8" y2="21" /><line x1="16" y1="3" x2="14" y2="21" />' },
    { name: 'at-sign', category: 'symbols', svg: '<circle cx="12" cy="12" r="4" /><path d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94" />' },
    { name: 'ampersand', category: 'symbols', svg: '<path d="M10 2v7.31" /><path d="M14 2v7.31" /><path d="M8.5 2h7" /><path d="M14 9.3a6.5 6.5 0 1 1-4 0" />' },
    { name: 'copyright', category: 'symbols', svg: '<circle cx="12" cy="12" r="10" /><path d="M15 9.35A4 4 0 1 1 12 12a4 4 0 0 1 0-2.65" />' },
    { name: 'registered', category: 'symbols', svg: '<circle cx="12" cy="12" r="10" /><path d="M12 6v12" /><path d="M9 9h6" /><path d="M9 15h6" />' },
    { name: 'tm', category: 'symbols', svg: '<path d="M4 9v12" /><path d="M4 15h6" /><path d="M14 9v12" /><path d="M14 15h6" />' }
  ],
  ui: [
    { name: 'home', category: 'ui', svg: '<path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" /><polyline points="9 22 9 12 15 12 15 22" />' },
    { name: 'settings', category: 'ui', svg: '<circle cx="12" cy="12" r="3" /><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />' },
    { name: 'user', category: 'ui', svg: '<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" /><circle cx="12" cy="7" r="4" />' },
    { name: 'search', category: 'ui', svg: '<circle cx="11" cy="11" r="8" /><line x1="21" y1="21" x2="16.65" y2="16.65" />' },
    { name: 'menu', category: 'ui', svg: '<line x1="3" y1="12" x2="21" y2="12" /><line x1="3" y1="6" x2="21" y2="6" /><line x1="3" y1="18" x2="21" y2="18" />' },
    { name: 'more-horizontal', category: 'ui', svg: '<circle cx="12" cy="12" r="1" /><circle cx="19" cy="12" r="1" /><circle cx="5" cy="12" r="1" />' },
    { name: 'more-vertical', category: 'ui', svg: '<circle cx="12" cy="12" r="1" /><circle cx="12" cy="5" r="1" /><circle cx="12" cy="19" r="1" />' }
  ],
  weather: [
    { name: 'sun', category: 'weather', svg: '<circle cx="12" cy="12" r="5" /><line x1="12" y1="1" x2="12" y2="3" /><line x1="12" y1="21" x2="12" y2="23" /><line x1="4.22" y1="4.22" x2="5.64" y2="5.64" /><line x1="18.36" y1="18.36" x2="19.78" y2="19.78" /><line x1="1" y1="12" x2="3" y2="12" /><line x1="21" y1="12" x2="23" y2="12" /><line x1="4.22" y1="19.78" x2="5.64" y2="18.36" /><line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />' },
    { name: 'cloud', category: 'weather', svg: '<path d="M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z" />' },
    { name: 'cloud-rain', category: 'weather', svg: '<path d="M16 13v8" /><path d="M8 13v8" /><path d="M12 15v8" /><path d="M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25" />' },
    { name: 'snowflake', category: 'weather', svg: '<line x1="2" y1="12" x2="22" y2="12" /><line x1="12" y1="2" x2="12" y2="22" /><line x1="4.93" y1="4.93" x2="19.07" y2="19.07" /><line x1="19.07" y1="4.93" x2="4.93" y2="19.07" />' },
    { name: 'wind', category: 'weather', svg: '<path d="M9.59 4.59A2 2 0 1 1 11 8H2m10.59 11.41A2 2 0 1 0 14 16H2m15.73-8.27A2.5 2.5 0 1 1 19.5 12H2" />' }
  ],
  social: [
    { name: 'mail', category: 'social', svg: '<path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" /><polyline points="22,6 12,13 2,6" />' },
    { name: 'phone', category: 'social', svg: '<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />' },
    { name: 'share-2', category: 'social', svg: '<circle cx="18" cy="5" r="3" /><circle cx="6" cy="12" r="3" /><circle cx="18" cy="19" r="3" /><line x1="8.59" y1="13.51" x2="15.42" y2="17.49" /><line x1="15.41" y1="6.51" x2="8.59" y2="10.49" />' },
    { name: 'share', category: 'social', svg: '<path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" /><polyline points="16 6 12 2 8 6" /><line x1="12" y1="2" x2="12" y2="15" />' },
    { name: 'link', category: 'social', svg: '<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" /><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />' }
  ]
});

// Selected icon
const selectedIcon = ref<IconDefinition | null>(null);

// Search query
const searchQuery = ref('');

// Computed
const currentIcons = computed(() => {
  const categoryIcons = icons.value[activeCategory.value] || [];
  if (!searchQuery.value) {
    return categoryIcons;
  }
  return categoryIcons.filter(icon =>
    icon.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

// Select icon
const selectIcon = (icon: IconDefinition) => {
  selectedIcon.value = icon;
};

// Insert icon
const insertIcon = () => {
  if (selectedIcon.value) {
    emit('insert-icon', selectedIcon.value);
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
    title="插入图标"
    width="600px"
    height="500px"
    @update:show="cancel"
  >
    <div class="icon-selector-dialog">
      <!-- Categories -->
      <div class="categories">
        <button
          v-for="category in iconCategories"
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
          placeholder="搜索图标..."
          class="search-input"
        />
      </div>

      <!-- Icons grid -->
      <div class="icons-grid">
        <div
          v-for="icon in currentIcons"
          :key="icon.name"
          class="icon-item"
          :class="{ selected: selectedIcon?.name === icon.name }"
          @click="selectIcon(icon)"
        >
          <div class="icon-preview">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="32"
              height="32"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              v-html="icon.svg"
            />
          </div>
          <span class="icon-name">{{ icon.name }}</span>
        </div>
      </div>

      <!-- Selected icon preview -->
      <div v-if="selectedIcon" class="selected-preview">
        <h4 class="preview-title">预览</h4>
        <div class="preview-content">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="64"
            height="64"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            v-html="selectedIcon.svg"
          />
        </div>
        <p class="preview-name">{{ selectedIcon.name }}</p>
      </div>
    </div>

    <template #footer>
      <button class="dialog-btn secondary" type="button" @click="cancel">
        取消
      </button>
      <button
        class="dialog-btn primary"
        :disabled="!selectedIcon"
        type="button"
        @click="insertIcon"
      >
        插入
      </button>
    </template>
  </BaseDialog>
</template>

<style scoped>
.icon-selector-dialog {
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

/* Icons grid */
.icons-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(64px, 1fr));
  gap: 12px;
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

.icon-item {
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

.icon-item:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

.icon-item.selected {
  background: var(--word-button-active);
  border-color: var(--word-button-pressed);
}

.icon-preview {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  color: var(--word-text-primary);
}

.icon-name {
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
  width: 64px;
  height: 64px;
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
