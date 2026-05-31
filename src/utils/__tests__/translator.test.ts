import { describe, it, expect } from 'vitest';
import { htmlToTypst } from '../translator';

describe('htmlToTypst', () => {
  it('should convert basic HTML to Typst', () => {
    const html = '<p>Hello World</p>';
    const result = htmlToTypst(html);

    expect(result).toContain('Hello World');
    expect(result).toContain('#set page');
  });

  it('should convert headings', () => {
    const html = '<h1>Title</h1><h2>Subtitle</h2>';
    const result = htmlToTypst(html);

    expect(result).toContain('= Title');
    expect(result).toContain('== Subtitle');
  });

  it('should convert bold text', () => {
    const html = '<p><strong>Bold text</strong></p>';
    const result = htmlToTypst(html);

    expect(result).toContain('*Bold text*');
  });

  it('should convert italic text', () => {
    const html = '<p><em>Italic text</em></p>';
    const result = htmlToTypst(html);

    expect(result).toContain('_Italic text_');
  });

  it('should convert lists', () => {
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

  it('should convert links', () => {
    const html = '<a href="https://example.com">Link</a>';
    const result = htmlToTypst(html);

    expect(result).toContain('Link');
  });

  it('should handle empty HTML', () => {
    const result = htmlToTypst('');

    expect(result).toContain('#set page');
  });

  it('should handle complex nested HTML', () => {
    const html = `
      <h1>Document Title</h1>
      <p>This is a <strong>bold</strong> and <em>italic</em> text.</p>
      <ul>
        <li>Item 1</li>
        <li>Item 2</li>
      </ul>
    `;
    const result = htmlToTypst(html);

    expect(result).toContain('= Document Title');
    expect(result).toContain('*bold*');
    expect(result).toContain('_italic_');
    expect(result).toContain('Item 1');
  });

  it('should preserve line breaks', () => {
    const html = '<p>Line 1<br>Line 2</p>';
    const result = htmlToTypst(html);

    expect(result).toContain('Line 1');
    expect(result).toContain('Line 2');
  });

  it('should handle special characters', () => {
    const html = '<p>&lt;div&gt; &amp; &quot;test&quot;</p>';
    const result = htmlToTypst(html);

    expect(result).toBeDefined();
  });

  it('should convert strikethrough text', () => {
    const html = '<p><s>Strikethrough</s></p>';
    const result = htmlToTypst(html);

    expect(result).toContain('#strike(Strikethrough)');
  });

  it('should convert code inline', () => {
    const html = '<p>Some <code>code</code> here</p>';
    const result = htmlToTypst(html);

    expect(result).toContain('`code`');
  });

  it('should convert code blocks', () => {
    const html = '<pre>const x = 1;</pre>';
    const result = htmlToTypst(html);

    expect(result).toContain('```');
    expect(result).toContain('const x = 1;');
  });

  it('should convert blockquotes', () => {
    const html = '<blockquote>Quote text</blockquote>';
    const result = htmlToTypst(html);

    expect(result).toContain('#block');
    expect(result).toContain('Quote text');
  });

  it('should convert horizontal rules', () => {
    const html = '<hr />';
    const result = htmlToTypst(html);

    expect(result).toContain('#line');
  });

  it('should convert tables', () => {
    const html = '<table><tr><td>Cell 1</td><td>Cell 2</td></tr></table>';
    const result = htmlToTypst(html);

    expect(result).toContain('#table');
    expect(result).toContain('columns:');
    expect(result).toContain('Cell 1');
    expect(result).toContain('Cell 2');
  });

  it('should convert h3 headings', () => {
    const html = '<h3>Subsection</h3>';
    const result = htmlToTypst(html);

    expect(result).toContain('=== Subsection');
  });

  it('should handle multiple consecutive line breaks', () => {
    const html = '<p>Line 1<br><br>Line 2</p>';
    const result = htmlToTypst(html);

    expect(result).toContain('Line 1');
    expect(result).toContain('Line 2');
  });

  it('should clean up excessive whitespace', () => {
    const html = '<p>Text</p><p>More text</p><p>Even more</p>';
    const result = htmlToTypst(html);

    // Should not have excessive newlines
    expect(result).not.toMatch(/\n{4,}/);
  });

  it('should handle underline', () => {
    const html = '<p><u>Underlined</u></p>';
    const result = htmlToTypst(html);

    expect(result).toContain('Underlined');
  });

  it('should handle mixed formatting', () => {
    const html = '<p><strong>Bold</strong> and <em>italic</em> and <s>strikethrough</s></p>';
    const result = htmlToTypst(html);

    expect(result).toContain('*Bold*');
    expect(result).toContain('_italic_');
    expect(result).toContain('#strike(strikethrough)');
  });

  it('should handle empty tables', () => {
    const html = '<table></table>';
    const result = htmlToTypst(html);

    expect(result).toBeDefined();
  });

  it('should handle tables with headers', () => {
    const html = '<table><tr><th>Header 1</th><th>Header 2</th></tr><tr><td>Data 1</td><td>Data 2</td></tr></table>';
    const result = htmlToTypst(html);

    expect(result).toContain('#table');
    expect(result).toContain('Header 1');
    expect(result).toContain('Header 2');
    expect(result).toContain('Data 1');
    expect(result).toContain('Data 2');
  });

  it('should handle nested lists', () => {
    const html = '<ul><li>Item 1</li><li>Item 2<ul><li>Subitem 1</li></ul></li></ul>';
    const result = htmlToTypst(html);

    expect(result).toContain('- Item 1');
    expect(result).toContain('- Item 2');
    // Note: translator doesn't handle nested lists, subitems are flattened
    expect(result).toContain('Subitem 1');
  });
});
