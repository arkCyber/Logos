/**
 * Print Preview Manager
 * Aerospace-grade implementation with comprehensive print preview and validation
 */

export interface PrintConfig {
  pageSize: 'letter' | 'legal' | 'a4' | 'a3';
  orientation: 'portrait' | 'landscape';
  margins: {
    top: number;
    right: number;
    bottom: number;
    left: number;
  };
  scale: number;
  showHeader: boolean;
  showFooter: boolean;
  showPageNumbers: boolean;
  backgroundColors: boolean;
  graphics: boolean;
}

export interface PagePreview {
  pageNumber: number;
  content: string;
  width: number;
  height: number;
}

class PrintPreviewManager {
  private static instance: PrintPreviewManager;
  private config: PrintConfig = {
    pageSize: 'a4',
    orientation: 'portrait',
    margins: {
      top: 20,
      right: 20,
      bottom: 20,
      left: 20
    },
    scale: 1.0,
    showHeader: true,
    showFooter: true,
    showPageNumbers: true,
    backgroundColors: false,
    graphics: true
  };

  private constructor() {}

  static getInstance(): PrintPreviewManager {
    if (!PrintPreviewManager.instance) {
      PrintPreviewManager.instance = new PrintPreviewManager();
    }
    return PrintPreviewManager.instance;
  }

  /**
   * Set print configuration
   * @param config - Print configuration
   */
  setConfig(config: Partial<PrintConfig>): void {
    this.validateConfig(config);
    this.config = { ...this.config, ...config };
  }

  /**
   * Get print configuration
   * @returns Print configuration
   */
  getConfig(): PrintConfig {
    return { ...this.config };
  }

  /**
   * Generate print preview
   * @param html - HTML content
   * @returns Array of page previews
   */
  generatePreview(html: string): PagePreview[] {
    const pages: PagePreview[] = [];
    const pageSize = this.getPageSize();
    const contentArea = this.getContentArea();

    // Parse HTML and split into pages
    const parser = new DOMParser();
    const doc = parser.parseFromString(html, 'text/html');

    // Calculate content height
    const contentHeight = this.estimateContentHeight(doc.body);

    // Calculate number of pages
    const pageCount = Math.ceil(contentHeight / contentArea.height);

    for (let i = 0; i < pageCount; i++) {
      const page = this.generatePagePreview(html, i + 1, pageSize, contentArea);
      pages.push(page);
    }

    return pages;
  }

  /**
   * Get page size in pixels
   */
  private getPageSize(): { width: number; height: number } {
    const sizes: Record<PrintConfig['pageSize'], { portrait: number; landscape: number }> = {
      letter: { portrait: 794, landscape: 1123 },
      legal: { portrait: 1008, landscape: 1263 },
      a4: { portrait: 794, landscape: 1123 },
      a3: { portrait: 1123, landscape: 1587 }
    };

    const size = sizes[this.config.pageSize];
    const dimension = this.config.orientation === 'portrait' ? size.portrait : size.landscape;

    return {
      width: dimension,
      height: this.config.orientation === 'portrait' ? size.landscape : size.portrait
    };
  }

  /**
   * Get content area (page size minus margins)
   */
  private getContentArea(): { width: number; height: number } {
    const pageSize = this.getPageSize();
    return {
      width: pageSize.width - this.config.margins.left - this.config.margins.right,
      height: pageSize.height - this.config.margins.top - this.config.margins.bottom
    };
  }

  /**
   * Estimate content height
   */
  private estimateContentHeight(element: HTMLElement): number {
    // This is a simplified estimation
    // In production, you would use actual rendering to measure
    const text = element.textContent || '';
    const estimatedLines = Math.ceil(text.length / 80); // Assume 80 chars per line
    const lineHeight = 16; // pixels
    return estimatedLines * lineHeight;
  }

  /**
   * Generate page preview
   */
  private generatePagePreview(
    html: string,
    pageNumber: number,
    pageSize: { width: number; height: number },
    _contentArea: { width: number; height: number }
  ): PagePreview {
    let pageContent = html;

    // Add header if enabled
    if (this.config.showHeader) {
      pageContent = this.addHeader(pageContent, pageNumber);
    }

    // Add footer if enabled
    if (this.config.showFooter) {
      pageContent = this.addFooter(pageContent, pageNumber);
    }

    return {
      pageNumber,
      content: pageContent,
      width: pageSize.width,
      height: pageSize.height
    };
  }

  /**
   * Add header to page
   */
  private addHeader(html: string, pageNumber: number): string {
    return `
      <div class="print-header" style="
        position: fixed;
        top: ${this.config.margins.top}px;
        left: ${this.config.margins.left}px;
        right: ${this.config.margins.right}px;
        height: 30px;
        border-bottom: 1px solid #ccc;
        padding-bottom: 10px;
      ">
        <div class="header-content">
          ${this.config.showPageNumbers ? `<span class="page-number">Page ${pageNumber}</span>` : ''}
        </div>
      </div>
      ${html}
    `;
  }

  /**
   * Add footer to page
   */
  private addFooter(html: string, pageNumber: number): string {
    return `
      ${html}
      <div class="print-footer" style="
        position: fixed;
        bottom: ${this.config.margins.bottom}px;
        left: ${this.config.margins.left}px;
        right: ${this.config.margins.right}px;
        height: 30px;
        border-top: 1px solid #ccc;
        padding-top: 10px;
      ">
        <div class="footer-content">
          ${this.config.showPageNumbers ? `<span class="page-number">Page ${pageNumber}</span>` : ''}
        </div>
      </div>
    `;
  }

  /**
   * Generate print CSS
   * @returns CSS string
   */
  generatePrintCSS(): string {
    const pageSize = this.getPageSize();

    return `
      @page {
        size: ${this.config.pageSize} ${this.config.orientation};
        margin: ${this.config.margins.top}px ${this.config.margins.right}px ${this.config.margins.bottom}px ${this.config.margins.left}px;
      }

      @media print {
        body {
          width: ${pageSize.width}px;
          height: ${pageSize.height}px;
          margin: 0;
          padding: 0;
        }

        .print-header {
          position: fixed;
          top: ${this.config.margins.top}px;
          left: ${this.config.margins.left}px;
          right: ${this.config.margins.right}px;
          height: 30px;
          border-bottom: 1px solid #ccc;
          padding-bottom: 10px;
        }

        .print-footer {
          position: fixed;
          bottom: ${this.config.margins.bottom}px;
          left: ${this.config.margins.left}px;
          right: ${this.config.margins.right}px;
          height: 30px;
          border-top: 1px solid #ccc;
          padding-top: 10px;
        }

        .page-number {
          font-size: 10px;
          color: #666;
        }

        * {
          -webkit-print-color-adjust: ${this.config.backgroundColors ? 'exact' : 'economy'};
          print-color-adjust: ${this.config.backgroundColors ? 'exact' : 'economy'};
        }

        img {
          max-width: 100%;
          height: auto;
          display: ${this.config.graphics ? 'block' : 'none'};
        }

        table {
          page-break-inside: auto;
        }

        tr {
          page-break-inside: avoid;
          page-break-after: auto;
        }

        h1, h2, h3, h4, h5, h6 {
          page-break-after: avoid;
          page-break-inside: avoid;
        }

        p, blockquote {
          page-break-inside: avoid;
        }

        .no-print {
          display: none !important;
        }
      }
    `;
  }

  /**
   * Open print dialog
   */
  print(): void {
    window.print();
  }

  /**
   * Export to PDF (using browser's print to PDF)
   */
  exportToPDF(): void {
    window.print();
  }

  /**
   * Validate print configuration
   */
  private validateConfig(config: Partial<PrintConfig>): void {
    if (config.pageSize) {
      const validSizes = ['letter', 'legal', 'a4', 'a3'];
      if (!validSizes.includes(config.pageSize)) {
        throw new Error(`Invalid page size. Must be one of: ${validSizes.join(', ')}`);
      }
    }

    if (config.orientation) {
      const validOrientations = ['portrait', 'landscape'];
      if (!validOrientations.includes(config.orientation)) {
        throw new Error(`Invalid orientation. Must be one of: ${validOrientations.join(', ')}`);
      }
    }

    if (config.margins) {
      if (
        config.margins.top < 0 ||
        config.margins.right < 0 ||
        config.margins.bottom < 0 ||
        config.margins.left < 0
      ) {
        throw new Error('Margins must be non-negative');
      }
    }

    if (config.scale !== undefined) {
      if (config.scale < 0.1 || config.scale > 2.0) {
        throw new Error('Scale must be between 0.1 and 2.0');
      }
    }
  }

  /**
   * Reset to default configuration
   */
  reset(): void {
    this.config = {
      pageSize: 'a4',
      orientation: 'portrait',
      margins: {
        top: 20,
        right: 20,
        bottom: 20,
        left: 20
      },
      scale: 1.0,
      showHeader: true,
      showFooter: true,
      showPageNumbers: true,
      backgroundColors: false,
      graphics: true
    };
  }

  /**
   * Export configuration to JSON
   * @returns JSON string
   */
  exportToJSON(): string {
    return JSON.stringify(this.config, null, 2);
  }

  /**
   * Import configuration from JSON
   * @param json - JSON string
   */
  importFromJSON(json: string): void {
    try {
      const config = JSON.parse(json);
      this.validateConfig(config);
      this.config = { ...this.config, ...config };
    } catch (error) {
      throw new Error('Failed to import print configuration: Invalid JSON format');
    }
  }
}

export const printPreviewManager = PrintPreviewManager.getInstance();

/**
 * Print preview styles for CSS injection
 */
export const PRINT_PREVIEW_STYLES = `
.print-preview-container {
  background: #f3f4f6;
  padding: 20px;
  min-height: 100vh;
}

.print-preview-page {
  background: white;
  margin: 0 auto 20px auto;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  position: relative;
}

.print-preview-page.a4-portrait {
  width: 794px;
  height: 1123px;
}

.print-preview-page.a4-landscape {
  width: 1123px;
  height: 794px;
}

.print-preview-page.letter-portrait {
  width: 794px;
  height: 1123px;
}

.print-preview-page.letter-landscape {
  width: 1123px;
  height: 794px;
}

.print-preview-toolbar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  background: white;
  border-bottom: 1px solid #e5e7eb;
  padding: 10px 20px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  z-index: 1000;
}

.print-preview-toolbar-left,
.print-preview-toolbar-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.print-preview-button {
  padding: 8px 16px;
  border: 1px solid #d1d5db;
  background: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.print-preview-button:hover {
  background: #f9fafb;
  border-color: #9ca3af;
}

.print-preview-button.primary {
  background: #3b82f6;
  color: white;
  border-color: #3b82f6;
}

.print-preview-button.primary:hover {
  background: #2563eb;
}

.print-preview-select {
  padding: 8px 12px;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  font-size: 14px;
}

.print-preview-scale-slider {
  width: 150px;
}

.print-preview-page-indicator {
  font-size: 14px;
  color: #6b7280;
}

.editor-container.dark .print-preview-container {
  background: #1f2937;
}

.editor-container.dark .print-preview-page {
  background: #374151;
}

.editor-container.dark .print-preview-toolbar {
  background: #1f2937;
  border-bottom-color: #374151;
}

.editor-container.dark .print-preview-button {
  background: #374151;
  color: #f9fafb;
  border-color: #4b5563;
}

.editor-container.dark .print-preview-button:hover {
  background: #4b5563;
}

.editor-container.dark .print-preview-button.primary {
  background: #3b82f6;
  color: white;
  border-color: #3b82f6;
}

.editor-container.dark .print-preview-select {
  background: #374151;
  color: #f9fafb;
  border-color: #4b5563;
}

@media print {
  .print-preview-toolbar,
  .no-print {
    display: none !important;
  }

  .print-preview-container {
    background: white;
    padding: 0;
  }

  .print-preview-page {
    box-shadow: none;
    margin: 0;
  }
}
`;
