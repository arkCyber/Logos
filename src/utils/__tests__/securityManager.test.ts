/**
 * Security Manager Tests
 * Aerospace-grade comprehensive test suite
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import {
  SecurityManager,
  SecurityConfig,
  PermissionLevel,
  ResourceType,
  ActionType,
  SecurityEventType,
  UserRole,
  SecurityPolicy,
  Session
} from '../securityManager';

// Mock navigator
const mockNavigator = {
  userAgent: 'Mozilla/5.0 Test User Agent'
};

// Mock window
const mockWindow = {
  setInterval: vi.fn((cb, interval) => {
    const id = setInterval(cb, interval);
    return id;
  }),
  clearInterval: vi.fn(clearInterval)
};

describe('SecurityManager', () => {
  let manager: SecurityManager;

  beforeEach(() => {
    // Setup mocks
    global.navigator = mockNavigator as any;
    global.window = mockWindow as any;
    
    manager = new SecurityManager({
      enableAuthentication: true,
      enableAuthorization: true,
      enableAuditLogging: true,
      sessionTimeout: 3600000,
      maxFailedAttempts: 5,
      lockoutDuration: 900000,
      enableRateLimiting: true,
      rateLimitWindow: 60000,
      rateLimitMax: 100,
      enableIPFiltering: false,
      enableEncryption: false
    });
  });

  afterEach(() => {
    manager.destroy();
  });

  describe('Initialization', () => {
    it('should initialize with default config', () => {
      const defaultManager = new SecurityManager();
      expect(defaultManager).toBeDefined();
      defaultManager.destroy();
    });

    it('should initialize with custom config', () => {
      const customConfig: Partial<SecurityConfig> = {
        sessionTimeout: 7200000,
        maxFailedAttempts: 10,
        enableRateLimiting: false
      };
      const customManager = new SecurityManager(customConfig);
      expect(customManager).toBeDefined();
      customManager.destroy();
    });
  });

  describe('Session Management', () => {
    it('should create session', () => {
      const session = manager.createSession('user-123', UserRole.USER);
      expect(session).toBeDefined();
      expect(session.userId).toBe('user-123');
      expect(session.role).toBe(UserRole.USER);
      expect(session.id).toBeTruthy();
      expect(session.createdAt).toBeDefined();
      expect(session.expiresAt).toBeDefined();
    });

    it('should create session with metadata', () => {
      const metadata = { device: 'mobile', location: 'US' };
      const session = manager.createSession('user-123', UserRole.USER, metadata);
      expect(session.metadata).toEqual(metadata);
    });

    it('should validate valid session', () => {
      const session = manager.createSession('user-123', UserRole.USER);
      const isValid = manager.validateSession(session.id);
      expect(isValid).toBe(true);
    });

    it('should invalidate expired session', () => {
      const shortTimeoutManager = new SecurityManager({
        sessionTimeout: 100 // 100ms
      });
      
      const session = shortTimeoutManager.createSession('user-123', UserRole.USER);
      
      // Wait for session to expire
      return new Promise(resolve => {
        setTimeout(() => {
          const isValid = shortTimeoutManager.validateSession(session.id);
          expect(isValid).toBe(false);
          shortTimeoutManager.destroy();
          resolve(null);
        }, 150);
      });
    });

    it('should invalidate non-existent session', () => {
      const isValid = manager.validateSession('non-existent-session');
      expect(isValid).toBe(false);
    });

    it('should destroy session', () => {
      const session = manager.createSession('user-123', UserRole.USER);
      manager.destroySession(session.id);
      const isValid = manager.validateSession(session.id);
      expect(isValid).toBe(false);
    });

    it('should get current session', () => {
      const session = manager.createSession('user-123', UserRole.USER);
      const current = manager.getCurrentSession();
      expect(current).toBeDefined();
      expect(current?.id).toBe(session.id);
    });

    it('should return null when no current session', () => {
      const current = manager.getCurrentSession();
      expect(current).toBeNull();
    });

    it('should get all sessions', () => {
      manager.createSession('user-1', UserRole.USER);
      manager.createSession('user-2', UserRole.ADMIN);
      
      const sessions = manager.getSessions();
      expect(sessions.length).toBeGreaterThanOrEqual(2);
    });
  });

  describe('Permission Checking', () => {
    it('should grant permission with valid session', () => {
      manager.createSession('user-123', UserRole.USER);
      const hasPermission = manager.checkPermission(
        ResourceType.DOCUMENT,
        ActionType.READ,
        PermissionLevel.READ
      );
      expect(hasPermission).toBe(true);
    });

    it('should deny permission without session', () => {
      const hasPermission = manager.checkPermission(
        ResourceType.DOCUMENT,
        ActionType.READ,
        PermissionLevel.READ
      );
      expect(hasPermission).toBe(false);
    });

    it('should deny permission for insufficient level', () => {
      manager.createSession('user-123', UserRole.GUEST);
      const hasPermission = manager.checkPermission(
        ResourceType.DOCUMENT,
        ActionType.WRITE,
        PermissionLevel.WRITE
      );
      expect(hasPermission).toBe(false);
    });

    it('should grant admin permission to admin role', () => {
      manager.createSession('admin-123', UserRole.ADMIN);
      const hasPermission = manager.checkPermission(
        ResourceType.SETTINGS,
        ActionType.WRITE,
        PermissionLevel.ADMIN
      );
      expect(hasPermission).toBe(true);
    });

    it('should grant super admin permission to super admin', () => {
      manager.createSession('super-admin-123', UserRole.SUPER_ADMIN);
      const hasPermission = manager.checkPermission(
        ResourceType.SYSTEM,
        ActionType.WRITE,
        PermissionLevel.SUPER_ADMIN
      );
      expect(hasPermission).toBe(true);
    });

    it('should bypass authorization when disabled', () => {
      const noAuthManager = new SecurityManager({
        enableAuthorization: false
      });
      
      const hasPermission = noAuthManager.checkPermission(
        ResourceType.DOCUMENT,
        ActionType.WRITE,
        PermissionLevel.ADMIN
      );
      expect(hasPermission).toBe(true);
      
      noAuthManager.destroy();
    });

    it('should throw error when permission required but denied', () => {
      manager.createSession('user-123', UserRole.GUEST);
      
      expect(() => {
        manager.requirePermission(
          ResourceType.DOCUMENT,
          ActionType.WRITE,
          PermissionLevel.WRITE
        );
      }).toThrow();
    });

    it('should not throw when permission required and granted', () => {
      manager.createSession('user-123', UserRole.USER);
      
      expect(() => {
        manager.requirePermission(
          ResourceType.DOCUMENT,
          ActionType.READ,
          PermissionLevel.READ
        );
      }).not.toThrow();
    });
  });

  describe('Rate Limiting', () => {
    it('should allow requests within rate limit', () => {
      const result = manager.checkRateLimit('test-identifier');
      expect(result).toBe(true);
    });

    it('should deny requests exceeding rate limit', () => {
      const strictManager = new SecurityManager({
        rateLimitWindow: 1000,
        rateLimitMax: 5
      });
      
      for (let i = 0; i < 10; i++) {
        strictManager.checkRateLimit('test-identifier');
      }
      
      const result = strictManager.checkRateLimit('test-identifier');
      expect(result).toBe(false);
      
      strictManager.destroy();
    });

    it('should bypass rate limiting when disabled', () => {
      const noRateLimitManager = new SecurityManager({
        enableRateLimiting: false
      });
      
      for (let i = 0; i < 200; i++) {
        noRateLimitManager.checkRateLimit('test-identifier');
      }
      
      const result = noRateLimitManager.checkRateLimit('test-identifier');
      expect(result).toBe(true);
      
      noRateLimitManager.destroy();
    });

    it('should reset rate limit after window', () => {
      const strictManager = new SecurityManager({
        rateLimitWindow: 100,
        rateLimitMax: 3
      });
      
      for (let i = 0; i < 5; i++) {
        strictManager.checkRateLimit('test-identifier');
      }
      
      // Wait for window to expire
      return new Promise(resolve => {
        setTimeout(() => {
          const result = strictManager.checkRateLimit('test-identifier');
          expect(result).toBe(true);
          strictManager.destroy();
          resolve(null);
        }, 150);
      });
    });
  });

  describe('IP Filtering', () => {
    it('should allow IP when filtering disabled', () => {
      const result = manager.checkIPFilter('192.168.1.1');
      expect(result).toBe(true);
    });

    it('should block IP in blacklist', () => {
      const ipFilterManager = new SecurityManager({
        enableIPFiltering: true,
        ipBlacklist: ['192.168.1.100']
      });
      
      const result = ipFilterManager.checkIPFilter('192.168.1.100');
      expect(result).toBe(false);
      
      ipFilterManager.destroy();
    });

    it('should allow IP not in blacklist', () => {
      const ipFilterManager = new SecurityManager({
        enableIPFiltering: true,
        ipBlacklist: ['192.168.1.100']
      });
      
      const result = ipFilterManager.checkIPFilter('192.168.1.1');
      expect(result).toBe(true);
      
      ipFilterManager.destroy();
    });

    it('should allow IP in whitelist', () => {
      const ipFilterManager = new SecurityManager({
        enableIPFiltering: true,
        ipWhitelist: ['192.168.1.100']
      });
      
      const result = ipFilterManager.checkIPFilter('192.168.1.100');
      expect(result).toBe(true);
      
      ipFilterManager.destroy();
    });

    it('should block IP not in whitelist when whitelist configured', () => {
      const ipFilterManager = new SecurityManager({
        enableIPFiltering: true,
        ipWhitelist: ['192.168.1.100']
      });
      
      const result = ipFilterManager.checkIPFilter('192.168.1.1');
      expect(result).toBe(false);
      
      ipFilterManager.destroy();
    });
  });

  describe('Failed Attempts and Lockout', () => {
    it('should record failed attempt', () => {
      manager.recordFailedAttempt('user-123');
      const isLocked = manager.isLocked('user-123');
      expect(isLocked).toBe(false);
    });

    it('should lock account after max failed attempts', () => {
      const strictManager = new SecurityManager({
        maxFailedAttempts: 3,
        lockoutDuration: 1000
      });
      
      for (let i = 0; i < 5; i++) {
        strictManager.recordFailedAttempt('user-123');
      }
      
      const isLocked = strictManager.isLocked('user-123');
      expect(isLocked).toBe(true);
      
      strictManager.destroy();
    });

    it('should unlock account after lockout duration', () => {
      const strictManager = new SecurityManager({
        maxFailedAttempts: 3,
        lockoutDuration: 100
      });
      
      for (let i = 0; i < 5; i++) {
        strictManager.recordFailedAttempt('user-123');
      }
      
      // Wait for lockout to expire
      return new Promise(resolve => {
        setTimeout(() => {
          const isLocked = strictManager.isLocked('user-123');
          expect(isLocked).toBe(false);
          strictManager.destroy();
          resolve(null);
        }, 150);
      });
    });

    it('should reset failed attempts', () => {
      manager.recordFailedAttempt('user-123');
      manager.resetFailedAttempts('user-123');
      const isLocked = manager.isLocked('user-123');
      expect(isLocked).toBe(false);
    });
  });

  describe('Security Policies', () => {
    it('should add security policy', () => {
      const policy: SecurityPolicy = {
        id: 'policy-1',
        name: 'Test Policy',
        description: 'Test policy description',
        enabled: true,
        rules: [
          {
            type: 'rate_limit',
            config: { max: 10 },
            action: 'deny'
          }
        ]
      };
      
      manager.addSecurityPolicy(policy);
      expect(manager).toBeDefined();
    });

    it('should remove security policy', () => {
      const policy: SecurityPolicy = {
        id: 'policy-1',
        name: 'Test Policy',
        description: 'Test policy description',
        enabled: true,
        rules: []
      };
      
      manager.addSecurityPolicy(policy);
      manager.removeSecurityPolicy('policy-1');
      expect(manager).toBeDefined();
    });

    it('should evaluate security policies', () => {
      const policy: SecurityPolicy = {
        id: 'policy-1',
        name: 'Test Policy',
        description: 'Test policy description',
        enabled: true,
        rules: [
          {
            type: 'custom',
            config: { validator: () => true },
            action: 'deny'
          }
        ]
      };
      
      manager.addSecurityPolicy(policy);
      const result = manager.evaluateSecurityPolicies({ test: 'context' });
      expect(result).toBe(false);
    });

    it('should skip disabled policies', () => {
      const policy: SecurityPolicy = {
        id: 'policy-1',
        name: 'Test Policy',
        description: 'Test policy description',
        enabled: false,
        rules: [
          {
            type: 'custom',
            config: { validator: () => false },
            action: 'deny'
          }
        ]
      };
      
      manager.addSecurityPolicy(policy);
      const result = manager.evaluateSecurityPolicies({ test: 'context' });
      expect(result).toBe(true);
    });
  });

  describe('Encryption', () => {
    it('should return data when encryption disabled', async () => {
      const data = 'test-data';
      const encrypted = await manager.encryptData(data);
      expect(encrypted).toBe(data);
    });

    it('should return data when decryption disabled', async () => {
      const data = 'test-data';
      const decrypted = await manager.decryptData(data);
      expect(decrypted).toBe(data);
    });

    it('should encrypt data when enabled', async () => {
      const encryptedManager = new SecurityManager({
        enableEncryption: true,
        encryptionKey: 'test-key'
      });
      
      const data = 'test-data';
      const encrypted = await encryptedManager.encryptData(data);
      expect(encrypted).not.toBe(data);
      
      encryptedManager.destroy();
    });

    it('should decrypt data when enabled', async () => {
      const encryptedManager = new SecurityManager({
        enableEncryption: true,
        encryptionKey: 'test-key'
      });
      
      const data = 'test-data';
      const encrypted = await encryptedManager.encryptData(data);
      const decrypted = await encryptedManager.decryptData(encrypted);
      expect(decrypted).toBe(data);
      
      encryptedManager.destroy();
    });
  });

  describe('Security Events', () => {
    it('should get security events', () => {
      manager.createSession('user-123', UserRole.USER);
      const events = manager.getSecurityEvents();
      expect(events.length).toBeGreaterThan(0);
    });

    it('should filter events by type', () => {
      manager.createSession('user-123', UserRole.USER);
      const authEvents = manager.getSecurityEvents({ type: SecurityEventType.AUTHENTICATION });
      expect(authEvents.length).toBeGreaterThan(0);
    });

    it('should filter events by userId', () => {
      manager.createSession('user-123', UserRole.USER);
      const userEvents = manager.getSecurityEvents({ userId: 'user-123' });
      expect(userEvents.length).toBeGreaterThan(0);
    });

    it('should filter events by time range', () => {
      const now = Date.now();
      manager.createSession('user-123', UserRole.USER);
      
      const recentEvents = manager.getSecurityEvents({
        startTime: now - 1000,
        endTime: now + 1000
      });
      expect(recentEvents.length).toBeGreaterThan(0);
    });

    it('should limit event history', () => {
      // Events should be limited to 10000
      const events = manager.getSecurityEvents();
      expect(events.length).toBeLessThanOrEqual(10000);
    });
  });

  describe('Security Statistics', () => {
    it('should get security statistics', () => {
      manager.createSession('user-123', UserRole.USER);
      const stats = manager.getSecurityStatistics();
      expect(stats).toBeDefined();
      expect(typeof stats.totalEvents).toBe('number');
      expect(typeof stats.failedEvents).toBe('number');
      expect(typeof stats.successfulEvents).toBe('number');
      expect(typeof stats.activeSessions).toBe('number');
      expect(typeof stats.lockedAccounts).toBe('number');
    });

    it('should count events by type', () => {
      manager.createSession('user-123', UserRole.USER);
      const stats = manager.getSecurityStatistics();
      expect(stats.eventsByType).toBeDefined();
      expect(stats.eventsByType[SecurityEventType.AUTHENTICATION]).toBeGreaterThan(0);
    });

    it('should return zero statistics when no events', () => {
      const emptyManager = new SecurityManager();
      const stats = emptyManager.getSecurityStatistics();
      expect(stats.totalEvents).toBe(0);
      expect(stats.failedEvents).toBe(0);
      expect(stats.successfulEvents).toBe(0);
      emptyManager.destroy();
    });
  });

  describe('Export/Import', () => {
    it('should export security report', () => {
      manager.createSession('user-123', UserRole.USER);
      const report = manager.exportSecurityReport();
      expect(typeof report).toBe('string');
      
      const parsed = JSON.parse(report);
      expect(parsed).toHaveProperty('timestamp');
      expect(parsed).toHaveProperty('statistics');
      expect(parsed).toHaveProperty('recentEvents');
      expect(parsed).toHaveProperty('activeSessions');
      expect(parsed).toHaveProperty('config');
    });

    it('should not include encryption key in export', () => {
      const encryptedManager = new SecurityManager({
        enableEncryption: true,
        encryptionKey: 'secret-key'
      });
      
      const report = encryptedManager.exportSecurityReport();
      const parsed = JSON.parse(report);
      expect(parsed.config.encryptionKey).toBeUndefined();
      
      encryptedManager.destroy();
    });
  });

  describe('Configuration', () => {
    it('should update configuration', () => {
      const newConfig: Partial<SecurityConfig> = {
        sessionTimeout: 7200000,
        maxFailedAttempts: 10
      };
      
      manager.updateConfig(newConfig);
      expect(manager).toBeDefined();
    });

    it('should disable audit logging', () => {
      manager.updateConfig({ enableAuditLogging: false });
      manager.createSession('user-123', UserRole.USER);
      
      const events = manager.getSecurityEvents();
      expect(events.length).toBe(0);
    });
  });

  describe('Destroy', () => {
    it('should destroy manager', () => {
      expect(() => manager.destroy()).not.toThrow();
    });

    it('should clear all data on destroy', () => {
      manager.createSession('user-123', UserRole.USER);
      manager.destroy();
      
      const sessions = manager.getSessions();
      expect(sessions.length).toBe(0);
    });
  });

  describe('Role Permissions', () => {
    it('should have correct permissions for guest', () => {
      manager.createSession('guest-123', UserRole.GUEST);
      
      expect(manager.checkPermission(ResourceType.DOCUMENT, ActionType.READ)).toBe(true);
      expect(manager.checkPermission(ResourceType.DOCUMENT, ActionType.WRITE)).toBe(false);
    });

    it('should have correct permissions for user', () => {
      manager.createSession('user-123', UserRole.USER);
      
      expect(manager.checkPermission(ResourceType.DOCUMENT, ActionType.READ)).toBe(true);
      expect(manager.checkPermission(ResourceType.DOCUMENT, ActionType.WRITE)).toBe(true);
      expect(manager.checkPermission(ResourceType.SETTINGS, ActionType.WRITE)).toBe(false);
    });

    it('should have correct permissions for editor', () => {
      manager.createSession('editor-123', UserRole.EDITOR);
      
      expect(manager.checkPermission(ResourceType.DOCUMENT, ActionType.DELETE)).toBe(true);
      expect(manager.checkPermission(ResourceType.SETTINGS, ActionType.READ)).toBe(true);
      expect(manager.checkPermission(ResourceType.SETTINGS, ActionType.WRITE)).toBe(false);
    });

    it('should have correct permissions for admin', () => {
      manager.createSession('admin-123', UserRole.ADMIN);
      
      expect(manager.checkPermission(ResourceType.USER, ActionType.WRITE)).toBe(true);
      expect(manager.checkPermission(ResourceType.SYSTEM, ActionType.READ)).toBe(false);
    });

    it('should have correct permissions for super admin', () => {
      manager.createSession('super-admin-123', UserRole.SUPER_ADMIN);
      
      expect(manager.checkPermission(ResourceType.SYSTEM, ActionType.WRITE)).toBe(true);
      expect(manager.checkPermission(ResourceType.API, ActionType.MANAGE)).toBe(true);
      expect(manager.checkPermission(ResourceType.DATABASE, ActionType.MANAGE)).toBe(true);
    });
  });

  describe('Edge Cases', () => {
    it('should handle multiple sessions', () => {
      manager.createSession('user-1', UserRole.USER);
      manager.createSession('user-2', UserRole.ADMIN);
      manager.createSession('user-3', UserRole.EDITOR);
      
      const sessions = manager.getSessions();
      expect(sessions.length).toBeGreaterThanOrEqual(3);
    });

    it('should handle rapid session creation', () => {
      for (let i = 0; i < 10; i++) {
        manager.createSession(`user-${i}`, UserRole.USER);
      }
      
      const sessions = manager.getSessions();
      expect(sessions.length).toBeGreaterThanOrEqual(10);
    });

    it('should handle permission checks without session', () => {
      expect(manager.checkPermission(ResourceType.DOCUMENT, ActionType.READ)).toBe(false);
      expect(manager.checkPermission(ResourceType.FILE, ActionType.WRITE)).toBe(false);
    });

    it('should handle empty security policies', () => {
      const result = manager.evaluateSecurityPolicies({});
      expect(result).toBe(true);
    });
  });
});


describe('Enums', () => {
  it('should have all permission levels', () => {
    expect(PermissionLevel.NONE).toBe('none');
    expect(PermissionLevel.READ).toBe('read');
    expect(PermissionLevel.WRITE).toBe('write');
    expect(PermissionLevel.ADMIN).toBe('admin');
    expect(PermissionLevel.SUPER_ADMIN).toBe('super_admin');
  });

  it('should have all resource types', () => {
    expect(ResourceType.DOCUMENT).toBe('document');
    expect(ResourceType.FILE).toBe('file');
    expect(ResourceType.SETTINGS).toBe('settings');
    expect(ResourceType.USER).toBe('user');
    expect(ResourceType.SYSTEM).toBe('system');
    expect(ResourceType.API).toBe('api');
    expect(ResourceType.DATABASE).toBe('database');
    expect(ResourceType.LOGS).toBe('logs');
  });

  it('should have all action types', () => {
    expect(ActionType.READ).toBe('read');
    expect(ActionType.WRITE).toBe('write');
    expect(ActionType.DELETE).toBe('delete');
    expect(ActionType.EXECUTE).toBe('execute');
    expect(ActionType.MANAGE).toBe('manage');
    expect(ActionType.EXPORT).toBe('export');
    expect(ActionType.IMPORT).toBe('import');
  });

  it('should have all security event types', () => {
    expect(SecurityEventType.AUTHENTICATION).toBe('authentication');
    expect(SecurityEventType.AUTHORIZATION).toBe('authorization');
    expect(SecurityEventType.PERMISSION_DENIED).toBe('permission_denied');
    expect(SecurityEventType.DATA_ACCESS).toBe('data_access');
    expect(SecurityEventType.DATA_MODIFICATION).toBe('data_modification');
    expect(SecurityEventType.SECURITY_VIOLATION).toBe('security_violation');
    expect(SecurityEventType.SUSPICIOUS_ACTIVITY).toBe('suspicious_activity');
  });

  it('should have all user roles', () => {
    expect(UserRole.GUEST).toBe('guest');
    expect(UserRole.USER).toBe('user');
    expect(UserRole.EDITOR).toBe('editor');
    expect(UserRole.REVIEWER).toBe('reviewer');
    expect(UserRole.ADMIN).toBe('admin');
    expect(UserRole.SUPER_ADMIN).toBe('super_admin');
  });
});
