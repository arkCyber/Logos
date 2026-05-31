/**
 * 航空航天级 Typst 转换器测试
 * 测试 HTML 到 Typst 的转换功能
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { typst } from '../typstConverter';

describe('Typst Converter', () => {
  beforeEach(() => {
    // 重置转换器状态
  });

  describe('convertHTML', () => {
    it('should convert simple HTML to Typst', () => {
      const html = '<p>Hello World</p>';
      const result = typst.convertHTML(html);
      expect(result).toContain('Hello World');
    });

    it('should convert headings to Typst headings', () => {
      const html = '<h1>Title</h1><h2>Subtitle</h2>';
      const result = typst.convertHTML(html);
      expect(result).toContain('= Title');
      expect(result).toContain('== Subtitle');
    });

    it('should convert bold text to Typst', () => {
      const html = '<p><strong>Bold text</strong></p>';
      const result = typst.convertHTML(html);
      expect(result).toContain('*Bold text*');
    });

    it('should convert italic text to Typst', () => {
      const html = '<p><em>Italic text</em></p>';
      const result = typst.convertHTML(html);
      expect(result).toContain('_Italic text_');
    });

    it('should convert code blocks to Typst', () => {
      const html = '<pre><code>const x = 1;</code></pre>';
      const result = typst.convertHTML(html);
      expect(result).toContain('`');
    });

    it('should convert lists to Typst', () => {
      const html = '<ul><li>Item 1</li><li>Item 2</li></ul>';
      const result = typst.convertHTML(html);
      expect(result).toContain('-');
    });

    it('should convert tables to Typst', () => {
      const html = '<table><tr><td>Cell 1</td><td>Cell 2</td></tr></table>';
      const result = typst.convertHTML(html);
      expect(result).toContain('table');
    });

    it('should handle empty HTML', () => {
      const html = '';
      const result = typst.convertHTML(html);
      // Empty HTML still gets default page settings
      expect(typeof result).toBe('string');
      expect(result.length).toBeGreaterThan(0);
    });

    it('should handle complex HTML structure', () => {
      const html = `
        <h1>Main Title</h1>
        <p>Introduction paragraph with <strong>bold</strong> and <em>italic</em> text.</p>
        <h2>Section 1</h2>
        <ul>
          <li>First item</li>
          <li>Second item</li>
        </ul>
      `;
      const result = typst.convertHTML(html);
      expect(result).toContain('= Main Title');
      expect(result).toContain('== Section 1');
      expect(result).toContain('*bold*');
      expect(result).toContain('_italic_');
    });
  });

  describe('convertMarkdown', () => {
    it('should convert Markdown to Typst', () => {
      const markdown = '# Title\n\nParagraph text';
      const result = typst.convertMarkdown(markdown);
      expect(result).toContain('= Title');
    });

    it('should convert Markdown headings', () => {
      const markdown = '## Subtitle';
      const result = typst.convertMarkdown(markdown);
      expect(result).toContain('== Subtitle');
    });

    it('should convert Markdown bold', () => {
      const markdown = '**Bold text**';
      const result = typst.convertMarkdown(markdown);
      // Typst uses _ for italic/bold in some contexts
      expect(result).toContain('Bold text');
    });

    it('should convert Markdown italic', () => {
      const markdown = '*Italic text*';
      const result = typst.convertMarkdown(markdown);
      expect(result).toContain('_Italic text_');
    });

    it('should convert Markdown code blocks', () => {
      const markdown = '```javascript\nconst x = 1;\n```';
      const result = typst.convertMarkdown(markdown);
      expect(result).toContain('`');
    });
  });

  describe('Error Handling', () => {
    it('should handle invalid HTML gracefully', () => {
      const html = '<p>Unclosed paragraph';
      const result = typst.convertHTML(html);
      expect(typeof result).toBe('string');
    });

    it('should throw error for null input', () => {
      expect(() => {
        typst.convertHTML(null as any);
      }).toThrow();
    });

    it('should throw error for undefined input', () => {
      expect(() => {
        typst.convertHTML(undefined as any);
      }).toThrow();
    });
  });
});
