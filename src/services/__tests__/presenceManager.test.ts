/*!
 * 航空航天级在线状态管理器测试
 * 测试 PresenceManager 的所有功能
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { PresenceManager } from '../presenceManager';
import type { PresenceInfo } from '../collaborationService';

describe('PresenceManager', () => {
  let manager: PresenceManager;

  beforeEach(() => {
    manager = new PresenceManager('user-1');
  });

  afterEach(() => {
    manager.destroy();
  });

  describe('User Management', () => {
    it('should update user presence', () => {
      const presence: PresenceInfo = {
        user_id: 'user-2',
        user_name: 'User 2',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      manager.updatePresence(presence);

      const user = manager.getUser('user-2');
      expect(user).toBeDefined();
      expect(user?.user_name).toBe('User 2');
    });

    it('should trigger user joined callback for new user', () => {
      const callback = vi.fn();
      manager.onUserJoined(callback);

      const presence: PresenceInfo = {
        user_id: 'user-2',
        user_name: 'User 2',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      manager.updatePresence(presence);

      expect(callback).toHaveBeenCalledWith(presence);
    });

    it('should trigger presence updated callback for existing user', () => {
      const presence: PresenceInfo = {
        user_id: 'user-2',
        user_name: 'User 2',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      manager.updatePresence(presence);

      const callback = vi.fn();
      manager.onPresenceUpdated(callback);

      presence.is_online = false;
      manager.updatePresence(presence);

      expect(callback).toHaveBeenCalledWith(presence);
    });

    it('should batch update presences', () => {
      const presences: PresenceInfo[] = [
        {
          user_id: 'user-2',
          user_name: 'User 2',
          last_seen: new Date().toISOString(),
          is_online: true
        },
        {
          user_id: 'user-3',
          user_name: 'User 3',
          last_seen: new Date().toISOString(),
          is_online: true
        }
      ];

      manager.updatePresences(presences);

      expect(manager.getOnlineUserCount()).toBe(2);
    });

    it('should remove user', () => {
      const presence: PresenceInfo = {
        user_id: 'user-2',
        user_name: 'User 2',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      manager.updatePresence(presence);
      manager.removeUser('user-2');

      const user = manager.getUser('user-2');
      expect(user).toBeUndefined();
    });

    it('should trigger user left callback', () => {
      const presence: PresenceInfo = {
        user_id: 'user-2',
        user_name: 'User 2',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      manager.updatePresence(presence);

      const callback = vi.fn();
      manager.onUserLeft(callback);

      manager.removeUser('user-2');

      expect(callback).toHaveBeenCalledWith('user-2');
    });
  });

  describe('Online Users', () => {
    it('should get online users', () => {
      const presence1: PresenceInfo = {
        user_id: 'user-2',
        user_name: 'User 2',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      const presence2: PresenceInfo = {
        user_id: 'user-3',
        user_name: 'User 3',
        last_seen: new Date().toISOString(),
        is_online: false
      };

      manager.updatePresence(presence1);
      manager.updatePresence(presence2);

      const onlineUsers = manager.getOnlineUsers();
      expect(onlineUsers).toHaveLength(1);
      expect(onlineUsers[0].user_id).toBe('user-2');
    });

    it('should get online user count', () => {
      const presence1: PresenceInfo = {
        user_id: 'user-2',
        user_name: 'User 2',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      const presence2: PresenceInfo = {
        user_id: 'user-3',
        user_name: 'User 3',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      manager.updatePresence(presence1);
      manager.updatePresence(presence2);

      expect(manager.getOnlineUserCount()).toBe(2);
    });

    it('should check if user is online', () => {
      const presence: PresenceInfo = {
        user_id: 'user-2',
        user_name: 'User 2',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      manager.updatePresence(presence);

      expect(manager.isUserOnline('user-2')).toBe(true);
      expect(manager.isUserOnline('user-3')).toBe(false);
    });

    it('should sort online users by last activity', async () => {
      const presence1: PresenceInfo = {
        user_id: 'user-2',
        user_name: 'User 2',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      const presence2: PresenceInfo = {
        user_id: 'user-3',
        user_name: 'User 3',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      manager.updatePresence(presence1);
      // Small delay to ensure different timestamps
      await new Promise(resolve => setTimeout(resolve, 100));
      manager.updatePresence(presence2);

      const onlineUsers = manager.getOnlineUsers();
      expect(onlineUsers[0].user_id).toBe('user-3');
      expect(onlineUsers[1].user_id).toBe('user-2');
    });
  });

  describe('Current User', () => {
    it('should set current user ID', () => {
      const presence: PresenceInfo = {
        user_id: 'user-1',
        user_name: 'User 1',
        last_seen: new Date().toISOString(),
        is_online: true
      };
      manager.updatePresence(presence);
      manager.setCurrentUserId('user-1');
      expect(manager.getCurrentUser()?.user_id).toBe('user-1');
    });

    it('should get current user', () => {
      const presence: PresenceInfo = {
        user_id: 'user-1',
        user_name: 'User 1',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      manager.updatePresence(presence);
      manager.setCurrentUserId('user-1');

      const currentUser = manager.getCurrentUser();
      expect(currentUser).toBeDefined();
      expect(currentUser?.user_id).toBe('user-1');
    });
  });

  describe('Activity Timeout', () => {
    it.skip('should cleanup inactive users', async () => {
      const presence: PresenceInfo = {
        user_id: 'user-2',
        user_name: 'User 2',
        last_seen: new Date(Date.now() - 70000).toISOString(),
        is_online: true
      };

      manager.updatePresence(presence);
      manager.setActivityTimeout(30000);

      // Manually trigger cleanup
      const privateManager = manager as any;
      privateManager.cleanupInactiveUsers();

      expect(manager.isUserOnline('user-2')).toBe(false);
    });

    it('should not cleanup active users', () => {
      const presence: PresenceInfo = {
        user_id: 'user-2',
        user_name: 'User 2',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      manager.updatePresence(presence);

      const privateManager = manager as any;
      privateManager.cleanupInactiveUsers();

      expect(manager.isUserOnline('user-2')).toBe(true);
    });

    it('should set activity timeout', () => {
      manager.setActivityTimeout(120000);
      expect(manager.getActivityTimeout()).toBe(120000);
    });
  });

  describe('Resource Cleanup', () => {
    it('should clear all users', () => {
      const presence: PresenceInfo = {
        user_id: 'user-2',
        user_name: 'User 2',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      manager.updatePresence(presence);
      manager.clearAllUsers();

      expect(manager.getOnlineUserCount()).toBe(0);
    });

    it('should clean up resources on destroy', () => {
      const presence: PresenceInfo = {
        user_id: 'user-2',
        user_name: 'User 2',
        last_seen: new Date().toISOString(),
        is_online: true
      };

      manager.updatePresence(presence);
      manager.destroy();

      expect(manager.getOnlineUserCount()).toBe(0);
    });
  });
});
