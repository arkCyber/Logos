<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, watch } from 'vue';

interface Props {
  show: boolean;
  title: string;
  width?: string;
  height?: string;
  maxWidth?: string;
  maxHeight?: string;
  closable?: boolean;
  maskClosable?: boolean;
  closeOnEscape?: boolean;
  zIndex?: number;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'close'): void;
  (e: 'confirm'): void;
  (e: 'cancel'): void;
  (e: 'apply'): void;
}

const props = withDefaults(defineProps<Props>(), {
  width: '500px',
  height: 'auto',
  maxWidth: '90vw',
  maxHeight: '90vh',
  closable: true,
  maskClosable: true,
  closeOnEscape: true,
  zIndex: 1000
});

// Convert zIndex to string for CSS
const zIndexString = computed(() => props.zIndex.toString());

const emit = defineEmits<Emits>();

const dialogRef = ref<HTMLElement | null>(null);
const isAnimating = ref(false);

// Handle escape key
const handleEscape = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && props.closeOnEscape && props.show) {
    close();
  }
};

// Handle mask click
const handleMaskClick = () => {
  if (props.maskClosable) {
    close();
  }
};

// Close dialog
const close = () => {
  isAnimating.value = true;
  setTimeout(() => {
    emit('update:show', false);
    emit('close');
    isAnimating.value = false;
  }, 150);
};

// Confirm dialog
const confirm = () => { // eslint-disable-line @typescript-eslint/no-unused-vars
  emit('confirm');
};

// Cancel dialog
const cancel = () => { // eslint-disable-line @typescript-eslint/no-unused-vars
  emit('cancel');
  close();
};

// Focus trap for accessibility
const focusFirstElement = () => {
  if (!dialogRef.value) {
return;
}
  
  const focusableElements = dialogRef.value.querySelectorAll(
    'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
  );
  
  if (focusableElements.length > 0) {
    (focusableElements[0] as HTMLElement).focus();
  }
};

// Watch show prop to handle animation and focus
watch(() => props.show, (newVal) => {
  if (newVal) {
    // Dialog opening
    setTimeout(() => {
      isAnimating.value = true;
      focusFirstElement();
    }, 10);
  } else {
    // Dialog closing
    isAnimating.value = false;
  }
});

onMounted(() => {
  document.addEventListener('keydown', handleEscape);
  if (props.show) {
    focusFirstElement();
  }
});

onBeforeUnmount(() => {
  document.removeEventListener('keydown', handleEscape);
});
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog-fade">
      <div
        v-if="show"
        class="dialog-mask"
        role="dialog"
        :aria-modal="true"
        aria-labelledby="dialog-title"
        @click="handleMaskClick"
      >
        <div
          ref="dialogRef"
          class="dialog-container"
          :class="{ 'is-animating': isAnimating }"
          :style="{
            width,
            height,
            maxWidth,
            maxHeight,
            zIndex: zIndexString
          }"
          @click.stop
        >
          <!-- Dialog Header -->
          <div v-if="title || closable" class="dialog-header">
            <h3 id="dialog-title" class="dialog-title">{{ title }}</h3>
            <button
              v-if="closable"
              class="dialog-close"
              aria-label="关闭对话框"
              type="button"
              @click="close"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <line x1="18" y1="6" x2="6" y2="18" />
                <line x1="6" y1="6" x2="18" y2="18" />
              </svg>
            </button>
          </div>

          <!-- Dialog Body -->
          <div class="dialog-body">
            <slot></slot>
          </div>

          <!-- Dialog Footer -->
          <div v-if="$slots.footer" class="dialog-footer">
            <slot name="footer"></slot>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.dialog-mask {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: v-bind('zIndex');
  padding: 20px;
}

.dialog-container {
  background: var(--word-bg-page);
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  display: flex;
  flex-direction: column;
  max-height: v-bind('maxHeight');
  overflow: hidden;
  transform: scale(0.95);
  opacity: 0;
  transition: all 0.15s ease-out;
}

.dialog-container.is-animating {
  transform: scale(1);
  opacity: 1;
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--word-border);
  flex-shrink: 0;
}

.dialog-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--word-text-primary);
}

.dialog-close {
  background: transparent;
  border: none;
  padding: 4px;
  border-radius: 4px;
  cursor: pointer;
  color: var(--word-text-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.dialog-close:hover {
  background: var(--word-button-hover);
  color: var(--word-text-primary);
}

.dialog-close:focus {
  outline: 2px solid var(--word-button-border-hover);
  outline-offset: 2px;
}

.dialog-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  min-height: 0;
}

.dialog-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
  padding: 16px 20px;
  border-top: 1px solid var(--word-border);
  flex-shrink: 0;
}

/* Dialog transition */
.dialog-fade-enter-active,
.dialog-fade-leave-active {
  transition: opacity 0.15s ease;
}

.dialog-fade-enter-from,
.dialog-fade-leave-to {
  opacity: 0;
}

/* Dark mode support */
:global(.dark) .dialog-container {
  background: var(--word-bg-canvas);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
}

:global(.dark) .dialog-header,
:global(.dark) .dialog-footer {
  border-color: var(--word-border);
}

:global(.dark) .dialog-title {
  color: var(--word-text-primary);
}
</style>
