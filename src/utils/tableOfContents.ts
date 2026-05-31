/**
 * Table of Contents Generator
 * Aerospace-grade implementation with comprehensive error handling and validation
 */

export interface TOCItem {
  id: string;
  level: number;
  text: string;
  position: number;
  children: TOCItem[];
}

export interface TOCConfig {
  maxDepth: number;
  includeNumbering: boolean;
  includePageNumbers: boolean;
  style: 'numbered' | 'bulleted' | 'plain';
}

class TableOfContentsGenerator {
  private static instance: TableOfContentsGenerator;
  private tocItems: TOCItem[] = [];
  private config: TOCConfig = {
    maxDepth: 6,
    includeNumbering: true,
    includePageNumbers: false,
    style: 'numbered'
  };

  private constructor() {}

  static getInstance(): TableOfContentsGenerator {
    if (!TableOfContentsGenerator.instance) {
      TableOfContentsGenerator.instance = new TableOfContentsGenerator();
    }
    return TableOfContentsGenerator.instance;
  }

  /**
   * Generate table of contents from HTML content
   * @param html - HTML content
   * @param config - TOC configuration
   * @returns Array of TOC items
   */
  generateFromHTML(html: string, config?: Partial<TOCConfig>): TOCItem[] {
    if (!html || html.trim().length === 0) {
      throw new Error('HTML content cannot be empty');
    }

    // Update config if provided
    if (config) {
      this.config = { ...this.config, ...config };
    }

    // Clear previous TOC
    this.tocItems = [];

    // Parse HTML and extract headings
    const parser = new DOMParser();
    const doc = parser.parseFromString(html, 'text/html');
    const headings = doc.querySelectorAll('h1, h2, h3, h4, h5, h6');

    let position = 0;
    const stack: TOCItem[] = [];

    headings.forEach((heading, index) => {
      const level = parseInt(heading.tagName.charAt(1));
      const text = heading.textContent || '';
      const id = heading.id || `heading-${index}`;

      if (level > this.config.maxDepth) {
        return; // Skip headings beyond max depth
      }

      const tocItem: TOCItem = {
        id,
        level,
        text,
        position,
        children: []
      };

      // Build hierarchical structure
      while (stack.length > 0 && stack[stack.length - 1].level >= level) {
        stack.pop();
      }

      if (stack.length === 0) {
        this.tocItems.push(tocItem);
      } else {
        stack[stack.length - 1].children.push(tocItem);
      }

      stack.push(tocItem);
      position++;
    });

    return this.tocItems;
  }

  /**
   * Generate table of contents from TipTap document
   * @param doc - TipTap document
   * @param config - TOC configuration
   * @returns Array of TOC items
   */
  generateFromTipTap(doc: any, config?: Partial<TOCConfig>): TOCItem[] {
    if (!doc || !doc.content) {
      throw new Error('Invalid TipTap document');
    }

    if (config) {
      this.config = { ...this.config, ...config };
    }

    this.tocItems = [];
    const position = 0;
    const stack: TOCItem[] = [];

    this.traverseTipTapNode(doc.content, stack, position);

    return this.tocItems;
  }

  /**
   * Traverse TipTap document nodes recursively
   */
  private traverseTipTapNode(nodes: any[], stack: TOCItem[], position: number): number {
    let currentPosition = position;

    nodes.forEach((node, index) => {
      if (node.type === 'heading') {
        const level = node.attrs?.level || 1;
        const text = this.extractTextFromNode(node);
        const id = node.attrs?.id || `heading-${index}`;

        if (level <= this.config.maxDepth) {
          const tocItem: TOCItem = {
            id,
            level,
            text,
            position: currentPosition,
            children: []
          };

          while (stack.length > 0 && stack[stack.length - 1].level >= level) {
            stack.pop();
          }

          if (stack.length === 0) {
            this.tocItems.push(tocItem);
          } else {
            stack[stack.length - 1].children.push(tocItem);
          }

          stack.push(tocItem);
          currentPosition++;
        }
      }

      if (node.content) {
        currentPosition = this.traverseTipTapNode(node.content, stack, currentPosition);
      }
    });

    return currentPosition;
  }

  /**
   * Extract text from TipTap node
   */
  private extractTextFromNode(node: any): string {
    if (node.text) {
      return node.text;
    }

    if (node.content) {
      return node.content.map((child: any) => this.extractTextFromNode(child)).join('');
    }

    return '';
  }

  /**
   * Generate HTML for table of contents
   * @param tocItems - TOC items
   * @returns HTML string
   */
  generateHTML(tocItems?: TOCItem[]): string {
    const items = tocItems || this.tocItems;
    if (items.length === 0) {
      return '<div class="toc-empty">No headings found</div>';
    }

    let html = '<div class="table-of-contents">';
    html += '<h3 class="toc-title">Table of Contents</h3>';
    html += '<ul class="toc-list">';

    html += this.generateTOCItemsHTML(items, 1);

    html += '</ul></div>';
    return html;
  }

  /**
   * Generate HTML for TOC items recursively
   */
  private generateTOCItemsHTML(items: TOCItem[], depth: number): string {
    let html = '';

    items.forEach((item, index) => {
      const numbering = this.config.includeNumbering ? this.generateNumbering(item, index) : '';

      html += `<li class="toc-item toc-level-${item.level}">`;
      html += `<a href="#${item.id}" class="toc-link" data-level="${item.level}">`;
      html += `<span class="toc-numbering">${numbering}</span>`;
      html += `<span class="toc-text">${item.text}</span>`;

      if (this.config.includePageNumbers) {
        html += '<span class="toc-page-number">--</span>';
      }

      html += '</a>';

      if (item.children.length > 0) {
        html += '<ul class="toc-sublist">';
        html += this.generateTOCItemsHTML(item.children, depth + 1);
        html += '</ul>';
      }

      html += '</li>';
    });

    return html;
  }

  /**
   * Generate numbering for TOC item
   */
  private generateNumbering(item: TOCItem, index: number): string {
    if (this.config.style === 'bulleted') {
      return '•';
    }

    if (this.config.style === 'plain') {
      return '';
    }

    // Numbered style
    const prefix = item.level === 1 ? '' : '.'.repeat(item.level - 1);
    return `${prefix}${index + 1}`;
  }

  /**
   * Update TOC when document changes
   * @param html - Updated HTML content
   */
  update(html: string): TOCItem[] {
    return this.generateFromHTML(html);
  }

  /**
   * Get current TOC items
   * @returns Array of TOC items
   */
  getTOC(): TOCItem[] {
    return this.tocItems;
  }

  /**
   * Clear TOC
   */
  clear(): void {
    this.tocItems = [];
  }

  /**
   * Set TOC configuration
   * @param config - TOC configuration
   */
  setConfig(config: Partial<TOCConfig>): void {
    this.config = { ...this.config, ...config };
  }

  /**
   * Get TOC configuration
   * @returns Current TOC configuration
   */
  getConfig(): TOCConfig {
    return { ...this.config };
  }

  /**
   * Validate TOC structure
   * @param tocItems - TOC items to validate
   * @returns True if valid
   */
  validate(tocItems?: TOCItem[]): boolean {
    const items = tocItems || this.tocItems;

    if (!Array.isArray(items)) {
      return false;
    }

    for (const item of items) {
      if (!item.id || !item.text || typeof item.level !== 'number') {
        return false;
      }

      if (item.level < 1 || item.level > 6) {
        return false;
      }

      if (item.children && item.children.length > 0) {
        if (!this.validate(item.children)) {
          return false;
        }
      }
    }

    return true;
  }
}

export const tocGenerator = TableOfContentsGenerator.getInstance();

/**
 * TOC styles for CSS injection
 */
export const TOC_STYLES = `
.table-of-contents {
  background: #f9fafb;
  padding: 20px;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  margin: 20px 0;
}

.toc-title {
  margin: 0 0 15px 0;
  font-size: 18px;
  font-weight: 600;
  color: #111827;
}

.toc-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.toc-sublist {
  list-style: none;
  padding-left: 20px;
  margin: 5px 0;
}

.toc-item {
  margin: 5px 0;
}

.toc-link {
  display: flex;
  align-items: center;
  text-decoration: none;
  color: #374151;
  padding: 4px 8px;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.toc-link:hover {
  background-color: #e5e7eb;
}

.toc-numbering {
  margin-right: 8px;
  font-weight: 500;
  color: #6b7280;
}

.toc-text {
  flex: 1;
}

.toc-page-number {
  margin-left: 10px;
  color: #9ca3af;
  font-size: 12px;
}

.toc-level-1 .toc-link {
  font-weight: 600;
  font-size: 14px;
}

.toc-level-2 .toc-link {
  font-weight: 500;
  font-size: 13px;
}

.toc-level-3 .toc-link,
.toc-level-4 .toc-link,
.toc-level-5 .toc-link,
.toc-level-6 .toc-link {
  font-weight: 400;
  font-size: 12px;
}

.toc-empty {
  color: #9ca3af;
  font-style: italic;
  padding: 10px;
}

.editor-container.dark .table-of-contents {
  background: #1f2937;
  border-color: #374151;
}

.editor-container.dark .toc-title {
  color: #f9fafb;
}

.editor-container.dark .toc-link {
  color: #d1d5db;
}

.editor-container.dark .toc-link:hover {
  background-color: #374151;
}

.editor-container.dark .toc-numbering {
  color: #9ca3af;
}

.editor-container.dark .toc-page-number {
  color: #6b7280;
}
`;
