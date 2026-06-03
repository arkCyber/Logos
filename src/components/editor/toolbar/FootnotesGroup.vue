<script setup lang="ts">
interface Emits {
  (e: 'insert-footnote'): void;
  (e: 'insert-endnote'): void;
}

const emit = defineEmits<Emits>();
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <button class="ribbon-button" title="插入脚注" @click="emit('insert-footnote')">
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
          <path d="M12 19V5" />
          <path d="M12 19a3 3 0 0 1-3-3" />
          <path d="M12 19a3 3 0 0 0 3-3" />
        </svg>
        <span>脚注</span>
      </button>
      <button class="ribbon-button" title="插入尾注" @click="emit('insert-endnote')">
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
          <path d="M12 5v14" />
          <path d="M12 5a3 3 0 0 0-3 3" />
          <path d="M12 5a3 3 0 0 1 3 3" />
        </svg>
        <span>尾注</span>
      </button>
    </div>
    <div class="group-label">脚注</div>
  </div>
</template>
