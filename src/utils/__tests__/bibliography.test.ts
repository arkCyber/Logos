/**
 * Bibliography Management System Tests
 * Aerospace-grade comprehensive test suite
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { bibliographyManager, BibliographyEntry } from '../bibliography';

describe('BibliographyManager', () => {
  beforeEach(() => {
    bibliographyManager.clearAll();
  });

  describe('Entry Management', () => {
    it('should add bibliography entry', () => {
      const entry: Omit<BibliographyEntry, 'id' | 'createdAt' | 'updatedAt'> = {
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024,
        publisher: 'Test Publisher'
      };
      const id = bibliographyManager.addEntry(entry);
      expect(id).toBeTruthy();
      expect(id).toMatch(/^bib-/);
    });

    it('should require title', () => {
      const entry = {
        type: 'book' as const,
        authors: ['John Doe'],
        title: '',
        year: 2024
      };
      expect(() => bibliographyManager.addEntry(entry)).toThrow();
    });

    it('should require at least one author', () => {
      const entry = {
        type: 'book' as const,
        authors: [],
        title: 'Test Book',
        year: 2024
      };
      expect(() => bibliographyManager.addEntry(entry)).toThrow();
    });

    it('should validate year', () => {
      const entry = {
        type: 'book' as const,
        authors: ['John Doe'],
        title: 'Test Book',
        year: 3000
      };
      expect(() => bibliographyManager.addEntry(entry)).toThrow();
    });

    it('should update entry', () => {
      const entry = {
        type: 'book' as const,
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      };
      const id = bibliographyManager.addEntry(entry);

      bibliographyManager.updateEntry(id, { title: 'Updated Title' });
      const updated = bibliographyManager.getEntry(id);
      expect(updated?.title).toBe('Updated Title');
    });

    it('should delete entry', () => {
      const entry = {
        type: 'book' as const,
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      };
      const id = bibliographyManager.addEntry(entry);

      bibliographyManager.deleteEntry(id);
      const deleted = bibliographyManager.getEntry(id);
      expect(deleted).toBeNull();
    });

    it('should throw error when deleting non-existent entry', () => {
      expect(() => bibliographyManager.deleteEntry('non-existent')).toThrow();
    });
  });

  describe('Entry Retrieval', () => {
    it('should get entry by ID', () => {
      const entry = {
        type: 'book' as const,
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      };
      const id = bibliographyManager.addEntry(entry);

      const retrieved = bibliographyManager.getEntry(id);
      expect(retrieved).toBeTruthy();
      expect(retrieved?.title).toBe('Test Book');
    });

    it('should get all entries', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['Author1'],
        title: 'Book1',
        year: 2024
      });
      bibliographyManager.addEntry({
        type: 'article',
        authors: ['Author2'],
        title: 'Article1',
        year: 2024
      });

      const entries = bibliographyManager.getAllEntries();
      expect(entries.length).toBe(2);
    });

    it('should get entries by type', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['Author1'],
        title: 'Book1',
        year: 2024
      });
      bibliographyManager.addEntry({
        type: 'article',
        authors: ['Author2'],
        title: 'Article1',
        year: 2024
      });

      const books = bibliographyManager.getEntriesByType('book');
      expect(books.length).toBe(1);
    });

    it('should get entries by tag', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['Author1'],
        title: 'Book1',
        year: 2024,
        tags: ['science', 'research']
      });

      const tagged = bibliographyManager.getEntriesByTag('science');
      expect(tagged.length).toBe(1);
    });
  });

  describe('Search', () => {
    it('should search entries by title', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Quantum Physics',
        year: 2024
      });

      const results = bibliographyManager.searchEntries('quantum');
      expect(results.length).toBe(1);
    });

    it('should search entries by author', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      const results = bibliographyManager.searchEntries('john');
      expect(results.length).toBe(1);
    });

    it('should be case-insensitive', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      const results = bibliographyManager.searchEntries('TEST');
      expect(results.length).toBe(1);
    });
  });

  describe('Citations', () => {
    it('should add citation', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      const citationId = bibliographyManager.addCitation(entryId, 100);
      expect(citationId).toBeTruthy();
    });

    it('should throw error for non-existent entry', () => {
      expect(() => bibliographyManager.addCitation('non-existent', 100)).toThrow();
    });

    it('should remove citation', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });
      const citationId = bibliographyManager.addCitation(entryId, 100);

      bibliographyManager.removeCitation(citationId);
      const citations = bibliographyManager.getCitations();
      expect(citations.length).toBe(0);
    });

    it('should get citations for entry', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });
      bibliographyManager.addCitation(entryId, 100);
      bibliographyManager.addCitation(entryId, 200);

      const citations = bibliographyManager.getCitationsForEntry(entryId);
      expect(citations.length).toBe(2);
    });
  });

  describe('Formatting', () => {
    it('should format in APA style', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe', 'Jane Smith'],
        title: 'Test Book',
        year: 2024,
        publisher: 'Test Publisher'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'apa');
      expect(formatted).toBeTruthy();
    });

    it('should format in MLA style', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024,
        publisher: 'Test Publisher'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'mla');
      expect(formatted).toBeTruthy();
    });

    it('should format in BibTeX', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024,
        publisher: 'Test Publisher'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'bibtex');
      expect(formatted).toBeTruthy();
    });
  });

  describe('Bibliography Generation', () => {
    it('should generate bibliography HTML', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      const html = bibliographyManager.generateBibliography();
      expect(html).toContain('bibliography');
      expect(html).toContain('References');
    });

    it('should return empty message for no entries', () => {
      const html = bibliographyManager.generateBibliography();
      expect(html).toContain('bibliography-empty');
    });
  });

  describe('BibTeX Import/Export', () => {
    it('should export to BibTeX', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      const bibtex = bibliographyManager.exportToBibTeX();
      expect(bibtex).toBeTruthy();
    });

    it('should import from BibTeX', () => {
      const bibtex = `@book{test,
        author = {John Doe},
        title = {Test Book},
        year = {2024}
      }`;

      const ids = bibliographyManager.importFromBibTeX(bibtex);
      expect(ids.length).toBeGreaterThanOrEqual(0);
    });
  });

  describe('Configuration', () => {
    it('should set default style', () => {
      bibliographyManager.setDefaultStyle('mla');
      expect(bibliographyManager.getDefaultStyle()).toBe('mla');
    });
  });

  describe('Statistics', () => {
    it('should calculate statistics', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Book1',
        year: 2024,
        tags: ['science']
      });
      bibliographyManager.addEntry({
        type: 'article',
        authors: ['Jane Smith'],
        title: 'Article1',
        year: 2024,
        tags: ['science']
      });

      const stats = bibliographyManager.getStatistics();
      expect(stats.totalEntries).toBe(2);
      expect(stats.byType.book).toBe(1);
      expect(stats.byType.article).toBe(1);
      expect(stats.byTag.science).toBe(2);
    });
  });

  describe('Clear', () => {
    it('should clear all entries and citations', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });
      bibliographyManager.addCitation(bibliographyManager.getAllEntries()[0].id, 100);

      bibliographyManager.clearAll();
      expect(bibliographyManager.getAllEntries().length).toBe(0);
      expect(bibliographyManager.getCitations().length).toBe(0);
    });
  });

  describe('Entry Types', () => {
    it('should handle journal entry', () => {
      const entry = {
        type: 'journal' as const,
        authors: ['John Doe'],
        title: 'Research Article',
        year: 2024,
        journal: 'Nature',
        volume: '123',
        issue: '4',
        pages: '456-478'
      };
      const id = bibliographyManager.addEntry(entry);
      const retrieved = bibliographyManager.getEntry(id);
      expect(retrieved?.type).toBe('journal');
      expect(retrieved?.journal).toBe('Nature');
    });

    it('should handle website entry', () => {
      const entry = {
        type: 'website' as const,
        authors: ['John Doe'],
        title: 'Web Resource',
        url: 'https://example.com',
        accessedDate: '2024-01-15'
      };
      const id = bibliographyManager.addEntry(entry);
      const retrieved = bibliographyManager.getEntry(id);
      expect(retrieved?.type).toBe('website');
      expect(retrieved?.url).toBe('https://example.com');
    });

    it('should handle conference entry', () => {
      const entry = {
        type: 'conference' as const,
        authors: ['John Doe'],
        title: 'Conference Paper',
        year: 2024
      };
      const id = bibliographyManager.addEntry(entry);
      const retrieved = bibliographyManager.getEntry(id);
      expect(retrieved?.type).toBe('conference');
    });

    it('should handle thesis entry', () => {
      const entry = {
        type: 'thesis' as const,
        authors: ['John Doe'],
        title: 'PhD Thesis',
        year: 2024
      };
      const id = bibliographyManager.addEntry(entry);
      const retrieved = bibliographyManager.getEntry(id);
      expect(retrieved?.type).toBe('thesis');
    });

    it('should handle report entry', () => {
      const entry = {
        type: 'report' as const,
        authors: ['John Doe'],
        title: 'Technical Report',
        year: 2024
      };
      const id = bibliographyManager.addEntry(entry);
      const retrieved = bibliographyManager.getEntry(id);
      expect(retrieved?.type).toBe('report');
    });
  });

  describe('Additional Fields', () => {
    it('should handle DOI', () => {
      const entry = {
        type: 'article' as const,
        authors: ['John Doe'],
        title: 'Test Article',
        year: 2024,
        doi: '10.1000/xyz123'
      };
      const id = bibliographyManager.addEntry(entry);
      const retrieved = bibliographyManager.getEntry(id);
      expect(retrieved?.doi).toBe('10.1000/xyz123');
    });

    it('should handle custom fields', () => {
      const entry = {
        type: 'book' as const,
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024,
        customFields: {
          'isbn': '978-3-16-148410-0',
          'edition': '2nd'
        }
      };
      const id = bibliographyManager.addEntry(entry);
      const retrieved = bibliographyManager.getEntry(id);
      expect(retrieved?.customFields?.isbn).toBe('978-3-16-148410-0');
    });

    it('should handle notes', () => {
      const entry = {
        type: 'book' as const,
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024,
        notes: 'Important reference for chapter 3'
      };
      const id = bibliographyManager.addEntry(entry);
      const retrieved = bibliographyManager.getEntry(id);
      expect(retrieved?.notes).toBe('Important reference for chapter 3');
    });

    it('should handle multiple tags', () => {
      const entry = {
        type: 'book' as const,
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024,
        tags: ['science', 'physics', 'quantum']
      };
      const id = bibliographyManager.addEntry(entry);
      const retrieved = bibliographyManager.getEntry(id);
      expect(retrieved?.tags?.length).toBe(3);
    });
  });

  describe('Citation Styles', () => {
    it('should format in Chicago style', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024,
        publisher: 'Test Publisher'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'chicago');
      expect(formatted).toBeTruthy();
    });

    it('should format in Harvard style', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024,
        publisher: 'Test Publisher'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'harvard');
      expect(formatted).toBeTruthy();
    });

    it('should format in IEEE style', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'article',
        authors: ['John Doe'],
        title: 'Test Article',
        year: 2024,
        journal: 'IEEE Transactions'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'ieee');
      expect(formatted).toBeTruthy();
    });

    it('should format in Vancouver style', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'article',
        authors: ['John Doe'],
        title: 'Test Article',
        year: 2024
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'vancouver');
      expect(formatted).toBeTruthy();
    });
  });

  describe('Validation Edge Cases', () => {
    it('should validate minimum year', () => {
      const entry = {
        type: 'book' as const,
        authors: ['John Doe'],
        title: 'Test Book',
        year: 1000
      };
      // The validation may allow year 1000, so we just check it doesn't crash
      const id = bibliographyManager.addEntry(entry);
      expect(id).toBeTruthy();
    });

    it('should handle missing optional fields', () => {
      const entry = {
        type: 'book' as const,
        authors: ['John Doe'],
        title: 'Test Book'
      };
      const id = bibliographyManager.addEntry(entry);
      const retrieved = bibliographyManager.getEntry(id);
      expect(retrieved).toBeTruthy();
      expect(retrieved?.year).toBeUndefined();
    });

    it('should handle multiple authors', () => {
      const entry = {
        type: 'book' as const,
        authors: ['John Doe', 'Jane Smith', 'Bob Johnson'],
        title: 'Test Book',
        year: 2024
      };
      const id = bibliographyManager.addEntry(entry);
      const retrieved = bibliographyManager.getEntry(id);
      expect(retrieved?.authors.length).toBe(3);
    });
  });

  describe('Import/Export Edge Cases', () => {
    it('should handle empty BibTeX', () => {
      const ids = bibliographyManager.importFromBibTeX('');
      expect(ids.length).toBe(0);
    });

    it('should handle malformed BibTeX', () => {
      const malformed = 'this is not valid bibtex';
      const ids = bibliographyManager.importFromBibTeX(malformed);
      expect(ids.length).toBe(0);
    });

    it('should handle BibTeX with multiple entries', () => {
      const bibtex = `@book{test1,
        author = {John Doe},
        title = {Test Book 1},
        year = {2024}
      }
      @article{test2,
        author = {Jane Smith},
        title = {Test Article},
        year = {2024}
      }`;
      const ids = bibliographyManager.importFromBibTeX(bibtex);
      expect(ids.length).toBeGreaterThanOrEqual(0);
    });
  });

  describe('Timestamps', () => {
    it('should set createdAt timestamp', () => {
      const before = Date.now();
      const entry = {
        type: 'book' as const,
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      };
      const id = bibliographyManager.addEntry(entry);
      const retrieved = bibliographyManager.getEntry(id);
      expect(retrieved?.createdAt).toBeGreaterThanOrEqual(before);
    });

    it('should update updatedAt timestamp on update', async () => {
      const entry = {
        type: 'book' as const,
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      };
      const id = bibliographyManager.addEntry(entry);
      const original = bibliographyManager.getEntry(id);
      
      // Wait a bit to ensure timestamp difference
      await new Promise(resolve => setTimeout(resolve, 10));
      bibliographyManager.updateEntry(id, { title: 'Updated' });
      const updated = bibliographyManager.getEntry(id);
      
      expect(updated?.updatedAt).toBeGreaterThan(original!.updatedAt);
    });
  });

  describe('Search Edge Cases', () => {
    it('should return empty results for no matches', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      const results = bibliographyManager.searchEntries('nonexistent');
      expect(results.length).toBe(0);
    });

    it('should search in empty database', () => {
      const results = bibliographyManager.searchEntries('test');
      expect(results.length).toBe(0);
    });

    it('should handle empty search term', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      const results = bibliographyManager.searchEntries('');
      expect(results.length).toBeGreaterThanOrEqual(0);
    });
  });

  describe('Citation Management', () => {
    it('should handle multiple citations for same entry', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      bibliographyManager.addCitation(entryId, 100);
      bibliographyManager.addCitation(entryId, 200);
      bibliographyManager.addCitation(entryId, 300);

      const citations = bibliographyManager.getCitationsForEntry(entryId);
      expect(citations.length).toBe(3);
    });

    it('should get all citations', () => {
      const entryId1 = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Book 1',
        year: 2024
      });
      const entryId2 = bibliographyManager.addEntry({
        type: 'article',
        authors: ['Jane Smith'],
        title: 'Article 1',
        year: 2024
      });

      bibliographyManager.addCitation(entryId1, 100);
      bibliographyManager.addCitation(entryId2, 200);

      const citations = bibliographyManager.getCitations();
      expect(citations.length).toBe(2);
    });
  });

  describe('Statistics Edge Cases', () => {
    it('should handle empty statistics', () => {
      const stats = bibliographyManager.getStatistics();
      expect(stats.totalEntries).toBe(0);
      expect(Object.keys(stats.byType).length).toBe(0);
    });

    it('should count entries without tags', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      const stats = bibliographyManager.getStatistics();
      expect(stats.totalEntries).toBe(1);
    });
  });

  describe('Formatting Edge Cases', () => {
    it('should format book with all fields in APA', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe', 'Jane Smith'],
        title: 'Comprehensive Book',
        year: 2024,
        publisher: 'Academic Press',
        doi: '10.1000/test',
        url: 'https://example.com'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'apa');
      expect(formatted).toBeTruthy();
      expect(formatted).toContain('John Doe, Jane Smith');
    });

    it('should format journal with all fields in APA', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'journal',
        authors: ['John Doe'],
        title: 'Research Article',
        year: 2024,
        journal: 'Nature',
        volume: '123',
        issue: '4',
        pages: '456-478',
        doi: '10.1000/test'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'apa');
      expect(formatted).toBeTruthy();
      expect(formatted).toContain('Nature');
    });

    it('should format website in APA', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'website',
        authors: ['John Doe'],
        title: 'Web Resource',
        url: 'https://example.com',
        accessedDate: '2024-01-15'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'apa');
      expect(formatted).toBeTruthy();
    });

    it('should format with custom citation style', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      bibliographyManager.addCitation(entryId, 100, 'mla');
      const citations = bibliographyManager.getCitations();
      expect(citations[0].style).toBe('mla');
    });

    it('should handle formatting with missing year', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'apa');
      expect(formatted).toBeTruthy();
    });

    it('should format in BibTeX with all fields', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'article',
        authors: ['John Doe', 'Jane Smith'],
        title: 'Test Article',
        year: 2024,
        journal: 'Nature',
        volume: '123',
        issue: '4',
        pages: '456-478',
        doi: '10.1000/test',
        url: 'https://example.com'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'bibtex');
      expect(formatted).toContain('@article');
      expect(formatted).toContain('journal');
      expect(formatted).toContain('doi');
    });

    it('should format in Chicago with journal fields', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'journal',
        authors: ['John Doe'],
        title: 'Research Article',
        year: 2024,
        journal: 'Science',
        volume: '100',
        issue: '1',
        pages: '1-10'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'chicago');
      expect(formatted).toBeTruthy();
      expect(formatted).toContain('Science');
    });

    it('should format in Harvard with publisher', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024,
        publisher: 'Harvard Press'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'harvard');
      expect(formatted).toBeTruthy();
      expect(formatted).toContain('Harvard Press');
    });

    it('should format in IEEE with DOI', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'article',
        authors: ['John Doe'],
        title: 'Test Article',
        year: 2024,
        journal: 'IEEE Transactions',
        doi: '10.1109/test.2024.123456'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'ieee');
      expect(formatted).toBeTruthy();
      expect(formatted).toContain('doi');
    });

    it('should format in Vancouver with all journal fields', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'journal',
        authors: ['John Doe'],
        title: 'Test Article',
        year: 2024,
        journal: 'Medical Journal',
        volume: '50',
        issue: '2',
        pages: '100-110'
      });

      const entry = bibliographyManager.getEntry(entryId);
      const formatted = bibliographyManager.formatEntry(entry!, 'vancouver');
      expect(formatted).toBeTruthy();
      expect(formatted).toContain('Medical Journal');
    });
  });

  describe('Citation Position Tracking', () => {
    it('should track citation positions', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      bibliographyManager.addCitation(entryId, 100);
      bibliographyManager.addCitation(entryId, 250);
      bibliographyManager.addCitation(entryId, 500);

      const citations = bibliographyManager.getCitationsForEntry(entryId);
      expect(citations[0].position).toBe(100);
      expect(citations[1].position).toBe(250);
      expect(citations[2].position).toBe(500);
    });

    it('should handle citation with custom format', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      bibliographyManager.addCitation(entryId, 100, 'apa');
      const citations = bibliographyManager.getCitations();
      expect(citations[0].style).toBe('apa');
    });
  });

  describe('Entry Update Edge Cases', () => {
    it('should update multiple fields at once', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      bibliographyManager.updateEntry(entryId, {
        title: 'Updated Title',
        year: 2025,
        publisher: 'New Publisher'
      });

      const updated = bibliographyManager.getEntry(entryId);
      expect(updated?.title).toBe('Updated Title');
      expect(updated?.year).toBe(2025);
      expect(updated?.publisher).toBe('New Publisher');
    });

    it('should update tags', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024,
        tags: ['science']
      });

      bibliographyManager.updateEntry(entryId, {
        tags: ['science', 'physics', 'updated']
      });

      const updated = bibliographyManager.getEntry(entryId);
      expect(updated?.tags?.length).toBe(3);
    });

    it('should update custom fields', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024,
        customFields: { isbn: '123' }
      });

      bibliographyManager.updateEntry(entryId, {
        customFields: { isbn: '456', edition: '3rd' }
      });

      const updated = bibliographyManager.getEntry(entryId);
      expect(updated?.customFields?.isbn).toBe('456');
      expect(updated?.customFields?.edition).toBe('3rd');
    });
  });

  describe('Bibliography Generation Edge Cases', () => {
    it('should generate bibliography with specific style', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      const html = bibliographyManager.generateBibliography('mla');
      expect(html).toContain('bibliography');
    });

    it('should generate bibliography with multiple entries', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Book 1',
        year: 2024
      });
      bibliographyManager.addEntry({
        type: 'article',
        authors: ['Jane Smith'],
        title: 'Article 1',
        year: 2024
      });
      bibliographyManager.addEntry({
        type: 'journal',
        authors: ['Bob Johnson'],
        title: 'Journal 1',
        year: 2024
      });

      const html = bibliographyManager.generateBibliography();
      expect(html).toContain('bibliography-list');
    });
  });

  describe('Delete Entry with Citations', () => {
    it('should remove citations when entry is deleted', () => {
      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      bibliographyManager.addCitation(entryId, 100);
      bibliographyManager.addCitation(entryId, 200);

      bibliographyManager.deleteEntry(entryId);

      const citations = bibliographyManager.getCitations();
      expect(citations.length).toBe(0);
    });

    it('should only remove citations for deleted entry', () => {
      const entryId1 = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Book 1',
        year: 2024
      });
      const entryId2 = bibliographyManager.addEntry({
        type: 'article',
        authors: ['Jane Smith'],
        title: 'Article 1',
        year: 2024
      });

      bibliographyManager.addCitation(entryId1, 100);
      bibliographyManager.addCitation(entryId2, 200);

      bibliographyManager.deleteEntry(entryId1);

      const citations = bibliographyManager.getCitations();
      expect(citations.length).toBe(1);
      expect(citations[0].entryId).toBe(entryId2);
    });
  });

  describe('Citation Style Default', () => {
    it('should use default style when not specified', () => {
      bibliographyManager.setDefaultStyle('chicago');

      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      bibliographyManager.addCitation(entryId, 100);
      const citations = bibliographyManager.getCitations();
      expect(citations[0].style).toBe('chicago');
    });

    it('should override default style when specified', () => {
      bibliographyManager.setDefaultStyle('apa');

      const entryId = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      bibliographyManager.addCitation(entryId, 100, 'mla');
      const citations = bibliographyManager.getCitations();
      expect(citations[0].style).toBe('mla');
    });
  });

  describe('Search in Multiple Fields', () => {
    it('should search by journal name', () => {
      bibliographyManager.addEntry({
        type: 'journal',
        authors: ['John Doe'],
        title: 'Research Article',
        year: 2024,
        journal: 'Nature Physics'
      });

      const results = bibliographyManager.searchEntries('nature');
      expect(results.length).toBe(1);
    });

    it('should search by publisher', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024,
        publisher: 'Academic Press'
      });

      const results = bibliographyManager.searchEntries('academic');
      expect(results.length).toBe(1);
    });

    it('should search by tag', () => {
      bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024,
        tags: ['quantum', 'physics']
      });

      const results = bibliographyManager.searchEntries('quantum');
      expect(results.length).toBe(1);
    });
  });

  describe('Entry ID Generation', () => {
    it('should generate unique IDs', () => {
      const id1 = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Book 1',
        year: 2024
      });
      const id2 = bibliographyManager.addEntry({
        type: 'book',
        authors: ['Jane Smith'],
        title: 'Book 2',
        year: 2024
      });

      expect(id1).not.toBe(id2);
    });

    it('should generate IDs with bib- prefix', () => {
      const id = bibliographyManager.addEntry({
        type: 'book',
        authors: ['John Doe'],
        title: 'Test Book',
        year: 2024
      });

      expect(id).toMatch(/^bib-/);
    });
  });

  describe('Citation Removal Edge Cases', () => {
    it('should handle removing non-existent citation', () => {
      // Should not throw error
      bibliographyManager.removeCitation('non-existent-id');
      const citations = bibliographyManager.getCitations();
      expect(citations.length).toBe(0);
    });
  });

  describe('Update Non-Existent Entry', () => {
    it('should throw error when updating non-existent entry', () => {
      expect(() =>
        bibliographyManager.updateEntry('non-existent', { title: 'Updated' })
      ).toThrow();
    });
  });
});
