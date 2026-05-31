<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';

interface Props {
  show: boolean;
  x: number;
  y: number;
  context?: 'text' | 'table' | 'image' | 'general';
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'action', action: string, payload?: any): void;
}

const props = withDefaults(defineProps<Props>(), {
  context: 'general'
});

const emit = defineEmits<Emits>();

const menuRef = ref<HTMLElement | null>(null);

// Menu items based on context
const menuItems = computed(() => {
  const baseItems = [
    { id: 'cut', label: '剪切', icon: 'cut', shortcut: 'Ctrl+X' },
    { id: 'copy', label: '复制', icon: 'copy', shortcut: 'Ctrl+C' },
    { id: 'paste', label: '粘贴', icon: 'paste', shortcut: 'Ctrl+V' },
    { type: 'separator' },
    { id: 'select-all', label: '全选', icon: 'select-all', shortcut: 'Ctrl+A' }
  ];

  const textItems = [
    { id: 'cut', label: '剪切', icon: 'cut', shortcut: 'Ctrl+X' },
    { id: 'copy', label: '复制', icon: 'copy', shortcut: 'Ctrl+C' },
    { id: 'paste', label: '粘贴', icon: 'paste', shortcut: 'Ctrl+V' },
    { type: 'separator' },
    { id: 'font', label: '字体...', icon: 'font' },
    { id: 'paragraph', label: '段落...', icon: 'paragraph' },
    { id: 'bullet-list', label: '项目符号', icon: 'list' },
    { id: 'numbered-list', label: '编号', icon: 'list-ordered' },
    { type: 'separator' },
    { id: 'link', label: '链接...', icon: 'link' },
    { id: 'comment', label: '新建批注', icon: 'comment' },
    { type: 'separator' },
    { id: 'select-all', label: '全选', icon: 'select-all', shortcut: 'Ctrl+A' }
  ];

  const tableItems = [
    { id: 'cut', label: '剪切', icon: 'cut', shortcut: 'Ctrl+X' },
    { id: 'copy', label: '复制', icon: 'copy', shortcut: 'Ctrl+C' },
    { id: 'paste', label: '粘贴', icon: 'paste', shortcut: 'Ctrl+V' },
    { type: 'separator' },
    { id: 'insert-row-above', label: '在上方插入行', icon: 'row-add' },
    { id: 'insert-row-below', label: '在下方插入行', icon: 'row-add' },
    { id: 'insert-column-left', label: '在左侧插入列', icon: 'column-add' },
    { id: 'insert-column-right', label: '在右侧插入列', icon: 'column-add' },
    { id: 'delete-row', label: '删除行', icon: 'row-delete' },
    { id: 'delete-column', label: '删除列', icon: 'column-delete' },
    { type: 'separator' },
    { id: 'merge-cells', label: '合并单元格', icon: 'merge' },
    { id: 'split-cells', label: '拆分单元格', icon: 'split' },
    { type: 'separator' },
    { id: 'table-properties', label: '表格属性...', icon: 'table' }
  ];

  const imageItems = [
    { id: 'cut', label: '剪切', icon: 'cut', shortcut: 'Ctrl+X' },
    { id: 'copy', label: '复制', icon: 'copy', shortcut: 'Ctrl+C' },
    { type: 'separator' },
    { id: 'wrap-text', label: '文字环绕', icon: 'wrap' },
    { id: 'bring-front', label: '置于顶层', icon: 'bring-front' },
    { id: 'send-back', label: '置于底层', icon: 'send-back' },
    { type: 'separator' },
    { id: 'format-picture', label: '设置图片格式...', icon: 'image' },
    { id: 'change-picture', label: '更改图片...', icon: 'image-change' },
    { type: 'separator' },
    { id: 'reset-picture', label: '重置图片', icon: 'reset' }
  ];

  switch (props.context) {
    case 'text':
      return textItems;
    case 'table':
      return tableItems;
    case 'image':
      return imageItems;
    default:
      return baseItems;
  }
});

// Position menu to fit in viewport
const menuPosition = computed(() => {
  if (!menuRef.value) {
return { left: props.x + 'px', top: props.y + 'px' };
}

  const menuWidth = menuRef.value.offsetWidth;
  const menuHeight = menuRef.value.offsetHeight;
  const viewportWidth = window.innerWidth;
  const viewportHeight = window.innerHeight;

  let left = props.x;
  let top = props.y;

  // Adjust horizontal position
  if (left + menuWidth > viewportWidth) {
    left = viewportWidth - menuWidth - 10;
  }

  // Adjust vertical position
  if (top + menuHeight > viewportHeight) {
    top = viewportHeight - menuHeight - 10;
  }

  return { left: left + 'px', top: top + 'px' };
});

// Handle menu item click
const handleItemClick = (item: any) => {
  if (item.type === 'separator') {
return;
}
  emit('action', item.id);
  emit('update:show', false);
};

// Close menu on click outside
const handleClickOutside = (event: MouseEvent) => {
  if (menuRef.value && !menuRef.value.contains(event.target as Node)) {
    emit('update:show', false);
  }
};

// Handle escape key
const handleEscape = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && props.show) {
    emit('update:show', false);
  }
};

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
  document.addEventListener('keydown', handleEscape);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside);
  document.removeEventListener('keydown', handleEscape);
});

// Icon SVGs
const icons: Record<string, string> = {
  cut: '<path d="M6 4v16"/><path d="M6 12l12-8"/><path d="M6 12l12 8"/>',
  copy: '<rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>',
  paste: '<path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><rect x="8" y="2" width="8" height="4" rx="1"/>',
  'select-all': '<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M9 9h6"/><path d="M9 12h6"/><path d="M9 15h6"/>',
  font: '<path d="M4 7V4h16v3"/><path d="M9 20h6"/><path d="M12 4v16"/>',
  paragraph: '<path d="M21 6v14"/><path d="M3 6h18"/><path d="M3 12h18"/><path d="M3 18h18"/>',
  list: '<line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/>',
  'list-ordered': '<line x1="10" y1="6" x2="21" y2="6"/><line x1="10" y1="12" x2="21" y2="12"/><line x1="10" y1="18" x2="21" y2="18"/><path d="M4 6h1v4"/><path d="M4 10h2"/><path d="M6 18H4c0-1 2-2 2-3s-1-1.5-2-1"/>',
  link: '<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>',
  comment: '<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>',
  'row-add': '<line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>',
  'row-delete': '<line x1="5" y1="12" x2="19" y2="12"/>',
  'column-add': '<line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>',
  'column-delete': '<line x1="12" y1="5" x2="12" y2="19"/>',
  merge: '<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M9 9h6v6H9z"/>',
  split: '<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M9 3v18"/><path d="M15 3v18"/>',
  table: '<rect x="3" y="3" width="18" height="18" rx="2"/><line x1="3" y1="9" x2="21" y2="9"/><line x1="3" y1="15" x2="21" y2="15"/><line x1="9" y1="3" x2="9" y2="21"/><line x1="15" y1="3" x2="15" y2="21"/>',
  wrap: '<path d="M3 6h18"/><path d="M3 12h18"/><path d="M3 18h18"/>',
  'bring-front': '<rect x="8" y="8" width="12" height="12" rx="2"/><path d="M4 16V6a2 2 0 0 1 2-2h10"/>',
  'send-back': '<rect x="4" y="4" width="12" height="12" rx="2"/><path d="M20 8v10a2 2 0 0 1-2 2H8"/>',
  image: '<rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="8.5" cy="8.5" r="1.5"/><polyline points="21 15 16 10 5 21"/>',
  'image-change': '<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/>',
  reset: '<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/>'
};
</script>

<template>
  <Teleport to="body">
    <Transition name="context-menu">
      <div
        v-if="show"
        ref="menuRef"
        class="context-menu"
        :style="menuPosition"
        role="menu"
        aria-label="上下文菜单"
      >
        <div
          v-for="(item, index) in menuItems"
          :key="index"
          class="context-menu-item"
          :class="{ separator: item.type === 'separator' }"
          :role="item.type === 'separator' ? 'separator' : 'menuitem'"
          :tabindex="item.type === 'separator' ? -1 : 0"
          @click="handleItemClick(item)"
        >
          <template v-if="item.type !== 'separator'">
            <svg
              v-if="item.icon && icons[item.icon]"
              class="menu-icon"
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              v-html="icons[item.icon]"
            />
            <span class="menu-label">{{ item.label }}</span>
            <span v-if="item.shortcut" class="menu-shortcut">{{ item.shortcut }}</span>
          </template>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.context-menu {
  position: fixed;
  background: var(--word-bg-page);
  border: 1px solid var(--word-border);
  border-radius: 4px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  padding: 4px 0;
  min-width: 200px;
  max-width: 300px;
  z-index: 9999;
  overflow: hidden;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 16px;
  cursor: pointer;
  transition: background 0.1s ease;
  font-size: 13px;
  color: var(--word-text-primary);
  user-select: none;
}

.context-menu-item:hover:not(.separator) {
  background: var(--word-button-hover);
}

.context-menu-item:focus:not(.separator) {
  outline: 2px solid var(--word-button-border-hover);
  outline-offset: -2px;
}

.context-menu-item.separator {
  padding: 4px 0;
  pointer-events: none;
}

.context-menu-item.separator::after {
  content: '';
  display: block;
  height: 1px;
  background: var(--word-divider);
  margin: 0 16px;
}

.menu-icon {
  flex-shrink: 0;
  color: var(--word-text-secondary);
}

.menu-label {
  flex: 1;
}

.menu-shortcut {
  color: var(--word-text-secondary);
  font-size: 12px;
}

/* Context menu transition */
.context-menu-enter-active,
.context-menu-leave-active {
  transition: opacity 0.1s ease, transform 0.1s ease;
}

.context-menu-enter-from,
.context-menu-leave-to {
  opacity: 0;
  transform: scale(0.95);
}

/* Dark mode */
:global(.dark) .context-menu {
  background: var(--word-bg-canvas);
  border-color: var(--word-border);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
}

:global(.dark) .context-menu-item:hover:not(.separator) {
  background: var(--word-button-hover);
}
</style>
