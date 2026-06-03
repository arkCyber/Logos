<script setup lang="ts">
import { Link } from 'lucide-vue-next';

interface Emits {
  (e: 'add-cross-reference'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="交叉引用" @click="emit('add-cross-reference')">
        <Link :size="20" />
        <span>交叉引用</span>
      </button>
    </div>
    <div class="group-label">交叉引用</div>
  </div>
</template>
