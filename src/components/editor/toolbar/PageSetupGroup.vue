<script setup lang="ts">
interface Emits {
  (e: 'show-page-setup'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="页面设置" @click="emit('show-page-setup')">
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
          <path d="M6 9V2h12v7" />
          <path d="M6 18V9h12v9" />
          <path d="M6 2h12v20H6z" />
        </svg>
        <span>页面设置</span>
      </button>
    </div>
    <div class="group-label">页面设置</div>
  </div>
</template>
