<script setup lang="ts">
import { ref } from 'vue';

interface Props {
  show: boolean;
  tableSelected: boolean;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'apply-style', style: string): void;
  (e: 'apply-border', border: BorderStyle): void;
  (e: 'apply-shading', shading: string): void;
}

interface BorderStyle {
  position: 'all' | 'top' | 'bottom' | 'left' | 'right' | 'horizontal' | 'vertical' | 'none';
  style: 'solid' | 'dashed' | 'dotted' | 'double';
  width: number;
  color: string;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

// Table styles
const activeStyle = ref('default');

const tableStyles = [
  { id: 'default', name: '默认网格', preview: 'grid' },
  { id: 'grid-table', name: '网格表', preview: 'grid' },
  { id: 'light-shading', name: '浅色底纹', preview: 'shading' },
  { id: 'medium-shading', name: '中度底纹', preview: 'shading-medium' },
  { id: 'list-table', name: '列表表', preview: 'list' },
  { id: 'no-border', name: '无边框', preview: 'none' }
];

// Border settings
const borderPosition = ref('all');
const borderStyle = ref('solid');
const borderWidth = ref(1);
const borderColor = ref('#000000');

const borderPositions = [
  { id: 'all', name: '全部边框' },
  { id: 'top', name: '上边框' },
  { id: 'bottom', name: '下边框' },
  { id: 'left', name: '左边框' },
  { id: 'right', name: '右边框' },
  { id: 'horizontal', name: '水平边框' },
  { id: 'vertical', name: '垂直边框' },
  { id: 'none', name: '无边框' }
];

const borderStyles = [
  { id: 'solid', name: '实线' },
  { id: 'dashed', name: '虚线' },
  { id: 'dotted', name: '点线' },
  { id: 'double', name: '双线' }
];

// Shading
const shadingColor = ref('#f0f0f0');

const shadingPresets = [
  { id: 'none', name: '无填充', color: 'transparent' },
  { id: 'light-gray', name: '浅灰', color: '#f0f0f0' },
  { id: 'medium-gray', name: '中灰', color: '#d0d0d0' },
  { id: 'dark-gray', name: '深灰', color: '#a0a0a0' },
  { id: 'light-blue', name: '浅蓝', color: '#e6f3ff' },
  { id: 'light-green', name: '浅绿', color: '#e6ffe6' },
  { id: 'light-yellow', name: '浅黄', color: '#ffffe6' },
  { id: 'light-red', name: '浅红', color: '#ffe6e6' }
];

// Apply table style
const applyTableStyle = (styleId: string) => {
  emit('apply-style', styleId);
};

// Apply border
const applyBorder = () => {
  const border: BorderStyle = {
    position: borderPosition.value as any,
    style: borderStyle.value as any,
    width: borderWidth.value,
    color: borderColor.value
  };
  emit('apply-border', border);
};

// Apply shading
const applyShading = (color: string) => {
  shadingColor.value = color;
  emit('apply-shading', color);
};

// Close tab
const closeTab = () => {
  emit('update:show', false);
};
</script>

<template>
  <Transition name="fade">
    <div v-if="show && tableSelected" class="table-design-tab">
      <!-- Table Styles -->
      <div class="tab-section">
        <h4 class="section-title">表格样式</h4>
        <div class="styles-grid">
          <div
            v-for="style in tableStyles"
            :key="style.id"
            class="style-item"
            :class="{ active: activeStyle === style.id }"
            @click="activeStyle = style.id; applyTableStyle(style.id)"
          >
            <div class="style-preview">
              <div class="preview-table" :class="`preview-${style.preview}`">
                <div class="preview-row">
                  <div class="preview-cell"></div>
                  <div class="preview-cell"></div>
                  <div class="preview-cell"></div>
                </div>
                <div class="preview-row">
                  <div class="preview-cell"></div>
                  <div class="preview-cell"></div>
                  <div class="preview-cell"></div>
                </div>
              </div>
            </div>
            <span class="style-name">{{ style.name }}</span>
          </div>
        </div>
      </div>

      <!-- Border Settings -->
      <div class="tab-section">
        <h4 class="section-title">边框</h4>
        <div class="border-settings">
          <div class="setting-row">
            <label>位置:</label>
            <select v-model="borderPosition" class="setting-select">
              <option v-for="pos in borderPositions" :key="pos.id" :value="pos.id">
                {{ pos.name }}
              </option>
            </select>
          </div>
          <div class="setting-row">
            <label>样式:</label>
            <select v-model="borderStyle" class="setting-select">
              <option v-for="style in borderStyles" :key="style.id" :value="style.id">
                {{ style.name }}
              </option>
            </select>
          </div>
          <div class="setting-row">
            <label>宽度:</label>
            <input
              v-model.number="borderWidth"
              type="number"
              min="0.5"
              max="5"
              step="0.5"
              class="setting-input"
            />
            <span>pt</span>
          </div>
          <div class="setting-row">
            <label>颜色:</label>
            <input
              v-model="borderColor"
              type="color"
              class="setting-color"
            />
          </div>
          <button class="apply-btn" type="button" @click="applyBorder">
            应用边框
          </button>
        </div>
      </div>

      <!-- Shading -->
      <div class="tab-section">
        <h4 class="section-title">底纹</h4>
        <div class="shading-presets">
          <div
            v-for="preset in shadingPresets"
            :key="preset.id"
            class="shading-item"
            :class="{ active: shadingColor === preset.color }"
            @click="applyShading(preset.color)"
          >
            <div
              class="shading-preview"
              :style="{ backgroundColor: preset.color }"
            ></div>
            <span class="shading-name">{{ preset.name }}</span>
          </div>
        </div>
        <div class="custom-shading">
          <label>自定义颜色:</label>
          <input
            v-model="shadingColor"
            type="color"
            class="setting-color"
            @input="applyShading(shadingColor)"
          />
        </div>
      </div>

      <!-- Close button -->
      <button class="close-tab-btn" type="button" @click="closeTab">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <line x1="18" y1="6" x2="6" y2="18" />
          <line x1="6" y1="6" x2="18" y2="18" />
        </svg>
      </button>
    </div>
  </Transition>
</template>

<style scoped>
.table-design-tab {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: var(--word-bg-page);
  border: 1px solid var(--word-border);
  border-radius: 0 0 4px 4px;
  padding: 16px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 100;
  max-height: 400px;
  overflow-y: auto;
}

.tab-section {
  margin-bottom: 16px;
}

.section-title {
  margin: 0 0 12px 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--word-text-primary);
}

/* Styles grid */
.styles-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 8px;
}

.style-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 8px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.style-item:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

.style-item.active {
  background: var(--word-button-active);
  border-color: var(--word-button-pressed);
}

.style-preview {
  width: 80px;
  height: 50px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.preview-table {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.preview-row {
  display: flex;
  gap: 2px;
}

.preview-cell {
  width: 20px;
  height: 12px;
  background: white;
  border: 1px solid #ccc;
}

.preview-grid .preview-cell {
  border: 1px solid #999;
}

.preview-shading .preview-cell {
  background: #f0f0f0;
  border: 1px solid #ccc;
}

.preview-shading-medium .preview-cell {
  background: #d0d0d0;
  border: 1px solid #999;
}

.preview-list .preview-row:nth-child(1) .preview-cell {
  background: #f0f0f0;
  border: 1px solid #ccc;
}

.preview-none .preview-cell {
  border: none;
  background: white;
}

.style-name {
  font-size: 11px;
  color: var(--word-text-secondary);
  text-align: center;
}

/* Border settings */
.border-settings {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.setting-row label {
  min-width: 50px;
  font-size: 12px;
  color: var(--word-text-primary);
}

.setting-select,
.setting-input {
  flex: 1;
  padding: 6px 10px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 12px;
}

.setting-select:focus,
.setting-input:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

.setting-color {
  width: 40px;
  height: 28px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
}

.apply-btn {
  padding: 8px 16px;
  background: var(--word-button-active);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  color: var(--word-text-primary);
  transition: all 0.15s ease;
  margin-top: 4px;
}

.apply-btn:hover {
  background: var(--word-button-pressed);
}

/* Shading */
.shading-presets {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(60px, 1fr));
  gap: 8px;
  margin-bottom: 12px;
}

.shading-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  cursor: pointer;
}

.shading-preview {
  width: 40px;
  height: 40px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  transition: all 0.15s ease;
}

.shading-item:hover .shading-preview {
  border-color: var(--word-button-border-hover);
}

.shading-item.active .shading-preview {
  border-color: var(--word-button-pressed);
  box-shadow: 0 0 0 2px var(--word-button-pressed);
}

.shading-name {
  font-size: 11px;
  color: var(--word-text-secondary);
}

.custom-shading {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
}

.custom-shading label {
  font-size: 12px;
  color: var(--word-text-primary);
}

/* Close button */
.close-tab-btn {
  position: absolute;
  top: 8px;
  right: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  color: var(--word-text-secondary);
  transition: all 0.15s ease;
}

.close-tab-btn:hover {
  background: var(--word-button-hover);
  color: var(--word-text-primary);
}

/* Transition */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* Dark mode */
:global(.dark) .table-design-tab {
  background: var(--word-bg-canvas);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
}

:global(.dark) .preview-cell {
  background: var(--word-bg-canvas);
  border-color: var(--word-border);
}
</style>
