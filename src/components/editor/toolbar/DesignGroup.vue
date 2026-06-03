<script setup lang="ts">
import { Palette, Image as ImageIcon, Layout } from 'lucide-vue-next';

interface Emits {
  (e: 'open-theme-dialog'): void;
  (e: 'open-background-dialog'): void;
  (e: 'open-layout-dialog'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="幻灯片主题" @click="emit('open-theme-dialog')">
        <Palette :size="20" />
        <span>主题</span>
      </button>
      <button class="ribbon-button" title="背景样式" @click="emit('open-background-dialog')">
        <ImageIcon :size="20" />
        <span>背景</span>
      </button>
      <button class="ribbon-button" title="幻灯片版式" @click="emit('open-layout-dialog')">
        <Layout :size="20" />
        <span>版式</span>
      </button>
    </div>
    <div class="group-label">设计</div>
  </div>
</template>
