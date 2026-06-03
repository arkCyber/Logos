<script setup lang="ts">
import { ref, watch } from 'vue';
import BaseDialog from './BaseDialog.vue';

interface Props {
  show: boolean;
  initialUrl?: string;
  initialText?: string;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'confirm', url: string, text: string): void;
}

const props = withDefaults(defineProps<Props>(), {
  initialUrl: '',
  initialText: ''
});

const emit = defineEmits<Emits>();

const url = ref(props.initialUrl);
const text = ref(props.initialText);

// Reset form when dialog opens
watch(() => props.show, (newVal) => {
  if (newVal) {
    url.value = props.initialUrl;
    text.value = props.initialText;
  }
});

const handleConfirm = () => {
  if (url.value.trim()) {
    emit('confirm', url.value.trim(), text.value.trim());
    emit('update:show', false);
  }
};

const handleCancel = () => {
  emit('update:show', false);
};

const handleRemoveLink = () => {
  emit('confirm', '', '');
  emit('update:show', false);
};
</script>

<template>
  <BaseDialog
    :show="show"
    title="插入链接"
    width="450px"
    @update:show="handleCancel"
  >
    <div class="link-dialog-content">
      <div class="form-group">
        <label class="form-label">
          链接地址
          <span class="required">*</span>
        </label>
        <input
          v-model="url"
          type="url"
          class="form-input"
          placeholder="https://example.com"
          @keyup.enter="handleConfirm"
        />
      </div>

      <div class="form-group">
        <label class="form-label">显示文本</label>
        <input
          v-model="text"
          type="text"
          class="form-input"
          placeholder="链接文本"
          @keyup.enter="handleConfirm"
        />
      </div>

      <div class="form-hint">
        提示：输入完整的 URL 地址，包括 http:// 或 https://
      </div>
    </div>

    <template #footer>
      <button class="dialog-button danger" @click="handleRemoveLink">
        移除链接
      </button>
      <div class="dialog-footer-right">
        <button class="dialog-button secondary" @click="handleCancel">
          取消
        </button>
        <button class="dialog-button primary" :disabled="!url.trim()" @click="handleConfirm">
          确定
        </button>
      </div>
    </template>
  </BaseDialog>
</template>

<style scoped>
.link-dialog-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--word-text-secondary);
}

.required {
  color: #ff4d4f;
  margin-left: 2px;
}

.form-input {
  padding: 8px 12px;
  border: 1px solid var(--word-border);
  border-radius: 4px;
  background: var(--word-bg-page);
  color: var(--word-text-primary);
  font-size: 14px;
  transition: all 0.15s ease;
}

.form-input:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
}

.form-input::placeholder {
  color: var(--word-text-secondary);
  opacity: 0.6;
}

.form-hint {
  font-size: 12px;
  color: var(--word-text-secondary);
  opacity: 0.8;
  padding: 8px 12px;
  background: var(--word-bg-canvas);
  border-radius: 4px;
  border-left: 3px solid var(--word-button-border-hover);
}

.dialog-footer-right {
  display: flex;
  gap: 8px;
  margin-left: auto;
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

.dialog-button.primary:hover:not(:disabled) {
  background: var(--word-button-primary-hover);
}

.dialog-button.primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.dialog-button.secondary {
  background: var(--word-bg-canvas);
  color: var(--word-text-primary);
  border: 1px solid var(--word-border);
}

.dialog-button.secondary:hover {
  background: var(--word-button-hover);
}

.dialog-button.danger {
  background: #fff1f0;
  color: #ff4d4f;
  border: 1px solid #ffa39e;
}

.dialog-button.danger:hover {
  background: #ffccc7;
}
</style>
