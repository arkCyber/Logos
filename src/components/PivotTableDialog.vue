<script setup lang="ts">
import { ref } from 'vue';

const emit = defineEmits<{
  apply: [pivot: PivotTableConfig];
  close: [];
}>();

// 数据透视表配置类型
interface PivotTableConfig {
  id: string;
  name: string;
  sourceRange: string;
  rowFields: string[];
  columnFields: string[];
  valueFields: PivotValueField[];
  filterFields: string[];
}

interface PivotValueField {
  field: string;
  aggregation: string;
  customName?: string;
}

// 聚合类型
const aggregationTypes = [
  { value: 'Sum', label: '求和' },
  { value: 'Average', label: '平均值' },
  { value: 'Count', label: '计数' },
  { value: 'CountNumbers', label: '数值计数' },
  { value: 'Max', label: '最大值' },
  { value: 'Min', label: '最小值' },
  { value: 'Product', label: '乘积' },
  { value: 'StdDev', label: '标准差' },
  { value: 'StdDevP', label: '总体标准差' },
  { value: 'Var', label: '方差' },
  { value: 'VarP', label: '总体方差' }
] as const;

// 表单数据
const pivotName = ref('');
const sourceRange = ref('');
const rowFields = ref<string[]>([]);
const columnFields = ref<string[]>([]);
const valueFields = ref<PivotValueField[]>([]);
const filterFields = ref<string[]>([]);

// 临时输入
const tempRowField = ref('');
const tempColumnField = ref('');
const tempFilterField = ref('');
const tempValueField = ref('');
const tempValueAggregation = ref('Sum');
const tempValueCustomName = ref('');

// 添加行字段
const addRowField = () => {
  if (tempRowField.value && !rowFields.value.includes(tempRowField.value)) {
    rowFields.value.push(tempRowField.value);
    tempRowField.value = '';
  }
};

// 移除行字段
const removeRowField = (index: number) => {
  rowFields.value.splice(index, 1);
};

// 添加列字段
const addColumnField = () => {
  if (tempColumnField.value && !columnFields.value.includes(tempColumnField.value)) {
    columnFields.value.push(tempColumnField.value);
    tempColumnField.value = '';
  }
};

// 移除列字段
const removeColumnField = (index: number) => {
  columnFields.value.splice(index, 1);
};

// 添加筛选字段
const addFilterField = () => {
  if (tempFilterField.value && !filterFields.value.includes(tempFilterField.value)) {
    filterFields.value.push(tempFilterField.value);
    tempFilterField.value = '';
  }
};

// 移除筛选字段
const removeFilterField = (index: number) => {
  filterFields.value.splice(index, 1);
};

// 添加值字段
const addValueField = () => {
  if (tempValueField.value) {
    valueFields.value.push({
      field: tempValueField.value,
      aggregation: tempValueAggregation.value,
      customName: tempValueCustomName.value || undefined
    });
    tempValueField.value = '';
    tempValueCustomName.value = '';
  }
};

// 移除值字段
const removeValueField = (index: number) => {
  valueFields.value.splice(index, 1);
};

// 应用数据透视表配置
const applyPivot = () => {
  const pivot: PivotTableConfig = {
    id: crypto.randomUUID(),
    name: pivotName.value,
    sourceRange: sourceRange.value,
    rowFields: rowFields.value,
    columnFields: columnFields.value,
    valueFields: valueFields.value,
    filterFields: filterFields.value
  };

  emit('apply', pivot);
  closeDialog();
};

// 关闭对话框
const closeDialog = () => {
  emit('close');
};
</script>

<template>
  <div class="pivot-table-dialog">
    <div class="dialog-header">
      <h3>创建数据透视表</h3>
      <button class="btn-close" @click="closeDialog">✕</button>
    </div>

    <div class="dialog-body">
      <!-- 透视表名称 -->
      <div class="form-group">
        <label>透视表名称</label>
        <input v-model="pivotName" type="text" placeholder="输入透视表名称" class="input-value" />
      </div>

      <!-- 数据源范围 -->
      <div class="form-group">
        <label>数据源范围</label>
        <input v-model="sourceRange" type="text" placeholder="例如: A1:F100" class="input-value" />
      </div>

      <!-- 行字段 -->
      <div class="field-section">
        <label>行字段</label>
        <div class="field-input-group">
          <input
            v-model="tempRowField"
            type="text"
            placeholder="输入字段名（如：年份）"
            class="input-field"
            @keyup.enter="addRowField"
          />
          <button class="btn-add" @click="addRowField">添加</button>
        </div>
        <div class="field-list">
          <div v-for="(field, index) in rowFields" :key="index" class="field-item">
            <span>{{ field }}</span>
            <button class="btn-remove" @click="removeRowField(index)">✕</button>
          </div>
        </div>
      </div>

      <!-- 列字段 -->
      <div class="field-section">
        <label>列字段</label>
        <div class="field-input-group">
          <input
            v-model="tempColumnField"
            type="text"
            placeholder="输入字段名（如：地区）"
            class="input-field"
            @keyup.enter="addColumnField"
          />
          <button class="btn-add" @click="addColumnField">添加</button>
        </div>
        <div class="field-list">
          <div v-for="(field, index) in columnFields" :key="index" class="field-item">
            <span>{{ field }}</span>
            <button class="btn-remove" @click="removeColumnField(index)">✕</button>
          </div>
        </div>
      </div>

      <!-- 值字段 -->
      <div class="field-section">
        <label>值字段</label>
        <div class="value-field-inputs">
          <input
            v-model="tempValueField"
            type="text"
            placeholder="字段名（如：销售额）"
            class="input-field"
          />
          <select v-model="tempValueAggregation" class="select-aggregation">
            <option v-for="agg in aggregationTypes" :key="agg.value" :value="agg.value">
              {{ agg.label }}
            </option>
          </select>
          <input
            v-model="tempValueCustomName"
            type="text"
            placeholder="自定义名称（可选）"
            class="input-field"
          />
          <button class="btn-add" @click="addValueField">添加</button>
        </div>
        <div class="field-list">
          <div v-for="(field, index) in valueFields" :key="index" class="field-item">
            <span>{{ field.field }} ({{ aggregationTypes.find(a => a.value === field.aggregation)?.label }})</span>
            <button class="btn-remove" @click="removeValueField(index)">✕</button>
          </div>
        </div>
      </div>

      <!-- 筛选字段 -->
      <div class="field-section">
        <label>筛选字段</label>
        <div class="field-input-group">
          <input
            v-model="tempFilterField"
            type="text"
            placeholder="输入字段名（如：产品类别）"
            class="input-field"
            @keyup.enter="addFilterField"
          />
          <button class="btn-add" @click="addFilterField">添加</button>
        </div>
        <div class="field-list">
          <div v-for="(field, index) in filterFields" :key="index" class="field-item">
            <span>{{ field }}</span>
            <button class="btn-remove" @click="removeFilterField(index)">✕</button>
          </div>
        </div>
      </div>
    </div>

    <div class="dialog-footer">
      <button class="btn-cancel" @click="closeDialog">取消</button>
      <button class="btn-apply" @click="applyPivot">应用</button>
    </div>
  </div>
</template>

<style scoped>
.pivot-table-dialog {
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

.input-value {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #d0d0d0;
  border-radius: 6px;
  font-size: 14px;
  transition: all 0.2s;
}

.input-value:focus {
  outline: none;
  border-color: #2196F3;
  box-shadow: 0 0 0 3px rgba(33, 150, 243, 0.1);
}

.field-section {
  margin-bottom: 24px;
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
}

.field-section label {
  display: block;
  margin-bottom: 10px;
  font-size: 14px;
  font-weight: 600;
  color: #333;
}

.field-input-group {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}

.value-field-inputs {
  display: grid;
  grid-template-columns: 2fr 1.5fr 1.5fr auto;
  gap: 8px;
  margin-bottom: 12px;
}

.input-field,
.select-aggregation {
  padding: 8px 12px;
  border: 1px solid #d0d0d0;
  border-radius: 6px;
  font-size: 14px;
  transition: all 0.2s;
}

.input-field:focus,
.select-aggregation:focus {
  outline: none;
  border-color: #2196F3;
  box-shadow: 0 0 0 3px rgba(33, 150, 243, 0.1);
}

.btn-add {
  padding: 8px 16px;
  background: #4CAF50;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: all 0.2s;
  white-space: nowrap;
}

.btn-add:hover {
  background: #45a049;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(76, 175, 80, 0.3);
}

.field-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.field-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 20px;
  font-size: 13px;
  color: #555;
}

.btn-remove {
  background: none;
  border: none;
  color: #999;
  cursor: pointer;
  font-size: 16px;
  padding: 0;
  line-height: 1;
  transition: color 0.2s;
}

.btn-remove:hover {
  color: #f44336;
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
