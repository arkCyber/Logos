/**
 * Multi-Column Layout Manager
 * Aerospace-grade implementation with comprehensive column layout and validation
 */

export interface ColumnConfig {
  columnCount: number;
  columnGap: string;
  columnRule: string;
  columnRuleColor: string;
  columnRuleStyle: string;
  columnRuleWidth: string;
  columnWidth: string;
  balance: boolean;
}

class MultiColumnManager {
  private static instance: MultiColumnManager;
  private config: ColumnConfig = {
    columnCount: 1,
    columnGap: '20px',
    columnRule: 'none',
    columnRuleColor: '#e5e7eb',
    columnRuleStyle: 'solid',
    columnRuleWidth: '1px',
    columnWidth: 'auto',
    balance: false
  };

  private constructor() {}

  static getInstance(): MultiColumnManager {
    if (!MultiColumnManager.instance) {
      MultiColumnManager.instance = new MultiColumnManager();
    }
    return MultiColumnManager.instance;
  }

  /**
   * Set column configuration
   * @param config - Column configuration
   */
  setConfig(config: Partial<ColumnConfig>): void {
    this.validateConfig(config);
    this.config = { ...this.config, ...config };
  }

  /**
   * Get current column configuration
   * @returns Column configuration
   */
  getConfig(): ColumnConfig {
    return { ...this.config };
  }

  /**
   * Set number of columns
   * @param count - Number of columns (1-6)
   */
  setColumnCount(count: number): void {
    if (count < 1 || count > 6) {
      throw new Error('Column count must be between 1 and 6');
    }
    this.config.columnCount = count;
  }

  /**
   * Get number of columns
   * @returns Number of columns
   */
  getColumnCount(): number {
    return this.config.columnCount;
  }

  /**
   * Set column gap
   * @param gap - Column gap (CSS value)
   */
  setColumnGap(gap: string): void {
    if (!gap || gap.trim().length === 0) {
      throw new Error('Column gap cannot be empty');
    }
    this.config.columnGap = gap;
  }

  /**
   * Set column rule
   * @param rule - Column rule (none, solid, dotted, dashed, double)
   */
  setColumnRule(rule: string): void {
    const validRules = ['none', 'solid', 'dotted', 'dashed', 'double'];
    if (!validRules.includes(rule)) {
      throw new Error(`Invalid column rule. Must be one of: ${validRules.join(', ')}`);
    }
    this.config.columnRule = rule;
  }

  /**
   * Set column rule color
   * @param color - CSS color value
   */
  setColumnRuleColor(color: string): void {
    if (!color || color.trim().length === 0) {
      throw new Error('Column rule color cannot be empty');
    }
    this.config.columnRuleColor = color;
  }

  /**
   * Set column rule width
   * @param width - CSS width value
   */
  setColumnRuleWidth(width: string): void {
    if (!width || width.trim().length === 0) {
      throw new Error('Column rule width cannot be empty');
    }
    this.config.columnRuleWidth = width;
  }

  /**
   * Enable or disable column balancing
   * @param balance - Enable balancing
   */
  setBalance(balance: boolean): void {
    this.config.balance = balance;
  }

  /**
   * Generate CSS for multi-column layout
   * @returns CSS string
   */
  generateCSS(): string {
    return `
      .multi-column-layout {
        column-count: ${this.config.columnCount};
        column-gap: ${this.config.columnGap};
        column-rule: ${this.config.columnRule} ${this.config.columnRuleWidth} ${this.config.columnRuleColor};
        column-width: ${this.config.columnWidth};
        column-fill: ${this.config.balance ? 'balance' : 'auto'};
      }
    `;
  }

  /**
   * Apply multi-column layout to HTML content
   * @param html - HTML content
   * @returns HTML with multi-column layout
   */
  applyLayout(html: string): string {
    if (this.config.columnCount === 1) {
      return html;
    }

    return `<div class="multi-column-layout">${html}</div>`;
  }

  /**
   * Remove multi-column layout from HTML
   * @param html - HTML content
   * @returns HTML without multi-column layout
   */
  removeLayout(html: string): string {
    return html.replace(/<div class="multi-column-layout">([\s\S]*?)<\/div>/, '$1');
  }

  /**
   * Validate column configuration
   */
  private validateConfig(config: Partial<ColumnConfig>): void {
    if (config.columnCount !== undefined) {
      if (config.columnCount < 1 || config.columnCount > 6) {
        throw new Error('Column count must be between 1 and 6');
      }
    }

    if (config.columnGap !== undefined) {
      if (!config.columnGap || config.columnGap.trim().length === 0) {
        throw new Error('Column gap cannot be empty');
      }
    }

    if (config.columnRule !== undefined) {
      const validRules = ['none', 'solid', 'dotted', 'dashed', 'double'];
      if (!validRules.includes(config.columnRule)) {
        throw new Error(`Invalid column rule. Must be one of: ${validRules.join(', ')}`);
      }
    }
  }

  /**
   * Reset to default configuration
   */
  reset(): void {
    this.config = {
      columnCount: 1,
      columnGap: '20px',
      columnRule: 'none',
      columnRuleColor: '#e5e7eb',
      columnRuleStyle: 'solid',
      columnRuleWidth: '1px',
      columnWidth: 'auto',
      balance: false
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
      throw new Error('Failed to import configuration: Invalid JSON format');
    }
  }
}

export const multiColumnManager = MultiColumnManager.getInstance();

/**
 * Multi-column styles for CSS injection
 */
export const MULTI_COLUMN_STYLES = `
.multi-column-layout {
  column-count: 1;
  column-gap: 20px;
  column-rule: none;
  column-width: auto;
  column-fill: auto;
  orphans: 3;
  widows: 3;
  break-inside: avoid;
}

.multi-column-layout.two-columns {
  column-count: 2;
}

.multi-column-layout.three-columns {
  column-count: 3;
}

.multi-column-layout.four-columns {
  column-count: 4;
}

.multi-column-layout.with-rule {
  column-rule: solid 1px #e5e7eb;
}

.multi-column-layout.balanced {
  column-fill: balance;
}

.editor-container.dark .multi-column-layout.with-rule {
  column-rule-color: #374151;
}

@media print {
  .multi-column-layout {
    break-inside: avoid;
  }
  
  .multi-column-layout p,
  .multi-column-layout h1,
  .multi-column-layout h2,
  .multi-column-layout h3,
  .multi-column-layout h4,
  .multi-column-layout h5,
  .multi-column-layout h6 {
    break-inside: avoid;
  }
}
`;
