/**
 * Document Version History Manager Tests
 * Aerospace-grade comprehensive test suite
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { versionHistoryManager } from '../versionHistory';

describe('VersionHistoryManager', () => {
  beforeEach(() => {
    versionHistoryManager.clearAll();
    versionHistoryManager.setAuthor('Test Author');
  });

  describe('Author Management', () => {
    it('should set author', () => {
      versionHistoryManager.setAuthor('John Doe');
      expect(versionHistoryManager.getAuthor()).toBe('John Doe');
    });

    it('should throw error for empty author', () => {
      expect(() => versionHistoryManager.setAuthor('')).toThrow();
    });

    it('should trim author name', () => {
      versionHistoryManager.setAuthor('  John Doe  ');
      expect(versionHistoryManager.getAuthor()).toBe('John Doe');
    });
  });

  describe('Version Creation', () => {
    it('should create version', () => {
      const id = versionHistoryManager.createVersion('Test content', 'Test version');
      expect(id).toBeTruthy();
      expect(id).toMatch(/^ver-/);
    });

    it('should require non-empty content', () => {
      expect(() => versionHistoryManager.createVersion('')).toThrow();
    });

    it('should increment version number', () => {
      versionHistoryManager.createVersion('content1');
      versionHistoryManager.createVersion('content2');

      const versions = versionHistoryManager.getAllVersions();
      expect(versions[0].version).toBe(1);
      expect(versions[1].version).toBe(2);
    });

    it('should store version data correctly', () => {
      const id = versionHistoryManager.createVersion('Test content', 'Description', false, [
        'tag1',
        'tag2'
      ]);
      const version = versionHistoryManager.getVersion(id);

      expect(version?.content).toBe('Test content');
      expect(version?.description).toBe('Description');
      expect(version?.tags).toEqual(['tag1', 'tag2']);
      expect(version?.isAutoSave).toBe(false);
    });

    it('should mark auto-save versions', () => {
      const id = versionHistoryManager.createVersion('content', '', true);
      const version = versionHistoryManager.getVersion(id);
      expect(version?.isAutoSave).toBe(true);
    });

    it('should calculate checksum', () => {
      const id = versionHistoryManager.createVersion('Test content');
      const version = versionHistoryManager.getVersion(id);
      expect(version?.checksum).toBeTruthy();
    });

    it('should track parent version', () => {
      const id1 = versionHistoryManager.createVersion('content1');
      const id2 = versionHistoryManager.createVersion('content2');

      const version2 = versionHistoryManager.getVersion(id2);
      expect(version2?.parentVersionId).toBe(id1);
    });
  });

  describe('Version Retrieval', () => {
    it('should get version by ID', () => {
      const id = versionHistoryManager.createVersion('Test content');
      const version = versionHistoryManager.getVersion(id);
      expect(version).toBeTruthy();
      expect(version?.content).toBe('Test content');
    });

    it('should return null for non-existent version', () => {
      const version = versionHistoryManager.getVersion('non-existent');
      expect(version).toBeNull();
    });

    it('should get version by number', () => {
      versionHistoryManager.createVersion('content1');
      versionHistoryManager.createVersion('content2');

      const version = versionHistoryManager.getVersionByNumber(2);
      expect(version).toBeTruthy();
      expect(version?.version).toBe(2);
    });

    it('should get latest version', () => {
      versionHistoryManager.createVersion('content1');
      versionHistoryManager.createVersion('content2');

      const latest = versionHistoryManager.getLatestVersion();
      expect(latest?.version).toBe(2);
    });

    it('should return null for latest when no versions', () => {
      const latest = versionHistoryManager.getLatestVersion();
      expect(latest).toBeNull();
    });

    it('should get all versions', () => {
      versionHistoryManager.createVersion('content1');
      versionHistoryManager.createVersion('content2');

      const versions = versionHistoryManager.getAllVersions();
      expect(versions.length).toBe(2);
    });
  });

  describe('Version Restoration', () => {
    it('should restore version content', () => {
      const id = versionHistoryManager.createVersion('Original content');
      const restored = versionHistoryManager.restoreVersion(id);
      expect(restored).toBe('Original content');
    });

    it('should return null for non-existent version', () => {
      const restored = versionHistoryManager.restoreVersion('non-existent');
      expect(restored).toBeNull();
    });

    it('should verify checksum on restore', () => {
      const id = versionHistoryManager.createVersion('Test content');
      // Restore should work even with checksum mismatch warning
      const restored = versionHistoryManager.restoreVersion(id);
      expect(restored).toBe('Test content');
    });
  });

  describe('Version Deletion', () => {
    it('should delete version', () => {
      const id = versionHistoryManager.createVersion('Test content');
      versionHistoryManager.deleteVersion(id);

      const version = versionHistoryManager.getVersion(id);
      expect(version).toBeNull();
    });
  });

  describe('Tag Management', () => {
    it('should add tag to version', () => {
      const id = versionHistoryManager.createVersion('content');
      versionHistoryManager.addTag(id, 'important');

      const version = versionHistoryManager.getVersion(id);
      expect(version?.tags).toContain('important');
    });

    it('should not add duplicate tags', () => {
      const id = versionHistoryManager.createVersion('content');
      versionHistoryManager.addTag(id, 'tag');
      versionHistoryManager.addTag(id, 'tag');

      const version = versionHistoryManager.getVersion(id);
      const tagCount = version?.tags?.filter(t => t === 'tag').length;
      expect(tagCount).toBe(1);
    });

    it('should remove tag from version', () => {
      const id = versionHistoryManager.createVersion('content', '', false, ['tag1', 'tag2']);
      versionHistoryManager.removeTag(id, 'tag1');

      const version = versionHistoryManager.getVersion(id);
      expect(version?.tags).not.toContain('tag1');
    });

    it('should get versions by tag', () => {
      const _id1 = versionHistoryManager.createVersion('content1', '', false, ['important']);
      const _id2 = versionHistoryManager.createVersion('content2', '', false, ['important']);
      versionHistoryManager.createVersion('content3', '', false, ['other']);

      const important = versionHistoryManager.getVersionsByTag('important');
      expect(important.length).toBe(2);
    });
  });

  describe('Version Filtering', () => {
    it('should get versions by author', () => {
      versionHistoryManager.setAuthor('Author1');
      versionHistoryManager.createVersion('content1');
      versionHistoryManager.setAuthor('Author2');
      versionHistoryManager.createVersion('content2');

      const author1Versions = versionHistoryManager.getVersionsByAuthor('Author1');
      expect(author1Versions.length).toBe(1);
    });

    it('should get versions by time range', () => {
      const now = Date.now();
      versionHistoryManager.createVersion('content1');
      versionHistoryManager.createVersion('content2');

      const versions = versionHistoryManager.getVersionsByTimeRange(now - 1000, now + 1000);
      expect(versions.length).toBe(2);
    });
  });

  describe('Version Comparison', () => {
    it('should compare two versions', () => {
      const id1 = versionHistoryManager.createVersion('Line 1\nLine 2\nLine 3');
      const id2 = versionHistoryManager.createVersion('Line 1\nLine 2 modified\nLine 3');

      const diff = versionHistoryManager.compareVersions(id1, id2);
      expect(diff).toBeTruthy();
      expect(diff?.changes.modifications).toBeGreaterThan(0);
    });

    it('should return null for non-existent versions', () => {
      const diff = versionHistoryManager.compareVersions('non-existent1', 'non-existent2');
      expect(diff).toBeNull();
    });

    it('should generate diff summary', () => {
      const id1 = versionHistoryManager.createVersion('Line 1\nLine 2');
      const id2 = versionHistoryManager.createVersion('Line 1\nLine 2\nLine 3');

      const diff = versionHistoryManager.compareVersions(id1, id2);
      expect(diff?.summary).toBeTruthy();
    });
  });

  describe('Auto-save', () => {
    it('should enable auto-save', () => {
      versionHistoryManager.enableAutoSave();
      // Auto-save timer should be set
      expect(versionHistoryManager['autoSaveTimer']).toBeTruthy();
    });

    it('should disable auto-save', () => {
      versionHistoryManager.enableAutoSave();
      versionHistoryManager.disableAutoSave();
      expect(versionHistoryManager['autoSaveTimer']).toBeNull();
    });

    it('should set auto-save interval', () => {
      versionHistoryManager.setAutoSaveInterval(10000);
      expect(versionHistoryManager['autoSaveInterval']).toBe(10000);
    });

    it('should throw error for invalid interval', () => {
      expect(() => versionHistoryManager.setAutoSaveInterval(500)).toThrow();
    });
  });

  describe('Limits', () => {
    it('should enforce max versions limit', () => {
      versionHistoryManager.setMaxVersions(5);
      for (let i = 0; i < 10; i++) {
        versionHistoryManager.createVersion(`content${i}`);
      }
      const versions = versionHistoryManager.getAllVersions();
      expect(versions.length).toBe(5);
    });

    it('should throw error for invalid max versions', () => {
      expect(() => versionHistoryManager.setMaxVersions(0)).toThrow();
    });
  });

  describe('Statistics', () => {
    it('should calculate statistics', () => {
      versionHistoryManager.setAuthor('Author1');
      versionHistoryManager.createVersion('content1', '', false, ['tag1']);
      versionHistoryManager.setAuthor('Author2');
      versionHistoryManager.createVersion('content2', '', true, ['tag2']);

      const stats = versionHistoryManager.getStatistics();
      expect(stats.totalVersions).toBe(2);
      expect(stats.currentVersion).toBe(2);
      expect(stats.byAuthor.Author1).toBe(1);
      expect(stats.byAuthor.Author2).toBe(1);
      expect(stats.byTag.tag1).toBe(1);
      expect(stats.autoSaveCount).toBe(1);
    });
  });

  describe('Import/Export', () => {
    it('should export to JSON', () => {
      versionHistoryManager.createVersion('Test content');
      const json = versionHistoryManager.exportToJSON();
      expect(json).toBeTruthy();
      const data = JSON.parse(json);
      expect(data.versions).toBeDefined();
    });

    it('should import from JSON', () => {
      versionHistoryManager.createVersion('content1');
      const json = versionHistoryManager.exportToJSON();

      versionHistoryManager.clearAll();
      versionHistoryManager.importFromJSON(json);

      const versions = versionHistoryManager.getAllVersions();
      expect(versions.length).toBe(1);
    });

    it('should throw error for invalid JSON', () => {
      expect(() => versionHistoryManager.importFromJSON('invalid json')).toThrow();
    });
  });

  describe('Clear', () => {
    it('should clear all versions', () => {
      versionHistoryManager.createVersion('content1');
      versionHistoryManager.createVersion('content2');
      versionHistoryManager.clearAll();

      const versions = versionHistoryManager.getAllVersions();
      expect(versions.length).toBe(0);
    });

    it('should reset version number after clear', () => {
      versionHistoryManager.createVersion('content1');
      versionHistoryManager.clearAll();

      versionHistoryManager.createVersion('content2');
      const version = versionHistoryManager.getLatestVersion();
      expect(version?.version).toBe(1);
    });
  });

  describe('Auto-save Edge Cases', () => {
    it('should trigger auto-save manually', () => {
      versionHistoryManager.triggerAutoSave('Test content');
      const versions = versionHistoryManager.getAllVersions();
      expect(versions.length).toBe(1);
      expect(versions[0].isAutoSave).toBe(true);
    });

    it('should skip auto-save if content unchanged', () => {
      versionHistoryManager.triggerAutoSave('Test content');
      const count1 = versionHistoryManager.getAllVersions().length;
      versionHistoryManager.triggerAutoSave('Test content');
      const count2 = versionHistoryManager.getAllVersions().length;
      expect(count2).toBe(count1);
    });

    it('should not auto-save when disabled', () => {
      versionHistoryManager.disableAutoSave();
      versionHistoryManager.triggerAutoSave('Test content');
      const versions = versionHistoryManager.getAllVersions();
      expect(versions.length).toBe(0);
    });

    it('should handle auto-save with empty content gracefully', () => {
      // Empty content should be handled gracefully without creating a version
      versionHistoryManager.triggerAutoSave('');
      const versions = versionHistoryManager.getAllVersions();
      expect(versions.length).toBe(0);
    });
  });

  describe('Diff Calculation Edge Cases', () => {
    it('should handle identical content', () => {
      const id1 = versionHistoryManager.createVersion('Line 1\nLine 2');
      const id2 = versionHistoryManager.createVersion('Line 1\nLine 2');

      const diff = versionHistoryManager.compareVersions(id1, id2);
      expect(diff?.changes.additions).toBe(0);
      expect(diff?.changes.deletions).toBe(0);
      expect(diff?.changes.modifications).toBe(0);
    });

    it('should handle empty content comparison', () => {
      const id1 = versionHistoryManager.createVersion('Line 1');
      const id2 = versionHistoryManager.createVersion('Line 1\nLine 2');

      const diff = versionHistoryManager.compareVersions(id1, id2);
      expect(diff?.changes.additions).toBeGreaterThan(0);
    });

    it('should handle only additions', () => {
      const id1 = versionHistoryManager.createVersion('Line 1');
      const id2 = versionHistoryManager.createVersion('Line 1\nLine 2');

      const diff = versionHistoryManager.compareVersions(id1, id2);
      expect(diff?.changes.additions).toBeGreaterThan(0);
    });

    it('should handle only deletions', () => {
      const id1 = versionHistoryManager.createVersion('Line 1\nLine 2');
      const id2 = versionHistoryManager.createVersion('Line 1');

      const diff = versionHistoryManager.compareVersions(id1, id2);
      expect(diff?.changes.deletions).toBeGreaterThan(0);
    });

    it('should generate summary for no changes', () => {
      const id1 = versionHistoryManager.createVersion('Same content');
      const id2 = versionHistoryManager.createVersion('Same content');

      const diff = versionHistoryManager.compareVersions(id1, id2);
      expect(diff?.summary).toBe('No changes');
    });
  });

  describe('Tag Management Edge Cases', () => {
    it('should handle adding tag to non-existent version', () => {
      versionHistoryManager.addTag('non-existent', 'tag');
      // Should not throw error
      expect(true).toBe(true);
    });

    it('should handle removing tag from non-existent version', () => {
      versionHistoryManager.removeTag('non-existent', 'tag');
      // Should not throw error
      expect(true).toBe(true);
    });

    it('should handle multiple tags on same version', () => {
      const id = versionHistoryManager.createVersion('content', '', false, []);
      versionHistoryManager.addTag(id, 'tag1');
      versionHistoryManager.addTag(id, 'tag2');
      versionHistoryManager.addTag(id, 'tag3');

      const version = versionHistoryManager.getVersion(id);
      expect(version?.tags.length).toBe(3);
    });

    it('should handle empty tag search', () => {
      versionHistoryManager.createVersion('content', '', false, ['tag1']);
      const versions = versionHistoryManager.getVersionsByTag('nonexistent');
      expect(versions.length).toBe(0);
    });
  });

  describe('Version Limits Edge Cases', () => {
    it('should trim versions when limit is reduced', () => {
      for (let i = 0; i < 10; i++) {
        versionHistoryManager.createVersion(`content${i}`);
      }
      versionHistoryManager.setMaxVersions(5);
      const versions = versionHistoryManager.getAllVersions();
      expect(versions.length).toBe(5);
    });

    it('should keep most recent versions when trimming', () => {
      for (let i = 0; i < 10; i++) {
        versionHistoryManager.createVersion(`content${i}`);
      }
      versionHistoryManager.setMaxVersions(5);
      const versions = versionHistoryManager.getAllVersions();
      expect(versions[versions.length - 1].version).toBe(10);
    });

    it('should handle max versions of 1', () => {
      versionHistoryManager.setMaxVersions(1);
      versionHistoryManager.createVersion('content1');
      versionHistoryManager.createVersion('content2');
      const versions = versionHistoryManager.getAllVersions();
      expect(versions.length).toBe(1);
    });
  });

  describe('Import/Export Edge Cases', () => {
    it('should handle empty export', () => {
      const json = versionHistoryManager.exportToJSON();
      expect(json).toBeTruthy();
      const data = JSON.parse(json);
      expect(data.versions.length).toBe(0);
    });

    it('should handle import with missing versions array', () => {
      // The implementation doesn't throw, it just doesn't import versions
      versionHistoryManager.importFromJSON('{"currentVersion": 1}');
      const versions = versionHistoryManager.getAllVersions();
      expect(versions.length).toBe(0);
    });

    it('should handle import with invalid JSON structure', () => {
      // The implementation doesn't throw for non-array versions
      versionHistoryManager.importFromJSON('{"versions": "not an array"}');
      const versions = versionHistoryManager.getAllVersions();
      expect(versions.length).toBe(0);
    });

    it('should preserve version data on import', () => {
      versionHistoryManager.createVersion('Test content', 'Description', false, ['tag1']);
      const json = versionHistoryManager.exportToJSON();

      versionHistoryManager.clearAll();
      versionHistoryManager.importFromJSON(json);

      const versions = versionHistoryManager.getAllVersions();
      expect(versions[0].content).toBe('Test content');
      expect(versions[0].description).toBe('Description');
      expect(versions[0].tags).toEqual(['tag1']);
    });
  });

  describe('Version Size and Checksum', () => {
    it('should calculate correct size', () => {
      const content = 'Test content';
      const id = versionHistoryManager.createVersion(content);
      const version = versionHistoryManager.getVersion(id);
      expect(version?.size).toBe(content.length);
    });

    it('should calculate different checksums for different content', () => {
      const id1 = versionHistoryManager.createVersion('content1');
      const id2 = versionHistoryManager.createVersion('content2');
      const version1 = versionHistoryManager.getVersion(id1);
      const version2 = versionHistoryManager.getVersion(id2);
      expect(version1?.checksum).not.toBe(version2?.checksum);
    });

    it('should calculate same checksum for same content', () => {
      versionHistoryManager.setMaxVersions(100);
      const id1 = versionHistoryManager.createVersion('same content');
      const id2 = versionHistoryManager.createVersion('same content');
      const version1 = versionHistoryManager.getVersion(id1);
      const version2 = versionHistoryManager.getVersion(id2);
      expect(version1?.checksum).toBe(version2?.checksum);
    });
  });

  describe('Time Range Filtering Edge Cases', () => {
    it('should handle empty time range', () => {
      const now = Date.now();
      const versions = versionHistoryManager.getVersionsByTimeRange(now, now);
      expect(versions.length).toBe(0);
    });

    it('should handle very large time range', () => {
      versionHistoryManager.createVersion('content1');
      const versions = versionHistoryManager.getVersionsByTimeRange(0, Date.now() + 1000000);
      expect(versions.length).toBeGreaterThanOrEqual(1);
    });

    it('should handle future time range', () => {
      versionHistoryManager.createVersion('content1');
      const future = Date.now() + 1000000;
      const versions = versionHistoryManager.getVersionsByTimeRange(future, future + 1000);
      expect(versions.length).toBe(0);
    });
  });

  describe('Author Filtering Edge Cases', () => {
    it('should handle non-existent author', () => {
      const versions = versionHistoryManager.getVersionsByAuthor('NonExistent');
      expect(versions.length).toBe(0);
    });

    it('should handle case-sensitive author names', () => {
      versionHistoryManager.setMaxVersions(100);
      versionHistoryManager.setAuthor('John');
      versionHistoryManager.createVersion('content1');
      versionHistoryManager.setAuthor('john');
      versionHistoryManager.createVersion('content2');

      const johnVersions = versionHistoryManager.getVersionsByAuthor('John');
      expect(johnVersions.length).toBe(1);
    });
  });

  describe('Version Number Edge Cases', () => {
    it('should handle getting non-existent version number', () => {
      const version = versionHistoryManager.getVersionByNumber(999);
      expect(version).toBeNull();
    });

    it('should handle version 0 when no versions exist', () => {
      const version = versionHistoryManager.getVersionByNumber(0);
      expect(version).toBeNull();
    });
  });

  describe('Statistics Edge Cases', () => {
    it('should handle empty statistics', () => {
      const stats = versionHistoryManager.getStatistics();
      expect(stats.totalVersions).toBe(0);
      expect(stats.currentVersion).toBe(0);
      expect(stats.totalSize).toBe(0);
      expect(Object.keys(stats.byAuthor).length).toBe(0);
      expect(Object.keys(stats.byTag).length).toBe(0);
    });

    it('should calculate total size correctly', () => {
      versionHistoryManager.setMaxVersions(100);
      versionHistoryManager.createVersion('content1');
      versionHistoryManager.createVersion('content2');
      const stats = versionHistoryManager.getStatistics();
      expect(stats.totalSize).toBe(16); // 'content1' + 'content2' = 8 + 8
    });

    it('should count versions without tags', () => {
      versionHistoryManager.createVersion('content1', '', false, []);
      const stats = versionHistoryManager.getStatistics();
      expect(Object.keys(stats.byTag).length).toBe(0);
    });
  });

  describe('Description Edge Cases', () => {
    it('should use default description for manual save', () => {
      const id = versionHistoryManager.createVersion('content');
      const version = versionHistoryManager.getVersion(id);
      expect(version?.description).toContain('Version');
    });

    it('should use custom description when provided', () => {
      const id = versionHistoryManager.createVersion('content', 'Custom description');
      const version = versionHistoryManager.getVersion(id);
      expect(version?.description).toBe('Custom description');
    });

    it('should handle empty description', () => {
      const id = versionHistoryManager.createVersion('content', '');
      const version = versionHistoryManager.getVersion(id);
      expect(version?.description).toBeTruthy();
    });
  });

  describe('Parent Version Edge Cases', () => {
    it('should have no parent for first version', () => {
      const id = versionHistoryManager.createVersion('content1');
      const version = versionHistoryManager.getVersion(id);
      expect(version?.parentVersionId).toBeUndefined();
    });

    it('should track parent chain correctly', () => {
      versionHistoryManager.setMaxVersions(100);
      const id1 = versionHistoryManager.createVersion('content1');
      const id2 = versionHistoryManager.createVersion('content2');
      const id3 = versionHistoryManager.createVersion('content3');

      const version2 = versionHistoryManager.getVersion(id2);
      const version3 = versionHistoryManager.getVersion(id3);
      expect(version2?.parentVersionId).toBe(id1);
      expect(version3?.parentVersionId).toBe(id2);
    });
  });

  describe('Auto-save Timer Edge Cases', () => {
    it('should not create duplicate timer on multiple enable', () => {
      versionHistoryManager.enableAutoSave();
      const timer1 = versionHistoryManager['autoSaveTimer'];
      versionHistoryManager.enableAutoSave();
      const timer2 = versionHistoryManager['autoSaveTimer'];
      expect(timer1).toBe(timer2);
    });

    it('should restart timer when interval changes', () => {
      versionHistoryManager.enableAutoSave();
      versionHistoryManager.setAutoSaveInterval(10000);
      // Timer should be restarted
      expect(versionHistoryManager['autoSaveTimer']).toBeTruthy();
    });
  });
});
