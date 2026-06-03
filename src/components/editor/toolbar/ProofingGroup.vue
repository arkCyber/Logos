<script setup lang="ts">
import { CheckCircle2 } from 'lucide-vue-next';

interface Emits {
  (e: 'check-spelling'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="拼写检查" @click="emit('check-spelling')">
        <CheckCircle2 :size="20" />
        <span>拼写检查</span>
      </button>
    </div>
    <div class="group-label">校对</div>
  </div>
</template>
