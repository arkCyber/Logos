/*!
 * 航空航天级协作服务
 * 实现 WebSocket 连接管理、消息发送/接收、重连机制
 */

import { invoke } from '@tauri-apps/api/core';
import { logger, LogCategory } from '../utils/logger';

// CRDT 操作类型
export enum CRDTType {
  Text = 'text',
  RichText = 'richtext',
  JSON = 'json',
}

// CRDT 操作
export interface CRDTOperation {
  type: 'insert' | 'delete' | 'retain' | 'format';
  id: string;
  position: number;
  length?: number;
  content?: string;
  format?: Record<string, string>;
  author: string;
  timestamp: string;
}

// 用户在线状态
export interface PresenceInfo {
  user_id: string;
  user_name: string;
  cursor_position?: number;
  selection?: [number, number];
  last_seen: string;
  is_online: boolean;
}

// 协作消息类型
export type CollaborationMessage =
  | { message_type: 'join'; user_id: string; user_name: string; document_id: string }
  | { message_type: 'leave'; user_id: string; document_id: string }
  | { message_type: 'operation'; user_id: string; document_id: string; operation: CRDTOperation }
  | { message_type: 'presence'; user_id: string; document_id: string; presence: PresenceInfo }
  | { message_type: 'sync_request'; user_id: string; document_id: string; since_version: number }
  | { message_type: 'sync_response'; user_id: string; document_id: string; operations: CRDTOperation[]; current_version: number }
  | { message_type: 'error'; message: string };

// 连接状态
export enum ConnectionStatus {
  Disconnected = 'disconnected',
  Connecting = 'connecting',
  Connected = 'connected',
  Reconnecting = 'reconnecting',
  Error = 'error',
}

// 事件回调类型
type EventCallback<T> = (data: T) => void;

/**
 * 航空航天级协作服务
 * 负责管理 WebSocket 连接、消息收发、重连机制
 */
export class CollaborationService {
  private documentId: string | null = null;
  private userId: string | null = null;
  private userName: string | null = null;
  private status: ConnectionStatus = ConnectionStatus.Disconnected;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000; // 初始重连延迟 1s
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private heartbeatTimer: ReturnType<typeof setInterval> | null = null;
  private messageQueue: CollaborationMessage[] = [];
  private isProcessingQueue = false;

  // 事件回调
  private onMessageCallbacks: EventCallback<CollaborationMessage>[] = [];
  private onConnectedCallbacks: EventCallback<void>[] = [];
  private onDisconnectedCallbacks: EventCallback<void>[] = [];
  private onErrorCallbacks: EventCallback<Error>[] = [];

  /**
   * 连接到协作服务器
   */
  async connect(documentId: string, userId: string, userName: string): Promise<void> {
    if (this.status === ConnectionStatus.Connected) {
      throw new Error('Already connected');
    }

    this.documentId = documentId;
    this.userId = userId;
    this.userName = userName;
    this.status = ConnectionStatus.Connecting;
    this.reconnectAttempts = 0;

    try {
      // 调用后端 WebSocket 连接
      await invoke('collaboration_join', {
        documentId,
        userId,
        userName
      });

      this.status = ConnectionStatus.Connected;
      this.reconnectAttempts = 0;
      this.reconnectDelay = 1000;

      // 启动心跳检测
      this.startHeartbeat();

      // 处理消息队列
      this.processMessageQueue();

      // 触发连接成功回调
      this.onConnectedCallbacks.forEach((callback) => callback());
    } catch (error) {
      this.status = ConnectionStatus.Error;
      this.onErrorCallbacks.forEach((callback) => callback(error as Error));
      throw error;
    }
  }

  /**
   * 断开连接
   */
  async disconnect(): Promise<void> {
    if (this.status === ConnectionStatus.Disconnected) {
      return;
    }

    // 停止心跳
    this.stopHeartbeat();

    // 清除重连定时器
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }

    try {
      if (this.documentId && this.userId) {
        await invoke('collaboration_leave', {
          documentId: this.documentId,
          userId: this.userId
        });
      }
    } catch (error) {
      logger.error('Error leaving collaboration', error, LogCategory.SYSTEM);
    }

    this.status = ConnectionStatus.Disconnected;
    this.documentId = null;
    this.userId = null;
    this.userName = null;
    this.messageQueue = [];

    // 触发断开连接回调
    this.onDisconnectedCallbacks.forEach((callback) => callback());
  }

  /**
   * 发送操作
   */
  async sendOperation(operation: CRDTOperation): Promise<void> {
    if (this.status !== ConnectionStatus.Connected || !this.documentId || !this.userId) {
      // 离线时缓存操作
      this.messageQueue.push({
        message_type: 'operation',
        user_id: this.userId || '',
        document_id: this.documentId || '',
        operation
      });
      return;
    }

    try {
      await invoke('collaboration_send_operation', {
        documentId: this.documentId,
        userId: this.userId,
        operation
      });
    } catch (error) {
      logger.error('Error sending operation', error, LogCategory.SYSTEM);
      this.onErrorCallbacks.forEach((callback) => callback(error as Error));
    }
  }

  /**
   * 发送在线状态
   */
  async sendPresence(presence: PresenceInfo): Promise<void> {
    if (this.status !== ConnectionStatus.Connected || !this.documentId || !this.userId) {
      return;
    }

    try {
      await invoke('collaboration_update_presence', {
        documentId: this.documentId,
        userId: this.userId,
        presence
      });
    } catch (error) {
      logger.error('Error sending presence', error, LogCategory.SYSTEM);
      this.onErrorCallbacks.forEach((callback) => callback(error as Error));
    }
  }

  /**
   * 请求同步
   */
  async requestSync(sinceVersion: number): Promise<CRDTOperation[]> {
    if (this.status !== ConnectionStatus.Connected || !this.documentId || !this.userId) {
      return [];
    }

    try {
      const operations = await invoke<CRDTOperation[]>('collaboration_request_sync', {
        documentId: this.documentId,
        userId: this.userId,
        sinceVersion
      });
      return operations;
    } catch (error) {
      logger.error('Error requesting sync', error, LogCategory.SYSTEM);
      this.onErrorCallbacks.forEach((callback) => callback(error as Error));
      return [];
    }
  }

  /**
   * 处理接收到的消息
   */
  handleMessage(message: CollaborationMessage): void {
    // 触发消息回调
    this.onMessageCallbacks.forEach((callback) => callback(message));
  }

  /**
   * 注册消息回调
   */
  onMessage(callback: EventCallback<CollaborationMessage>): void {
    this.onMessageCallbacks.push(callback);
  }

  /**
   * 注册连接成功回调
   */
  onConnected(callback: EventCallback<void>): void {
    this.onConnectedCallbacks.push(callback);
  }

  /**
   * 注册断开连接回调
   */
  onDisconnected(callback: EventCallback<void>): void {
    this.onDisconnectedCallbacks.push(callback);
  }

  /**
   * 注册错误回调
   */
  onError(callback: EventCallback<Error>): void {
    this.onErrorCallbacks.push(callback);
  }

  /**
   * 获取连接状态
   */
  getStatus(): ConnectionStatus {
    return this.status;
  }

  /**
   * 获取文档 ID
   */
  getDocumentId(): string | null {
    return this.documentId;
  }

  /**
   * 获取用户 ID
   */
  getUserId(): string | null {
    return this.userId;
  }

  /**
   * 获取用户名
   */
  getUserName(): string | null {
    return this.userName;
  }

  /**
   * 启动心跳检测
   */
  private startHeartbeat(): void {
    this.heartbeatTimer = setInterval(() => {
      if (this.status === ConnectionStatus.Connected) {
        // 发送心跳
        this.sendPresence({
          user_id: this.userId || '',
          user_name: this.userName || '',
          last_seen: new Date().toISOString(),
          is_online: true
        });
      }
    }, 30000); // 每 30 秒发送一次心跳
  }

  /**
   * 停止心跳检测
   */
  private stopHeartbeat(): void {
    if (this.heartbeatTimer) {
      clearInterval(this.heartbeatTimer);
      this.heartbeatTimer = null;
    }
  }

  /**
   * 处理消息队列
   */
  private async processMessageQueue(): Promise<void> {
    if (this.isProcessingQueue || this.messageQueue.length === 0) {
      return;
    }

    this.isProcessingQueue = true;

    while (this.messageQueue.length > 0 && this.status === ConnectionStatus.Connected) {
      const message = this.messageQueue.shift();
      if (!message) {
break;
}
      try {
        if (message.message_type === 'operation') {
          await this.sendOperation(message.operation);
        }
      } catch (error) {
        logger.error('Error processing queued message', error, LogCategory.SYSTEM);
        // 重新加入队列
        this.messageQueue.unshift(message);
        break;
      }
    }

    this.isProcessingQueue = false;
  }

  /**
   * 自动重连
   */
  private async reconnect(): Promise<void> {
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      this.status = ConnectionStatus.Error;
      this.onErrorCallbacks.forEach((callback) =>
        callback(new Error('Max reconnection attempts reached'))
      );
      return;
    }

    this.status = ConnectionStatus.Reconnecting;
    this.reconnectAttempts++;

    try {
      if (this.documentId && this.userId && this.userName) {
        await this.connect(this.documentId, this.userId, this.userName);
      }
    } catch (error) {
      logger.error('Reconnection failed', error, LogCategory.SYSTEM);
      // 指数退避
      this.reconnectDelay = Math.min(this.reconnectDelay * 2, 30000);
      this.reconnectTimer = setTimeout(() => this.reconnect(), this.reconnectDelay);
    }
  }

  /**
   * 清理资源
   */
  async destroy(): Promise<void> {
    await this.disconnect();
    this.onMessageCallbacks = [];
    this.onConnectedCallbacks = [];
    this.onDisconnectedCallbacks = [];
    this.onErrorCallbacks = [];
  }
}

// 全局协作服务实例
let collaborationServiceInstance: CollaborationService | null = null;

/**
 * 获取协作服务单例
 */
export function getCollaborationService(): CollaborationService {
  if (!collaborationServiceInstance) {
    collaborationServiceInstance = new CollaborationService();
  }
  return collaborationServiceInstance;
}

/**
 * 销毁协作服务单例
 */
export function destroyCollaborationService(): void {
  if (collaborationServiceInstance) {
    collaborationServiceInstance.destroy();
    collaborationServiceInstance = null;
  }
}
