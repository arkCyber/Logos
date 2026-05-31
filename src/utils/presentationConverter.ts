/**
 * Aerospace-grade presentation format converter
 * Converts between different presentation formats (Slidev, Typst, PPTX, etc.)
 * using the unified data platform format
 */

import type {
  PresentationDocument,
  Slide,
  SlideElement,
  TextElement,
  ImageElement
} from '../types/presentation';
import { SlideLayout, ElementType } from '../types/presentation';
import { htmlToTypstSlides } from './slideTranslator';

export enum RenderTarget {
  SLIDEV = 'slidev',
  TYPST = 'typst',
  PPTX = 'pptx',
  REVEAL_JS = 'reveal_js',
}

export class PresentationConverter {
  /**
   * Convert unified document to Slidev Markdown format
   */
  static toSlidev(document: PresentationDocument): string {
    const lines: string[] = [];

    // Add frontmatter
    lines.push('---');
    lines.push(`theme: ${document.theme.name}`);
    lines.push(`aspectRatio: ${document.settings.aspectRatio.replace('-', ':')}`);
    lines.push(`title: ${document.metadata.title}`);
    if (document.metadata.author) {
      lines.push(`author: ${document.metadata.author}`);
    }
    lines.push('---');
    lines.push('');

    // Convert each slide
    document.slides.forEach((slide) => {
      lines.push('---');
      lines.push('');

      // Add slide title
      if (slide.title) {
        lines.push(`# ${slide.title}`);
        lines.push('');
      }

      // Convert elements
      slide.elements.forEach((element) => {
        lines.push(this.elementToSlidevMarkdown(element));
        lines.push('');
      });
    });

    return lines.join('\n');
  }

  /**
   * Convert unified document to Typst format
   */
  static toTypst(document: PresentationDocument): string {
    // Convert to HTML first, then use existing slideTranslator
    const html = this.toHtml(document);
    return htmlToTypstSlides(html, {
      theme: 'university-theme',
      aspectRatio: document.settings.aspectRatio as '16-9' | '4-3',
      showSlideNumbers: document.settings.showSlideNumbers
    });
  }

  /**
   * Convert unified document to HTML format
   */
  static toHtml(document: PresentationDocument): string {
    const slides: string[] = [];

    document.slides.forEach((slide) => {
      let slideHtml = '<div class="slide">\n';

      // Add title
      if (slide.title) {
        slideHtml += `  <h1>${this.escapeHtml(slide.title)}</h1>\n`;
      }

      // Add elements
      slide.elements.forEach((element) => {
        slideHtml += `  ${this.elementToHtml(element)}\n`;
      });

      slideHtml += '</div>';
      slides.push(slideHtml);
    });

    return slides.join('<hr class="slide-separator" />\n');
  }

  /**
   * Convert unified document to PPTX data structure
   */
  static toPptx(document: PresentationDocument): any {
    // This would interface with the Rust ppt_service
    // For now, return a simplified structure
    return {
      metadata: document.metadata,
      theme: document.theme,
      slides: document.slides.map((slide) => ({
        id: slide.id,
        index: slide.index,
        layout: slide.layout,
        title: slide.title,
        elements: slide.elements,
        notes: slide.notes,
        transition: slide.transition
      })),
      settings: document.settings
    };
  }

  /**
   * Convert element to Slidev Markdown
   */
  private static elementToSlidevMarkdown(element: SlideElement): string {
    switch (element.type) {
      case 'text':
        return this.textElementToSlidev(element as TextElement);
      case 'image':
        return this.imageElementToSlidev(element as ImageElement);
      case 'code':
        return this.codeElementToSlidev(element);
      default:
        return `<!-- Unsupported element type: ${element.type} -->`;
    }
  }

  /**
   * Convert text element to Slidev Markdown
   */
  private static textElementToSlidev(element: TextElement): string {
    const { content, style } = element;
    let markdown = content;

    // Apply basic formatting
    if (style?.fontWeight === 'bold') {
      markdown = `**${markdown}**`;
    }
    if (style?.fontStyle === 'italic') {
      markdown = `_${markdown}_`;
    }

    return markdown;
  }

  /**
   * Convert image element to Slidev Markdown
   */
  private static imageElementToSlidev(element: ImageElement): string {
    return `<img src="${element.src}" alt="${element.alt || ''}" />`;
  }

  /**
   * Convert code element to Slidev Markdown
   */
  private static codeElementToSlidev(element: any): string {
    const language = element.language || 'text';
    return `\`\`\`${language}\n${element.content}\n\`\`\``;
  }

  /**
   * Convert element to HTML
   */
  private static elementToHtml(element: SlideElement): string {
    switch (element.type) {
      case 'text':
        return this.textElementToHtml(element as TextElement);
      case 'image':
        return this.imageElementToHtml(element as ImageElement);
      case 'shape':
        return this.shapeElementToHtml(element);
      case 'table':
        return this.tableElementToHtml(element);
      default:
        return `<!-- Unsupported element type: ${element.type} -->`;
    }
  }

  /**
   * Convert text element to HTML
   */
  private static textElementToHtml(element: TextElement): string {
    const { content, style } = element;
    let html = '<p style="';

    const styles: string[] = [];
    if (style?.fontSize) {
styles.push(`font-size: ${style.fontSize}px`);
}
    if (style?.color) {
styles.push(`color: ${style.color}`);
}
    if (style?.textAlign) {
styles.push(`text-align: ${style.textAlign}`);
}
    if (style?.fontFamily) {
styles.push(`font-family: ${style.fontFamily}`);
}

    html += styles.join('; ') + '">';
    html += this.escapeHtml(content);
    html += '</p>';

    return html;
  }

  /**
   * Convert image element to HTML
   */
  private static imageElementToHtml(element: ImageElement): string {
    return `<img src="${element.src}" alt="${element.alt || ''}" style="width: ${element.size.width}px; height: ${element.size.height}px;" />`;
  }

  /**
   * Convert shape element to HTML
   */
  private static shapeElementToHtml(element: any): string {
    const { shape, style, size } = element;
    let html = `<div style="width: ${size.width}px; height: ${size.height}px; background-color: ${style?.fillColor || '#000'};`;

    if (shape === 'circle') {
      html += 'border-radius: 50%;';
    } else if (style?.borderRadius) {
      html += `border-radius: ${style.borderRadius}px;`;
    }

    html += '"></div>';
    return html;
  }

  /**
   * Convert table element to HTML
   */
  private static tableElementToHtml(element: any): string {
    const { data, rows, columns } = element;
    let html = '<table>';

    for (let i = 0; i < rows; i++) {
      html += '<tr>';
      for (let j = 0; j < columns; j++) {
        const cellValue = data[i]?.[j] || '';
        html += `<td>${this.escapeHtml(cellValue)}</td>`;
      }
      html += '</tr>';
    }

    html += '</table>';
    return html;
  }

  /**
   * Escape HTML special characters
   */
  private static escapeHtml(text: string): string {
    const map: Record<string, string> = {
      '&': '&amp;',
      '<': '&lt;',
      '>': '&gt;',
      '"': '&quot;',
      "'": '&#039;'
    };
    return text.replace(/[&<>"']/g, (m) => map[m]);
  }

  /**
   * Convert HTML to unified document
   */
  static fromHtml(html: string, title: string = 'Untitled'): PresentationDocument {
    // Parse HTML and convert to unified format
    // This is a simplified implementation
    const slides: Slide[] = [];
    const slideContents = html.split(/<hr\s*class=["']slide-separator["']\s*\/?>/i);

    slideContents.forEach((content, index) => {
      const slide: Slide = {
        id: crypto.randomUUID(),
        index,
        layout: SlideLayout.BLANK,
        elements: [],
        background: {
          type: 'color',
          value: '#ffffff'
        }
      };

      // Extract title
      const titleMatch = content.match(/<h1>(.*?)<\/h1>/i);
      if (titleMatch) {
        slide.title = titleMatch[1];
      }

      // Extract paragraphs as text elements
      const paragraphMatches = content.matchAll(/<p[^>]*>(.*?)<\/p>/gi);
      for (const match of paragraphMatches) {
        const textContent = match[1].replace(/<[^>]*>/g, ''); // Remove nested HTML
        if (textContent.trim()) {
          slide.elements.push({
            id: crypto.randomUUID(),
            type: ElementType.TEXT,
            content: textContent,
            position: { x: 0, y: 0 },
            size: { width: 800, height: 100 },
            style: {
              fontSize: 24,
              color: '#333333'
            }
          });
        }
      }

      // Extract images
      const imageMatches = content.matchAll(/<img[^>]*src=["']([^"']*)["'][^>]*>/gi);
      for (const match of imageMatches) {
        slide.elements.push({
          id: crypto.randomUUID(),
          type: ElementType.IMAGE,
          src: match[1],
          position: { x: 0, y: 0 },
          size: { width: 400, height: 300 }
        });
      }

      slides.push(slide);
    });

    return {
      metadata: {
        id: crypto.randomUUID(),
        type: 'presentation',
        title,
        version: '1.0.0',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      },
      theme: {
        name: 'default',
        colors: {
          primary: '#007bff',
          secondary: '#6c757d',
          background: '#ffffff',
          text: '#333333',
          accent: '#28a745'
        },
        fonts: {
          heading: 'Arial',
          body: 'Helvetica',
          code: 'Courier New'
        },
        spacing: {
          padding: 20,
          margin: 10
        }
      },
      slides,
      settings: {
        aspectRatio: '16-9',
        showSlideNumbers: true,
        enableTransitions: true
      }
    };
  }

  /**
   * Convert Markdown to unified document
   */
  static fromMarkdown(markdown: string, title: string = 'Untitled'): PresentationDocument {
    // Parse Markdown and convert to unified format
    const slides: Slide[] = [];
    const slideContents = markdown.split(/^---$/gm);

    slideContents.forEach((content, index) => {
      const lines = content.trim().split('\n');
      const slide: Slide = {
        id: crypto.randomUUID(),
        index,
        layout: SlideLayout.BLANK,
        elements: [],
        background: {
          type: 'color',
          value: '#ffffff'
        }
      };

      let currentElement: TextElement | null = null;

      for (const line of lines) {
        // Headers
        if (line.startsWith('# ')) {
          slide.title = line.substring(2).trim();
        } else if (line.startsWith('## ')) {
          // Subheading as text element
          slide.elements.push({
            id: crypto.randomUUID(),
            type: ElementType.TEXT,
            content: line.substring(3).trim(),
            position: { x: 0, y: 0 },
            size: { width: 800, height: 50 },
            style: {
              fontSize: 32,
              fontWeight: 'bold',
              color: '#333333'
            }
          });
        } else if (line.startsWith('```')) {
          if (currentElement && (currentElement as any).language) {
            // End code block
            slide.elements.push(currentElement);
            currentElement = null;
          } else {
            // Start code block
            const language = line.substring(3).trim() || 'text';
            currentElement = {
              id: crypto.randomUUID(),
              type: 'code',
              content: '',
              language,
              position: { x: 0, y: 0 },
              size: { width: 800, height: 200 }
            } as any;
          }
        } else if (currentElement && (currentElement as any).language) {
          // Code content
          (currentElement as any).content += line + '\n';
        } else if (line.trim()) {
          slide.elements.push({
            id: crypto.randomUUID(),
            type: ElementType.TEXT,
            content: line.trim(),
            position: { x: 0, y: 0 },
            size: { width: 800, height: 40 },
            style: {
              fontSize: 24,
              color: '#333333'
            }
          });
        }
      }

      if (currentElement) {
        slide.elements.push(currentElement);
      }

      slides.push(slide);
    });

    return {
      metadata: {
        id: crypto.randomUUID(),
        type: 'presentation',
        title,
        version: '1.0.0',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      },
      theme: {
        name: 'default',
        colors: {
          primary: '#007bff',
          secondary: '#6c757d',
          background: '#ffffff',
          text: '#333333',
          accent: '#28a745'
        },
        fonts: {
          heading: 'Arial',
          body: 'Helvetica',
          code: 'Courier New'
        },
        spacing: {
          padding: 20,
          margin: 10
        }
      },
      slides,
      settings: {
        aspectRatio: '16-9',
        showSlideNumbers: true,
        enableTransitions: true
      }
    };
  }
}
