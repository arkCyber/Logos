<script setup lang="ts">
import { Moon, Image } from 'lucide-vue-next';

interface Emits {
  (e: 'toggle-theme'): void;
  (e: 'toggle-wallpaper'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="主题" @click="emit('toggle-theme')">
        <Moon :size="20" />
        <span>主题</span>
      </button>
      <button class="ribbon-button" title="墙纸" @click="emit('toggle-wallpaper')">
        <Image :size="20" />
        <span>墙纸</span>
      </button>
    </div>
    <div class="group-label">窗口</div>
  </div>
</template>
