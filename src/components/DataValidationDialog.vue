<script setup lang="ts">
import { ref, computed } from 'vue';

const emit = defineEmits<{
  apply: [rule: DataValidationRule];
  close: [];
}>();

// 数据验证规则类型
interface DataValidationRule {
  id: string;
  range: string;
  validationType: string;
  operator: string;
  value1: string;
  value2?: string;
  errorMessage?: string;
  showErrorAlert: boolean;
}

// 验证类型
const validationTypes = [
  { value: 'WholeNumber', label: '整数' },
  { value: 'Decimal', label: '小数' },
  { value: 'List', label: '列表' },
  { value: 'Date', label: '日期' },
  { value: 'Time', label: '时间' },
  { value: 'TextLength', label: '文本长度' },
  { value: 'Custom', label: '自定义公式' }
] as const;

// 运算符
const operators = [
  { value: 'between', label: '介于' },
  { value: 'notBetween', label: '不介于' },
  { value: 'equal', label: '等于' },
  { value: 'notEqual', label: '不等于' },
  { value: 'greaterThan', label: '大于' },
  { value: 'lessThan', label: '小于' },
  { value: 'greaterThanOrEqual', label: '大于或等于' },
  { value: 'lessThanOrEqual', label: '小于或等于' }
] as const;

// 表单数据
const selectedValidationType = ref('WholeNumber');
const selectedOperator = ref('between');
const cellRange = ref('');
const value1 = ref('');
const value2 = ref('');
const listItems = ref('');
const customFormula = ref('');
const errorMessage = ref('');
const showErrorAlert = ref(true);

// 是否需要第二个值
const needsSecondValue = computed(() => {
  return ['between', 'notBetween'].includes(selectedOperator.value);
});

// 是否需要列表输入
const needsListInput = computed(() => {
  return selectedValidationType.value === 'List';
});

// 是否需要公式输入
const needsFormulaInput = computed(() => {
  return selectedValidationType.value === 'Custom';
});

// 是否需要值输入
const needsValueInput = computed(() => {
  return !needsListInput.value && !needsFormulaInput.value;
});

// 应用数据验证规则
const applyRule = () => {
  const rule: DataValidationRule = {
    id: crypto.randomUUID(),
    range: cellRange.value,
    validationType: selectedValidationType.value,
    operator: selectedOperator.value,
    value1: needsListInput.value ? listItems.value : needsFormulaInput.value ? customFormula.value : value1.value,
    value2: needsSecondValue.value ? value2.value : undefined,
    errorMessage: errorMessage.value,
    showErrorAlert: showErrorAlert.value
  };

  emit('apply', rule);
  closeDialog();
};

// 关闭对话框
const closeDialog = () => {
  emit('close');
};
</script>

<template>
  <div class="data-validation-dialog">
    <div class="dialog-header">
      <h3>数据验证</h3>
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

      <!-- 验证类型 -->
      <div class="form-group">
        <label>验证类型</label>
        <select v-model="selectedValidationType" class="select-rule">
          <option v-for="type in validationTypes" :key="type.value" :value="type.value">
            {{ type.label }}
          </option>
        </select>
      </div>

      <!-- 运算符 -->
      <div v-if="!needsListInput && !needsFormulaInput" class="form-group">
        <label>运算符</label>
        <select v-model="selectedOperator" class="select-rule">
          <option v-for="op in operators" :key="op.value" :value="op.value">
            {{ op.label }}
          </option>
        </select>
      </div>

      <!-- 值输入 -->
      <div v-if="needsValueInput" class="form-group">
        <label>值</label>
        <input v-model="value1" type="text" placeholder="输入值" class="input-value" />
      </div>

      <div v-if="needsSecondValue" class="form-group">
        <label>结束值</label>
        <input v-model="value2" type="text" placeholder="输入结束值" class="input-value" />
      </div>

      <!-- 列表输入 -->
      <div v-if="needsListInput" class="form-group">
        <label>列表项（用逗号分隔）</label>
        <textarea
          v-model="listItems"
          placeholder="例如: 选项1,选项2,选项3"
          class="input-textarea"
          rows="3"
        ></textarea>
      </div>

      <!-- 自定义公式 -->
      <div v-if="needsFormulaInput" class="form-group">
        <label>自定义公式</label>
        <input
          v-model="customFormula"
          type="text"
          placeholder="=SUM(A1:A10)>100"
          class="input-value"
        />
      </div>

      <!-- 错误消息 -->
      <div class="form-group">
        <label>错误消息（可选）</label>
        <input
          v-model="errorMessage"
          type="text"
          placeholder="输入无效数据时显示的错误消息"
          class="input-value"
        />
      </div>

      <!-- 显示错误提示 -->
      <div class="form-group checkbox-group">
        <label class="checkbox-label">
          <input v-model="showErrorAlert" type="checkbox" />
          <span>输入无效数据时显示错误提示</span>
        </label>
      </div>
    </div>

    <div class="dialog-footer">
      <button class="btn-cancel" @click="closeDialog">取消</button>
      <button class="btn-apply" @click="applyRule">应用</button>
    </div>
  </div>
</template>

<style scoped>
.data-validation-dialog {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 450px;
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
.select-rule,
.input-textarea {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #d0d0d0;
  border-radius: 6px;
  font-size: 14px;
  transition: all 0.2s;
  font-family: inherit;
}

.input-range:focus,
.input-value:focus,
.select-rule:focus,
.input-textarea:focus {
  outline: none;
  border-color: #2196F3;
  box-shadow: 0 0 0 3px rgba(33, 150, 243, 0.1);
}

.input-textarea {
  resize: vertical;
  min-height: 80px;
}

.checkbox-group {
  margin-top: 20px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
  color: #555;
}

.checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
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
