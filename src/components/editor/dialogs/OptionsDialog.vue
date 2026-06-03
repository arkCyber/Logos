<script setup lang="ts">
import { ref, computed } from 'vue';
import BaseDialog from './BaseDialog.vue';

interface Props {
  show: boolean;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'apply', settings: OptionsSettings): void;
}

interface OptionsSettings {
  general: {
    autoSave: boolean;
    autoSaveInterval: number;
    showMiniToolbar: boolean;
    enableAnimations: boolean;
  };
  display: {
    theme: 'light' | 'dark' | 'auto';
    fontSize: number;
    showRuler: boolean;
    showGridLines: boolean;
  };
  editing: {
    smartQuotes: boolean;
    autoCorrect: boolean;
    spellCheck: boolean;
    grammarCheck: boolean;
  };
  advanced: {
    enableTelemetry: boolean;
    checkUpdates: boolean;
    hardwareAcceleration: boolean;
  };
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// Tab state
const activeTab = ref<'general' | 'display' | 'editing' | 'advanced'>('general');

// Settings state
const settings = ref<OptionsSettings>({
  general: {
    autoSave: true,
    autoSaveInterval: 30,
    showMiniToolbar: true,
    enableAnimations: true
  },
  display: {
    theme: 'light',
    fontSize: 11,
    showRuler: true,
    showGridLines: false
  },
  editing: {
    smartQuotes: true,
    autoCorrect: true,
    spellCheck: true,
    grammarCheck: false
  },
  advanced: {
    enableTelemetry: false,
    checkUpdates: true,
    hardwareAcceleration: true
  }
});

// Computed
const tabTitle = computed(() => {
  const titles = {
    general: '常规',
    display: '显示',
    editing: '编辑',
    advanced: '高级'
  };
  return titles[activeTab.value];
});

// Apply settings
const handleApply = () => {
  emit('apply', settings.value);
  emit('update:show', false);
};

// Cancel
const handleCancel = () => {
  emit('update:show', false);
};

// Reset to defaults
const handleReset = () => {
  if (confirm('确定要重置所有设置为默认值吗？')) {
    settings.value = {
      general: {
        autoSave: true,
        autoSaveInterval: 30,
        showMiniToolbar: true,
        enableAnimations: true
      },
      display: {
        theme: 'light',
        fontSize: 11,
        showRuler: true,
        showGridLines: false
      },
      editing: {
        smartQuotes: true,
        autoCorrect: true,
        spellCheck: true,
        grammarCheck: false
      },
      advanced: {
        enableTelemetry: false,
        checkUpdates: true,
        hardwareAcceleration: true
      }
    };
  }
};
</script>

<template>
  <BaseDialog
    :show="show"
    title="选项"
    width="700px"
    height="550px"
    @update:show="handleCancel"
  >
    <div class="options-dialog">
      <!-- Sidebar tabs -->
      <div class="options-sidebar">
        <button
          class="sidebar-tab"
          :class="{ active: activeTab === 'general' }"
          type="button"
          @click="activeTab = 'general'"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="18"
            height="18"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <circle cx="12" cy="12" r="3" />
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
          </svg>
          常规
        </button>
        <button
          class="sidebar-tab"
          :class="{ active: activeTab === 'display' }"
          type="button"
          @click="activeTab = 'display'"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="18"
            height="18"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <rect x="2" y="3" width="20" height="14" rx="2" />
            <line x1="8" y1="21" x2="16" y2="21" />
            <line x1="12" y1="17" x2="12" y2="21" />
          </svg>
          显示
        </button>
        <button
          class="sidebar-tab"
          :class="{ active: activeTab === 'editing' }"
          type="button"
          @click="activeTab = 'editing'"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="18"
            height="18"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
            <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
          </svg>
          编辑
        </button>
        <button
          class="sidebar-tab"
          :class="{ active: activeTab === 'advanced' }"
          type="button"
          @click="activeTab = 'advanced'"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="18"
            height="18"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <circle cx="12" cy="12" r="10" />
            <line x1="12" y1="16" x2="12" y2="12" />
            <line x1="12" y1="8" x2="12.01" y2="8" />
          </svg>
          高级
        </button>
      </div>

      <!-- Content area -->
      <div class="options-content">
        <h3 class="content-title">{{ tabTitle }}</h3>

        <!-- General Tab -->
        <div v-if="activeTab === 'general'" class="options-section">
          <div class="option-group">
            <h4 class="group-title">保存</h4>
            <label class="option-item">
              <input v-model="settings.general.autoSave" type="checkbox" />
              <div class="option-info">
                <span class="option-label">自动保存</span>
                <span class="option-description">每隔指定时间自动保存文档</span>
              </div>
            </label>
            <div v-if="settings.general.autoSave" class="option-item nested">
              <label for="auto-save-interval">自动保存间隔（秒）</label>
              <input
                id="auto-save-interval"
                v-model.number="settings.general.autoSaveInterval"
                type="number"
                min="10"
                max="300"
              />
            </div>
          </div>

          <div class="option-group">
            <h4 class="group-title">界面</h4>
            <label class="option-item">
              <input v-model="settings.general.showMiniToolbar" type="checkbox" />
              <div class="option-info">
                <span class="option-label">显示迷你工具栏</span>
                <span class="option-description">选中文本时显示快速格式化工具栏</span>
              </div>
            </label>
            <label class="option-item">
              <input v-model="settings.general.enableAnimations" type="checkbox" />
              <div class="option-info">
                <span class="option-label">启用动画</span>
                <span class="option-description">启用界面动画效果</span>
              </div>
            </label>
          </div>
        </div>

        <!-- Display Tab -->
        <div v-if="activeTab === 'display'" class="options-section">
          <div class="option-group">
            <h4 class="group-title">主题</h4>
            <div class="option-item">
              <label for="theme-select">主题</label>
              <select id="theme-select" v-model="settings.display.theme">
                <option value="light">浅色</option>
                <option value="dark">深色</option>
                <option value="auto">跟随系统</option>
              </select>
            </div>
          </div>

          <div class="option-group">
            <h4 class="group-title">字体</h4>
            <div class="option-item">
              <label for="font-size-select">默认字号</label>
              <select id="font-size-select" v-model="settings.display.fontSize">
                <option :value="10">10pt</option>
                <option :value="11">11pt</option>
                <option :value="12">12pt</option>
                <option :value="14">14pt</option>
              </select>
            </div>
          </div>

          <div class="option-group">
            <h4 class="group-title">页面元素</h4>
            <label class="option-item">
              <input v-model="settings.display.showRuler" type="checkbox" />
              <div class="option-info">
                <span class="option-label">显示标尺</span>
                <span class="option-description">在编辑器中显示水平和垂直标尺</span>
              </div>
            </label>
            <label class="option-item">
              <input v-model="settings.display.showGridLines" type="checkbox" />
              <div class="option-info">
                <span class="option-label">显示网格线</span>
                <span class="option-description">在编辑器中显示网格线</span>
              </div>
            </label>
          </div>
        </div>

        <!-- Editing Tab -->
        <div v-if="activeTab === 'editing'" class="options-section">
          <div class="option-group">
            <h4 class="group-title">自动更正</h4>
            <label class="option-item">
              <input v-model="settings.editing.smartQuotes" type="checkbox" />
              <div class="option-info">
                <span class="option-label">智能引号</span>
                <span class="option-description">自动将直引号转换为弯引号</span>
              </div>
            </label>
            <label class="option-item">
              <input v-model="settings.editing.autoCorrect" type="checkbox" />
              <div class="option-info">
                <span class="option-label">自动更正</span>
                <span class="option-description">自动更正常见的拼写错误</span>
              </div>
            </label>
          </div>

          <div class="option-group">
            <h4 class="group-title">校对</h4>
            <label class="option-item">
              <input v-model="settings.editing.spellCheck" type="checkbox" />
              <div class="option-info">
                <span class="option-label">拼写检查</span>
                <span class="option-description">实时检查拼写错误</span>
              </div>
            </label>
            <label class="option-item">
              <input v-model="settings.editing.grammarCheck" type="checkbox" />
              <div class="option-info">
                <span class="option-label">语法检查</span>
                <span class="option-description">实时检查语法错误</span>
              </div>
            </label>
          </div>
        </div>

        <!-- Advanced Tab -->
        <div v-if="activeTab === 'advanced'" class="options-section">
          <div class="option-group">
            <h4 class="group-title">隐私</h4>
            <label class="option-item">
              <input v-model="settings.advanced.enableTelemetry" type="checkbox" />
              <div class="option-info">
                <span class="option-label">启用遥测</span>
                <span class="option-description">发送匿名使用数据以帮助改进产品</span>
              </div>
            </label>
          </div>

          <div class="option-group">
            <h4 class="group-title">更新</h4>
            <label class="option-item">
              <input v-model="settings.advanced.checkUpdates" type="checkbox" />
              <div class="option-info">
                <span class="option-label">自动检查更新</span>
                <span class="option-description">定期检查应用程序更新</span>
              </div>
            </label>
          </div>

          <div class="option-group">
            <h4 class="group-title">性能</h4>
            <label class="option-item">
              <input v-model="settings.advanced.hardwareAcceleration" type="checkbox" />
              <div class="option-info">
                <span class="option-label">硬件加速</span>
                <span class="option-description">使用 GPU 加速渲染（需要重启）</span>
              </div>
            </label>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <button class="dialog-btn secondary" type="button" @click="handleReset">
        重置默认
      </button>
      <button class="dialog-btn secondary" type="button" @click="handleCancel">
        取消
      </button>
      <button class="dialog-btn primary" type="button" @click="handleApply">
        确定
      </button>
    </template>
  </BaseDialog>
</template>

<style scoped>
.options-dialog {
  display: flex;
  height: 100%;
  gap: 0;
}

.options-sidebar {
  width: 180px;
  border-right: 1px solid var(--word-border);
  display: flex;
  flex-direction: column;
  padding: 16px 0;
  background: var(--word-button-bg);
}

.sidebar-tab {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 20px;
  background: transparent;
  border: none;
  border-left: 3px solid transparent;
  cursor: pointer;
  font-size: 14px;
  color: var(--word-text-secondary);
  transition: all 0.15s ease;
  text-align: left;
}

.sidebar-tab:hover {
  color: var(--word-text-primary);
  background: var(--word-button-hover);
}

.sidebar-tab.active {
  color: var(--word-text-primary);
  background: var(--word-button-active);
  border-left-color: var(--word-button-pressed);
  font-weight: 500;
}

.options-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 24px;
  overflow-y: auto;
}

.content-title {
  margin: 0 0 20px 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--word-text-primary);
}

.options-section {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.option-group {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.group-title {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--word-text-primary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.option-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 8px 0;
}

.option-item.nested {
  padding-left: 28px;
}

.option-item input[type="checkbox"] {
  margin-top: 2px;
  cursor: pointer;
}

.option-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.option-label {
  font-size: 14px;
  color: var(--word-text-primary);
  font-weight: 500;
}

.option-description {
  font-size: 12px;
  color: var(--word-text-secondary);
  line-height: 1.4;
}

.option-item label {
  font-size: 13px;
  color: var(--word-text-primary);
  font-weight: 500;
}

.option-item select,
.option-item input[type="number"] {
  padding: 6px 12px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 13px;
  min-width: 120px;
}

.option-item select:focus,
.option-item input[type="number"]:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

.dialog-btn {
  padding: 8px 24px;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border: none;
}

.dialog-btn.primary {
  background: var(--word-button-active);
  color: var(--word-text-primary);
}

.dialog-btn.primary:hover {
  background: var(--word-button-pressed);
}

.dialog-btn.secondary {
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  color: var(--word-text-primary);
}

.dialog-btn.secondary:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

/* Dark mode */
:global(.dark) .options-sidebar {
  background: var(--word-bg-canvas);
  border-right-color: var(--word-border);
}

:global(.dark) .sidebar-tab:hover {
  background: var(--word-button-hover);
}

:global(.dark) .sidebar-tab.active {
  background: var(--word-button-active);
}

:global(.dark) .option-item select,
:global(.dark) .option-item input[type="number"] {
  background: var(--word-bg-canvas);
  border-color: var(--word-border);
  color: var(--word-text-primary);
}

:global(.dark) .option-item select:focus,
:global(.dark) .option-item input[type="number"]:focus {
  border-color: var(--word-button-border-hover);
}
</style>
