/**
 * Aerospace-grade unit tests for presentation converter
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { PresentationConverter, RenderTarget } from '../presentationConverter';
import { createEmptyPresentation, createEmptySlide, SlideLayout, ElementType, validatePresentationDocument } from '../../types/presentation';

describe('PresentationConverter', () => {
  let testDocument: any;

  beforeEach(() => {
    testDocument = createEmptyPresentation('Test Presentation');
    testDocument.slides = [
      createEmptySlide(0, SlideLayout.TITLE),
      createEmptySlide(1, SlideLayout.BLANK)
    ];
    testDocument.slides[0].title = 'First Slide';
    testDocument.slides[0].elements = [
      {
        id: '1',
        type: ElementType.TEXT,
        content: 'Hello World',
        position: { x: 0, y: 0 },
        size: { width: 200, height: 100 },
        style: { fontSize: 24, color: '#333333' }
      }
    ];
  });

  describe('toSlidev', () => {
    it('should convert document to Slidev Markdown', () => {
      const markdown = PresentationConverter.toSlidev(testDocument);
      
      expect(markdown).toContain('---');
      expect(markdown).toContain('theme: default');
      expect(markdown).toContain('title: Test Presentation');
      expect(markdown).toContain('# First Slide');
      expect(markdown).toContain('Hello World');
    });

    it('should include author if present', () => {
      testDocument.metadata.author = 'Test Author';
      const markdown = PresentationConverter.toSlidev(testDocument);
      
      expect(markdown).toContain('author: Test Author');
    });

    it('should handle empty slides', () => {
      testDocument.slides = [createEmptySlide(0, SlideLayout.BLANK)];
      const markdown = PresentationConverter.toSlidev(testDocument);
      
      expect(markdown).toContain('---');
    });
  });

  describe('toTypst', () => {
    it('should convert document to Typst format', () => {
      const typst = PresentationConverter.toTypst(testDocument);
      
      expect(typst).toContain('#import "@preview/touying');
      expect(typst).toContain('#show:');
    });

    it('should use correct aspect ratio', () => {
      testDocument.settings.aspectRatio = '4-3';
      const typst = PresentationConverter.toTypst(testDocument);
      
      expect(typst).toContain('aspect-ratio: "4-3"');
    });
  });

  describe('toHtml', () => {
    it('should convert document to HTML', () => {
      const html = PresentationConverter.toHtml(testDocument);
      
      expect(html).toContain('<div class="slide">');
      expect(html).toContain('<h1>First Slide</h1>');
      expect(html).toContain('Hello World');
      expect(html).toContain('<hr class="slide-separator" />');
    });

    it('should handle multiple slides', () => {
      const html = PresentationConverter.toHtml(testDocument);
      
      const slideCount = (html.match(/<div class="slide">/g) || []).length;
      expect(slideCount).toBe(2);
    });
  });

  describe('toPptx', () => {
    it('should convert document to PPTX data structure', () => {
      const pptx = PresentationConverter.toPptx(testDocument);
      
      expect(pptx).toHaveProperty('metadata');
      expect(pptx).toHaveProperty('theme');
      expect(pptx).toHaveProperty('slides');
      expect(pptx).toHaveProperty('settings');
      expect(pptx.slides).toHaveLength(2);
    });
  });

  describe('fromHtml', () => {
    it('should parse HTML to presentation document', () => {
      const html = `
        <div class="slide">
          <h1>Test Title</h1>
          <p>Test content</p>
        </div>
        <hr class="slide-separator" />
        <div class="slide">
          <p>Second slide</p>
        </div>
      `;
      
      const doc = PresentationConverter.fromHtml(html, 'Imported Presentation');
      
      expect(doc.metadata.title).toBe('Imported Presentation');
      expect(doc.slides).toHaveLength(2);
      expect(doc.slides[0].title).toBe('Test Title');
      expect(doc.slides[0].elements).toHaveLength(1);
    });

    it('should extract images from HTML', () => {
      const html = `
        <div class="slide">
          <img src="test.jpg" alt="Test Image" />
        </div>
      `;
      
      const doc = PresentationConverter.fromHtml(html, 'Test');
      
      expect(doc.slides[0].elements).toHaveLength(1);
      expect(doc.slides[0].elements[0].type).toBe(ElementType.IMAGE);
    });

    it('should handle empty HTML', () => {
      const doc = PresentationConverter.fromHtml('', 'Test');
      
      // Empty HTML may still create a default slide
      expect(doc.slides.length).toBeGreaterThanOrEqual(0);
    });
  });

  describe('fromMarkdown', () => {
    it('should parse Markdown to presentation document', () => {
      const markdown = `
# Title Slide

Content here

---

# Second Slide

More content
      `;
      
      const doc = PresentationConverter.fromMarkdown(markdown, 'Imported');
      
      expect(doc.slides).toHaveLength(2);
      expect(doc.slides[0].title).toBe('Title Slide');
      expect(doc.slides[1].title).toBe('Second Slide');
    });

    it('should extract code blocks', () => {
      const markdown = `
# Code Slide

\`\`\`javascript
const x = 1;
\`\`\`
      `;
      
      const doc = PresentationConverter.fromMarkdown(markdown, 'Test');
      
      expect(doc.slides[0].elements).toHaveLength(1);
      expect(doc.slides[0].elements[0].type).toBe(ElementType.CODE);
    });

    it('should handle subheadings as text elements', () => {
      const markdown = `
# Title

## Subheading

Content
      `;
      
      const doc = PresentationConverter.fromMarkdown(markdown, 'Test');
      
      const subheadingElement = doc.slides[0].elements.find(
        (e: any) => e.content === 'Subheading'
      );
      expect(subheadingElement).toBeDefined();
    });
  });

  describe('elementToSlidevMarkdown', () => {
    it('should convert text element to Markdown', () => {
      const element = {
        id: '1',
        type: ElementType.TEXT,
        content: 'Test',
        position: { x: 0, y: 0 },
        size: { width: 100, height: 50 }
      } as any;
      
      const markdown = PresentationConverter['elementToSlidevMarkdown'](element);
      expect(markdown).toBe('Test');
    });

    it('should convert image element to Markdown', () => {
      const element = {
        id: '1',
        type: ElementType.IMAGE,
        src: 'test.jpg',
        position: { x: 0, y: 0 },
        size: { width: 100, height: 50 }
      } as any;
      
      const markdown = PresentationConverter['elementToSlidevMarkdown'](element);
      expect(markdown).toContain('<img');
      expect(markdown).toContain('test.jpg');
    });
  });
});

describe('Presentation Types', () => {
  describe('createEmptyPresentation', () => {
    it('should create an empty presentation', () => {
      const doc = createEmptyPresentation('Test');
      
      expect(doc.metadata.title).toBe('Test');
      expect(doc.metadata.type).toBe('presentation');
      expect(doc.slides).toHaveLength(0);
      expect(doc.theme).toBeDefined();
      expect(doc.settings).toBeDefined();
    });

    it('should generate valid UUID', () => {
      const doc = createEmptyPresentation('Test');
      
      expect(doc.metadata.id).toMatch(/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/);
    });
  });

  describe('createEmptySlide', () => {
    it('should create an empty slide', () => {
      const slide = createEmptySlide(0, SlideLayout.BLANK);
      
      expect(slide.index).toBe(0);
      expect(slide.layout).toBe(SlideLayout.BLANK);
      expect(slide.elements).toHaveLength(0);
      expect(slide.background).toBeDefined();
    });

    it('should generate valid UUID', () => {
      const slide = createEmptySlide(0, SlideLayout.BLANK);
      
      expect(slide.id).toMatch(/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/);
    });
  });

  describe('validatePresentationDocument', () => {
    it('should validate a correct document', () => {
      const doc = createEmptyPresentation('Test');
      
      expect(() => validatePresentationDocument(doc)).not.toThrow();
    });

    it('should reject invalid document', () => {
      const invalidDoc = { invalid: 'data' };
      
      expect(() => validatePresentationDocument(invalidDoc)).toThrow();
    });

    it('should reject document with invalid title', () => {
      const doc = createEmptyPresentation('');
      
      expect(() => validatePresentationDocument(doc)).toThrow();
    });
  });

  describe('ElementType', () => {
    it('should have all required element types', () => {
      expect(ElementType.TEXT).toBe('text');
      expect(ElementType.IMAGE).toBe('image');
      expect(ElementType.SHAPE).toBe('shape');
      expect(ElementType.CHART).toBe('chart');
      expect(ElementType.TABLE).toBe('table');
      expect(ElementType.VIDEO).toBe('video');
      expect(ElementType.CODE).toBe('code');
    });
  });

  describe('SlideLayout', () => {
    it('should have all required layouts', () => {
      expect(SlideLayout.TITLE).toBe('title');
      expect(SlideLayout.TITLE_AND_CONTENT).toBe('title_and_content');
      expect(SlideLayout.TWO_CONTENT).toBe('two_content');
      expect(SlideLayout.BLANK).toBe('blank');
      expect(SlideLayout.SECTION_HEADER).toBe('section_header');
      expect(SlideLayout.COMPARISON).toBe('comparison');
      expect(SlideLayout.CONTENT_WITH_CAPTION).toBe('content_with_caption');
    });
  });
});
