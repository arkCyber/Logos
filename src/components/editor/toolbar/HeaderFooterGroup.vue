<script setup lang="ts">
import { Settings, Heading, Baseline, X, Hash } from 'lucide-vue-next';

interface Props {
  isEditingHeader: boolean;
  isEditingFooter: boolean;
}

interface Emits {
  (e: 'toggle-header-footer-dialog'): void;
  (e: 'enter-header-edit-mode'): void;
  (e: 'enter-footer-edit-mode'): void;
  (e: 'exit-header-footer-edit-mode'): void;
  (e: 'toggle-page-number-dialog'): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button-large" title="页眉和页脚设置" @click="emit('toggle-header-footer-dialog')">
        <Settings :size="32" />
        <span>页眉页脚</span>
      </button>
      <button class="ribbon-button-large" title="编辑页眉" @click="emit('enter-header-edit-mode')">
        <Heading :size="32" />
        <span>页眉</span>
      </button>
      <button class="ribbon-button-large" title="编辑页脚" @click="emit('enter-footer-edit-mode')">
        <Baseline :size="32" />
        <span>页脚</span>
      </button>
      <button v-if="isEditingHeader || isEditingFooter" class="ribbon-button-large" title="关闭页眉页脚编辑" @click="emit('exit-header-footer-edit-mode')">
        <X :size="32" />
        <span>关闭</span>
      </button>
      <button class="ribbon-button-large" title="插入页码" @click="emit('toggle-page-number-dialog')">
        <Hash :size="32" />
        <span>页码</span>
      </button>
    </div>
    <div class="group-label">页眉页脚</div>
  </div>
</template>
