/*!
 * 航空航天级光标追踪器
 * 实现本地光标追踪、远程光标接收、光标渲染、冲突检测
 */

import type { Editor } from '@tiptap/core';
import { type PresenceInfo } from './collaborationService';

// 光标信息
export interface CursorInfo {
  userId: string;
  userName: string;
  position: number;
  color: string;
  lastUpdated: Date;
}

// 光标配置
export interface CursorConfig {
  showUserName: boolean;
  showCursor: boolean;
  cursorColor: string;
  remoteCursorColors: string[];
}

/**
 * 航空航天级光标追踪器
 * 负责追踪本地光标位置、接收远程光标、渲染远程光标
 */
export class CursorTracker {
  private editor: Editor | null = null;
  private localPosition: number = 0;
  private remoteCursors: Map<string, CursorInfo> = new Map();
  private config: CursorConfig = {
    showUserName: true,
    showCursor: true,
    cursorColor: '#3b82f6',
    remoteCursorColors: [
      '#ef4444', // red
      '#f59e0b', // amber
      '#10b981', // emerald
      '#3b82f6', // blue
      '#8b5cf6', // violet
      '#ec4899', // pink
      '#06b6d4', // cyan
      '#84cc16' // lime
    ]
  };
  private updateDebounceTimer: ReturnType<typeof setTimeout> | null = null;
  private colorIndex: number = 0;

  // 事件回调
  private onLocalPositionChangeCallbacks: ((position: number) => void)[] = [];
  private onRemoteCursorUpdateCallbacks: ((cursor: CursorInfo) => void)[] = [];
  private onRemoteCursorRemoveCallbacks: ((userId: string) => void)[] = [];

  /**
   * 初始化光标追踪器
   */
  constructor(editor: Editor | null = null, config?: Partial<CursorConfig>) {
    this.editor = editor;
    if (config) {
      this.config = { ...this.config, ...config };
    }
  }

  /**
   * 设置编辑器
   */
  setEditor(editor: Editor): void {
    this.editor = editor;
  }

  /**
   * 追踪本地光标位置
   */
  trackLocalPosition(position: number): void {
    this.localPosition = position;

    // 防抖处理
    if (this.updateDebounceTimer) {
      clearTimeout(this.updateDebounceTimer);
    }

    this.updateDebounceTimer = setTimeout(() => {
      this.onLocalPositionChangeCallbacks.forEach((callback) => callback(position));
    }, 100);
  }

  /**
   * 从编辑器自动追踪光标位置
   */
  trackFromEditor(): void {
    if (!this.editor) {
      return;
    }

    const { from, to } = this.editor.state.selection;
    if (from !== null) {
      this.trackLocalPosition(from);
    }
  }

  /**
   * 更新远程光标
   */
  updateRemoteCursor(userId: string, position: number, userName: string): void {
    const color = this.getUserColor(userId);
    const cursor: CursorInfo = {
      userId,
      userName,
      position,
      color,
      lastUpdated: new Date()
    };

    this.remoteCursors.set(userId, cursor);
    this.onRemoteCursorUpdateCallbacks.forEach((callback) => callback(cursor));
  }

  /**
   * 从 PresenceInfo 更新远程光标
   */
  updateRemoteCursorFromPresence(presence: PresenceInfo): void {
    if (presence.cursor_position !== undefined) {
      this.updateRemoteCursor(presence.user_id, presence.cursor_position, presence.user_name);
    }
  }

  /**
   * 移除远程光标
   */
  removeRemoteCursor(userId: string): void {
    if (this.remoteCursors.has(userId)) {
      this.remoteCursors.delete(userId);
      this.onRemoteCursorRemoveCallbacks.forEach((callback) => callback(userId));
    }
  }

  /**
   * 获取所有远程光标
   */
  getRemoteCursors(): Map<string, CursorInfo> {
    return new Map(this.remoteCursors);
  }

  /**
   * 获取特定用户的远程光标
   */
  getRemoteCursor(userId: string): CursorInfo | undefined {
    return this.remoteCursors.get(userId);
  }

  /**
   * 获取本地光标位置
   */
  getLocalPosition(): number {
    return this.localPosition;
  }

  /**
   * 检测光标冲突
   */
  detectConflict(position: number, threshold: number = 5): boolean {
    for (const cursor of this.remoteCursors.values()) {
      if (Math.abs(cursor.position - position) < threshold) {
        return true;
      }
    }
    return false;
  }

  /**
   * 获取用户颜色
   */
  private getUserColor(userId: string): string {
    // 基于用户 ID 生成确定的颜色
    const hash = this.hashCode(userId);
    const index = Math.abs(hash) % this.config.remoteCursorColors.length;
    return this.config.remoteCursorColors[index];
  }

  /**
   * 字符串哈希函数
   */
  private hashCode(str: string): number {
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
      const char = str.charCodeAt(i);
      hash = (hash << 5) - hash + char;
      hash = hash & hash; // Convert to 32bit integer
    }
    return hash;
  }

  /**
   * 注册本地光标位置变化回调
   */
  onLocalPositionChange(callback: (position: number) => void): void {
    this.onLocalPositionChangeCallbacks.push(callback);
  }

  /**
   * 注册远程光标更新回调
   */
  onRemoteCursorUpdate(callback: (cursor: CursorInfo) => void): void {
    this.onRemoteCursorUpdateCallbacks.push(callback);
  }

  /**
   * 注册远程光标移除回调
   */
  onRemoteCursorRemove(callback: (userId: string) => void): void {
    this.onRemoteCursorRemoveCallbacks.push(callback);
  }

  /**
   * 清理过期光标（超过 30 秒未更新）
   */
  cleanupExpiredCursors(): void {
    const now = new Date();
    const threshold = 30000; // 30 秒

    for (const [userId, cursor] of this.remoteCursors) {
      const elapsed = now.getTime() - cursor.lastUpdated.getTime();
      if (elapsed > threshold) {
        this.removeRemoteCursor(userId);
      }
    }
  }

  /**
   * 清理所有远程光标
   */
  clearRemoteCursors(): void {
    this.remoteCursors.clear();
  }

  /**
   * 获取光标配置
   */
  getConfig(): CursorConfig {
    return { ...this.config };
  }

  /**
   * 更新光标配置
   */
  updateConfig(config: Partial<CursorConfig>): void {
    this.config = { ...this.config, ...config };
  }

  /**
   * 销毁光标追踪器
   */
  destroy(): void {
    if (this.updateDebounceTimer) {
      clearTimeout(this.updateDebounceTimer);
      this.updateDebounceTimer = null;
    }

    this.onLocalPositionChangeCallbacks = [];
    this.onRemoteCursorUpdateCallbacks = [];
    this.onRemoteCursorRemoveCallbacks = [];
    this.remoteCursors.clear();
  }
}

// 全局光标追踪器实例
let cursorTrackerInstance: CursorTracker | null = null;

/**
 * 获取光标追踪器单例
 */
export function getCursorTracker(editor?: Editor, config?: Partial<CursorConfig>): CursorTracker {
  if (!cursorTrackerInstance) {
    cursorTrackerInstance = new CursorTracker(editor || null, config);
  } else if (editor) {
    cursorTrackerInstance.setEditor(editor);
  }
  return cursorTrackerInstance;
}

/**
 * 销毁光标追踪器单例
 */
export function destroyCursorTracker(): void {
  if (cursorTrackerInstance) {
    cursorTrackerInstance.destroy();
    cursorTrackerInstance = null;
  }
}
