/**
 * 航空航天级错误处理系统
 * 提供完整的错误追踪、恢复和报告机制
 */

import { logger } from './logger';

/**
 * 通知接口
 */
export interface NotificationHandler {
  show(message: string, type: 'info' | 'success' | 'warning' | 'error'): void;
}

/**
 * 通知类型
 */
export type NotificationType = 'info' | 'success' | 'warning' | 'error';

/**
 * 错误严重性级别（航空航天标准）
 */
export enum ErrorSeverity {
  DEBUG = 'debug',           // 调试信息，不影响系统运行
  INFO = 'info',             // 一般信息
  WARNING = 'warning',       // 警告，需要关注但不影响核心功能
  ERROR = 'error',           // 错误，影响部分功能
  CRITICAL = 'critical',     // 严重错误，影响核心功能
  FATAL = 'fatal'            // 致命错误，系统无法继续运行
}

/**
 * 错误类别（航空航天标准）
 */
export enum ErrorCategory {
  SYSTEM = 'system',         // 系统级错误
  NETWORK = 'network',       // 网络错误
  FILE_IO = 'file_io',       // 文件I/O错误
  VALIDATION = 'validation', // 验证错误
  BUSINESS = 'business',    // 业务逻辑错误
  SECURITY = 'security',    // 安全错误
  PERFORMANCE = 'performance', // 性能问题
  EXTERNAL = 'external'     // 外部依赖错误
}

/**
 * 错误上下文信息
 */
export interface ErrorContext {
  timestamp: number;
  userId?: string;
  sessionId?: string;
  component?: string;
  action?: string;
  stackTrace?: string;
  additionalData?: Record<string, any>;
}

/**
 * 航空航天级应用错误类
 */
export class AppError extends Error {
  constructor(
    message: string,
    public code: string,
    public severity: ErrorSeverity = ErrorSeverity.ERROR,
    public category: ErrorCategory = ErrorCategory.SYSTEM,
    public context?: ErrorContext,
    public recoverable: boolean = true
  ) {
    super(message);
    this.name = 'AppError';

    // 保持正确的堆栈跟踪
    if ((Error as any).captureStackTrace && typeof (Error as any).captureStackTrace === 'function') {
      (Error as any).captureStackTrace(this, AppError);
    }

    // 添加时间戳
    if (!this.context) {
      this.context = { timestamp: Date.now() };
    } else if (!this.context.timestamp) {
      this.context.timestamp = Date.now();
    }

    // 捕获堆栈跟踪
    if (this.stack && !this.context.stackTrace) {
      this.context.stackTrace = this.stack;
    }
  }

  /**
   * 转换为JSON格式用于日志记录
   */
  toJSON(): Record<string, any> {
    return {
      name: this.name,
      message: this.message,
      code: this.code,
      severity: this.severity,
      category: this.category,
      recoverable: this.recoverable,
      context: this.context,
      stack: this.stack
    };
  }
}

/**
 * 错误代码枚举（航空航天标准）
 */
export enum ErrorCode {
  // 文件操作错误
  FILE_NOT_FOUND = 'FILE_NOT_FOUND',
  FILE_READ_ERROR = 'FILE_READ_ERROR',
  FILE_WRITE_ERROR = 'FILE_WRITE_ERROR',
  FILE_PARSE_ERROR = 'FILE_PARSE_ERROR',
  FILE_CORRUPTED = 'FILE_CORRUPTED',
  FILE_LOCKED = 'FILE_LOCKED',

  // 网络错误
  NETWORK_ERROR = 'NETWORK_ERROR',
  API_ERROR = 'API_ERROR',
  TIMEOUT_ERROR = 'TIMEOUT_ERROR',
  CONNECTION_LOST = 'CONNECTION_LOST',
  DNS_RESOLUTION_FAILED = 'DNS_RESOLUTION_FAILED',

  // 验证错误
  VALIDATION_ERROR = 'VALIDATION_ERROR',
  INVALID_INPUT = 'INVALID_INPUT',
  DATA_CORRUPTION = 'DATA_CORRUPTION',
  SCHEMA_VIOLATION = 'SCHEMA_VIOLATION',

  // 业务逻辑错误
  OPERATION_FAILED = 'OPERATION_FAILED',
  PERMISSION_DENIED = 'PERMISSION_DENIED',
  RESOURCE_NOT_FOUND = 'RESOURCE_NOT_FOUND',
  RESOURCE_LOCKED = 'RESOURCE_LOCKED',
  CONFLICT_ERROR = 'CONFLICT_ERROR',
  CONVERSION_ERROR = 'CONVERSION_ERROR',
  
  // Typst 相关错误
  TYPST_PARSE_ERROR = 'TYPST_PARSE_ERROR',
  TYPST_COMPILE_ERROR = 'TYPST_COMPILE_ERROR',
  TYPST_EXPORT_ERROR = 'TYPST_EXPORT_ERROR',
  TYPST_TEMPLATE_ERROR = 'TYPST_TEMPLATE_ERROR',
  TYPST_SYNTAX_ERROR = 'TYPST_SYNTAX_ERROR',

  // 系统错误
  UNKNOWN_ERROR = 'UNKNOWN_ERROR',
  INTERNAL_ERROR = 'INTERNAL_ERROR',
  OUT_OF_MEMORY = 'OUT_OF_MEMORY',
  STACK_OVERFLOW = 'STACK_OVERFLOW',

  // 安全错误
  AUTHENTICATION_FAILED = 'AUTHENTICATION_FAILED',
  AUTHORIZATION_FAILED = 'AUTHORIZATION_FAILED',
  SECURITY_VIOLATION = 'SECURITY_VIOLATION',
  MALICIOUS_INPUT = 'MALICIOUS_INPUT',

  // 性能错误
  PERFORMANCE_DEGRADATION = 'PERFORMANCE_DEGRADATION',
  RESOURCE_EXHAUSTION = 'RESOURCE_EXHAUSTION',
  DEADLOCK_DETECTED = 'DEADLOCK_DETECTED'
}

/**
 * 错误消息映射（航空航天标准）
 */
const ERROR_MESSAGES: Record<string, string> = {
  [ErrorCode.FILE_NOT_FOUND]: '文件未找到',
  [ErrorCode.FILE_READ_ERROR]: '读取文件失败',
  [ErrorCode.FILE_WRITE_ERROR]: '保存文件失败',
  [ErrorCode.FILE_PARSE_ERROR]: '文件格式错误',
  [ErrorCode.FILE_CORRUPTED]: '文件已损坏',
  [ErrorCode.FILE_LOCKED]: '文件已被锁定',
  [ErrorCode.NETWORK_ERROR]: '网络连接失败',
  [ErrorCode.API_ERROR]: 'API 请求失败',
  [ErrorCode.TIMEOUT_ERROR]: '操作超时',
  [ErrorCode.CONNECTION_LOST]: '网络连接已断开',
  [ErrorCode.DNS_RESOLUTION_FAILED]: 'DNS解析失败',
  [ErrorCode.VALIDATION_ERROR]: '数据验证失败',
  [ErrorCode.INVALID_INPUT]: '输入无效',
  [ErrorCode.DATA_CORRUPTION]: '数据已损坏',
  [ErrorCode.SCHEMA_VIOLATION]: '数据结构违反',
  [ErrorCode.OPERATION_FAILED]: '操作失败',
  [ErrorCode.PERMISSION_DENIED]: '权限不足',
  [ErrorCode.RESOURCE_NOT_FOUND]: '资源未找到',
  [ErrorCode.RESOURCE_LOCKED]: '资源已被锁定',
  [ErrorCode.CONFLICT_ERROR]: '操作冲突',
  [ErrorCode.CONVERSION_ERROR]: '数据转换失败',
  [ErrorCode.TYPST_PARSE_ERROR]: 'Typst 解析错误',
  [ErrorCode.TYPST_COMPILE_ERROR]: 'Typst 编译错误',
  [ErrorCode.TYPST_EXPORT_ERROR]: 'Typst 导出错误',
  [ErrorCode.TYPST_TEMPLATE_ERROR]: 'Typst 模板错误',
  [ErrorCode.TYPST_SYNTAX_ERROR]: 'Typst 语法错误',
  [ErrorCode.UNKNOWN_ERROR]: '未知错误',
  [ErrorCode.INTERNAL_ERROR]: '内部错误',
  [ErrorCode.OUT_OF_MEMORY]: '内存不足',
  [ErrorCode.STACK_OVERFLOW]: '堆栈溢出',
  [ErrorCode.AUTHENTICATION_FAILED]: '身份验证失败',
  [ErrorCode.AUTHORIZATION_FAILED]: '授权失败',
  [ErrorCode.SECURITY_VIOLATION]: '安全违规',
  [ErrorCode.MALICIOUS_INPUT]: '检测到恶意输入',
  [ErrorCode.PERFORMANCE_DEGRADATION]: '性能下降',
  [ErrorCode.RESOURCE_EXHAUSTION]: '资源耗尽',
  [ErrorCode.DEADLOCK_DETECTED]: '检测到死锁'
};

/**
 * 通知处理器实例
 */
let notificationHandler: NotificationHandler | null = null;

/**
 * 设置通知处理器
 */
export function setNotificationHandler(handler: NotificationHandler): void {
  notificationHandler = handler;
  logger.info('Notification handler set');
}

/**
 * 获取通知处理器
 */
export function getNotificationHandler(): NotificationHandler | null {
  return notificationHandler;
}

/**
 * 清除通知处理器
 */
export function clearNotificationHandler(): void {
  notificationHandler = null;
  logger.info('Notification handler cleared');
}

/**
 * 显示通知
 */
function showNotification(message: string, type: NotificationType = 'error'): void {
  if (notificationHandler) {
    try {
      notificationHandler.show(message, type);
    } catch (error) {
      logger.error('Failed to show notification', error);
    }
  }
}

/**
 * 错误历史记录（航空航天标准）
 */
interface ErrorRecord {
  error: AppError;
  timestamp: number;
  resolved: boolean;
  resolutionTime?: number;
}

const errorHistory: ErrorRecord[] = [];
const MAX_ERROR_HISTORY = 1000;

/**
 * 记录错误到历史
 */
function recordError(error: AppError): void {
  const record: ErrorRecord = {
    error,
    timestamp: Date.now(),
    resolved: false
  };

  errorHistory.push(record);

  // 保持历史记录在限制范围内
  if (errorHistory.length > MAX_ERROR_HISTORY) {
    errorHistory.shift();
  }

  // 根据严重性级别记录日志
  switch (error.severity) {
    case ErrorSeverity.DEBUG:
      logger.debug(error.message, error.toJSON());
      break;
    case ErrorSeverity.INFO:
      logger.info(error.message, error.toJSON());
      break;
    case ErrorSeverity.WARNING:
      logger.warn(error.message, error.toJSON());
      break;
    case ErrorSeverity.ERROR:
      logger.error(error.message, error.toJSON());
      break;
    case ErrorSeverity.CRITICAL:
      logger.error(`[CRITICAL] ${error.message}`, error.toJSON());
      break;
    case ErrorSeverity.FATAL:
      logger.error(`[FATAL] ${error.message}`, error.toJSON());
      break;
  }
}

/**
 * 获取错误统计信息
 */
export function getErrorStatistics(): {
  total: number;
  bySeverity: Record<string, number>;
  byCategory: Record<string, number>;
  recent: number;
} {
  const bySeverity: Record<string, number> = {};
  const byCategory: Record<string, number> = {};
  const now = Date.now();
  const oneHourAgo = now - 3600000;

  errorHistory.forEach(record => {
    if (!record.resolved) {
      bySeverity[record.error.severity] = (bySeverity[record.error.severity] || 0) + 1;
      byCategory[record.error.category] = (byCategory[record.error.category] || 0) + 1;
    }

    if (record.timestamp > oneHourAgo) {
      bySeverity['recent'] = (bySeverity['recent'] || 0) + 1;
    }
  });

  return {
    total: errorHistory.filter(r => !r.resolved).length,
    bySeverity,
    byCategory,
    recent: bySeverity['recent'] || 0
  };
}

/**
 * 处理错误并返回用户友好的消息（航空航天标准）
 */
export function handleError(error: unknown, context: string): string {
  let message = ERROR_MESSAGES[ErrorCode.UNKNOWN_ERROR];
  let severity: ErrorSeverity = ErrorSeverity.ERROR;

  if (error instanceof AppError) {
    message = error.message;
    const _code = error.code;
    severity = error.severity;
    const _category = error.category;

    // 记录错误
    recordError(error);

    // 根据严重性决定是否需要紧急处理
    if (severity === ErrorSeverity.CRITICAL || severity === ErrorSeverity.FATAL) {
      // 触发紧急警报
      triggerCriticalAlert(error);
    }
  } else if (error instanceof Error) {
    message = error.message || ERROR_MESSAGES[ErrorCode.UNKNOWN_ERROR];

    // 转换为AppError
    const appError = new AppError(
      message,
      ErrorCode.INTERNAL_ERROR,
      ErrorSeverity.ERROR,
      ErrorCategory.SYSTEM,
      {
        timestamp: Date.now(),
        component: context,
        stackTrace: error.stack
      }
    );
    recordError(appError);
  } else if (typeof error === 'string') {
    message = error;

    const appError = new AppError(
      message,
      ErrorCode.UNKNOWN_ERROR,
      ErrorSeverity.WARNING,
      ErrorCategory.SYSTEM,
      {
        timestamp: Date.now(),
        component: context
      }
    );
    recordError(appError);
  } else {
    const appError = new AppError(
      ERROR_MESSAGES[ErrorCode.UNKNOWN_ERROR],
      ErrorCode.UNKNOWN_ERROR,
      ErrorSeverity.ERROR,
      ErrorCategory.SYSTEM,
      {
        timestamp: Date.now(),
        component: context,
        additionalData: { originalError: error }
      }
    );
    recordError(appError);
  }

  return message;
}

/**
 * 触发严重错误警报
 */
function triggerCriticalAlert(error: AppError): void {
  // 在实际应用中，这里应该：
  // 1. 发送警报到监控系统
  // 2. 通知运维团队
  // 3. 记录到专门的错误追踪系统
  // 4. 可能触发自动恢复机制

  logger.error(`[CRITICAL ALERT] ${error.code}: ${error.message}`, {
    severity: error.severity,
    category: error.category,
    context: error.context,
    timestamp: Date.now()
  });

  // 在浏览器环境中，可以显示特殊的错误界面
  if (typeof window !== 'undefined') {
    logger.error('CRITICAL ERROR DETECTED', error, LogCategory.SYSTEM);
  }
}

/**
 * 包装异步函数，自动处理错误（航空航天标准）
 */
export async function withErrorHandling<T>(
  fn: () => Promise<T>,
  context: string,
  options?: {
    fallback?: T;
    showNotification?: boolean;
    rethrow?: boolean;
    severity?: ErrorSeverity;
    category?: ErrorCategory;
    onRetry?: (attempt: number, error: any) => void;
    maxRetries?: number;
  }
): Promise<T | undefined> {
  const { maxRetries = 0, onRetry } = options || {};
  let __lastError: any;

  for (let attempt = 1; attempt <= (maxRetries + 1); attempt++) {
    try {
      return await fn();
    } catch (error) {
      __lastError = error;

      if (attempt <= maxRetries) {
        if (onRetry) {
          onRetry(attempt, error);
        }

        // 指数退避
        const delay = Math.min(1000 * Math.pow(2, attempt - 1), 10000);
        await new Promise(resolve => setTimeout(resolve, delay));
        continue;
      }

      const message = handleError(error, context);

      // 显示通知（如果需要）
      if (options?.showNotification) {
        showNotification(message, 'error');
      }

      // 重新抛出错误（如果需要）
      if (options?.rethrow) {
        throw error;
      }

      return options?.fallback;
    }
  }

  return options?.fallback;
}

/**
 * 包装同步函数，自动处理错误
 */
export function withErrorHandlingSync<T>(
  fn: () => T,
  context: string,
  options?: {
    fallback?: T;
    showNotification?: boolean;
    rethrow?: boolean;
  }
): T | undefined {
  try {
    return fn();
  } catch (error) {
    const message = handleError(error, context);

    if (options?.showNotification) {
      showNotification(message, 'error');
    }

    if (options?.rethrow) {
      throw error;
    }

    return options?.fallback;
  }
}

/**
 * 创建应用错误（航空航天标准）
 */
export function createError(
  code: ErrorCode | string,
  customMessage?: string,
  severity: ErrorSeverity = ErrorSeverity.ERROR,
  category: ErrorCategory = ErrorCategory.SYSTEM,
  context?: ErrorContext
): AppError {
  const message = customMessage || ERROR_MESSAGES[code] || ERROR_MESSAGES[ErrorCode.UNKNOWN_ERROR];
  return new AppError(message, code, severity, category, context);
}

/**
 * 验证函数（航空航天标准）
 */
export function validate<T>(
  value: T,
  validator: (value: T) => boolean,
  errorMessage: string,
  severity: ErrorSeverity = ErrorSeverity.ERROR
): T {
  if (!validator(value)) {
    throw createError(ErrorCode.VALIDATION_ERROR, errorMessage, severity, ErrorCategory.VALIDATION);
  }
  return value;
}

/**
 * 断言函数（航空航天标准）
 */
export function assert(
  condition: boolean,
  message: string,
  code: ErrorCode = ErrorCode.INTERNAL_ERROR,
  severity: ErrorSeverity = ErrorSeverity.CRITICAL
): asserts condition {
  if (!condition) {
    throw createError(code, message, severity, ErrorCategory.SYSTEM);
  }
}

/**
 * 重试函数（航空航天标准）
 */
export async function retry<T>(
  fn: () => Promise<T>,
  options: {
    maxAttempts?: number;
    delay?: number;
    backoff?: boolean;
    onRetry?: (attempt: number, error: any) => void;
    shouldRetry?: (error: any) => boolean;
  } = {}
): Promise<T> {
  const { maxAttempts = 3, delay = 1000, backoff = true, onRetry, shouldRetry } = options;

  let __lastError: any;

  for (let attempt = 1; attempt <= maxAttempts; attempt++) {
    try {
      return await fn();
    } catch (error) {
      __lastError = error;

      // 检查是否应该重试
      if (shouldRetry && !shouldRetry(error)) {
        throw error;
      }

      if (attempt < maxAttempts) {
        const waitTime = backoff ? delay * Math.pow(2, attempt - 1) : delay;

        if (onRetry) {
          onRetry(attempt, error);
        }

        logger.warn(`Retry attempt ${attempt}/${maxAttempts} after ${waitTime}ms`, error);
        await new Promise(resolve => setTimeout(resolve, waitTime));
      }
    }
  }

  throw _lastError;
}

/**
 * 超时函数（航空航天标准）
 */
export async function withTimeout<T>(
  fn: () => Promise<T>,
  timeoutMs: number,
  errorMessage?: string,
  severity: ErrorSeverity = ErrorSeverity.ERROR
): Promise<T> {
  return Promise.race([
    fn(),
    new Promise<T>((_, reject) =>
      setTimeout(
        () =>
          reject(
            createError(
              ErrorCode.TIMEOUT_ERROR,
              errorMessage || `操作超时 (${timeoutMs}ms)`,
              severity,
              ErrorCategory.PERFORMANCE
            )
          ),
        timeoutMs
      )
    )
  ]);
}

/**
 * 断路器模式（航空航天标准）
 * 用于防止级联故障
 */
export class CircuitBreaker {
  private failureCount = 0;
  private lastFailureTime = 0;
  private state: 'closed' | 'open' | 'half-open' = 'closed';

  constructor(
    private threshold: number = 5,
    private timeout: number = 60000,
    private resetTimeout: number = 30000
  ) {}

  async execute<T>(fn: () => Promise<T>): Promise<T> {
    if (this.state === 'open') {
      if (Date.now() - this.lastFailureTime > this.resetTimeout) {
        this.state = 'half-open';
      } else {
        throw createError(
          ErrorCode.OPERATION_FAILED,
          '断路器已打开，服务暂时不可用',
          ErrorSeverity.WARNING,
          ErrorCategory.SYSTEM
        );
      }
    }

    try {
      const result = await fn();
      this.onSuccess();
      return result;
    } catch (error) {
      this.onFailure();
      throw error;
    }
  }

  private onSuccess(): void {
    this.failureCount = 0;
    this.state = 'closed';
  }

  private onFailure(): void {
    this.failureCount++;
    this.lastFailureTime = Date.now();

    if (this.failureCount >= this.threshold) {
      this.state = 'open';
      logger.error(`Circuit breaker opened after ${this.failureCount} failures`);
    }
  }

  getState(): string {
    return this.state;
  }

  reset(): void {
    this.failureCount = 0;
    this.state = 'closed';
    this.lastFailureTime = 0;
  }
}

/**
 * 健康检查接口（航空航天标准）
 */
export interface HealthCheck {
  name: string;
  check: () => Promise<boolean>;
  critical: boolean;
}

const healthChecks: HealthCheck[] = [];

/**
 * 注册健康检查
 */
export function registerHealthCheck(check: HealthCheck): void {
  healthChecks.push(check);
}

/**
 * 清除所有健康检查
 */
export function clearHealthChecks(): void {
  healthChecks.length = 0;
}

/**
 * 执行所有健康检查
 */
export async function performHealthChecks(): Promise<{
  healthy: boolean;
  checks: Array<{ name: string; healthy: boolean; critical: boolean }>;
}> {
  const results = await Promise.all(
    healthChecks.map(async check => ({
      name: check.name,
      healthy: await check.check().catch(() => false),
      critical: check.critical
    }))
  );

  const healthy = results.every(r => !r.critical || r.healthy);

  return {
    healthy,
    checks: results
  };
}

/**
 * 清除错误历史
 */
export function clearErrorHistory(): void {
  errorHistory.length = 0;
  logger.info('Error history cleared');
}

/**
 * 导出错误历史（用于调试和分析）
 */
export function exportErrorHistory(): ErrorRecord[] {
  return [...errorHistory];
}
