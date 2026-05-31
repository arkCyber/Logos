<script setup lang="ts">
interface Props {
  wordCount: number;
  charCount: number;
  currentPage: number;
  totalPages: number;
  zoomLevel: number;
  isDarkMode: boolean;
  viewMode: 'focus' | 'read' | 'print' | 'web';
}

interface Emits {
  (e: 'zoom-in'): void;
  (e: 'zoom-out'): void;
  (e: 'zoom-change', value: number): void;
  (e: 'toggle-theme'): void;
  (e: 'view-mode-change', value: 'focus' | 'read' | 'print' | 'web'): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();
</script>

<template>
  <div class="status-bar">
    <div class="status-left">
      <div class="status-item" title="当前页码">页 {{ currentPage }} / {{ totalPages }}</div>
      <div class="status-separator">|</div>
      <div class="status-item" title="字数统计">{{ wordCount }} 字</div>
      <div class="status-separator">|</div>
      <div class="status-item" title="字符统计">{{ charCount }} 字符</div>
    </div>
    <div class="status-right">
      <!-- Word-style View Mode Toggles -->
      <div class="view-modes">
        <button 
          class="view-mode-btn" 
          :class="{ active: viewMode === 'focus' }" 
          title="专注模式 (Focus)" 
          aria-label="专注模式"
          @click="emit('view-mode-change', 'focus')"
        >
          <!-- Lucide Focus Icon -->
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-focus">
            <circle cx="12" cy="12" r="3"/>
            <path d="M3 7V5a2 2 0 0 1 2-2h2"/>
            <path d="M17 3h2a2 2 0 0 1 2 2v2"/>
            <path d="M21 17v2a2 2 0 0 1-2 2h-2"/>
            <path d="M7 21H5a2 2 0 0 1-2-2v-2"/>
          </svg>
          <span class="focus-text">Focus</span>
        </button>
        <button 
          class="view-mode-btn" 
          :class="{ active: viewMode === 'read' }" 
          title="阅读视图 (Read Mode)" 
          aria-label="阅读视图"
          @click="emit('view-mode-change', 'read')"
        >
          <!-- Lucide BookOpen Icon -->
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-book-open">
            <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/>
            <path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/>
          </svg>
        </button>
        <button 
          class="view-mode-btn" 
          :class="{ active: viewMode === 'print' }" 
          title="页面视图 (Print Layout)" 
          aria-label="页面视图"
          @click="emit('view-mode-change', 'print')"
        >
          <!-- Lucide FileText Icon -->
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-file-text">
            <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/>
            <path d="M14 2v4a2 2 0 0 0 2 2h4"/>
            <path d="M10 9H8"/>
            <path d="M16 13H8"/>
            <path d="M16 17H8"/>
          </svg>
        </button>
        <button 
          class="view-mode-btn" 
          :class="{ active: viewMode === 'web' }" 
          title="Web 版式 (Web Layout)" 
          aria-label="Web 版式"
          @click="emit('view-mode-change', 'web')"
        >
          <!-- Lucide Globe Icon -->
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-globe">
            <circle cx="12" cy="12" r="10"/>
            <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/>
            <path d="M2 12h20"/>
          </svg>
        </button>
      </div>

      <div class="status-separator">|</div>

      <div class="zoom-controls">
        <button class="zoom-out" title="缩小" aria-label="缩小" @click="emit('zoom-out')">
          <!-- Lucide Minus Icon -->
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-minus">
            <line x1="5" x2="19" y1="12" y2="12" />
          </svg>
        </button>
        <input
          type="range"
          class="zoom-slider"
          :value="zoomLevel"
          min="25"
          max="400"
          step="25"
          aria-label="缩放级别"
          @input="(e) => emit('zoom-change', Number((e.target as HTMLInputElement).value))"
        />
        <button class="zoom-in" title="放大" aria-label="放大" @click="emit('zoom-in')">
          <!-- Lucide Plus Icon -->
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-plus">
            <path d="M5 12h14" />
            <path d="M12 5v14" />
          </svg>
        </button>
        <span class="zoom-percent">{{ zoomLevel }}%</span>
      </div>
      <div class="status-separator">|</div>
      <button class="status-item theme-toggle" title="切换主题" aria-label="切换主题" @click="emit('toggle-theme')">
        <!-- Lucide Sun and Moon Icons -->
        <svg v-if="!isDarkMode" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-sun">
          <circle cx="12" cy="12" r="4"/>
          <path d="M12 2v2"/>
          <path d="M12 20v2"/>
          <path d="M4.93 4.93l1.41 1.41"/>
          <path d="M17.66 17.66l1.41 1.41"/>
          <path d="M2 12h2"/>
          <path d="M20 12h2"/>
          <path d="M6.34 17.66l-1.41 1.41"/>
          <path d="M19.07 4.93l-1.41 1.41"/>
        </svg>
        <svg v-else xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-moon">
          <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.status-bar {
  height: 24px;
  background: var(--word-statusbar-bg);
  border-top: 1px solid var(--word-statusbar-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 12px;
  font-family: var(--word-font-ui);
  font-size: 11px;
  color: var(--word-statusbar-text);
  flex-shrink: 0;
  user-select: none;
}

.status-left,
.status-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-item {
  padding: 2px 8px;
  height: 20px;
  display: flex;
  align-items: center;
  cursor: pointer;
  border-radius: 2px;
  transition: background 0.15s ease;
  color: var(--word-statusbar-text);
}

.status-item:hover {
  background: var(--word-statusbar-hover);
}

.status-item.theme-toggle {
  padding: 2px 6px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.status-item.theme-toggle svg {
  transition: transform 0.2s ease;
}

.status-item.theme-toggle:hover svg {
  transform: scale(1.1);
}

.status-separator {
  color: var(--word-divider);
  user-select: none;
  font-size: 10px;
}

.zoom-controls {
  display: flex;
  align-items: center;
  gap: 6px;
}

.zoom-out,
.zoom-in {
  width: 20px;
  height: 20px;
  padding: 0;
  background: var(--word-button-bg);
  border: 1px solid var(--word-button-border);
  border-radius: 2px;
  color: var(--word-text-primary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.15s ease;
}

.zoom-out:hover,
.zoom-in:hover {
  background: var(--word-button-hover);
  border-color: var(--word-button-border-hover);
  transform: scale(1.05);
}

.zoom-out:active,
.zoom-in:active {
  background: var(--word-button-active);
  transform: scale(0.95);
}

.zoom-slider {
  width: 80px;
  height: 4px;
  -webkit-appearance: none;
  appearance: none;
  background: var(--word-divider);
  border-radius: 2px;
  outline: none;
  cursor: pointer;
  transition: background 0.15s ease;
}

.zoom-slider:hover {
  background: var(--word-text-tertiary);
}

.zoom-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 12px;
  height: 12px;
  background: var(--word-accent);
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.15s ease;
}

.zoom-slider::-webkit-slider-thumb:hover {
  background: var(--word-accent-hover);
  transform: scale(1.15);
}

.zoom-slider::-moz-range-thumb {
  width: 12px;
  height: 12px;
  background: var(--word-accent);
  border: none;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.15s ease;
}

.zoom-slider::-moz-range-thumb:hover {
  background: var(--word-accent-hover);
  transform: scale(1.15);
}

.zoom-percent {
  min-width: 40px;
  text-align: center;
  font-weight: var(--word-font-weight-medium);
  color: var(--word-text-secondary);
}

/* Word-style View Modes Toggles */
.view-modes {
  display: flex;
  align-items: center;
  height: 20px;
  border-radius: 2px;
  overflow: hidden;
}

.view-mode-btn {
  background: none;
  border: none;
  height: 20px;
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 0 8px;
  color: var(--word-text-secondary);
  cursor: pointer;
  transition: all 0.15s ease;
  font-family: var(--word-font-ui);
  font-size: 11px;
}

.view-mode-btn:hover {
  background: rgba(0, 0, 0, 0.05);
  color: var(--word-text-primary);
}

.dark .view-mode-btn:hover {
  background: rgba(255, 255, 255, 0.1);
}

.view-mode-btn.active {
  background: rgba(0, 0, 0, 0.1) !important;
  color: var(--word-text-primary) !important;
}

.dark .view-mode-btn.active {
  background: rgba(255, 255, 255, 0.15) !important;
}

.focus-text {
  font-weight: var(--word-font-weight-medium);
  letter-spacing: -0.2px;
}
</style>
