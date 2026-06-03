<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { Bold, Italic, Underline, X, Link, List, ListOrdered, Highlighter, Palette } from 'lucide-vue-next';

interface Props {
  show: boolean;
  x: number;
  y: number;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'action', action: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const toolbarRef = ref<HTMLElement | null>(null);

// Toolbar position - adjusted to fit in viewport
const toolbarPosition = computed(() => {
  if (!toolbarRef.value) {
    return { left: props.x + 'px', top: props.y + 'px' };
  }

  const toolbarWidth = toolbarRef.value.offsetWidth;
  const toolbarHeight = toolbarRef.value.offsetHeight;
  const viewportWidth = window.innerWidth;
  const viewportHeight = window.innerHeight;

  let left = props.x;
  let top = props.y;

  // Adjust horizontal position
  if (left + toolbarWidth > viewportWidth) {
    left = viewportWidth - toolbarWidth - 10;
  }

  // Adjust vertical position (show above selection by default)
  if (top - toolbarHeight < 10) {
    top = top + 20; // Show below if not enough space above
  } else {
    top = top - toolbarHeight - 10; // Show above
  }

  return { left: left + 'px', top: top + 'px' };
});

// Handle action
const handleAction = (action: string) => {
  emit('action', action);
};

// Close on click outside
const handleClickOutside = (event: MouseEvent) => {
  if (toolbarRef.value && !toolbarRef.value.contains(event.target as Node)) {
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
</script>

<template>
  <Teleport to="body">
    <Transition name="mini-toolbar">
      <div
        v-if="show"
        ref="toolbarRef"
        class="mini-toolbar"
        :style="toolbarPosition"
        role="toolbar"
        aria-label="迷你工具栏"
      >
        <!-- Close button -->
        <button
          class="mini-toolbar-btn close-btn"
          title="关闭"
          @click="emit('update:show', false)"
        >
          <X :size="14" />
        </button>

        <!-- Font formatting -->
        <div class="mini-toolbar-divider"></div>
        <button
          class="mini-toolbar-btn"
          title="加粗 (Ctrl+B)"
          @click="handleAction('bold')"
        >
          <Bold :size="16" />
        </button>
        <button
          class="mini-toolbar-btn"
          title="斜体 (Ctrl+I)"
          @click="handleAction('italic')"
        >
          <Italic :size="16" />
        </button>
        <button
          class="mini-toolbar-btn"
          title="下划线 (Ctrl+U)"
          @click="handleAction('underline')"
        >
          <Underline :size="16" />
        </button>

        <!-- Color -->
        <div class="mini-toolbar-divider"></div>
        <button
          class="mini-toolbar-btn"
          title="字体颜色"
          @click="handleAction('text-color')"
        >
          <Palette :size="16" />
        </button>
        <button
          class="mini-toolbar-btn"
          title="高亮"
          @click="handleAction('highlight')"
        >
          <Highlighter :size="16" />
        </button>

        <!-- Lists -->
        <div class="mini-toolbar-divider"></div>
        <button
          class="mini-toolbar-btn"
          title="项目符号"
          @click="handleAction('bullet-list')"
        >
          <List :size="16" />
        </button>
        <button
          class="mini-toolbar-btn"
          title="编号"
          @click="handleAction('numbered-list')"
        >
          <ListOrdered :size="16" />
        </button>

        <!-- Link -->
        <div class="mini-toolbar-divider"></div>
        <button
          class="mini-toolbar-btn"
          title="插入链接"
          @click="handleAction('link')"
        >
          <Link :size="16" />
        </button>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.mini-toolbar {
  position: fixed;
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  background: var(--word-bg-page, #ffffff);
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 6px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
  z-index: 10000;
  user-select: none;
}

.mini-toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  border-radius: 4px;
  color: var(--word-text-primary, #333);
  cursor: pointer;
  transition: all 0.15s ease;
}

.mini-toolbar-btn:hover {
  background: var(--word-button-hover, #f5f5f5);
  color: var(--word-accent, #007bff);
}

.mini-toolbar-btn:active {
  background: var(--word-button-active, #e8e8e8);
  transform: scale(0.95);
}

.mini-toolbar-btn.close-btn {
  color: var(--word-text-secondary, #666);
}

.mini-toolbar-btn.close-btn:hover {
  color: #dc2626;
  background: #fef2f2;
}

.mini-toolbar-divider {
  width: 1px;
  height: 20px;
  background: var(--word-divider, #e0e0e0);
  margin: 0 4px;
}

/* Mini toolbar transition */
.mini-toolbar-enter-active,
.mini-toolbar-leave-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}

.mini-toolbar-enter-from,
.mini-toolbar-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}

/* Dark mode */
:global(.dark) .mini-toolbar {
  background: var(--word-bg-canvas, #1e1e1e);
  border-color: var(--word-border, #3e3e42);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
}

:global(.dark) .mini-toolbar-btn {
  color: var(--word-text-primary, #ffffff);
}

:global(.dark) .mini-toolbar-btn:hover {
  background: var(--word-button-hover, #2d2d30);
  color: var(--word-accent, #0078d4);
}

:global(.dark) .mini-toolbar-btn.close-btn {
  color: var(--word-text-secondary, #cccccc);
}

:global(.dark) .mini-toolbar-btn.close-btn:hover {
  color: #f87171;
  background: #450a0a;
}

:global(.dark) .mini-toolbar-divider {
  background: var(--word-divider, #3e3e42);
}
</style>
