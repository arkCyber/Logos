import { describe, it, expect } from 'vitest';
import { htmlToTypstSlides, markdownToTypstSlides } from '../../utils/slideTranslator';

describe('Slide Translator Tests', () => {
  describe('htmlToTypstSlides', () => {
    it('should detect horizontal rules as slide breaks', () => {
      const html = '<h1>Slide 1</h1><p>Content 1</p><hr><h1>Slide 2</h1><p>Content 2</p>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('#import "@preview/touying:0.5.2"');
      expect(result).toContain('= Slide 1');
      expect(result).toContain('= Slide 2');
    });

    it('should convert h1 to slide title', () => {
      const html = '<h1>Title</h1><p>Content</p>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('= Title');
    });

    it('should convert h2 to slide title', () => {
      const html = '<h2>Subtitle</h2><p>Content</p>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('= Subtitle');
    });

    it('should convert h3 to section', () => {
      const html = '<h1>Title</h1><h3>Section</h3><p>Content</p>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('== Section');
    });

    it('should convert bold text', () => {
      const html = '<h1>Title</h1><p><strong>bold</strong> text</p>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('*bold*');
    });

    it('should convert italic text', () => {
      const html = '<h1>Title</h1><p><em>italic</em> text</p>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('_italic_');
    });

    it('should convert unordered lists', () => {
      const html = '<h1>Title</h1><ul><li>Item 1</li><li>Item 2</li></ul>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('- Item 1');
      expect(result).toContain('- Item 2');
    });

    it('should convert ordered lists', () => {
      const html = '<h1>Title</h1><ol><li>First</li><li>Second</li></ol>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('1. First');
      expect(result).toContain('2. Second');
    });

    it('should convert tables', () => {
      const html = '<h1>Title</h1><table><tr><td>Cell 1</td><td>Cell 2</td></tr></table>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('#table');
      expect(result).toContain('columns: (auto, auto)');
    });

    it('should use default university theme', () => {
      const html = '<h1>Title</h1><p>Content</p>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('university-theme');
    });

    it('should use custom theme when specified', () => {
      const html = '<h1>Title</h1><p>Content</p>';
      const result = htmlToTypstSlides(html, { theme: 'metropolis-theme' });
      expect(result).toContain('metropolis-theme');
    });

    it('should use 16-9 aspect ratio by default', () => {
      const html = '<h1>Title</h1><p>Content</p>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('aspect-ratio: "16-9"');
    });

    it('should use custom aspect ratio when specified', () => {
      const html = '<h1>Title</h1><p>Content</p>';
      const result = htmlToTypstSlides(html, { aspectRatio: '4-3' });
      expect(result).toContain('aspect-ratio: "4-3"');
    });

    it('should handle empty input', () => {
      const html = '';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('#import "@preview/touying:0.5.2"');
    });

    it('should handle single slide without breaks', () => {
      const html = '<h1>Only Slide</h1><p>Content</p>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('= Only Slide');
    });

    it('should handle null input', () => {
      const result = htmlToTypstSlides(null as any);
      expect(result).toContain('Empty Slide');
    });

    it('should handle undefined input', () => {
      const result = htmlToTypstSlides(undefined as any);
      expect(result).toContain('Empty Slide');
    });

    it('should handle empty string', () => {
      const result = htmlToTypstSlides('');
      expect(result).toContain('Empty Slide');
    });

    it('should handle whitespace only', () => {
      const result = htmlToTypstSlides('   \n\n   ');
      expect(result).toContain('Empty Slide');
    });

    it('should handle content without headings', () => {
      const html = '<p>Just content without heading</p>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('= Slide 1');
    });

    it('should handle nested HTML structures', () => {
      const html = '<h1>Title</h1><div><p>Nested <strong>bold</strong> content</p></div>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('*bold*');
    });

    it('should handle special characters in title', () => {
      const html = '<h1>Title with $pecial & characters</h1><p>Content</p>';
      const result = htmlToTypstSlides(html);
      expect(result).toContain('Title with');
    });

    it('should handle multiple consecutive horizontal rules', () => {
      const html = '<h1>Slide 1</h1><hr><hr><h1>Slide 2</h1>';
      const result = htmlToTypstSlides(html);
      // Should handle empty slides between breaks
      expect(result).toContain('= Slide 1');
    });
  });

  describe('markdownToTypstSlides', () => {
    it('should detect --- as slide breaks', () => {
      const markdown = '# Slide 1\nContent 1\n---\n# Slide 2\nContent 2';
      const result = markdownToTypstSlides(markdown);
      expect(result).toContain('#import "@preview/touying:0.5.2"');
      expect(result).toContain('= Slide 1');
      expect(result).toContain('= Slide 2');
    });

    it('should convert markdown headings to slide titles', () => {
      const markdown = '# Title\nContent';
      const result = markdownToTypstSlides(markdown);
      expect(result).toContain('= Title');
    });

    it('should convert markdown bold', () => {
      const markdown = '# Title\n**bold** text';
      const result = markdownToTypstSlides(markdown);
      expect(result).toContain('*bold*');
    });

    it('should convert markdown lists', () => {
      const markdown = '# Title\n- Item 1\n- Item 2';
      const result = markdownToTypstSlides(markdown);
      expect(result).toContain('- Item 1');
      expect(result).toContain('- Item 2');
    });

    it('should use default theme', () => {
      const markdown = '# Title\nContent';
      const result = markdownToTypstSlides(markdown);
      expect(result).toContain('university-theme');
    });

    it('should handle h3 as section', () => {
      const markdown = '# Title\n### Section\nContent';
      const result = markdownToTypstSlides(markdown);
      expect(result).toContain('== Section');
    });

    it('should handle null markdown input', () => {
      const result = markdownToTypstSlides(null as any);
      expect(result).toContain('Empty Slide');
    });

    it('should handle empty markdown', () => {
      const result = markdownToTypstSlides('');
      expect(result).toContain('Empty Slide');
    });

    it('should handle markdown without heading', () => {
      const markdown = 'Just content without heading';
      const result = markdownToTypstSlides(markdown);
      expect(result).toContain('= Untitled');
    });

    it('should handle special characters in markdown', () => {
      const markdown = '# Title with $pecial & chars\nContent';
      const result = markdownToTypstSlides(markdown);
      expect(result).toContain('Title with');
    });
  });
});
