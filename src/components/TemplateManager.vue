<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { logger, LogCategory } from '../utils/logger';

interface TemplateVariable {
  name: string;
  default_value?: string;
  description?: string;
  required: boolean;
}

interface TemplateMetadata {
  category: string;
  author?: string;
  version?: string;
  created_at: string;
  updated_at: string;
  thumbnail?: string;
  previous_versions?: TemplateVersionHistory[];
}

interface TemplateVersionHistory {
  version: string;
  updated_at: string;
  content_hash: string;
}

interface Template {
  name: string;
  description: string;
  content: string;
  variables: TemplateVariable[];
  metadata: TemplateMetadata;
}

const templates = ref<Template[]>([]);
const searchQuery = ref('');
const selectedCategory = ref('');
const filteredTemplates = computed(() => {
  let result = templates.value;

  if (selectedCategory.value) {
    result = result.filter(t => t.metadata.category === selectedCategory.value);
  }

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    result = result.filter(t =>
      t.name.toLowerCase().includes(query) ||
      t.description.toLowerCase().includes(query)
    );
  }

  return result;
});

const showCreateDialog = ref(false);
const showImportDialog = ref(false);
const showDetailsDialog = ref(false);
const showApplyDialog = ref(false);
const showEditMetadataDialog = ref(false);

const selectedTemplate = ref<Template | null>(null);
const templateToApply = ref<Template | null>(null);
const variableValues = ref<Record<string, string>>({});
const metadataUpdates = ref({
  category: '',
  author: '',
  version: ''
});

const newTemplate = ref({
  name: '',
  description: '',
  category: 'Custom',
  content: '',
  author: '',
  version: '1.0.0'
});

onMounted(async () => {
  await loadTemplates();
});

async function loadTemplates() {
  try {
    templates.value = await invoke('get_all_typist_templates');
  } catch (error) {
    logger.error('Failed to load templates', error as Error, LogCategory.SYSTEM);
  }
}

function handleSearch() {
  // Filter is handled by computed property
}

function handleCategoryChange() {
  // Filter is handled by computed property
}

function getCategoryLabel(category: string | undefined): string {
  if (!category) {
return '未知';
}
  const labels: Record<string, string> = {
    Academic: '学术类',
    Business: '商业类',
    Technical: '技术类',
    Creative: '创意类',
    Presentation: '演示类',
    Custom: '自定义'
  };
  return labels[category] || category;
}

function formatDate(dateString: string | undefined): string {
  if (!dateString) {
return '未知';
}
  return new Date(dateString).toLocaleString('zh-CN');
}

function selectTemplate(template: Template) {
  selectedTemplate.value = template;
}

function showTemplateDetails(template: Template) {
  selectedTemplate.value = template;
  showDetailsDialog.value = true;
}

async function applyTemplate(template: Template) {
  templateToApply.value = template;
  variableValues.value = {};
  
  // Initialize with default values
  template.variables.forEach(variable => {
    if (variable.default_value) {
      variableValues.value[variable.name] = variable.default_value;
    }
  });
  
  if (template.variables.length > 0) {
    showApplyDialog.value = true;
  } else {
    await confirmApplyTemplate();
  }
}

async function confirmApplyTemplate() {
  if (!templateToApply.value) {
return;
}
  
  try {
    const rendered = await invoke<string>('render_typist_template', {
      name: templateToApply.value.name,
      values: variableValues.value
    });
    
    // Emit event to parent component with rendered content
    emit('template-applied', {
      template: templateToApply.value as Template,
      rendered,
      values: variableValues.value
    });
    
    showApplyDialog.value = false;
  } catch (error) {
    logger.error('Failed to apply template', error as Error, LogCategory.SYSTEM);
    alert('应用模板失败: ' + error);
  }
}

async function createTemplate() {
  try {
    // Validate template name
    const name = newTemplate.value.name.trim();
    if (!name) {
      alert('模板名称不能为空');
      return;
    }
    
    // Validate template name format (alphanumeric, hyphens, underscores)
    if (!/^[a-zA-Z0-9_-]+$/.test(name)) {
      alert('模板名称只能包含字母、数字、连字符和下划线');
      return;
    }
    
    // Validate content is not empty
    if (!newTemplate.value.content.trim()) {
      alert('模板内容不能为空');
      return;
    }
    
    // Validate version format
    if (newTemplate.value.version && !/^\d+\.\d+\.\d+$/.test(newTemplate.value.version)) {
      alert('版本号格式应为 x.y.z (例如: 1.0.0)');
      return;
    }
    
    const template: Template = {
      name,
      description: newTemplate.value.description.trim(),
      content: newTemplate.value.content,
      variables: extractVariables(newTemplate.value.content),
      metadata: {
        category: newTemplate.value.category,
        author: newTemplate.value.author?.trim() || undefined,
        version: newTemplate.value.version || undefined,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      }
    };
    
    await invoke('register_typist_template', { template });
    
    // Reset form
    newTemplate.value = {
      name: '',
      description: '',
      category: 'Custom',
      content: '',
      author: '',
      version: '1.0.0'
    };
    
    showCreateDialog.value = false;
    await loadTemplates();
  } catch (error) {
    logger.error('Failed to create template', error as Error, LogCategory.SYSTEM);
    alert('创建模板失败: ' + error);
  }
}

function extractVariables(content: string): TemplateVariable[] {
  const variables: TemplateVariable[] = [];
  const seen = new Set<string>();
  const regex = /\{\{(\w+)\}\}/g;
  let match;
  
  while ((match = regex.exec(content)) !== null) {
    const varName = match[1];
    if (!seen.has(varName)) {
      seen.add(varName);
      variables.push({
        name: varName,
        required: false
      });
    }
  }
  
  return variables;
}

async function handleFileImport(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  
  if (!file) {
return;
}
  
  try {
    const content = await file.text();
    
    // Determine format based on file extension
    let format = 'json';
    if (file.name.endsWith('.yaml') || file.name.endsWith('.yml')) {
      format = 'yaml';
    } else if (file.name.endsWith('.toml')) {
      format = 'toml';
    } else if (file.name.endsWith('.typ')) {
      format = 'typ';
    }
    
    // Use backend import command
    const templateName = await invoke<string>('import_typist_template', {
      data: content,
      format: format
    });
    
    showImportDialog.value = false;
    await loadTemplates();
    alert(`模板 "${templateName}" 导入成功`);
  } catch (error) {
    logger.error('Failed to import template', error as Error, LogCategory.SYSTEM);
    alert('导入模板失败: ' + error);
  }
}

async function exportTemplate(template: Template) {
  try {
    // Show format selection dialog
    const format = prompt('选择导出格式:\n1. JSON\n2. YAML\n3. TOML\n4. Typ (原始内容)\n\n输入数字 (1-4):', '1');
    
    let formatString = 'json';
    let extension = 'json';
    
    switch (format) {
      case '1':
        formatString = 'json';
        extension = 'json';
        break;
      case '2':
        formatString = 'yaml';
        extension = 'yaml';
        break;
      case '3':
        formatString = 'toml';
        extension = 'toml';
        break;
      case '4':
        formatString = 'typ';
        extension = 'typ';
        break;
      default:
        alert('无效的选择，使用默认格式 JSON');
        formatString = 'json';
        extension = 'json';
    }
    
    const exported = await invoke<string>('export_typist_template', {
      name: template.name,
      format: formatString
    });
    
    const blob = new Blob([exported], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${template.name}.${extension}`;
    a.click();
    URL.revokeObjectURL(url);
  } catch (error) {
    logger.error('Failed to export template', error as Error, LogCategory.SYSTEM);
    alert('导出模板失败: ' + error);
  }
}

async function deleteTemplate(name: string) {
  if (!confirm(`确定要删除模板 "${name}" 吗？`)) {
return;
}
  
  try {
    await invoke('remove_typist_template', { name });
    await loadTemplates();
  } catch (error) {
    logger.error('Failed to delete template', error as Error, LogCategory.SYSTEM);
    alert('删除模板失败: ' + error);
  }
}

async function generatePreview(template: Template) {
  try {
    const previewData = await invoke<number[]>('generate_typist_template_preview', {
      name: template.name
    });
    
    // Convert array to base64 for display
    const base64 = btoa(String.fromCharCode(...new Uint8Array(previewData)));
    const imageUrl = `data:image/png;base64,${base64}`;
    
    // Show preview in a new window or dialog
    const previewWindow = window.open('', '_blank');
    if (previewWindow) {
      previewWindow.document.write(`
        <html>
          <head><title>预览: ${template.name}</title></head>
          <body style="margin: 0; display: flex; justify-content: center; align-items: center; min-height: 100vh; background: #f0f0f0;">
            <img src="${imageUrl}" style="max-width: 100%; max-height: 100vh; box-shadow: 0 4px 20px rgba(0,0,0,0.1);" />
          </body>
        </html>
      `);
      previewWindow.document.close();
    }
  } catch (error) {
    logger.error('Failed to generate preview', error as Error, LogCategory.SYSTEM);
    alert('生成预览失败: ' + error);
  }
}

async function updateMetadata() {
  if (!selectedTemplate.value) {
return;
}
  
  try {
    const updates: Record<string, string> = {};
    
    if (metadataUpdates.value.category) {
      updates.category = metadataUpdates.value.category;
    }
    if (metadataUpdates.value.author) {
      updates.author = metadataUpdates.value.author;
    }
    if (metadataUpdates.value.version) {
      updates.version = metadataUpdates.value.version;
    }
    
    await invoke('update_typist_template_metadata', {
      name: selectedTemplate.value.name,
      updates
    });
    
    // Reset form
    metadataUpdates.value = {
      category: '',
      author: '',
      version: ''
    };
    
    showEditMetadataDialog.value = false;
    await loadTemplates();
    alert('元数据更新成功');
  } catch (error) {
    logger.error('Failed to update metadata', error as Error, LogCategory.SYSTEM);
    alert('更新元数据失败: ' + error);
  }
}

async function loadFromDirectory() {
  const directory = prompt('输入模板目录路径:', '/Users/arksong/LOGOS/src-tauri/typist-templates');
  
  if (!directory) {
return;
}
  
  try {
    const count = await invoke<number>('load_typist_templates_from_directory', {
      dir: directory
    });
    
    await loadTemplates();
    alert(`成功从目录加载 ${count} 个模板`);
  } catch (error) {
    logger.error('Failed to load templates from directory', error as Error, LogCategory.SYSTEM);
    alert('从目录加载模板失败: ' + error);
  }
}

const emit = defineEmits<{
  'template-applied': [data: { template: Template; rendered: string; values: Record<string, string> }];
}>();
</script>

<template>
  <div class="template-manager">
    <div class="template-header">
      <h2>模板管理器</h2>
      <div class="template-actions">
        <button class="btn-primary" @click="showCreateDialog = true">
          <span class="icon">+</span> 创建模板
        </button>
        <button class="btn-secondary" @click="showImportDialog = true">
          <span class="icon">📥</span> 导入模板
        </button>
        <button class="btn-secondary" @click="loadFromDirectory">
          <span class="icon">📁</span> 从目录加载
        </button>
      </div>
    </div>

    <div class="template-filters">
      <div class="search-box">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="搜索模板..."
          @input="handleSearch"
        />
      </div>
      <div class="category-filter">
        <select v-model="selectedCategory" @change="handleCategoryChange">
          <option value="">所有分类</option>
          <option value="Academic">学术类</option>
          <option value="Business">商业类</option>
          <option value="Technical">技术类</option>
          <option value="Creative">创意类</option>
          <option value="Presentation">演示类</option>
          <option value="Custom">自定义</option>
        </select>
      </div>
    </div>

    <div v-if="filteredTemplates.length > 0" class="template-grid">
      <div
        v-for="template in filteredTemplates"
        :key="template.name"
        class="template-card"
        @click="selectTemplate(template)"
      >
        <div class="template-preview">
          <img
            v-if="template.metadata.thumbnail"
            :src="template.metadata.thumbnail"
            :alt="template.name"
          />
          <div v-else class="placeholder-preview">
            <span>{{ template.name.charAt(0).toUpperCase() }}</span>
          </div>
        </div>
        <div class="template-info">
          <h3>{{ template.name }}</h3>
          <p>{{ template.description }}</p>
          <div class="template-meta">
            <span class="category">{{ getCategoryLabel(template.metadata.category) }}</span>
            <span v-if="template.metadata.version" class="version">
              v{{ template.metadata.version }}
            </span>
            <span v-if="template.metadata.author" class="author">
              {{ template.metadata.author }}
            </span>
          </div>
        </div>
        <div class="template-actions-overlay">
          <button class="btn-apply" @click.stop="applyTemplate(template)">
            应用模板
          </button>
          <button class="btn-details" @click.stop="showTemplateDetails(template)">
            详情
          </button>
          <button class="btn-preview" @click.stop="generatePreview(template)">
            生成预览
          </button>
          <button class="btn-export" @click.stop="exportTemplate(template)">
            导出
          </button>
          <button
            v-if="template.metadata.category === 'Custom'"
            class="btn-delete"
            @click.stop="deleteTemplate(template.name)"
          >
            删除
          </button>
        </div>
      </div>
    </div>

    <div v-else class="empty-state">
      <p>没有找到匹配的模板</p>
    </div>

    <!-- Create Template Dialog -->
    <div v-if="showCreateDialog" class="dialog-overlay" @click="showCreateDialog = false">
      <div class="dialog" @click.stop>
        <h3>创建自定义模板</h3>
        <form @submit.prevent="createTemplate">
          <div class="form-group">
            <label>模板名称</label>
            <input v-model="newTemplate.name" type="text" required />
          </div>
          <div class="form-group">
            <label>描述</label>
            <input v-model="newTemplate.description" type="text" required />
          </div>
          <div class="form-group">
            <label>分类</label>
            <select v-model="newTemplate.category" required>
              <option value="Academic">学术类</option>
              <option value="Business">商业类</option>
              <option value="Technical">技术类</option>
              <option value="Creative">创意类</option>
              <option value="Presentation">演示类</option>
              <option value="Custom">自定义</option>
            </select>
          </div>
          <div class="form-group">
            <label>模板内容</label>
            <textarea
              v-model="newTemplate.content"
              rows="10"
              placeholder="使用 {{variable_name}} 作为变量占位符"
              required
            ></textarea>
          </div>
          <div class="form-group">
            <label>作者</label>
            <input v-model="newTemplate.author" type="text" />
          </div>
          <div class="form-group">
            <label>版本</label>
            <input v-model="newTemplate.version" type="text" placeholder="1.0.0" />
          </div>
          <div class="dialog-actions">
            <button type="button" class="btn-secondary" @click="showCreateDialog = false">
              取消
            </button>
            <button type="submit" class="btn-primary">创建</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Import Template Dialog -->
    <div v-if="showImportDialog" class="dialog-overlay" @click="showImportDialog = false">
      <div class="dialog" @click.stop>
        <h3>导入模板</h3>
        <div class="form-group">
          <label>选择文件</label>
          <input type="file" accept=".typ,.json,.yaml,.yml,.toml" @change="handleFileImport" />
        </div>
        <div class="dialog-actions">
          <button type="button" class="btn-secondary" @click="showImportDialog = false">
            取消
          </button>
        </div>
      </div>
    </div>

    <!-- Template Details Dialog -->
    <div v-if="showDetailsDialog" class="dialog-overlay" @click="showDetailsDialog = false">
      <div class="dialog dialog-large" @click.stop>
        <h3>{{ selectedTemplate?.name }}</h3>
        <div class="template-details">
          <div class="detail-section">
            <h4>描述</h4>
            <p>{{ selectedTemplate?.description }}</p>
          </div>
          <div class="detail-section">
            <h4>元数据</h4>
            <div class="metadata-grid">
              <div>
                <span class="label">分类:</span>
                <span>{{ getCategoryLabel(selectedTemplate?.metadata.category) }}</span>
              </div>
              <div>
                <span class="label">作者:</span>
                <span>{{ selectedTemplate?.metadata.author || '未知' }}</span>
              </div>
              <div>
                <span class="label">版本:</span>
                <span>{{ selectedTemplate?.metadata.version || '未指定' }}</span>
              </div>
              <div>
                <span class="label">创建时间:</span>
                <span>{{ formatDate(selectedTemplate?.metadata.created_at) }}</span>
              </div>
              <div>
                <span class="label">更新时间:</span>
                <span>{{ formatDate(selectedTemplate?.metadata.updated_at) }}</span>
              </div>
            </div>
            <button class="btn-secondary" style="margin-top: 10px;" @click="showEditMetadataDialog = true">
              编辑元数据
            </button>
          </div>
          <div v-if="selectedTemplate?.metadata.previous_versions && selectedTemplate.metadata.previous_versions.length > 0" class="detail-section">
            <h4>版本历史</h4>
            <div class="version-history">
              <div v-for="(version, index) in selectedTemplate.metadata.previous_versions" :key="index" class="version-item">
                <span class="version-number">v{{ version.version }}</span>
                <span class="version-date">{{ formatDate(version.updated_at) }}</span>
                <span class="version-hash">{{ version.content_hash.substring(0, 8) }}</span>
              </div>
            </div>
          </div>
          <div class="detail-section">
            <h4>变量</h4>
            <div class="variables-list">
              <div v-for="variable in selectedTemplate?.variables" :key="variable.name" class="variable-item">
                <div class="variable-name">
                  {{ variable.name }}
                  <span v-if="variable.required" class="required">*</span>
                </div>
                <div class="variable-description">
                  {{ variable.description || '无描述' }}
                </div>
                <div v-if="variable.default_value" class="variable-default">
                  默认: {{ variable.default_value }}
                </div>
              </div>
            </div>
          </div>
          <div class="detail-section">
            <h4>预览</h4>
            <div class="preview-area">
              <pre>{{ selectedTemplate?.content }}</pre>
            </div>
          </div>
        </div>
        <div class="dialog-actions">
          <button class="btn-secondary" @click="showDetailsDialog = false">关闭</button>
          <button class="btn-primary" @click="selectedTemplate && applyTemplate(selectedTemplate)">应用模板</button>
        </div>
      </div>
    </div>

    <!-- Apply Template Dialog -->
    <div v-if="showApplyDialog" class="dialog-overlay" @click="showApplyDialog = false">
      <div class="dialog dialog-large" @click.stop>
        <h3>应用模板: {{ templateToApply?.name }}</h3>
        <div class="template-variables">
          <div
            v-for="variable in templateToApply?.variables"
            :key="variable.name"
            class="variable-input"
          >
            <label>
              {{ variable.name }}
              <span v-if="variable.required" class="required">*</span>
            </label>
            <input
              v-if="!variable.description?.includes('多行')"
              v-model="variableValues[variable.name]"
              type="text"
              :placeholder="variable.default_value || ''"
              :required="variable.required"
            />
            <textarea
              v-else
              v-model="variableValues[variable.name]"
              :placeholder="variable.default_value || ''"
              :required="variable.required"
              rows="3"
            ></textarea>
            <small v-if="variable.description">{{ variable.description }}</small>
          </div>
        </div>
        <div class="dialog-actions">
          <button class="btn-secondary" @click="showApplyDialog = false">取消</button>
          <button class="btn-primary" @click="confirmApplyTemplate">应用</button>
        </div>
      </div>
    </div>

    <!-- Edit Metadata Dialog -->
    <div v-if="showEditMetadataDialog" class="dialog-overlay" @click="showEditMetadataDialog = false">
      <div class="dialog" @click.stop>
        <h3>编辑元数据: {{ selectedTemplate?.name }}</h3>
        <form @submit.prevent="updateMetadata">
          <div class="form-group">
            <label>分类</label>
            <select v-model="metadataUpdates.category">
              <option value="">保持不变</option>
              <option value="Academic">学术类</option>
              <option value="Business">商业类</option>
              <option value="Technical">技术类</option>
              <option value="Creative">创意类</option>
              <option value="Presentation">演示类</option>
              <option value="Custom">自定义</option>
            </select>
          </div>
          <div class="form-group">
            <label>作者</label>
            <input v-model="metadataUpdates.author" type="text" placeholder="保持不变" />
          </div>
          <div class="form-group">
            <label>版本</label>
            <input v-model="metadataUpdates.version" type="text" placeholder="1.0.0" />
          </div>
          <div class="dialog-actions">
            <button type="button" class="btn-secondary" @click="showEditMetadataDialog = false">
              取消
            </button>
            <button type="submit" class="btn-primary">更新</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<style scoped>
.template-manager {
  padding: 20px;
  max-width: 1400px;
  margin: 0 auto;
}

.template-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.template-header h2 {
  margin: 0;
  font-size: 24px;
  color: #333;
}

.template-actions {
  display: flex;
  gap: 10px;
}

.btn-primary,
.btn-secondary,
.btn-apply,
.btn-details,
.btn-export,
.btn-delete,
.btn-preview {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: background-color 0.2s;
}

.btn-primary {
  background-color: #007bff;
  color: white;
}

.btn-primary:hover {
  background-color: #0056b3;
}

.btn-secondary {
  background-color: #6c757d;
  color: white;
}

.btn-secondary:hover {
  background-color: #545b62;
}

.btn-apply {
  background-color: #28a745;
  color: white;
}

.btn-apply:hover {
  background-color: #218838;
}

.btn-details {
  background-color: #17a2b8;
  color: white;
}

.btn-details:hover {
  background-color: #138496;
}

.btn-export {
  background-color: #ffc107;
  color: #212529;
}

.btn-export:hover {
  background-color: #e0a800;
}

.btn-delete {
  background-color: #dc3545;
  color: white;
}

.btn-delete:hover {
  background-color: #c82333;
}

.btn-preview {
  background-color: #6f42c1;
  color: white;
}

.btn-preview:hover {
  background-color: #5a32a3;
}

.icon {
  margin-right: 5px;
}

.template-filters {
  display: flex;
  gap: 15px;
  margin-bottom: 20px;
}

.search-box input,
.category-filter select {
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.search-box input {
  flex: 1;
  max-width: 300px;
}

.template-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
}

.template-card {
  border: 1px solid #ddd;
  border-radius: 8px;
  overflow: hidden;
  cursor: pointer;
  transition: box-shadow 0.2s, transform 0.2s;
  position: relative;
}

.template-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transform: translateY(-2px);
}

.template-preview {
  height: 160px;
  background-color: #f5f5f5;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.template-preview img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.placeholder-preview {
  width: 80px;
  height: 80px;
  background-color: #007bff;
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32px;
  font-weight: bold;
}

.template-info {
  padding: 15px;
}

.template-info h3 {
  margin: 0 0 8px 0;
  font-size: 16px;
  color: #333;
}

.template-info p {
  margin: 0 0 10px 0;
  font-size: 13px;
  color: #666;
  line-height: 1.4;
}

.template-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  font-size: 12px;
  color: #888;
}

.template-meta span {
  padding: 2px 6px;
  background-color: #f0f0f0;
  border-radius: 3px;
}

.template-actions-overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background-color: rgba(255, 255, 255, 0.95);
  padding: 10px;
  display: flex;
  gap: 5px;
  transform: translateY(100%);
  transition: transform 0.2s;
}

.template-card:hover .template-actions-overlay {
  transform: translateY(0);
}

.template-actions-overlay button {
  flex: 1;
  padding: 6px 8px;
  font-size: 12px;
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
  color: #888;
}

.version-history {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.version-item {
  display: flex;
  gap: 15px;
  padding: 8px;
  background-color: #f9f9f9;
  border-radius: 4px;
  font-size: 13px;
}

.version-number {
  font-weight: bold;
  color: #007bff;
}

.version-date {
  color: #666;
}

.version-hash {
  color: #999;
  font-family: monospace;
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background-color: white;
  border-radius: 8px;
  padding: 24px;
  max-width: 500px;
  width: 90%;
  max-height: 90vh;
  overflow-y: auto;
}

.dialog-large {
  max-width: 800px;
}

.dialog h3 {
  margin: 0 0 20px 0;
  font-size: 20px;
  color: #333;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-weight: 500;
  color: #333;
}

.form-group input,
.form-group select,
.form-group textarea {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  box-sizing: border-box;
}

.form-group textarea {
  resize: vertical;
  font-family: monospace;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 20px;
}

.template-details {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.detail-section h4 {
  margin: 0 0 10px 0;
  font-size: 16px;
  color: #333;
}

.metadata-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 10px;
}

.metadata-grid > div {
  display: flex;
  gap: 8px;
}

.metadata-grid .label {
  font-weight: 500;
  color: #666;
}

.variables-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.variable-item {
  padding: 10px;
  background-color: #f5f5f5;
  border-radius: 4px;
}

.variable-name {
  font-weight: 500;
  color: #333;
}

.required {
  color: #dc3545;
}

.variable-description {
  font-size: 13px;
  color: #666;
  margin-top: 4px;
}

.variable-default {
  font-size: 12px;
  color: #888;
  margin-top: 4px;
}

.preview-area {
  background-color: #f5f5f5;
  padding: 15px;
  border-radius: 4px;
  max-height: 300px;
  overflow-y: auto;
}

.preview-area pre {
  margin: 0;
  white-space: pre-wrap;
  word-wrap: break-word;
  font-family: monospace;
  font-size: 13px;
}

.template-variables {
  display: flex;
  flex-direction: column;
  gap: 15px;
  max-height: 60vh;
  overflow-y: auto;
}

.variable-input label {
  display: block;
  margin-bottom: 6px;
  font-weight: 500;
  color: #333;
}

.variable-input input,
.variable-input textarea {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  box-sizing: border-box;
}

.variable-input small {
  display: block;
  margin-top: 4px;
  font-size: 12px;
  color: #666;
}
</style>
