<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { logger, LogCategory } from '../utils/logger';

interface TemplateVariable {
  name: string;
  default_value?: string;
  description?: string;
  required: boolean;
}

interface TemplateAuthor {
  name: string;
  avatar?: string;
  email?: string;
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

interface TemplateRating {
  average: number;
  count: number;
  userRating?: number;
}

interface TemplateComment {
  id: string;
  user: string;
  content: string;
  rating: number;
  created_at: string;
}

interface EnhancedTemplate {
  id: string;
  name: string;
  description: string;
  content: string;
  variables: TemplateVariable[];
  metadata: TemplateMetadata;
  rating: TemplateRating;
  downloads: number;
  author: TemplateAuthor;
  tags: string[];
  isOfficial: boolean;
  isFeatured: boolean;
  comments: TemplateComment[];
}

const templates = ref<EnhancedTemplate[]>([]);
const searchQuery = ref('');
const selectedCategory = ref('');
const selectedTags = ref<string[]>([]);
const sortBy = ref('popular'); // popular, newest, rating, downloads
const viewMode = ref<'grid' | 'list'>('grid');

const filteredTemplates = computed(() => {
  let result = templates.value;

  // 分类过滤
  if (selectedCategory.value) {
    result = result.filter(t => t.metadata.category === selectedCategory.value);
  }

  // 标签过滤
  if (selectedTags.value.length > 0) {
    result = result.filter(t => 
      selectedTags.value.some(tag => t.tags.includes(tag))
    );
  }

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    result = result.filter(t =>
      t.name.toLowerCase().includes(query) ||
      t.description.toLowerCase().includes(query) ||
      t.tags.some(tag => tag.toLowerCase().includes(query))
    );
  }

  // 排序
  switch (sortBy.value) {
    case 'popular':
      result = result.sort((a, b) => b.downloads - a.downloads);
      break;
    case 'newest':
      result = result.sort((a, b) => 
        new Date(b.metadata.created_at).getTime() - new Date(a.metadata.created_at).getTime()
      );
      break;
    case 'rating':
      result = result.sort((a, b) => b.rating.average - a.rating.average);
      break;
    case 'downloads':
      result = result.sort((a, b) => b.downloads - a.downloads);
      break;
  }

  return result;
});

const allTags = computed(() => {
  const tagSet = new Set<string>();
  templates.value.forEach(t => {
    t.tags.forEach(tag => tagSet.add(tag));
  });
  return Array.from(tagSet).sort();
});

const categories = computed(() => {
  const categorySet = new Set<string>();
  templates.value.forEach(t => {
    categorySet.add(t.metadata.category);
  });
  return Array.from(categorySet).sort();
});

// 对话框状态
const showCreateDialog = ref(false);
const showImportDialog = ref(false);
const showDetailsDialog = ref(false);
const showApplyDialog = ref(false);
const showEditMetadataDialog = ref(false);
const showRatingDialog = ref(false);
const showCommentDialog = ref(false);
const showShareDialog = ref(false);

const selectedTemplate = ref<EnhancedTemplate | null>(null);
const templateToApply = ref<EnhancedTemplate | null>(null);
const variableValues = ref<Record<string, string>>({});
const userRating = ref(0);
const userComment = ref('');

const newTemplate = ref({
  name: '',
  description: '',
  category: 'Custom',
  content: '',
  author: '',
  version: '1.0.0',
  tags: [] as string[]
});

onMounted(async () => {
  await loadTemplates();
});

async function loadTemplates() {
  try {
    const data = await invoke('get_all_typist_templates');
    // 转换为增强模板格式
    templates.value = (data as any[]).map(t => ({
      ...t,
      id: t.id || t.name,
      rating: t.rating || { average: 0, count: 0 },
      downloads: t.downloads || 0,
      author: t.author || { name: t.metadata?.author || 'Unknown' },
      tags: t.tags || [],
      isOfficial: t.isOfficial || false,
      isFeatured: t.isFeatured || false,
      comments: t.comments || []
    }));
    logger.info('Templates loaded', { count: templates.value.length }, LogCategory.BUSINESS);
  } catch (error) {
    logger.error('Failed to load templates', error as Error, LogCategory.SYSTEM);
  }
}

function toggleTag(tag: string) {
  const index = selectedTags.value.indexOf(tag);
  if (index > -1) {
    selectedTags.value.splice(index, 1);
  } else {
    selectedTags.value.push(tag);
  }
}

function showTemplateDetails(template: EnhancedTemplate) {
  selectedTemplate.value = template;
  showDetailsDialog.value = true;
}

async function applyTemplate(template: EnhancedTemplate) {
  templateToApply.value = template;
  variableValues.value = {};
  
  // 初始化默认值
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
    const result = await invoke('apply_typist_template', {
      templateId: templateToApply.value.id,
      variables: variableValues.value
    });
    
    logger.info('Template applied', { templateId: templateToApply.value.id }, LogCategory.BUSINESS);
    showApplyDialog.value = false;
    
    // 触发应用事件
    emit('applied', result);
  } catch (error) {
    logger.error('Failed to apply template', error as Error, LogCategory.BUSINESS);
  }
}

async function rateTemplate(template: EnhancedTemplate, rating: number) {
  try {
    await invoke('rate_typist_template', {
      templateId: template.id,
      rating
    });
    
    // 更新本地评分
    template.rating.userRating = rating;
    template.rating.count++;
    template.rating.average = (template.rating.average * (template.rating.count - 1) + rating) / template.rating.count;
    
    logger.info('Template rated', { templateId: template.id, rating }, LogCategory.BUSINESS);
    showRatingDialog.value = false;
  } catch (error) {
    logger.error('Failed to rate template', error as Error, LogCategory.BUSINESS);
  }
}

async function addComment(template: EnhancedTemplate) {
  if (!userComment.value.trim()) {
return;
}
  
  try {
    const comment = await invoke('add_typist_template_comment', {
      templateId: template.id,
      content: userComment.value,
      rating: userRating.value
    }) as TemplateComment;
    
    template.comments.push(comment);
    userComment.value = '';
    userRating.value = 0;
    
    logger.info('Comment added', { templateId: template.id }, LogCategory.BUSINESS);
    showCommentDialog.value = false;
  } catch (error) {
    logger.error('Failed to add comment', error as Error, LogCategory.BUSINESS);
  }
}

async function downloadTemplate(template: EnhancedTemplate) {
  try {
    await invoke('download_typist_template', { templateId: template.id });
    template.downloads++;
    
    logger.info('Template downloaded', { templateId: template.id }, LogCategory.BUSINESS);
  } catch (error) {
    logger.error('Failed to download template', error as Error, LogCategory.BUSINESS);
  }
}

async function shareTemplate(template: EnhancedTemplate) {
  selectedTemplate.value = template;
  showShareDialog.value = true;
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

function getStarRating(rating: number): string {
  const fullStars = Math.floor(rating);
  const hasHalfStar = rating % 1 >= 0.5;
  let stars = '⭐'.repeat(fullStars);
  if (hasHalfStar) {
stars += '⭐';
}
  return stars || '暂无评分';
}

const emit = defineEmits<{
  applied: [result: any];
  selected: [template: EnhancedTemplate];
}>();
</script>

<template>
  <div class="enhanced-template-library">
    <!-- 顶部工具栏 -->
    <div class="library-header">
      <h2>模板库</h2>
      
      <div class="header-actions">
        <div class="search-box">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索模板..."
            class="search-input"
          />
        </div>
        
        <button class="btn-primary" @click="showCreateDialog = true">
          + 创建模板
        </button>
        <button class="btn-secondary" @click="showImportDialog = true">
          导入模板
        </button>
      </div>
    </div>

    <!-- 过滤器和排序 -->
    <div class="library-filters">
      <div class="filter-group">
        <label>分类:</label>
        <select v-model="selectedCategory" class="filter-select">
          <option value="">全部</option>
          <option v-for="category in categories" :key="category" :value="category">
            {{ getCategoryLabel(category) }}
          </option>
        </select>
      </div>
      
      <div class="filter-group">
        <label>标签:</label>
        <div class="tags-filter">
          <button
            v-for="tag in allTags"
            :key="tag"
            class="tag-btn"
            :class="{ active: selectedTags.includes(tag) }"
            @click="toggleTag(tag)"
          >
            {{ tag }}
          </button>
        </div>
      </div>
      
      <div class="filter-group">
        <label>排序:</label>
        <select v-model="sortBy" class="filter-select">
          <option value="popular">热门</option>
          <option value="newest">最新</option>
          <option value="rating">评分</option>
          <option value="downloads">下载量</option>
        </select>
      </div>
      
      <div class="view-toggle">
        <button
          class="view-btn"
          :class="{ active: viewMode === 'grid' }"
          @click="viewMode = 'grid'"
        >
          ⊞
        </button>
        <button
          class="view-btn"
          :class="{ active: viewMode === 'list' }"
          @click="viewMode = 'list'"
        >
          ☰
        </button>
      </div>
    </div>

    <!-- 精选模板 -->
    <div v-if="templates.some(t => t.isFeatured)" class="featured-section">
      <h3>精选模板</h3>
      <div class="featured-grid">
        <div
          v-for="template in templates.filter(t => t.isFeatured)"
          :key="template.id"
          class="featured-card"
          @click="showTemplateDetails(template)"
        >
          <div class="featured-thumbnail">
            <img v-if="template.metadata.thumbnail" :src="template.metadata.thumbnail" :alt="template.name" />
            <div v-else class="placeholder-thumbnail">
              📄
            </div>
            <span class="featured-badge">精选</span>
          </div>
          <div class="featured-info">
            <h4>{{ template.name }}</h4>
            <p>{{ template.description }}</p>
            <div class="featured-meta">
              <span class="rating">{{ getStarRating(template.rating.average) }}</span>
              <span class="downloads">{{ template.downloads }} 下载</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 模板列表 -->
    <div class="templates-container">
      <div v-if="viewMode === 'grid'" class="templates-grid">
        <div
          v-for="template in filteredTemplates"
          :key="template.id"
          class="template-card"
          @click="showTemplateDetails(template)"
        >
          <div class="card-thumbnail">
            <img v-if="template.metadata.thumbnail" :src="template.metadata.thumbnail" :alt="template.name" />
            <div v-else class="placeholder-thumbnail">
              📄
            </div>
            <span v-if="template.isOfficial" class="official-badge">官方</span>
            <span v-if="template.isFeatured" class="featured-badge">精选</span>
          </div>
          
          <div class="card-content">
            <h4>{{ template.name }}</h4>
            <p class="description">{{ template.description }}</p>
            
            <div class="card-meta">
              <span class="category">{{ getCategoryLabel(template.metadata.category) }}</span>
              <span class="author">{{ template.author.name }}</span>
            </div>
            
            <div class="card-stats">
              <span class="rating" :title="`评分: ${template.rating.average.toFixed(1)}`">
                {{ getStarRating(template.rating.average) }}
              </span>
              <span class="downloads" :title="`下载量: ${template.downloads}`">
                ⬇️ {{ template.downloads }}
              </span>
              <span class="comments" :title="`评论数: ${template.comments.length}`">
                💬 {{ template.comments.length }}
              </span>
            </div>
            
            <div class="card-tags">
              <span v-for="tag in template.tags.slice(0, 3)" :key="tag" class="tag">
                {{ tag }}
              </span>
            </div>
          </div>
          
          <div class="card-actions">
            <button class="btn-use" @click.stop="applyTemplate(template)">
              使用
            </button>
            <button class="btn-download" @click.stop="downloadTemplate(template)">
              下载
            </button>
            <button class="btn-share" @click.stop="shareTemplate(template)">
              分享
            </button>
          </div>
        </div>
      </div>
      
      <div v-else class="templates-list">
        <div
          v-for="template in filteredTemplates"
          :key="template.id"
          class="list-item"
          @click="showTemplateDetails(template)"
        >
          <div class="list-thumbnail">
            <img v-if="template.metadata.thumbnail" :src="template.metadata.thumbnail" :alt="template.name" />
            <div v-else class="placeholder-thumbnail">📄</div>
          </div>
          
          <div class="list-content">
            <div class="list-header">
              <h4>{{ template.name }}</h4>
              <div class="list-badges">
                <span v-if="template.isOfficial" class="official-badge">官方</span>
                <span v-if="template.isFeatured" class="featured-badge">精选</span>
              </div>
            </div>
            
            <p class="description">{{ template.description }}</p>
            
            <div class="list-meta">
              <span class="category">{{ getCategoryLabel(template.metadata.category) }}</span>
              <span class="author">{{ template.author.name }}</span>
              <span class="version">{{ template.metadata.version }}</span>
              <span class="date">{{ formatDate(template.metadata.updated_at) }}</span>
            </div>
            
            <div class="list-stats">
              <span class="rating">{{ getStarRating(template.rating.average) }}</span>
              <span class="downloads">⬇️ {{ template.downloads }}</span>
              <span class="comments">💬 {{ template.comments.length }}</span>
            </div>
          </div>
          
          <div class="list-actions">
            <button class="btn-use" @click.stop="applyTemplate(template)">使用</button>
            <button class="btn-download" @click.stop="downloadTemplate(template)">下载</button>
          </div>
        </div>
      </div>
      
      <div v-if="filteredTemplates.length === 0" class="empty-state">
        <p>没有找到匹配的模板</p>
      </div>
    </div>

    <!-- 模板详情对话框 -->
    <div v-if="showDetailsDialog && selectedTemplate" class="dialog-overlay" @click.self="showDetailsDialog = false">
      <div class="dialog">
        <div class="dialog-header">
          <h2>{{ selectedTemplate.name }}</h2>
          <button class="btn-close" @click="showDetailsDialog = false">✕</button>
        </div>
        
        <div class="dialog-content">
          <div class="detail-thumbnail">
            <img v-if="selectedTemplate.metadata.thumbnail" :src="selectedTemplate.metadata.thumbnail" />
            <div v-else class="placeholder-thumbnail">📄</div>
          </div>
          
          <div class="detail-info">
            <p class="description">{{ selectedTemplate.description }}</p>
            
            <div class="detail-meta">
              <div class="meta-item">
                <label>分类:</label>
                <span>{{ getCategoryLabel(selectedTemplate.metadata.category) }}</span>
              </div>
              <div class="meta-item">
                <label>作者:</label>
                <span>{{ selectedTemplate.author.name }}</span>
              </div>
              <div class="meta-item">
                <label>版本:</label>
                <span>{{ selectedTemplate.metadata.version }}</span>
              </div>
              <div class="meta-item">
                <label>更新时间:</label>
                <span>{{ formatDate(selectedTemplate.metadata.updated_at) }}</span>
              </div>
            </div>
            
            <div class="detail-stats">
              <div class="stat-item">
                <span class="stat-value">{{ selectedTemplate.rating.average.toFixed(1) }}</span>
                <span class="stat-label">评分</span>
              </div>
              <div class="stat-item">
                <span class="stat-value">{{ selectedTemplate.downloads }}</span>
                <span class="stat-label">下载</span>
              </div>
              <div class="stat-item">
                <span class="stat-value">{{ selectedTemplate.comments.length }}</span>
                <span class="stat-label">评论</span>
              </div>
            </div>
            
            <div class="detail-tags">
              <span v-for="tag in selectedTemplate.tags" :key="tag" class="tag">{{ tag }}</span>
            </div>
            
            <div v-if="selectedTemplate.variables.length > 0" class="detail-variables">
              <h4>变量</h4>
              <div v-for="variable in selectedTemplate.variables" :key="variable.name" class="variable-item">
                <span class="variable-name">{{ variable.name }}</span>
                <span class="variable-desc">{{ variable.description }}</span>
                <span v-if="variable.required" class="required-badge">必填</span>
              </div>
            </div>
            
            <div class="detail-comments">
              <div class="comments-header">
                <h4>评论 ({{ selectedTemplate.comments.length }})</h4>
                <button class="btn-small" @click="showCommentDialog = true">添加评论</button>
              </div>
              
              <div v-if="selectedTemplate.comments.length > 0" class="comments-list">
                <div v-for="comment in selectedTemplate.comments" :key="comment.id" class="comment-item">
                  <div class="comment-header">
                    <span class="comment-user">{{ comment.user }}</span>
                    <span class="comment-date">{{ formatDate(comment.created_at) }}</span>
                    <span class="comment-rating">{{ getStarRating(comment.rating) }}</span>
                  </div>
                  <p class="comment-content">{{ comment.content }}</p>
                </div>
              </div>
              <div v-else class="no-comments">
                <p>暂无评论</p>
              </div>
            </div>
          </div>
        </div>
        
        <div class="dialog-footer">
          <button class="btn-secondary" @click="showRatingDialog = true">
            评分 {{ selectedTemplate.rating.userRating ? `(${selectedTemplate.rating.userRating})` : '' }}
          </button>
          <button class="btn-primary" @click="applyTemplate(selectedTemplate)">使用模板</button>
          <button class="btn-secondary" @click="downloadTemplate(selectedTemplate)">下载</button>
          <button class="btn-secondary" @click="shareTemplate(selectedTemplate)">分享</button>
        </div>
      </div>
    </div>

    <!-- 评分对话框 -->
    <div v-if="showRatingDialog && selectedTemplate" class="dialog-overlay" @click.self="showRatingDialog = false">
      <div class="dialog small">
        <div class="dialog-header">
          <h2>评分模板</h2>
          <button class="btn-close" @click="showRatingDialog = false">✕</button>
        </div>
        
        <div class="dialog-content">
          <p>为 "{{ selectedTemplate.name }}" 评分:</p>
          <div class="rating-input">
            <button
              v-for="star in 5"
              :key="star"
              class="star-btn"
              :class="{ active: userRating >= star }"
              @click="userRating = star"
            >
              ⭐
            </button>
          </div>
        </div>
        
        <div class="dialog-footer">
          <button class="btn-secondary" @click="showRatingDialog = false">取消</button>
          <button class="btn-primary" @click="rateTemplate(selectedTemplate, userRating)">提交评分</button>
        </div>
      </div>
    </div>

    <!-- 评论对话框 -->
    <div v-if="showCommentDialog && selectedTemplate" class="dialog-overlay" @click.self="showCommentDialog = false">
      <div class="dialog small">
        <div class="dialog-header">
          <h2>添加评论</h2>
          <button class="btn-close" @click="showCommentDialog = false">✕</button>
        </div>
        
        <div class="dialog-content">
          <div class="comment-form">
            <textarea
              v-model="userComment"
              placeholder="写下你的评论..."
              class="comment-textarea"
              rows="4"
            ></textarea>
            
            <div class="rating-input">
              <label>评分:</label>
              <button
                v-for="star in 5"
                :key="star"
                class="star-btn"
                :class="{ active: userRating >= star }"
                @click="userRating = star"
              >
                ⭐
              </button>
            </div>
          </div>
        </div>
        
        <div class="dialog-footer">
          <button class="btn-secondary" @click="showCommentDialog = false">取消</button>
          <button class="btn-primary" @click="addComment(selectedTemplate)">提交评论</button>
        </div>
      </div>
    </div>

    <!-- 分享对话框 -->
    <div v-if="showShareDialog && selectedTemplate" class="dialog-overlay" @click.self="showShareDialog = false">
      <div class="dialog small">
        <div class="dialog-header">
          <h2>分享模板</h2>
          <button class="btn-close" @click="showShareDialog = false">✕</button>
        </div>
        
        <div class="dialog-content">
          <p>分享 "{{ selectedTemplate.name }}" :</p>
          <div class="share-options">
            <button class="share-btn">🔗 复制链接</button>
            <button class="share-btn">📧 邮件分享</button>
            <button class="share-btn">💬 社交媒体</button>
          </div>
        </div>
        
        <div class="dialog-footer">
          <button class="btn-secondary" @click="showShareDialog = false">关闭</button>
        </div>
      </div>
    </div>

    <!-- 应用模板对话框 -->
    <div v-if="showApplyDialog && templateToApply" class="dialog-overlay" @click.self="showApplyDialog = false">
      <div class="dialog">
        <div class="dialog-header">
          <h2>应用模板</h2>
          <button class="btn-close" @click="showApplyDialog = false">✕</button>
        </div>
        
        <div class="dialog-content">
          <p>为 "{{ templateToApply.name }}" 设置变量值:</p>
          <div class="variables-form">
            <div v-for="variable in templateToApply.variables" :key="variable.name" class="variable-input">
              <label>
                {{ variable.name }}
                <span v-if="variable.required" class="required">*</span>
              </label>
              <input
                v-model="variableValues[variable.name]"
                type="text"
                :placeholder="variable.default_value || ''"
                class="input-field"
              />
              <span v-if="variable.description" class="variable-hint">{{ variable.description }}</span>
            </div>
          </div>
        </div>
        
        <div class="dialog-footer">
          <button class="btn-secondary" @click="showApplyDialog = false">取消</button>
          <button class="btn-primary" @click="confirmApplyTemplate">应用</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.enhanced-template-library {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-primary, #ffffff);
}

.library-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px;
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.library-header h2 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary, #333333);
}

.header-actions {
  display: flex;
  gap: 12px;
  align-items: center;
}

.search-box {
  position: relative;
}

.search-input {
  padding: 8px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 6px;
  font-size: 14px;
  width: 250px;
}

.btn-primary,
.btn-secondary,
.btn-small,
.btn-use,
.btn-download,
.btn-share {
  padding: 8px 16px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: var(--primary-color, #007bff);
  color: white;
  border-color: var(--primary-color, #007bff);
}

.btn-primary:hover {
  background: var(--primary-dark, #0056b3);
}

.btn-secondary {
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #333333);
}

.btn-secondary:hover {
  background: var(--bg-secondary, #f5f5f5);
}

.btn-small {
  padding: 4px 8px;
  font-size: 12px;
}

.library-filters {
  display: flex;
  gap: 20px;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color, #e0e0e0);
  align-items: center;
  flex-wrap: wrap;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.filter-group label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary, #666666);
}

.filter-select {
  padding: 6px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 4px;
  font-size: 14px;
  background: var(--bg-primary, #ffffff);
}

.tags-filter {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.tag-btn {
  padding: 4px 10px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 12px;
  background: var(--bg-primary, #ffffff);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.tag-btn:hover {
  border-color: var(--primary-color, #007bff);
}

.tag-btn.active {
  background: var(--primary-color, #007bff);
  color: white;
  border-color: var(--primary-color, #007bff);
}

.view-toggle {
  display: flex;
  gap: 4px;
  margin-left: auto;
}

.view-btn {
  padding: 6px 10px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 4px;
  background: var(--bg-primary, #ffffff);
  cursor: pointer;
}

.view-btn.active {
  background: var(--primary-color, #007bff);
  color: white;
  border-color: var(--primary-color, #007bff);
}

.featured-section {
  padding: 20px;
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.featured-section h3 {
  margin: 0 0 16px 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary, #333333);
}

.featured-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.featured-card {
  border: 2px solid var(--accent-color, #2196f3);
  border-radius: 8px;
  overflow: hidden;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}

.featured-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.featured-thumbnail {
  position: relative;
  height: 160px;
  background: var(--bg-secondary, #f5f5f5);
  display: flex;
  align-items: center;
  justify-content: center;
}

.featured-thumbnail img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.placeholder-thumbnail {
  font-size: 48px;
  color: var(--text-secondary, #999999);
}

.featured-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  background: var(--accent-color, #2196f3);
  color: white;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.featured-info {
  padding: 12px;
}

.featured-info h4 {
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #333333);
}

.featured-info p {
  margin: 0 0 12px 0;
  font-size: 13px;
  color: var(--text-secondary, #666666);
  line-height: 1.4;
}

.featured-meta {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.templates-container {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.templates-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.template-card {
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 8px;
  overflow: hidden;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
  display: flex;
  flex-direction: column;
}

.template-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.card-thumbnail {
  position: relative;
  height: 140px;
  background: var(--bg-secondary, #f5f5f5);
  display: flex;
  align-items: center;
  justify-content: center;
}

.card-thumbnail img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.official-badge {
  position: absolute;
  top: 8px;
  left: 8px;
  background: var(--success-color, #4caf50);
  color: white;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.card-content {
  padding: 12px;
  flex: 1;
  display: flex;
  flex-direction: column;
}

.card-content h4 {
  margin: 0 0 8px 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary, #333333);
}

.description {
  margin: 0 0 12px 0;
  font-size: 13px;
  color: var(--text-secondary, #666666);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.card-meta {
  display: flex;
  gap: 8px;
  font-size: 12px;
  color: var(--text-secondary, #666666);
  margin-bottom: 8px;
}

.card-stats {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: var(--text-secondary, #666666);
  margin-bottom: 8px;
}

.card-tags {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
  margin-bottom: 12px;
}

.tag {
  padding: 2px 8px;
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 12px;
  font-size: 11px;
  color: var(--text-secondary, #666666);
}

.card-actions {
  display: flex;
  gap: 8px;
  padding: 12px;
  border-top: 1px solid var(--border-color, #e0e0e0);
}

.btn-use {
  flex: 1;
  background: var(--primary-color, #007bff);
  color: white;
  border-color: var(--primary-color, #007bff);
}

.btn-download,
.btn-share {
  padding: 8px 12px;
}

.templates-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.list-item {
  display: flex;
  gap: 16px;
  padding: 16px;
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s;
}

.list-item:hover {
  background: var(--bg-secondary, #f5f5f5);
}

.list-thumbnail {
  width: 80px;
  height: 80px;
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.list-thumbnail img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 6px;
}

.list-content {
  flex: 1;
}

.list-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.list-header h4 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #333333);
}

.list-badges {
  display: flex;
  gap: 4px;
}

.list-meta {
  display: flex;
  gap: 16px;
  font-size: 12px;
  color: var(--text-secondary, #666666);
  margin-bottom: 8px;
}

.list-stats {
  display: flex;
  gap: 16px;
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.list-actions {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.empty-state {
  text-align: center;
  padding: 40px;
  color: var(--text-secondary, #666666);
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: var(--bg-primary, #ffffff);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
}

.dialog.small {
  max-width: 400px;
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px;
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.dialog-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary, #333333);
}

.btn-close {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--text-secondary, #666666);
  padding: 4px 8px;
}

.dialog-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 20px;
  border-top: 1px solid var(--border-color, #e0e0e0);
}

.detail-thumbnail {
  width: 100%;
  height: 200px;
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 20px;
}

.detail-thumbnail img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 8px;
}

.detail-meta {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  margin-bottom: 20px;
}

.meta-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.meta-item label {
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.detail-stats {
  display: flex;
  gap: 24px;
  margin-bottom: 20px;
}

.stat-item {
  text-align: center;
}

.stat-value {
  font-size: 24px;
  font-weight: 600;
  color: var(--primary-color, #007bff);
}

.stat-label {
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.detail-tags {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  margin-bottom: 20px;
}

.detail-variables {
  margin-bottom: 20px;
}

.detail-variables h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
}

.variable-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 4px;
  margin-bottom: 8px;
}

.variable-name {
  font-weight: 500;
  color: var(--text-primary, #333333);
}

.variable-desc {
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.required-badge {
  background: var(--error-color, #f44336);
  color: white;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 10px;
}

.detail-comments {
  margin-top: 20px;
}

.comments-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.comments-header h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
}

.comment-item {
  padding: 12px;
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 6px;
  margin-bottom: 8px;
}

.comment-header {
  display: flex;
  gap: 12px;
  margin-bottom: 8px;
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.comment-user {
  font-weight: 500;
  color: var(--text-primary, #333333);
}

.comment-content {
  margin: 0;
  font-size: 14px;
  color: var(--text-primary, #333333);
  line-height: 1.4;
}

.no-comments {
  text-align: center;
  padding: 20px;
  color: var(--text-secondary, #666666);
}

.rating-input {
  display: flex;
  gap: 8px;
  justify-content: center;
  margin: 20px 0;
}

.star-btn {
  font-size: 32px;
  background: none;
  border: none;
  cursor: pointer;
  opacity: 0.3;
  transition: opacity 0.2s;
}

.star-btn:hover,
.star-btn.active {
  opacity: 1;
}

.comment-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.comment-textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 6px;
  font-size: 14px;
  resize: vertical;
}

.variables-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.variable-input {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.variable-input label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary, #333333);
}

.required {
  color: var(--error-color, #f44336);
}

.input-field {
  padding: 8px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 4px;
  font-size: 14px;
}

.variable-hint {
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.share-options {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.share-btn {
  padding: 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 6px;
  background: var(--bg-primary, #ffffff);
  cursor: pointer;
  transition: background 0.2s;
}

.share-btn:hover {
  background: var(--bg-secondary, #f5f5f5);
}
</style>
