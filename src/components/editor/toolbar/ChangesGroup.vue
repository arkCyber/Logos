<script setup lang="ts">
import { Edit, Check, X } from 'lucide-vue-next';

interface Emits {
  (e: 'track-changes'): void;
  (e: 'accept-change'): void;
  (e: 'reject-change'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="修订" @click="emit('track-changes')">
        <Edit :size="20" />
        <span>修订</span>
      </button>
      <button class="ribbon-button" title="接受" @click="emit('accept-change')">
        <Check :size="20" />
        <span>接受</span>
      </button>
      <button class="ribbon-button" title="拒绝" @click="emit('reject-change')">
        <X :size="20" />
        <span>拒绝</span>
      </button>
    </div>
    <div class="group-label">更改</div>
  </div>
</template>
