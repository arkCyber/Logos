<script setup lang="ts">
import { Link, Bookmark, MessageSquare } from 'lucide-vue-next';

interface Emits {
  (e: 'insert-link'): void;
  (e: 'insert-bookmark'): void;
  (e: 'add-comment'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button-large" title="插入超链接" @click="emit('insert-link')">
        <Link :size="32" />
        <span>链接</span>
      </button>
      <button class="ribbon-button-large" title="插入书签" @click="emit('insert-bookmark')">
        <Bookmark :size="32" />
        <span>书签</span>
      </button>
      <button class="ribbon-button-large" title="添加批注" @click="emit('add-comment')">
        <MessageSquare :size="32" />
        <span>批注</span>
      </button>
    </div>
    <div class="group-label">链接与批注</div>
  </div>
</template>
