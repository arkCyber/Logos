<script setup lang="ts">
import { Monitor } from 'lucide-vue-next';

interface Emits {
  (e: 'toggle-slide-mode'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="幻灯片模式" @click="emit('toggle-slide-mode')">
        <Monitor :size="20" />
        <span>幻灯片模式</span>
      </button>
    </div>
    <div class="group-label">幻灯片模式</div>
  </div>
</template>
