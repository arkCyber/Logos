<script setup lang="ts">
import { Maximize } from 'lucide-vue-next';

interface Emits {
  (e: 'toggle-fullscreen'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="全屏" @click="emit('toggle-fullscreen')">
        <Maximize :size="20" />
        <span>全屏</span>
      </button>
    </div>
    <div class="group-label">缩放</div>
  </div>
</template>
