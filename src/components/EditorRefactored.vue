<script setup lang="ts">
import { ref, onMounted, watch, nextTick } from 'vue';
import { EditorContent } from '@tiptap/vue-3';
import QuickAccessToolbar from './editor/QuickAccessToolbar.vue';
import RibbonToolbar from './editor/RibbonToolbar.vue';
import StatusBar from './editor/StatusBar.vue';
import { useEditorState } from '../composables/useEditorState';
import { useDocumentOperations } from '../composables/useDocumentOperations';

// Use composables
const {
  isDarkMode,
  autoSaveEnabled,
  zoomLevel,
  fontSize,
  fontFamily,
  wordCount,
  charCount,
  currentPage,
  totalPages,
  toggleTheme,
  zoomIn,
  zoomOut,
  zoom100
} = useEditorState();

const {
  editor,
  toggleBold,
  toggleItalic,
  toggleUnderline,
  toggleStrike,
  toggleHeading,
  toggleBulletList,
  toggleOrderedList,
  setTextAlign,
  undo,
  redo,
  cutSelection,
  copySelection,
  pasteFromClipboard,
  getWordCount,
  getCharCount
} = useDocumentOperations();

// Ribbon state
const activeRibbonTab = ref('home');
const showFileBackstage = ref(false);

// Update word count on editor changes
watch(editor, () => {
  if (editor.value) {
    wordCount.value = getWordCount();
    charCount.value = getCharCount();
  }
});

// Ribbon event handlers
const handleRibbonAction = (action: string, payload?: any) => {
  switch (action) {
    case 'toggle-bold':
      toggleBold();
      break;
    case 'toggle-italic':
      toggleItalic();
      break;
    case 'toggle-underline':
      toggleUnderline();
      break;
    case 'toggle-strike':
      toggleStrike();
      break;
    case 'set-heading':
      toggleHeading(payload);
      break;
    case 'toggle-bullet-list':
      toggleBulletList();
      break;
    case 'toggle-ordered-list':
      toggleOrderedList();
      break;
    case 'set-text-align':
      setTextAlign(payload);
      break;
    case 'paste':
      pasteFromClipboard();
      break;
    case 'cut':
      cutSelection();
      break;
    case 'copy':
      copySelection();
      break;
    case 'update-font-family':
      fontFamily.value = payload;
      break;
    case 'update-font-size':
      fontSize.value = payload;
      break;
  }
};

// Quick access toolbar handlers
const handleQuickAccessAction = (action: string) => {
  switch (action) {
    case 'save':
      console.log('Save document');
      break;
    case 'undo':
      undo();
      break;
    case 'redo':
      redo();
      break;
    case 'toggle-search':
      console.log('Toggle search dialog');
      break;
  }
};

onMounted(() => {
  console.log('Editor mounted');
});
</script>

<template>
  <div class="editor-container" :class="{ dark: isDarkMode }">
    <!-- Quick Access Toolbar -->
    <QuickAccessToolbar
      :show-file-backstage="showFileBackstage"
      @toggle-file-backstage="showFileBackstage = !showFileBackstage"
      @save="handleQuickAccessAction('save')"
      @undo="handleQuickAccessAction('undo')"
      @redo="handleQuickAccessAction('redo')"
      @toggle-search="handleQuickAccessAction('toggle-search')"
    />

    <!-- Ribbon Toolbar -->
    <RibbonToolbar
      :active-tab="activeRibbonTab"
      :font-family="fontFamily"
      :font-size="fontSize"
      @set-active-tab="activeRibbonTab = $event"
      @toggle-bold="handleRibbonAction('toggle-bold')"
      @toggle-italic="handleRibbonAction('toggle-italic')"
      @toggle-underline="handleRibbonAction('toggle-underline')"
      @toggle-strike="handleRibbonAction('toggle-strike')"
      @set-heading="handleRibbonAction('set-heading', $event)"
      @toggle-bullet-list="handleRibbonAction('toggle-bullet-list')"
      @toggle-ordered-list="handleRibbonAction('toggle-ordered-list')"
      @set-text-align="handleRibbonAction('set-text-align', $event)"
      @paste="handleRibbonAction('paste')"
      @cut="handleRibbonAction('cut')"
      @copy="handleRibbonAction('copy')"
      @update-font-family="handleRibbonAction('update-font-family', $event)"
      @update-font-size="handleRibbonAction('update-font-size', $event)"
    />

    <!-- Editor Container -->
    <div class="editor-container-wrapper">
      <div class="page-container a4">
        <EditorContent :editor="editor" />
      </div>
    </div>

    <!-- Status Bar -->
    <StatusBar
      :word-count="wordCount"
      :char-count="charCount"
      :current-page="currentPage"
      :total-pages="totalPages"
      :zoom-level="zoomLevel"
      :is-dark-mode="isDarkMode"
      @zoom-in="zoomIn"
      @zoom-out="zoomOut"
      @zoom-100="zoom100"
      @toggle-theme="toggleTheme"
    />
  </div>
</template>

<style scoped>
.editor-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.editor-container.dark {
  background: var(--word-bg-canvas);
}

.editor-container-wrapper {
  flex: 1;
  display: flex;
  overflow: hidden;
  background: var(--word-bg-canvas);
  min-height: 0;
}

.page-container {
  background: var(--word-bg-page);
  box-shadow: var(--word-shadow-page);
  margin: 20px auto;
  width: 794px;
  min-height: 1123px;
  padding: 50px;
  overflow-y: auto;
}

.page-container.a4 {
  width: 794px;
  min-height: 1123px;
}

:deep(.ProseMirror) {
  outline: none;
  min-height: 100%;
}

:deep(.ProseMirror p) {
  margin: 1em 0;
}

:deep(.ProseMirror h1) {
  font-size: 2em;
  font-weight: bold;
  margin: 0.67em 0;
}

:deep(.ProseMirror h2) {
  font-size: 1.5em;
  font-weight: bold;
  margin: 0.75em 0;
}

:deep(.ProseMirror h3) {
  font-size: 1.17em;
  font-weight: bold;
  margin: 0.83em 0;
}

:deep(.ProseMirror ul),
:deep(.ProseMirror ol) {
  padding-left: 1.5em;
}

:deep(.ProseMirror blockquote) {
  border-left: 3px solid #ddd;
  padding-left: 1em;
  margin-left: 0;
  color: #666;
}

:deep(.ProseMirror code) {
  background: #f4f4f4;
  padding: 0.2em 0.4em;
  border-radius: 3px;
  font-family: monospace;
}

:deep(.ProseMirror pre) {
  background: #f4f4f4;
  padding: 1em;
  border-radius: 5px;
  overflow-x: auto;
}

:deep(.ProseMirror pre code) {
  background: none;
  padding: 0;
}
</style>
