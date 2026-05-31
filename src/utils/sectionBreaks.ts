/**
 * Section Breaks Manager
 * Aerospace-grade implementation with comprehensive section management and validation
 */

export type SectionBreakType = 'next-page' | 'continuous' | 'even-page' | 'odd-page';

export interface SectionBreak {
  id: string;
  type: SectionBreakType;
  position: number;
  sectionNumber: number;
  createdAt: number;
}

export interface SectionConfig {
  orientation: 'portrait' | 'landscape';
  pageSize: 'letter' | 'legal' | 'a4' | 'a3';
  margins: {
    top: string;
    right: string;
    bottom: string;
    left: string;
  };
  headerEnabled: boolean;
  footerEnabled: boolean;
  pageNumbering: 'continue' | 'restart' | 'none';
  startingNumber?: number;
}

class SectionBreaksManager {
  private static instance: SectionBreaksManager;
  private sectionBreaks: SectionBreak[] = [];
  private sectionConfigs: Map<number, SectionConfig> = new Map();
  private nextSectionNumber: number = 1;

  private constructor() {
    // Initialize default section config
    this.sectionConfigs.set(0, this.getDefaultConfig());
  }

  static getInstance(): SectionBreaksManager {
    if (!SectionBreaksManager.instance) {
      SectionBreaksManager.instance = new SectionBreaksManager();
    }
    return SectionBreaksManager.instance;
  }

  /**
   * Get default section configuration
   */
  private getDefaultConfig(): SectionConfig {
    return {
      orientation: 'portrait',
      pageSize: 'a4',
      margins: {
        top: '2.54cm',
        right: '2.54cm',
        bottom: '2.54cm',
        left: '2.54cm'
      },
      headerEnabled: false,
      footerEnabled: false,
      pageNumbering: 'continue'
    };
  }

  /**
   * Add a section break
   * @param type - Break type
   * @param position - Position in document
   * @param config - Section configuration (optional)
   * @returns Section break ID
   */
  addSectionBreak(
    type: SectionBreakType,
    position: number,
    config?: Partial<SectionConfig>
  ): string {
    const sectionBreak: SectionBreak = {
      id: this.generateId(),
      type,
      position,
      sectionNumber: this.nextSectionNumber++,
      createdAt: Date.now()
    };

    this.sectionBreaks.push(sectionBreak);

    // Add section config if provided
    if (config) {
      const fullConfig = { ...this.getDefaultConfig(), ...config };
      this.sectionConfigs.set(sectionBreak.sectionNumber, fullConfig);
    } else {
      // Copy previous section config
      const prevConfig = this.sectionConfigs.get(sectionBreak.sectionNumber - 1);
      if (prevConfig) {
        this.sectionConfigs.set(sectionBreak.sectionNumber, { ...prevConfig });
      }
    }

    return sectionBreak.id;
  }

  /**
   * Remove a section break
   * @param id - Section break ID
   */
  removeSectionBreak(id: string): void {
    const index = this.sectionBreaks.findIndex(sb => sb.id === id);
    if (index !== -1) {
      const sectionBreak = this.sectionBreaks[index];
      this.sectionBreaks.splice(index, 1);
      this.sectionConfigs.delete(sectionBreak.sectionNumber);
      this.renumberSections();
    }
  }

  /**
   * Get a section break by ID
   * @param id - Section break ID
   * @returns Section break or null
   */
  getSectionBreak(id: string): SectionBreak | null {
    return this.sectionBreaks.find(sb => sb.id === id) || null;
  }

  /**
   * Get all section breaks
   * @returns Array of section breaks
   */
  getSectionBreaks(): SectionBreak[] {
    return [...this.sectionBreaks];
  }

  /**
   * Get section number for a position
   * @param position - Position in document
   * @returns Section number
   */
  getSectionNumber(position: number): number {
    let sectionNumber = 0;
    for (const breakPoint of this.sectionBreaks) {
      if (position >= breakPoint.position) {
        sectionNumber = breakPoint.sectionNumber;
      } else {
        break;
      }
    }
    return sectionNumber;
  }

  /**
   * Get section configuration
   * @param sectionNumber - Section number
   * @returns Section configuration
   */
  getSectionConfig(sectionNumber: number): SectionConfig {
    return this.sectionConfigs.get(sectionNumber) || this.getDefaultConfig();
  }

  /**
   * Set section configuration
   * @param sectionNumber - Section number
   * @param config - Section configuration
   */
  setSectionConfig(sectionNumber: number, config: Partial<SectionConfig>): void {
    const currentConfig = this.sectionConfigs.get(sectionNumber) || this.getDefaultConfig();
    const newConfig = { ...currentConfig, ...config };
    this.validateConfig(newConfig);
    this.sectionConfigs.set(sectionNumber, newConfig);
  }

  /**
   * Update section break type
   * @param id - Section break ID
   * @param type - New break type
   */
  updateSectionBreakType(id: string, type: SectionBreakType): void {
    const sectionBreak = this.sectionBreaks.find(sb => sb.id === id);
    if (sectionBreak) {
      sectionBreak.type = type;
    }
  }

  /**
   * Renumber sections after deletion
   */
  private renumberSections(): void {
    this.sectionBreaks.forEach((sb, index) => {
      sb.sectionNumber = index + 1;
    });
    this.nextSectionNumber = this.sectionBreaks.length + 1;
  }

  /**
   * Generate HTML for section break
   * @param type - Break type
   * @returns HTML string
   */
  generateSectionBreakHTML(type: SectionBreakType): string {
    const className = `section-break section-break-${type}`;
    const label = this.getBreakLabel(type);
    return `<div class="${className}" data-type="${type}" title="${label}"></div>`;
  }

  /**
   * Get human-readable label for break type
   */
  private getBreakLabel(type: SectionBreakType): string {
    switch (type) {
      case 'next-page':
        return 'Next Page';
      case 'continuous':
        return 'Continuous';
      case 'even-page':
        return 'Even Page';
      case 'odd-page':
        return 'Odd Page';
      default:
        return 'Section Break';
    }
  }

  /**
   * Validate section configuration
   */
  private validateConfig(config: SectionConfig): void {
    const validOrientations = ['portrait', 'landscape'];
    if (!validOrientations.includes(config.orientation)) {
      throw new Error(`Invalid orientation. Must be one of: ${validOrientations.join(', ')}`);
    }

    const validPageSizes = ['letter', 'legal', 'a4', 'a3'];
    if (!validPageSizes.includes(config.pageSize)) {
      throw new Error(`Invalid page size. Must be one of: ${validPageSizes.join(', ')}`);
    }

    const validNumbering = ['continue', 'restart', 'none'];
    if (!validNumbering.includes(config.pageNumbering)) {
      throw new Error(`Invalid page numbering. Must be one of: ${validNumbering.join(', ')}`);
    }

    if (config.pageNumbering === 'restart' && config.startingNumber === undefined) {
      throw new Error('Starting number is required when page numbering is set to restart');
    }
  }

  /**
   * Clear all section breaks
   */
  clearAll(): void {
    this.sectionBreaks = [];
    this.sectionConfigs.clear();
    this.sectionConfigs.set(0, this.getDefaultConfig());
    this.nextSectionNumber = 1;
  }

  /**
   * Get statistics
   */
  getStatistics(): {
    totalSections: number;
    byType: Record<string, number>;
  } {
    const byType: Record<string, number> = {};

    this.sectionBreaks.forEach(sb => {
      byType[sb.type] = (byType[sb.type] || 0) + 1;
    });

    return {
      totalSections: this.sectionBreaks.length + 1, // +1 for initial section
      byType
    };
  }

  /**
   * Generate unique ID
   */
  private generateId(): string {
    return `section-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  /**
   * Export to JSON
   * @returns JSON string
   */
  exportToJSON(): string {
    const data = {
      sectionBreaks: this.sectionBreaks,
      sectionConfigs: Array.from(this.sectionConfigs.entries()),
      exportedAt: Date.now()
    };
    return JSON.stringify(data, null, 2);
  }

  /**
   * Import from JSON
   * @param json - JSON string
   */
  importFromJSON(json: string): void {
    try {
      const data = JSON.parse(json);
      if (data.sectionBreaks && Array.isArray(data.sectionBreaks)) {
        this.sectionBreaks = data.sectionBreaks;
      }
      if (data.sectionConfigs && Array.isArray(data.sectionConfigs)) {
        this.sectionConfigs = new Map(data.sectionConfigs);
      }
      this.nextSectionNumber = this.sectionBreaks.length + 1;
    } catch (error) {
      throw new Error('Failed to import section breaks: Invalid JSON format');
    }
  }
}

export const sectionBreaksManager = SectionBreaksManager.getInstance();

/**
 * Section break styles for CSS injection
 */
export const SECTION_BREAK_STYLES = `
.section-break {
  width: 100%;
  height: 2px;
  margin: 20px 0;
  border: none;
  position: relative;
}

.section-break::before {
  content: '';
  position: absolute;
  left: 0;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
}

.section-break-next-page::before {
  border-top: 2px dashed #9ca3af;
}

.section-break-continuous::before {
  border-top: 1px solid #e5e7eb;
}

.section-break-even-page::before {
  border-top: 2px dotted #9ca3af;
}

.section-break-odd-page::before {
  border-top: 2px double #9ca3af;
}

.section-break::after {
  content: attr(title);
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  background: #f9fafb;
  padding: 2px 8px;
  font-size: 10px;
  color: #6b7280;
  border-radius: 4px;
  white-space: nowrap;
}

.editor-container.dark .section-break-next-page::before {
  border-top-color: #4b5563;
}

.editor-container.dark .section-break-continuous::before {
  border-top-color: #374151;
}

.editor-container.dark .section-break-even-page::before {
  border-top-color: #4b5563;
}

.editor-container.dark .section-break-odd-page::before {
  border-top-color: #4b5563;
}

.editor-container.dark .section-break::after {
  background: #1f2937;
  color: #9ca3af;
}

@media print {
  .section-break-next-page {
    page-break-before: always;
  }

  .section-break-continuous {
    page-break-before: auto;
  }

  .section-break-even-page {
    page-break-before: left;
  }

  .section-break-odd-page {
    page-break-before: right;
  }

  .section-break {
    display: none;
  }
}
`;
