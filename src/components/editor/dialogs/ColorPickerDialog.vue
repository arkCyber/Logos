<script setup lang="ts">
import { ref } from 'vue';
import BaseDialog from './BaseDialog.vue';

interface Props {
  show: boolean;
  initialColor?: string;
  title?: string;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'confirm', color: string): void;
}

const props = withDefaults(defineProps<Props>(), {
  initialColor: '#000000',
  title: '选择颜色'
});

const emit = defineEmits<Emits>();

const selectedColor = ref(props.initialColor);
const customColor = ref(props.initialColor);

// Predefined colors
const predefinedColors = [
  '#000000', '#434343', '#666666', '#999999', '#b7b7b7', '#cccccc', '#d9d9d9', '#efefef', '#ffffff',
  '#ff0000', '#ff9900', '#ffff00', '#00ff00', '#00ffff', '#0000ff', '#9900ff', '#ff00ff',
  '#f4cccc', '#fce5cd', '#fff2cc', '#d9ead3', '#d0e0e3', '#cfe2f3', '#d9d2e9', '#ead1dc',
  '#ea9999', '#f9cb9c', '#ffe599', '#b6d7a8', '#a2c4c9', '#9fc5e8', '#b4a7d6', '#d5a6bd',
  '#e06666', '#f6b26b', '#ffd966', '#93c47d', '#76a5af', '#6fa8dc', '#8e7cc3', '#c27ba0',
  '#cc0000', '#e69138', '#f1c232', '#6aa84f', '#45818e', '#3d85c6', '#674ea7', '#a64d79',
  '#990000', '#b45f06', '#bf9000', '#38761d', '#134f5c', '#1155cc', '#351c75', '#741b47',
  '#660000', '#783f04', '#7f6000', '#274e13', '#0c343d', '#1c4587', '#20124d', '#4c1130'
];

const handleColorSelect = (color: string) => {
  selectedColor.value = color;
  customColor.value = color;
};

const handleCustomColorChange = (event: Event) => {
  const target = event.target as HTMLInputElement;
  customColor.value = target.value;
  selectedColor.value = target.value;
};

const handleConfirm = () => {
  emit('confirm', selectedColor.value);
  emit('update:show', false);
};

const handleCancel = () => {
  emit('update:show', false);
};
</script>

<template>
  <BaseDialog
    :show="show"
    :title="title"
    width="400px"
    @update:show="handleCancel"
  >
    <div class="color-picker-content">
      <!-- Color Preview -->
      <div class="color-preview">
        <div
          class="color-swatch"
          :style="{ backgroundColor: selectedColor }"
        ></div>
        <input
          v-model="selectedColor"
          type="text"
          class="color-input"
          maxlength="7"
        />
      </div>

      <!-- Custom Color Picker -->
      <div class="custom-color-section">
        <label class="section-label">自定义颜色</label>
        <input
          v-model="customColor"
          type="color"
          class="color-picker-input"
          @input="handleCustomColorChange"
        />
      </div>

      <!-- Predefined Colors -->
      <div class="predefined-colors-section">
        <label class="section-label">预设颜色</label>
        <div class="color-grid">
          <div
            v-for="color in predefinedColors"
            :key="color"
            class="color-item"
            :class="{ selected: selectedColor === color }"
            :style="{ backgroundColor: color }"
            @click="handleColorSelect(color)"
          ></div>
        </div>
      </div>
    </div>

    <template #footer>
      <button class="dialog-button secondary" @click="handleCancel">
        取消
      </button>
      <button class="dialog-button primary" @click="handleConfirm">
        确定
      </button>
    </template>
  </BaseDialog>
</template>

<style scoped>
.color-picker-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.color-preview {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: var(--word-bg-canvas);
  border-radius: 4px;
  border: 1px solid var(--word-border);
}

.color-swatch {
  width: 48px;
  height: 48px;
  border-radius: 4px;
  border: 1px solid var(--word-border);
  flex-shrink: 0;
}

.color-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--word-border);
  border-radius: 4px;
  background: var(--word-bg-page);
  color: var(--word-text-primary);
  font-family: monospace;
  font-size: 14px;
}

.custom-color-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.section-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--word-text-secondary);
}

.color-picker-input {
  width: 100%;
  height: 40px;
  border: 1px solid var(--word-border);
  border-radius: 4px;
  cursor: pointer;
  padding: 4px;
}

.predefined-colors-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.color-grid {
  display: grid;
  grid-template-columns: repeat(9, 1fr);
  gap: 4px;
}

.color-item {
  width: 32px;
  height: 32px;
  border-radius: 4px;
  border: 1px solid var(--word-border);
  cursor: pointer;
  transition: all 0.15s ease;
}

.color-item:hover {
  transform: scale(1.1);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

.color-item.selected {
  border: 2px solid var(--word-button-border-hover);
  box-shadow: 0 0 0 2px var(--word-bg-page);
}

.dialog-button {
  padding: 8px 20px;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border: none;
}

.dialog-button.primary {
  background: var(--word-button-primary);
  color: white;
}

.dialog-button.primary:hover {
  background: var(--word-button-primary-hover);
}

.dialog-button.secondary {
  background: var(--word-bg-canvas);
  color: var(--word-text-primary);
  border: 1px solid var(--word-border);
}

.dialog-button.secondary:hover {
  background: var(--word-button-hover);
}
</style>
