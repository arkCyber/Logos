<script setup lang="ts">
import { ref, computed, onMounted, watch, shallowRef } from 'vue';
import type { PresentationDocument, Slide, SlideElement, TextElement, ElementType } from '../types/presentation';
import { createEmptyPresentation, createEmptySlide, ElementType as ET } from '../types/presentation';
import { PresentationConverter } from '../utils/presentationConverter';

// Simple debounce implementation
function debounce<T extends (...args: any[]) => any>(func: T, wait: number): T {
  let timeout: number | null = null;
  return ((...args: Parameters<T>) => {
    if (timeout) {
clearTimeout(timeout);
}
    timeout = window.setTimeout(() => func(...args), wait);
  }) as T;
}

// Props
const props = defineProps<{
  document?: PresentationDocument;
}>();

// Emits
const emit = defineEmits<{
  'update:document': [doc: PresentationDocument];
  'save': [doc: PresentationDocument];
}>();

// State
const document = shallowRef<PresentationDocument>(props.document || createEmptyPresentation('Untitled Presentation'));
const currentSlideIndex = ref(0);
const selectedElementId = ref<string | null>(null);
const history = shallowRef<PresentationDocument[]>([]);
const historyIndex = ref(-1);
const draggingIndex = ref<number | null>(null);

// Computed
const slides = computed(() => document.value.slides);
const currentSlide = computed(() => slides.value[currentSlideIndex.value] || createEmptySlide(0));
const selectedElement = computed(() => {
  if (!selectedElementId.value) {
return null;
}
  return currentSlide.value.elements.find(el => el.id === selectedElementId.value) || null;
});
const canUndo = computed(() => historyIndex.value > 0);
const canRedo = computed(() => historyIndex.value < history.value.length - 1);

const canvasStyle = computed(() => {
  const bgValue = currentSlide.value.background?.value;
  return {
    width: document.value.settings.aspectRatio === '16-9' ? '960px' : '800px',
    height: document.value.settings.aspectRatio === '16-9' ? '540px' : '600px',
    backgroundColor: (bgValue as string | undefined) ?? '#ffffff'
  };
});

// Methods
function selectSlide(index: number) {
  currentSlideIndex.value = index;
  selectedElementId.value = null;
}

function selectElement(id: string) {
  selectedElementId.value = id;
}

function deselectElement() {
  selectedElementId.value = null;
}

function addSlide() {
  const newSlide = createEmptySlide(slides.value.length);
  document.value.slides.push(newSlide);
  currentSlideIndex.value = slides.value.length - 1;
  saveToHistory();
  emit('update:document', document.value);
}

function deleteCurrentSlide() {
  if (slides.value.length <= 1) {
return;
}
  document.value.slides.splice(currentSlideIndex.value, 1);
  if (currentSlideIndex.value >= slides.value.length) {
    currentSlideIndex.value = slides.value.length - 1;
  }
  saveToHistory();
  emit('update:document', document.value);
}

function addElement(type: string) {
  const elementType = type as ElementType;
  let element: SlideElement;

  if (elementType === ET.TEXT) {
    element = {
      id: crypto.randomUUID(),
      type: ET.TEXT,
      position: { x: 100, y: 100 },
      size: { width: 200, height: 100 },
      content: 'Double-click to edit',
      style: {
        fontSize: 24,
        color: '#333333'
      }
    } as TextElement;
  } else if (elementType === ET.SHAPE) {
    element = {
      id: crypto.randomUUID(),
      type: ET.SHAPE,
      position: { x: 100, y: 100 },
      size: { width: 200, height: 100 },
      shape: 'rectangle',
      style: {
        fillColor: '#007bff'
      }
    } as any;
  } else if (elementType === ET.TABLE) {
    element = {
      id: crypto.randomUUID(),
      type: ET.TABLE,
      position: { x: 100, y: 100 },
      size: { width: 300, height: 200 },
      rows: 3,
      columns: 3,
      data: [['', '', ''], ['', '', ''], ['', '', '']]
    } as any;
  } else if (elementType === ET.IMAGE) {
    element = {
      id: crypto.randomUUID(),
      type: ET.IMAGE,
      position: { x: 100, y: 100 },
      size: { width: 200, height: 150 },
      src: ''
    } as any;
  } else if (elementType === ET.CODE) {
    element = {
      id: crypto.randomUUID(),
      type: ET.CODE,
      position: { x: 100, y: 100 },
      size: { width: 400, height: 200 },
      content: '',
      language: 'text'
    } as any;
  } else {
    element = {
      id: crypto.randomUUID(),
      type: elementType,
      position: { x: 100, y: 100 },
      size: { width: 200, height: 100 }
    } as any;
  }

  currentSlide.value.elements.push(element);
  selectedElementId.value = element.id;
  saveToHistory();
  emit('update:document', document.value);
}

function elementStyle(element: SlideElement) {
  return {
    position: 'absolute' as const,
    left: `${element.position.x}px`,
    top: `${element.position.y}px`,
    width: `${element.size.width}px`,
    height: `${element.size.height}px`,
    transform: `rotate(${element.rotation || 0}deg)`,
    opacity: element.opacity ?? 1,
    zIndex: element.zIndex ?? 1
  };
}

function shapeStyle(element: any) {
  const style: any = {
    width: '100%',
    height: '100%',
    backgroundColor: element.style?.fillColor || '#007bff'
  };

  if (element.shape === 'circle') {
    style.borderRadius = '50%';
  }

  return style;
}

function updateTextContent(element: TextElement, event: Event) {
  const target = event.target as HTMLElement;
  element.content = target.innerText;
  saveToHistory();
  emit('update:document', document.value);
}

function updateElement() {
  debouncedEmit();
}

function updateSlide() {
  debouncedEmit();
}

// Debounced emit to reduce frequent updates
const debouncedEmit = debounce(() => {
  emit('update:document', document.value);
}, 100);

// Drag and resize state
let isDragging = false;
let isResizing = false;
let dragStart = { x: 0, y: 0 };
let elementStart = { x: 0, y: 0, width: 0, height: 0 };
let resizeHandle = '';

function startDrag(event: MouseEvent, element: SlideElement) {
  isDragging = true;
  dragStart = { x: event.clientX, y: event.clientY };
  elementStart = { ...element.position, width: element.size.width, height: element.size.height };

  window.addEventListener('mousemove', onDrag);
  window.addEventListener('mouseup', stopDrag);
}

function onDrag(event: MouseEvent) {
  if (!isDragging || !selectedElement.value) {
return;
}

  const dx = event.clientX - dragStart.x;
  const dy = event.clientY - dragStart.y;

  selectedElement.value.position.x = elementStart.x + dx;
  selectedElement.value.position.y = elementStart.y + dy;
}

function stopDrag() {
  if (isDragging) {
    isDragging = false;
    saveToHistory();
    emit('update:document', document.value);
  }
  window.removeEventListener('mousemove', onDrag);
  window.removeEventListener('mouseup', stopDrag);
}

function startResize(event: MouseEvent, element: SlideElement, handle: string) {
  isResizing = true;
  resizeHandle = handle;
  dragStart = { x: event.clientX, y: event.clientY };
  elementStart = { ...element.position, width: element.size.width, height: element.size.height };

  window.addEventListener('mousemove', onResize);
  window.addEventListener('mouseup', stopResize);
}

function onResize(event: MouseEvent) {
  if (!isResizing || !selectedElement.value) {
return;
}

  const dx = event.clientX - dragStart.x;
  const dy = event.clientY - dragStart.y;

  if (resizeHandle.includes('e')) {
    selectedElement.value.size.width = Math.max(50, elementStart.width + dx);
  }
  if (resizeHandle.includes('w')) {
    selectedElement.value.size.width = Math.max(50, elementStart.width - dx);
    selectedElement.value.position.x = elementStart.x + dx;
  }
  if (resizeHandle.includes('s')) {
    selectedElement.value.size.height = Math.max(50, elementStart.height + dy);
  }
  if (resizeHandle.includes('n')) {
    selectedElement.value.size.height = Math.max(50, elementStart.height - dy);
    selectedElement.value.position.y = elementStart.y + dy;
  }
}

function stopResize() {
  if (isResizing) {
    isResizing = false;
    saveToHistory();
    emit('update:document', document.value);
  }
  window.removeEventListener('mousemove', onResize);
  window.removeEventListener('mouseup', stopResize);
}

// History management
function saveToHistory() {
  // Remove any future history if we're not at the end
  if (historyIndex.value < history.value.length - 1) {
    history.value = history.value.slice(0, historyIndex.value + 1);
  }
  
  history.value.push(JSON.parse(JSON.stringify(document.value)));
  historyIndex.value = history.value.length - 1;
}

function undo() {
  if (canUndo.value) {
    historyIndex.value--;
    document.value = JSON.parse(JSON.stringify(history.value[historyIndex.value]));
    emit('update:document', document.value);
  }
}

function redo() {
  if (canRedo.value) {
    historyIndex.value++;
    document.value = JSON.parse(JSON.stringify(history.value[historyIndex.value]));
    emit('update:document', document.value);
  }
}

// Export
function exportPresentation(format: string) {
  let content: string;
  let filename: string;
  let mimeType: string;

  switch (format) {
    case 'slidev':
      content = PresentationConverter.toSlidev(document.value);
      filename = `${document.value.metadata.title}.md`;
      mimeType = 'text/markdown';
      break;
    case 'typst':
      content = PresentationConverter.toTypst(document.value);
      filename = `${document.value.metadata.title}.typ`;
      mimeType = 'text/plain';
      break;
    case 'pptx':
      // This would call the Rust backend
      alert('PPTX export requires backend integration');
      return;
    default:
      return;
  }

  const blob = new Blob([content], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const a = window.document.createElement('a');
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

// Drag and drop handlers
function onDragStart(event: DragEvent, index: number) {
  draggingIndex.value = index;
  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = 'move';
    event.dataTransfer.setData('text/plain', index.toString());
  }
}

function onDragOver(event: DragEvent, index: number) {
  event.preventDefault();
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move';
  }
}

function onDrop(event: DragEvent, targetIndex: number) {
  event.preventDefault();
  
  const sourceIndex = draggingIndex.value;
  if (sourceIndex === null || sourceIndex === targetIndex) {
    return;
  }

  // Reorder slides
  const slides = document.value.slides;
  const [movedSlide] = slides.splice(sourceIndex, 1);
  slides.splice(targetIndex, 0, movedSlide);

  // Update slide indices
  slides.forEach((slide, index) => {
    slide.index = index;
  });

  // Update current slide index if needed
  if (currentSlideIndex.value === sourceIndex) {
    currentSlideIndex.value = targetIndex;
  } else if (sourceIndex < currentSlideIndex.value && targetIndex >= currentSlideIndex.value) {
    currentSlideIndex.value--;
  } else if (sourceIndex > currentSlideIndex.value && targetIndex <= currentSlideIndex.value) {
    currentSlideIndex.value++;
  }

  draggingIndex.value = null;
  saveToHistory();
  emit('update:document', document.value);
}

function onDragEnd() {
  draggingIndex.value = null;
}

// Lifecycle
onMounted(() => {
  saveToHistory();
});

watch(() => props.document, (newDoc) => {
  if (newDoc) {
    document.value = newDoc;
    saveToHistory();
  }
}, { deep: true });
</script>

<template>
  <div class="presentation-editor">
    <!-- Toolbar -->
    <div class="editor-toolbar">
      <div class="toolbar-group">
        <button class="toolbar-btn" title="Add Slide" @click="addSlide">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
            <line x1="12" y1="8" x2="12" y2="16"></line>
            <line x1="8" y1="12" x2="16" y2="12"></line>
          </svg>
        </button>
        <button class="toolbar-btn" title="Delete Slide" :disabled="slides.length <= 1" @click="deleteCurrentSlide">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="3 6 5 6 21 6"></polyline>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
          </svg>
        </button>
      </div>

      <div class="toolbar-group">
        <button class="toolbar-btn" title="Add Text" @click="addElement('text')">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M4 7V4h16v3"></path>
            <path d="M9 20h6"></path>
            <path d="M12 4v16"></path>
          </svg>
        </button>
        <button class="toolbar-btn" title="Add Image" @click="addElement('image')">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
            <circle cx="8.5" cy="8.5" r="1.5"></circle>
            <polyline points="21 15 16 10 5 21"></polyline>
          </svg>
        </button>
        <button class="toolbar-btn" title="Add Shape" @click="addElement('shape')">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          </svg>
        </button>
        <button class="toolbar-btn" title="Add Table" @click="addElement('table')">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
            <line x1="3" y1="9" x2="21" y2="9"></line>
            <line x1="3" y1="15" x2="21" y2="15"></line>
            <line x1="9" y1="3" x2="9" y2="21"></line>
            <line x1="15" y1="3" x2="15" y2="21"></line>
          </svg>
        </button>
      </div>

      <div class="toolbar-group">
        <button class="toolbar-btn" title="Undo" :disabled="!canUndo" @click="undo">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 7v6h6"></path>
            <path d="M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13"></path>
          </svg>
        </button>
        <button class="toolbar-btn" title="Redo" :disabled="!canRedo" @click="redo">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 7v6h-6"></path>
            <path d="M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3L21 13"></path>
          </svg>
        </button>
      </div>

      <div class="toolbar-group">
        <button class="toolbar-btn" title="Export as Slidev" @click="exportPresentation('slidev')">
          Export Slidev
        </button>
        <button class="toolbar-btn" title="Export as Typst" @click="exportPresentation('typst')">
          Export Typst
        </button>
        <button class="toolbar-btn" title="Export as PPTX" @click="exportPresentation('pptx')">
          Export PPTX
        </button>
      </div>
    </div>

    <!-- Main Editor Area -->
    <div class="editor-main">
      <!-- Slide Thumbnails -->
      <div class="slide-thumbnails">
        <div
          v-for="(slide, index) in slides"
          :key="slide.id"
          :class="['slide-thumbnail', { active: currentSlideIndex === index, dragging: draggingIndex === index }]"
          draggable="true"
          @click="selectSlide(index)"
          @dragstart="onDragStart($event, index)"
          @dragover="onDragOver($event, index)"
          @drop="onDrop($event, index)"
          @dragend="onDragEnd"
        >
          <div class="thumbnail-content">
            <div v-if="slide.title" class="thumbnail-title">{{ slide.title }}</div>
            <div class="thumbnail-elements">{{ slide.elements.length }} elements</div>
          </div>
          <div class="thumbnail-number">{{ index + 1 }}</div>
        </div>
      </div>

      <!-- Canvas -->
      <div class="editor-canvas">
        <div
          class="slide-canvas"
          :style="canvasStyle"
          @click.self="deselectElement"
        >
          <!-- Ensure background exists -->
          <div v-if="!currentSlide.background" class="slide-background" />
          <!-- Render slide elements -->
          <div
            v-for="element in currentSlide.elements"
            :key="element.id"
            :class="['slide-element', { selected: selectedElementId === element.id }]"
            :style="elementStyle(element)"
            @click.stop="selectElement(element.id)"
            @mousedown="startDrag($event, element)"
          >
            <!-- Text Element -->
            <div v-if="element.type === 'text'" class="element-text" contenteditable="true" @blur="updateTextContent(element, $event)">
              {{ element.content }}
            </div>

            <!-- Image Element -->
            <img v-else-if="element.type === 'image'" :src="element.src" :alt="element.alt || ''" class="element-image" />

            <!-- Shape Element -->
            <div v-else-if="element.type === 'shape'" class="element-shape" :style="shapeStyle(element)" />

            <!-- Table Element -->
            <div v-else-if="element.type === 'table'" class="element-table">
              <table>
                <tbody>
                  <tr v-for="row in element.rows" :key="row">
                    <td v-for="col in element.columns" :key="col">
                      {{ element.data[row]?.[col] || '' }}
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>

            <!-- Code Element -->
            <div v-else-if="element.type === 'code'" class="element-code">
              <pre><code>{{ element.content }}</code></pre>
            </div>

            <!-- Selection handles -->
            <div v-if="selectedElementId === element.id" class="selection-handles">
              <div class="handle handle-nw" @mousedown.stop="startResize($event, element, 'nw')"></div>
              <div class="handle handle-ne" @mousedown.stop="startResize($event, element, 'ne')"></div>
              <div class="handle handle-sw" @mousedown.stop="startResize($event, element, 'sw')"></div>
              <div class="handle handle-se" @mousedown.stop="startResize($event, element, 'se')"></div>
            </div>
          </div>
        </div>
      </div>

      <!-- Properties Panel -->
      <div class="properties-panel">
        <h3>Properties</h3>
        
        <div v-if="selectedElement" class="property-section">
          <h4>Element Properties</h4>
          
          <div class="property-row">
            <label>Type:</label>
            <span>{{ selectedElement.type }}</span>
          </div>

          <div class="property-row">
            <label>Position X:</label>
            <input v-model.number="selectedElement.position.x" type="number" @input="updateElement" />
          </div>

          <div class="property-row">
            <label>Position Y:</label>
            <input v-model.number="selectedElement.position.y" type="number" @input="updateElement" />
          </div>

          <div class="property-row">
            <label>Width:</label>
            <input v-model.number="selectedElement.size.width" type="number" @input="updateElement" />
          </div>

          <div class="property-row">
            <label>Height:</label>
            <input v-model.number="selectedElement.size.height" type="number" @input="updateElement" />
          </div>

          <div class="property-row">
            <label>Rotation:</label>
            <input v-model.number="selectedElement.rotation" type="number" @input="updateElement" />
          </div>

          <div class="property-row">
            <label>Opacity:</label>
            <input v-model.number="selectedElement.opacity" type="range" min="0" max="1" step="0.1" @input="updateElement" />
          </div>

          <!-- Text-specific properties -->
          <template v-if="selectedElement.type === 'text'">
            <div class="property-row">
              <label>Font Size:</label>
              <input v-model.number="selectedElement.style.fontSize" type="number" @input="updateElement" />
            </div>
            <div class="property-row">
              <label>Color:</label>
              <input v-model="selectedElement.style.color" type="color" @input="updateElement" />
            </div>
            <div class="property-row">
              <label>Font Weight:</label>
              <select v-model="selectedElement.style.fontWeight" @input="updateElement">
                <option value="normal">Normal</option>
                <option value="bold">Bold</option>
              </select>
            </div>
          </template>

          <!-- Shape-specific properties -->
          <template v-if="selectedElement.type === 'shape'">
            <div class="property-row">
              <label>Shape:</label>
              <select v-model="selectedElement.shape" @input="updateElement">
                <option value="rectangle">Rectangle</option>
                <option value="circle">Circle</option>
                <option value="triangle">Triangle</option>
              </select>
            </div>
            <div class="property-row">
              <label>Fill Color:</label>
              <input v-model="selectedElement.style.fillColor" type="color" @input="updateElement" />
            </div>
          </template>
        </div>

        <div v-else class="property-section">
          <p>Select an element to edit its properties</p>
        </div>

        <div class="property-section">
          <h4>Slide Properties</h4>
          <div class="property-row">
            <label>Layout:</label>
            <select v-model="currentSlide.layout" @input="updateSlide">
              <option value="blank">Blank</option>
              <option value="title">Title</option>
              <option value="title_and_content">Title and Content</option>
              <option value="two_content">Two Content</option>
            </select>
          </div>
          <div class="property-row">
            <label>Background:</label>
            <input type="color" :value="currentSlide.background?.value" @input="(e) => { if(!currentSlide.background) currentSlide.background = { type: 'color', value: (e.target as HTMLInputElement).value }; else currentSlide.background.value = (e.target as HTMLInputElement).value; updateSlide(); }" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.presentation-editor {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #f5f5f5;
}

.editor-toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: white;
  border-bottom: 1px solid #e0e0e0;
}

.toolbar-group {
  display: flex;
  gap: 4px;
  padding-right: 12px;
  border-right: 1px solid #e0e0e0;
}

.toolbar-group:last-child {
  border-right: none;
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  padding: 0;
  background: transparent;
  border: 1px solid transparent;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.toolbar-btn:hover:not(:disabled) {
  background: #f0f0f0;
  border-color: #d0d0d0;
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.editor-main {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.slide-thumbnails {
  width: 200px;
  background: white;
  border-right: 1px solid #e0e0e0;
  overflow-y: auto;
  padding: 12px;
}

.slide-thumbnail {
  position: relative;
  width: 100%;
  aspect-ratio: 16/9;
  background: #f5f5f5;
  border: 2px solid transparent;
  border-radius: 4px;
  margin-bottom: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.slide-thumbnail:hover {
  border-color: #007bff;
}

.slide-thumbnail.active {
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.2);
}

.slide-thumbnail.dragging {
  opacity: 0.5;
  border-style: dashed;
}

.thumbnail-content {
  padding: 8px;
  font-size: 12px;
}

.thumbnail-title {
  font-weight: bold;
  margin-bottom: 4px;
}

.thumbnail-elements {
  color: #666;
}

.thumbnail-number {
  position: absolute;
  bottom: 4px;
  right: 4px;
  background: rgba(0, 0, 0, 0.5);
  color: white;
  padding: 2px 6px;
  border-radius: 10px;
  font-size: 10px;
}

.editor-canvas {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #e0e0e0;
  overflow: auto;
  padding: 24px;
}

.slide-canvas {
  position: relative;
  background: white;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.slide-element {
  position: absolute;
  cursor: move;
  user-select: none;
}

.slide-element.selected {
  outline: 2px solid #007bff;
}

.element-text {
  width: 100%;
  height: 100%;
  padding: 8px;
  outline: none;
  overflow: hidden;
}

.element-image {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.element-shape {
  width: 100%;
  height: 100%;
}

.element-table table {
  width: 100%;
  height: 100%;
  border-collapse: collapse;
}

.element-table td {
  border: 1px solid #ccc;
  padding: 4px;
  text-align: center;
}

.element-code {
  width: 100%;
  height: 100%;
  background: #f5f5f5;
  padding: 8px;
  overflow: auto;
  font-family: monospace;
  font-size: 12px;
}

.selection-handles {
  position: absolute;
  top: -4px;
  left: -4px;
  right: -4px;
  bottom: -4px;
  pointer-events: none;
}

.handle {
  position: absolute;
  width: 8px;
  height: 8px;
  background: white;
  border: 1px solid #007bff;
  pointer-events: auto;
}

.handle-nw {
  top: 0;
  left: 0;
  cursor: nw-resize;
}

.handle-ne {
  top: 0;
  right: 0;
  cursor: ne-resize;
}

.handle-sw {
  bottom: 0;
  left: 0;
  cursor: sw-resize;
}

.handle-se {
  bottom: 0;
  right: 0;
  cursor: se-resize;
}

.properties-panel {
  width: 280px;
  background: white;
  border-left: 1px solid #e0e0e0;
  overflow-y: auto;
  padding: 16px;
}

.properties-panel h3 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: bold;
}

.property-section {
  margin-bottom: 24px;
}

.property-section h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: bold;
  color: #666;
}

.property-row {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
}

.property-row label {
  width: 100px;
  font-size: 12px;
  color: #666;
}

.property-row input,
.property-row select {
  flex: 1;
  padding: 4px 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 12px;
}

.property-row input[type="color"] {
  width: 40px;
  height: 28px;
  padding: 0;
}

.property-row input[type="range"] {
  flex: 1;
}
</style>
