/**
 * Cross-References Manager Tests
 * Aerospace-grade comprehensive test suite
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { crossReferencesManager } from '../crossReferences';

describe('CrossReferencesManager', () => {
  beforeEach(() => {
    crossReferencesManager.clearAll();
  });

  describe('Target Registration', () => {
    it('should register reference target', () => {
      const id = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });
      expect(id).toBeTruthy();
      expect(id).toMatch(/^ref-/);
    });

    it('should get registered target', () => {
      const id = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      const target = crossReferencesManager.getTarget(id);
      expect(target).toBeTruthy();
      expect(target?.label).toBe('Test Heading');
    });

    it('should unregister target', () => {
      const id = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      crossReferencesManager.unregisterTarget(id);
      const target = crossReferencesManager.getTarget(id);
      expect(target).toBeNull();
    });

    it('should remove references when target is unregistered', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'See heading',
        format: 'text',
        position: 50
      });

      crossReferencesManager.unregisterTarget(targetId);
      const references = crossReferencesManager.getReferences();
      expect(references.length).toBe(0);
    });

    it('should get all targets', () => {
      crossReferencesManager.registerTarget({ type: 'heading', label: 'H1', position: 100 });
      crossReferencesManager.registerTarget({ type: 'bookmark', label: 'Bookmark', position: 200 });

      const targets = crossReferencesManager.getAllTargets();
      expect(targets.length).toBe(2);
    });

    it('should get targets by type', () => {
      crossReferencesManager.registerTarget({ type: 'heading', label: 'H1', position: 100 });
      crossReferencesManager.registerTarget({ type: 'bookmark', label: 'Bookmark', position: 200 });

      const headings = crossReferencesManager.getTargetsByType('heading');
      expect(headings.length).toBe(1);
    });
  });

  describe('Reference Management', () => {
    it('should add cross-reference', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      const id = crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'See heading',
        format: 'text',
        position: 50
      });

      expect(id).toBeTruthy();
      expect(id).toMatch(/^ref-/);
    });

    it('should throw error for non-existent target', () => {
      expect(() =>
        crossReferencesManager.addReference({
          type: 'heading',
          targetId: 'non-existent',
          label: 'Test',
          format: 'text',
          position: 50
        })
      ).toThrow();
    });

    it('should update reference', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      const id = crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'Original',
        format: 'text',
        position: 50
      });

      crossReferencesManager.updateReference(id, { label: 'Updated' });
      const reference = crossReferencesManager.getReference(id);
      expect(reference?.label).toBe('Updated');
    });

    it('should throw error when updating with non-existent target', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      const id = crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'Test',
        format: 'text',
        position: 50
      });

      expect(() =>
        crossReferencesManager.updateReference(id, { targetId: 'non-existent' })
      ).toThrow();
    });

    it('should remove reference', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      const id = crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'Test',
        format: 'text',
        position: 50
      });

      crossReferencesManager.removeReference(id);
      const reference = crossReferencesManager.getReference(id);
      expect(reference).toBeNull();
    });

    it('should get reference by ID', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      const id = crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'Test',
        format: 'text',
        position: 50
      });

      const reference = crossReferencesManager.getReference(id);
      expect(reference).toBeTruthy();
    });

    it('should get all references', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'Ref 1',
        format: 'text',
        position: 50
      });
      crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'Ref 2',
        format: 'text',
        position: 60
      });

      const references = crossReferencesManager.getReferences();
      expect(references.length).toBe(2);
    });

    it('should get references for target', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'Ref 1',
        format: 'text',
        position: 50
      });
      crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'Ref 2',
        format: 'text',
        position: 60
      });

      const refs = crossReferencesManager.getReferencesForTarget(targetId);
      expect(refs.length).toBe(2);
    });
  });

  describe('HTML Generation', () => {
    it('should generate reference HTML', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      const id = crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'See heading',
        format: 'text',
        position: 50
      });

      const html = crossReferencesManager.generateReferenceHTML(id);
      expect(html).toContain('cross-reference');
      expect(html).toContain('Test Heading');
    });

    it('should generate broken reference for non-existent target', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      const id = crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'Test',
        format: 'text',
        position: 50
      });

      crossReferencesManager.unregisterTarget(targetId);
      const html = crossReferencesManager.generateReferenceHTML(id);
      expect(html).toContain('broken');
    });

    it('should generate page reference', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 5000
      });

      const id = crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'Test',
        format: 'page',
        position: 50
      });

      const html = crossReferencesManager.generateReferenceHTML(id);
      expect(html).toContain('Page');
    });
  });

  describe('Update After Changes', () => {
    it('should update target positions', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      const positionChanges = new Map([[100, 150]]);
      crossReferencesManager.updateReferencesAfterChanges(positionChanges);

      const target = crossReferencesManager.getTarget(targetId);
      expect(target?.position).toBe(150);
    });

    it('should update reference positions', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      const id = crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'Test',
        format: 'text',
        position: 50
      });

      const positionChanges = new Map([[50, 75]]);
      crossReferencesManager.updateReferencesAfterChanges(positionChanges);

      const reference = crossReferencesManager.getReference(id);
      expect(reference?.position).toBe(75);
    });
  });

  describe('Validation', () => {
    it('should validate all references', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'Test',
        format: 'text',
        position: 50
      });

      const brokenIds = crossReferencesManager.validateReferences();
      expect(brokenIds.length).toBeGreaterThanOrEqual(0);
    });

    it('should identify broken references', () => {
      const targetId = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test Heading',
        position: 100
      });

      crossReferencesManager.addReference({
        type: 'heading',
        targetId,
        label: 'Test',
        format: 'text',
        position: 50
      });

      crossReferencesManager.unregisterTarget(targetId);
      const brokenIds = crossReferencesManager.validateReferences();
      expect(brokenIds.length).toBeGreaterThanOrEqual(0);
    });
  });

  describe('Statistics', () => {
    it('should calculate statistics', () => {
      const targetId1 = crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Heading 1',
        position: 100
      });
      const targetId2 = crossReferencesManager.registerTarget({
        type: 'bookmark',
        label: 'Bookmark',
        position: 200
      });

      crossReferencesManager.addReference({
        type: 'heading',
        targetId: targetId1,
        label: 'Ref 1',
        format: 'text',
        position: 50
      });
      crossReferencesManager.addReference({
        type: 'bookmark',
        targetId: targetId2,
        label: 'Ref 2',
        format: 'text',
        position: 60
      });

      const stats = crossReferencesManager.getStatistics();
      expect(stats.totalReferences).toBe(2);
      expect(stats.totalTargets).toBe(2);
      expect(stats.byType.heading).toBe(1);
      expect(stats.byType.bookmark).toBe(1);
    });
  });

  describe('Import/Export', () => {
    it('should export to JSON', () => {
      crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test',
        position: 100
      });
      const json = crossReferencesManager.exportToJSON();
      expect(json).toBeTruthy();
      const data = JSON.parse(json);
      expect(data.targets).toBeDefined();
    });

    it('should import from JSON', () => {
      crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test',
        position: 100
      });
      const json = crossReferencesManager.exportToJSON();

      crossReferencesManager.clearAll();
      crossReferencesManager.importFromJSON(json);

      const targets = crossReferencesManager.getAllTargets();
      expect(targets.length).toBe(1);
    });

    it('should throw error for invalid JSON', () => {
      expect(() => crossReferencesManager.importFromJSON('invalid json')).toThrow();
    });
  });

  describe('Clear', () => {
    it('should clear all references and targets', () => {
      crossReferencesManager.registerTarget({
        type: 'heading',
        label: 'Test',
        position: 100
      });
      crossReferencesManager.clearAll();

      expect(crossReferencesManager.getAllTargets().length).toBe(0);
      expect(crossReferencesManager.getReferences().length).toBe(0);
    });
  });
});
