<script setup lang="ts">
import { ref, computed } from 'vue';
import BaseDialog from './BaseDialog.vue';

interface Props {
  show: boolean;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'apply-style', style: StyleDefinition): void;
}

interface StyleDefinition {
  type: 'paragraph' | 'character';
  name: string;
  basedOn?: string;
  following?: string;
  formatting: {
    font?: string;
    size?: number;
    bold?: boolean;
    italic?: boolean;
    underline?: boolean;
    color?: string;
    backgroundColor?: string;
    alignment?: 'left' | 'center' | 'right' | 'justify';
    lineSpacing?: number;
    spacingBefore?: number;
    spacingAfter?: number;
    indent?: number;
  };
}

defineProps<Props>();
const emit = defineEmits<Emits>();

// Tab state
const activeTab = ref<'paragraph' | 'character'>('paragraph');

// Style list
const paragraphStyles = ref<StyleDefinition[]>([
  {
    type: 'paragraph',
    name: '正常',
    formatting: {
      font: 'Calibri',
      size: 11,
      alignment: 'left',
      lineSpacing: 1.15
    }
  },
  {
    type: 'paragraph',
    name: '无间距',
    formatting: {
      font: 'Calibri',
      size: 11,
      alignment: 'left',
      lineSpacing: 1,
      spacingBefore: 0,
      spacingAfter: 0
    }
  },
  {
    type: 'paragraph',
    name: '标题1',
    formatting: {
      font: 'Calibri',
      size: 18,
      bold: true,
      alignment: 'left',
      spacingBefore: 12,
      spacingAfter: 8
    }
  },
  {
    type: 'paragraph',
    name: '标题2',
    formatting: {
      font: 'Calibri',
      size: 14,
      bold: true,
      alignment: 'left',
      spacingBefore: 12,
      spacingAfter: 4
    }
  },
  {
    type: 'paragraph',
    name: '引用',
    formatting: {
      font: 'Calibri',
      size: 11,
      italic: true,
      alignment: 'left',
      indent: 24,
      spacingBefore: 6,
      spacingAfter: 6
    }
  }
]);

const characterStyles = ref<StyleDefinition[]>([
  {
    type: 'character',
    name: '默认段落字体',
    formatting: {
      font: 'Calibri',
      size: 11
    }
  },
  {
    type: 'character',
    name: '强调',
    formatting: {
      font: 'Calibri',
      size: 11,
      italic: true,
      color: '#FF0000'
    }
  },
  {
    type: 'character',
    name: '强强调',
    formatting: {
      font: 'Calibri',
      size: 11,
      bold: true,
      color: '#FF0000'
    }
  },
  {
    type: 'character',
    name: '书名',
    formatting: {
      font: 'Calibri',
      size: 11,
      italic: true
    }
  }
]);

// Selected style
const selectedStyle = ref<StyleDefinition | null>(null);
const isEditing = ref(false);

// New style form
const newStyleForm = ref<StyleDefinition>({ // eslint-disable-line @typescript-eslint/no-unused-vars
  type: 'paragraph',
  name: '',
  formatting: {
    font: 'Calibri',
    size: 11
  }
});

// Computed
const currentStyles = computed(() => {
  return activeTab.value === 'paragraph' ? paragraphStyles.value : characterStyles.value;
});

const filteredStyles = computed(() => {
  return currentStyles.value;
});

// Select style
const selectStyle = (style: StyleDefinition) => {
  selectedStyle.value = style;
  isEditing.value = false;
};

// Create new style
const createNewStyle = () => {
  selectedStyle.value = {
    type: activeTab.value,
    name: '',
    formatting: {
      font: 'Calibri',
      size: 11
    }
  };
  isEditing.value = true;
};

// Edit style
const editStyle = () => {
  if (selectedStyle.value) {
    isEditing.value = true;
  }
};

// Delete style
const deleteStyle = () => {
  if (selectedStyle.value && confirm('确定要删除此样式吗？')) {
    const styles = activeTab.value === 'paragraph' ? paragraphStyles.value : characterStyles.value;
    const index = styles.findIndex(s => s.name === selectedStyle.value?.name);
    if (index > -1) {
      styles.splice(index, 1);
      selectedStyle.value = null;
    }
  }
};

// Save style
const saveStyle = () => {
  if (!selectedStyle.value || !selectedStyle.value.name.trim()) {
    alert('请输入样式名称');
    return;
  }

  const styles = activeTab.value === 'paragraph' ? paragraphStyles.value : characterStyles.value;
  const existingIndex = styles.findIndex(s => s.name === selectedStyle.value!.name);

  if (existingIndex > -1) {
    styles[existingIndex] = { ...selectedStyle.value };
  } else {
    styles.push({ ...selectedStyle.value });
  }

  isEditing.value = false;
};

// Cancel edit
const cancelEdit = () => {
  isEditing.value = false;
  if (!selectedStyle.value?.name) {
    selectedStyle.value = null;
  }
};

// Apply style
const applyStyle = () => {
  if (selectedStyle.value) {
    emit('apply-style', selectedStyle.value);
    emit('update:show', false);
  }
};

// Close dialog
const handleClose = () => {
  emit('update:show', false);
};
</script>

<template>
  <BaseDialog
    :show="show"
    title="样式"
    width="700px"
    height="500px"
    @update:show="handleClose"
  >
    <div class="style-manager-dialog">
      <!-- Tab buttons -->
      <div class="style-tabs">
        <button
          class="style-tab"
          :class="{ active: activeTab === 'paragraph' }"
          type="button"
          @click="activeTab = 'paragraph'"
        >
          段落样式
        </button>
        <button
          class="style-tab"
          :class="{ active: activeTab === 'character' }"
          type="button"
          @click="activeTab = 'character'"
        >
          字符样式
        </button>
      </div>

      <div class="style-content">
        <!-- Style list -->
        <div class="style-list">
          <div class="style-list-header">
            <button class="style-list-btn" type="button" @click="createNewStyle">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              >
                <line x1="12" y1="5" x2="12" y2="19" />
                <line x1="5" y1="12" x2="19" y2="12" />
              </svg>
              新建样式
            </button>
          </div>

          <div class="style-items">
            <div
              v-for="style in filteredStyles"
              :key="style.name"
              class="style-item"
              :class="{ selected: selectedStyle?.name === style.name }"
              @click="selectStyle(style)"
            >
              <div class="style-preview">
                <div
                  class="style-preview-text"
                  :style="{
                    fontFamily: style.formatting.font,
                    fontSize: style.formatting.size + 'px',
                    fontWeight: style.formatting.bold ? 'bold' : 'normal',
                    fontStyle: style.formatting.italic ? 'italic' : 'normal',
                    textDecoration: style.formatting.underline ? 'underline' : 'none',
                    color: style.formatting.color,
                    backgroundColor: style.formatting.backgroundColor,
                    textAlign: style.formatting.alignment
                  }"
                >
                  {{ style.name }}
                </div>
              </div>
              <span class="style-name">{{ style.name }}</span>
            </div>
          </div>
        </div>

        <!-- Style details -->
        <div class="style-details">
          <div v-if="!selectedStyle" class="style-empty">
            <p>选择一个样式查看详情，或创建新样式</p>
          </div>

          <div v-else-if="!isEditing" class="style-info">
            <div class="style-info-header">
              <h3>{{ selectedStyle.name }}</h3>
              <div class="style-info-actions">
                <button class="style-action-btn" type="button" @click="editStyle">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                  >
                    <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
                    <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
                  </svg>
                  修改
                </button>
                <button class="style-action-btn danger" type="button" @click="deleteStyle">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                  >
                    <polyline points="3 6 5 6 21 6" />
                    <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
                  </svg>
                  删除
                </button>
              </div>
            </div>

            <div class="style-preview-large">
              <div
                class="style-preview-large-text"
                :style="{
                  fontFamily: selectedStyle.formatting.font,
                  fontSize: selectedStyle.formatting.size + 'px',
                  fontWeight: selectedStyle.formatting.bold ? 'bold' : 'normal',
                  fontStyle: selectedStyle.formatting.italic ? 'italic' : 'normal',
                  textDecoration: selectedStyle.formatting.underline ? 'underline' : 'none',
                  color: selectedStyle.formatting.color,
                  backgroundColor: selectedStyle.formatting.backgroundColor,
                  textAlign: selectedStyle.formatting.alignment,
                  lineHeight: selectedStyle.formatting.lineSpacing,
                  paddingTop: selectedStyle.formatting.spacingBefore + 'px',
                  paddingBottom: selectedStyle.formatting.spacingAfter + 'px',
                  paddingLeft: selectedStyle.formatting.indent + 'px'
                }"
              >
                样式预览文本
              </div>
            </div>

            <div class="style-properties">
              <div class="style-property">
                <span class="property-label">字体:</span>
                <span class="property-value">{{ selectedStyle.formatting.font }}</span>
              </div>
              <div class="style-property">
                <span class="property-label">字号:</span>
                <span class="property-value">{{ selectedStyle.formatting.size }}pt</span>
              </div>
              <div v-if="selectedStyle.formatting.bold" class="style-property">
                <span class="property-label">粗体:</span>
                <span class="property-value">是</span>
              </div>
              <div v-if="selectedStyle.formatting.italic" class="style-property">
                <span class="property-label">斜体:</span>
                <span class="property-value">是</span>
              </div>
              <div v-if="selectedStyle.formatting.underline" class="style-property">
                <span class="property-label">下划线:</span>
                <span class="property-value">是</span>
              </div>
              <div v-if="selectedStyle.formatting.color" class="style-property">
                <span class="property-label">颜色:</span>
                <span class="property-value">{{ selectedStyle.formatting.color }}</span>
              </div>
            </div>

            <button class="apply-style-btn" type="button" @click="applyStyle">
              应用样式
            </button>
          </div>

          <div v-else class="style-editor">
            <div class="style-editor-header">
              <h3>{{ selectedStyle.name ? '修改样式' : '新建样式' }}</h3>
            </div>

            <div class="style-editor-form">
              <div class="form-group">
                <label for="style-name">样式名称</label>
                <input
                  id="style-name"
                  v-model="selectedStyle.name"
                  type="text"
                  placeholder="输入样式名称"
                />
              </div>

              <div class="form-group">
                <label for="style-font">字体</label>
                <select id="style-font" v-model="selectedStyle.formatting.font">
                  <option value="Calibri">Calibri</option>
                  <option value="'Microsoft YaHei'">微软雅黑</option>
                  <option value="'SimSun'">宋体</option>
                  <option value="Arial">Arial</option>
                  <option value="'Times New Roman'">Times New Roman</option>
                </select>
              </div>

              <div class="form-group">
                <label for="style-size">字号</label>
                <input
                  id="style-size"
                  v-model.number="selectedStyle.formatting.size"
                  type="number"
                  min="8"
                  max="72"
                />
              </div>

              <div class="form-group checkboxes">
                <label>
                  <input
                    v-model="selectedStyle.formatting.bold"
                    type="checkbox"
                  />
                  粗体
                </label>
                <label>
                  <input
                    v-model="selectedStyle.formatting.italic"
                    type="checkbox"
                  />
                  斜体
                </label>
                <label>
                  <input
                    v-model="selectedStyle.formatting.underline"
                    type="checkbox"
                  />
                  下划线
                </label>
              </div>

              <div class="form-group">
                <label for="style-color">颜色</label>
                <input
                  id="style-color"
                  v-model="selectedStyle.formatting.color"
                  type="color"
                />
              </div>

              <template v-if="selectedStyle.type === 'paragraph'">
                <div class="form-group">
                  <label for="style-alignment">对齐</label>
                  <select id="style-alignment" v-model="selectedStyle.formatting.alignment">
                    <option value="left">左对齐</option>
                    <option value="center">居中</option>
                    <option value="right">右对齐</option>
                    <option value="justify">两端对齐</option>
                  </select>
                </div>

                <div class="form-group">
                  <label for="style-line-spacing">行距</label>
                  <input
                    id="style-line-spacing"
                    v-model.number="selectedStyle.formatting.lineSpacing"
                    type="number"
                    step="0.1"
                    min="0.5"
                    max="3"
                  />
                </div>

                <div class="form-group">
                  <label for="style-spacing-before">段前间距</label>
                  <input
                    id="style-spacing-before"
                    v-model.number="selectedStyle.formatting.spacingBefore"
                    type="number"
                    min="0"
                  />
                </div>

                <div class="form-group">
                  <label for="style-spacing-after">段后间距</label>
                  <input
                    id="style-spacing-after"
                    v-model.number="selectedStyle.formatting.spacingAfter"
                    type="number"
                    min="0"
                  />
                </div>

                <div class="form-group">
                  <label for="style-indent">首行缩进</label>
                  <input
                    id="style-indent"
                    v-model.number="selectedStyle.formatting.indent"
                    type="number"
                    min="0"
                  />
                </div>
              </template>
            </div>

            <div class="style-editor-actions">
              <button class="dialog-btn secondary" type="button" @click="cancelEdit">
                取消
              </button>
              <button class="dialog-btn primary" type="button" @click="saveStyle">
                保存
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </BaseDialog>
</template>

<style scoped>
.style-manager-dialog {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.style-tabs {
  display: flex;
  border-bottom: 1px solid var(--word-border);
  margin-bottom: 16px;
}

.style-tab {
  padding: 12px 24px;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  cursor: pointer;
  font-size: 14px;
  color: var(--word-text-secondary);
  transition: all 0.15s ease;
}

.style-tab:hover {
  color: var(--word-text-primary);
  background: var(--word-button-hover);
}

.style-tab.active {
  color: var(--word-text-primary);
  border-bottom-color: var(--word-button-pressed);
  font-weight: 600;
}

.style-content {
  display: flex;
  gap: 16px;
  flex: 1;
  min-height: 0;
}

.style-list {
  width: 200px;
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--word-border);
  padding-right: 16px;
}

.style-list-header {
  margin-bottom: 8px;
}

.style-list-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  color: var(--word-text-primary);
  transition: all 0.15s ease;
}

.style-list-btn:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

.style-items {
  flex: 1;
  overflow-y: auto;
}

.style-item {
  padding: 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s ease;
  margin-bottom: 4px;
}

.style-item:hover {
  background: var(--word-button-hover);
}

.style-item.selected {
  background: var(--word-button-active);
  border: 1px solid var(--word-button-pressed);
}

.style-preview {
  margin-bottom: 4px;
}

.style-preview-text {
  padding: 4px;
  background: white;
  border: 1px solid var(--word-border);
  min-height: 24px;
}

.style-name {
  font-size: 12px;
  color: var(--word-text-secondary);
}

.style-details {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
}

.style-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--word-text-secondary);
}

.style-info {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.style-info-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--word-border);
}

.style-info-header h3 {
  margin: 0;
  font-size: 16px;
  color: var(--word-text-primary);
}

.style-info-actions {
  display: flex;
  gap: 8px;
}

.style-action-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 12px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  color: var(--word-text-primary);
  transition: all 0.15s ease;
}

.style-action-btn:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
}

.style-action-btn.danger {
  color: #dc2626;
}

.style-action-btn.danger:hover {
  background: #fef2f2;
  border-color: #dc2626;
}

.style-preview-large {
  padding: 20px;
  background: white;
  border: 1px solid var(--word-border);
  border-radius: 4px;
  min-height: 100px;
}

.style-preview-large-text {
  color: var(--word-text-primary);
}

.style-properties {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.style-property {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
}

.property-label {
  color: var(--word-text-secondary);
}

.property-value {
  color: var(--word-text-primary);
  font-weight: 500;
}

.apply-style-btn {
  padding: 10px 24px;
  background: var(--word-button-active);
  border: none;
  border-radius: 4px;
  color: var(--word-text-primary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.apply-style-btn:hover {
  background: var(--word-button-pressed);
}

.style-editor {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.style-editor-header h3 {
  margin: 0;
  font-size: 16px;
  color: var(--word-text-primary);
}

.style-editor-form {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-group label {
  font-size: 13px;
  color: var(--word-text-primary);
  font-weight: 500;
}

.form-group input,
.form-group select {
  padding: 8px 12px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 13px;
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

.form-group.checkboxes {
  flex-direction: row;
  gap: 16px;
}

.form-group.checkboxes label {
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
}

.style-editor-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding-top: 12px;
  border-top: 1px solid var(--word-border);
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
:global(.dark) .style-preview-text,
:global(.dark) .style-preview-large-text {
  background: var(--word-bg-canvas);
}
</style>
