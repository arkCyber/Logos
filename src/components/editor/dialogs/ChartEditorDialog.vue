<script setup lang="ts">
import { ref, computed } from 'vue';
import BaseDialog from './BaseDialog.vue';

interface Props {
  show: boolean;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'insert-chart', chart: ChartDefinition): void;
}

interface ChartDefinition {
  type: string;
  title: string;
  data: ChartDataPoint[];
  style: ChartStyle;
}

interface ChartDataPoint {
  label: string;
  value: number;
}

interface ChartStyle {
  color: string;
  showLegend: boolean;
  showGrid: boolean;
  showLabels: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// Chart type
const activeChartType = ref('column');

const chartTypes = [
  { id: 'column', name: '柱状图', icon: '<rect x="4" y="12" width="4" height="10" /><rect x="10" y="6" width="4" height="16" /><rect x="16" y="10" width="4" height="12" />' },
  { id: 'bar', name: '条形图', icon: '<rect x="2" y="4" width="10" height="4" /><rect x="2" y="10" width="16" height="4" /><rect x="2" y="16" width="12" height="4" />' },
  { id: 'line', name: '折线图', icon: '<polyline points="2 18 8 10 14 14 20 6" />' },
  { id: 'pie', name: '饼图', icon: '<circle cx="12" cy="12" r="10" fill="none" /><path d="M12 12 L12 2 A10 10 0 0 1 22 12 Z" />' },
  { id: 'area', name: '面积图', icon: '<path d="M2 18 L8 10 L14 14 L20 6 L20 22 L2 22 Z" />' },
  { id: 'scatter', name: '散点图', icon: '<circle cx="6" cy="16" r="2" /><circle cx="12" cy="8" r="2" /><circle cx="18" cy="14" r="2" />' },
  { id: 'doughnut', name: '环形图', icon: '<circle cx="12" cy="12" r="10" fill="none" /><circle cx="12" cy="12" r="6" fill="none" />' },
  { id: 'radar', name: '雷达图', icon: '<polygon points="12 2 22 8 18 20 6 20 2 8" fill="none" />' }
];

// Chart title
const chartTitle = ref('图表标题');

// Data points
const dataPoints = ref<ChartDataPoint[]>([
  { label: '数据1', value: 10 },
  { label: '数据2', value: 20 },
  { label: '数据3', value: 15 }
]);

// Add data point
const addDataPoint = () => {
  const index = dataPoints.value.length + 1;
  dataPoints.value.push({ label: `数据${index}`, value: 0 });
};

// Remove data point
const removeDataPoint = (index: number) => {
  dataPoints.value.splice(index, 1);
};

// Update data point
const updateDataPoint = (index: number, field: 'label' | 'value', value: string | number) => {
  dataPoints.value[index][field] = value as never;
};

// Chart style
const chartStyle = ref<ChartStyle>({
  color: '#0078d4',
  showLegend: true,
  showGrid: true,
  showLabels: true
});

// Color presets
const colorPresets = [
  { id: 'blue', name: '蓝色', color: '#0078d4' },
  { id: 'green', name: '绿色', color: '#10b981' },
  { id: 'red', name: '红色', color: '#ef4444' },
  { id: 'orange', name: '橙色', color: '#f97316' },
  { id: 'purple', name: '紫色', color: '#8b5cf6' },
  { id: 'teal', name: '青色', color: '#14b8a6' }
];

// Insert chart
const insertChart = () => {
  const chart: ChartDefinition = {
    type: activeChartType.value,
    title: chartTitle.value,
    data: dataPoints.value,
    style: chartStyle.value
  };
  emit('insert-chart', chart);
  emit('update:show', false);
};

// Cancel
const cancel = () => {
  emit('update:show', false);
};
</script>

<template>
  <BaseDialog
    :show="show"
    title="插入图表"
    width="600px"
    height="550px"
    @update:show="cancel"
  >
    <div class="chart-editor-dialog">
      <!-- Chart type selection -->
      <div class="dialog-section">
        <h4 class="section-title">图表类型</h4>
        <div class="chart-types-grid">
          <div
            v-for="type in chartTypes"
            :key="type.id"
            class="chart-type-item"
            :class="{ active: activeChartType === type.id }"
            @click="activeChartType = type.id"
          >
            <div class="chart-type-icon">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="32"
                height="32"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                v-html="type.icon"
              />
            </div>
            <span class="chart-type-name">{{ type.name }}</span>
          </div>
        </div>
      </div>

      <!-- Chart title -->
      <div class="dialog-section">
        <h4 class="section-title">图表标题</h4>
        <input
          v-model="chartTitle"
          type="text"
          placeholder="输入图表标题"
          class="title-input"
        />
      </div>

      <!-- Data points -->
      <div class="dialog-section">
        <h4 class="section-title">数据</h4>
        <div class="data-points-list">
          <div
            v-for="(point, index) in dataPoints"
            :key="index"
            class="data-point-row"
          >
            <input
              :value="point.label"
              type="text"
              placeholder="标签"
              class="data-input"
              @input="updateDataPoint(index, 'label', ($event.target as HTMLInputElement).value)"
            />
            <input
              :value="point.value"
              type="number"
              placeholder="值"
              class="data-input number"
              @input="updateDataPoint(index, 'value', Number(($event.target as HTMLInputElement).value))"
            />
            <button
              class="remove-btn"
              title="删除"
              type="button"
              @click="removeDataPoint(index)"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              >
                <line x1="18" y1="6" x2="6" y2="18" />
                <line x1="6" y1="6" x2="18" y2="18" />
              </svg>
            </button>
          </div>
        </div>
        <button class="add-data-btn" type="button" @click="addDataPoint">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <line x1="12" y1="5" x2="12" y2="19" />
            <line x1="5" y1="12" x2="19" y2="12" />
          </svg>
          添加数据点
        </button>
      </div>

      <!-- Chart style -->
      <div class="dialog-section">
        <h4 class="section-title">样式</h4>
        <div class="style-settings">
          <div class="setting-row">
            <label>颜色:</label>
            <div class="color-presets">
              <div
                v-for="preset in colorPresets"
                :key="preset.id"
                class="color-preset"
                :class="{ active: chartStyle.color === preset.color }"
                :style="{ backgroundColor: preset.color }"
                @click="chartStyle.color = preset.color"
              ></div>
            </div>
            <input
              v-model="chartStyle.color"
              type="color"
              class="color-input"
            />
          </div>
          <div class="setting-row checkboxes">
            <label class="checkbox-label">
              <input v-model="chartStyle.showLegend" type="checkbox" />
              <span>显示图例</span>
            </label>
            <label class="checkbox-label">
              <input v-model="chartStyle.showGrid" type="checkbox" />
              <span>显示网格</span>
            </label>
            <label class="checkbox-label">
              <input v-model="chartStyle.showLabels" type="checkbox" />
              <span>显示标签</span>
            </label>
          </div>
        </div>
      </div>

      <!-- Preview -->
      <div class="dialog-section">
        <h4 class="section-title">预览</h4>
        <div class="preview-area">
          <div class="chart-preview">
            <h5 class="preview-title">{{ chartTitle }}</h5>
            <div class="preview-chart">
              <div class="preview-bars">
                <div
                  v-for="(point, index) in dataPoints"
                  :key="index"
                  class="preview-bar"
                  :style="{
                    height: `${Math.min(point.value * 2, 100)}%`,
                    backgroundColor: chartStyle.color
                  }"
                ></div>
              </div>
              <div class="preview-labels">
                <span
                  v-for="(point, index) in dataPoints"
                  :key="index"
                  class="preview-label"
                >
                  {{ point.label }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <button class="dialog-btn secondary" type="button" @click="cancel">
        取消
      </button>
      <button class="dialog-btn primary" type="button" @click="insertChart">
        插入图表
      </button>
    </template>
  </BaseDialog>
</template>

<style scoped>
.chart-editor-dialog {
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: 100%;
  overflow-y: auto;
}

.dialog-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.section-title {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--word-text-primary);
}

/* Chart types grid */
.chart-types-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
  gap: 8px;
}

.chart-type-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 12px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.chart-type-item:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

.chart-type-item.active {
  background: var(--word-button-active);
  border-color: var(--word-button-pressed);
}

.chart-type-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  color: var(--word-text-primary);
}

.chart-type-name {
  font-size: 11px;
  color: var(--word-text-secondary);
  text-align: center;
}

/* Title input */
.title-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 14px;
}

.title-input:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

/* Data points */
.data-points-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.data-point-row {
  display: flex;
  gap: 8px;
  align-items: center;
}

.data-input {
  flex: 1;
  padding: 6px 10px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 13px;
}

.data-input:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

.data-input.number {
  width: 100px;
}

.remove-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  color: var(--word-text-secondary);
  transition: all 0.15s ease;
}

.remove-btn:hover {
  background: var(--word-button-hover);
  color: #ef4444;
}

.add-data-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  color: var(--word-text-primary);
  transition: all 0.15s ease;
  align-self: flex-start;
}

.add-data-btn:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

/* Style settings */
.style-settings {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.setting-row label {
  min-width: 50px;
  font-size: 13px;
  color: var(--word-text-primary);
}

.color-presets {
  display: flex;
  gap: 6px;
}

.color-preset {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  cursor: pointer;
  border: 2px solid transparent;
  transition: all 0.15s ease;
}

.color-preset:hover {
  border-color: var(--word-button-border-hover);
}

.color-preset.active {
  border-color: var(--word-button-pressed);
  box-shadow: 0 0 0 2px var(--word-button-pressed);
}

.color-input {
  width: 40px;
  height: 28px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
}

.checkboxes {
  display: flex;
  gap: 16px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--word-text-primary);
  cursor: pointer;
}

/* Preview */
.preview-area {
  padding: 16px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-border);
  border-radius: 4px;
}

.chart-preview {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.preview-title {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--word-text-primary);
}

.preview-chart {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
  max-width: 300px;
}

.preview-bars {
  display: flex;
  align-items: flex-end;
  gap: 12px;
  height: 100px;
  padding: 0 20px;
  border-bottom: 1px solid var(--word-border);
}

.preview-bar {
  flex: 1;
  min-width: 20px;
  border-radius: 2px 2px 0 0;
  transition: height 0.3s ease;
}

.preview-labels {
  display: flex;
  gap: 12px;
  padding: 0 20px;
}

.preview-label {
  flex: 1;
  min-width: 20px;
  font-size: 11px;
  color: var(--word-text-secondary);
  text-align: center;
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

.dialog-btn.primary:hover {
  background: var(--word-button-pressed);
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
:global(.dark) .preview-bars {
  border-bottom-color: var(--word-border);
}
</style>
