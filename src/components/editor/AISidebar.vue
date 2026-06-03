<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import WallpaperSelector from '../WallpaperSelector.vue';

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: Date;
}

interface Emits {
  (e: 'close'): void;
}

const emit = defineEmits<Emits>();

const messages = ref<Message[]>([
  {
    id: '1',
    role: 'assistant',
    content: '你好！我是AI助手，有什么可以帮助你的吗？',
    timestamp: new Date()
  }
]);

const inputMessage = ref('');
const isTyping = ref(false);
const selectedWallpaper = ref<string | null>('/sascha-roder-zb3r_kTcVbU-unsplash.jpg');

// 墙纸样式计算
const wallpaperStyle = computed(() => {
  if (selectedWallpaper.value) {
    const isDataUrl = selectedWallpaper.value.startsWith('data:');
    const imagePath = isDataUrl ? selectedWallpaper.value : selectedWallpaper.value;
    return {
      backgroundImage: `url('${imagePath}')`,
      backgroundSize: 'cover',
      backgroundPosition: 'center',
      backgroundRepeat: 'no-repeat',
      backgroundAttachment: 'fixed'
    };
  }
  return {
    backgroundImage: 'none'
  };
});

const sendMessage = () => {
  if (!inputMessage.value.trim()) {
    return;
  }

  // Add user message
  messages.value.push({
    id: Date.now().toString(),
    role: 'user',
    content: inputMessage.value,
    timestamp: new Date()
  });

  const userMessage = inputMessage.value;
  inputMessage.value = '';
  isTyping.value = true;

  // Simulate AI response
  setTimeout(() => {
    isTyping.value = false;
    messages.value.push({
      id: (Date.now() + 1).toString(),
      role: 'assistant',
      content: `我收到了你的消息："${userMessage}"。这是一个模拟的AI响应，实际功能需要连接到AI服务。`,
      timestamp: new Date()
    });
  }, 1000);
};

const formatTime = (date: Date) => {
  return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
};

const closeSidebar = () => {
  emit('close');
};

const handleWallpaperError = (message: string) => {
  console.error('Wallpaper error:', message);
};

const loadSelectedWallpaper = () => {
  try {
    const saved = localStorage.getItem('ai-sidebar-wallpaper');
    if (saved) {
      selectedWallpaper.value = JSON.parse(saved);
    }
  } catch (e) {
    console.error('Failed to load selected wallpaper:', e);
  }
};

const saveSelectedWallpaper = (wallpaper: string | null) => {
  try {
    localStorage.setItem('ai-sidebar-wallpaper', JSON.stringify(wallpaper));
  } catch (e) {
    console.error('Failed to save selected wallpaper:', e);
  }
};

const selectWallpaper = (wallpaper: string | null) => {
  selectedWallpaper.value = wallpaper;
  saveSelectedWallpaper(wallpaper);
};

onMounted(() => {
  loadSelectedWallpaper();
});
</script>

<template>
  <div class="ai-sidebar editor-side-panel editor-side-panel--right" :style="wallpaperStyle">
    <div class="ai-sidebar-header">
      <div class="ai-header-title">
        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z"/>
        </svg>
        <span>AI智能体助手</span>
      </div>
      <div class="ai-header-actions">
        <WallpaperSelector
          @select="selectWallpaper"
          @error="handleWallpaperError"
        />
        <button class="close-button" title="关闭" @click="closeSidebar">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>
    </div>

    <div ref="messagesContainer" class="ai-sidebar-messages">
      <div
        v-for="message in messages"
        :key="message.id"
        :class="['message', message.role]"
      >
        <div class="message-avatar">
          <svg v-if="message.role === 'assistant'" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z"/>
          </svg>
          <svg v-else xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
            <circle cx="12" cy="7" r="4"/>
          </svg>
        </div>
        <div class="message-body">
          <div class="message-content">{{ message.content }}</div>
          <div class="message-time">{{ formatTime(message.timestamp) }}</div>
        </div>
      </div>
      <div v-if="isTyping" class="message assistant typing">
        <div class="message-avatar">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z"/>
          </svg>
        </div>
        <div class="message-body">
          <div class="message-content">正在输入...</div>
        </div>
      </div>
    </div>

    <div class="ai-sidebar-input">
      <textarea
        v-model="inputMessage"
        placeholder="输入消息..."
        rows="2"
        @keydown.enter.prevent="sendMessage"
      ></textarea>
      <button class="send-button" :disabled="!inputMessage.trim()" title="发送" @click="sendMessage">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="22" y1="2" x2="11" y2="13"/>
          <polygon points="22 2 15 22 11 13 2 9 22 2"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.ai-sidebar {
  width: var(--editor-sidebar-ai-width, 360px);
  min-width: var(--editor-sidebar-ai-width, 360px);
  max-width: var(--editor-sidebar-ai-width, 360px);
  height: 100%;
  background: #ffffff;
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow: hidden;
  position: relative;
}

.ai-sidebar::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.85);
  pointer-events: none;
  z-index: 0;
}

.ai-sidebar-header {
  position: relative;
  z-index: 1;
  padding: 12px 16px;
  border-bottom: 1px solid rgba(229, 231, 235, 0.8);
  background: rgba(249, 250, 251, 0.9);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
}

.ai-header-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  font-weight: 600;
  color: #374151;
}

.ai-header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.close-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  color: #6b7280;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.close-button:hover {
  background: #e5e7eb;
  color: #374151;
}

.ai-sidebar-messages {
  position: relative;
  z-index: 1;
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(4px);
}

.ai-sidebar-messages::-webkit-scrollbar {
  width: 6px;
}

.ai-sidebar-messages::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

.ai-sidebar-messages::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 3px;
}

.ai-sidebar-messages::-webkit-scrollbar-thumb:hover {
  background: #a1a1a1;
}

.message {
  display: flex;
  gap: 10px;
  max-width: 100%;
}

.message.user {
  flex-direction: row-reverse;
}

.message-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: #f3f4f6;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  color: #6b7280;
}

.message.user .message-avatar {
  background: var(--word-accent, #0078d4);
  color: white;
}

.message-body {
  display: flex;
  flex-direction: column;
  gap: 4px;
  max-width: calc(100% - 42px);
}

.message-content {
  padding: 10px 14px;
  border-radius: 12px;
  font-size: 13px;
  line-height: 1.5;
  word-wrap: break-word;
  white-space: pre-wrap;
}

.message.user .message-content {
  background: var(--word-accent, #0078d4);
  color: white;
  border-bottom-right-radius: 4px;
}

.message.assistant .message-content {
  background: #f3f4f6;
  color: #374151;
  border-bottom-left-radius: 4px;
}

.message.typing .message-content {
  color: #6b7280;
  font-style: italic;
}

.message-time {
  font-size: 11px;
  color: #9ca3af;
  padding: 0 4px;
}

.message.user .message-time {
  text-align: right;
}

.ai-sidebar-input {
  position: relative;
  z-index: 1;
  padding: 12px 16px;
  border-top: 1px solid rgba(229, 231, 235, 0.8);
  display: flex;
  gap: 8px;
  background: rgba(249, 250, 251, 0.9);
  backdrop-filter: blur(8px);
  flex-shrink: 0;
}

.ai-sidebar-input textarea {
  flex: 1;
  padding: 10px 12px;
  border: 1px solid #d1d5db;
  border-radius: 8px;
  font-size: 13px;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  resize: none;
  outline: none;
  background: #ffffff;
  color: #374151;
  transition: border-color 0.15s ease;
}

.ai-sidebar-input textarea:focus {
  border-color: var(--word-accent, #0078d4);
}

.ai-sidebar-input textarea::placeholder {
  color: #9ca3af;
}

.send-button {
  width: 40px;
  height: 40px;
  background: var(--word-accent, #0078d4);
  border: none;
  border-radius: 8px;
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.15s ease;
  flex-shrink: 0;
}

.send-button:hover:not(:disabled) {
  background: #005a9e;
}

.send-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
