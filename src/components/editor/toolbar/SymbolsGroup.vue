<script setup lang="ts">
import { Sigma, Smile } from 'lucide-vue-next';

interface Emits {
  (e: 'toggle-math-dialog'): void;
  (e: 'insert-emoji'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button-large" title="插入数学公式" @click="emit('toggle-math-dialog')">
        <Sigma :size="32" />
        <span>公式</span>
      </button>
      <button class="ribbon-button-large" title="插入表情符号" @click="emit('insert-emoji')">
        <Smile :size="32" />
        <span>表情</span>
      </button>
    </div>
    <div class="group-label">符号</div>
  </div>
</template>
