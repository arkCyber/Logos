<script setup lang="ts">
import { ref, computed } from 'vue';

const emit = defineEmits<{
  insert: [formula: string];
  close: [];
}>();

// 函数分类
const functionCategories = [
  { id: 'math', name: '数学与三角函数', icon: '🔢' },
  { id: 'statistical', name: '统计函数', icon: '📊' },
  { id: 'logical', name: '逻辑函数', icon: '🔀' },
  { id: 'text', name: '文本函数', icon: '📝' },
  { id: 'date', name: '日期与时间', icon: '📅' },
  { id: 'lookup', name: '查找与引用', icon: '🔍' },
  { id: 'financial', name: '财务函数', icon: '💰' },
  { id: 'engineering', name: '工程函数', icon: '⚙️' }
] as const;

// 函数库
const functionLibrary: Record<string, FunctionInfo[]> = {
  math: [
    { name: 'SUM', syntax: 'SUM(number1, [number2], ...)', description: '求和', example: 'SUM(A1:A10)' },
    { name: 'AVERAGE', syntax: 'AVERAGE(number1, [number2], ...)', description: '计算平均值', example: 'AVERAGE(A1:A10)' },
    { name: 'MAX', syntax: 'MAX(number1, [number2], ...)', description: '返回最大值', example: 'MAX(A1:A10)' },
    { name: 'MIN', syntax: 'MIN(number1, [number2], ...)', description: '返回最小值', example: 'MIN(A1:A10)' },
    { name: 'ROUND', syntax: 'ROUND(number, num_digits)', description: '四舍五入', example: 'ROUND(3.14159, 2)' },
    { name: 'ABS', syntax: 'ABS(number)', description: '返回绝对值', example: 'ABS(-5)' },
    { name: 'POWER', syntax: 'POWER(number, power)', description: '返回数字的幂', example: 'POWER(2, 3)' },
    { name: 'SQRT', syntax: 'SQRT(number)', description: '返回平方根', example: 'SQRT(16)' },
    { name: 'MOD', syntax: 'MOD(number, divisor)', description: '返回余数', example: 'MOD(10, 3)' }
  ],
  statistical: [
    { name: 'COUNT', syntax: 'COUNT(value1, [value2], ...)', description: '计算包含数字的单元格个数', example: 'COUNT(A1:A10)' },
    { name: 'COUNTA', syntax: 'COUNTA(value1, [value2], ...)', description: '计算非空单元格个数', example: 'COUNTA(A1:A10)' },
    { name: 'STDEV', syntax: 'STDEV(number1, [number2], ...)', description: '估算样本标准偏差', example: 'STDEV(A1:A10)' },
    { name: 'VAR', syntax: 'VAR(number1, [number2], ...)', description: '估算样本方差', example: 'VAR(A1:A10)' }
  ],
  logical: [
    { name: 'IF', syntax: 'IF(logical_test, value_if_true, [value_if_false])', description: '根据条件返回不同值', example: 'IF(A1>10, "高", "低")' },
    { name: 'AND', syntax: 'AND(logical1, [logical2], ...)', description: '所有条件都为真时返回TRUE', example: 'AND(A1>0, B1<10)' },
    { name: 'OR', syntax: 'OR(logical1, [logical2], ...)', description: '任一条件为真时返回TRUE', example: 'OR(A1>10, B1<5)' },
    { name: 'NOT', syntax: 'NOT(logical)', description: '对逻辑值求反', example: 'NOT(A1>10)' }
  ],
  text: [
    { name: 'CONCAT', syntax: 'CONCAT(text1, [text2], ...)', description: '连接文本', example: 'CONCAT("Hello", " ", "World")' },
    { name: 'LEFT', syntax: 'LEFT(text, [num_chars])', description: '从左侧提取指定字符', example: 'LEFT("Hello", 2)' },
    { name: 'RIGHT', syntax: 'RIGHT(text, [num_chars])', description: '从右侧提取指定字符', example: 'RIGHT("Hello", 2)' },
    { name: 'MID', syntax: 'MID(text, start_num, num_chars)', description: '从指定位置提取字符', example: 'MID("Hello", 2, 2)' },
    { name: 'LEN', syntax: 'LEN(text)', description: '返回文本长度', example: 'LEN("Hello")' },
    { name: 'UPPER', syntax: 'UPPER(text)', description: '转换为大写', example: 'UPPER("hello")' },
    { name: 'LOWER', syntax: 'LOWER(text)', description: '转换为小写', example: 'LOWER("HELLO")' },
    { name: 'TRIM', syntax: 'TRIM(text)', description: '删除空格', example: 'TRIM("  hello  ")' }
  ],
  date: [
    { name: 'TODAY', syntax: 'TODAY()', description: '返回当前日期', example: 'TODAY()' },
    { name: 'NOW', syntax: 'NOW()', description: '返回当前日期和时间', example: 'NOW()' },
    { name: 'YEAR', syntax: 'YEAR(serial_number)', description: '返回年份', example: 'YEAR(A1)' },
    { name: 'MONTH', syntax: 'MONTH(serial_number)', description: '返回月份', example: 'MONTH(A1)' },
    { name: 'DAY', syntax: 'DAY(serial_number)', description: '返回日期', example: 'DAY(A1)' }
  ],
  lookup: [
    { name: 'VLOOKUP', syntax: 'VLOOKUP(lookup_value, table_array, col_index_num, [range_lookup])', description: '垂直查找', example: 'VLOOKUP(A1, B1:D10, 2, FALSE)' },
    { name: 'HLOOKUP', syntax: 'HLOOKUP(lookup_value, table_array, row_index_num, [range_lookup])', description: '水平查找', example: 'HLOOKUP(A1, B1:D10, 2, FALSE)' },
    { name: 'INDEX', syntax: 'INDEX(array, row_num, [column_num])', description: '返回数组中的值', example: 'INDEX(A1:C10, 2, 3)' },
    { name: 'MATCH', syntax: 'MATCH(lookup_value, lookup_array, [match_type])', description: '返回匹配项的位置', example: 'MATCH("Apple", A1:A10, 0)' }
  ],
  financial: [
    { name: 'PMT', syntax: 'PMT(rate, nper, pv, [fv], [type])', description: '计算贷款偿还额', example: 'PMT(0.05/12, 60, 10000)' },
    { name: 'PV', syntax: 'PV(rate, nper, pmt, [fv], [type])', description: '计算现值', example: 'PV(0.05/12, 60, 200)' },
    { name: 'FV', syntax: 'FV(rate, nper, pmt, [pv], [type])', description: '计算未来值', example: 'FV(0.05/12, 60, 200)' }
  ],
  engineering: [
    { name: 'CONVERT', syntax: 'CONVERT(number, from_unit, to_unit)', description: '单位转换', example: 'CONVERT(100, "m", "ft")' },
    { name: 'DEC2HEX', syntax: 'DEC2HEX(number, [places])', description: '十进制转十六进制', example: 'DEC2HEX(255)' },
    { name: 'HEX2DEC', syntax: 'HEX2DEC(number)', description: '十六进制转十进制', example: 'HEX2DEC("FF")' }
  ]
};

interface FunctionInfo {
  name: string;
  syntax: string;
  description: string;
  example: string;
}

// 状态
const selectedCategory = ref('math');
const searchQuery = ref('');
const selectedFunction = ref<FunctionInfo | null>(null);

// 过滤后的函数列表
const filteredFunctions = computed(() => {
  const functions = functionLibrary[selectedCategory.value] || [];
  if (!searchQuery.value) {
    return functions;
  }
  const query = searchQuery.value.toLowerCase();
  return functions.filter(
    (f) =>
      f.name.toLowerCase().includes(query) ||
      f.description.toLowerCase().includes(query)
  );
});

// 选择分类
const selectCategory = (categoryId: string) => {
  selectedCategory.value = categoryId;
  selectedFunction.value = null;
};

// 选择函数
const selectFunction = (func: FunctionInfo) => {
  selectedFunction.value = func;
};

// 插入函数
const insertFunction = () => {
  if (selectedFunction.value) {
    emit('insert', `=${selectedFunction.value.name}()`);
    closeDialog();
  }
};

// 关闭对话框
const closeDialog = () => {
  emit('close');
};
</script>

<template>
  <div class="function-library-dialog">
    <div class="dialog-header">
      <h3>函数库</h3>
      <button class="btn-close" @click="closeDialog">✕</button>
    </div>

    <div class="dialog-body">
      <!-- 搜索框 -->
      <div class="search-box">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="搜索函数..."
          class="input-search"
        />
      </div>

      <div class="library-content">
        <!-- 分类列表 -->
        <div class="category-list">
          <div
            v-for="category in functionCategories"
            :key="category.id"
            :class="['category-item', { active: selectedCategory === category.id }]"
            @click="selectCategory(category.id)"
          >
            <span class="category-icon">{{ category.icon }}</span>
            <span class="category-name">{{ category.name }}</span>
          </div>
        </div>

        <!-- 函数列表 -->
        <div class="function-list">
          <div
            v-for="func in filteredFunctions"
            :key="func.name"
            :class="['function-item', { active: selectedFunction?.name === func.name }]"
            @click="selectFunction(func)"
          >
            <div class="function-name">{{ func.name }}</div>
            <div class="function-desc">{{ func.description }}</div>
          </div>
        </div>

        <!-- 函数详情 -->
        <div class="function-detail">
          <div v-if="selectedFunction" class="detail-content">
            <h4 class="detail-name">{{ selectedFunction.name }}</h4>
            <div class="detail-section">
              <label>语法</label>
              <code class="syntax-code">{{ selectedFunction.syntax }}</code>
            </div>
            <div class="detail-section">
              <label>说明</label>
              <p class="detail-description">{{ selectedFunction.description }}</p>
            </div>
            <div class="detail-section">
              <label>示例</label>
              <code class="example-code">{{ selectedFunction.example }}</code>
            </div>
            <button class="btn-insert" @click="insertFunction">插入函数</button>
          </div>
          <div v-else class="detail-placeholder">
            <p>选择一个函数查看详情</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.function-library-dialog {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 800px;
  max-height: 85vh;
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
  overflow: hidden;
  display: flex;
  flex-direction: column;
  padding: 0;
}

.search-box {
  padding: 12px 20px;
  border-bottom: 1px solid #e0e0e0;
}

.input-search {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #d0d0d0;
  border-radius: 6px;
  font-size: 14px;
  transition: all 0.2s;
}

.input-search:focus {
  outline: none;
  border-color: #2196F3;
  box-shadow: 0 0 0 3px rgba(33, 150, 243, 0.1);
}

.library-content {
  flex: 1;
  display: grid;
  grid-template-columns: 200px 250px 1fr;
  overflow: hidden;
}

.category-list {
  border-right: 1px solid #e0e0e0;
  overflow-y: auto;
}

.category-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  cursor: pointer;
  transition: all 0.2s;
  border-bottom: 1px solid #f0f0f0;
}

.category-item:hover {
  background: #f8f9fa;
}

.category-item.active {
  background: #e3f2fd;
  border-left: 3px solid #2196F3;
}

.category-icon {
  font-size: 18px;
}

.category-name {
  font-size: 14px;
  color: #333;
}

.function-list {
  border-right: 1px solid #e0e0e0;
  overflow-y: auto;
}

.function-item {
  padding: 12px 16px;
  cursor: pointer;
  transition: all 0.2s;
  border-bottom: 1px solid #f0f0f0;
}

.function-item:hover {
  background: #f8f9fa;
}

.function-item.active {
  background: #e3f2fd;
}

.function-name {
  font-size: 14px;
  font-weight: 600;
  color: #333;
  margin-bottom: 4px;
}

.function-desc {
  font-size: 12px;
  color: #666;
}

.function-detail {
  overflow-y: auto;
  padding: 20px;
}

.detail-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.detail-name {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #333;
}

.detail-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.detail-section label {
  font-size: 13px;
  font-weight: 600;
  color: #555;
}

.syntax-code,
.example-code {
  padding: 8px 12px;
  background: #f5f5f5;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  font-size: 13px;
  color: #333;
  word-break: break-all;
}

.detail-description {
  margin: 0;
  font-size: 14px;
  color: #555;
  line-height: 1.5;
}

.btn-insert {
  padding: 10px 20px;
  background: #2196F3;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  align-self: flex-start;
}

.btn-insert:hover {
  background: #1976D2;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(33, 150, 243, 0.3);
}

.detail-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #999;
  font-size: 14px;
}
</style>
