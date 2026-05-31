/*!
 * 航空航天级光标追踪器测试
 * 测试 CursorTracker 的所有功能
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { CursorTracker, type CursorInfo } from '../cursorTracker';

// Mock Editor
const mockEditor = {
  state: {
    selection: {
      from: 10,
      to: 20
    }
  }
};

describe('CursorTracker', () => {
  let tracker: CursorTracker;

  beforeEach(() => {
    tracker = new CursorTracker(mockEditor as any);
  });

  afterEach(() => {
    tracker.destroy();
  });

  describe('Local Cursor Tracking', () => {
    it('should track local cursor position', () => {
      tracker.trackLocalPosition(15);

      expect(tracker.getLocalPosition()).toBe(15);
    });

    it('should debounce position updates', async () => {
      const callback = vi.fn();
      tracker.onLocalPositionChange(callback);

      tracker.trackLocalPosition(10);
      tracker.trackLocalPosition(15);
      tracker.trackLocalPosition(20);

      // Callback should only be called once after debounce
      await new Promise(resolve => setTimeout(resolve, 150));
      expect(callback).toHaveBeenCalledTimes(1);
      expect(callback).toHaveBeenCalledWith(20);
    });

    it('should track from editor selection', () => {
      tracker.trackFromEditor();

      expect(tracker.getLocalPosition()).toBe(10);
    });
  });

  describe('Remote Cursor Management', () => {
    it('should update remote cursor', () => {
      tracker.updateRemoteCursor('user-1', 10, 'User 1');

      const cursor = tracker.getRemoteCursor('user-1');
      expect(cursor).toBeDefined();
      expect(cursor?.position).toBe(10);
      expect(cursor?.userName).toBe('User 1');
    });

    it('should assign consistent color to user', () => {
      tracker.updateRemoteCursor('user-1', 10, 'User 1');
      const cursor1 = tracker.getRemoteCursor('user-1');

      tracker.updateRemoteCursor('user-1', 20, 'User 1');
      const cursor2 = tracker.getRemoteCursor('user-1');

      expect(cursor1?.color).toBe(cursor2?.color);
    });

    it('should assign different colors to different users', () => {
      tracker.updateRemoteCursor('user-1', 10, 'User 1');
      tracker.updateRemoteCursor('user-2', 20, 'User 2');

      const cursor1 = tracker.getRemoteCursor('user-1');
      const cursor2 = tracker.getRemoteCursor('user-2');

      expect(cursor1?.color).not.toBe(cursor2?.color);
    });

    it('should remove remote cursor', () => {
      tracker.updateRemoteCursor('user-1', 10, 'User 1');
      tracker.removeRemoteCursor('user-1');

      const cursor = tracker.getRemoteCursor('user-1');
      expect(cursor).toBeUndefined();
    });

    it('should get all remote cursors', () => {
      tracker.updateRemoteCursor('user-1', 10, 'User 1');
      tracker.updateRemoteCursor('user-2', 20, 'User 2');

      const cursors = tracker.getRemoteCursors();
      expect(cursors.size).toBe(2);
    });

    it('should update cursor from presence info', () => {
      const presence = {
        user_id: 'user-1',
        user_name: 'User 1',
        cursor_position: 15,
        last_seen: new Date().toISOString(),
        is_online: true
      };

      tracker.updateRemoteCursorFromPresence(presence);

      const cursor = tracker.getRemoteCursor('user-1');
      expect(cursor?.position).toBe(15);
    });
  });

  describe('Conflict Detection', () => {
    it('should detect cursor conflict', () => {
      tracker.updateRemoteCursor('user-1', 10, 'User 1');

      const hasConflict = tracker.detectConflict(12, 5);
      expect(hasConflict).toBe(true);
    });

    it('should not detect conflict when cursors are far apart', () => {
      tracker.updateRemoteCursor('user-1', 10, 'User 1');

      const hasConflict = tracker.detectConflict(20, 5);
      expect(hasConflict).toBe(false);
    });

    it('should use custom threshold for conflict detection', () => {
      tracker.updateRemoteCursor('user-1', 10, 'User 1');

      const hasConflict = tracker.detectConflict(15, 10);
      expect(hasConflict).toBe(true);
    });
  });

  describe('Cursor Cleanup', () => {
    it('should cleanup expired cursors', () => {
      tracker.updateRemoteCursor('user-1', 10, 'User 1');

      // Manually set last updated to old time
      const cursor = tracker.getRemoteCursor('user-1') as CursorInfo;
      cursor.lastUpdated = new Date(Date.now() - 40000); // 40 seconds ago

      tracker.cleanupExpiredCursors();

      const cursorAfter = tracker.getRemoteCursor('user-1');
      expect(cursorAfter).toBeUndefined();
    });

    it('should not cleanup recent cursors', () => {
      tracker.updateRemoteCursor('user-1', 10, 'User 1');

      tracker.cleanupExpiredCursors();

      const cursor = tracker.getRemoteCursor('user-1');
      expect(cursor).toBeDefined();
    });

    it('should clear all remote cursors', () => {
      tracker.updateRemoteCursor('user-1', 10, 'User 1');
      tracker.updateRemoteCursor('user-2', 20, 'User 2');

      tracker.clearRemoteCursors();

      expect(tracker.getRemoteCursors().size).toBe(0);
    });
  });

  describe('Event Callbacks', () => {
    it('should trigger local position change callback', async () => {
      const callback = vi.fn();
      tracker.onLocalPositionChange(callback);

      tracker.trackLocalPosition(15);

      await new Promise(resolve => setTimeout(resolve, 150));
      expect(callback).toHaveBeenCalledWith(15);
    });

    it('should trigger remote cursor update callback', () => {
      const callback = vi.fn();
      tracker.onRemoteCursorUpdate(callback);

      tracker.updateRemoteCursor('user-1', 10, 'User 1');

      expect(callback).toHaveBeenCalled();
    });

    it('should trigger remote cursor remove callback', () => {
      const callback = vi.fn();
      tracker.onRemoteCursorRemove(callback);

      tracker.updateRemoteCursor('user-1', 10, 'User 1');
      tracker.removeRemoteCursor('user-1');

      expect(callback).toHaveBeenCalledWith('user-1');
    });
  });

  describe('Configuration', () => {
    it('should get default config', () => {
      const config = tracker.getConfig();

      expect(config.showUserName).toBe(true);
      expect(config.showCursor).toBe(true);
      expect(config.remoteCursorColors).toHaveLength(8);
    });

    it('should update config', () => {
      tracker.updateConfig({ showUserName: false });

      const config = tracker.getConfig();
      expect(config.showUserName).toBe(false);
    });
  });

  describe('Resource Cleanup', () => {
    it('should clean up resources on destroy', () => {
      tracker.updateRemoteCursor('user-1', 10, 'User 1');
      tracker.destroy();

      expect(tracker.getRemoteCursors().size).toBe(0);
    });
  });
});
