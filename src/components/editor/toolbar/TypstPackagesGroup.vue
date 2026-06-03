<script setup lang="ts">
import { Package } from 'lucide-vue-next';

interface Emits {
  (e: 'open-typst-package-browser'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="Typst 包" @click="emit('open-typst-package-browser')">
        <Package :size="20" />
        <span>Typst 包</span>
      </button>
    </div>
    <div class="group-label">Typst 包</div>
  </div>
</template>
