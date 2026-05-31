/*!
 * 航空航天级操作广播器
 * 实现本地编辑操作转换、操作广播、远程操作接收和应用
 */

import type { Editor } from '@tiptap/core';
import type { CRDTOperation } from './collaborationService';
import { getCollaborationService } from './collaborationService';

// 操作转换结果
export interface TransformResult {
  operation: CRDTOperation;
  transformed: boolean;
}

// 操作广播器配置
export interface BroadcasterConfig {
  enableBatching: boolean;
  batchSize: number;
  batchDelay: number;
  enableTransformation: boolean;
}

/**
 * 航空航天级操作广播器
 * 负责将本地编辑操作转换为 CRDTOperation、广播操作、接收远程操作并应用到编辑器
 */
export class OperationBroadcaster {
  private editor: Editor | null = null;
  private userId: string | null = null;
  localVersion: number = 0;
  remoteVersion: number = 0;
  private pendingOperations: CRDTOperation[] = [];
  private operationQueue: CRDTOperation[] = [];
  private config: BroadcasterConfig = {
    enableBatching: true,
    batchSize: 10,
    batchDelay: 100,
    enableTransformation: true
  };
  private batchTimer: ReturnType<typeof setTimeout> | null = null;
  private isProcessingQueue = false;

  // 事件回调
  private onOperationBroadcastCallbacks: ((operation: CRDTOperation) => void)[] = [];
  private onOperationReceivedCallbacks: ((operation: CRDTOperation) => void)[] = [];
  private onOperationAppliedCallbacks: ((operation: CRDTOperation) => void)[] = [];
  private onConflictDetectedCallbacks: ((localOp: CRDTOperation, remoteOp: CRDTOperation) => void)[] = [];

  /**
   * 初始化操作广播器
   */
  constructor(editor: Editor | null = null, config?: Partial<BroadcasterConfig>) {
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
   * 设置用户 ID
   */
  setUserId(userId: string): void {
    this.userId = userId;
  }

  /**
   * 广播操作
   */
  async broadcastOperation(operation: CRDTOperation): Promise<void> {
    if (!this.userId) {
      console.warn('User ID not set, cannot broadcast operation');
      return;
    }

    // 设置操作作者
    operation.author = this.userId;

    if (this.config.enableBatching) {
      // 批处理模式
      this.operationQueue.push(operation);
      this.scheduleBatchProcessing();
    } else {
      // 立即发送
      await this.sendOperation(operation);
    }

    this.onOperationBroadcastCallbacks.forEach((callback) => callback(operation));
  }

  /**
   * 从 TipTap Transaction 广播操作
   */
  async broadcastFromTransaction(transaction: any): Promise<void> {
    if (!this.editor || !this.userId) {
      return;
    }

    const operations = this.transactionToOperations(transaction);
    for (const operation of operations) {
      await this.broadcastOperation(operation);
    }
  }

  /**
   * 接收远程操作
   */
  async receiveOperation(operation: CRDTOperation): Promise<void> {
    // 操作转换
    let transformedOp = operation;
    if (this.config.enableTransformation) {
      for (const pendingOp of this.pendingOperations) {
        transformedOp = this.transformOperation(transformedOp, pendingOp);
      }
    }

    // 检测冲突
    if (this.detectConflict(transformedOp)) {
      this.onConflictDetectedCallbacks.forEach((callback) => callback(transformedOp, operation));
    }

    // 应用到编辑器
    await this.applyOperationToEditor(transformedOp);

    // 更新版本
    this.remoteVersion++;
    this.pendingOperations.push(transformedOp);

    this.onOperationReceivedCallbacks.forEach((callback) => callback(transformedOp));
  }

  /**
   * 批量接收远程操作
   */
  async receiveOperations(operations: CRDTOperation[]): Promise<void> {
    for (const operation of operations) {
      await this.receiveOperation(operation);
    }
  }

  /**
   * 操作转换（OT 算法）
   */
  transformOperation(operation: CRDTOperation, against: CRDTOperation): CRDTOperation {
    const transformed = { ...operation };

    switch (operation.type) {
      case 'insert':
        return this.transformInsert(transformed, against);
      case 'delete':
        return this.transformDelete(transformed, against);
      default:
        return transformed;
    }
  }

  /**
   * 转换插入操作
   */
  private transformInsert(operation: CRDTOperation, against: CRDTOperation): CRDTOperation {
    if (against.type === 'insert' && against.position <= operation.position) {
      operation.position += (against.content?.length || 0);
    } else if (against.type === 'delete' && against.position < operation.position) {
      operation.position -= against.length || 0;
    }
    return operation;
  }

  /**
   * 转换删除操作
   */
  private transformDelete(operation: CRDTOperation, against: CRDTOperation): CRDTOperation {
    if (against.type === 'insert' && against.position <= operation.position) {
      operation.position += (against.content?.length || 0);
    } else if (against.type === 'delete' && against.position < operation.position) {
      operation.position -= against.length || 0;
    }
    return operation;
  }

  /**
   * 检测冲突
   */
  detectConflict(operation: CRDTOperation): boolean {
    // 简单冲突检测：检查操作是否在同一位置
    for (const pendingOp of this.pendingOperations) {
      if (pendingOp.position === operation.position && pendingOp.author !== operation.author) {
        return true;
      }
    }
    return false;
  }

  /**
   * 应用操作到编辑器
   */
  async applyOperationToEditor(operation: CRDTOperation): Promise<void> {
    if (!this.editor) {
      return;
    }

    const { view, state } = this.editor;

    switch (operation.type) {
      case 'insert':
        await this.applyInsertOperation(operation);
        break;
      case 'delete':
        await this.applyDeleteOperation(operation);
        break;
      case 'format':
        await this.applyFormatOperation(operation);
        break;
      case 'retain':
        // 保留操作无需应用
        break;
    }

    this.onOperationAppliedCallbacks.forEach((callback) => callback(operation));
  }

  /**
   * 应用插入操作
   */
  private async applyInsertOperation(operation: CRDTOperation): Promise<void> {
    if (!this.editor || !operation.content) {
      return;
    }

    const { from } = this.editor.state.selection;
    if (from === null) {
      return;
    }

    this.editor
      .chain()
      .focus()
      .insertContentAt(operation.position, operation.content)
      .run();
  }

  /**
   * 应用删除操作
   */
  private async applyDeleteOperation(operation: CRDTOperation): Promise<void> {
    if (!this.editor || !operation.length) {
      return;
    }

    this.editor
      .chain()
      .focus()
      .deleteRange({ from: operation.position, to: operation.position + operation.length })
      .run();
  }

  /**
   * 应用格式操作
   */
  private async applyFormatOperation(operation: CRDTOperation): Promise<void> {
    if (!this.editor || !operation.length || !operation.format) {
      return;
    }

    const { from, to } = this.editor.state.selection;
    if (from === null || to === null) {
      return;
    }

    // 应用格式
    for (const [key, value] of Object.entries(operation.format)) {
      if (key === 'bold' && value === 'true') {
        this.editor.chain().focus().toggleBold().run();
      } else if (key === 'italic' && value === 'true') {
        this.editor.chain().focus().toggleItalic().run();
      } else if (key === 'underline' && value === 'true') {
        this.editor.chain().focus().toggleUnderline().run();
      } else if (key === 'strike' && value === 'true') {
        this.editor.chain().focus().toggleStrike().run();
      } else if (key === 'code' && value === 'true') {
        this.editor.chain().focus().toggleCode().run();
      }
    }
  }

  /**
   * 将 TipTap Transaction 转换为 CRDTOperation
   */
  private transactionToOperations(transaction: any): CRDTOperation[] {
    const operations: CRDTOperation[] = [];

    if (!transaction || !transaction.docChanged) {
      return operations;
    }

    // 简化实现：将文档变更转换为插入/删除操作
    // 实际实现需要更复杂的逻辑来解析 TipTap Transaction
    const steps = transaction.steps || [];
    for (const step of steps) {
      // 这里需要根据 TipTap 的 step 结构进行解析
      // 暂时返回空数组，需要根据实际 TipTap API 实现
    }

    return operations;
  }

  /**
   * 发送操作
   */
  private async sendOperation(operation: CRDTOperation): Promise<void> {
    const collaborationService = getCollaborationService();
    await collaborationService.sendOperation(operation);
    this.localVersion++;
  }

  /**
   * 调度批处理
   */
  private scheduleBatchProcessing(): void {
    if (this.batchTimer) {
      clearTimeout(this.batchTimer);
    }

    this.batchTimer = setTimeout(() => {
      this.processBatch();
    }, this.config.batchDelay);
  }

  /**
   * 处理批次
   */
  private async processBatch(): Promise<void> {
    if (this.isProcessingQueue || this.operationQueue.length === 0) {
      return;
    }

    this.isProcessingQueue = true;

    const batch = this.operationQueue.splice(0, this.config.batchSize);
    for (const operation of batch) {
      await this.sendOperation(operation);
    }

    this.isProcessingQueue = false;

    // 如果还有操作，继续处理
    if (this.operationQueue.length > 0) {
      this.scheduleBatchProcessing();
    }
  }

  /**
   * 获取本地版本
   */
  getLocalVersion(): number {
    return this.localVersion;
  }

  /**
   * 获取远程版本
   */
  getRemoteVersion(): number {
    return this.remoteVersion;
  }

  /**
   * 获取待处理操作
   */
  getPendingOperations(): CRDTOperation[] {
    return [...this.pendingOperations];
  }

  /**
   * 清除待处理操作
   */
  clearPendingOperations(): void {
    this.pendingOperations = [];
  }

  /**
   * 注册操作广播回调
   */
  onOperationBroadcast(callback: (operation: CRDTOperation) => void): void {
    this.onOperationBroadcastCallbacks.push(callback);
  }

  /**
   * 注册操作接收回调
   */
  onOperationReceived(callback: (operation: CRDTOperation) => void): void {
    this.onOperationReceivedCallbacks.push(callback);
  }

  /**
   * 注册操作应用回调
   */
  onOperationApplied(callback: (operation: CRDTOperation) => void): void {
    this.onOperationAppliedCallbacks.push(callback);
  }

  /**
   * 注册冲突检测回调
   */
  onConflictDetected(callback: (localOp: CRDTOperation, remoteOp: CRDTOperation) => void): void {
    this.onConflictDetectedCallbacks.push(callback);
  }

  /**
   * 获取配置
   */
  getConfig(): BroadcasterConfig {
    return { ...this.config };
  }

  /**
   * 更新配置
   */
  updateConfig(config: Partial<BroadcasterConfig>): void {
    this.config = { ...this.config, ...config };
  }

  /**
   * 销毁操作广播器
   */
  destroy(): void {
    if (this.batchTimer) {
      clearTimeout(this.batchTimer);
      this.batchTimer = null;
    }

    this.onOperationBroadcastCallbacks = [];
    this.onOperationReceivedCallbacks = [];
    this.onOperationAppliedCallbacks = [];
    this.onConflictDetectedCallbacks = [];
    this.operationQueue = [];
    this.pendingOperations = [];
  }
}

// 全局操作广播器实例
let operationBroadcasterInstance: OperationBroadcaster | null = null;

/**
 * 获取操作广播器单例
 */
export function getOperationBroadcaster(editor?: Editor, config?: Partial<BroadcasterConfig>): OperationBroadcaster {
  if (!operationBroadcasterInstance) {
    operationBroadcasterInstance = new OperationBroadcaster(editor || null, config);
  } else if (editor) {
    operationBroadcasterInstance.setEditor(editor);
  }
  return operationBroadcasterInstance;
}

/**
 * 销毁操作广播器单例
 */
export function destroyOperationBroadcaster(): void {
  if (operationBroadcasterInstance) {
    operationBroadcasterInstance.destroy();
    operationBroadcasterInstance = null;
  }
}
