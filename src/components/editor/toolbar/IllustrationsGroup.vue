<script setup lang="ts">
import {
  Image, Triangle, Maximize2, Minimize2,
  AlignLeft, AlignCenter, AlignRight, WrapText,
  Crop, RotateCw, FlipHorizontal, FlipVertical
} from 'lucide-vue-next';

interface Emits {
  (e: 'insert-image'): void;
  (e: 'insert-shape'): void;
  (e: 'resize-image', width?: number, height?: number): void;
  (e: 'align-image', alignment: 'left' | 'center' | 'right'): void;
  (e: 'wrap-image', wrap: 'inline' | 'text' | 'tight'): void;
  (e: 'crop-image'): void;
  (e: 'rotate-image', degrees: number): void;
  (e: 'flip-image', direction: 'horizontal' | 'vertical'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <!-- Row 1: Basic Insert -->
      <div class="illustrations-row">
        <button class="ribbon-button-large" title="图片" @click="emit('insert-image')">
          <Image :size="32" />
          <span>图片</span>
        </button>
        <button class="ribbon-button-large" title="形状" @click="emit('insert-shape')">
          <Triangle :size="32" />
          <span>形状</span>
        </button>
      </div>

      <!-- Row 2: Size Operations -->
      <div class="illustrations-row illustrations-buttons-compact">
        <button class="ribbon-button-small" title="放大图片" @click="emit('resize-image', undefined, undefined)">
          <Maximize2 :size="16" />
        </button>
        <button class="ribbon-button-small" title="缩小图片" @click="emit('resize-image', undefined, undefined)">
          <Minimize2 :size="16" />
        </button>
      </div>

      <!-- Row 3: Alignment Operations -->
      <div class="illustrations-row illustrations-buttons-compact">
        <button class="ribbon-button-small" title="左对齐" @click="emit('align-image', 'left')">
          <AlignLeft :size="16" />
        </button>
        <button class="ribbon-button-small" title="居中对齐" @click="emit('align-image', 'center')">
          <AlignCenter :size="16" />
        </button>
        <button class="ribbon-button-small" title="右对齐" @click="emit('align-image', 'right')">
          <AlignRight :size="16" />
        </button>
      </div>

      <!-- Row 4: Wrap Operations -->
      <div class="illustrations-row illustrations-buttons-compact">
        <button class="ribbon-button-small" title="内嵌" @click="emit('wrap-image', 'inline')">
          <WrapText :size="16" />
        </button>
        <button class="ribbon-button-small" title="四周环绕" @click="emit('wrap-image', 'text')">
          <WrapText :size="16" />
        </button>
        <button class="ribbon-button-small" title="紧密环绕" @click="emit('wrap-image', 'tight')">
          <WrapText :size="16" />
        </button>
      </div>

      <!-- Row 5: Transform Operations -->
      <div class="illustrations-row illustrations-buttons-compact">
        <button class="ribbon-button-small" title="裁剪" @click="emit('crop-image')">
          <Crop :size="16" />
        </button>
        <button class="ribbon-button-small" title="旋转90度" @click="emit('rotate-image', 90)">
          <RotateCw :size="16" />
        </button>
        <button class="ribbon-button-small" title="水平翻转" @click="emit('flip-image', 'horizontal')">
          <FlipHorizontal :size="16" />
        </button>
        <button class="ribbon-button-small" title="垂直翻转" @click="emit('flip-image', 'vertical')">
          <FlipVertical :size="16" />
        </button>
      </div>
    </div>
    <div class="group-label">
      <span>插图</span>
    </div>
  </div>
</template>

<style scoped>
.group-content {
  flex-direction: column;
  flex-wrap: nowrap;
  gap: 0;
}

.illustrations-row {
  display: flex;
  gap: 1px;
  align-items: center;
}

.illustrations-buttons-compact {
  display: flex;
  gap: 1px;
}
</style>
