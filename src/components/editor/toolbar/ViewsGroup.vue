<script setup lang="ts">
import { Monitor, Globe } from 'lucide-vue-next';

interface Emits {
  (e: 'toggle-print-layout'): void;
  (e: 'toggle-web-layout'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="页面视图" @click="emit('toggle-print-layout')">
        <Monitor :size="20" />
        <span>页面视图</span>
      </button>
      <button class="ribbon-button" title="Web版式" @click="emit('toggle-web-layout')">
        <Globe :size="20" />
        <span>Web版式</span>
      </button>
    </div>
    <div class="group-label">视图</div>
  </div>
</template>
