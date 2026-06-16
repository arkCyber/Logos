<script setup lang="ts">
import { ref, watch } from 'vue';
import BaseDialog from './BaseDialog.vue';

interface Props {
  show: boolean;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'apply', options: FontDialogOptions): void;
}

interface FontDialogOptions {
  fontFamily: string;
  fontSize: number;
  bold: boolean;
  italic: boolean;
  underline: boolean;
  underlineStyle: string;
  strikethrough: boolean;
  subscript: boolean;
  superscript: boolean;
  textColor: string;
  highlightColor: string;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const fontFamilies = [
  'Calibri',
  'Arial',
  'Times New Roman',
  'SimSun',
  'Microsoft YaHei',
  'Cambria',
  'Georgia',
  'Verdana',
  'Tahoma',
  'Courier New',
  'Consolas'
];

const fontSizes = [
  8, 9, 10, 11, 12, 14, 16, 18, 20, 22, 24, 26, 28, 36, 48, 72
];

const underlineStyles = [
  { value: 'none', label: '无' },
  { value: 'single', label: '单下划线' },
  { value: 'double', label: '双下划线' },
  { value: 'dotted', label: '点下划线' },
  { value: 'wavy', label: '波浪下划线' }
];

const form = ref<FontDialogOptions>({
  fontFamily: 'Calibri',
  fontSize: 11,
  bold: false,
  italic: false,
  underline: false,
  underlineStyle: 'single',
  strikethrough: false,
  subscript: false,
  superscript: false,
  textColor: '#000000',
  highlightColor: '#ffff00'
});

watch(() => props.show, (open) => {
  if (open) {
    resetToDefaults();
  }
});

const resetToDefaults = () => {
  form.value = {
    fontFamily: 'Calibri',
    fontSize: 11,
    bold: false,
    italic: false,
    underline: false,
    underlineStyle: 'single',
    strikethrough: false,
    subscript: false,
    superscript: false,
    textColor: '#000000',
    highlightColor: '#ffff00'
  };
};

const handleApply = () => {
  emit('apply', { ...form.value });
  emit('update:show', false);
};

const handleCancel = () => {
  emit('update:show', false);
};

const handleClose = () => {
  emit('update:show', false);
};
</script>

<template>
  <BaseDialog
    :show="show"
    title="字体"
    width="400px"
    @update:show="handleClose"
  >
    <div class="font-dialog-content">
      <!-- Font Family -->
      <div class="form-group">
        <label class="form-label">字体</label>
        <select v-model="form.fontFamily" class="form-select">
          <option v-for="font in fontFamilies" :key="font" :value="font">
            {{ font }}
          </option>
        </select>
      </div>

      <!-- Font Size -->
      <div class="form-group">
        <label class="form-label">字号</label>
        <div class="size-row">
          <select v-model="form.fontSize" class="form-select size-select">
            <option v-for="size in fontSizes" :key="size" :value="size">
              {{ size }}
            </option>
          </select>
          <span class="unit">pt</span>
        </div>
      </div>

      <!-- Style Toggles -->
      <div class="form-group">
        <label class="form-label">字形</label>
        <div class="style-toggles">
          <button
            type="button"
            class="style-toggle"
            :class="{ active: form.bold }"
            title="加粗 (Ctrl+B)"
            @click="form.bold = !form.bold"
          >
            <strong>B</strong>
          </button>
          <button
            type="button"
            class="style-toggle"
            :class="{ active: form.italic }"
            title="斜体 (Ctrl+I)"
            @click="form.italic = !form.italic"
          >
            <em>I</em>
          </button>
          <button
            type="button"
            class="style-toggle"
            :class="{ active: form.underline }"
            title="下划线 (Ctrl+U)"
            @click="form.underline = !form.underline"
          >
            <span style="text-decoration: underline;">U</span>
          </button>
          <button
            type="button"
            class="style-toggle"
            :class="{ active: form.strikethrough }"
            title="删除线"
            @click="form.strikethrough = !form.strikethrough"
          >
            <span style="text-decoration: line-through;">S</span>
          </button>
          <button
            type="button"
            class="style-toggle"
            :class="{ active: form.subscript }"
            title="下标"
            @click="form.subscript = !form.subscript"
          >
            X<sub>2</sub>
          </button>
          <button
            type="button"
            class="style-toggle"
            :class="{ active: form.superscript }"
            title="上标"
            @click="form.superscript = !form.superscript"
          >
            X<sup>2</sup>
          </button>
        </div>
      </div>

      <!-- Underline Style -->
      <div class="form-group">
        <label class="form-label">下划线线型</label>
        <select v-model="form.underlineStyle" class="form-select">
          <option v-for="style in underlineStyles" :key="style.value" :value="style.value">
            {{ style.label }}
          </option>
        </select>
      </div>

      <!-- Colors -->
      <div class="form-group">
        <label class="form-label">字体颜色</label>
        <div class="color-row">
          <input
            v-model="form.textColor"
            type="color"
            class="color-input"
          />
          <span class="color-value">{{ form.textColor }}</span>
        </div>
      </div>

      <div class="form-group">
        <label class="form-label">突出显示颜色</label>
        <div class="color-row">
          <input
            v-model="form.highlightColor"
            type="color"
            class="color-input"
          />
          <span class="color-value">{{ form.highlightColor }}</span>
        </div>
      </div>

      <!-- Effects Preview -->
      <div class="form-group">
        <label class="form-label">预览</label>
        <div class="preview-box">
          <span
            :style="{
              fontFamily: form.fontFamily,
              fontSize: form.fontSize + 'pt',
              fontWeight: form.bold ? 'bold' : 'normal',
              fontStyle: form.italic ? 'italic' : 'normal',
              textDecoration: form.underline ? 'underline' : (form.strikethrough ? 'line-through' : 'none'),
              color: form.textColor,
              backgroundColor: form.highlightColor
            }"
          >
            Aa Bb Cc 文字示例
          </span>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <button class="btn btn-secondary" @click="handleCancel">取消</button>
        <button class="btn btn-primary" @click="handleApply">确定</button>
      </div>
    </template>
  </BaseDialog>
</template>

<style scoped>
.font-dialog-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 8px 0;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--word-text-primary, #333);
}

.form-select {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--word-border, #ccc);
  border-radius: 4px;
  font-size: 13px;
  background: var(--word-bg-primary, #fff);
  color: var(--word-text-primary, #333);
  cursor: pointer;
}

.form-select:focus {
  outline: none;
  border-color: var(--word-accent, #0078d4);
  box-shadow: 0 0 0 1px var(--word-accent, #0078d4);
}

.size-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.size-select {
  width: 80px;
}

.unit {
  font-size: 13px;
  color: var(--word-text-secondary, #666);
}

.style-toggles {
  display: flex;
  gap: 4px;
}

.style-toggle {
  width: 36px;
  height: 32px;
  border: 1px solid var(--word-border, #ccc);
  border-radius: 4px;
  background: var(--word-bg-primary, #fff);
  color: var(--word-text-primary, #333);
  font-size: 14px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.style-toggle:hover {
  background: var(--word-bg-hover, #f0f0f0);
}

.style-toggle.active {
  background: var(--word-accent, #0078d4);
  border-color: var(--word-accent, #0078d4);
  color: #fff;
}

.color-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.color-input {
  width: 40px;
  height: 32px;
  padding: 2px;
  border: 1px solid var(--word-border, #ccc);
  border-radius: 4px;
  cursor: pointer;
}

.color-value {
  font-family: 'Courier New', monospace;
  font-size: 12px;
  color: var(--word-text-secondary, #666);
}

.preview-box {
  padding: 12px 16px;
  border: 1px solid var(--word-border, #ccc);
  border-radius: 4px;
  background: var(--word-bg-secondary, #f9f9f9);
  min-height: 40px;
  display: flex;
  align-items: center;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding-top: 8px;
}

.btn {
  padding: 8px 20px;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border: 1px solid transparent;
}

.btn-primary {
  background: var(--word-accent, #0078d4);
  color: #fff;
  border-color: var(--word-accent, #0078d4);
}

.btn-primary:hover {
  background: #106ebe;
}

.btn-secondary {
  background: var(--word-bg-primary, #fff);
  color: var(--word-text-primary, #333);
  border-color: var(--word-border, #ccc);
}

.btn-secondary:hover {
  background: var(--word-bg-hover, #f0f0f0);
}
</style>
