<script setup lang="ts">
import { ref } from 'vue';
import { List, ChevronRight, ChevronDown, FileText } from 'lucide-vue-next';

interface Props {
  show: boolean;
  headings: Array<{ id: string; level: number; text: string; children?: any[] }>;
}

interface Emits {
  (e: 'close'): void;
  (e: 'navigate-to', id: string): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

const expandedItems = ref<Set<string>>(new Set());

const toggleExpand = (id: string) => {
  if (expandedItems.value.has(id)) {
    expandedItems.value.delete(id);
  } else {
    expandedItems.value.add(id);
  }
};

const navigateTo = (id: string) => {
  emit('navigate-to', id);
};

const getIndent = (level: number) => {
  return (level - 1) * 16;
};
</script>

<template>
  <div v-if="show" class="document-outline editor-side-panel editor-side-panel--left">
    <div class="outline-header">
      <div class="header-content">
        <FileText :size="18" />
        <h3>文档大纲</h3>
      </div>
      <button class="close-button" title="关闭" @click="emit('close')">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>
    
    <div class="outline-content">
      <div v-if="headings.length === 0" class="empty-outline">
        <List :size="32" />
        <p>文档中没有标题</p>
        <p class="hint">添加标题后此处将显示文档大纲</p>
      </div>
      
      <div v-else class="outline-tree">
        <div
          v-for="heading in headings"
          :key="heading.id"
          class="outline-item"
          :style="{ paddingLeft: getIndent(heading.level) + 'px' }"
        >
          <div class="outline-item-content" @click="navigateTo(heading.id)">
            <button
              v-if="heading.children && heading.children.length > 0"
              class="expand-button"
              @click.stop="toggleExpand(heading.id)"
            >
              <ChevronRight v-if="!expandedItems.has(heading.id)" :size="14" />
              <ChevronDown v-else :size="14" />
            </button>
            <span v-else class="expand-spacer"></span>
            <span class="heading-text">{{ heading.text }}</span>
          </div>
          
          <div v-if="heading.children && heading.children.length > 0 && expandedItems.has(heading.id)" class="outline-children">
            <div
              v-for="child in heading.children"
              :key="child.id"
              class="outline-item"
              :style="{ paddingLeft: getIndent(child.level) + 'px' }"
            >
              <div class="outline-item-content" @click="navigateTo(child.id)">
                <button
                  v-if="child.children && child.children.length > 0"
                  class="expand-button"
                  @click.stop="toggleExpand(child.id)"
                >
                  <ChevronRight v-if="!expandedItems.has(child.id)" :size="14" />
                  <ChevronDown v-else :size="14" />
                </button>
                <span v-else class="expand-spacer"></span>
                <span class="heading-text">{{ child.text }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.document-outline {
  width: var(--editor-sidebar-outline-width, 280px);
  min-width: var(--editor-sidebar-outline-width, 280px);
  max-width: var(--editor-sidebar-outline-width, 280px);
  height: 100%;
  background: #ffffff;
  border-right: 1px solid #d1d5db;
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow: hidden;
}

.dark .document-outline {
  background: #1e1e1e;
  border-color: #3e3e42;
}

.outline-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid #e5e7eb;
  background: #f9fafb;
}

.dark .outline-header {
  background: #2d2d30;
  border-bottom-color: #3e3e42;
}

.header-content {
  display: flex;
  align-items: center;
  gap: 8px;
}

.header-content h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #374151;
}

.dark .header-content h3 {
  color: #e5e7eb;
}

.close-button {
  background: none;
  border: none;
  padding: 4px;
  cursor: pointer;
  border-radius: 4px;
  color: #6b7280;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-button:hover {
  background: #e5e7eb;
  color: #374151;
}

.dark .close-button:hover {
  background: #3e3e42;
  color: #e5e7eb;
}

.outline-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.empty-outline {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  color: #9ca3af;
  text-align: center;
}

.empty-outline svg {
  margin-bottom: 12px;
  opacity: 0.5;
}

.empty-outline p {
  margin: 4px 0;
  font-size: 13px;
}

.empty-outline .hint {
  font-size: 12px;
  opacity: 0.7;
}

.outline-tree {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.outline-item {
  display: flex;
  flex-direction: column;
}

.outline-item-content {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.15s;
}

.outline-item-content:hover {
  background: #f3f4f6;
}

.dark .outline-item-content:hover {
  background: #3e3e42;
}

.expand-button {
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
  color: #6b7280;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.expand-button:hover {
  color: #374151;
}

.dark .expand-button:hover {
  color: #e5e7eb;
}

.expand-spacer {
  width: 14px;
  flex-shrink: 0;
}

.heading-text {
  font-size: 13px;
  color: #374151;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
}

.dark .heading-text {
  color: #e5e7eb;
}

.outline-children {
  display: flex;
  flex-direction: column;
  gap: 2px;
}
</style>
