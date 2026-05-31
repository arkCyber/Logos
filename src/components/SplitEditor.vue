<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface EditorState {
  file_id: string;
  source: string;
  cursor_position: number;
  selection: [number, number] | null;
  last_modified: number;
  is_dirty: boolean;
}

interface PreviewState {
  page_count: number;
  current_page: number;
  zoom: number;
  last_rendered: number;
  is_rendering: boolean;
}

interface EditorEvent {
  SourceChanged?: { file_id: string; new_content: string; position: number };
  CursorMoved?: { file_id: string; position: number };
  SelectionChanged?: { file_id: string; selection: [number, number] | null };
  PreviewUpdated?: { file_id: string; page_count: number };
  Error?: { file_id: string; message: string };
}

const props = defineProps<{
  fileId?: string;
  initialContent?: string;
  enableRealtimePreview?: boolean;
  enableSyncScroll?: boolean;
  enableTwoWaySync?: boolean;
}>();

const emit = defineEmits<{
  (e: 'content-changed', content: string): void;
  (e: 'cursor-moved', position: number): void;
  (e: 'selection-changed', selection: [number, number] | null): void;
  (e: 'preview-updated', pageCount: number): void;
  (e: 'error', message: string): void;
}>();

// State
const sourceCode = ref(props.initialContent || '');
const cursorPosition = ref(0);
const selection = ref<[number, number] | null>(null);
const previewPages = ref(0);
const currentPage = ref(0);
const isRendering = ref(false);
const zoom = ref(1.0);

// Refs
const codeEditorRef = ref<HTMLTextAreaElement | null>(null);
const previewContainerRef = ref<HTMLDivElement | null>(null);

// Configuration
const config = {
  enable_realtime_preview: props.enableRealtimePreview ?? true,
  enable_sync_scroll: props.enableSyncScroll ?? true,
  enable_two_way_sync: props.enableTwoWaySync ?? true,
  auto_save_interval_ms: 30000,
  preview_update_delay_ms: 500
};

// Initialize preview editor
const initializeEditor = async () => {
  if (!props.fileId) {
return;
}

  try {
    await invoke('preview_editor_open_file', {
      fileId: props.fileId,
      content: sourceCode.value,
      config
    });
  } catch (error) {
    console.error('Failed to initialize preview editor:', error);
    emit('error', `Failed to initialize editor: ${error}`);
  }
};

// Update source code
const updateSource = async (newContent: string, position: number) => {
  if (!props.fileId) {
return;
}

  try {
    await invoke('preview_editor_update_source', {
      fileId: props.fileId,
      newContent,
      position
    });
    sourceCode.value = newContent;
    cursorPosition.value = position;
    emit('content-changed', newContent);
  } catch (error) {
    console.error('Failed to update source:', error);
    emit('error', `Failed to update source: ${error}`);
  }
};

// Move cursor
const moveCursor = async (position: number) => {
  if (!props.fileId) {
return;
}

  try {
    await invoke('preview_editor_move_cursor', {
      fileId: props.fileId,
      position
    });
    cursorPosition.value = position;
    emit('cursor-moved', position);
  } catch (error) {
    console.error('Failed to move cursor:', error);
  }
};

// Set selection
const setSelection = async (sel: [number, number] | null) => {
  if (!props.fileId) {
return;
}

  try {
    await invoke('preview_editor_set_selection', {
      fileId: props.fileId,
      selection: sel
    });
    selection.value = sel;
    emit('selection-changed', sel);
  } catch (error) {
    console.error('Failed to set selection:', error);
  }
};

// Sync cursor to preview position
const syncCursorToPreview = async (page: number, x: number, y: number) => {
  if (!props.fileId || !config.enable_sync_scroll) {
return;
}

  try {
    await invoke('preview_editor_sync_cursor_to_preview', {
      fileId: props.fileId,
      page,
      x,
      y
    });
  } catch (error) {
    console.error('Failed to sync cursor to preview:', error);
  }
};

// Handle code editor input
const handleCodeInput = async (event: Event) => {
  const target = event.target as HTMLTextAreaElement;
  const newContent = target.value;
  const position = target.selectionStart;

  await updateSource(newContent, position);
};

// Handle code editor cursor movement
const handleCodeCursorMove = async (event: Event) => {
  const target = event.target as HTMLTextAreaElement;
  const position = target.selectionStart;

  if (position !== cursorPosition.value) {
    await moveCursor(position);
  }
};

// Handle code editor selection
const handleCodeSelection = async (event: Event) => {
  const target = event.target as HTMLTextAreaElement;
  const start = target.selectionStart;
  const end = target.selectionEnd;

  if (start !== end) {
    await setSelection([start, end]);
  } else {
    await setSelection(null);
  }
};

// Handle preview click
const handlePreviewClick = async (event: MouseEvent) => {
  if (!previewContainerRef.value || !props.fileId) {
return;
}

  const rect = previewContainerRef.value.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;

  await syncCursorToPreview(currentPage.value, x, y);
};

// Handle preview scroll
const handlePreviewScroll = async (event: Event) => {
  if (!config.enable_sync_scroll) {
return;
}

  const target = event.target as HTMLElement;
  const scrollTop = target.scrollTop;
  const scrollHeight = target.scrollHeight;
  const clientHeight = target.clientHeight;

  // Calculate current page based on scroll position
  const page = Math.floor((scrollTop / (scrollHeight - clientHeight)) * previewPages.value);
  if (page !== currentPage.value) {
    currentPage.value = page;
  }
};

// Listen to editor events from backend
const setupEventListeners = async () => {
  if (!props.fileId) {
return () => {};
}

  const unlisten = await listen<EditorEvent>(`editor-event-${props.fileId}`, (event) => {
    const payload = event.payload;

    if (payload.SourceChanged) {
      sourceCode.value = payload.SourceChanged.new_content;
      cursorPosition.value = payload.SourceChanged.position;
      emit('content-changed', payload.SourceChanged.new_content);
    }

    if (payload.CursorMoved) {
      cursorPosition.value = payload.CursorMoved.position;
      emit('cursor-moved', payload.CursorMoved.position);
    }

    if (payload.SelectionChanged) {
      selection.value = payload.SelectionChanged.selection;
      emit('selection-changed', payload.SelectionChanged.selection);
    }

    if (payload.PreviewUpdated) {
      previewPages.value = payload.PreviewUpdated.page_count;
      emit('preview-updated', payload.PreviewUpdated.page_count);
    }

    if (payload.Error) {
      emit('error', payload.Error.message);
    }
  });

  return unlisten;
};

// Lifecycle
onMounted(async () => {
  await initializeEditor();
  const unlisten = await setupEventListeners();

  onBeforeUnmount(() => {
    if (unlisten) {
      unlisten();
    }
  });
});

// Watch for prop changes
watch(() => props.initialContent, (newContent) => {
  if (newContent !== undefined && newContent !== sourceCode.value) {
    updateSource(newContent, cursorPosition.value);
  }
});

watch(() => props.fileId, async (newFileId) => {
  if (newFileId) {
    await initializeEditor();
  }
});
</script>

<template>
  <div class="split-editor">
    <!-- Left Panel: Code Editor -->
    <div class="editor-panel">
      <div class="panel-header">
        <h3>Typst Source</h3>
        <div class="panel-actions">
          <button class="btn-save" @click="() => emit('content-changed', sourceCode)">
            Save
          </button>
        </div>
      </div>
      <textarea
        ref="codeEditorRef"
        v-model="sourceCode"
        class="code-editor"
        spellcheck="false"
        placeholder="Enter Typst code here..."
        @input="handleCodeInput"
        @click="handleCodeCursorMove"
        @keyup="handleCodeCursorMove"
        @select="handleCodeSelection"
      ></textarea>
      <div class="editor-footer">
        <span class="cursor-position">Position: {{ cursorPosition }}</span>
        <span v-if="selection" class="selection-info">
          Selection: {{ selection[0] }} - {{ selection[1] }}
        </span>
      </div>
    </div>

    <!-- Right Panel: Preview -->
    <div class="preview-panel">
      <div class="panel-header">
        <h3>Preview</h3>
        <div class="panel-actions">
          <button class="btn-zoom" @click="zoom = Math.max(0.5, zoom - 0.1)">-</button>
          <span class="zoom-level">{{ Math.round(zoom * 100) }}%</span>
          <button class="btn-zoom" @click="zoom = Math.min(3, zoom + 0.1)">+</button>
        </div>
      </div>
      <div
        ref="previewContainerRef"
        class="preview-container"
        @click="handlePreviewClick"
        @scroll="handlePreviewScroll"
      >
        <div v-if="isRendering" class="rendering-indicator">
          Rendering...
        </div>
        <div
          v-else
          class="preview-content"
          :style="{ transform: `scale(${zoom})` }"
        >
          <!-- Preview content will be rendered here -->
          <div v-if="previewPages === 0" class="empty-preview">
            No preview available
          </div>
          <div v-else class="pages">
            <div
              v-for="page in previewPages"
              :key="page"
              class="page"
              :class="{ active: page === currentPage }"
            >
              Page {{ page }}
            </div>
          </div>
        </div>
      </div>
      <div class="preview-footer">
        <span class="page-info">Page {{ currentPage + 1 }} / {{ previewPages }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.split-editor {
  display: flex;
  height: 100vh;
  width: 100%;
  overflow: hidden;
}

.editor-panel,
.preview-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  border-right: 1px solid #e0e0e0;
}

.preview-panel {
  border-right: none;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  background: #f5f5f5;
  border-bottom: 1px solid #e0e0e0;
}

.panel-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #333;
}

.panel-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

.btn-save,
.btn-zoom {
  padding: 4px 12px;
  border: 1px solid #d0d0d0;
  background: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  transition: all 0.2s;
}

.btn-save:hover,
.btn-zoom:hover {
  background: #f0f0f0;
}

.zoom-level {
  font-size: 12px;
  color: #666;
  min-width: 40px;
  text-align: center;
}

.code-editor {
  flex: 1;
  padding: 16px;
  border: none;
  resize: none;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 14px;
  line-height: 1.6;
  background: #fafafa;
  color: #333;
  outline: none;
}

.code-editor::placeholder {
  color: #999;
}

.preview-container {
  flex: 1;
  overflow: auto;
  background: #f0f0f0;
  position: relative;
}

.preview-content {
  min-height: 100%;
  transform-origin: top left;
  transition: transform 0.2s;
}

.rendering-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #666;
  font-size: 14px;
}

.empty-preview {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #999;
  font-size: 14px;
}

.pages {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.page {
  background: white;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  min-height: 400px;
  padding: 40px;
  border-radius: 4px;
}

.page.active {
  outline: 2px solid #2196f3;
}

.editor-footer,
.preview-footer {
  padding: 8px 16px;
  background: #f5f5f5;
  border-top: 1px solid #e0e0e0;
  font-size: 12px;
  color: #666;
  display: flex;
  gap: 16px;
}

.cursor-position,
.selection-info,
.page-info {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}
</style>
