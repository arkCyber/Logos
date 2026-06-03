<script setup lang="ts">
interface Emits {
  (e: 'set-columns', count: number): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="一栏" aria-label="一栏" @click="emit('set-columns', 1)">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="18" height="18" rx="2" />
        </svg>
        <span>一栏</span>
      </button>
      <button class="ribbon-button" title="两栏" aria-label="两栏" @click="emit('set-columns', 2)">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="18" height="18" rx="2" />
          <line x1="12" y1="3" x2="12" y2="21" />
        </svg>
        <span>两栏</span>
      </button>
      <button class="ribbon-button" title="三栏" aria-label="三栏" @click="emit('set-columns', 3)">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="18" height="18" rx="2" />
          <line x1="8" y1="3" x2="8" y2="21" />
          <line x1="16" y1="3" x2="16" y2="21" />
        </svg>
        <span>三栏</span>
      </button>
    </div>
    <div class="group-label">分栏</div>
  </div>
</template>
