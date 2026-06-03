<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import DOMPurify from 'dompurify';

interface ChartDataPoint {
  label: string;
  value: number;
  color?: string;
}

interface ChartData {
  title: string;
  labels: string[];
  datasets: ChartDataPoint[];
  x_axis_label?: string;
  y_axis_label?: string;
}

interface Props {
  modelValue: string;
}

interface Emits {
  (e: 'update:modelValue', value: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const showEditor = ref(false);
const chartType = ref('bar');
const chartTitle = ref('My Chart');
const xAxisLabel = ref('');
const yAxisLabel = ref('');
const chartData = ref<ChartDataPoint[]>([
  { label: 'A', value: 10 },
  { label: 'B', value: 20 },
  { label: 'C', value: 15 }
]);
const chartSvg = ref('');
const isGenerating = ref(false);
const isError = ref(false);
const errorMessage = ref('');

const chartTypes = [
  { value: 'pie', label: 'Pie Chart' },
  { value: 'bar', label: 'Bar Chart' },
  { value: 'line', label: 'Line Chart' },
  { value: 'area', label: 'Area Chart' },
  { value: 'scatter', label: 'Scatter Plot' },
  { value: 'doughnut', label: 'Doughnut Chart' }
];

// Generate chart using Rust backend
const generateChart = async () => {
  isGenerating.value = true;
  isError.value = false;

  try {
    const data: ChartData = {
      title: chartTitle.value,
      labels: chartData.value.map(d => d.label),
      datasets: chartData.value,
      x_axis_label: xAxisLabel.value || undefined,
      y_axis_label: yAxisLabel.value || undefined
    };

    const svg = await invoke<string>('generate_chart', {
      chartType: chartType.value,
      data,
      config: {
        width: 800,
        height: 600,
        backgroundColor: '#ffffff',
        showLegend: true,
        showGrid: true,
        animation: false
      }
    });

    // Sanitize SVG to prevent XSS attacks
    const sanitizedSvg = DOMPurify.sanitize(svg, {
      USE_PROFILES: { svg: true, svgFilters: true }
    });
    chartSvg.value = sanitizedSvg;
    emit('update:modelValue', sanitizedSvg);
  } catch (error) {
    isError.value = true;
    errorMessage.value = error instanceof Error ? error.message : 'Failed to generate chart';
  } finally {
    isGenerating.value = false;
  }
};

// Open editor
const openEditor = () => {
  showEditor.value = true;
  if (props.modelValue) {
    chartSvg.value = props.modelValue;
  }
};

// Close editor
const closeEditor = () => {
  showEditor.value = false;
};

// Add data point
const addDataPoint = () => {
  chartData.value.push({ label: `Item ${chartData.value.length + 1}`, value: 0 });
};

// Remove data point
const removeDataPoint = (index: number) => {
  chartData.value.splice(index, 1);
};

// Update data point
const updateDataPoint = (index: number, field: keyof ChartDataPoint, value: string | number) => {
  chartData.value[index][field] = value as never;
};

onMounted(() => {
  if (props.modelValue) {
    chartSvg.value = props.modelValue;
  }
});
</script>

<template>
  <div class="chart-editor">
    <!-- Chart Preview -->
    <div class="chart-preview" @click="openEditor">
      <div v-if="chartSvg" class="chart-svg" v-html="chartSvg"></div>
      <div v-else class="placeholder">Click to add chart</div>
    </div>

    <!-- Editor Modal -->
    <div v-if="showEditor" class="editor-modal">
      <div class="editor-content">
        <div class="editor-header">
          <h3>Chart Editor</h3>
          <button class="close-button" @click="closeEditor">&times;</button>
        </div>

        <div class="editor-body">
          <!-- Chart Type -->
          <div class="form-section">
            <label>Chart Type:</label>
            <select v-model="chartType" class="form-select">
              <option v-for="type in chartTypes" :key="type.value" :value="type.value">
                {{ type.label }}
              </option>
            </select>
          </div>

          <!-- Chart Title -->
          <div class="form-section">
            <label>Chart Title:</label>
            <input
              v-model="chartTitle"
              type="text"
              class="form-input"
              placeholder="Enter chart title"
            />
          </div>

          <!-- Axis Labels -->
          <div class="form-row">
            <div class="form-section">
              <label>X-Axis Label:</label>
              <input
                v-model="xAxisLabel"
                type="text"
                class="form-input"
                placeholder="X-axis label"
              />
            </div>
            <div class="form-section">
              <label>Y-Axis Label:</label>
              <input
                v-model="yAxisLabel"
                type="text"
                class="form-input"
                placeholder="Y-axis label"
              />
            </div>
          </div>

          <!-- Data Points -->
          <div class="form-section">
            <label>Data Points:</label>
            <div class="data-points">
              <div v-for="(point, index) in chartData" :key="index" class="data-point-row">
                <input
                  v-model="point.label"
                  type="text"
                  class="form-input small"
                  placeholder="Label"
                  @input="
                    updateDataPoint(index, 'label', ($event.target as HTMLInputElement).value)
                  "
                />
                <input
                  v-model.number="point.value"
                  type="number"
                  class="form-input small"
                  placeholder="Value"
                  @input="
                    updateDataPoint(
                      index,
                      'value',
                      parseFloat(($event.target as HTMLInputElement).value)
                    )
                  "
                />
                <input
                  v-model="point.color"
                  type="color"
                  class="form-input color"
                  @input="
                    updateDataPoint(index, 'color', ($event.target as HTMLInputElement).value)
                  "
                />
                <button class="remove-button" @click="removeDataPoint(index)">&times;</button>
              </div>
            </div>
            <button class="add-button" @click="addDataPoint">+ Add Data Point</button>
          </div>

          <!-- Preview -->
          <div class="form-section">
            <label>Preview:</label>
            <div class="preview-box">
              <div v-if="isGenerating" class="loading-indicator">Generating chart...</div>
              <div v-else-if="isError" class="error-message">{{ errorMessage }}</div>
              <div v-else-if="chartSvg" class="chart-svg" v-html="chartSvg"></div>
              <div v-else class="placeholder">Click "Generate" to preview</div>
            </div>
          </div>
        </div>

        <div class="editor-footer">
          <button class="generate-button" :disabled="isGenerating" @click="generateChart">
            {{ isGenerating ? 'Generating...' : 'Generate' }}
          </button>
          <button class="cancel-button" @click="closeEditor">Cancel</button>
          <button class="insert-button" @click="closeEditor">Insert</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.chart-editor {
  position: relative;
  display: inline-block;
}

.chart-preview {
  min-width: 200px;
  min-height: 150px;
  border: 2px dashed #d1d5db;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
  background: #f9fafb;
}

.chart-preview:hover {
  border-color: #3b82f6;
  background: #eff6ff;
}

.chart-svg {
  width: 100%;
  height: 100%;
}

.placeholder {
  color: #6b7280;
  font-size: 14px;
}

/* Editor Modal */
.editor-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.editor-content {
  background: white;
  border-radius: 8px;
  width: 90%;
  max-width: 900px;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid #e5e7eb;
}

.editor-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #111827;
}

.close-button {
  background: none;
  border: none;
  font-size: 24px;
  color: #6b7280;
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s;
}

.close-button:hover {
  background: #f3f4f6;
  color: #111827;
}

.editor-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.form-section {
  margin-bottom: 20px;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  margin-bottom: 20px;
}

label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: #374151;
  margin-bottom: 8px;
}

.form-input,
.form-select {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  font-size: 14px;
  transition: border-color 0.2s;
}

.form-input:focus,
.form-select:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.form-input.small {
  width: 120px;
}

.form-input.color {
  width: 50px;
  padding: 2px;
  height: 38px;
}

.data-points {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 12px;
}

.data-point-row {
  display: flex;
  gap: 8px;
  align-items: center;
}

.remove-button {
  background: #ef4444;
  color: white;
  border: none;
  width: 32px;
  height: 32px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 18px;
  transition: all 0.2s;
}

.remove-button:hover {
  background: #dc2626;
}

.add-button {
  padding: 8px 16px;
  background: #10b981;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.add-button:hover {
  background: #059669;
}

.preview-box {
  min-height: 300px;
  padding: 16px;
  border: 1px solid #e5e7eb;
  border-radius: 4px;
  background: #f9fafb;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: auto;
}

.loading-indicator {
  color: #6b7280;
  font-style: italic;
}

.error-message {
  color: #ef4444;
  font-size: 14px;
}

.editor-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid #e5e7eb;
}

.generate-button,
.cancel-button,
.insert-button {
  padding: 8px 16px;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.generate-button {
  background: #10b981;
  border: 1px solid #10b981;
  color: white;
}

.generate-button:hover:not(:disabled) {
  background: #059669;
}

.generate-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.cancel-button {
  background: white;
  border: 1px solid #d1d5db;
  color: #374151;
}

.cancel-button:hover {
  background: #f3f4f6;
}

.insert-button {
  background: #3b82f6;
  border: 1px solid #3b82f6;
  color: white;
}

.insert-button:hover {
  background: #2563eb;
}

/* Dark mode */
.editor-container.dark .chart-preview {
  background: #1f2937;
  border-color: #374151;
}

.editor-container.dark .chart-preview:hover {
  background: #374151;
  border-color: #60a5fa;
}

.editor-container.dark .placeholder {
  color: #9ca3af;
}

.editor-container.dark .editor-content {
  background: #1f2937;
}

.editor-container.dark .editor-header {
  border-bottom-color: #374151;
}

.editor-container.dark .editor-header h3 {
  color: #f9fafb;
}

.editor-container.dark .close-button {
  color: #9ca3af;
}

.editor-container.dark .close-button:hover {
  background: #374151;
  color: #f9fafb;
}

.editor-container.dark label {
  color: #d1d5db;
}

.editor-container.dark .form-input,
.editor-container.dark .form-select {
  background: #374151;
  border-color: #4b5563;
  color: #f9fafb;
}

.editor-container.dark .form-input:focus,
.editor-container.dark .form-select:focus {
  border-color: #60a5fa;
}

.editor-container.dark .preview-box {
  background: #374151;
  border-color: #4b5563;
}

.editor-container.dark .editor-footer {
  border-top-color: #374151;
}

.editor-container.dark .cancel-button {
  background: #374151;
  border-color: #4b5563;
  color: #f9fafb;
}

.editor-container.dark .cancel-button:hover {
  background: #4b5563;
}
</style>
