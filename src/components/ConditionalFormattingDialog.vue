<script setup lang="ts">
import { ref, computed } from 'vue';

// 条件格式规则类型
interface ConditionalFormatRule {
  id: string;
  range: string;
  ruleType: string;
  ruleData: string;
  formatData: string;
  priority: number;
}

const emit = defineEmits<{
  apply: [rule: ConditionalFormatRule];
  close: [];
}>();

// 规则类型
const ruleTypes = [
  { value: 'GreaterThan', label: '大于' },
  { value: 'LessThan', label: '小于' },
  { value: 'EqualTo', label: '等于' },
  { value: 'Between', label: '介于' },
  { value: 'NotBetween', label: '不介于' },
  { value: 'ContainsText', label: '包含文本' },
  { value: 'NotContainsText', label: '不包含文本' },
  { value: 'Duplicate', label: '重复值' },
  { value: 'Unique', label: '唯一值' },
  { value: 'TopN', label: '前N项' },
  { value: 'BottomN', label: '后N项' },
  { value: 'AboveAverage', label: '高于平均值' },
  { value: 'BelowAverage', label: '低于平均值' },
  { value: 'Formula', label: '公式' }
] as const;

// 格式类型
const formatTypes = [
  { value: 'solid', label: '纯色填充' },
  { value: 'gradient', label: '渐变填充' },
  { value: 'dataBar', label: '数据条' },
  { value: 'colorScale', label: '色阶' },
  { value: 'iconSet', label: '图标集' }
] as const;

// 表单数据
const selectedRuleType = ref('GreaterThan');
const cellRange = ref('');
const value1 = ref('');
const value2 = ref('');
const nValue = ref(10);
const formula = ref('');
const selectedFormatType = ref('solid');

// 格式设置
const backgroundColor = ref('#ff0000');
const textColor = ref('#ffffff');
const fontWeight = ref('normal');
const fontStyle = ref('normal');
const textDecoration = ref('none');

// 是否需要第二个值
const needsSecondValue = computed(() => {
  return ['Between', 'NotBetween'].includes(selectedRuleType.value);
});

// 是否需要N值
const needsNValue = computed(() => {
  return ['TopN', 'BottomN'].includes(selectedRuleType.value);
});

// 是否需要公式
const needsFormula = computed(() => {
  return selectedRuleType.value === 'Formula';
});

// 是否需要值输入
const needsValueInput = computed(() => {
  return !['Duplicate', 'Unique', 'AboveAverage', 'BelowAverage', 'Formula'].includes(selectedRuleType.value);
});

// 应用条件格式
const applyRule = () => {
  const rule: ConditionalFormatRule = {
    id: crypto.randomUUID(),
    range: cellRange.value,
    ruleType: selectedRuleType.value,
    ruleData: buildRuleData(),
    formatData: buildFormatData(),
    priority: 0
  };

  emit('apply', rule);
  closeDialog();
};

// 构建规则数据
const buildRuleData = (): string => {
  const data: Record<string, any> = {};

  if (needsValueInput.value) {
    data.value1 = value1.value;
  }

  if (needsSecondValue.value) {
    data.value2 = value2.value;
  }

  if (needsNValue.value) {
    data.n = nValue.value;
  }

  if (needsFormula.value) {
    data.formula = formula.value;
  }

  return JSON.stringify(data);
};

// 构建格式数据
const buildFormatData = (): string => {
  const format: Record<string, any> = {
    type: selectedFormatType.value,
    backgroundColor: backgroundColor.value,
    textColor: textColor.value,
    fontWeight: fontWeight.value,
    fontStyle: fontStyle.value,
    textDecoration: textDecoration.value
  };

  return JSON.stringify(format);
};

// 关闭对话框
const closeDialog = () => {
  emit('close');
};

// 预览格式
const previewStyle = computed(() => ({
  backgroundColor: backgroundColor.value,
  color: textColor.value,
  fontWeight: fontWeight.value === 'bold' ? 'bold' : 'normal',
  fontStyle: fontStyle.value === 'italic' ? 'italic' : 'normal',
  textDecoration: textDecoration.value === 'underline' ? 'underline' : 'none'
}));
</script>

<template>
  <div class="conditional-formatting-dialog">
    <div class="dialog-header">
      <h3>条件格式</h3>
      <button class="btn-close" @click="closeDialog">✕</button>
    </div>

    <div class="dialog-body">
      <!-- 单元格范围 -->
      <div class="form-group">
        <label>应用范围</label>
        <input
          v-model="cellRange"
          type="text"
          placeholder="例如: A1:D10"
          class="input-range"
        />
      </div>

      <!-- 规则类型 -->
      <div class="form-group">
        <label>规则类型</label>
        <select v-model="selectedRuleType" class="select-rule">
          <option v-for="type in ruleTypes" :key="type.value" :value="type.value">
            {{ type.label }}
          </option>
        </select>
      </div>

      <!-- 规则参数 -->
      <div v-if="needsValueInput" class="form-group">
        <label>值</label>
        <input v-model="value1" type="text" placeholder="输入值" class="input-value" />
      </div>

      <div v-if="needsSecondValue" class="form-group">
        <label>结束值</label>
        <input v-model="value2" type="text" placeholder="输入结束值" class="input-value" />
      </div>

      <div v-if="needsNValue" class="form-group">
        <label>N值</label>
        <input v-model.number="nValue" type="number" min="1" class="input-value" />
      </div>

      <div v-if="needsFormula" class="form-group">
        <label>公式</label>
        <input v-model="formula" type="text" placeholder="=SUM(A1:A10)" class="input-value" />
      </div>

      <!-- 格式类型 -->
      <div class="form-group">
        <label>格式类型</label>
        <select v-model="selectedFormatType" class="select-rule">
          <option v-for="type in formatTypes" :key="type.value" :value="type.value">
            {{ type.label }}
          </option>
        </select>
      </div>

      <!-- 格式设置 -->
      <div class="format-settings">
        <div class="form-group">
          <label>背景颜色</label>
          <input v-model="backgroundColor" type="color" class="input-color" />
        </div>

        <div class="form-group">
          <label>文字颜色</label>
          <input v-model="textColor" type="color" class="input-color" />
        </div>

        <div class="form-group">
          <label>字体粗细</label>
          <select v-model="fontWeight" class="select-rule">
            <option value="normal">正常</option>
            <option value="bold">粗体</option>
          </select>
        </div>

        <div class="form-group">
          <label>字体样式</label>
          <select v-model="fontStyle" class="select-rule">
            <option value="normal">正常</option>
            <option value="italic">斜体</option>
          </select>
        </div>

        <div class="form-group">
          <label>下划线</label>
          <select v-model="textDecoration" class="select-rule">
            <option value="none">无</option>
            <option value="underline">下划线</option>
          </select>
        </div>
      </div>

      <!-- 预览 -->
      <div class="form-group">
        <label>预览</label>
        <div class="preview-box" :style="previewStyle">
          示例文本
        </div>
      </div>
    </div>

    <div class="dialog-footer">
      <button class="btn-cancel" @click="closeDialog">取消</button>
      <button class="btn-apply" @click="applyRule">应用</button>
    </div>
  </div>
</template>

<style scoped>
.conditional-formatting-dialog {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 500px;
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

.input-range,
.input-value,
.select-rule {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #d0d0d0;
  border-radius: 6px;
  font-size: 14px;
  transition: all 0.2s;
}

.input-range:focus,
.input-value:focus,
.select-rule:focus {
  outline: none;
  border-color: #2196F3;
  box-shadow: 0 0 0 3px rgba(33, 150, 243, 0.1);
}

.input-color {
  width: 60px;
  height: 36px;
  padding: 2px;
  border: 1px solid #d0d0d0;
  border-radius: 6px;
  cursor: pointer;
}

.format-settings {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  padding: 12px;
  background: #f8f9fa;
  border-radius: 8px;
  margin-bottom: 16px;
}

.preview-box {
  padding: 12px;
  border: 1px solid #d0d0d0;
  border-radius: 6px;
  text-align: center;
  font-size: 14px;
  min-height: 40px;
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
