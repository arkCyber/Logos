<script setup lang="ts">
import { ref } from 'vue';

const emit = defineEmits<{ // eslint-disable-line @typescript-eslint/no-unused-vars
  insertContent: [code: string];
}>();

const isLoading = ref(false); // eslint-disable-line @typescript-eslint/no-unused-vars

// Univer集成需要进一步调试API类型
// 暂时显示占位符信息
</script>

<template>
  <div class="univer-spreadsheet-container">
    <div class="placeholder-message">
      <h3>Univer 集成</h3>
      <p>Univer集成需要进一步调试API类型</p>
      <p>当前使用 Luckysheet 作为电子表格解决方案</p>
    </div>
  </div>
</template>

<style scoped>
.univer-spreadsheet-container {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  background: #f8f9fa;
}

.placeholder-message {
  text-align: center;
  padding: 40px;
  color: #666;
}

.placeholder-message h3 {
  margin-bottom: 12px;
  color: #333;
}

.placeholder-message p {
  margin: 8px 0;
  font-size: 14px;
}
</style>
