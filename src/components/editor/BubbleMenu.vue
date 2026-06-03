<script setup lang="ts">
import {
  Bold, Italic, Underline, Strikethrough,
  Code, Link, List, ListOrdered,
  Quote, Heading1, Heading2, Heading3
} from 'lucide-vue-next';

const props = defineProps<{
  editor: any;
  show: boolean;
  x: number;
  y: number;
}>();

const emit = defineEmits<{ // eslint-disable-line @typescript-eslint/no-unused-vars
  (e: 'close'): void;
}>();

const toggleBold = () => {
  props.editor?.chain().focus().toggleBold().run();
};

const toggleItalic = () => {
  props.editor?.chain().focus().toggleItalic().run();
};

const toggleUnderline = () => {
  props.editor?.chain().focus().toggleUnderline().run();
};

const toggleStrike = () => {
  props.editor?.chain().focus().toggleStrike().run();
};

const toggleCode = () => {
  props.editor?.chain().focus().toggleCode().run();
};

const setLink = () => {
  const url = window.prompt('输入链接地址:');
  if (url) {
    props.editor?.chain().focus().setLink({ href: url }).run();
  }
};

const unsetLink = () => {
  props.editor?.chain().focus().unsetLink().run();
};

const toggleBulletList = () => {
  props.editor?.chain().focus().toggleBulletList().run();
};

const toggleOrderedList = () => {
  props.editor?.chain().focus().toggleOrderedList().run();
};

const toggleBlockquote = () => {
  props.editor?.chain().focus().toggleBlockquote().run();
};

const setHeading = (level: 1 | 2 | 3) => {
  props.editor?.chain().focus().toggleHeading({ level }).run();
};
</script>

<template>
  <Teleport to="body">
    <Transition name="bubble">
      <div
        v-if="show && editor"
        class="bubble-menu"
        :style="{ left: x + 'px', top: y + 'px' }"
      >
        <div class="bubble-menu-content">
          <button
            class="bubble-button"
            :class="{ active: editor.isActive('bold') }"
            title="加粗"
            @click="toggleBold"
          >
            <Bold :size="16" />
          </button>
          <button
            class="bubble-button"
            :class="{ active: editor.isActive('italic') }"
            title="斜体"
            @click="toggleItalic"
          >
            <Italic :size="16" />
          </button>
          <button
            class="bubble-button"
            :class="{ active: editor.isActive('underline') }"
            title="下划线"
            @click="toggleUnderline"
          >
            <Underline :size="16" />
          </button>
          <button
            class="bubble-button"
            :class="{ active: editor.isActive('strike') }"
            title="删除线"
            @click="toggleStrike"
          >
            <Strikethrough :size="16" />
          </button>
          <button
            class="bubble-button"
            :class="{ active: editor.isActive('code') }"
            title="代码"
            @click="toggleCode"
          >
            <Code :size="16" />
          </button>
          <div class="bubble-divider"></div>
          <button
            class="bubble-button"
            :class="{ active: editor.isActive('link') }"
            title="链接"
            @click="editor.isActive('link') ? unsetLink() : setLink()"
          >
            <Link :size="16" />
          </button>
          <div class="bubble-divider"></div>
          <button
            class="bubble-button"
            :class="{ active: editor.isActive('bulletList') }"
            title="无序列表"
            @click="toggleBulletList"
          >
            <List :size="16" />
          </button>
          <button
            class="bubble-button"
            :class="{ active: editor.isActive('orderedList') }"
            title="有序列表"
            @click="toggleOrderedList"
          >
            <ListOrdered :size="16" />
          </button>
          <button
            class="bubble-button"
            :class="{ active: editor.isActive('blockquote') }"
            title="引用"
            @click="toggleBlockquote"
          >
            <Quote :size="16" />
          </button>
          <div class="bubble-divider"></div>
          <button
            class="bubble-button"
            :class="{ active: editor.isActive('heading', { level: 1 }) }"
            title="标题1"
            @click="setHeading(1)"
          >
            <Heading1 :size="16" />
          </button>
          <button
            class="bubble-button"
            :class="{ active: editor.isActive('heading', { level: 2 }) }"
            title="标题2"
            @click="setHeading(2)"
          >
            <Heading2 :size="16" />
          </button>
          <button
            class="bubble-button"
            :class="{ active: editor.isActive('heading', { level: 3 }) }"
            title="标题3"
            @click="setHeading(3)"
          >
            <Heading3 :size="16" />
          </button>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.bubble-menu {
  position: fixed;
  background: var(--word-bg, #ffffff);
  border: 1px solid var(--word-border, #e0e0e0);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  padding: 4px;
  z-index: 1000;
}

.bubble-menu-content {
  display: flex;
  align-items: center;
  gap: 2px;
}

.bubble-button {
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

.bubble-button:hover {
  background: var(--word-button-hover, #f5f5f5);
}

.bubble-button.active {
  background: var(--word-accent, #007bff);
  color: white;
}

.bubble-divider {
  width: 1px;
  height: 20px;
  background: var(--word-border, #e0e0e0);
  margin: 0 4px;
}

.bubble-enter-active,
.bubble-leave-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}

.bubble-enter-from,
.bubble-leave-to {
  opacity: 0;
  transform: translateY(4px);
}

/* Dark mode */
:global(.dark) .bubble-menu {
  background: #1e1e1e;
  border-color: #3e3e42;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
}

:global(.dark) .bubble-button {
  color: #cccccc;
}

:global(.dark) .bubble-button:hover {
  background: #2d2d30;
}

:global(.dark) .bubble-button.active {
  background: #0078d4;
  color: white;
}

:global(.dark) .bubble-divider {
  background: #3e3e42;
}
</style>
