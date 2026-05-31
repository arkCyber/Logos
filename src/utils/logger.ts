/**
 * 航空航天级日志记录系统
 * 提供完整的日志记录、分析、追踪和报告功能
 */

/**
 * 日志级别（航空航天标准）
 */
export enum LogLevel {
  TRACE = 'trace',         // 最详细的追踪信息
  DEBUG = 'debug',         // 调试信息
  INFO = 'info',           // 一般信息
  WARNING = 'warning',     // 警告信息
  ERROR = 'error',         // 错误信息
  CRITICAL = 'critical',   // 严重错误
  FATAL = 'fatal'          // 致命错误
}

/**
 * 日志类别
 */
export enum LogCategory {
  SYSTEM = 'system',         // 系统日志
  NETWORK = 'network',       // 网络日志
  DATABASE = 'database',     // 数据库日志
  SECURITY = 'security',     // 安全日志
  PERFORMANCE = 'performance', // 性能日志
  BUSINESS = 'business',    // 业务日志
  USER_ACTION = 'user_action', // 用户操作日志
  API = 'api',              // API日志
  UI = 'ui',                // UI日志
  BACKGROUND = 'background'  // 后台任务日志
}

/**
 * 日志条目接口
 */
export interface LogEntry {
  id: string;
  level: LogLevel;
  category: LogCategory;
  message: string;
  timestamp: number;
  data?: any;
  stack?: string;
  userId?: string;
  sessionId?: string;
  requestId?: string;
  component?: string;
  action?: string;
  duration?: number;
  metadata?: Record<string, any>;
}

/**
 * 日志配置接口
 */
export interface LoggerConfig {
  maxLogs: number;
  maxMemoryLogs: number;
  enableConsole: boolean;
  enableFileLogging: boolean;
  enableRemoteLogging: boolean;
  logLevel: LogLevel;
  categories: LogCategory[];
  enableCompression: boolean;
  enableEncryption: boolean;
  retentionDays: number;
}

/**
 * 默认配置
 */
const DEFAULT_CONFIG: LoggerConfig = {
  maxLogs: 10000,
  maxMemoryLogs: 1000,
  enableConsole: true,
  enableFileLogging: false,
  enableRemoteLogging: false,
  logLevel: LogLevel.INFO,
  categories: Object.values(LogCategory),
  enableCompression: false,
  enableEncryption: false,
  retentionDays: 30
};

/**
 * Sentry集成接口
 */
interface SentryIntegration {
  captureException(error: unknown, context?: any): void;
  captureMessage(message: string, level?: string): void;
  configureScope(callback: (scope: any) => void): void;
}

/**
 * 航空航天级日志记录器
 */
class Logger {
  private logs: LogEntry[] = [];
  private config: LoggerConfig;
  private isDev = import.meta.env.DEV;
  private sentry: SentryIntegration | null = null;
  private sentryEnabled = false;
  private performanceMarks: Map<string, number> = new Map();
  private logCounters: Map<string, number> = new Map();

  constructor(config: Partial<LoggerConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.initialize();
  }

  /**
   * 初始化日志系统
   */
  private initialize(): void {
    this.info('Logger initialized', {
      config: this.config,
      environment: this.isDev ? 'development' : 'production'
    });

    // 设置定期日志清理
    if (typeof window !== 'undefined') {
      setInterval(() => this.cleanupOldLogs(), 3600000); // 每小时清理一次
    }
  }

  /**
   * 更新配置
   */
  updateConfig(config: Partial<LoggerConfig>): void {
    this.config = { ...this.config, ...config };
    this.info('Logger configuration updated', { config: this.config });
  }

  /**
   * 生成唯一ID
   */
  private generateId(): string {
    return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  /**
   * 检查是否应该记录此级别的日志
   */
  private shouldLog(level: LogLevel, category: LogCategory): boolean {
    const levelPriority = {
      [LogLevel.TRACE]: 0,
      [LogLevel.DEBUG]: 1,
      [LogLevel.INFO]: 2,
      [LogLevel.WARNING]: 3,
      [LogLevel.ERROR]: 4,
      [LogLevel.CRITICAL]: 5,
      [LogLevel.FATAL]: 6
    };

    return (
      levelPriority[level] >= levelPriority[this.config.logLevel] &&
      this.config.categories.includes(category)
    );
  }

  /**
   * 记录日志
   */
  private log(
    level: LogLevel,
    category: LogCategory,
    message: string,
    data?: any,
    stack?: string,
    metadata?: Record<string, any>
  ): void {
    if (!this.shouldLog(level, category)) {
      return;
    }

    const entry: LogEntry = {
      id: this.generateId(),
      level,
      category,
      message,
      timestamp: Date.now(),
      data,
      stack,
      metadata
    };

    this.logs.push(entry);

    // 更新计数器
    const counterKey = `${level}_${category}`;
    this.logCounters.set(counterKey, (this.logCounters.get(counterKey) || 0) + 1);

    // 限制内存中的日志数量
    if (this.logs.length > this.config.maxMemoryLogs) {
      this.logs.shift();
    }

    // 控制台输出
    if (this.config.enableConsole) {
      this.outputToConsole(entry);
    }

    // 发送到远程日志服务
    if (this.config.enableRemoteLogging && level >= LogLevel.ERROR) {
      this.sendToRemoteLogging(entry);
    }
  }

  /**
   * 输出到控制台
   */
  private outputToConsole(entry: LogEntry): void {
    const colors = {
      [LogLevel.TRACE]: '#888',
      [LogLevel.DEBUG]: '#888',
      [LogLevel.INFO]: '#0078D4',
      [LogLevel.WARNING]: '#F7630C',
      [LogLevel.ERROR]: '#A4262C',
      [LogLevel.CRITICAL]: '#D13438',
      [LogLevel.FATAL]: '#000000'
    };

    const color = colors[entry.level];
    const prefix = `[${entry.level.toUpperCase()}] [${entry.category}]`;

    const consoleMethod = {
      [LogLevel.TRACE]: console.log,
      [LogLevel.DEBUG]: console.log,
      [LogLevel.INFO]: console.info,
      [LogLevel.WARNING]: console.warn,
      [LogLevel.ERROR]: console.error,
      [LogLevel.CRITICAL]: console.error,
      [LogLevel.FATAL]: console.error
    }[entry.level];

    consoleMethod(
      `%c${prefix} ${entry.message}`,
      `color: ${color}; font-weight: bold;`,
      entry.data || '',
      entry.metadata || ''
    );

    if (entry.stack) {
      console.error(entry.stack);
    }
  }

  /**
   * 发送到远程日志服务
   */
  private sendToRemoteLogging(entry: LogEntry): void {
    // 在实际应用中，这里应该发送到日志聚合服务
    // 如 ELK, Splunk, CloudWatch 等
    if (this.isDev) {
      console.log('[Remote Logging]', entry);
    }
  }

  /**
   * 清理旧日志
   */
  private cleanupOldLogs(): void {
    const cutoffTime = Date.now() - (this.config.retentionDays * 24 * 60 * 60 * 1000);
    const beforeCount = this.logs.length;
    this.logs = this.logs.filter(log => log.timestamp > cutoffTime);
    const afterCount = this.logs.length;

    if (beforeCount !== afterCount) {
      this.info(`Cleaned up ${beforeCount - afterCount} old log entries`);
    }
  }

  /**
   * TRACE级别日志
   */
  trace(message: string, data?: any, category: LogCategory = LogCategory.SYSTEM): void {
    this.log(LogLevel.TRACE, category, message, data);
  }

  /**
   * DEBUG级别日志
   */
  debug(message: string, data?: any, category: LogCategory = LogCategory.SYSTEM): void {
    this.log(LogLevel.DEBUG, category, message, data);
  }

  /**
   * INFO级别日志
   */
  info(message: string, data?: any, category: LogCategory = LogCategory.SYSTEM): void {
    this.log(LogLevel.INFO, category, message, data);
  }

  /**
   * WARNING级别日志
   */
  warn(message: string, data?: any, category: LogCategory = LogCategory.SYSTEM): void {
    this.log(LogLevel.WARNING, category, message, data);
  }

  /**
   * ERROR级别日志
   */
  error(message: string, error?: Error | any, category: LogCategory = LogCategory.SYSTEM): void {
    const stack = error instanceof Error ? error.stack : undefined;
    this.log(LogLevel.ERROR, category, message, error, stack);

    // 发送到错误追踪服务
    if (!this.isDev) {
      this.sendToErrorTracking(message, error);
    }
  }

  /**
   * CRITICAL级别日志
   */
  critical(message: string, error?: Error | any, category: LogCategory = LogCategory.SYSTEM): void {
    const stack = error instanceof Error ? error.stack : undefined;
    this.log(LogLevel.CRITICAL, category, message, error, stack);

    if (!this.isDev) {
      this.sendToErrorTracking(message, error);
    }
  }

  /**
   * FATAL级别日志
   */
  fatal(message: string, error?: Error | any, category: LogCategory = LogCategory.SYSTEM): void {
    const stack = error instanceof Error ? error.stack : undefined;
    this.log(LogLevel.FATAL, category, message, error, stack);

    if (!this.isDev) {
      this.sendToErrorTracking(message, error);
    }
  }

  /**
   * 用户操作日志
   */
  userAction(action: string, data?: any): void {
    this.log(LogLevel.INFO, LogCategory.USER_ACTION, action, data, undefined, { action });
  }

  /**
   * 性能测量开始
   */
  startPerformanceMark(name: string): void {
    this.performanceMarks.set(name, performance.now());
  }

  /**
   * 性能测量结束
   */
  endPerformanceMark(name: string, category: LogCategory = LogCategory.PERFORMANCE): void {
    const startTime = this.performanceMarks.get(name);
    if (startTime === undefined) {
      this.warn(`Performance mark "${name}" not found`, {}, category);
      return;
    }

    const endTime = performance.now();
    const duration = endTime - startTime;
    this.performanceMarks.delete(name);

    this.log(
      LogLevel.INFO,
      category,
      `Performance: ${name}`,
      { duration: `${duration.toFixed(2)}ms` },
      undefined,
      { performance: true, duration }
    );
  }

  /**
   * 测量函数执行时间
   */
  measure<T>(name: string, fn: () => T, category: LogCategory = LogCategory.PERFORMANCE): T {
    this.startPerformanceMark(name);
    try {
      return fn();
    } finally {
      this.endPerformanceMark(name, category);
    }
  }

  /**
   * 测量异步函数执行时间
   */
  async measureAsync<T>(
    name: string,
    fn: () => Promise<T>,
    category: LogCategory = LogCategory.PERFORMANCE
  ): Promise<T> {
    this.startPerformanceMark(name);
    try {
      return await fn();
    } finally {
      this.endPerformanceMark(name, category);
    }
  }

  /**
   * 初始化Sentry集成
   */
  initSentry(sentryInstance: SentryIntegration): void {
    this.sentry = sentryInstance;
    this.sentryEnabled = true;
    this.info('Sentry integration initialized', {}, LogCategory.SYSTEM);
  }

  /**
   * 禁用Sentry集成
   */
  disableSentry(): void {
    this.sentry = null;
    this.sentryEnabled = false;
    this.info('Sentry integration disabled', {}, LogCategory.SYSTEM);
  }

  /**
   * 发送到错误追踪服务
   */
  private sendToErrorTracking(message: string, error?: unknown): void {
    if (!this.sentryEnabled || !this.sentry) {
      return;
    }

    try {
      if (error instanceof Error) {
        this.sentry.captureException(error, {
          tags: { component: 'logger' },
          extra: { message }
        });
      } else {
        this.sentry.captureMessage(message, 'error');
      }
    } catch (e) {
      console.error('Failed to send error to Sentry:', e);
    }
  }

  /**
   * 获取所有日志
   */
  getLogs(): LogEntry[] {
    return [...this.logs];
  }

  /**
   * 获取特定级别的日志
   */
  getLogsByLevel(level: LogLevel): LogEntry[] {
    return this.logs.filter(log => log.level === level);
  }

  /**
   * 获取特定类别的日志
   */
  getLogsByCategory(category: LogCategory): LogEntry[] {
    return this.logs.filter(log => log.category === category);
  }

  /**
   * 获取时间范围内的日志
   */
  getLogsByTimeRange(startTime: number, endTime: number): LogEntry[] {
    return this.logs.filter(log => log.timestamp >= startTime && log.timestamp <= endTime);
  }

  /**
   * 搜索日志
   */
  searchLogs(query: string): LogEntry[] {
    const lowerQuery = query.toLowerCase();
    return this.logs.filter(
      log =>
        log.message.toLowerCase().includes(lowerQuery) ||
        JSON.stringify(log.data || '').toLowerCase().includes(lowerQuery)
    );
  }

  /**
   * 获取日志统计信息
   */
  getLogStatistics(): {
    total: number;
    byLevel: Record<LogLevel, number>;
    byCategory: Record<LogCategory, number>;
    recent: number;
  } {
    const byLevel: Record<LogLevel, number> = {
      [LogLevel.TRACE]: 0,
      [LogLevel.DEBUG]: 0,
      [LogLevel.INFO]: 0,
      [LogLevel.WARNING]: 0,
      [LogLevel.ERROR]: 0,
      [LogLevel.CRITICAL]: 0,
      [LogLevel.FATAL]: 0
    };

    const byCategory: Record<LogCategory, number> = {
      [LogLevel.TRACE]: 0,
      [LogLevel.DEBUG]: 0,
      [LogLevel.INFO]: 0,
      [LogLevel.WARNING]: 0,
      [LogLevel.ERROR]: 0,
      [LogLevel.CRITICAL]: 0,
      [LogLevel.FATAL]: 0
    } as any;

    const oneHourAgo = Date.now() - 3600000;

    this.logs.forEach(log => {
      byLevel[log.level]++;
      byCategory[log.category] = (byCategory[log.category] || 0) + 1;
    });

    const recent = this.logs.filter(log => log.timestamp > oneHourAgo).length;

    return {
      total: this.logs.length,
      byLevel,
      byCategory,
      recent
    };
  }

  /**
   * 清除日志
   */
  clearLogs(): void {
    this.logs = [];
    this.logCounters.clear();
    this.info('Logs cleared', {}, LogCategory.SYSTEM);
  }

  /**
   * 导出日志为JSON
   */
  exportLogs(format: 'json' | 'csv' = 'json'): string {
    if (format === 'json') {
      return JSON.stringify(this.logs, null, 2);
    } else if (format === 'csv') {
      const headers = ['id', 'level', 'category', 'message', 'timestamp', 'data'];
      const rows = this.logs.map(log =>
        headers.map(header => {
          const value = log[header as keyof LogEntry];
          return typeof value === 'string' ? `"${value.replace(/"/g, '""')}"` : value;
        }).join(',')
      );
      return [headers.join(','), ...rows].join('\n');
    }
    return '';
  }

  /**
   * 下载日志文件
   */
  downloadLogs(format: 'json' | 'csv' = 'json'): void {
    const content = this.exportLogs(format);
    const mimeType = format === 'json' ? 'application/json' : 'text/csv';
    const blob = new Blob([content], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `logs-${Date.now()}.${format}`;
    a.click();
    URL.revokeObjectURL(url);
  }

  /**
   * 创建日志快照
   */
  createSnapshot(): string {
    return JSON.stringify({
      timestamp: Date.now(),
      statistics: this.getLogStatistics(),
      recentLogs: this.logs.slice(-100)
    });
  }
}

// 导出单例
export const logger = new Logger();

// 导出便捷函数
export const log = {
  trace: (msg: string, data?: any, category?: LogCategory) => logger.trace(msg, data, category),
  debug: (msg: string, data?: any, category?: LogCategory) => logger.debug(msg, data, category),
  info: (msg: string, data?: any, category?: LogCategory) => logger.info(msg, data, category),
  warn: (msg: string, data?: any, category?: LogCategory) => logger.warn(msg, data, category),
  error: (msg: string, error?: any, category?: LogCategory) => logger.error(msg, error, category),
  critical: (msg: string, error?: any, category?: LogCategory) => logger.critical(msg, error, category),
  fatal: (msg: string, error?: any, category?: LogCategory) => logger.fatal(msg, error, category),
  userAction: (action: string, data?: any) => logger.userAction(action, data),
  measure: <T>(name: string, fn: () => T, category?: LogCategory) => logger.measure(name, fn, category),
  measureAsync: <T>(name: string, fn: () => Promise<T>, category?: LogCategory) =>
    logger.measureAsync(name, fn, category),
  initSentry: (sentryInstance: SentryIntegration) => logger.initSentry(sentryInstance),
  disableSentry: () => logger.disableSentry(),
  getLogs: () => logger.getLogs(),
  getStatistics: () => logger.getLogStatistics(),
  clearLogs: () => logger.clearLogs(),
  exportLogs: (format?: 'json' | 'csv') => logger.exportLogs(format),
  downloadLogs: (format?: 'json' | 'csv') => logger.downloadLogs(format)
};
