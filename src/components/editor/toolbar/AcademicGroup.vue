<script setup lang="ts">
import { Quote, Library } from 'lucide-vue-next';

interface Emits {
  (e: 'insert-footnote'): void;
  (e: 'insert-bibliography'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="插入脚注" @click="emit('insert-footnote')">
        <Quote :size="20" />
        <span>脚注</span>
      </button>
      <button class="ribbon-button" title="插入参考文献" @click="emit('insert-bibliography')">
        <Library :size="20" />
        <span>参考文献</span>
      </button>
    </div>
    <div class="group-label">学术引用</div>
  </div>
</template>
