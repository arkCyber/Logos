/**
 * 航空航天级 Typst 语法高亮器测试
 * 测试 Typst 代码语法高亮功能
 */

import { describe, it, expect } from 'vitest';
import { typstHighlighter, highlightTypst, highlightTypstToHTML } from '../typstHighlighter';

describe('Typst Highlighter', () => {
  describe('highlight', () => {
    it('should highlight Typst keywords', () => {
      const code = '#set page(paper: "a4")';
      const result = typstHighlighter.highlight(code);
      expect(typeof result).toBe('string');
    });

    it('should highlight Typst functions', () => {
      const code = '#rect(width: 100pt, height: 50pt)';
      const result = typstHighlighter.highlight(code);
      expect(typeof result).toBe('string');
    });

    it('should highlight Typst operators', () => {
      const code = 'let x = 1 + 2';
      const result = typstHighlighter.highlight(code);
      expect(typeof result).toBe('string');
    });

    it('should highlight Typst strings', () => {
      const code = '#text("Hello World")';
      const result = typstHighlighter.highlight(code);
      expect(typeof result).toBe('string');
    });

    it('should highlight Typst numbers', () => {
      const code = '#set text(size: 12pt)';
      const result = typstHighlighter.highlight(code);
      expect(typeof result).toBe('string');
    });

    it('should handle empty code', () => {
      const code = '';
      const result = typstHighlighter.highlight(code);
      expect(result).toBe('');
    });

    it('should handle complex Typst code', () => {
      const code = `
#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))
#set text(font: "SimSun", size: 11pt)

= 标题

== 子标题

*粗体文本*
_斜体文本_
\`代码\`

#table(
  columns: 2,
  [列1], [列2],
  [数据1], [数据2]
)
`;
      const result = typstHighlighter.highlight(code);
      expect(typeof result).toBe('string');
      expect(result.length).toBeGreaterThan(0);
    });
  });

  describe('highlightToHTML', () => {
    it('should convert Typst to highlighted HTML', () => {
      const code = '#set page(paper: "a4")';
      const result = typstHighlighter.highlightToHTML(code);
      expect(typeof result).toBe('string');
      expect(result).toContain('<');
    });

    it('should escape HTML in code', () => {
      const code = '#text("<script>alert(1)</script>")';
      const result = typstHighlighter.highlightToHTML(code);
      expect(result).not.toContain('<script>');
      expect(result).toContain('&lt;');
    });

    it('should handle errors gracefully', () => {
      const code = null as any;
      const result = typstHighlighter.highlightToHTML(code);
      expect(typeof result).toBe('string');
      expect(result).toContain('<pre><code>');
    });
  });

  describe('Convenience Functions', () => {
    it('highlightTypst should work as alias', () => {
      const code = '#set page(paper: "a4")';
      const result = highlightTypst(code);
      expect(typeof result).toBe('string');
    });

    it('highlightTypstToHTML should work as alias', () => {
      const code = '#set page(paper: "a4")';
      const result = highlightTypstToHTML(code);
      expect(typeof result).toBe('string');
    });
  });

  describe('getLanguageDefinition', () => {
    it('should return language definition', () => {
      const definition = typstHighlighter.getLanguageDefinition();
      expect(definition).toBeDefined();
      expect(definition.keywords).toBeDefined();
      expect(definition.functions).toBeDefined();
      expect(definition.operators).toBeDefined();
      expect(definition.symbols).toBeDefined();
    });

    it('should contain expected keywords', () => {
      const definition = typstHighlighter.getLanguageDefinition();
      expect(definition.keywords).toContain('set');
      expect(definition.keywords).toContain('show');
      expect(definition.keywords).toContain('let');
    });

    it('should contain expected functions', () => {
      const definition = typstHighlighter.getLanguageDefinition();
      expect(definition.functions).toContain('text');
      expect(definition.functions).toContain('rect');
      expect(definition.functions).toContain('table');
    });
  });
});
