<script setup lang="ts">
import { ref, computed } from 'vue';
import BaseDialog from './BaseDialog.vue';

interface Props {
  show: boolean;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'insert-shape', shape: ShapeDefinition): void;
}

interface ShapeDefinition {
  type: string;
  name: string;
  svg: string;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// Shape categories
const activeCategory = ref('basic');

const shapeCategories = [
  { id: 'basic', name: '基本形状' },
  { id: 'lines', name: '线条' },
  { id: 'rectangles', name: '矩形' },
  { id: 'ovals', name: '椭圆' },
  { id: 'arrows', name: '箭头' },
  { id: 'flowchart', name: '流程图' },
  { id: 'stars', name: '星形' },
  { id: 'callouts', name: '标注' }
];

// Shape definitions
const shapes = ref<Record<string, ShapeDefinition[]>>({
  basic: [
    { type: 'rectangle', name: '矩形', svg: '<rect x="2" y="2" width="20" height="20" rx="2" />' },
    { type: 'rounded-rectangle', name: '圆角矩形', svg: '<rect x="2" y="2" width="20" height="20" rx="5" />' },
    { type: 'square', name: '正方形', svg: '<rect x="5" y="5" width="14" height="14" rx="2" />' },
    { type: 'parallelogram', name: '平行四边形', svg: '<polygon points="4,2 20,2 16,22 0,22" />' },
    { type: 'trapezoid', name: '梯形', svg: '<polygon points="4,2 20,2 16,22 8,22" />' },
    { type: 'diamond', name: '菱形', svg: '<polygon points="12,2 22,12 12,22 2,12" />' },
    { type: 'pentagon', name: '五边形', svg: '<polygon points="12,2 22,8 19,22 5,22 2,8" />' },
    { type: 'hexagon', name: '六边形', svg: '<polygon points="12,2 22,8 22,16 12,22 2,16 2,8" />' },
    { type: 'octagon', name: '八边形', svg: '<polygon points="8,2 16,2 22,8 22,16 16,22 8,22 2,16 2,8" />' },
    { type: 'triangle', name: '三角形', svg: '<polygon points="12,2 22,22 2,22" />' },
    { type: 'right-triangle', name: '直角三角形', svg: '<polygon points="2,2 22,22 2,22" />' },
    { type: 'ellipse', name: '椭圆', svg: '<ellipse cx="12" cy="12" rx="10" ry="6" />' }
  ],
  lines: [
    { type: 'line', name: '直线', svg: '<line x1="2" y1="22" x2="22" y2="2" />' },
    { type: 'arrow-line', name: '箭头线', svg: '<line x1="2" y1="22" x2="22" y2="2" /><polygon points="22,2 18,6 20,2 22,2 20,2 18,6" />' },
    { type: 'double-arrow', name: '双向箭头', svg: '<line x1="2" y1="22" x2="22" y2="2" /><polygon points="2,22 6,18 2,20 2,22 2,20 6,18" /><polygon points="22,2 18,6 20,2 22,2 20,2 18,6" />' },
    { type: 'elbow-connector', name: '肘形连接符', svg: '<polyline points="2,22 2,12 22,12" />' },
    { type: 'curved-connector', name: '曲线连接符', svg: '<path d="M2,22 Q12,22 12,12 T22,12" />' }
  ],
  rectangles: [
    { type: 'rectangle', name: '矩形', svg: '<rect x="2" y="2" width="20" height="20" rx="2" />' },
    { type: 'rounded-rectangle', name: '圆角矩形', svg: '<rect x="2" y="2" width="20" height="20" rx="5" />' },
    { type: 'snip-rectangle', name: '剪角矩形', svg: '<polygon points="2,2 18,2 22,2 22,22 2,22" />' },
    { type: 'folded-corner', name: '折角矩形', svg: '<polygon points="2,2 18,2 22,6 22,22 2,22" />' }
  ],
  ovals: [
    { type: 'ellipse', name: '椭圆', svg: '<ellipse cx="12" cy="12" rx="10" ry="6" />' },
    { type: 'circle', name: '圆形', svg: '<circle cx="12" cy="12" r="10" />' },
    { type: 'oval', name: '椭圆', svg: '<ellipse cx="12" cy="12" rx="8" ry="10" />' }
  ],
  arrows: [
    { type: 'right-arrow', name: '右箭头', svg: '<polygon points="2,8 14,8 14,2 22,12 14,22 14,16 2,16" />' },
    { type: 'left-arrow', name: '左箭头', svg: '<polygon points="22,8 10,8 10,2 2,12 10,22 10,16 22,16" />' },
    { type: 'up-arrow', name: '上箭头', svg: '<polygon points="8,22 8,10 2,10 12,2 22,10 16,10 16,22" />' },
    { type: 'down-arrow', name: '下箭头', svg: '<polygon points="8,2 8,14 2,14 12,22 22,14 16,14 16,2" />' },
    { type: 'quad-arrow', name: '四向箭头', svg: '<polygon points="12,2 8,8 2,8 2,12 8,12 8,16 2,16 2,12 2,12 8,12 8,16 2,16 2,12 8,12 8,8 12,2" /><polygon points="12,22 16,16 22,16 22,12 16,12 16,8 22,8 22,12 22,12 16,12 16,8 22,8 22,12 16,12 16,16 12,22" /><polygon points="2,12 8,8 8,2 12,2 12,8 16,8 16,2 12,2 12,8 16,8 16,2 12,2 12,8 8,8 8,2 2,12" /><polygon points="22,12 16,16 16,22 12,22 12,16 8,16 8,22 12,22 12,16 8,16 8,22 12,22 12,16 16,16 16,22 22,12" />' }
  ],
  flowchart: [
    { type: 'process', name: '过程', svg: '<rect x="2" y="2" width="20" height="20" rx="2" />' },
    { type: 'decision', name: '决策', svg: '<polygon points="12,2 22,12 12,22 2,12" />' },
    { type: 'data', name: '数据', svg: '<polygon points="2,2 16,2 22,12 16,22 2,22" />' },
    { type: 'document', name: '文档', svg: '<polygon points="2,2 16,2 22,8 22,22 2,22" />' },
    { type: 'terminator', name: '终止符', svg: '<rect x="2" y="2" width="20" height="20" rx="10" />' }
  ],
  stars: [
    { type: 'star-5', name: '五角星', svg: '<polygon points="12,2 15,9 22,9 17,14 19,22 12,17 5,22 7,14 2,9 9,9" />' },
    { type: 'star-6', name: '六角星', svg: '<polygon points="12,2 15,8 22,8 18,13 22,19 15,19 12,24 9,19 2,19 6,13 2,8 9,8" />' },
    { type: 'star-8', name: '八角星', svg: '<polygon points="12,2 14,8 20,8 16,12 20,16 14,16 12,22 10,16 4,16 8,12 4,8 10,8" />' },
    { type: 'burst', name: '爆炸', svg: '<polygon points="12,2 14,6 22,4 16,10 22,14 14,14 12,22 10,14 2,14 8,10 2,4 10,6" />' }
  ],
  callouts: [
    { type: 'rounded-rect-callout', name: '圆角矩形标注', svg: '<rect x="2" y="2" width="20" height="20" rx="5" /><polygon points="12,22 16,28 20,22" />' },
    { type: 'oval-callout', name: '椭圆标注', svg: '<ellipse cx="12" cy="12" rx="10" ry="10" /><polygon points="12,22 16,28 20,22" />' },
    { type: 'cloud-callout', name: '云形标注', svg: '<path d="M12,2 C8,2 5,4 4,7 C2,7 1,9 1,12 C1,15 2,17 4,17 C4,20 6,22 9,22 C10,24 13,24 15,22 C18,22 20,20 21,17 C23,17 24,15 24,12 C24,9 23,7 21,7 C20,4 18,2 15,2 C14,2 13,2 12,2 Z" />' }
  ]
});

// Selected shape
const selectedShape = ref<ShapeDefinition | null>(null);

// Search query
const searchQuery = ref('');

// Computed
const currentShapes = computed(() => {
  const categoryShapes = shapes.value[activeCategory.value] || [];
  if (!searchQuery.value) {
    return categoryShapes;
  }
  return categoryShapes.filter(shape =>
    shape.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

// Select shape
const selectShape = (shape: ShapeDefinition) => {
  selectedShape.value = shape;
};

// Insert shape
const insertShape = () => {
  if (selectedShape.value) {
    emit('insert-shape', selectedShape.value);
    emit('update:show', false);
  }
};

// Cancel
const cancel = () => {
  emit('update:show', false);
};
</script>

<template>
  <BaseDialog
    :show="show"
    title="插入形状"
    width="700px"
    height="500px"
    @update:show="cancel"
  >
    <div class="shape-selector-dialog">
      <!-- Categories -->
      <div class="categories">
        <button
          v-for="category in shapeCategories"
          :key="category.id"
          class="category-btn"
          :class="{ active: activeCategory === category.id }"
          type="button"
          @click="activeCategory = category.id"
        >
          {{ category.name }}
        </button>
      </div>

      <!-- Search -->
      <div class="search-bar">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="搜索形状..."
          class="search-input"
        />
      </div>

      <!-- Shapes grid -->
      <div class="shapes-grid">
        <div
          v-for="shape in currentShapes"
          :key="shape.type"
          class="shape-item"
          :class="{ selected: selectedShape?.type === shape.type }"
          @click="selectShape(shape)"
        >
          <div class="shape-preview">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="48"
              height="48"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              v-html="shape.svg"
            />
          </div>
          <span class="shape-name">{{ shape.name }}</span>
        </div>
      </div>

      <!-- Selected shape preview -->
      <div v-if="selectedShape" class="selected-preview">
        <h4 class="preview-title">预览</h4>
        <div class="preview-content">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="120"
            height="120"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            v-html="selectedShape.svg"
          />
        </div>
        <p class="preview-name">{{ selectedShape.name }}</p>
      </div>
    </div>

    <template #footer>
      <button class="dialog-btn secondary" type="button" @click="cancel">
        取消
      </button>
      <button
        class="dialog-btn primary"
        :disabled="!selectedShape"
        type="button"
        @click="insertShape"
      >
        插入
      </button>
    </template>
  </BaseDialog>
</template>

<style scoped>
.shape-selector-dialog {
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: 100%;
}

/* Categories */
.categories {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
  border-bottom: 1px solid var(--word-border);
  padding-bottom: 12px;
}

.category-btn {
  padding: 8px 16px;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  cursor: pointer;
  font-size: 13px;
  color: var(--word-text-secondary);
  transition: all 0.15s ease;
}

.category-btn:hover {
  color: var(--word-text-primary);
  background: var(--word-button-hover);
}

.category-btn.active {
  color: var(--word-text-primary);
  border-bottom-color: var(--word-button-pressed);
  font-weight: 600;
}

/* Search */
.search-bar {
  display: flex;
}

.search-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 13px;
}

.search-input:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

/* Shapes grid */
.shapes-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
  gap: 12px;
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

.shape-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.shape-item:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

.shape-item.selected {
  background: var(--word-button-active);
  border-color: var(--word-button-pressed);
}

.shape-preview {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  color: var(--word-text-primary);
}

.shape-name {
  font-size: 11px;
  color: var(--word-text-secondary);
  text-align: center;
}

/* Selected preview */
.selected-preview {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 16px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-border);
  border-radius: 4px;
}

.preview-title {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--word-text-primary);
}

.preview-content {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 120px;
  height: 120px;
  background: white;
  border: 1px solid var(--word-border);
  border-radius: 4px;
  color: var(--word-text-primary);
}

.preview-name {
  margin: 0;
  font-size: 13px;
  color: var(--word-text-secondary);
}

/* Dialog buttons */
.dialog-btn {
  padding: 8px 24px;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border: none;
}

.dialog-btn.primary {
  background: var(--word-button-active);
  color: var(--word-text-primary);
}

.dialog-btn.primary:hover:not(:disabled) {
  background: var(--word-button-pressed);
}

.dialog-btn.primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.dialog-btn.secondary {
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  color: var(--word-text-primary);
}

.dialog-btn.secondary:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

/* Dark mode */
:global(.dark) .preview-content {
  background: var(--word-bg-canvas);
}
</style>
