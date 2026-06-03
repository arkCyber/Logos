<script setup lang="ts">
import { Type } from 'lucide-vue-next';

interface Emits {
  (e: 'open-typst-font-manager'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="Typst 字体" @click="emit('open-typst-font-manager')">
        <Type :size="20" />
        <span>Typst 字体</span>
      </button>
    </div>
    <div class="group-label">Typst 字体</div>
  </div>
</template>
