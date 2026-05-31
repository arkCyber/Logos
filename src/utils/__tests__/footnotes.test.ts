/**
 * Footnotes and Endnotes Management Tests
 * Aerospace-grade comprehensive test suite
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { footnoteManager } from '../footnotes';

describe('FootnoteManager', () => {
  beforeEach(() => {
    footnoteManager.clearAll();
  });

  describe('Footnote Management', () => {
    it('should add footnote', () => {
      const id = footnoteManager.addFootnote('Test footnote', 100);
      expect(id).toBeTruthy();
      expect(id).toMatch(/^note-/);
    });

    it('should require non-empty content', () => {
      expect(() => footnoteManager.addFootnote('', 100)).toThrow();
      expect(() => footnoteManager.addFootnote('   ', 100)).toThrow();
    });

    it('should trim content', () => {
      const id = footnoteManager.addFootnote('  Test footnote  ', 100);
      const footnote = footnoteManager.getNote(id);
      expect(footnote?.content).toBe('Test footnote');
    });

    it('should increment footnote number', () => {
      footnoteManager.addFootnote('Note 1', 100);
      footnoteManager.addFootnote('Note 2', 200);

      const footnotes = footnoteManager.getFootnotes();
      expect(footnotes[0].number).toBe(1);
      expect(footnotes[1].number).toBe(2);
    });

    it('should update footnote', () => {
      const id = footnoteManager.addFootnote('Original', 100);
      footnoteManager.updateNote(id, 'Updated');

      const footnote = footnoteManager.getNote(id);
      expect(footnote?.content).toBe('Updated');
    });

    it('should delete footnote', () => {
      const id = footnoteManager.addFootnote('Test', 100);
      footnoteManager.deleteNote(id);

      const footnote = footnoteManager.getNote(id);
      expect(footnote).toBeNull();
    });

    it('should renumber after deletion', () => {
      footnoteManager.addFootnote('Note 1', 100);
      footnoteManager.addFootnote('Note 2', 200);
      footnoteManager.addFootnote('Note 3', 300);

      footnoteManager.deleteNote(footnoteManager.getFootnotes()[1].id);

      const footnotes = footnoteManager.getFootnotes();
      expect(footnotes[0].number).toBe(1);
      expect(footnotes[1].number).toBe(2);
    });
  });

  describe('Endnote Management', () => {
    it('should add endnote', () => {
      const id = footnoteManager.addEndnote('Test endnote', 100);
      expect(id).toBeTruthy();
    });

    it('should require non-empty content', () => {
      expect(() => footnoteManager.addEndnote('', 100)).toThrow();
    });

    it('should increment endnote number', () => {
      footnoteManager.addEndnote('Note 1', 100);
      footnoteManager.addEndnote('Note 2', 200);

      const endnotes = footnoteManager.getEndnotes();
      expect(endnotes[0].number).toBe(1);
      expect(endnotes[1].number).toBe(2);
    });

    it('should update endnote', () => {
      const id = footnoteManager.addEndnote('Original', 100);
      footnoteManager.updateNote(id, 'Updated');

      const endnote = footnoteManager.getNote(id);
      expect(endnote?.content).toBe('Updated');
    });

    it('should delete endnote', () => {
      const id = footnoteManager.addEndnote('Test', 100);
      footnoteManager.deleteNote(id);

      const endnote = footnoteManager.getNote(id);
      expect(endnote).toBeNull();
    });
  });

  describe('Note Retrieval', () => {
    it('should get note by ID', () => {
      const id = footnoteManager.addFootnote('Test', 100);
      const note = footnoteManager.getNote(id);
      expect(note).toBeTruthy();
      expect(note?.content).toBe('Test');
    });

    it('should return null for non-existent note', () => {
      const note = footnoteManager.getNote('non-existent');
      expect(note).toBeNull();
    });

    it('should get all footnotes', () => {
      footnoteManager.addFootnote('Note 1', 100);
      footnoteManager.addFootnote('Note 2', 200);

      const footnotes = footnoteManager.getFootnotes();
      expect(footnotes.length).toBe(2);
    });

    it('should get all endnotes', () => {
      footnoteManager.addEndnote('Note 1', 100);
      footnoteManager.addEndnote('Note 2', 200);

      const endnotes = footnoteManager.getEndnotes();
      expect(endnotes.length).toBe(2);
    });

    it('should get all notes', () => {
      footnoteManager.addFootnote('Footnote', 100);
      footnoteManager.addEndnote('Endnote', 200);

      const allNotes = footnoteManager.getAllNotes();
      expect(allNotes.length).toBe(2);
    });

    it('should get footnotes by position range', () => {
      footnoteManager.addFootnote('Note 1', 100);
      footnoteManager.addFootnote('Note 2', 200);
      footnoteManager.addFootnote('Note 3', 300);

      const inRange = footnoteManager.getFootnotesByRange(150, 250);
      expect(inRange.length).toBe(1);
    });
  });

  describe('HTML Generation', () => {
    it('should generate footnotes HTML', () => {
      footnoteManager.addFootnote('Test footnote', 100);
      const html = footnoteManager.generateFootnotesHTML();
      expect(html).toContain('footnotes-section');
      expect(html).toContain('Test footnote');
    });

    it('should return empty string for no footnotes', () => {
      const html = footnoteManager.generateFootnotesHTML();
      expect(html).toBe('');
    });

    it('should generate endnotes HTML', () => {
      footnoteManager.addEndnote('Test endnote', 100);
      const html = footnoteManager.generateEndnotesHTML();
      expect(html).toContain('endnotes-section');
      expect(html).toContain('Test endnote');
    });

    it('should generate footnote reference', () => {
      const id = footnoteManager.addFootnote('Test', 100);
      const ref = footnoteManager.generateFootnoteReference(id);
      expect(ref).toContain('footnote-ref');
      expect(ref).toContain('1');
    });

    it('should generate endnote reference', () => {
      const id = footnoteManager.addEndnote('Test', 100);
      const ref = footnoteManager.generateEndnoteReference(id);
      expect(ref).toContain('endnote-ref');
      expect(ref).toContain('1');
    });

    it('should return empty string for non-existent reference', () => {
      const ref = footnoteManager.generateFootnoteReference('non-existent');
      expect(ref).toBe('');
    });
  });

  describe('Statistics', () => {
    it('should calculate statistics', () => {
      footnoteManager.addFootnote('Note 1', 100);
      footnoteManager.addFootnote('Note 2', 200);
      footnoteManager.addEndnote('Note 3', 300);

      const stats = footnoteManager.getStatistics();
      expect(stats.totalFootnotes).toBe(2);
      expect(stats.totalEndnotes).toBe(1);
      expect(stats.totalNotes).toBe(3);
    });
  });

  describe('Import/Export', () => {
    it('should export to JSON', () => {
      footnoteManager.addFootnote('Test', 100);
      const json = footnoteManager.exportToJSON();
      expect(json).toBeTruthy();
      const data = JSON.parse(json);
      expect(data.footnotes).toBeDefined();
    });

    it('should import from JSON', () => {
      footnoteManager.addFootnote('Test', 100);
      const json = footnoteManager.exportToJSON();

      footnoteManager.clearAll();
      footnoteManager.importFromJSON(json);

      const footnotes = footnoteManager.getFootnotes();
      expect(footnotes.length).toBe(1);
    });

    it('should throw error for invalid JSON', () => {
      expect(() => footnoteManager.importFromJSON('invalid json')).toThrow();
    });
  });

  describe('Clear', () => {
    it('should clear footnotes', () => {
      footnoteManager.addFootnote('Test', 100);
      footnoteManager.clearFootnotes();
      expect(footnoteManager.getFootnotes().length).toBe(0);
    });

    it('should clear endnotes', () => {
      footnoteManager.addEndnote('Test', 100);
      footnoteManager.clearEndnotes();
      expect(footnoteManager.getEndnotes().length).toBe(0);
    });

    it('should clear all notes', () => {
      footnoteManager.addFootnote('Footnote', 100);
      footnoteManager.addEndnote('Endnote', 200);
      footnoteManager.clearAll();

      expect(footnoteManager.getFootnotes().length).toBe(0);
      expect(footnoteManager.getEndnotes().length).toBe(0);
    });
  });

  describe('Error Handling', () => {
    it('should throw error when updating non-existent note', () => {
      expect(() => footnoteManager.updateNote('non-existent', 'content')).toThrow();
    });

    it('should throw error when deleting non-existent note', () => {
      expect(() => footnoteManager.deleteNote('non-existent')).toThrow();
    });

    it('should throw error when updating with empty content', () => {
      const id = footnoteManager.addFootnote('Test', 100);
      expect(() => footnoteManager.updateNote(id, '')).toThrow();
    });
  });
});
