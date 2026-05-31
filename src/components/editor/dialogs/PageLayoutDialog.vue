<script setup lang="ts">
import { ref, computed } from 'vue';
import BaseDialog from './BaseDialog.vue';

interface Props {
  show: boolean;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'apply', settings: PageLayoutSettings): void;
}

interface PageLayoutSettings {
  orientation: 'portrait' | 'landscape';
  pageSize: string;
  margins: {
    top: number;
    right: number;
    bottom: number;
    left: number;
  };
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// Page size presets
const pageSizePresets = [
  { id: 'a4', name: 'A4', width: 210, height: 297 },
  { id: 'a3', name: 'A3', width: 297, height: 420 },
  { id: 'letter', name: 'Letter', width: 216, height: 279 },
  { id: 'legal', name: 'Legal', width: 216, height: 356 }
];

// Margin presets
const marginPresets = [
  { id: 'normal', name: '普通', top: 2.54, right: 2.54, bottom: 2.54, left: 2.54 },
  { id: 'narrow', name: '窄', top: 1.27, right: 1.27, bottom: 1.27, left: 1.27 },
  { id: 'moderate', name: '适中', top: 2.54, right: 1.91, bottom: 2.54, left: 1.91 },
  { id: 'wide', name: '宽', top: 3.17, right: 3.17, bottom: 3.17, left: 3.17 }
];

// Current settings
const orientation = ref<'portrait' | 'landscape'>('portrait');
const selectedPageSize = ref('a4');
const selectedMarginPreset = ref('normal');
const customMargins = ref({
  top: 2.54,
  right: 2.54,
  bottom: 2.54,
  left: 2.54
});

const useCustomMargins = ref(false);

// Computed
const currentPageSize = computed(() => {
  return pageSizePresets.find(p => p.id === selectedPageSize.value) || pageSizePresets[0];
});

const currentMarginPreset = computed(() => {
  return marginPresets.find(m => m.id === selectedMarginPreset.value) || marginPresets[0];
});

// Apply margin preset
const applyMarginPreset = () => {
  const preset = currentMarginPreset.value;
  customMargins.value = {
    top: preset.top,
    right: preset.right,
    bottom: preset.bottom,
    left: preset.left
  };
};

// Watch margin preset changes
const handleMarginPresetChange = () => {
  useCustomMargins.value = false;
  applyMarginPreset();
};

// Handle custom margin change
const handleCustomMarginChange = () => {
  useCustomMargins.value = true;
};

// Apply settings
const handleApply = () => {
  const settings: PageLayoutSettings = {
    orientation: orientation.value,
    pageSize: selectedPageSize.value,
    margins: customMargins.value
  };
  emit('apply', settings);
  emit('update:show', false);
};

// Cancel
const handleCancel = () => {
  emit('update:show', false);
};

// Initialize with default preset
applyMarginPreset();
</script>

<template>
  <BaseDialog
    :show="show"
    title="页面设置"
    width="600px"
    @update:show="emit('update:show', $event)"
  >
    <div class="page-layout-dialog">
      <!-- Orientation -->
      <div class="dialog-section">
        <h4 class="section-title">页面方向</h4>
        <div class="orientation-options">
          <button
            class="orientation-btn"
            :class="{ active: orientation === 'portrait' }"
            type="button"
            @click="orientation = 'portrait'"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="48"
              height="64"
              viewBox="0 0 48 64"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <rect x="2" y="2" width="44" height="60" rx="2" />
            </svg>
            <span>纵向</span>
          </button>
          <button
            class="orientation-btn"
            :class="{ active: orientation === 'landscape' }"
            type="button"
            @click="orientation = 'landscape'"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="64"
              height="48"
              viewBox="0 0 64 48"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <rect x="2" y="2" width="60" height="44" rx="2" />
            </svg>
            <span>横向</span>
          </button>
        </div>
      </div>

      <!-- Page Size -->
      <div class="dialog-section">
        <h4 class="section-title">纸张大小</h4>
        <div class="page-size-grid">
          <button
            v-for="size in pageSizePresets"
            :key="size.id"
            class="page-size-btn"
            :class="{ active: selectedPageSize === size.id }"
            type="button"
            @click="selectedPageSize = size.id"
          >
            <div class="page-size-preview">
              <div
                class="page-size-rect"
                :style="{
                  width: size.width + 'px',
                  height: size.height + 'px'
                }"
              ></div>
            </div>
            <span>{{ size.name }}</span>
            <span class="page-size-dimensions">{{ size.width }} × {{ size.height }} mm</span>
          </button>
        </div>
      </div>

      <!-- Margins -->
      <div class="dialog-section">
        <h4 class="section-title">页边距</h4>
        <div class="margin-presets">
          <button
            v-for="preset in marginPresets"
            :key="preset.id"
            class="margin-preset-btn"
            :class="{ active: selectedMarginPreset === preset.id && !useCustomMargins }"
            type="button"
            @click="handleMarginPresetChange"
          >
            {{ preset.name }}
          </button>
        </div>

        <div class="custom-margins">
          <div class="margin-input-group">
            <label for="margin-top">上</label>
            <input
              id="margin-top"
              v-model.number="customMargins.top"
              type="number"
              step="0.1"
              min="0"
              max="10"
              @input="handleCustomMarginChange"
            />
            <span>cm</span>
          </div>
          <div class="margin-input-group">
            <label for="margin-bottom">下</label>
            <input
              id="margin-bottom"
              v-model.number="customMargins.bottom"
              type="number"
              step="0.1"
              min="0"
              max="10"
              @input="handleCustomMarginChange"
            />
            <span>cm</span>
          </div>
          <div class="margin-input-group">
            <label for="margin-left">左</label>
            <input
              id="margin-left"
              v-model.number="customMargins.left"
              type="number"
              step="0.1"
              min="0"
              max="10"
              @input="handleCustomMarginChange"
            />
            <span>cm</span>
          </div>
          <div class="margin-input-group">
            <label for="margin-right">右</label>
            <input
              id="margin-right"
              v-model.number="customMargins.right"
              type="number"
              step="0.1"
              min="0"
              max="10"
              @input="handleCustomMarginChange"
            />
            <span>cm</span>
          </div>
        </div>
      </div>

      <!-- Preview -->
      <div class="dialog-section">
        <h4 class="section-title">预览</h4>
        <div class="page-preview">
          <div
            class="page-preview-inner"
            :style="{
              width: currentPageSize.width + 'px',
              height: currentPageSize.height + 'px',
              padding: `${customMargins.top * 3}px ${customMargins.right * 3}px ${customMargins.bottom * 3}px ${customMargins.left * 3}px`
            }"
          >
            <div class="preview-content">
              <div class="preview-line"></div>
              <div class="preview-line"></div>
              <div class="preview-line"></div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
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
.page-layout-dialog {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.dialog-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.section-title {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--word-text-primary);
}

/* Orientation */
.orientation-options {
  display: flex;
  gap: 16px;
}

.orientation-btn {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px;
  background: var(--word-button-bg);
  border: 2px solid var(--word-button-border);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.orientation-btn:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

.orientation-btn.active {
  background: var(--word-button-active);
  border-color: var(--word-button-pressed);
}

.orientation-btn svg {
  color: var(--word-text-primary);
}

.orientation-btn span {
  font-size: 13px;
  color: var(--word-text-primary);
}

/* Page Size */
.page-size-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
}

.page-size-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: var(--word-button-bg);
  border: 2px solid var(--word-button-border);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.page-size-btn:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

.page-size-btn.active {
  background: var(--word-button-active);
  border-color: var(--word-button-pressed);
}

.page-size-preview {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 60px;
}

.page-size-rect {
  background: var(--word-text-secondary);
  border: 1px solid var(--word-border);
  transition: all 0.15s ease;
}

.page-size-btn span {
  font-size: 12px;
  color: var(--word-text-primary);
}

.page-size-dimensions {
  font-size: 11px;
  color: var(--word-text-secondary);
}

/* Margins */
.margin-presets {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.margin-preset-btn {
  padding: 8px 16px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  color: var(--word-text-primary);
  transition: all 0.15s ease;
}

.margin-preset-btn:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

.margin-preset-btn.active {
  background: var(--word-button-active);
  border-color: var(--word-button-pressed);
}

.custom-margins {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  margin-top: 8px;
}

.margin-input-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.margin-input-group label {
  font-size: 13px;
  color: var(--word-text-primary);
  min-width: 30px;
}

.margin-input-group input {
  flex: 1;
  padding: 6px 8px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 13px;
}

.margin-input-group input:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

.margin-input-group span {
  font-size: 13px;
  color: var(--word-text-secondary);
}

/* Preview */
.page-preview {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-border);
  border-radius: 8px;
  min-height: 200px;
}

.page-preview-inner {
  background: white;
  border: 1px solid var(--word-border);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transition: all 0.15s ease;
}

.preview-content {
  display: flex;
  flex-direction: column;
  gap: 8px;
  height: 100%;
}

.preview-line {
  height: 8px;
  background: var(--word-text-secondary);
  border-radius: 2px;
  opacity: 0.3;
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
:global(.dark) .page-preview-inner {
  background: var(--word-bg-canvas);
}

:global(.dark) .preview-line {
  background: var(--word-text-primary);
}
</style>
