<script setup lang="ts">
import { ArrowDown, Filter } from 'lucide-vue-next';

interface Emits {
  (e: 'sort-data'): void;
  (e: 'filter-data'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="排序" @click="emit('sort-data')">
        <ArrowDown :size="20" />
        <span>排序</span>
      </button>
      <button class="ribbon-button" title="筛选" @click="emit('filter-data')">
        <Filter :size="20" />
        <span>筛选</span>
      </button>
    </div>
    <div class="group-label">数据</div>
  </div>
</template>
