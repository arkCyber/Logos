import { describe, it, expect } from 'vitest';
import { htmlToTypstSlides, markdownToTypstSlides } from '../slideTranslator';

describe('htmlToTypstSlides', () => {
  it('should convert HTML to Typst slides', () => {
    const html = '<h1>Slide 1</h1><p>Content</p>';
    const result = htmlToTypstSlides(html);

    expect(result).toContain('#import "@preview/touying:0.5.2"');
    expect(result).toContain('Slide 1');
  });

  it('should handle custom theme', () => {
    const html = '<h1>Title</h1>';
    const result = htmlToTypstSlides(html, { theme: 'simple' });

    expect(result).toContain('simple');
  });

  it('should handle custom aspect ratio', () => {
    const html = '<h1>Title</h1>';
    const result = htmlToTypstSlides(html, { aspectRatio: '4-3' });

    expect(result).toContain('4-3');
  });

  it('should handle empty HTML', () => {
    const result = htmlToTypstSlides('');

    expect(result).toContain('#import "@preview/touying');
  });

  it('should split content into slides', () => {
    const html = '<h1>Slide 1</h1><hr><h1>Slide 2</h1>';
    const result = htmlToTypstSlides(html);

    expect(result).toContain('Slide 1');
    expect(result).toContain('Slide 2');
  });

  it('should handle invalid input', () => {
    const result = htmlToTypstSlides(null as any);

    expect(result).toBeDefined();
    expect(result).toContain('#import "@preview/touying');
  });
});

describe('markdownToTypstSlides', () => {
  it('should convert markdown to Typst slides', () => {
    const markdown = '# Slide 1\n\nContent\n\n---\n\n# Slide 2';
    const result = markdownToTypstSlides(markdown);

    expect(result).toContain('Slide 1');
    expect(result).toContain('Slide 2');
  });

  it('should handle empty markdown', () => {
    const result = markdownToTypstSlides('');

    expect(result).toContain('#import "@preview/touying');
  });

  it('should handle custom config', () => {
    const markdown = '# Title';
    const result = markdownToTypstSlides(markdown, {
      theme: 'metropolis',
      aspectRatio: '16-9'
    });

    expect(result).toContain('metropolis');
  });

  it('should handle invalid input', () => {
    const result = markdownToTypstSlides(null as any);

    expect(result).toBeDefined();
  });
});
