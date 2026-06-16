<script setup lang="ts">
import { ref, computed } from 'vue';
import BaseDialog from './BaseDialog.vue';

interface Props {
  show: boolean;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'insert-wordart', wordart: WordArtDefinition): void;
}

interface WordArtDefinition {
  text: string;
  style: string;
  fill: string;
  stroke: string;
  strokeWidth: number;
  fontSize: number;
  fontFamily: string;
  letterSpacing: number;
  transform: string;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

// WordArt text
const wordartText = ref('WordArt');

// WordArt styles
const activeStyle = ref('arch-up');

const wordartStyles = [
  { id: 'arch-up', name: '拱形向上', transform: 'rotate(-180deg)' },
  { id: 'arch-down', name: '拱形向下', transform: 'rotate(0deg)' },
  { id: 'circle', name: '圆形', transform: 'rotate(0deg)' },
  { id: 'wave', name: '波浪', transform: 'rotate(0deg)' },
  { id: 'double-wave', name: '双波浪', transform: 'rotate(0deg)' },
  { id: 'fade-up', name: '渐变向上', transform: 'rotate(0deg)' },
  { id: 'fade-down', name: '渐变向下', transform: 'rotate(0deg)' },
  { id: 'slant-up', name: '倾斜向上', transform: 'rotate(-10deg)' },
  { id: 'slant-down', name: '倾斜向下', transform: 'rotate(10deg)' },
  { id: 'inflated', name: '膨胀', transform: 'scale(1.1)' },
  { id: 'deflated', name: '收缩', transform: 'scale(0.9)' },
  { id: 'perspective', name: '透视', transform: 'perspective(500px) rotateX(10deg)' }
];

// Fill color
const fillColor = ref('#0078d4');

// Stroke color
const strokeColor = ref('#ffffff');

// Stroke width
const strokeWidth = ref(1);

// Font size
const fontSize = ref(48);

// Font family
const fontFamily = ref('Arial');

const fontFamilies = [
  { id: 'Arial', name: 'Arial' },
  { id: 'Times New Roman', name: 'Times New Roman' },
  { id: 'Georgia', name: 'Georgia' },
  { id: 'Verdana', name: 'Verdana' },
  { id: 'Comic Sans MS', name: 'Comic Sans MS' },
  { id: 'Impact', name: 'Impact' },
  { id: 'Trebuchet MS', name: 'Trebuchet MS' }
];

// Letter spacing
const letterSpacing = ref(0);

// Computed preview style
const previewStyle = computed(() => {
  const style = wordartStyles.find(s => s.id === activeStyle.value);
  return {
    color: fillColor.value,
    WebkitTextStroke: `${strokeWidth.value}px ${strokeColor.value}`,
    fontSize: `${fontSize.value}px`,
    fontFamily: fontFamily.value,
    letterSpacing: `${letterSpacing.value}px`,
    transform: style?.transform || 'none',
    textShadow: getTransformEffect(activeStyle.value)
  };
});

// Get transform effect based on style
const getTransformEffect = (styleId: string): string => {
  switch (styleId) {
    case 'arch-up':
      return '0 -10px 10px rgba(0,0,0,0.3)';
    case 'arch-down':
      return '0 10px 10px rgba(0,0,0,0.3)';
    case 'wave':
      return '0 0 20px rgba(0,0,0,0.2)';
    case 'circle':
      return '0 0 30px rgba(0,0,0,0.3)';
    default:
      return 'none';
  }
};

// Insert WordArt
const insertWordArt = () => {
  const wordart: WordArtDefinition = {
    text: wordartText.value,
    style: activeStyle.value,
    fill: fillColor.value,
    stroke: strokeColor.value,
    strokeWidth: strokeWidth.value,
    fontSize: fontSize.value,
    fontFamily: fontFamily.value,
    letterSpacing: letterSpacing.value,
    transform: previewStyle.value.transform
  };
  emit('insert-wordart', wordart);
  emit('update:show', false);
};

// Cancel
const cancel = () => {
  emit('update:show', false);
};
</script>

<template>
  <BaseDialog
    :show="show"
    title="艺术字"
    width="600px"
    height="550px"
    @update:show="cancel"
  >
    <div class="wordart-dialog">
      <!-- Text input -->
      <div class="dialog-section">
        <h4 class="section-title">文本</h4>
        <input
          v-model="wordartText"
          type="text"
          placeholder="输入艺术字文本"
          class="text-input"
        />
      </div>

      <!-- Style selection -->
      <div class="dialog-section">
        <h4 class="section-title">样式</h4>
        <div class="styles-grid">
          <div
            v-for="style in wordartStyles"
            :key="style.id"
            class="style-item"
            :class="{ active: activeStyle === style.id }"
            @click="activeStyle = style.id"
          >
            <div class="style-preview">
              <span
                :style="{
                  fontSize: '16px',
                  transform: style.transform,
                  color: '#0078d4'
                }"
              >
                Aa
              </span>
            </div>
            <span class="style-name">{{ style.name }}</span>
          </div>
        </div>
      </div>

      <!-- Font settings -->
      <div class="dialog-section">
        <h4 class="section-title">字体设置</h4>
        <div class="font-settings">
          <div class="setting-row">
            <label>字体大小</label>
            <input
              v-model.number="fontSize"
              type="number"
              min="12"
              max="120"
              class="number-input"
            />
          </div>
          <div class="setting-row">
            <label>字体</label>
            <select v-model="fontFamily" class="select-input">
              <option v-for="font in fontFamilies" :key="font.id" :value="font.id">
                {{ font.name }}
              </option>
            </select>
          </div>
          <div class="setting-row">
            <label>字间距</label>
            <input
              v-model.number="letterSpacing"
              type="number"
              min="-10"
              max="20"
              class="number-input"
            />
          </div>
        </div>
      </div>

      <!-- Color settings -->
      <div class="dialog-section">
        <h4 class="section-title">颜色设置</h4>
        <div class="color-settings">
          <div class="setting-row">
            <label>填充颜色</label>
            <input
              v-model="fillColor"
              type="color"
              class="color-input"
            />
          </div>
          <div class="setting-row">
            <label>描边颜色</label>
            <input
              v-model="strokeColor"
              type="color"
              class="color-input"
            />
          </div>
          <div class="setting-row">
            <label>描边宽度</label>
            <input
              v-model.number="strokeWidth"
              type="number"
              min="0"
              max="10"
              class="number-input"
            />
          </div>
        </div>
      </div>

      <!-- Preview -->
      <div class="dialog-section">
        <h4 class="section-title">预览</h4>
        <div class="preview-area">
          <div class="preview-content" :style="previewStyle">
            {{ wordartText }}
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <button class="dialog-btn secondary" type="button" @click="cancel">
        取消
      </button>
      <button class="dialog-btn primary" type="button" @click="insertWordArt">
        插入
      </button>
    </template>
  </BaseDialog>
</template>

<style scoped>
.wordart-dialog {
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: 100%;
  overflow-y: auto;
}

.dialog-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.section-title {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--word-text-primary);
}

/* Text input */
.text-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 14px;
}

.text-input:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

/* Styles grid */
.styles-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
  gap: 8px;
}

.style-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
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
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  background: white;
  border: 1px solid var(--word-border);
  border-radius: 4px;
}

.style-name {
  font-size: 11px;
  color: var(--word-text-secondary);
  text-align: center;
}

/* Font settings */
.font-settings,
.color-settings {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.setting-row label {
  min-width: 80px;
  font-size: 13px;
  color: var(--word-text-primary);
}

.number-input,
.select-input {
  flex: 1;
  padding: 6px 10px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 13px;
}

.number-input:focus,
.select-input:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

.color-input {
  width: 50px;
  height: 32px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
}

/* Preview */
.preview-area {
  padding: 20px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-border);
  border-radius: 4px;
  min-height: 100px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.preview-content {
  font-weight: bold;
  text-align: center;
  transition: all 0.15s ease;
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
:global(.dark) .style-preview {
  background: var(--word-bg-canvas);
}
</style>
