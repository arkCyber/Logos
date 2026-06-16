/**
 * Audit Logger for aerospace-grade operation tracking
 * Records critical operations for security and compliance
 */

import { logger, LogCategory } from './logger';

export enum AuditAction {
  PACKAGE_INSTALL = 'PACKAGE_INSTALL',
  PACKAGE_UNINSTALL = 'PACKAGE_UNINSTALL',
  PACKAGE_UPDATE = 'PACKAGE_UPDATE',
  PACKAGE_VIEW = 'PACKAGE_VIEW',
  FONT_UPLOAD = 'FONT_UPLOAD',
  FONT_DELETE = 'FONT_DELETE',
  FONT_VIEW = 'FONT_VIEW',
  SEARCH = 'SEARCH',
  FILTER = 'FILTER',
  REFRESH = 'REFRESH',
}

export interface AuditLogEntry {
  action: AuditAction;
  timestamp: string;
  userId?: string;
  details: Record<string, any>;
  success: boolean;
  errorMessage?: string;
}

class AuditLogger {
  private logs: AuditLogEntry[] = [];
  private maxLogs = 1000; // Keep last 1000 logs in memory
  private sessionId: string = this.generateSessionId();

  private generateSessionId(): string {
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
      const r = (Math.random() * 16) | 0;
      const v = c === 'x' ? r : (r & 0x3) | 0x8;
      return v.toString(16);
    });
  }

  /**
   * Log an audit event
   */
  log(action: AuditAction, details: Record<string, any>, success: boolean = true, errorMessage?: string): void {
    const entry: AuditLogEntry = {
      action,
      timestamp: new Date().toISOString(),
      userId: this.getCurrentUserId(),
      details,
      success,
      errorMessage
    };

    this.logs.push(entry);

    // Keep only the last maxLogs entries
    if (this.logs.length > this.maxLogs) {
      this.logs = this.logs.slice(-this.maxLogs);
    }

    // Also log to the main logger
    if (success) {
      logger.info(`Audit: ${action}`, details, LogCategory.SYSTEM);
    } else {
      logger.error(`Audit: ${action} failed`, new Error(errorMessage || 'Unknown error'), LogCategory.SYSTEM);
    }
  }

  /**
   * Get all audit logs
   */
  getLogs(): AuditLogEntry[] {
    return [...this.logs];
  }

  /**
   * Get logs filtered by action
   */
  getLogsByAction(action: AuditAction): AuditLogEntry[] {
    return this.logs.filter(log => log.action === action);
  }

  /**
   * Get logs filtered by time range
   */
  getLogsByTimeRange(startTime: Date, endTime: Date): AuditLogEntry[] {
    return this.logs.filter(log => {
      const logTime = new Date(log.timestamp);
      return logTime >= startTime && logTime <= endTime;
    });
  }

  /**
   * Get failed operations
   */
  getFailedLogs(): AuditLogEntry[] {
    return this.logs.filter(log => !log.success);
  }

  /**
   * Clear all logs (should only be called by admin)
   */
  clearLogs(): void {
    this.logs = [];
    logger.warn('Audit logs cleared', {}, LogCategory.SYSTEM);
  }

  /**
   * Export logs as JSON
   */
  exportLogs(): string {
    return JSON.stringify(this.logs, null, 2);
  }

  /**
   * Get current user/session ID
   */
  private getCurrentUserId(): string | undefined {
    return this.sessionId;
  }
}

// Singleton instance
export const auditLogger = new AuditLogger();
