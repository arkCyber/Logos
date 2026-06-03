<script setup lang="ts">
import { BookOpen } from 'lucide-vue-next';

interface Emits {
  (e: 'insert-citation'): void;
  (e: 'insert-bibliography'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="插入引用" @click="emit('insert-citation')">
        <BookOpen :size="20" />
        <span>引用</span>
      </button>
      <button class="ribbon-button" title="参考文献" @click="emit('insert-bibliography')">
        <BookOpen :size="20" />
        <span>参考文献</span>
      </button>
    </div>
    <div class="group-label">引用</div>
  </div>
</template>
