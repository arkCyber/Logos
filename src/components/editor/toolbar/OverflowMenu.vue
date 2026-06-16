<script setup lang="ts">

interface Props {
  show: boolean;
  x: number;
  y: number;
}

interface Emits {
  (e: 'update:show', value: boolean): void;
  (e: 'insert-video'): void;
  (e: 'insert-audio'): void;
  (e: 'rehearse-timings'): void;
  (e: 'record-slide-show'): void;
  (e: 'check-spelling'): void;
  (e: 'add-comment'): void;
  (e: 'translate-text'): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

const closeMenu = () => {
  emit('update:show', false);
};

const insertVideo = () => {
  emit('insert-video');
  closeMenu();
};

const insertAudio = () => {
  emit('insert-audio');
  closeMenu();
};

const rehearseTimings = () => {
  emit('rehearse-timings');
  closeMenu();
};

const recordSlideShow = () => {
  emit('record-slide-show');
  closeMenu();
};

const checkSpelling = () => {
  emit('check-spelling');
  closeMenu();
};

const addComment = () => {
  emit('add-comment');
  closeMenu();
};

const translateText = () => {
  emit('translate-text');
  closeMenu();
};
</script>

<template>
  <div 
    v-if="show" 
    class="overflow-menu-dropdown" 
    :style="{ left: x + 'px', top: y + 'px' }" 
    @click="closeMenu"
  >
    <div class="overflow-menu-section">
      <div class="overflow-menu-title">媒体</div>
      <button class="overflow-menu-item" @click="insertVideo">
        <span>🎬</span>
        <span>插入视频</span>
      </button>
      <button class="overflow-menu-item" @click="insertAudio">
        <span>🎵</span>
        <span>插入音频</span>
      </button>
    </div>
    <div class="overflow-menu-section">
      <div class="overflow-menu-title">演示</div>
      <button class="overflow-menu-item" @click="rehearseTimings">
        <span>⏱</span>
        <span>排练计时</span>
      </button>
      <button class="overflow-menu-item" @click="recordSlideShow">
        <span>🎥</span>
        <span>录制幻灯片</span>
      </button>
    </div>
    <div class="overflow-menu-section">
      <div class="overflow-menu-title">审阅</div>
      <button class="overflow-menu-item" @click="checkSpelling">
        <span>✓</span>
        <span>拼写检查</span>
      </button>
      <button class="overflow-menu-item" @click="addComment">
        <span>💬</span>
        <span>添加批注</span>
      </button>
      <button class="overflow-menu-item" @click="translateText">
        <span>🌐</span>
        <span>翻译</span>
      </button>
    </div>
  </div>
</template>

<style scoped>
/* Styles will be inherited from parent component's global styles */
</style>
