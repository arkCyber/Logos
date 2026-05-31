<script setup lang="ts">
import { ref, computed } from 'vue';
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, PointElement, LineElement, ArcElement } from 'chart.js';
import { Bar, Line, Pie, Doughnut, Scatter } from 'vue-chartjs';

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, PointElement, LineElement, ArcElement);

const emit = defineEmits<{
  apply: [chart: ChartConfig];
  close: [];
}>();

// 图表配置类型
interface ChartConfig {
  id: string;
  name: string;
  chartType: string;
  dataRange: string;
  title: string;
  xAxisTitle: string;
  yAxisTitle: string;
  legendPosition: string;
  styleData: string;
}

// 图表类型
const chartTypes = [
  { value: 'Line', label: '折线图' },
  { value: 'Bar', label: '柱状图' },
  { value: 'Column', label: '条形图' },
  { value: 'Pie', label: '饼图' },
  { value: 'Doughnut', label: '环形图' },
  { value: 'Scatter', label: '散点图' },
  { value: 'Area', label: '面积图' },
  { value: 'Radar', label: '雷达图' }
] as const;

// 图例位置
const legendPositions = [
  { value: 'top', label: '顶部' },
  { value: 'bottom', label: '底部' },
  { value: 'left', label: '左侧' },
  { value: 'right', label: '右侧' },
  { value: 'none', label: '无' }
] as const;

// 表单数据
const chartName = ref('');
const selectedChartType = ref('Line');
const dataRange = ref('');
const chartTitle = ref('');
const xAxisTitle = ref('');
const yAxisTitle = ref('');
const selectedLegendPosition = ref('top');

// 预览数据
const previewData = computed(() => ({
  labels: ['2025', '2026', '2027'],
  datasets: [
    {
      label: '营收',
      data: [12.5, 18.2, 25.6],
      backgroundColor: 'rgba(33, 150, 243, 0.5)',
      borderColor: 'rgba(33, 150, 243, 1)',
      borderWidth: 2
    },
    {
      label: '利润',
      data: [3.2, 5.8, 8.9],
      backgroundColor: 'rgba(76, 175, 80, 0.5)',
      borderColor: 'rgba(76, 175, 80, 1)',
      borderWidth: 2
    }
  ]
}));

const chartOptions = computed(() => ({
  responsive: true,
  maintainAspectRatio: true,
  plugins: {
    legend: {
      position: selectedLegendPosition.value as any
    },
    title: {
      display: !!chartTitle.value,
      text: chartTitle.value || '图表预览'
    }
  },
  scales: {
    x: {
      title: {
        display: !!xAxisTitle.value,
        text: xAxisTitle.value
      }
    },
    y: {
      title: {
        display: !!yAxisTitle.value,
        text: yAxisTitle.value
      }
    }
  }
}));

// 应用图表配置
const applyChart = () => {
  const chart: ChartConfig = {
    id: crypto.randomUUID(),
    name: chartName.value,
    chartType: selectedChartType.value,
    dataRange: dataRange.value,
    title: chartTitle.value,
    xAxisTitle: xAxisTitle.value,
    yAxisTitle: yAxisTitle.value,
    legendPosition: selectedLegendPosition.value,
    styleData: JSON.stringify({
      colors: ['#2196F3', '#4CAF50', '#FF9800', '#F44336'],
      fontSize: 12
    })
  };

  emit('apply', chart);
  closeDialog();
};

// 关闭对话框
const closeDialog = () => {
  emit('close');
};

// 获取预览组件
const previewComponent = computed(() => {
  switch (selectedChartType.value) {
    case 'Bar':
    case 'Column':
      return Bar;
    case 'Line':
    case 'Area':
      return Line;
    case 'Pie':
      return Pie;
    case 'Doughnut':
      return Doughnut;
    case 'Scatter':
      return Scatter;
    default:
      return Line;
  }
});
</script>

<template>
  <div class="chart-dialog">
    <div class="dialog-header">
      <h3>创建图表</h3>
      <button class="btn-close" @click="closeDialog">✕</button>
    </div>

    <div class="dialog-body">
      <!-- 图表名称 -->
      <div class="form-group">
        <label>图表名称</label>
        <input v-model="chartName" type="text" placeholder="输入图表名称" class="input-value" />
      </div>

      <!-- 图表类型 -->
      <div class="form-group">
        <label>图表类型</label>
        <select v-model="selectedChartType" class="select-rule">
          <option v-for="type in chartTypes" :key="type.value" :value="type.value">
            {{ type.label }}
          </option>
        </select>
      </div>

      <!-- 数据范围 -->
      <div class="form-group">
        <label>数据范围</label>
        <input v-model="dataRange" type="text" placeholder="例如: A1:D10" class="input-value" />
      </div>

      <!-- 图表标题 -->
      <div class="form-group">
        <label>图表标题</label>
        <input v-model="chartTitle" type="text" placeholder="输入图表标题" class="input-value" />
      </div>

      <!-- X轴标题 -->
      <div class="form-group">
        <label>X轴标题</label>
        <input v-model="xAxisTitle" type="text" placeholder="输入X轴标题" class="input-value" />
      </div>

      <!-- Y轴标题 -->
      <div class="form-group">
        <label>Y轴标题</label>
        <input v-model="yAxisTitle" type="text" placeholder="输入Y轴标题" class="input-value" />
      </div>

      <!-- 图例位置 -->
      <div class="form-group">
        <label>图例位置</label>
        <select v-model="selectedLegendPosition" class="select-rule">
          <option v-for="pos in legendPositions" :key="pos.value" :value="pos.value">
            {{ pos.label }}
          </option>
        </select>
      </div>

      <!-- 预览 -->
      <div class="form-group">
        <label>预览</label>
        <div class="chart-preview">
          <component
            :is="previewComponent"
            :data="previewData"
            :options="chartOptions"
            style="max-height: 200px;"
          />
        </div>
      </div>
    </div>

    <div class="dialog-footer">
      <button class="btn-cancel" @click="closeDialog">取消</button>
      <button class="btn-apply" @click="applyChart">应用</button>
    </div>
  </div>
</template>

<style scoped>
.chart-dialog {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 550px;
  max-height: 90vh;
  background: white;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  z-index: 1000;
  display: flex;
  flex-direction: column;
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid #e0e0e0;
}

.dialog-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #333;
}

.btn-close {
  background: none;
  border: none;
  font-size: 20px;
  cursor: pointer;
  color: #666;
  padding: 4px 8px;
  border-radius: 4px;
  transition: all 0.2s;
}

.btn-close:hover {
  background: #f0f0f0;
  color: #333;
}

.dialog-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-size: 13px;
  font-weight: 500;
  color: #555;
}

.input-value,
.select-rule {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #d0d0d0;
  border-radius: 6px;
  font-size: 14px;
  transition: all 0.2s;
}

.input-value:focus,
.select-rule:focus {
  outline: none;
  border-color: #2196F3;
  box-shadow: 0 0 0 3px rgba(33, 150, 243, 0.1);
}

.chart-preview {
  padding: 16px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  background: #fafafa;
  min-height: 200px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 16px 20px;
  border-top: 1px solid #e0e0e0;
}

.btn-cancel,
.btn-apply {
  padding: 8px 20px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-cancel {
  background: #f0f0f0;
  color: #333;
}

.btn-cancel:hover {
  background: #e0e0e0;
}

.btn-apply {
  background: #2196F3;
  color: white;
}

.btn-apply:hover {
  background: #1976D2;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(33, 150, 243, 0.3);
}
</style>
