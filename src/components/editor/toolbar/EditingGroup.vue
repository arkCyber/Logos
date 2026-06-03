<script setup lang="ts">
import { Search, Replace, SquareCheck, Box, FileText } from 'lucide-vue-next';

interface Emits {
  (e: 'find-text'): void;
  (e: 'replace-text'): void;
  (e: 'select-all'): void;
  (e: 'select-objects'): void;
  (e: 'select-similar-formatting'): void;
}

const emit = defineEmits<Emits>();

const selectAll = () => {
  emit('select-all');
};

const selectObjects = () => {
  emit('select-objects');
};

const selectSimilarFormatting = () => {
  emit('select-similar-formatting');
};
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="查找" @click="emit('find-text')">
        <Search :size="20" />
        <span>查找</span>
      </button>
      <button class="ribbon-button" title="替换" @click="emit('replace-text')">
        <Replace :size="20" />
        <span>替换</span>
      </button>
      <button class="ribbon-button" title="全选" @click="selectAll">
        <SquareCheck :size="20" />
        <span>全选</span>
      </button>
      <button class="ribbon-button" title="选择对象" @click="selectObjects">
        <Box :size="20" />
        <span>选择对象</span>
      </button>
      <button class="ribbon-button" title="选择格式相似的文本" @click="selectSimilarFormatting">
        <FileText :size="20" />
        <span>格式相似</span>
      </button>
    </div>
    <div class="group-label">编辑</div>
  </div>
</template>
