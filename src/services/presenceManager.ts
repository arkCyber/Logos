/*!
 * 航空航天级在线状态管理器
 * 实现用户在线状态管理、在线用户列表、用户加入/离开事件处理
 */

import { ref } from 'vue';
import type { PresenceInfo } from './collaborationService';

// 用户活动信息
interface UserActivity {
  presence: PresenceInfo;
  lastActivity: Date;
}

/**
 * 航空航天级在线状态管理器
 * 负责管理用户在线状态、显示在线用户列表、处理用户加入/离开事件
 */
export class PresenceManager {
  private users: Map<string, UserActivity> = new Map();
  private currentUserId: string | null = null;
  private activityTimeout: number = 60000; // 60 秒无活动自动离线
  private cleanupTimer: ReturnType<typeof setInterval> | null = null;

  // 事件回调
  private onUserJoinedCallbacks: ((user: PresenceInfo) => void)[] = [];
  private onUserLeftCallbacks: ((userId: string) => void)[] = [];
  private onPresenceUpdatedCallbacks: ((presence: PresenceInfo) => void)[] = [];

  /**
   * 初始化在线状态管理器
   */
  constructor(currentUserId: string | null = null) {
    this.currentUserId = currentUserId;
    this.startCleanupTimer();
  }

  /**
   * 设置当前用户 ID
   */
  setCurrentUserId(userId: string): void {
    this.currentUserId = userId;
  }

  /**
   * 更新用户在线状态
   */
  updatePresence(presence: PresenceInfo): void {
    const userId = presence.user_id;

    // 如果是新用户
    if (!this.users.has(userId)) {
      this.users.set(userId, {
        presence,
        lastActivity: new Date()
      });
      this.onUserJoinedCallbacks.forEach((callback) => callback(presence));
    } else {
      // 更新现有用户
      const existing = this.users.get(userId);
      if (existing) {
        existing.presence = presence;
        existing.lastActivity = new Date();
        this.onPresenceUpdatedCallbacks.forEach((callback) => callback(presence));
      }
    }
  }

  /**
   * 批量更新用户在线状态
   */
  updatePresences(presences: PresenceInfo[]): void {
    presences.forEach((presence) => this.updatePresence(presence));
  }

  /**
   * 移除用户
   */
  removeUser(userId: string): void {
    if (this.users.has(userId)) {
      this.users.delete(userId);
      this.onUserLeftCallbacks.forEach((callback) => callback(userId));
    }
  }

  /**
   * 获取所有在线用户
   */
  getOnlineUsers(): PresenceInfo[] {
    const users: PresenceInfo[] = [];
    for (const activity of this.users.values()) {
      if (activity.presence.is_online) {
        users.push(activity.presence);
      }
    }
    // 按最后活动时间排序
    return users.sort((a, b) => {
      const activityA = this.users.get(a.user_id)?.lastActivity.getTime() ?? 0;
      const activityB = this.users.get(b.user_id)?.lastActivity.getTime() ?? 0;
      return activityB - activityA;
    });
  }

  /**
   * 获取特定用户
   */
  getUser(userId: string): PresenceInfo | undefined {
    const activity = this.users.get(userId);
    return activity?.presence;
  }

  /**
   * 获取在线用户数量
   */
  getOnlineUserCount(): number {
    return this.getOnlineUsers().length;
  }

  /**
   * 检查用户是否在线
   */
  isUserOnline(userId: string): boolean {
    const activity = this.users.get(userId);
    return activity?.presence.is_online || false;
  }

  /**
   * 获取当前用户
   */
  getCurrentUser(): PresenceInfo | undefined {
    if (this.currentUserId) {
      return this.getUser(this.currentUserId);
    }
    return undefined;
  }

  /**
   * 注册用户加入回调
   */
  onUserJoined(callback: (user: PresenceInfo) => void): void {
    this.onUserJoinedCallbacks.push(callback);
  }

  /**
   * 注册用户离开回调
   */
  onUserLeft(callback: (userId: string) => void): void {
    this.onUserLeftCallbacks.push(callback);
  }

  /**
   * 注册在线状态更新回调
   */
  onPresenceUpdated(callback: (presence: PresenceInfo) => void): void {
    this.onPresenceUpdatedCallbacks.push(callback);
  }

  /**
   * 启动清理定时器
   */
  private startCleanupTimer(): void {
    this.cleanupTimer = setInterval(() => {
      this.cleanupInactiveUsers();
    }, 30000); // 每 30 秒清理一次
  }

  /**
   * 停止清理定时器
   */
  private stopCleanupTimer(): void {
    if (this.cleanupTimer) {
      clearInterval(this.cleanupTimer);
      this.cleanupTimer = null;
    }
  }

  /**
   * 清理不活跃用户
   */
  private cleanupInactiveUsers(): void {
    const now = new Date();
    const inactiveUsers: string[] = [];

    for (const [userId, activity] of this.users) {
      const elapsed = now.getTime() - activity.lastActivity.getTime();
      if (elapsed > this.activityTimeout) {
        inactiveUsers.push(userId);
      }
    }

    inactiveUsers.forEach((userId) => {
      this.removeUser(userId);
    });
  }

  /**
   * 清理所有用户
   */
  clearAllUsers(): void {
    this.users.clear();
  }

  /**
   * 获取活动超时时间
   */
  getActivityTimeout(): number {
    return this.activityTimeout;
  }

  /**
   * 设置活动超时时间
   */
  setActivityTimeout(timeout: number): void {
    this.activityTimeout = timeout;
  }

  /**
   * 销毁在线状态管理器
   */
  destroy(): void {
    this.stopCleanupTimer();
    this.onUserJoinedCallbacks = [];
    this.onUserLeftCallbacks = [];
    this.onPresenceUpdatedCallbacks = [];
    this.users.clear();
  }
}

// 全局在线状态管理器实例
let presenceManagerInstance: PresenceManager | null = null;

/**
 * 获取在线状态管理器单例
 */
export function getPresenceManager(currentUserId?: string): PresenceManager {
  if (!presenceManagerInstance) {
    presenceManagerInstance = new PresenceManager(currentUserId || null);
  } else if (currentUserId) {
    presenceManagerInstance.setCurrentUserId(currentUserId);
  }
  return presenceManagerInstance;
}

/**
 * 销毁在线状态管理器单例
 */
export function destroyPresenceManager(): void {
  if (presenceManagerInstance) {
    presenceManagerInstance.destroy();
    presenceManagerInstance = null;
  }
}

/**
 * Vue 3 响应式在线状态管理器
 * 提供响应式状态用于 Vue 组件
 */
export function usePresenceManager(currentUserId?: string) {
  const manager = getPresenceManager(currentUserId);
  const onlineUsers = ref<PresenceInfo[]>(manager.getOnlineUsers());
  const onlineUserCount = ref<number>(manager.getOnlineUserCount());

  // 监听用户加入事件
  manager.onUserJoined((_user) => {
    onlineUsers.value = manager.getOnlineUsers();
    onlineUserCount.value = manager.getOnlineUserCount();
  });

  // 监听用户离开事件
  manager.onUserLeft(() => {
    onlineUsers.value = manager.getOnlineUsers();
    onlineUserCount.value = manager.getOnlineUserCount();
  });

  // 监听在线状态更新事件
  manager.onPresenceUpdated(() => {
    onlineUsers.value = manager.getOnlineUsers();
    onlineUserCount.value = manager.getOnlineUserCount();
  });

  return {
    manager,
    onlineUsers,
    onlineUserCount,
    updatePresence: (presence: PresenceInfo) => manager.updatePresence(presence),
    removeUser: (userId: string) => manager.removeUser(userId),
    isUserOnline: (userId: string) => manager.isUserOnline(userId),
    getUser: (userId: string) => manager.getUser(userId),
    getCurrentUser: () => manager.getCurrentUser()
  };
}
