<script setup lang="ts">
import {
  Heading1, Heading2, Heading3, List, ListOrdered,
  Quote, Image, Code, Table, Plus
} from 'lucide-vue-next';

const props = defineProps<{
  editor: any;
  show: boolean;
  x: number;
  y: number;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

const setHeading = (level: 1 | 2 | 3) => {
  props.editor?.chain().focus().toggleHeading({ level }).run();
  emit('close');
};

const toggleBulletList = () => {
  props.editor?.chain().focus().toggleBulletList().run();
  emit('close');
};

const toggleOrderedList = () => {
  props.editor?.chain().focus().toggleOrderedList().run();
  emit('close');
};

const toggleBlockquote = () => {
  props.editor?.chain().focus().toggleBlockquote().run();
  emit('close');
};

const insertImage = () => {
  const url = window.prompt('输入图片 URL:');
  if (url) {
    props.editor?.chain().focus().setImage({ src: url }).run();
  }
  emit('close');
};

const toggleCodeBlock = () => {
  props.editor?.chain().focus().toggleCodeBlock().run();
  emit('close');
};

const insertTable = () => {
  props.editor?.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run();
  emit('close');
};

const insertHorizontalRule = () => {
  props.editor?.chain().focus().setHorizontalRule().run();
  emit('close');
};
</script>

<template>
  <Teleport to="body">
    <Transition name="floating">
      <div
        v-if="show && editor"
        class="floating-menu"
        :style="{ left: x + 'px', top: y + 'px' }"
      >
        <div class="floating-menu-content">
          <button
            class="floating-button"
            :class="{ active: editor.isActive('heading', { level: 1 }) }"
            title="标题1"
            @click="setHeading(1)"
          >
            <Heading1 :size="16" />
          </button>
          <button
            class="floating-button"
            :class="{ active: editor.isActive('heading', { level: 2 }) }"
            title="标题2"
            @click="setHeading(2)"
          >
            <Heading2 :size="16" />
          </button>
          <button
            class="floating-button"
            :class="{ active: editor.isActive('heading', { level: 3 }) }"
            title="标题3"
            @click="setHeading(3)"
          >
            <Heading3 :size="16" />
          </button>
          <div class="floating-divider"></div>
          <button
            class="floating-button"
            :class="{ active: editor.isActive('bulletList') }"
            title="无序列表"
            @click="toggleBulletList"
          >
            <List :size="16" />
          </button>
          <button
            class="floating-button"
            :class="{ active: editor.isActive('orderedList') }"
            title="有序列表"
            @click="toggleOrderedList"
          >
            <ListOrdered :size="16" />
          </button>
          <button
            class="floating-button"
            :class="{ active: editor.isActive('blockquote') }"
            title="引用"
            @click="toggleBlockquote"
          >
            <Quote :size="16" />
          </button>
          <div class="floating-divider"></div>
          <button
            class="floating-button"
            title="图片"
            @click="insertImage"
          >
            <Image :size="16" />
          </button>
          <button
            class="floating-button"
            :class="{ active: editor.isActive('codeBlock') }"
            title="代码块"
            @click="toggleCodeBlock"
          >
            <Code :size="16" />
          </button>
          <button
            class="floating-button"
            title="表格"
            @click="insertTable"
          >
            <Table :size="16" />
          </button>
          <button
            class="floating-button"
            title="分隔线"
            @click="insertHorizontalRule"
          >
            <Plus :size="16" />
          </button>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.floating-menu {
  position: fixed;
  background: var(--word-bg, #ffffff);
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  padding: 4px;
  z-index: 1000;
}

.floating-menu-content {
  display: flex;
  align-items: center;
  gap: 2px;
}

.floating-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  border-radius: 4px;
  color: var(--word-text-primary, #333);
  cursor: pointer;
  transition: all 0.2s;
}

.floating-button:hover {
  background: var(--word-button-hover, #f5f5f5);
}

.floating-button.active {
  background: var(--word-accent, #007bff);
  color: white;
}

.floating-divider {
  width: 1px;
  height: 20px;
  background: var(--word-border, #e0e0e0);
  margin: 0 4px;
}

.floating-enter-active,
.floating-leave-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}

.floating-enter-from,
.floating-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

/* Dark mode */
:global(.dark) .floating-menu {
  background: #1e1e1e;
  border-color: #3e3e42;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
}

:global(.dark) .floating-button {
  color: #cccccc;
}

:global(.dark) .floating-button:hover {
  background: #2d2d30;
}

:global(.dark) .floating-button.active {
  background: #0078d4;
  color: white;
}

:global(.dark) .floating-divider {
  background: #3e3e42;
}
</style>
