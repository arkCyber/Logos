<script setup lang="ts">
import { ref, computed } from 'vue';

interface Revision {
  id: string;
  type: 'insert' | 'delete' | 'format';
  author: string;
  timestamp: number;
  content: string;
  accepted: boolean;
  rejected: boolean;
}

interface Props {
  show: boolean;
  revisions: Revision[];
  trackChanges: boolean;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'toggle-track-changes', enabled: boolean): void;
  (e: 'accept-revision', revisionId: string): void;
  (e: 'reject-revision', revisionId: string): void;
  (e: 'accept-all'): void;
  (e: 'reject-all'): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// Filter
const filterType = ref('all');
const showAccepted = ref(false);
const showRejected = ref(false);

// Computed
const filteredRevisions = computed(() => {
  let filtered = props.revisions;

  if (filterType.value !== 'all') {
    filtered = filtered.filter(r => r.type === filterType.value);
  }

  if (!showAccepted.value) {
    filtered = filtered.filter(r => !r.accepted);
  }

  if (!showRejected.value) {
    filtered = filtered.filter(r => !r.rejected);
  }

  return filtered.sort((a, b) => b.timestamp - a.timestamp);
});

// Format timestamp
const formatTimestamp = (timestamp: number) => {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const minutes = Math.floor(diff / 60000);
  const hours = Math.floor(diff / 3600000);
  const days = Math.floor(diff / 86400000);

  if (minutes < 1) {
return '刚刚';
}
  if (minutes < 60) {
return `${minutes}分钟前`;
}
  if (hours < 24) {
return `${hours}小时前`;
}
  if (days < 7) {
return `${days}天前`;
}
  return date.toLocaleDateString();
};

// Get revision type label
const getRevisionTypeLabel = (type: string) => {
  switch (type) {
    case 'insert':
      return '插入';
    case 'delete':
      return '删除';
    case 'format':
      return '格式';
    default:
      return type;
  }
};

// Get revision type color
const getRevisionTypeColor = (type: string) => {
  switch (type) {
    case 'insert':
      return '#10b981';
    case 'delete':
      return '#ef4444';
    case 'format':
      return '#3b82f6';
    default:
      return '#6b7280';
  }
};

// Toggle track changes
const toggleTrackChanges = () => {
  emit('toggle-track-changes', !props.trackChanges);
};

// Accept revision
const acceptRevision = (revisionId: string) => {
  emit('accept-revision', revisionId);
};

// Reject revision
const rejectRevision = (revisionId: string) => {
  emit('reject-revision', revisionId);
};

// Accept all
const acceptAll = () => {
  if (confirm('确定要接受所有修订吗？')) {
    emit('accept-all');
  }
};

// Reject all
const rejectAll = () => {
  if (confirm('确定要拒绝所有修订吗？')) {
    emit('reject-all');
  }
};

// Close panel
const closePanel = () => {
  emit('update:show', false);
};
</script>

<template>
  <Transition name="slide">
    <div v-if="show" class="revision-panel">
      <!-- Header -->
      <div class="panel-header">
        <h3 class="panel-title">修订模式</h3>
        <div class="panel-actions">
          <label class="toggle-label">
            <input
              :checked="trackChanges"
              type="checkbox"
              @change="toggleTrackChanges"
            />
            <span>跟踪更改</span>
          </label>
          <button class="close-btn" type="button" @click="closePanel">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>
        </div>
      </div>

      <!-- Filters -->
      <div class="filters">
        <div class="filter-group">
          <label>类型:</label>
          <select v-model="filterType" class="filter-select">
            <option value="all">全部</option>
            <option value="insert">插入</option>
            <option value="delete">删除</option>
            <option value="format">格式</option>
          </select>
        </div>
        <div class="filter-group">
          <label class="checkbox-label">
            <input v-model="showAccepted" type="checkbox" />
            <span>显示已接受</span>
          </label>
        </div>
        <div class="filter-group">
          <label class="checkbox-label">
            <input v-model="showRejected" type="checkbox" />
            <span>显示已拒绝</span>
          </label>
        </div>
      </div>

      <!-- Bulk actions -->
      <div class="bulk-actions">
        <button class="btn success" type="button" @click="acceptAll">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <polyline points="20 6 9 17 4 12" />
          </svg>
          接受全部
        </button>
        <button class="btn danger" type="button" @click="rejectAll">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
          拒绝全部
        </button>
      </div>

      <!-- Revisions list -->
      <div class="revisions-list">
        <div
          v-for="revision in filteredRevisions"
          :key="revision.id"
          class="revision-item"
          :class="{
            accepted: revision.accepted,
            rejected: revision.rejected
          }"
        >
          <!-- Revision header -->
          <div class="revision-header">
            <div class="revision-type" :style="{ color: getRevisionTypeColor(revision.type) }">
              <span class="type-badge">{{ getRevisionTypeLabel(revision.type) }}</span>
            </div>
            <div class="revision-author">{{ revision.author }}</div>
            <div class="revision-time">{{ formatTimestamp(revision.timestamp) }}</div>
          </div>

          <!-- Revision content -->
          <div class="revision-content">
            <div class="content-label">内容:</div>
            <div class="content-text">{{ revision.content }}</div>
          </div>

          <!-- Revision actions -->
          <div class="revision-actions">
            <button
              v-if="!revision.accepted && !revision.rejected"
              class="action-btn accept"
              title="接受"
              type="button"
              @click="acceptRevision(revision.id)"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              >
                <polyline points="20 6 9 17 4 12" />
              </svg>
              接受
            </button>
            <button
              v-if="!revision.accepted && !revision.rejected"
              class="action-btn reject"
              title="拒绝"
              type="button"
              @click="rejectRevision(revision.id)"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              >
                <line x1="18" y1="6" x2="6" y2="18" />
                <line x1="6" y1="6" x2="18" y2="18" />
              </svg>
              拒绝
            </button>
            <div v-if="revision.accepted" class="status-badge accepted">
              已接受
            </div>
            <div v-if="revision.rejected" class="status-badge rejected">
              已拒绝
            </div>
          </div>
        </div>

        <!-- Empty state -->
        <div v-if="filteredRevisions.length === 0" class="empty-state">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="48"
            height="48"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.5"
          >
            <path d="M12 20h9" />
            <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" />
          </svg>
          <p>暂无修订</p>
        </div>
      </div>

      <!-- Stats -->
      <div class="stats">
        <div class="stat-item">
          <span class="stat-label">总修订:</span>
          <span class="stat-value">{{ revisions.length }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">待处理:</span>
          <span class="stat-value">{{ revisions.filter(r => !r.accepted && !r.rejected).length }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">已接受:</span>
          <span class="stat-value">{{ revisions.filter(r => r.accepted).length }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">已拒绝:</span>
          <span class="stat-value">{{ revisions.filter(r => r.rejected).length }}</span>
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.revision-panel {
  position: fixed;
  right: 0;
  top: 0;
  bottom: 0;
  width: 400px;
  background: var(--word-bg-page);
  border-left: 1px solid var(--word-border);
  display: flex;
  flex-direction: column;
  z-index: 1000;
  box-shadow: -4px 0 12px rgba(0, 0, 0, 0.1);
}

/* Header */
.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border-bottom: 1px solid var(--word-border);
}

.panel-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--word-text-primary);
}

.panel-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.toggle-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--word-text-secondary);
  cursor: pointer;
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  color: var(--word-text-secondary);
  transition: all 0.15s ease;
}

.close-btn:hover {
  background: var(--word-button-hover);
  color: var(--word-text-primary);
}

/* Filters */
.filters {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px 16px;
  border-bottom: 1px solid var(--word-border);
  flex-wrap: wrap;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.filter-group label {
  font-size: 13px;
  color: var(--word-text-secondary);
}

.filter-select {
  padding: 6px 10px;
  border: 1px solid var(--word-button-border);
  border-radius: 4px;
  background: var(--word-button-bg);
  color: var(--word-text-primary);
  font-size: 13px;
}

.filter-select:focus {
  outline: none;
  border-color: var(--word-button-border-hover);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--word-text-secondary);
  cursor: pointer;
}

/* Bulk actions */
.bulk-actions {
  display: flex;
  gap: 8px;
  padding: 12px 16px;
  border-bottom: 1px solid var(--word-border);
}

.btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  border: none;
}

.btn.success {
  background: #10b981;
  color: white;
}

.btn.success:hover {
  background: #059669;
}

.btn.danger {
  background: #ef4444;
  color: white;
}

.btn.danger:hover {
  background: #dc2626;
}

/* Revisions list */
.revisions-list {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.revision-item {
  padding: 12px;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 6px;
  transition: all 0.15s ease;
}

.revision-item.accepted {
  opacity: 0.6;
  border-color: #10b981;
}

.revision-item.rejected {
  opacity: 0.6;
  border-color: #ef4444;
}

.revision-item:hover {
  border-color: var(--word-button-border-hover);
}

.revision-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
}

.revision-type {
  display: flex;
  align-items: center;
}

.type-badge {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
  background: currentColor;
  color: white;
}

.revision-author {
  font-size: 13px;
  font-weight: 600;
  color: var(--word-text-primary);
}

.revision-time {
  font-size: 12px;
  color: var(--word-text-secondary);
  margin-left: auto;
}

.revision-content {
  margin-bottom: 8px;
}

.content-label {
  font-size: 12px;
  color: var(--word-text-secondary);
  margin-bottom: 4px;
}

.content-text {
  font-size: 13px;
  color: var(--word-text-primary);
  line-height: 1.4;
  background: var(--word-bg-page);
  padding: 8px;
  border-radius: 4px;
  max-height: 60px;
  overflow-y: auto;
}

.revision-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 12px;
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.15s ease;
  border: none;
}

.action-btn.accept {
  background: #10b981;
  color: white;
}

.action-btn.accept:hover {
  background: #059669;
}

.action-btn.reject {
  background: #ef4444;
  color: white;
}

.action-btn.reject:hover {
  background: #dc2626;
}

.status-badge {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.status-badge.accepted {
  background: #10b981;
  color: white;
}

.status-badge.rejected {
  background: #ef4444;
  color: white;
}

/* Empty state */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 40px 20px;
  color: var(--word-text-secondary);
}

.empty-state p {
  margin: 0;
  font-size: 14px;
}

/* Stats */
.stats {
  display: flex;
  padding: 12px 16px;
  border-top: 1px solid var(--word-border);
  gap: 16px;
  background: var(--word-button-bg);
}

.stat-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat-label {
  font-size: 11px;
  color: var(--word-text-secondary);
}

.stat-value {
  font-size: 16px;
  font-weight: 600;
  color: var(--word-text-primary);
}

/* Transition */
.slide-enter-active,
.slide-leave-active {
  transition: transform 0.3s ease;
}

.slide-enter-from,
.slide-leave-to {
  transform: translateX(100%);
}

/* Dark mode */
:global(.dark) .revision-panel {
  background: var(--word-bg-canvas);
  box-shadow: -4px 0 12px rgba(0, 0, 0, 0.4);
}
</style>
