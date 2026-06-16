<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue';
import { HelpCircle, Keyboard, Info, X, ExternalLink, BookOpen, MessageCircle, Github } from 'lucide-vue-next';

interface Props {
  show: boolean;
}

interface Emits {
  (e: 'close'): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

const activeTab = ref<'shortcuts' | 'about' | 'feedback'>('shortcuts');

const shortcuts = [
  { category: '文件操作', items: [
    { key: 'Ctrl+S', action: '保存文档' },
    { key: 'Ctrl+O', action: '打开文件' },
    { key: 'Ctrl+N', action: '新建文档' },
    { key: 'Ctrl+P', action: '打印文档' }
  ]},
  { category: '编辑操作', items: [
    { key: 'Ctrl+Z', action: '撤销' },
    { key: 'Ctrl+Y', action: '重做' },
    { key: 'Ctrl+C', action: '复制' },
    { key: 'Ctrl+V', action: '粘贴' },
    { key: 'Ctrl+X', action: '剪切' },
    { key: 'Ctrl+A', action: '全选' },
    { key: 'Ctrl+F', action: '查找' },
    { key: 'Ctrl+H', action: '替换' }
  ]},
  { category: '文本格式', items: [
    { key: 'Ctrl+B', action: '加粗' },
    { key: 'Ctrl+I', action: '斜体' },
    { key: 'Ctrl+U', action: '下划线' },
    { key: 'Ctrl+Shift+S', action: '删除线' }
  ]},
  { category: '段落格式', items: [
    { key: 'Ctrl+1', action: '标题 1' },
    { key: 'Ctrl+2', action: '标题 2' },
    { key: 'Ctrl+3', action: '标题 3' },
    { key: 'Ctrl+L', action: '左对齐' },
    { key: 'Ctrl+E', action: '居中对齐' },
    { key: 'Ctrl+R', action: '右对齐' },
    { key: 'Ctrl+J', action: '两端对齐' }
  ]},
  { category: '其他功能', items: [
    { key: 'F1', action: '打开帮助' },
    { key: 'Ctrl+/', action: '切换侧边栏' },
    { key: 'Escape', action: '关闭对话框' }
  ]}
];

const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    emit('close');
  }
};

onMounted(() => {
  document.addEventListener('keydown', handleKeyDown);
});

onBeforeUnmount(() => {
  document.removeEventListener('keydown', handleKeyDown);
});
</script>

<template>
  <Transition name="help">
    <div v-if="show" class="help-overlay" @click.self="emit('close')">
      <div class="help-dialog" @click.stop>
        <!-- Header -->
        <div class="help-header">
          <div class="help-title">
            <HelpCircle :size="24" />
            <h2>帮助</h2>
          </div>
          <button class="close-button" @click="emit('close')">
            <X :size="20" />
          </button>
        </div>

        <!-- Tabs -->
        <div class="help-tabs">
          <button
            class="tab-button"
            :class="{ active: activeTab === 'shortcuts' }"
            @click="activeTab = 'shortcuts'"
          >
            <Keyboard :size="18" />
            <span>快捷键</span>
          </button>
          <button
            class="tab-button"
            :class="{ active: activeTab === 'about' }"
            @click="activeTab = 'about'"
          >
            <Info :size="18" />
            <span>关于</span>
          </button>
          <button
            class="tab-button"
            :class="{ active: activeTab === 'feedback' }"
            @click="activeTab = 'feedback'"
          >
            <MessageCircle :size="18" />
            <span>反馈</span>
          </button>
        </div>

        <!-- Content -->
        <div class="help-content">
          <!-- Shortcuts Tab -->
          <div v-if="activeTab === 'shortcuts'" class="tab-panel">
            <div v-for="category in shortcuts" :key="category.category" class="shortcut-category">
              <h3>{{ category.category }}</h3>
              <div class="shortcut-list">
                <div v-for="item in category.items" :key="item.key" class="shortcut-item">
                  <kbd class="shortcut-key">{{ item.key }}</kbd>
                  <span class="shortcut-action">{{ item.action }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- About Tab -->
          <div v-if="activeTab === 'about'" class="tab-panel">
            <div class="about-section">
              <div class="about-logo">
                <div class="logo-icon">L</div>
                <h3>Logos智道办公软件</h3>
              </div>
              <p class="about-description">
                Logos智道办公软件是一款现代化的办公软件，基于 Tauri、Vue3、TipTap 和 Typst 构建。
                提供专业的文档编辑、实时预览和 AI 辅助功能。
              </p>
              <div class="about-info">
                <div class="info-item">
                  <span class="info-label">版本:</span>
                  <span class="info-value">1.0.0</span>
                </div>
                <div class="info-item">
                  <span class="info-label">构建:</span>
                  <span class="info-value">2024.06</span>
                </div>
              </div>
              <div class="about-links">
                <a href="https://github.com/your-org/LOGOS" target="_blank" class="link-button">
                  <Github :size="16" />
                  <span>GitHub</span>
                  <ExternalLink :size="14" />
                </a>
                <a href="#" class="link-button">
                  <BookOpen :size="16" />
                  <span>文档</span>
                  <ExternalLink :size="14" />
                </a>
              </div>
            </div>
          </div>

          <!-- Feedback Tab -->
          <div v-if="activeTab === 'feedback'" class="tab-panel">
            <div class="feedback-section">
              <h3>反馈与支持</h3>
              <p class="feedback-description">
                如果您遇到问题或有建议，请通过以下方式联系我们：
              </p>
              <div class="feedback-links">
                <a href="https://github.com/your-org/LOGOS/issues" target="_blank" class="feedback-link">
                  <Github :size="20" />
                  <div class="link-content">
                    <h4>GitHub Issues</h4>
                    <p>报告 Bug 或提交功能请求</p>
                  </div>
                  <ExternalLink :size="16" />
                </a>
                <a href="#" class="feedback-link">
                  <MessageCircle :size="20" />
                  <div class="link-content">
                    <h4>社区论坛</h4>
                    <p>与其他用户交流讨论</p>
                  </div>
                  <ExternalLink :size="16" />
                </a>
                <a href="#" class="feedback-link">
                  <BookOpen :size="20" />
                  <div class="link-content">
                    <h4>在线文档</h4>
                    <p>查看完整的使用指南</p>
                  </div>
                  <ExternalLink :size="16" />
                </a>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.help-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
}

.help-dialog {
  background-color: var(--word-bg, #ffffff);
  border-radius: 12px;
  box-shadow: 0 12px 48px rgba(0, 0, 0, 0.2);
  width: 90%;
  max-width: 700px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.help-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid var(--word-border, #e0e0e0);
}

.help-title {
  display: flex;
  align-items: center;
  gap: 12px;
}

.help-title h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--word-text-primary, #333);
}

.close-button {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  border-radius: 6px;
  color: var(--word-text-secondary, #666);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.close-button:hover {
  background-color: var(--word-button-hover, #f5f5f5);
  color: var(--word-text-primary, #333);
}

.help-tabs {
  display: flex;
  border-bottom: 1px solid var(--word-border, #e0e0e0);
  padding: 0 24px;
}

.tab-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 14px 20px;
  border: none;
  background: none;
  color: var(--word-text-secondary, #666);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border-bottom: 2px solid transparent;
}

.tab-button:hover {
  color: var(--word-text-primary, #333);
  background-color: var(--word-button-hover, #f5f5f5);
}

.tab-button.active {
  color: var(--word-accent, #007bff);
  border-bottom-color: var(--word-accent, #007bff);
}

.help-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.tab-panel {
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Shortcuts Styles */
.shortcut-category {
  margin-bottom: 24px;
}

.shortcut-category h3 {
  margin: 0 0 12px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--word-text-primary, #333);
}

.shortcut-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.shortcut-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 8px 0;
}

.shortcut-key {
  background-color: var(--word-button-bg, #f5f5f5);
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 4px;
  padding: 4px 10px;
  font-family: var(--word-font-ui, 'Segoe UI', system-ui);
  font-size: 13px;
  font-weight: 500;
  color: var(--word-text-primary, #333);
  min-width: 100px;
  text-align: center;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.shortcut-action {
  font-size: 14px;
  color: var(--word-text-secondary, #666);
}

/* About Styles */
.about-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 20px;
  padding: 20px 0;
}

.about-logo {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.logo-icon {
  width: 64px;
  height: 64px;
  background: linear-gradient(135deg, #007bff 0%, #0056b3 100%);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32px;
  font-weight: 700;
  color: white;
  box-shadow: 0 4px 12px rgba(0, 123, 255, 0.3);
}

.about-logo h3 {
  margin: 0;
  font-size: 24px;
  font-weight: 700;
  color: var(--word-text-primary, #333);
}

.about-description {
  max-width: 500px;
  font-size: 14px;
  line-height: 1.6;
  color: var(--word-text-secondary, #666);
}

.about-info {
  display: flex;
  gap: 32px;
  padding: 16px 24px;
  background-color: var(--word-button-bg, #f5f5f5);
  border-radius: 8px;
}

.info-item {
  display: flex;
  gap: 8px;
  font-size: 14px;
}

.info-label {
  color: var(--word-text-secondary, #666);
  font-weight: 500;
}

.info-value {
  color: var(--word-text-primary, #333);
  font-weight: 600;
}

.about-links {
  display: flex;
  gap: 12px;
}

.link-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background-color: var(--word-button-bg, #f5f5f5);
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 6px;
  color: var(--word-text-primary, #333);
  font-size: 14px;
  font-weight: 500;
  text-decoration: none;
  cursor: pointer;
  transition: all 0.2s;
}

.link-button:hover {
  background-color: var(--word-button-hover, #e8e8e8);
  border-color: var(--word-accent, #007bff);
  color: var(--word-accent, #007bff);
}

/* Feedback Styles */
.feedback-section {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.feedback-section h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--word-text-primary, #333);
}

.feedback-description {
  font-size: 14px;
  color: var(--word-text-secondary, #666);
  line-height: 1.6;
}

.feedback-links {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.feedback-link {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 20px;
  background-color: var(--word-button-bg, #f5f5f5);
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 8px;
  text-decoration: none;
  color: var(--word-text-primary, #333);
  transition: all 0.2s;
}

.feedback-link:hover {
  background-color: var(--word-button-hover, #e8e8e8);
  border-color: var(--word-accent, #007bff);
  transform: translateX(4px);
}

.link-content {
  flex: 1;
}

.link-content h4 {
  margin: 0 0 4px 0;
  font-size: 15px;
  font-weight: 600;
}

.link-content p {
  margin: 0;
  font-size: 13px;
  color: var(--word-text-secondary, #666);
}

/* Transition animations */
.help-enter-active,
.help-leave-active {
  transition: opacity 0.2s ease;
}

.help-enter-from,
.help-leave-to {
  opacity: 0;
}

.help-enter-active .help-dialog,
.help-leave-active .help-dialog {
  transition: transform 0.2s ease, opacity 0.2s ease;
}

.help-enter-from .help-dialog,
.help-leave-to .help-dialog {
  transform: scale(0.95);
  opacity: 0;
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  .help-dialog {
    background-color: #1e1e1e;
  }
  
  .help-header {
    border-bottom-color: #3e3e42;
  }
  
  .help-title h2 {
    color: #ffffff;
  }
  
  .close-button {
    color: #cccccc;
  }
  
  .close-button:hover {
    background-color: #2d2d30;
    color: #ffffff;
  }
  
  .help-tabs {
    border-bottom-color: #3e3e42;
  }
  
  .tab-button {
    color: #cccccc;
  }
  
  .tab-button:hover {
    color: #ffffff;
    background-color: #2d2d30;
  }
  
  .tab-button.active {
    color: #0078d4;
    border-bottom-color: #0078d4;
  }
  
  .shortcut-category h3 {
    color: #ffffff;
  }
  
  .shortcut-key {
    background-color: #2d2d30;
    border-color: #3e3e42;
    color: #ffffff;
  }
  
  .shortcut-action {
    color: #cccccc;
  }
  
  .about-logo h3 {
    color: #ffffff;
  }
  
  .about-description {
    color: #cccccc;
  }
  
  .about-info {
    background-color: #2d2d30;
  }
  
  .info-label {
    color: #cccccc;
  }
  
  .info-value {
    color: #ffffff;
  }
  
  .link-button {
    background-color: #2d2d30;
    border-color: #3e3e42;
    color: #ffffff;
  }
  
  .link-button:hover {
    background-color: #3e3e42;
    border-color: #0078d4;
    color: #0078d4;
  }
  
  .feedback-section h3 {
    color: #ffffff;
  }
  
  .feedback-description {
    color: #cccccc;
  }
  
  .feedback-link {
    background-color: #2d2d30;
    border-color: #3e3e42;
    color: #ffffff;
  }
  
  .feedback-link:hover {
    background-color: #3e3e42;
    border-color: #0078d4;
  }
  
  .link-content h4 {
    color: #ffffff;
  }
  
  .link-content p {
    color: #cccccc;
  }
}
</style>
