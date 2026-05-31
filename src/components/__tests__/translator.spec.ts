import { describe, it, expect } from 'vitest';
import { htmlToTypst } from '../../utils/translator';

describe('Translator Utility Tests', () => {
  describe('htmlToTypst', () => {
    it('should convert h1 to Typst heading', () => {
      const html = '<h1>Title</h1>';
      const result = htmlToTypst(html);
      expect(result).toContain('= Title');
    });

    it('should convert h2 to Typst heading', () => {
      const html = '<h2>Subtitle</h2>';
      const result = htmlToTypst(html);
      expect(result).toContain('== Subtitle');
    });

    it('should convert h3 to Typst heading', () => {
      const html = '<h3>Section</h3>';
      const result = htmlToTypst(html);
      expect(result).toContain('=== Section');
    });

    it('should convert bold text', () => {
      const html = '<strong>bold</strong>';
      const result = htmlToTypst(html);
      expect(result).toContain('*bold*');
    });

    it('should convert italic text', () => {
      const html = '<em>italic</em>';
      const result = htmlToTypst(html);
      expect(result).toContain('_italic_');
    });

    it('should convert code blocks', () => {
      const html = '<code>code</code>';
      const result = htmlToTypst(html);
      expect(result).toContain('`code`');
    });

    it('should convert paragraphs', () => {
      const html = '<p>Paragraph</p>';
      const result = htmlToTypst(html);
      expect(result).toContain('Paragraph');
    });

    it('should convert unordered lists', () => {
      const html = '<ul><li>Item 1</li><li>Item 2</li></ul>';
      const result = htmlToTypst(html);
      expect(result).toContain('- Item 1');
      expect(result).toContain('- Item 2');
    });

    it('should convert ordered lists', () => {
      const html = '<ol><li>First</li><li>Second</li></ol>';
      const result = htmlToTypst(html);
      expect(result).toContain('1. First');
      expect(result).toContain('2. Second');
    });

    it('should add Typst header with page settings', () => {
      const html = '<p>Content</p>';
      const result = htmlToTypst(html);
      expect(result).toContain('#set page');
      expect(result).toContain('#set text');
    });

    it('should handle empty input', () => {
      const html = '';
      const result = htmlToTypst(html);
      expect(result).toContain('#set page');
    });

    it('should handle complex HTML', () => {
      const html = '<h1>Title</h1><p>Paragraph with <strong>bold</strong> text</p>';
      const result = htmlToTypst(html);
      expect(result).toContain('= Title');
      expect(result).toContain('Paragraph');
      expect(result).toContain('*bold*');
    });
  });
});
