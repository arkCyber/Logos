<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import type { PresentationDocument, Slide } from '../types/presentation'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { PresentationConverter } from '../utils/presentationConverter';

// Props
const props = defineProps<{
  document: PresentationDocument;
}>();

// Emits
const emit = defineEmits<{
  'exit': [];
  'slide-change': [index: number];
}>();

// State
const currentSlideIndex = ref(0);
const presenterMode = ref(false);
const thumbnailsCollapsed = ref(false);
const slidevUrl = ref<string | null>(null);
const mainSlideRef = ref<HTMLIFrameElement | null>(null);
const timer = ref(0);
const timerInterval = ref<number | null>(null);

// Computed
const slides = computed(() => props.document.slides);
const totalSlides = computed(() => slides.value.length);
const currentSlide = computed(() => slides.value[currentSlideIndex.value]);
const nextSlide = computed(() => {
  if (currentSlideIndex.value < totalSlides.value - 1) {
    return slides.value[currentSlideIndex.value + 1];
  }
  return null;
});
const formattedTime = computed(() => {
  const minutes = Math.floor(timer.value / 60);
  const seconds = timer.value % 60;
  return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
});

// Methods
function previousSlide() {
  if (currentSlideIndex.value > 0) {
    currentSlideIndex.value--;
    emit('slide-change', currentSlideIndex.value);
    updateSlidevSlide();
  }
}

function goToNextSlide() {
  if (currentSlideIndex.value < totalSlides.value - 1) {
    currentSlideIndex.value++;
    emit('slide-change', currentSlideIndex.value);
    updateSlidevSlide();
  }
}

function goToSlide(index: number) {
  currentSlideIndex.value = index;
  emit('slide-change', index);
  updateSlidevSlide();
}

function toggleFullscreen() {
  if (!document.fullscreenElement) {
    mainSlideRef.value?.requestFullscreen();
  } else {
    document.exitFullscreen();
  }
}

function togglePresenterMode() {
  presenterMode.value = !presenterMode.value;
}

function exitPreview() {
  emit('exit');
}

function updateSlidevSlide() {
  // Post message to iframe to change slide
  if (mainSlideRef.value?.contentWindow) {
    mainSlideRef.value.contentWindow.postMessage(
      { type: 'slidev:goto', slide: currentSlideIndex.value },
      '*'
    );
  }
}

function onIframeLoad() {
  // Initialize Slidev communication
  if (mainSlideRef.value?.contentWindow) {
    mainSlideRef.value.contentWindow.postMessage(
      { type: 'slidev:init', totalSlides: totalSlides.value },
      '*'
    );
  }
}

function generateSlidevPreview() {
  // Convert document to Slidev markdown
  const markdown = PresentationConverter.toSlidev(props.document);
  
  // Create a blob URL for the markdown content
  const blob = new Blob([markdown], { type: 'text/markdown' });
  const _url = URL.createObjectURL(blob);
  
  // In a real implementation, this would be served by a Slidev dev server
  // For now, we'll use a data URL approach
  slidevUrl.value = `data:text/markdown;charset=utf-8,${encodeURIComponent(markdown)}`;
}

function startTimer() {
  timerInterval.value = window.setInterval(() => {
    timer.value++;
  }, 1000);
}

function stopTimer() {
  if (timerInterval.value) {
    clearInterval(timerInterval.value);
    timerInterval.value = null;
  }
}

// Keyboard navigation
function handleKeydown(event: KeyboardEvent) {
  switch (event.key) {
    case 'ArrowLeft':
    case 'ArrowUp':
    case 'PageUp':
      event.preventDefault();
      previousSlide();
      break;
    case 'ArrowRight':
    case 'ArrowDown':
    case 'PageDown':
    case ' ':
      event.preventDefault();
      goToNextSlide();
      break;
    case 'Home':
      event.preventDefault();
      goToSlide(0);
      break;
    case 'End':
      event.preventDefault();
      goToSlide(totalSlides.value - 1);
      break;
    case 'f':
    case 'F':
      if (!event.ctrlKey && !event.metaKey) {
        event.preventDefault();
        toggleFullscreen();
      }
      break;
    case 'p':
    case 'P':
      if (!event.ctrlKey && !event.metaKey) {
        event.preventDefault();
        togglePresenterMode();
      }
      break;
    case 'Escape':
      event.preventDefault();
      exitPreview();
      break;
  }
}

// Lifecycle
onMounted(() => {
  generateSlidevPreview();
  startTimer();
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  stopTimer();
  window.removeEventListener('keydown', handleKeydown);
  if (slidevUrl.value) {
    URL.revokeObjectURL(slidevUrl.value);
  }
});

watch(() => props.document, () => {
  generateSlidevPreview();
}, { deep: true });
</script>

<template>
  <div class="slidev-integration">
    <div class="slidev-header">
      <div class="slidev-controls">
        <button :disabled="currentSlideIndex === 0" class="control-btn" @click="previousSlide">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="15 18 9 12 15 6"></polyline>
          </svg>
        </button>
        <span class="slide-counter">{{ currentSlideIndex + 1 }} / {{ totalSlides }}</span>
        <button :disabled="currentSlideIndex === totalSlides - 1" class="control-btn" @click="goToNextSlide">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="9 18 15 12 9 6"></polyline>
          </svg>
        </button>
      </div>
      <div class="slidev-actions">
        <button class="control-btn" title="Fullscreen" @click="toggleFullscreen">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path>
          </svg>
        </button>
        <button class="control-btn" :class="{ active: presenterMode }" title="Presenter Mode" @click="togglePresenterMode">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
            <line x1="8" y1="21" x2="16" y2="21"></line>
            <line x1="12" y1="17" x2="12" y2="21"></line>
          </svg>
        </button>
        <button class="control-btn" title="Exit Preview" @click="exitPreview">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>
    </div>

    <div class="slidev-content" :class="{ 'presenter-mode': presenterMode }">
      <!-- Main slide view -->
      <div ref="mainSlideRef" class="slidev-main">
        <iframe
          v-if="slidevUrl"
          :src="slidevUrl"
          class="slidev-iframe"
          @load="onIframeLoad"
        ></iframe>
        <div v-else class="slidev-placeholder">
          <div class="placeholder-content">
            <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
              <line x1="3" y1="9" x2="21" y2="9"></line>
              <line x1="9" y1="21" x2="9" y2="9"></line>
            </svg>
            <p>Generating preview...</p>
          </div>
        </div>
      </div>

      <!-- Presenter view (notes, next slide) -->
      <div v-if="presenterMode" class="slidev-presenter">
        <div class="presenter-section">
          <h4>Current Slide Notes</h4>
          <div class="presenter-notes">
            {{ currentSlide?.notes || 'No notes for this slide' }}
          </div>
        </div>
        <div class="presenter-section">
          <h4>Next Slide</h4>
          <div class="presenter-next">
            <div v-if="nextSlide" class="next-slide-preview">
              <div v-if="nextSlide.title" class="next-slide-title">{{ nextSlide.title }}</div>
              <div class="next-slide-elements">{{ nextSlide.elements.length }} elements</div>
            </div>
            <div v-else class="next-slide-empty">No next slide</div>
          </div>
        </div>
        <div class="presenter-section">
          <h4>Timer</h4>
          <div class="presenter-timer">{{ formattedTime }}</div>
        </div>
      </div>
    </div>

    <!-- Slide thumbnails for quick navigation -->
    <div class="slidev-thumbnails" :class="{ collapsed: thumbnailsCollapsed }">
      <button class="thumbnail-toggle" @click="thumbnailsCollapsed = !thumbnailsCollapsed">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline :points="thumbnailsCollapsed ? '9 18 15 12 9 6' : '15 18 9 12 15 6'"></polyline>
        </svg>
      </button>
      <div v-if="!thumbnailsCollapsed" class="thumbnail-list">
        <div
          v-for="(slide, index) in slides"
          :key="slide.id"
          :class="['thumbnail-item', { active: currentSlideIndex === index }]"
          @click="goToSlide(index)"
        >
          <div class="thumbnail-number">{{ index + 1 }}</div>
          <div class="thumbnail-label">{{ slide.title || `Slide ${index + 1}` }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.slidev-integration {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #1a1a1a;
  color: white;
}

.slidev-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #2a2a2a;
  border-bottom: 1px solid #3a3a3a;
}

.slidev-controls {
  display: flex;
  align-items: center;
  gap: 12px;
}

.slide-counter {
  font-size: 14px;
  color: #aaa;
  min-width: 60px;
  text-align: center;
}

.slidev-actions {
  display: flex;
  gap: 8px;
}

.control-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  padding: 0;
  background: #3a3a3a;
  border: 1px solid #4a4a4a;
  border-radius: 6px;
  color: white;
  cursor: pointer;
  transition: all 0.2s;
}

.control-btn:hover:not(:disabled) {
  background: #4a4a4a;
}

.control-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.control-btn.active {
  background: #007bff;
  border-color: #007bff;
}

.slidev-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.slidev-content.presenter-mode {
  gap: 16px;
  padding: 16px;
}

.slidev-main {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #000;
}

.slidev-iframe {
  width: 100%;
  height: 100%;
  border: none;
}

.slidev-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
}

.placeholder-content {
  text-align: center;
  color: #666;
}

.placeholder-content svg {
  margin-bottom: 16px;
}

.placeholder-content p {
  font-size: 16px;
}

.slidev-presenter {
  width: 320px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  background: #2a2a2a;
  border-radius: 8px;
  padding: 16px;
}

.presenter-section h4 {
  margin: 0 0 8px 0;
  font-size: 12px;
  text-transform: uppercase;
  color: #888;
  letter-spacing: 0.5px;
}

.presenter-notes {
  flex: 1;
  padding: 12px;
  background: #1a1a1a;
  border-radius: 4px;
  font-size: 13px;
  line-height: 1.5;
  color: #ccc;
  overflow-y: auto;
}

.presenter-next {
  padding: 12px;
  background: #1a1a1a;
  border-radius: 4px;
}

.next-slide-preview {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.next-slide-title {
  font-weight: bold;
  font-size: 14px;
}

.next-slide-elements {
  font-size: 12px;
  color: #888;
}

.next-slide-empty {
  color: #666;
  font-size: 13px;
}

.presenter-timer {
  font-size: 32px;
  font-weight: bold;
  text-align: center;
  font-variant-numeric: tabular-nums;
}

.slidev-thumbnails {
  position: fixed;
  right: 0;
  top: 60px;
  bottom: 0;
  width: 200px;
  background: #2a2a2a;
  border-left: 1px solid #3a3a3a;
  transition: transform 0.3s ease;
  z-index: 100;
}

.slidev-thumbnails.collapsed {
  transform: translateX(100%);
}

.thumbnail-toggle {
  position: absolute;
  left: -32px;
  top: 12px;
  width: 32px;
  height: 32px;
  background: #2a2a2a;
  border: 1px solid #3a3a3a;
  border-radius: 4px 0 0 4px;
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.thumbnail-list {
  padding: 12px;
  overflow-y: auto;
  height: 100%;
}

.thumbnail-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s;
  margin-bottom: 4px;
}

.thumbnail-item:hover {
  background: #3a3a3a;
}

.thumbnail-item.active {
  background: #007bff;
}

.thumbnail-number {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 50%;
  font-size: 12px;
}

.thumbnail-label {
  flex: 1;
  font-size: 13px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
