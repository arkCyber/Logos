<script setup lang="ts">
import { Search } from 'lucide-vue-next';

interface Emits {
  (e: 'insert-vlookup'): void;
  (e: 'insert-hlookup'): void;
  (e: 'insert-index-match'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="VLOOKUP" @click="emit('insert-vlookup')">
        <Search :size="20" />
        <span>VLOOKUP</span>
      </button>
      <button class="ribbon-button" title="HLOOKUP" @click="emit('insert-hlookup')">
        <Search :size="20" />
        <span>HLOOKUP</span>
      </button>
      <button class="ribbon-button" title="INDEX/MATCH" @click="emit('insert-index-match')">
        <Search :size="20" />
        <span>INDEX/MATCH</span>
      </button>
    </div>
    <div class="group-label">查找与引用</div>
  </div>
</template>
