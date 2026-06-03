<script setup lang="ts">
import {
  Table, Plus, Minus, SplitSquareHorizontal,
  Trash2, ArrowUp, ArrowDown,
  ArrowLeft, ArrowRight, Merge
} from 'lucide-vue-next';

interface Emits {
  (e: 'insert-table'): void;
  (e: 'delete-table'): void;
  (e: 'add-column-before'): void;
  (e: 'add-column-after'): void;
  (e: 'delete-column'): void;
  (e: 'add-row-before'): void;
  (e: 'add-row-after'): void;
  (e: 'delete-row'): void;
  (e: 'merge-cells'): void;
  (e: 'split-cell'): void;
  (e: 'toggle-header-row'): void;
  (e: 'toggle-header-column'): void;
  (e: 'toggle-header-cell'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <!-- Row 1: Basic Table Operations -->
      <div class="table-row">
        <button class="ribbon-button-large" title="插入表格" @click="emit('insert-table')">
          <Table :size="32" />
          <span>表格</span>
        </button>
        <button class="ribbon-button-small" title="删除表格" @click="emit('delete-table')">
          <Trash2 :size="16" />
        </button>
      </div>

      <!-- Row 2: Column Operations -->
      <div class="table-row table-buttons-compact">
        <button class="ribbon-button-small" title="在左侧插入列" @click="emit('add-column-before')">
          <ArrowLeft :size="16" />
        </button>
        <button class="ribbon-button-small" title="在右侧插入列" @click="emit('add-column-after')">
          <ArrowRight :size="16" />
        </button>
        <button class="ribbon-button-small" title="删除列" @click="emit('delete-column')">
          <Minus :size="16" />
        </button>
      </div>

      <!-- Row 3: Row Operations -->
      <div class="table-row table-buttons-compact">
        <button class="ribbon-button-small" title="在上方插入行" @click="emit('add-row-before')">
          <ArrowUp :size="16" />
        </button>
        <button class="ribbon-button-small" title="在下方插入行" @click="emit('add-row-after')">
          <ArrowDown :size="16" />
        </button>
        <button class="ribbon-button-small" title="删除行" @click="emit('delete-row')">
          <Minus :size="16" />
        </button>
      </div>

      <!-- Row 4: Cell Operations -->
      <div class="table-row table-buttons-compact">
        <button class="ribbon-button-small" title="合并单元格" @click="emit('merge-cells')">
          <Merge :size="16" />
        </button>
        <button class="ribbon-button-small" title="拆分单元格" @click="emit('split-cell')">
          <SplitSquareHorizontal :size="16" />
        </button>
      </div>

      <!-- Row 5: Header Operations -->
      <div class="table-row table-buttons-compact">
        <button class="ribbon-button-small" title="切换表头行" @click="emit('toggle-header-row')">
          <Plus :size="16" />
        </button>
        <button class="ribbon-button-small" title="切换表头列" @click="emit('toggle-header-column')">
          <Plus :size="16" />
        </button>
        <button class="ribbon-button-small" title="切换表头单元格" @click="emit('toggle-header-cell')">
          <Plus :size="16" />
        </button>
      </div>
    </div>
    <div class="group-label">
      <span>表格</span>
    </div>
  </div>
</template>

<style scoped>
.group-content {
  flex-direction: column;
  flex-wrap: nowrap;
  gap: 0;
}

.table-row {
  display: flex;
  gap: 1px;
  align-items: center;
}

.table-buttons-compact {
  display: flex;
  gap: 1px;
}
</style>
