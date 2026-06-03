<script setup lang="ts">
import { MessageSquare } from 'lucide-vue-next';

interface Emits {
  (e: 'add-comment'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="新建批注" @click="emit('add-comment')">
        <MessageSquare :size="20" />
        <span>新建批注</span>
      </button>
    </div>
    <div class="group-label">批注</div>
  </div>
</template>
