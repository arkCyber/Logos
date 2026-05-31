<script setup lang="ts">
import { ref, computed } from 'vue';

const emit = defineEmits<{
  apply: [analysis: AnalysisConfig];
  close: [];
}>();

// 分析配置类型
interface AnalysisConfig {
  id: string;
  name: string;
  analysisType: string;
  dataRange: string;
  parameters: Record<string, any>;
}

// 分析类型
const analysisTypes = [
  { value: 'regression', label: '回归分析', icon: '📈' },
  { value: 'correlation', label: '相关性分析', icon: '🔗' },
  { value: 'descriptive', label: '描述性统计', icon: '📊' },
  { value: 'goalSeek', label: '目标寻求', icon: '🎯' },
  { value: 'scenario', label: '方案管理器', icon: '📋' },
  { value: 'anova', label: '方差分析', icon: '🔬' },
  { value: 'ttest', label: 't检验', icon: '🧪' }
] as const;

// 表单数据
const analysisName = ref('');
const selectedAnalysisType = ref('regression');
const dataRange = ref('');

// 回归分析参数
const regressionYRange = ref('');
const regressionXRange = ref('');
const regressionType = ref('linear');

// 相关性分析参数
const correlationRange = ref('');
const correlationMethod = ref('pearson');

// 目标寻求参数
const goalSeekCell = ref('');
const goalSeekTarget = ref('');
const goalSeekChangingCell = ref('');

// 方案管理器参数
const scenarioName = ref('');
const scenarioChangingCells = ref('');
const scenarioValues = ref('');

// 是否显示特定参数
const showRegressionParams = computed(() => selectedAnalysisType.value === 'regression');
const showCorrelationParams = computed(() => selectedAnalysisType.value === 'correlation');
const showGoalSeekParams = computed(() => selectedAnalysisType.value === 'goalSeek');
const showScenarioParams = computed(() => selectedAnalysisType.value === 'scenario');

// 应用分析
const applyAnalysis = () => {
  const parameters: Record<string, any> = {
    dataRange: dataRange.value
  };

  if (showRegressionParams.value) {
    parameters.yRange = regressionYRange.value;
    parameters.xRange = regressionXRange.value;
    parameters.type = regressionType.value;
  } else if (showCorrelationParams.value) {
    parameters.range = correlationRange.value;
    parameters.method = correlationMethod.value;
  } else if (showGoalSeekParams.value) {
    parameters.cell = goalSeekCell.value;
    parameters.target = goalSeekTarget.value;
    parameters.changingCell = goalSeekChangingCell.value;
  } else if (showScenarioParams.value) {
    parameters.name = scenarioName.value;
    parameters.changingCells = scenarioChangingCells.value;
    parameters.values = scenarioValues.value;
  }

  const analysis: AnalysisConfig = {
    id: crypto.randomUUID(),
    name: analysisName.value,
    analysisType: selectedAnalysisType.value,
    dataRange: dataRange.value,
    parameters
  };

  emit('apply', analysis);
  closeDialog();
};

// 关闭对话框
const closeDialog = () => {
  emit('close');
};
</script>

<template>
  <div class="data-analysis-dialog">
    <div class="dialog-header">
      <h3>数据分析工具</h3>
      <button class="btn-close" @click="closeDialog">✕</button>
    </div>

    <div class="dialog-body">
      <!-- 分析名称 -->
      <div class="form-group">
        <label>分析名称</label>
        <input v-model="analysisName" type="text" placeholder="输入分析名称" class="input-value" />
      </div>

      <!-- 分析类型 -->
      <div class="form-group">
        <label>分析类型</label>
        <div class="analysis-type-grid">
          <div
            v-for="type in analysisTypes"
            :key="type.value"
            :class="['analysis-type-item', { active: selectedAnalysisType === type.value }]"
            @click="selectedAnalysisType = type.value"
          >
            <span class="type-icon">{{ type.icon }}</span>
            <span class="type-label">{{ type.label }}</span>
          </div>
        </div>
      </div>

      <!-- 数据范围 -->
      <div class="form-group">
        <label>数据范围</label>
        <input v-model="dataRange" type="text" placeholder="例如: A1:D100" class="input-value" />
      </div>

      <!-- 回归分析参数 -->
      <div v-if="showRegressionParams" class="parameter-section">
        <h4>回归分析参数</h4>
        <div class="form-group">
          <label>Y变量范围</label>
          <input v-model="regressionYRange" type="text" placeholder="例如: A1:A10" class="input-value" />
        </div>
        <div class="form-group">
          <label>X变量范围</label>
          <input v-model="regressionXRange" type="text" placeholder="例如: B1:B10" class="input-value" />
        </div>
        <div class="form-group">
          <label>回归类型</label>
          <select v-model="regressionType" class="select-rule">
            <option value="linear">线性回归</option>
            <option value="polynomial">多项式回归</option>
            <option value="exponential">指数回归</option>
            <option value="logarithmic">对数回归</option>
          </select>
        </div>
      </div>

      <!-- 相关性分析参数 -->
      <div v-if="showCorrelationParams" class="parameter-section">
        <h4>相关性分析参数</h4>
        <div class="form-group">
          <label>数据范围</label>
          <input v-model="correlationRange" type="text" placeholder="例如: A1:D10" class="input-value" />
        </div>
        <div class="form-group">
          <label>相关系数方法</label>
          <select v-model="correlationMethod" class="select-rule">
            <option value="pearson">Pearson相关系数</option>
            <option value="spearman">Spearman秩相关</option>
            <option value="kendall">Kendall's Tau</option>
          </select>
        </div>
      </div>

      <!-- 目标寻求参数 -->
      <div v-if="showGoalSeekParams" class="parameter-section">
        <h4>目标寻求参数</h4>
        <div class="form-group">
          <label>目标单元格</label>
          <input v-model="goalSeekCell" type="text" placeholder="例如: B10" class="input-value" />
        </div>
        <div class="form-group">
          <label>目标值</label>
          <input v-model="goalSeekTarget" type="text" placeholder="例如: 1000" class="input-value" />
        </div>
        <div class="form-group">
          <label>可变单元格</label>
          <input v-model="goalSeekChangingCell" type="text" placeholder="例如: A1" class="input-value" />
        </div>
      </div>

      <!-- 方案管理器参数 -->
      <div v-if="showScenarioParams" class="parameter-section">
        <h4>方案管理器参数</h4>
        <div class="form-group">
          <label>方案名称</label>
          <input v-model="scenarioName" type="text" placeholder="例如: 乐观情况" class="input-value" />
        </div>
        <div class="form-group">
          <label>可变单元格（逗号分隔）</label>
          <input v-model="scenarioChangingCells" type="text" placeholder="例如: A1,B1,C1" class="input-value" />
        </div>
        <div class="form-group">
          <label>单元格值（逗号分隔）</label>
          <input v-model="scenarioValues" type="text" placeholder="例如: 100,200,300" class="input-value" />
        </div>
      </div>
    </div>

    <div class="dialog-footer">
      <button class="btn-cancel" @click="closeDialog">取消</button>
      <button class="btn-apply" @click="applyAnalysis">应用</button>
    </div>
  </div>
</template>

<style scoped>
.data-analysis-dialog {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 600px;
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
  margin-bottom: 20px;
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

.analysis-type-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 10px;
}

.analysis-type-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px 12px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  background: white;
}

.analysis-type-item:hover {
  border-color: #2196F3;
  background: #f8f9fa;
}

.analysis-type-item.active {
  border-color: #2196F3;
  background: #e3f2fd;
  box-shadow: 0 0 0 2px rgba(33, 150, 243, 0.2);
}

.type-icon {
  font-size: 24px;
}

.type-label {
  font-size: 12px;
  color: #555;
  text-align: center;
}

.parameter-section {
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
  margin-bottom: 20px;
}

.parameter-section h4 {
  margin: 0 0 16px 0;
  font-size: 14px;
  font-weight: 600;
  color: #333;
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
