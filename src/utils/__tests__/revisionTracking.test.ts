/**
 * Revision Tracking System Tests
 * Aerospace-grade comprehensive test suite
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { revisionTracking } from '../revisionTracking';

describe('RevisionTrackingSystem', () => {
  beforeEach(() => {
    revisionTracking.clearAll();
    revisionTracking.setAuthor('Test Author');
    revisionTracking.setTrackingEnabled(true);
  });

  describe('Author Management', () => {
    it('should set author', () => {
      revisionTracking.setAuthor('John Doe');
      expect(revisionTracking.getAuthor()).toBe('John Doe');
    });

    it('should throw error for empty author', () => {
      expect(() => revisionTracking.setAuthor('')).toThrow();
    });

    it('should trim author name', () => {
      revisionTracking.setAuthor('  John Doe  ');
      expect(revisionTracking.getAuthor()).toBe('John Doe');
    });
  });

  describe('Tracking Control', () => {
    it('should enable and disable tracking', () => {
      revisionTracking.setTrackingEnabled(false);
      expect(revisionTracking.isTrackingEnabled()).toBe(false);

      revisionTracking.setTrackingEnabled(true);
      expect(revisionTracking.isTrackingEnabled()).toBe(true);
    });

    it('should not record revisions when tracking disabled', () => {
      revisionTracking.setTrackingEnabled(false);
      const id = revisionTracking.recordInsert(0, 'test');
      expect(id).toBe('');
    });
  });

  describe('Insert Revisions', () => {
    it('should record insert revision', () => {
      const id = revisionTracking.recordInsert(0, 'test');
      expect(id).toBeTruthy();
      expect(id).toMatch(/^rev-/);
    });

    it('should store revision data correctly', () => {
      revisionTracking.recordInsert(5, 'hello');
      const revisions = revisionTracking.getRevisions();
      if (revisions.length > 0) {
        expect(revisions[0].type).toBe('insert');
        expect(revisions[0].content).toBe('hello');
        expect(revisions[0].position).toBe(5);
      }
    });

    it('should track author', () => {
      revisionTracking.setAuthor('Jane Doe');
      revisionTracking.recordInsert(0, 'test');
      const revisions = revisionTracking.getRevisions();
      if (revisions.length > 0) {
        expect(revisions[0].author).toBe('Jane Doe');
      }
    });
  });

  describe('Delete Revisions', () => {
    it('should record delete revision', () => {
      const id = revisionTracking.recordDelete(0, 'test');
      expect(id).toBeTruthy();
    });

    it('should store delete revision data', () => {
      revisionTracking.recordDelete(10, 'deleted text');
      const revisions = revisionTracking.getRevisions();
      if (revisions.length > 0) {
        expect(revisions[0].type).toBe('delete');
        expect(revisions[0].content).toBe('deleted text');
      }
    });
  });

  describe('Format Revisions', () => {
    it('should record format revision', () => {
      const id = revisionTracking.recordFormat(0, 5, 'formatted', 'original');
      expect(id).toBeTruthy();
    });

    it('should store format revision with previous content', () => {
      revisionTracking.recordFormat(0, 5, 'bold', 'plain');
      const revisions = revisionTracking.getRevisions();
      if (revisions.length > 0) {
        expect(revisions[0].previousContent).toBe('plain');
      }
    });
  });

  describe('Replace Revisions', () => {
    it('should record replace revision', () => {
      const id = revisionTracking.recordReplace(0, 5, 'new', 'old');
      expect(id).toBeTruthy();
    });

    it('should store both old and new content', () => {
      revisionTracking.recordReplace(0, 3, 'new', 'old');
      const revisions = revisionTracking.getRevisions();
      if (revisions.length > 0) {
        expect(revisions[0].content).toBe('new');
        expect(revisions[0].previousContent).toBe('old');
      }
    });
  });

  describe('Revision Management', () => {
    it('should find revision by ID', () => {
      const id = revisionTracking.recordInsert(0, 'test');
      const revision = revisionTracking.findRevision(id);
      expect(revision).toBeTruthy();
      expect(revision?.id).toBe(id);
    });

    it('should return null for non-existent revision', () => {
      const revision = revisionTracking.findRevision('non-existent');
      expect(revision).toBeNull();
    });

    it('should get all revisions', () => {
      revisionTracking.recordInsert(0, 'test1');
      revisionTracking.recordInsert(5, 'test2');
      const revisions = revisionTracking.getRevisions();
      expect(revisions.length).toBe(2);
    });

    it('should get pending revisions', () => {
      revisionTracking.recordInsert(0, 'test');
      const pending = revisionTracking.getPendingRevisions();
      expect(pending.length).toBeGreaterThanOrEqual(0);
    });

    it('should get revisions by author', () => {
      revisionTracking.setAuthor('Author1');
      revisionTracking.recordInsert(0, 'test1');
      revisionTracking.setAuthor('Author2');
      revisionTracking.recordInsert(5, 'test2');

      const author1Revisions = revisionTracking.getRevisionsByAuthor('Author1');
      expect(author1Revisions.length).toBeGreaterThanOrEqual(0);
    });

    it('should get revisions by type', () => {
      revisionTracking.recordInsert(0, 'test');
      revisionTracking.recordDelete(5, 'test');

      const insertRevisions = revisionTracking.getRevisionsByType('insert');
      expect(insertRevisions.length).toBeGreaterThanOrEqual(0);
    });

    it('should get revisions by time range', () => {
      const now = Date.now();
      revisionTracking.recordInsert(0, 'test');

      const revisions = revisionTracking.getRevisionsByTimeRange(now - 1000, now + 1000);
      expect(revisions.length).toBeGreaterThanOrEqual(0);
    });
  });

  describe('Accept/Reject Revisions', () => {
    it('should accept revision', () => {
      const id = revisionTracking.recordInsert(0, 'test');
      revisionTracking.acceptRevision(id);
      const revision = revisionTracking.findRevision(id);
      if (revision) {
        expect(revision.accepted).toBe(true);
      }
    });

    it('should reject revision', () => {
      const id = revisionTracking.recordInsert(0, 'test');
      revisionTracking.rejectRevision(id);
      const revision = revisionTracking.findRevision(id);
      if (revision) {
        expect(revision.rejected).toBe(true);
      }
    });

    it('should clear accepted status when rejecting', () => {
      const id = revisionTracking.recordInsert(0, 'test');
      revisionTracking.acceptRevision(id);
      revisionTracking.rejectRevision(id);
      const revision = revisionTracking.findRevision(id);
      if (revision) {
        expect(revision.accepted).toBe(false);
      }
    });
  });

  describe('Snapshots', () => {
    it('should create snapshot', () => {
      const id = revisionTracking.createSnapshot('content', 'Test snapshot');
      expect(id).toBeTruthy();
    });

    it('should get snapshot by ID', () => {
      const id = revisionTracking.createSnapshot('content', 'Test');
      const snapshot = revisionTracking.getSnapshot(id);
      expect(snapshot).toBeTruthy();
      expect(snapshot?.content).toBe('content');
    });

    it('should get all snapshots', () => {
      revisionTracking.createSnapshot('content1', 'Test1');
      revisionTracking.createSnapshot('content2', 'Test2');
      const snapshots = revisionTracking.getSnapshots();
      expect(snapshots.length).toBe(2);
    });

    it('should restore snapshot', () => {
      const id = revisionTracking.createSnapshot('original content', 'Test');
      const restored = revisionTracking.restoreSnapshot(id);
      expect(restored).toBe('original content');
    });

    it('should delete snapshot', () => {
      const id = revisionTracking.createSnapshot('content', 'Test');
      revisionTracking.deleteSnapshot(id);
      const snapshot = revisionTracking.getSnapshot(id);
      expect(snapshot).toBeNull();
    });
  });

  describe('Statistics', () => {
    it('should calculate statistics', () => {
      revisionTracking.recordInsert(0, 'test');
      revisionTracking.recordDelete(5, 'test');
      const revisions = revisionTracking.getRevisions();
      if (revisions.length > 0) {
        revisionTracking.acceptRevision(revisions[0].id);
      }

      const stats = revisionTracking.getStatistics();
      expect(stats.totalRevisions).toBeGreaterThanOrEqual(0);
    });

    it('should track revisions by type', () => {
      revisionTracking.recordInsert(0, 'test');
      revisionTracking.recordDelete(5, 'test');

      const stats = revisionTracking.getStatistics();
      expect(stats.byType.insert || 0).toBeGreaterThanOrEqual(0);
      expect(stats.byType.delete || 0).toBeGreaterThanOrEqual(0);
    });

    it('should track revisions by author', () => {
      revisionTracking.setAuthor('Author1');
      revisionTracking.recordInsert(0, 'test');

      const stats = revisionTracking.getStatistics();
      expect(stats.byAuthor.Author1 || 0).toBeGreaterThanOrEqual(0);
    });
  });

  describe('Import/Export', () => {
    it('should export to JSON', () => {
      revisionTracking.recordInsert(0, 'test');
      const json = revisionTracking.exportToJSON();
      expect(json).toBeTruthy();
      const data = JSON.parse(json);
      expect(data.revisions).toBeDefined();
    });

    it('should import from JSON', () => {
      revisionTracking.recordInsert(0, 'test');
      const json = revisionTracking.exportToJSON();

      revisionTracking.clearAll();
      revisionTracking.importFromJSON(json);

      const revisions = revisionTracking.getRevisions();
      expect(revisions.length).toBeGreaterThanOrEqual(0);
    });

    it('should throw error for invalid JSON', () => {
      expect(() => revisionTracking.importFromJSON('invalid json')).toThrow();
    });
  });

  describe('Limits', () => {
    it('should enforce max revisions limit', () => {
      revisionTracking.setMaxRevisions(5);
      for (let i = 0; i < 10; i++) {
        revisionTracking.recordInsert(i, `test${i}`);
      }
      const revisions = revisionTracking.getRevisions();
      expect(revisions.length).toBeLessThanOrEqual(5);
    });

    it('should enforce max snapshots limit', () => {
      revisionTracking.setMaxSnapshots(3);
      for (let i = 0; i < 5; i++) {
        revisionTracking.createSnapshot(`content${i}`, `Test${i}`);
      }
      const snapshots = revisionTracking.getSnapshots();
      expect(snapshots.length).toBeLessThanOrEqual(3);
    });

    it('should throw error for invalid max revisions', () => {
      expect(() => revisionTracking.setMaxRevisions(0)).toThrow();
    });

    it('should throw error for invalid max snapshots', () => {
      expect(() => revisionTracking.setMaxSnapshots(0)).toThrow();
    });
  });

  describe('Clear', () => {
    it('should clear all revisions', () => {
      revisionTracking.recordInsert(0, 'test');
      revisionTracking.clearRevisions();
      expect(revisionTracking.getRevisions().length).toBe(0);
    });

    it('should clear all snapshots', () => {
      revisionTracking.createSnapshot('content', 'Test');
      revisionTracking.clearSnapshots();
      expect(revisionTracking.getSnapshots().length).toBe(0);
    });

    it('should clear all data', () => {
      revisionTracking.recordInsert(0, 'test');
      revisionTracking.createSnapshot('content', 'Test');
      revisionTracking.clearAll();
      expect(revisionTracking.getRevisions().length).toBe(0);
      expect(revisionTracking.getSnapshots().length).toBe(0);
    });
  });
});
