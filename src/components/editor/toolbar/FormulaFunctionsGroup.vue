<script setup lang="ts">
import { Library } from 'lucide-vue-next';

interface Emits {
  (e: 'insert-formula'): void;
  (e: 'open-function-library'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="插入公式" @click="emit('insert-formula')">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="M6 3v18" />
          <path d="M6 9h12" />
          <path d="M6 15h12" />
        </svg>
        <span>插入公式</span>
      </button>
      <button class="ribbon-button" title="函数库" @click="emit('open-function-library')">
        <Library :size="20" />
        <span>函数库</span>
      </button>
    </div>
    <div class="group-label">公式与函数</div>
  </div>
</template>
