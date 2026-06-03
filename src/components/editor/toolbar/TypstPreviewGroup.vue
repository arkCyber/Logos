<script setup lang="ts">
import { Eye } from 'lucide-vue-next';

interface Emits {
  (e: 'toggle-typst-preview'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="Typst 预览" @click="emit('toggle-typst-preview')">
        <Eye :size="20" />
        <span>Typst 预览</span>
      </button>
    </div>
    <div class="group-label">Typst 预览</div>
  </div>
</template>
