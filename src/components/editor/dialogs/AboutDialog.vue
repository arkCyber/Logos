<script setup lang="ts">
import { ref } from 'vue';
import BaseDialog from './BaseDialog.vue';

interface Props {
  show: boolean;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const appVersion = ref('1.0.0');
const buildDate = ref('2026-06-01');
</script>

<template>
  <BaseDialog :show="show" title="关于" @update:show="emit('update:show', $event)">
    <div class="about-content">
      <div class="about-logo">
        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/>
          <polyline points="14 2 14 8 20 8"/>
        </svg>
      </div>
      <h2 class="about-title">Logos智道办公软件</h2>
      <p class="about-version">版本 {{ appVersion }}</p>
      <p class="about-build">构建日期: {{ buildDate }}</p>

      <div class="about-section">
        <h3>功能特性</h3>
        <ul>
          <li>富文本编辑器</li>
          <li>Typst 排版支持</li>
          <li>电子表格集成</li>
          <li>AI 智能体助手</li>
          <li>PDF/文档导出</li>
        </ul>
      </div>

      <div class="about-section">
        <h3>技术栈</h3>
        <ul>
          <li>Vue 3 + TypeScript</li>
          <li>Tauri</li>
          <li>TipTap 编辑器</li>
          <li>Typst</li>
        </ul>
      </div>

      <div class="about-footer">
        <p>© 2026 Logos智道办公软件. All rights reserved.</p>
      </div>
    </div>
  </BaseDialog>
</template>

<style scoped>
.about-content {
  padding: 20px;
  text-align: center;
}

.about-logo {
  display: flex;
  justify-content: center;
  margin-bottom: 16px;
  color: #007bff;
}

.about-title {
  font-size: 24px;
  font-weight: 600;
  margin: 0 0 8px 0;
  color: #333;
}

.about-version {
  font-size: 16px;
  color: #666;
  margin: 0 0 4px 0;
}

.about-build {
  font-size: 14px;
  color: #999;
  margin: 0 0 24px 0;
}

.about-section {
  text-align: left;
  margin: 20px 0;
  padding: 16px;
  background: #f5f5f5;
  border-radius: 8px;
}

.about-section h3 {
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 12px 0;
  color: #333;
}

.about-section ul {
  margin: 0;
  padding-left: 20px;
  list-style: disc;
}

.about-section li {
  font-size: 14px;
  color: #666;
  margin: 4px 0;
}

.about-footer {
  margin-top: 24px;
  padding-top: 16px;
  border-top: 1px solid #e5e5e5;
}

.about-footer p {
  font-size: 12px;
  color: #999;
  margin: 0;
}
</style>
