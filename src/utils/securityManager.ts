/**
 * 航空航天级安全管理和权限控制系统
 * 提供完整的安全防护、权限控制和审计功能
 */

import { logger, LogCategory } from './logger';
import { createError, ErrorCode, ErrorSeverity, ErrorCategory } from './errorHandler';

/**
 * 权限级别
 */
export enum PermissionLevel {
  NONE = 'none',
  READ = 'read',
  WRITE = 'write',
  ADMIN = 'admin',
  SUPER_ADMIN = 'super_admin'
}

/**
 * 资源类型
 */
export enum ResourceType {
  DOCUMENT = 'document',
  FILE = 'file',
  SETTINGS = 'settings',
  USER = 'user',
  SYSTEM = 'system',
  API = 'api',
  DATABASE = 'database',
  LOGS = 'logs'
}

/**
 * 操作类型
 */
export enum ActionType {
  READ = 'read',
  WRITE = 'write',
  DELETE = 'delete',
  EXECUTE = 'execute',
  MANAGE = 'manage',
  EXPORT = 'export',
  IMPORT = 'import'
}

/**
 * 安全事件类型
 */
export enum SecurityEventType {
  AUTHENTICATION = 'authentication',
  AUTHORIZATION = 'authorization',
  PERMISSION_DENIED = 'permission_denied',
  DATA_ACCESS = 'data_access',
  DATA_MODIFICATION = 'data_modification',
  SECURITY_VIOLATION = 'security_violation',
  SUSPICIOUS_ACTIVITY = 'suspicious_activity'
}

/**
 * 用户角色
 */
export enum UserRole {
  GUEST = 'guest',
  USER = 'user',
  EDITOR = 'editor',
  REVIEWER = 'reviewer',
  ADMIN = 'admin',
  SUPER_ADMIN = 'super_admin'
}

/**
 * 权限接口
 */
export interface Permission {
  resource: ResourceType;
  action: ActionType;
  level: PermissionLevel;
  conditions?: Record<string, any>;
}

/**
 * 安全事件接口
 */
export interface SecurityEvent {
  id: string;
  type: SecurityEventType;
  userId?: string;
  sessionId?: string;
  resource?: ResourceType;
  action?: ActionType;
  success: boolean;
  timestamp: number;
  details?: Record<string, any>;
  ipAddress?: string;
  userAgent?: string;
}

/**
 * 会话接口
 */
export interface Session {
  id: string;
  userId: string;
  role: UserRole;
  createdAt: number;
  lastActivity: number;
  expiresAt: number;
  ipAddress?: string;
  userAgent?: string;
  metadata?: Record<string, any>;
}

/**
 * 安全策略接口
 */
export interface SecurityPolicy {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
  rules: SecurityRule[];
}

/**
 * 安全规则接口
 */
export interface SecurityRule {
  type: 'rate_limit' | 'ip_whitelist' | 'ip_blacklist' | 'time_based' | 'custom';
  config: Record<string, any>;
  action: 'allow' | 'deny' | 'log' | 'alert';
}

/**
 * 安全配置接口
 */
export interface SecurityConfig {
  enableAuthentication: boolean;
  enableAuthorization: boolean;
  enableAuditLogging: boolean;
  sessionTimeout: number;
  maxFailedAttempts: number;
  lockoutDuration: number;
  enableRateLimiting: boolean;
  rateLimitWindow: number;
  rateLimitMax: number;
  enableIPFiltering: boolean;
  ipWhitelist: string[];
  ipBlacklist: string[];
  enableEncryption: boolean;
  encryptionKey?: string;
}

/**
 * 默认配置
 */
const DEFAULT_CONFIG: SecurityConfig = {
  enableAuthentication: true,
  enableAuthorization: true,
  enableAuditLogging: true,
  sessionTimeout: 3600000, // 1小时
  maxFailedAttempts: 5,
  lockoutDuration: 900000, // 15分钟
  enableRateLimiting: true,
  rateLimitWindow: 60000, // 1分钟
  rateLimitMax: 100,
  enableIPFiltering: false,
  ipWhitelist: [],
  ipBlacklist: [],
  enableEncryption: false
};

/**
 * 角色权限映射
 */
const ROLE_PERMISSIONS: Record<UserRole, Permission[]> = {
  [UserRole.GUEST]: [
    { resource: ResourceType.DOCUMENT, action: ActionType.READ, level: PermissionLevel.READ }
  ],
  [UserRole.USER]: [
    { resource: ResourceType.DOCUMENT, action: ActionType.READ, level: PermissionLevel.READ },
    { resource: ResourceType.DOCUMENT, action: ActionType.WRITE, level: PermissionLevel.WRITE },
    { resource: ResourceType.FILE, action: ActionType.READ, level: PermissionLevel.READ },
    { resource: ResourceType.FILE, action: ActionType.WRITE, level: PermissionLevel.WRITE }
  ],
  [UserRole.EDITOR]: [
    { resource: ResourceType.DOCUMENT, action: ActionType.READ, level: PermissionLevel.READ },
    { resource: ResourceType.DOCUMENT, action: ActionType.WRITE, level: PermissionLevel.WRITE },
    { resource: ResourceType.DOCUMENT, action: ActionType.DELETE, level: PermissionLevel.WRITE },
    { resource: ResourceType.FILE, action: ActionType.READ, level: PermissionLevel.READ },
    { resource: ResourceType.FILE, action: ActionType.WRITE, level: PermissionLevel.WRITE },
    { resource: ResourceType.FILE, action: ActionType.DELETE, level: PermissionLevel.WRITE },
    { resource: ResourceType.SETTINGS, action: ActionType.READ, level: PermissionLevel.READ }
  ],
  [UserRole.REVIEWER]: [
    { resource: ResourceType.DOCUMENT, action: ActionType.READ, level: PermissionLevel.READ },
    { resource: ResourceType.DOCUMENT, action: ActionType.WRITE, level: PermissionLevel.WRITE },
    { resource: ResourceType.FILE, action: ActionType.READ, level: PermissionLevel.READ },
    { resource: ResourceType.SETTINGS, action: ActionType.READ, level: PermissionLevel.READ }
  ],
  [UserRole.ADMIN]: [
    { resource: ResourceType.DOCUMENT, action: ActionType.READ, level: PermissionLevel.ADMIN },
    { resource: ResourceType.DOCUMENT, action: ActionType.WRITE, level: PermissionLevel.ADMIN },
    { resource: ResourceType.DOCUMENT, action: ActionType.DELETE, level: PermissionLevel.ADMIN },
    { resource: ResourceType.FILE, action: ActionType.READ, level: PermissionLevel.ADMIN },
    { resource: ResourceType.FILE, action: ActionType.WRITE, level: PermissionLevel.ADMIN },
    { resource: ResourceType.FILE, action: ActionType.DELETE, level: PermissionLevel.ADMIN },
    { resource: ResourceType.SETTINGS, action: ActionType.READ, level: PermissionLevel.ADMIN },
    { resource: ResourceType.SETTINGS, action: ActionType.WRITE, level: PermissionLevel.ADMIN },
    { resource: ResourceType.USER, action: ActionType.READ, level: PermissionLevel.ADMIN },
    { resource: ResourceType.USER, action: ActionType.WRITE, level: PermissionLevel.ADMIN }
  ],
  [UserRole.SUPER_ADMIN]: [
    { resource: ResourceType.DOCUMENT, action: ActionType.READ, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.DOCUMENT, action: ActionType.WRITE, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.DOCUMENT, action: ActionType.DELETE, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.FILE, action: ActionType.READ, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.FILE, action: ActionType.WRITE, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.FILE, action: ActionType.DELETE, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.SETTINGS, action: ActionType.READ, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.SETTINGS, action: ActionType.WRITE, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.USER, action: ActionType.READ, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.USER, action: ActionType.WRITE, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.USER, action: ActionType.DELETE, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.SYSTEM, action: ActionType.READ, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.SYSTEM, action: ActionType.WRITE, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.API, action: ActionType.MANAGE, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.DATABASE, action: ActionType.MANAGE, level: PermissionLevel.SUPER_ADMIN },
    { resource: ResourceType.LOGS, action: ActionType.READ, level: PermissionLevel.SUPER_ADMIN }
  ]
};

/**
 * 航空航天级安全管理器
 */
export class SecurityManager {
  private config: SecurityConfig;
  private sessions: Map<string, Session> = new Map();
  private securityEvents: SecurityEvent[] = [];
  private failedAttempts: Map<string, number> = new Map();
  private lockouts: Map<string, number> = new Map();
  private rateLimitTracker: Map<string, number[]> = new Map();
  private securityPolicies: SecurityPolicy[] = [];
  private currentSession: Session | null = null;

  constructor(config: Partial<SecurityConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.initialize();
  }

  /**
   * 初始化安全管理器
   */
  private initialize(): void {
    logger.info('Security manager initialized', { config: this.config }, LogCategory.SECURITY);

    // 设置定期清理
    if (typeof window !== 'undefined') {
      setInterval(() => this.cleanupExpiredSessions(), 60000); // 每分钟清理一次
      setInterval(() => this.cleanupOldEvents(), 3600000); // 每小时清理一次
    }
  }

  /**
   * 生成会话ID
   */
  private generateSessionId(): string {
    return `session-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  /**
   * 生成事件ID
   */
  private generateEventId(): string {
    return `event-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  /**
   * 获取客户端IP地址
   */
  private getClientIP(): string {
    // 在实际应用中，这里应该从请求头获取真实IP
    return '127.0.0.1';
  }

  /**
   * 获取用户代理
   */
  private getUserAgent(): string {
    return navigator.userAgent;
  }

  /**
   * 创建会话
   */
  createSession(userId: string, role: UserRole, metadata?: Record<string, any>): Session {
    const now = Date.now();
    const session: Session = {
      id: this.generateSessionId(),
      userId,
      role,
      createdAt: now,
      lastActivity: now,
      expiresAt: now + this.config.sessionTimeout,
      ipAddress: this.getClientIP(),
      userAgent: this.getUserAgent(),
      metadata
    };

    this.sessions.set(session.id, session);
    this.currentSession = session;

    this.logSecurityEvent(SecurityEventType.AUTHENTICATION, true, { session });

    logger.info('Session created', { sessionId: session.id, userId, role }, LogCategory.SECURITY);

    return session;
  }

  /**
   * 验证会话
   */
  validateSession(sessionId: string): boolean {
    const session = this.sessions.get(sessionId);
    if (!session) {
      return false;
    }

    if (Date.now() > session.expiresAt) {
      this.sessions.delete(sessionId);
      this.logSecurityEvent(SecurityEventType.AUTHENTICATION, false, {
        reason: 'session_expired',
        sessionId
      });
      return false;
    }

    // 更新最后活动时间
    session.lastActivity = Date.now();
    this.currentSession = session;

    return true;
  }

  /**
   * 销毁会话
   */
  destroySession(sessionId: string): void {
    const session = this.sessions.get(sessionId);
    if (session) {
      this.sessions.delete(sessionId);
      this.logSecurityEvent(SecurityEventType.AUTHENTICATION, true, {
        action: 'logout',
        sessionId
      });

      if (this.currentSession?.id === sessionId) {
        this.currentSession = null;
      }

      logger.info('Session destroyed', { sessionId }, LogCategory.SECURITY);
    }
  }

  /**
   * 清理过期会话
   */
  private cleanupExpiredSessions(): void {
    const now = Date.now();
    let cleanedCount = 0;

    for (const [sessionId, session] of this.sessions.entries()) {
      if (now > session.expiresAt) {
        this.sessions.delete(sessionId);
        cleanedCount++;
      }
    }

    if (cleanedCount > 0) {
      logger.info(`Cleaned up ${cleanedCount} expired sessions`, {}, LogCategory.SECURITY);
    }
  }

  /**
   * 检查权限
   */
  checkPermission(
    resource: ResourceType,
    action: ActionType,
    requiredLevel: PermissionLevel = PermissionLevel.READ
  ): boolean {
    if (!this.config.enableAuthorization) {
      return true;
    }

    if (!this.currentSession) {
      this.logSecurityEvent(SecurityEventType.PERMISSION_DENIED, false, {
        resource,
        action,
        reason: 'no_session'
      });
      return false;
    }

    const role = this.currentSession.role;
    const permissions = ROLE_PERMISSIONS[role];

    for (const permission of permissions) {
      if (permission.resource === resource && permission.action === action) {
        const levelPriority = {
          [PermissionLevel.NONE]: 0,
          [PermissionLevel.READ]: 1,
          [PermissionLevel.WRITE]: 2,
          [PermissionLevel.ADMIN]: 3,
          [PermissionLevel.SUPER_ADMIN]: 4
        };

        if (levelPriority[permission.level] >= levelPriority[requiredLevel]) {
          this.logSecurityEvent(SecurityEventType.AUTHORIZATION, true, {
            resource,
            action,
            role
          });
          return true;
        }
      }
    }

    this.logSecurityEvent(SecurityEventType.PERMISSION_DENIED, false, {
      resource,
      action,
      role,
      requiredLevel
    });

    return false;
  }

  /**
   * 要求权限
   */
  requirePermission(
    resource: ResourceType,
    action: ActionType,
    requiredLevel: PermissionLevel = PermissionLevel.READ
  ): void {
    if (!this.checkPermission(resource, action, requiredLevel)) {
      throw createError(
        ErrorCode.PERMISSION_DENIED,
        `Permission denied: ${action} on ${resource}`,
        ErrorSeverity.ERROR,
        ErrorCategory.SECURITY
      );
    }
  }

  /**
   * 记录安全事件
   */
  private logSecurityEvent(
    type: SecurityEventType,
    success: boolean,
    details?: Record<string, any>
  ): void {
    if (!this.config.enableAuditLogging) {
      return;
    }

    const event: SecurityEvent = {
      id: this.generateEventId(),
      type,
      userId: this.currentSession?.userId,
      sessionId: this.currentSession?.id,
      success,
      timestamp: Date.now(),
      details,
      ipAddress: this.getClientIP(),
      userAgent: this.getUserAgent()
    };

    this.securityEvents.push(event);

    // 限制事件历史数量
    if (this.securityEvents.length > 10000) {
      this.securityEvents.shift();
    }

    // 记录到日志
    if (success) {
      logger.debug(`Security event: ${type}`, { event }, LogCategory.SECURITY);
    } else {
      logger.warn(`Security event (failed): ${type}`, { event }, LogCategory.SECURITY);
    }
  }

  /**
   * 清理旧事件
   */
  private cleanupOldEvents(): void {
    const cutoffTime = Date.now() - (30 * 24 * 60 * 60 * 1000); // 30天
    const beforeCount = this.securityEvents.length;
    this.securityEvents = this.securityEvents.filter(event => event.timestamp > cutoffTime);
    const afterCount = this.securityEvents.length;

    if (beforeCount !== afterCount) {
      logger.info(`Cleaned up ${beforeCount - afterCount} old security events`, {}, LogCategory.SECURITY);
    }
  }

  /**
   * 检查速率限制
   */
  checkRateLimit(identifier: string): boolean {
    if (!this.config.enableRateLimiting) {
      return true;
    }

    const now = Date.now();
    const windowStart = now - this.config.rateLimitWindow;

    if (!this.rateLimitTracker.has(identifier)) {
      this.rateLimitTracker.set(identifier, []);
    }

    const timestamps = this.rateLimitTracker.get(identifier);
    if (!timestamps) {
      this.rateLimitTracker.set(identifier, []);
      return true;
    }

    // 清除窗口外的记录
    const validTimestamps = timestamps.filter(t => t > windowStart);
    this.rateLimitTracker.set(identifier, validTimestamps);

    // 检查是否超过限制
    if (validTimestamps.length >= this.config.rateLimitMax) {
      this.logSecurityEvent(SecurityEventType.SUSPICIOUS_ACTIVITY, false, {
        type: 'rate_limit_exceeded',
        identifier,
        count: validTimestamps.length
      });
      return false;
    }

    // 记录当前请求
    validTimestamps.push(now);
    this.rateLimitTracker.set(identifier, validTimestamps);

    return true;
  }

  /**
   * 检查IP过滤
   */
  checkIPFilter(ipAddress: string): boolean {
    if (!this.config.enableIPFiltering) {
      return true;
    }

    // 检查黑名单
    if (this.config.ipBlacklist.includes(ipAddress)) {
      this.logSecurityEvent(SecurityEventType.SECURITY_VIOLATION, false, {
        type: 'ip_blacklisted',
        ipAddress
      });
      return false;
    }

    // 检查白名单（如果配置了）
    if (this.config.ipWhitelist.length > 0 && !this.config.ipWhitelist.includes(ipAddress)) {
      this.logSecurityEvent(SecurityEventType.SECURITY_VIOLATION, false, {
        type: 'ip_not_whitelisted',
        ipAddress
      });
      return false;
    }

    return true;
  }

  /**
   * 记录失败尝试
   */
  recordFailedAttempt(identifier: string): void {
    const currentAttempts = (this.failedAttempts.get(identifier) || 0) + 1;
    this.failedAttempts.set(identifier, currentAttempts);

    if (currentAttempts >= this.config.maxFailedAttempts) {
      // 锁定账户
      const lockoutUntil = Date.now() + this.config.lockoutDuration;
      this.lockouts.set(identifier, lockoutUntil);

      this.logSecurityEvent(SecurityEventType.SUSPICIOUS_ACTIVITY, false, {
        type: 'account_locked',
        identifier,
        attempts: currentAttempts
      });

      logger.warn(`Account locked: ${identifier}`, { attempts: currentAttempts }, LogCategory.SECURITY);
    }
  }

  /**
   * 检查是否被锁定
   */
  isLocked(identifier: string): boolean {
    const lockoutUntil = this.lockouts.get(identifier);
    if (!lockoutUntil) {
      return false;
    }

    if (Date.now() > lockoutUntil) {
      this.lockouts.delete(identifier);
      this.failedAttempts.delete(identifier);
      return false;
    }

    return true;
  }

  /**
   * 重置失败尝试
   */
  resetFailedAttempts(identifier: string): void {
    this.failedAttempts.delete(identifier);
    this.lockouts.delete(identifier);
  }

  /**
   * 添加安全策略
   */
  addSecurityPolicy(policy: SecurityPolicy): void {
    this.securityPolicies.push(policy);
    logger.info(`Security policy added: ${policy.name}`, { policy }, LogCategory.SECURITY);
  }

  /**
   * 移除安全策略
   */
  removeSecurityPolicy(policyId: string): void {
    this.securityPolicies = this.securityPolicies.filter(p => p.id !== policyId);
    logger.info(`Security policy removed: ${policyId}`, {}, LogCategory.SECURITY);
  }

  /**
   * 评估安全策略
   */
  evaluateSecurityPolicies(context: Record<string, any>): boolean {
    for (const policy of this.securityPolicies) {
      if (!policy.enabled) {
continue;
}

      for (const rule of policy.rules) {
        const result = this.evaluateRule(rule, context);

        if (rule.action === 'deny' && result) {
          this.logSecurityEvent(SecurityEventType.SECURITY_VIOLATION, false, {
            policy: policy.name,
            rule: rule.type
          });
          return false;
        }

        if (rule.action === 'alert' && result) {
          this.logSecurityEvent(SecurityEventType.SUSPICIOUS_ACTIVITY, true, {
            policy: policy.name,
            rule: rule.type
          });
        }
      }
    }

    return true;
  }

  /**
   * 评估单个规则
   */
  private evaluateRule(rule: SecurityRule, context: Record<string, any>): boolean {
    switch (rule.type) {
      case 'rate_limit':
        return this.checkRateLimit(context.identifier || 'default');
      case 'ip_whitelist':
        return this.config.ipWhitelist.includes(context.ipAddress || '');
      case 'ip_blacklist':
        return this.config.ipBlacklist.includes(context.ipAddress || '');
      case 'time_based': {
        const now = new Date();
        const hour = now.getHours();
        const allowedHours = rule.config.hours || [0, 23];
        return allowedHours.includes(hour);
      }
      case 'custom':
        return rule.config.validator?.(context) || false;
      default:
        return true;
    }
  }

  /**
   * 加密数据
   */
  async encryptData(data: string): Promise<string> {
    if (!this.config.enableEncryption || !this.config.encryptionKey) {
      return data;
    }

    // 在实际应用中，这里应该使用真正的加密算法
    // 如 AES-GCM
    return btoa(data); // 简化的base64编码
  }

  /**
   * 解密数据
   */
  async decryptData(encryptedData: string): Promise<string> {
    if (!this.config.enableEncryption || !this.config.encryptionKey) {
      return encryptedData;
    }

    // 在实际应用中，这里应该使用真正的解密算法
    try {
      return atob(encryptedData);
    } catch {
      throw createError(
        ErrorCode.SECURITY_VIOLATION,
        'Failed to decrypt data',
        ErrorSeverity.ERROR,
        ErrorCategory.SECURITY
      );
    }
  }

  /**
   * 获取当前会话
   */
  getCurrentSession(): Session | null {
    return this.currentSession;
  }

  /**
   * 获取所有会话
   */
  getSessions(): Session[] {
    return Array.from(this.sessions.values());
  }

  /**
   * 获取安全事件
   */
  getSecurityEvents(filters?: {
    type?: SecurityEventType;
    userId?: string;
    startTime?: number;
    endTime?: number;
  }): SecurityEvent[] {
    let events = [...this.securityEvents];

    if (filters) {
      if (filters.type) {
        events = events.filter(e => e.type === filters.type);
      }
      if (filters.userId) {
        events = events.filter(e => e.userId === filters.userId);
      }
      if (filters.startTime !== undefined) {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        events = events.filter(e => e.timestamp >= filters.startTime!);
      }
      if (filters.endTime !== undefined) {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        events = events.filter(e => e.timestamp <= filters.endTime!);
      }
    }

    return events;
  }

  /**
   * 获取安全统计
   */
  getSecurityStatistics(): {
    totalEvents: number;
    failedEvents: number;
    successfulEvents: number;
    eventsByType: Record<SecurityEventType, number>;
    activeSessions: number;
    lockedAccounts: number;
  } {
    const eventsByType: Record<SecurityEventType, number> = {
      [SecurityEventType.AUTHENTICATION]: 0,
      [SecurityEventType.AUTHORIZATION]: 0,
      [SecurityEventType.PERMISSION_DENIED]: 0,
      [SecurityEventType.DATA_ACCESS]: 0,
      [SecurityEventType.DATA_MODIFICATION]: 0,
      [SecurityEventType.SECURITY_VIOLATION]: 0,
      [SecurityEventType.SUSPICIOUS_ACTIVITY]: 0
    };

    this.securityEvents.forEach(event => {
      eventsByType[event.type]++;
    });

    const failedEvents = this.securityEvents.filter(e => !e.success).length;
    const successfulEvents = this.securityEvents.filter(e => e.success).length;

    return {
      totalEvents: this.securityEvents.length,
      failedEvents,
      successfulEvents,
      eventsByType,
      activeSessions: this.sessions.size,
      lockedAccounts: this.lockouts.size
    };
  }

  /**
   * 导出安全报告
   */
  exportSecurityReport(): string {
    const statistics = this.getSecurityStatistics();
    const recentEvents = this.securityEvents.slice(-100);
    const activeSessions = this.getSessions();

    return JSON.stringify({
      timestamp: Date.now(),
      statistics,
      recentEvents,
      activeSessions,
      config: {
        ...this.config,
        encryptionKey: undefined // 不导出加密密钥
      }
    }, null, 2);
  }

  /**
   * 下载安全报告
   */
  downloadSecurityReport(): void {
    const content = this.exportSecurityReport();
    const blob = new Blob([content], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `security-report-${Date.now()}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }

  /**
   * 更新配置
   */
  updateConfig(config: Partial<SecurityConfig>): void {
    this.config = { ...this.config, ...config };
    logger.info('Security configuration updated', { config: this.config }, LogCategory.SECURITY);
  }

  /**
   * 销毁
   */
  destroy(): void {
    this.sessions.clear();
    this.securityEvents = [];
    this.failedAttempts.clear();
    this.lockouts.clear();
    this.rateLimitTracker.clear();
    this.currentSession = null;

    logger.info('Security manager destroyed', {}, LogCategory.SECURITY);
  }
}

// 导出单例
export const securityManager = new SecurityManager();

// 导出便捷函数
export const security = {
  createSession: (userId: string, role: UserRole, metadata?: Record<string, any>) =>
    securityManager.createSession(userId, role, metadata),
  validateSession: (sessionId: string) => securityManager.validateSession(sessionId),
  destroySession: (sessionId: string) => securityManager.destroySession(sessionId),
  checkPermission: (resource: ResourceType, action: ActionType, level?: PermissionLevel) =>
    securityManager.checkPermission(resource, action, level),
  requirePermission: (resource: ResourceType, action: ActionType, level?: PermissionLevel) =>
    securityManager.requirePermission(resource, action, level),
  getCurrentSession: () => securityManager.getCurrentSession(),
  getSessions: () => securityManager.getSessions(),
  getSecurityEvents: (filters?: Parameters<typeof securityManager.getSecurityEvents>[0]) =>
    securityManager.getSecurityEvents(filters),
  getStatistics: () => securityManager.getSecurityStatistics(),
  exportReport: () => securityManager.exportSecurityReport(),
  downloadReport: () => securityManager.downloadSecurityReport(),
  addSecurityPolicy: (policy: SecurityPolicy) => securityManager.addSecurityPolicy(policy),
  removeSecurityPolicy: (policyId: string) => securityManager.removeSecurityPolicy(policyId),
  encryptData: (data: string) => securityManager.encryptData(data),
  decryptData: (encryptedData: string) => securityManager.decryptData(encryptedData),
  updateConfig: (config: Partial<SecurityConfig>) => securityManager.updateConfig(config)
};
