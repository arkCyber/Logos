<script setup lang="ts">
import { Clipboard, Scissors, Copy, Paintbrush } from 'lucide-vue-next';

interface Emits {
  (e: 'paste'): void;
  (e: 'cut'): void;
  (e: 'copy'): void;
  (e: 'format-painter'): void;
}

const emit = defineEmits<Emits>();

const pasteFromClipboard = () => {
  emit('paste');
};

const cutSelection = () => {
  emit('cut');
};

const copySelection = () => {
  emit('copy');
};

const formatPainter = () => {
  emit('format-painter');
};
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button-large" title="粘贴" @click="pasteFromClipboard">
        <Clipboard :size="32" />
        <span>粘贴</span>
      </button>
      <div class="clipboard-buttons-col">
        <button class="ribbon-button-small" title="剪切" @click="cutSelection">
          <Scissors :size="16" />
        </button>
        <button class="ribbon-button-small" title="复制" @click="copySelection">
          <Copy :size="16" />
        </button>
        <button class="ribbon-button-small" title="格式刷" @click="formatPainter">
          <Paintbrush :size="16" />
        </button>
      </div>
    </div>
    <div class="group-label">剪贴板</div>
  </div>
</template>

<style scoped>
.clipboard-buttons-col {
  display: flex;
  flex-direction: column;
  gap: 2px;
}
</style>
