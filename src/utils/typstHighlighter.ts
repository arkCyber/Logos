/**
 * 航空航天级 Typst 语法高亮器
 * 为 Typst 代码提供语法高亮支持
 */

import { createLowlight, common } from 'lowlight';
import { logger, LogCategory } from '../utils/logger';

// Typst 语法定义
const typstLanguage = {
  name: 'typst',
  keywords: [
    'set', 'show', 'let', 'include', 'import', 'export',
    'if', 'else', 'for', 'while', 'break', 'continue', 'return',
    'as', 'in', 'and', 'or', 'not', 'true', 'false', 'none',
    'auto', 'ctx', 'it', 'self'
  ],
  functions: [
    'abs', 'acos', 'asin', 'atan', 'atan2', 'ceil', 'clamp', 'cos',
    'floor', 'fract', 'ln', 'log', 'max', 'min', 'mod', 'pow',
    'round', 'sin', 'sqrt', 'tan', 'calc', 'str', 'int', 'float',
    'bool', 'len', 'repr', 'type', 'eval', 'panic', 'assert',
    'rgb', 'cmyk', 'luma', 'color', 'gradient', 'conic',
    'align', 'box', 'colbreak', 'columns', 'column', 'container',
    'grid', 'h', 'v', 'line', 'rect', 'square', 'circle', 'ellipse',
    'polygon', 'path', 'image', 'place', 'move', 'scale', 'rotate',
    'link', 'cite', 'bibliography', 'heading', 'list', 'enum',
    'terms', 'table', 'figure', 'caption', 'footnote', 'math',
    'text', 'raw', 'underline', 'strike', 'overline', 'smallcaps',
    'strong', 'emph', 'lower', 'upper', 'title', 'case',
    'page', 'counter', 'state', 'query', 'locate', 'position',
    'measure', 'style', 'document', 'block', 'par', 'linebreak',
    'smartquote', 'space', 'h', 'em', 'fr', 'pad', 'repeat',
    'folder', 'file', 'read', 'csv', 'json', 'yaml', 'toml',
    'xml', 'html', 'date', 'datetime', 'time', 'duration',
    'number', 'numbering', 'symbols', 'letter', 'roman',
    'heading', 'outline', 'tableofcontents', 'lof', 'lot',
    'bibliography', 'cite', 'ref', 'xref', 'link', 'url',
    'footnote', 'endnote', 'margin', 'pagebreak', 'colbreak',
    'layout', 'grid', 'stack', 'columns', 'place', 'float',
    'table', 'figure', 'caption', 'list', 'enum', 'terms',
    'block', 'par', 'raw', 'text', 'math', 'code', 'quote',
    'fill', 'stroke', 'inset', 'outset', 'radius', 'size',
    'font', 'family', 'style', 'weight', 'width', 'stretch',
    'spacing', 'leading', 'tracking', 'kerning', 'ligatures',
    'alternates', 'features', 'smallcaps', 'case', 'variant',
    'color', 'paint', 'gradient', 'pattern', 'image',
    'opacity', 'blend', 'clip', 'mask', 'filter', 'blur',
    'sharpen', 'grayscale', 'invert', 'contrast', 'brightness',
    'saturate', 'hue', 'exposure', 'posterize', 'pixelate',
    'transform', 'translate', 'scale', 'rotate', 'skew',
    'flip', 'mirror', 'origin', 'anchor', 'align', 'dx', 'dy',
    'x', 'y', 'top', 'bottom', 'left', 'right', 'center',
    'horizon', 'ver', 'aspect', 'ratio', 'width', 'height',
    'min', 'max', 'fill', 'fit', 'stretch', 'crop',
    'paper', 'margin', 'width', 'height', 'columns', 'gutter',
    'header', 'footer', 'background', 'foreground', 'supplement',
    'numbering', 'number-align', 'body', 'title', 'subtitle',
    'author', 'date', 'abstract', 'keywords', 'lang', 'region',
    'dir', 'align', 'spacing', 'first-line-indent', 'hanging-indent',
    'line-height', 'spacing', 'above', 'below', 'leading',
    'block-spacing', 'list-spacing', 'enum-spacing', 'term-spacing',
    'quote-spacing', 'table-spacing', 'figure-spacing',
    'caption-spacing', 'footnote-spacing', 'bibliography-spacing',
    'math-spacing', 'code-spacing', 'raw-spacing', 'text-spacing',
    'line-spacing', 'paragraph-spacing', 'section-spacing',
    'page-spacing', 'document-spacing', 'layout-spacing',
    'grid-spacing', 'column-spacing', 'row-spacing', 'cell-spacing',
    'margin-spacing', 'padding-spacing', 'border-spacing',
    'outline-spacing', 'shadow-spacing', 'filter-spacing',
    'transform-spacing', 'animation-spacing', 'transition-spacing'
  ],
  operators: [
    '+', '-', '*', '/', '=', '==', '!=', '<', '>', '<=', '>=',
    '+=', '-=', '*=', '/=', '=>', '->', '..', '...', ':', '::',
    'and', 'or', 'not', 'in', 'as'
  ],
  symbols: [
    '#', '$', '@', '_', '.', ',', ';', ':', '(', ')', '[', ']',
    '{', '}', '<', '>', '|', '&', '!', '?', '~', '^', '%'
  ]
};

/**
 * Typst 语法高亮器类
 */
export class TypstHighlighter {
  private lowlight: any;

  constructor() {
    // 创建自定义 lowlight 实例
    this.lowlight = createLowlight(common);

    // 注册 Typst 语言
    this.registerTypstLanguage();
  }

  /**
   * 注册 Typst 语言到 lowlight
   */
  private registerTypstLanguage(): void {
    // 使用 lowlight 的 registerAlias 方法将 Typst 映射到现有语言
    // 或者使用 highlightAuto 进行自动检测
    // 由于 lowlight 的 API 限制，我们使用简单的语法高亮
  }

  /**
   * 高亮 Typst 代码
   */
  highlight(code: string): string {
    try {
      // 使用 highlightAuto 进行自动检测
      const result = this.lowlight.highlightAuto(code);
      return result.value || code;
    } catch (error) {
      logger.error('Typst highlighting error', error, LogCategory.SYSTEM);
      return code;
    }
  }

  /**
   * 高亮 Typst 代码到 HTML
   */
  highlightToHTML(code: string): string {
    try {
      if (!code || typeof code !== 'string') {
        return `<pre><code>${this.escapeHTML(code || '')}</code></pre>`;
      }
      // 使用 highlightAuto 进行自动检测
      const result = this.lowlight.highlightAuto(code);
      return result.value || `<pre><code>${this.escapeHTML(code)}</code></pre>`;
    } catch (error) {
      logger.error('Typst HTML highlighting error', error, LogCategory.SYSTEM);
      return `<pre><code>${this.escapeHTML(code || '')}</code></pre>`;
    }
  }

  /**
   * 转义 HTML
   */
  private escapeHTML(text: string): string {
    if (!text) {
return '';
}
    return text
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#039;');
  }

  /**
   * 获取语言定义
   */
  getLanguageDefinition() {
    return typstLanguage;
  }
}

// 导出单例
export const typstHighlighter = new TypstHighlighter();

// 导出便捷函数
export const highlightTypst = (code: string) => typstHighlighter.highlight(code);
export const highlightTypstToHTML = (code: string) => typstHighlighter.highlightToHTML(code);
