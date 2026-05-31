<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import { usePresenceManager } from '../services/presenceManager';
import { getCursorTracker, type CursorInfo } from '../services/cursorTracker';
import { getCollaborationService, ConnectionStatus } from '../services/collaborationService';
import { logger, LogCategory } from '../utils/logger';

// Props
interface Props {
  documentId?: string;
  userId?: string;
  userName?: string;
  isActive?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  isActive: false
});

// Emits
const emit = defineEmits<{
  conflictDetected: [localOp: unknown, remoteOp: unknown];
  conflictResolved: [];
}>();

// 状态
const showOnlineUsers = ref(false);
const hasConflict = ref(false);
const showConnectionStatus = ref(false);
const connectionStatus = ref<ConnectionStatus>(ConnectionStatus.Disconnected);

// 协作服务
const collaborationService = getCollaborationService();
const { onlineUsers, onlineUserCount } = usePresenceManager(props.userId);
const cursorTracker = getCursorTracker();

// 当前用户 ID
const currentUserId = computed(() => props.userId);

// 状态文本
const statusText = computed(() => {
  switch (connectionStatus.value) {
    case ConnectionStatus.Connected:
      return '协作中';
    case ConnectionStatus.Connecting:
      return '连接中...';
    case ConnectionStatus.Reconnecting:
      return '重连中...';
    case ConnectionStatus.Error:
      return '连接错误';
    default:
      return '未连接';
  }
});

// 状态图标
const statusIcon = computed(() => {
  switch (connectionStatus.value) {
    case ConnectionStatus.Connected:
      return '🟢';
    case ConnectionStatus.Connecting:
      return '🟡';
    case ConnectionStatus.Reconnecting:
      return '🟠';
    case ConnectionStatus.Error:
      return '🔴';
    default:
      return '⚪';
  }
});

// 状态类
const statusClass = computed(() => {
  return `status-${connectionStatus.value}`;
});

// 连接状态类
const connectionStatusClass = computed(() => {
  return `connection-${connectionStatus.value}`;
});

// 连接状态文本
const connectionStatusText = computed(() => {
  switch (connectionStatus.value) {
    case ConnectionStatus.Connected:
      return '协作连接已建立';
    case ConnectionStatus.Connecting:
      return '正在连接协作服务器...';
    case ConnectionStatus.Reconnecting:
      return '正在重新连接...';
    case ConnectionStatus.Error:
      return '协作连接失败';
    default:
      return '协作连接已断开';
  }
});

// 远程光标
const remoteCursors = computed(() => {
  const cursors = cursorTracker.getRemoteCursors();
  return Array.from(cursors.values());
});

// 光标样式
const cursorStyle = (cursor: CursorInfo) => {
  return {
    left: `${cursor.position}px`,
    top: '0px',
    backgroundColor: cursor.color
  };
};

// 获取用户颜色
const getUserColor = (userId: string): string => {
  const colors = [
    '#ef4444', '#f59e0b', '#10b981', '#3b82f6', '#8b5cf6',
    '#ec4899', '#06b6d4', '#84cc16', '#f97316', '#6366f1'
  ];
  const hash = userId.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
  return colors[Math.abs(hash) % colors.length];
};

// 获取用户首字母
const getUserInitials = (userName: string): string => {
  return userName
    .split(' ')
    .map((name) => name[0])
    .join('')
    .toUpperCase()
    .slice(0, 2);
};

// 切换用户面板
const toggleUsersPanel = () => {
  showOnlineUsers.value = !showOnlineUsers.value;
};

// 解决冲突
const resolveConflict = () => {
  hasConflict.value = false;
  emit('conflictResolved');
};

// 监听协作服务状态
const setupCollaborationService = () => {
  collaborationService.onConnected(() => {
    connectionStatus.value = ConnectionStatus.Connected;
    showConnectionStatus.value = true;
    setTimeout(() => {
      showConnectionStatus.value = false;
    }, 3000);
  });

  collaborationService.onDisconnected(() => {
    connectionStatus.value = ConnectionStatus.Disconnected;
    showConnectionStatus.value = true;
    setTimeout(() => {
      showConnectionStatus.value = false;
    }, 3000);
  });

  collaborationService.onError((error) => {
    connectionStatus.value = ConnectionStatus.Error;
    showConnectionStatus.value = true;
    logger.error('Collaboration error', error as Error, LogCategory.SYSTEM);
    setTimeout(() => {
      showConnectionStatus.value = false;
    }, 5000);
  });
};

// 监听光标更新
const setupCursorTracker = () => {
  cursorTracker.onRemoteCursorUpdate((_cursor) => {
    // 光标更新时自动刷新
  });

  cursorTracker.onRemoteCursorRemove((_userId) => {
    // 光标移除时自动刷新
  });
};

// 定期清理过期光标
const startCursorCleanup = () => {
  setInterval(() => {
    cursorTracker.cleanupExpiredCursors();
  }, 30000); // 每 30 秒清理一次
};

// 连接到协作服务器
const connectToCollaboration = async () => {
  if (!props.documentId || !props.userId || !props.userName) {
    return;
  }

  try {
    connectionStatus.value = ConnectionStatus.Connecting;
    await collaborationService.connect(props.documentId, props.userId, props.userName);
  } catch (error) {
    logger.error('Failed to connect to collaboration', error as Error, LogCategory.SYSTEM);
    connectionStatus.value = ConnectionStatus.Error;
  }
};

// 断开协作连接
const disconnectFromCollaboration = async () => {
  try {
    await collaborationService.disconnect();
  } catch (error) {
    logger.error('Failed to disconnect from collaboration', error as Error, LogCategory.SYSTEM);
  }
};

// 监听 props 变化
watch(
  () => props.isActive,
  async (isActive) => {
    if (isActive) {
      await connectToCollaboration();
    } else {
      await disconnectFromCollaboration();
    }
  },
  { immediate: true }
);

// 生命周期
onMounted(() => {
  setupCollaborationService();
  setupCursorTracker();
  startCursorCleanup();
});

onUnmounted(() => {
  disconnectFromCollaboration();
});
</script>

<template>
  <div class="collaboration-ui" :class="{ 'collaboration-active': isActive }">
    <!-- 在线用户列表 -->
    <div v-if="showOnlineUsers" class="online-users-panel">
      <div class="panel-header">
        <span class="panel-title">在线用户 ({{ onlineUserCount }})</span>
        <button class="close-button" @click="toggleUsersPanel">×</button>
      </div>
      <div class="user-list">
        <div
          v-for="user in onlineUsers"
          :key="user.user_id"
          class="user-item"
          :class="{ 'current-user': user.user_id === currentUserId }"
        >
          <div class="user-avatar" :style="{ backgroundColor: getUserColor(user.user_id) }">
            {{ getUserInitials(user.user_name) }}
          </div>
          <div class="user-info">
            <span class="user-name">{{ user.user_name }}</span>
            <span class="user-status" :class="{ 'online': user.is_online }">
              {{ user.is_online ? '在线' : '离线' }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- 协作状态指示器 -->
    <div class="collaboration-status-bar">
      <button
        class="status-button"
        :class="statusClass"
        :title="statusText"
        @click="toggleUsersPanel"
      >
        <span class="status-icon">{{ statusIcon }}</span>
        <span class="status-text">{{ statusText }}</span>
        <span class="user-count">{{ onlineUserCount }}</span>
      </button>
    </div>

    <!-- 远程光标渲染 -->
    <teleport to="body">
      <div
        v-for="cursor in remoteCursors"
        :key="cursor.userId"
        class="remote-cursor"
        :style="cursorStyle(cursor)"
      >
        <div class="cursor-pointer"></div>
        <span class="cursor-label">{{ cursor.userName }}</span>
      </div>
    </teleport>

    <!-- 冲突警告 -->
    <transition name="fade">
      <div v-if="hasConflict" class="conflict-warning">
        <div class="warning-content">
          <span class="warning-icon">⚠️</span>
          <span class="warning-text">检测到编辑冲突</span>
          <button class="resolve-button" @click="resolveConflict">解决</button>
        </div>
      </div>
    </transition>

    <!-- 连接状态提示 -->
    <transition name="fade">
      <div v-if="showConnectionStatus" class="connection-status" :class="connectionStatusClass">
        {{ connectionStatusText }}
      </div>
    </transition>
  </div>
</template>

<style scoped>
.collaboration-ui {
  position: relative;
  z-index: 1000;
}

.collaboration-active {
  z-index: 1001;
}

/* 在线用户面板 */
.online-users-panel {
  position: fixed;
  top: 60px;
  right: 20px;
  width: 280px;
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 1002;
  overflow: hidden;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #f8fafc;
  border-bottom: 1px solid #e2e8f0;
}

.panel-title {
  font-size: 14px;
  font-weight: 600;
  color: #1e293b;
}

.close-button {
  background: none;
  border: none;
  font-size: 20px;
  color: #64748b;
  cursor: pointer;
  padding: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
}

.close-button:hover {
  background: #e2e8f0;
}

.user-list {
  max-height: 300px;
  overflow-y: auto;
}

.user-item {
  display: flex;
  align-items: center;
  padding: 10px 16px;
  border-bottom: 1px solid #f1f5f9;
  transition: background 0.2s;
}

.user-item:hover {
  background: #f8fafc;
}

.user-item.current-user {
  background: #eff6ff;
}

.user-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 12px;
  font-weight: 600;
  margin-right: 12px;
}

.user-info {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.user-name {
  font-size: 14px;
  font-weight: 500;
  color: #1e293b;
}

.user-status {
  font-size: 12px;
  color: #64748b;
}

.user-status.online {
  color: #10b981;
}

/* 协作状态栏 */
.collaboration-status-bar {
  position: fixed;
  bottom: 20px;
  right: 20px;
  z-index: 1001;
}

.status-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 20px;
  font-size: 14px;
  cursor: pointer;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transition: all 0.2s;
}

.status-button:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transform: translateY(-1px);
}

.status-icon {
  font-size: 16px;
}

.status-text {
  color: #1e293b;
}

.user-count {
  background: #3b82f6;
  color: white;
  font-size: 12px;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: 10px;
}

.status-connected {
  border-color: #10b981;
}

.status-connecting {
  border-color: #f59e0b;
}

.status-reconnecting {
  border-color: #f97316;
}

.status-error {
  border-color: #ef4444;
}

.status-disconnected {
  border-color: #94a3b8;
}

/* 远程光标 */
.remote-cursor {
  position: absolute;
  pointer-events: none;
  z-index: 1000;
  transition: left 0.1s ease-out;
}

.cursor-pointer {
  width: 2px;
  height: 20px;
  background: currentColor;
  animation: blink 1s infinite;
}

@keyframes blink {
  0%, 50% {
    opacity: 1;
  }
  51%, 100% {
    opacity: 0;
  }
}

.cursor-label {
  position: absolute;
  top: 20px;
  left: 0;
  background: currentColor;
  color: white;
  font-size: 11px;
  padding: 2px 6px;
  border-radius: 4px;
  white-space: nowrap;
}

/* 冲突警告 */
.conflict-warning {
  position: fixed;
  top: 80px;
  left: 50%;
  transform: translateX(-50%);
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: 8px;
  padding: 12px 16px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 1003;
}

.warning-content {
  display: flex;
  align-items: center;
  gap: 12px;
}

.warning-icon {
  font-size: 18px;
}

.warning-text {
  font-size: 14px;
  color: #991b1b;
  font-weight: 500;
}

.resolve-button {
  background: #dc2626;
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  font-size: 13px;
  cursor: pointer;
  transition: background 0.2s;
}

.resolve-button:hover {
  background: #b91c1c;
}

/* 连接状态提示 */
.connection-status {
  position: fixed;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  z-index: 1004;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.connection-connected {
  background: #d1fae5;
  color: #065f46;
  border: 1px solid #a7f3d0;
}

.connection-connecting {
  background: #fef3c7;
  color: #92400e;
  border: 1px solid #fde68a;
}

.connection-reconnecting {
  background: #ffedd5;
  color: #9a3412;
  border: 1px solid #fed7aa;
}

.connection-error {
  background: #fee2e2;
  color: #991b1b;
  border: 1px solid #fecaca;
}

.connection-disconnected {
  background: #f1f5f9;
  color: #475569;
  border: 1px solid #e2e8f0;
}

/* 过渡动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
