import { describe, it, expect, beforeEach, vi } from 'vitest';
import { CursorTracker, getCursorTracker, destroyCursorTracker } from '../cursorTracker';
import type { Editor } from '@tiptap/core';
import { logger } from '../../utils/logger';

/**
 * Unit tests for CursorTracker
 * Tests cursor position tracking, validation, and visibility checking
 */

describe('CursorTracker', () => {
  let cursorTracker: CursorTracker;
  let mockEditor: Partial<Editor>;

  beforeEach(() => {
    // Reset singleton
    destroyCursorTracker();

    // Create mock editor
    mockEditor = {
      isDestroyed: false,
      state: {
        selection: {
          from: 0,
          to: 0,
          $from: {} as any,
          $to: {} as any,
          empty: true,
          eq: vi.fn(),
          map: vi.fn(),
          content: vi.fn(),
          replace: vi.fn(),
          replaceWith: vi.fn(),
          toJSON: vi.fn()
        },
        doc: {
          content: {
            size: 100
          }
        } as any
      } as any,
      view: {
        dom: document.createElement('div'),
        state: {
          selection: {
            from: 0,
            to: 0
          }
        } as any,
        coordsAtPos: vi.fn().mockReturnValue({ top: 100, left: 50, bottom: 120, right: 70 })
      } as any
    };

    cursorTracker = new CursorTracker(mockEditor as Editor);
  });

  describe('trackLocalPosition', () => {
    it('should track valid cursor position', () => {
      cursorTracker.trackLocalPosition(50);
      expect(cursorTracker.getLocalPosition()).toBe(50);
    });

    it('should reject negative positions', () => {
      const loggerSpy = vi.spyOn(logger, 'warn').mockImplementation(() => {});
      cursorTracker.trackLocalPosition(-5);
      expect(cursorTracker.getLocalPosition()).toBe(0);
      expect(loggerSpy).toHaveBeenCalledWith(
        'Invalid negative position, using 0',
        { position: -5 },
        expect.anything()
      );
      loggerSpy.mockRestore();
    });

    it('should not update if position unchanged', () => {
      cursorTracker.trackLocalPosition(10);
      const history1 = cursorTracker.getCursorHistory();
      cursorTracker.trackLocalPosition(10);
      const history2 = cursorTracker.getCursorHistory();
      expect(history1.length).toBe(history2.length);
    });

    it('should maintain cursor history', () => {
      cursorTracker.trackLocalPosition(10);
      cursorTracker.trackLocalPosition(20);
      cursorTracker.trackLocalPosition(30);
      
      const history = cursorTracker.getCursorHistory();
      expect(history).toContain(0); // initial
      expect(history).toContain(10);
      expect(history).toContain(20);
    });

    it('should limit history size', () => {
      for (let i = 0; i < 100; i++) {
        cursorTracker.trackLocalPosition(i);
      }
      
      const history = cursorTracker.getCursorHistory();
      expect(history.length).toBeLessThanOrEqual(50);
    });
  });

  describe('trackFromEditor', () => {
    it('should track cursor from editor state', () => {
      // Create new mock with updated selection
      const updatedEditor = {
        ...mockEditor,
        state: {
          ...mockEditor.state,
          selection: {
            ...mockEditor.state!.selection,
            from: 42,
            to: 42
          }
        }
      } as any;
      
      const tracker = new CursorTracker(updatedEditor);
      tracker.trackFromEditor();
      expect(tracker.getLocalPosition()).toBe(42);
    });

    it('should handle null editor gracefully', () => {
      const loggerSpy = vi.spyOn(logger, 'warn').mockImplementation(() => {});
      const tracker = new CursorTracker(null);
      tracker.trackFromEditor();
      expect(loggerSpy).toHaveBeenCalledWith(
        'Cannot track from editor: editor is null',
        {},
        expect.anything()
      );
      loggerSpy.mockRestore();
    });

    it('should handle destroyed editor gracefully', () => {
      const loggerSpy = vi.spyOn(logger, 'warn').mockImplementation(() => {});
      const destroyedEditor = {
        ...mockEditor,
        isDestroyed: true
      } as any;

      const tracker = new CursorTracker(destroyedEditor);
      tracker.trackFromEditor();
      expect(loggerSpy).toHaveBeenCalledWith(
        'Cannot track from editor: editor is destroyed',
        {},
        expect.anything()
      );
      loggerSpy.mockRestore();
    });
  });

  describe('getValidatedLocalPosition', () => {
    it('should return position within document bounds', () => {
      cursorTracker.trackLocalPosition(50);
      const validated = cursorTracker.getValidatedLocalPosition();
      expect(validated).toBe(50);
    });

    it('should clamp position to document size', () => {
      cursorTracker.trackLocalPosition(200); // exceeds doc size of 100
      const validated = cursorTracker.getValidatedLocalPosition();
      expect(validated).toBeLessThanOrEqual(99); // size - 1
    });

    it('should handle negative positions', () => {
      // Force negative position through direct property access (for testing)
      (cursorTracker as any).localPosition = -10;
      const validated = cursorTracker.getValidatedLocalPosition();
      expect(validated).toBe(0);
    });
  });

  describe('getPreviousPosition', () => {
    it('should return null for empty history', () => {
      const prev = cursorTracker.getPreviousPosition();
      expect(prev).toBeNull();
    });

    it('should return last position from history', () => {
      cursorTracker.trackLocalPosition(10);
      cursorTracker.trackLocalPosition(20);
      
      const prev = cursorTracker.getPreviousPosition();
      expect(prev).toBe(10);
    });
  });

  describe('remote cursor management', () => {
    it('should update remote cursor', () => {
      cursorTracker.updateRemoteCursor('user1', 50, 'Alice');
      
      const remoteCursor = cursorTracker.getRemoteCursor('user1');
      expect(remoteCursor).toBeDefined();
      expect(remoteCursor?.position).toBe(50);
      expect(remoteCursor?.userName).toBe('Alice');
    });

    it('should remove remote cursor', () => {
      cursorTracker.updateRemoteCursor('user1', 50, 'Alice');
      cursorTracker.removeRemoteCursor('user1');
      
      const remoteCursor = cursorTracker.getRemoteCursor('user1');
      expect(remoteCursor).toBeUndefined();
    });

    it('should get all remote cursors', () => {
      cursorTracker.updateRemoteCursor('user1', 50, 'Alice');
      cursorTracker.updateRemoteCursor('user2', 75, 'Bob');
      
      const cursors = cursorTracker.getRemoteCursors();
      expect(cursors.size).toBe(2);
    });

    it('should assign consistent colors to users', () => {
      cursorTracker.updateRemoteCursor('user1', 50, 'Alice');
      const cursor1 = cursorTracker.getRemoteCursor('user1');
      
      cursorTracker.updateRemoteCursor('user1', 60, 'Alice');
      const cursor2 = cursorTracker.getRemoteCursor('user1');
      
      expect(cursor1?.color).toBe(cursor2?.color);
    });
  });

  describe('conflict detection', () => {
    it('should detect cursor conflict within threshold', () => {
      cursorTracker.updateRemoteCursor('user1', 50, 'Alice');
      
      const hasConflict = cursorTracker.detectConflict(52, 5);
      expect(hasConflict).toBe(true);
    });

    it('should not detect conflict outside threshold', () => {
      cursorTracker.updateRemoteCursor('user1', 50, 'Alice');
      
      const hasConflict = cursorTracker.detectConflict(60, 5);
      expect(hasConflict).toBe(false);
    });

    it('should use default threshold of 5', () => {
      cursorTracker.updateRemoteCursor('user1', 50, 'Alice');
      
      const hasConflict = cursorTracker.detectConflict(53);
      expect(hasConflict).toBe(true);
    });
  });

  describe('cursor visibility', () => {
    it('should check cursor visibility', () => {
      // Mock DOM structure
      const scrollParent = document.createElement('div');
      scrollParent.getBoundingClientRect = vi.fn().mockReturnValue({
        top: 0,
        left: 0,
        bottom: 500,
        right: 800
      });
      
      if (mockEditor.view?.dom) {
        Object.defineProperty(mockEditor.view.dom, 'parentElement', {
          value: scrollParent,
          configurable: true
        });
      }
      
      const isVisible = cursorTracker.isCursorVisible();
      expect(typeof isVisible).toBe('boolean');
    });

    it('should return false for destroyed editor', () => {
      const destroyedEditor = {
        ...mockEditor,
        isDestroyed: true
      } as any;
      
      const tracker = new CursorTracker(destroyedEditor);
      const isVisible = tracker.isCursorVisible();
      expect(isVisible).toBe(false);
    });
  });

  describe('cursor coordinates', () => {
    it('should get cursor screen coordinates', () => {
      const coords = cursorTracker.getCursorScreenCoordinates();
      expect(coords).toBeDefined();
      expect(coords?.top).toBe(100);
      expect(coords?.left).toBe(50);
    });

    it('should return null for destroyed editor', () => {
      const destroyedEditor = {
        ...mockEditor,
        isDestroyed: true
      } as any;
      
      const tracker = new CursorTracker(destroyedEditor);
      const coords = tracker.getCursorScreenCoordinates();
      expect(coords).toBeNull();
    });
  });

  describe('statistics', () => {
    it('should provide cursor tracking stats', () => {
      cursorTracker.trackLocalPosition(10);
      cursorTracker.updateRemoteCursor('user1', 50, 'Alice');
      
      const stats = cursorTracker.getStats();
      expect(stats.localPosition).toBe(10);
      expect(stats.remoteCursorCount).toBe(1);
      expect(stats.historySize).toBeGreaterThan(0);
    });
  });

  describe('cleanup', () => {
    it('should cleanup expired cursors', () => {
      cursorTracker.updateRemoteCursor('user1', 50, 'Alice');
      
      // Mock old timestamp
      const remoteCursor = cursorTracker.getRemoteCursor('user1');
      if (remoteCursor) {
        remoteCursor.lastUpdated = new Date(Date.now() - 35000); // 35 seconds ago
      }
      
      cursorTracker.cleanupExpiredCursors();
      
      const cursor = cursorTracker.getRemoteCursor('user1');
      expect(cursor).toBeUndefined();
    });

    it('should clear all remote cursors', () => {
      cursorTracker.updateRemoteCursor('user1', 50, 'Alice');
      cursorTracker.updateRemoteCursor('user2', 75, 'Bob');
      
      cursorTracker.clearRemoteCursors();
      
      const cursors = cursorTracker.getRemoteCursors();
      expect(cursors.size).toBe(0);
    });
  });

  describe('configuration', () => {
    it('should get current config', () => {
      const config = cursorTracker.getConfig();
      expect(config.showCursor).toBe(true);
      expect(config.showUserName).toBe(true);
    });

    it('should update config', () => {
      cursorTracker.updateConfig({ showUserName: false });
      
      const config = cursorTracker.getConfig();
      expect(config.showUserName).toBe(false);
    });
  });

  describe('destroy', () => {
    it('should cleanup all resources', () => {
      cursorTracker.trackLocalPosition(50);
      cursorTracker.updateRemoteCursor('user1', 50, 'Alice');
      
      cursorTracker.destroy();
      
      const stats = cursorTracker.getStats();
      expect(stats.remoteCursorCount).toBe(0);
      expect(stats.historySize).toBe(0);
    });
  });
});

describe('CursorTracker singleton', () => {
  beforeEach(() => {
    destroyCursorTracker();
  });

  it('should create singleton instance', () => {
    const tracker1 = getCursorTracker();
    const tracker2 = getCursorTracker();
    
    expect(tracker1).toBe(tracker2);
  });

  it('should set editor on existing instance', () => {
    const tracker1 = getCursorTracker();
    
    const mockEditor = {
      isDestroyed: false,
      state: {
        selection: { from: 42, to: 42 }
      }
    } as any;
    
    const tracker2 = getCursorTracker(mockEditor);
    expect(tracker1).toBe(tracker2);
  });

  it('should destroy singleton', () => {
    const tracker1 = getCursorTracker();
    destroyCursorTracker();
    const tracker2 = getCursorTracker();
    
    expect(tracker1).not.toBe(tracker2);
  });
});
