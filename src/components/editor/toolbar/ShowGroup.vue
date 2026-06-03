<script setup lang="ts">
interface Emits {
  (e: 'toggle-navigation-pane'): void;
  (e: 'toggle-gridlines'): void;
  (e: 'toggle-format-marks'): void;
  (e: 'toggle-ruler'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="网格线" @click="emit('toggle-gridlines')">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <line x1="3" y1="3" x2="21" y2="3" />
          <line x1="3" y1="7" x2="21" y2="7" />
          <line x1="3" y1="11" x2="21" y2="11" />
          <line x1="3" y1="15" x2="21" y2="15" />
          <line x1="3" y1="19" x2="21" y2="19" />
        </svg>
        <span>网格线</span>
      </button>
      <button class="ribbon-button" title="格式标记" @click="emit('toggle-format-marks')">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
          <polyline points="14 2 14 8 20 8" />
          <line x1="16" y1="13" x2="8" y2="13" />
          <line x1="16" y1="17" x2="8" y2="17" />
          <polyline points="10 9 9 9 8 9" />
        </svg>
        <span>格式标记</span>
      </button>
      <button class="ribbon-button" title="标尺" @click="emit('toggle-ruler')">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <line x1="2" y1="12" x2="22" y2="12" />
          <line x1="2" y1="12" x2="2" y2="16" />
          <line x1="6" y1="12" x2="6" y2="14" />
          <line x1="10" y1="12" x2="10" y2="14" />
          <line x1="14" y1="12" x2="14" y2="14" />
          <line x1="18" y1="12" x2="18" y2="14" />
          <line x1="22" y1="12" x2="22" y2="16" />
        </svg>
        <span>标尺</span>
      </button>
      <button class="ribbon-button" title="导航窗格" @click="emit('toggle-navigation-pane')">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <rect x="3" y="3" width="18" height="18" rx="2" />
          <line x1="9" y1="3" x2="9" y2="21" />
        </svg>
        <span>导航窗格</span>
      </button>
    </div>
    <div class="group-label">显示</div>
  </div>
</template>
