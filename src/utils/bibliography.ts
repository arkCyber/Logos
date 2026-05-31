/**
 * Bibliography Management System
 * Aerospace-grade implementation with comprehensive citation management and validation
 */

export type CitationStyle = 'apa' | 'mla' | 'chicago' | 'harvard' | 'ieee' | 'vancouver' | 'bibtex';

export interface BibliographyEntry {
  id: string;
  type: 'book' | 'article' | 'journal' | 'website' | 'conference' | 'thesis' | 'report';
  authors: string[];
  title: string;
  year?: number;
  publisher?: string;
  journal?: string;
  volume?: string;
  issue?: string;
  pages?: string;
  doi?: string;
  url?: string;
  accessedDate?: string;
  customFields?: Record<string, string>;
  tags?: string[];
  notes?: string;
  createdAt: number;
  updatedAt: number;
}

export interface Citation {
  id: string;
  entryId: string;
  position: number;
  style: CitationStyle;
  customFormat?: string;
}

class BibliographyManager {
  private static instance: BibliographyManager;
  private entries: Map<string, BibliographyEntry> = new Map();
  private citations: Citation[] = [];
  private defaultStyle: CitationStyle = 'apa';

  private constructor() {}

  static getInstance(): BibliographyManager {
    if (!BibliographyManager.instance) {
      BibliographyManager.instance = new BibliographyManager();
    }
    return BibliographyManager.instance;
  }

  /**
   * Add a bibliography entry
   * @param entry - Bibliography entry
   * @returns Entry ID
   */
  addEntry(entry: Omit<BibliographyEntry, 'id' | 'createdAt' | 'updatedAt'>): string {
    this.validateEntry(entry);

    const id = this.generateId();
    const now = Date.now();

    const fullEntry: BibliographyEntry = {
      ...entry,
      id,
      createdAt: now,
      updatedAt: now
    };

    this.entries.set(id, fullEntry);
    return id;
  }

  /**
   * Update a bibliography entry
   * @param id - Entry ID
   * @param updates - Partial entry updates
   */
  updateEntry(id: string, updates: Partial<BibliographyEntry>): void {
    const entry = this.entries.get(id);
    if (!entry) {
      throw new Error(`Entry with id ${id} not found`);
    }

    const updatedEntry = {
      ...entry,
      ...updates,
      id,
      updatedAt: Date.now()
    };

    this.validateEntry(updatedEntry);
    this.entries.set(id, updatedEntry);
  }

  /**
   * Delete a bibliography entry
   * @param id - Entry ID
   */
  deleteEntry(id: string): void {
    if (!this.entries.has(id)) {
      throw new Error(`Entry with id ${id} not found`);
    }

    // Remove associated citations
    this.citations = this.citations.filter(c => c.entryId !== id);
    this.entries.delete(id);
  }

  /**
   * Get a bibliography entry by ID
   * @param id - Entry ID
   * @returns Entry or null
   */
  getEntry(id: string): BibliographyEntry | null {
    return this.entries.get(id) || null;
  }

  /**
   * Get all bibliography entries
   * @returns Array of entries
   */
  getAllEntries(): BibliographyEntry[] {
    return Array.from(this.entries.values());
  }

  /**
   * Search entries by query
   * @param query - Search query
   * @returns Array of matching entries
   */
  searchEntries(query: string): BibliographyEntry[] {
    const lowerQuery = query.toLowerCase();
    return this.getAllEntries().filter(
      entry =>
        entry.title.toLowerCase().includes(lowerQuery) ||
        entry.authors.some(a => a.toLowerCase().includes(lowerQuery)) ||
        entry.journal?.toLowerCase().includes(lowerQuery) ||
        entry.publisher?.toLowerCase().includes(lowerQuery) ||
        entry.tags?.some(t => t.toLowerCase().includes(lowerQuery))
    );
  }

  /**
   * Get entries by type
   * @param type - Entry type
   * @returns Array of entries
   */
  getEntriesByType(type: BibliographyEntry['type']): BibliographyEntry[] {
    return this.getAllEntries().filter(e => e.type === type);
  }

  /**
   * Get entries by tag
   * @param tag - Tag name
   * @returns Array of entries
   */
  getEntriesByTag(tag: string): BibliographyEntry[] {
    return this.getAllEntries().filter(e => e.tags?.includes(tag));
  }

  /**
   * Add a citation
   * @param entryId - Entry ID
   * @param position - Citation position in document
   * @param style - Citation style
   * @returns Citation ID
   */
  addCitation(entryId: string, position: number, style?: CitationStyle): string {
    if (!this.entries.has(entryId)) {
      throw new Error(`Entry with id ${entryId} not found`);
    }

    const citation: Citation = {
      id: this.generateId(),
      entryId,
      position,
      style: style || this.defaultStyle
    };

    this.citations.push(citation);
    return citation.id;
  }

  /**
   * Remove a citation
   * @param citationId - Citation ID
   */
  removeCitation(citationId: string): void {
    const index = this.citations.findIndex(c => c.id === citationId);
    if (index !== -1) {
      this.citations.splice(index, 1);
    }
  }

  /**
   * Get all citations
   * @returns Array of citations
   */
  getCitations(): Citation[] {
    return [...this.citations];
  }

  /**
   * Get citations for a specific entry
   * @param entryId - Entry ID
   * @returns Array of citations
   */
  getCitationsForEntry(entryId: string): Citation[] {
    return this.citations.filter(c => c.entryId === entryId);
  }

  /**
   * Format a bibliography entry according to style
   * @param entry - Bibliography entry
   * @param style - Citation style
   * @returns Formatted citation string
   */
  formatEntry(entry: BibliographyEntry, style?: CitationStyle): string {
    const citationStyle = style || this.defaultStyle;

    switch (citationStyle) {
      case 'apa':
        return this.formatAPA(entry);
      case 'mla':
        return this.formatMLA(entry);
      case 'chicago':
        return this.formatChicago(entry);
      case 'harvard':
        return this.formatHarvard(entry);
      case 'ieee':
        return this.formatIEEE(entry);
      case 'vancouver':
        return this.formatVancouver(entry);
      case 'bibtex':
        return this.formatBibTeX(entry);
      default:
        return this.formatAPA(entry);
    }
  }

  /**
   * Format entry in APA style
   */
  private formatAPA(entry: BibliographyEntry): string {
    const authors = entry.authors.join(', ');
    let citation = `${authors} (${entry.year}). ${entry.title}.`;

    if (entry.journal) {
      citation += ` ${entry.journal}`;
      if (entry.volume) {
citation += `, ${entry.volume}`;
}
      if (entry.issue) {
citation += `(${entry.issue})`;
}
      if (entry.pages) {
citation += `, ${entry.pages}`;
}
    } else if (entry.publisher) {
      citation += ` ${entry.publisher}.`;
    }

    if (entry.doi) {
      citation += ` https://doi.org/${entry.doi}`;
    } else if (entry.url) {
      citation += ` ${entry.url}`;
    }

    return citation;
  }

  /**
   * Format entry in MLA style
   */
  private formatMLA(entry: BibliographyEntry): string {
    const authors = entry.authors.join(', ');
    let citation = `${authors}. "${entry.title}."`;

    if (entry.journal) {
      citation += ` ${entry.journal}`;
      if (entry.volume) {
citation += `, vol. ${entry.volume}`;
}
      if (entry.issue) {
citation += `, no. ${entry.issue}`;
}
      if (entry.year) {
citation += `, ${entry.year}`;
}
      if (entry.pages) {
citation += `, pp. ${entry.pages}`;
}
    } else if (entry.publisher) {
      citation += ` ${entry.publisher}`;
      if (entry.year) {
citation += `, ${entry.year}`;
}
    }

    return citation + '.';
  }

  /**
   * Format entry in Chicago style
   */
  private formatChicago(entry: BibliographyEntry): string {
    const authors = entry.authors.join(', ');
    let citation = `${authors}. ${entry.title}.`;

    if (entry.journal) {
      citation += ` ${entry.journal}`;
      if (entry.volume) {
citation += ` ${entry.volume}`;
}
      if (entry.issue) {
citation += `, no. ${entry.issue}`;
}
      if (entry.year) {
citation += ` (${entry.year})`;
}
      if (entry.pages) {
citation += `: ${entry.pages}`;
}
    } else if (entry.publisher) {
      citation += ` ${entry.publisher}`;
      if (entry.year) {
citation += `, ${entry.year}`;
}
    }

    return citation + '.';
  }

  /**
   * Format entry in Harvard style
   */
  private formatHarvard(entry: BibliographyEntry): string {
    const authors = entry.authors.join(', ');
    let citation = `${authors} (${entry.year}) ${entry.title}.`;

    if (entry.journal) {
      citation += ` ${entry.journal}`;
      if (entry.volume) {
citation += `, ${entry.volume}`;
}
      if (entry.issue) {
citation += `(${entry.issue})`;
}
      if (entry.pages) {
citation += `, pp. ${entry.pages}`;
}
    } else if (entry.publisher) {
      citation += ` ${entry.publisher}.`;
    }

    return citation;
  }

  /**
   * Format entry in IEEE style
   */
  private formatIEEE(entry: BibliographyEntry): string {
    const authors = entry.authors.join(', ');
    let citation = `${authors}, "${entry.title},"`;

    if (entry.journal) {
      citation += ` ${entry.journal}`;
      if (entry.volume) {
citation += `, vol. ${entry.volume}`;
}
      if (entry.issue) {
citation += `, no. ${entry.issue}`;
}
      if (entry.pages) {
citation += `, pp. ${entry.pages}`;
}
      if (entry.year) {
citation += `, ${entry.year}`;
}
    } else if (entry.publisher) {
      citation += ` ${entry.publisher}`;
      if (entry.year) {
citation += `, ${entry.year}`;
}
    }

    if (entry.doi) {
      citation += `, doi: ${entry.doi}`;
    }

    return citation + '.';
  }

  /**
   * Format entry in Vancouver style
   */
  private formatVancouver(entry: BibliographyEntry): string {
    const authors = entry.authors.join(', ');
    let citation = `${authors}. ${entry.title}.`;

    if (entry.journal) {
      citation += ` ${entry.journal}`;
      if (entry.year) {
citation += ` ${entry.year}`;
}
      if (entry.volume) {
citation += `;${entry.volume}`;
}
      if (entry.issue) {
citation += `(${entry.issue})`;
}
      if (entry.pages) {
citation += `:${entry.pages}`;
}
    } else if (entry.publisher) {
      citation += ` ${entry.publisher}`;
      if (entry.year) {
citation += `; ${entry.year}`;
}
    }

    return citation + '.';
  }

  /**
   * Format entry in BibTeX format
   */
  private formatBibTeX(entry: BibliographyEntry): string {
    const bibtexType = entry.type === 'article' ? 'article' : 'misc';
    const key = entry.authors[0]?.split(' ')[1]?.toLowerCase() + entry.year || 'unknown';

    let bibtex = `@${bibtexType}{${key},\n`;
    bibtex += `  author = {${entry.authors.join(' and ')}},\n`;
    bibtex += `  title = {${entry.title}},\n`;

    if (entry.year) {
bibtex += `  year = {${entry.year}},\n`;
}
    if (entry.journal) {
bibtex += `  journal = {${entry.journal}},\n`;
}
    if (entry.volume) {
bibtex += `  volume = {${entry.volume}},\n`;
}
    if (entry.issue) {
bibtex += `  number = {${entry.issue}},\n`;
}
    if (entry.pages) {
bibtex += `  pages = {${entry.pages}},\n`;
}
    if (entry.publisher) {
bibtex += `  publisher = {${entry.publisher}},\n`;
}
    if (entry.doi) {
bibtex += `  doi = {${entry.doi}},\n`;
}
    if (entry.url) {
bibtex += `  url = {${entry.url}},\n`;
}

    bibtex = bibtex.slice(0, -2) + '\n}';
    return bibtex;
  }

  /**
   * Generate complete bibliography
   * @param style - Citation style
   * @returns Formatted bibliography HTML
   */
  generateBibliography(style?: CitationStyle): string {
    const entries = this.getAllEntries();
    if (entries.length === 0) {
      return '<div class="bibliography-empty">No bibliography entries</div>';
    }

    let html = '<div class="bibliography">';
    html += '<h3 class="bibliography-title">References</h3>';
    html += '<ol class="bibliography-list">';

    entries.forEach(entry => {
      const formatted = this.formatEntry(entry, style);
      html += `<li class="bibliography-item">${formatted}</li>`;
    });

    html += '</ol></div>';
    return html;
  }

  /**
   * Set default citation style
   * @param style - Citation style
   */
  setDefaultStyle(style: CitationStyle): void {
    this.defaultStyle = style;
  }

  /**
   * Get default citation style
   * @returns Default style
   */
  getDefaultStyle(): CitationStyle {
    return this.defaultStyle;
  }

  /**
   * Import entries from BibTeX
   * @param bibtex - BibTeX string
   * @returns Array of imported entry IDs
   */
  importFromBibTeX(bibtex: string): string[] {
    const importedIds: string[] = [];
    const entries = this.parseBibTeX(bibtex);

    entries.forEach(entry => {
      const id = this.addEntry(entry);
      importedIds.push(id);
    });

    return importedIds;
  }

  /**
   * Parse BibTeX string
   */
  private parseBibTeX(bibtex: string): BibliographyEntry[] {
    // Simplified BibTeX parser
    // In production, use a proper BibTeX parser library
    const entries: BibliographyEntry[] = [];
    const regex = /@(\w+)\s*{([^,]+),\s*([^}]+)}/g;
    let match;

    while ((match = regex.exec(bibtex)) !== null) {
      const [, type, _key, content] = match;
      const entry: Partial<BibliographyEntry> = {
        type: type as BibliographyEntry['type']
      };

      // Parse key-value pairs
      const pairs = content.split(',').map(p => p.trim());
      pairs.forEach(pair => {
        const [key, value] = pair.split('=').map(s => s.trim());
        const cleanValue = value.replace(/[{}"]/g, '');

        switch (key.toLowerCase()) {
          case 'author':
            entry.authors = cleanValue.split(' and ');
            break;
          case 'title':
            entry.title = cleanValue;
            break;
          case 'year':
            entry.year = parseInt(cleanValue);
            break;
          case 'journal':
            entry.journal = cleanValue;
            break;
          case 'publisher':
            entry.publisher = cleanValue;
            break;
          case 'volume':
            entry.volume = cleanValue;
            break;
          case 'number':
            entry.issue = cleanValue;
            break;
          case 'pages':
            entry.pages = cleanValue;
            break;
          case 'doi':
            entry.doi = cleanValue;
            break;
          case 'url':
            entry.url = cleanValue;
            break;
        }
      });

      if (entry.title && entry.authors) {
        entries.push(entry as BibliographyEntry);
      }
    }

    return entries;
  }

  /**
   * Export entries to BibTeX
   * @returns BibTeX string
   */
  exportToBibTeX(): string {
    return this.getAllEntries()
      .map(entry => this.formatBibTeX(entry))
      .join('\n\n');
  }

  /**
   * Clear all entries and citations
   */
  clearAll(): void {
    this.entries.clear();
    this.citations = [];
  }

  /**
   * Validate bibliography entry
   */
  private validateEntry(entry: Partial<BibliographyEntry>): void {
    if (!entry.title || entry.title.trim().length === 0) {
      throw new Error('Entry title is required');
    }

    if (!entry.authors || entry.authors.length === 0) {
      throw new Error('At least one author is required');
    }

    if (entry.year && (entry.year < 0 || entry.year > new Date().getFullYear() + 10)) {
      throw new Error('Invalid year');
    }
  }

  /**
   * Generate unique ID
   */
  private generateId(): string {
    return `bib-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  /**
   * Get statistics
   */
  getStatistics(): {
    totalEntries: number;
    totalCitations: number;
    byType: Record<string, number>;
    byTag: Record<string, number>;
  } {
    const byType: Record<string, number> = {};
    const byTag: Record<string, number> = {};

    this.getAllEntries().forEach(entry => {
      byType[entry.type] = (byType[entry.type] || 0) + 1;
      entry.tags?.forEach(tag => {
        byTag[tag] = (byTag[tag] || 0) + 1;
      });
    });

    return {
      totalEntries: this.entries.size,
      totalCitations: this.citations.length,
      byType,
      byTag
    };
  }
}

export const bibliographyManager = BibliographyManager.getInstance();

/**
 * Bibliography styles for CSS injection
 */
export const BIBLIOGRAPHY_STYLES = `
.bibliography {
  background: #f9fafb;
  padding: 20px;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  margin: 20px 0;
}

.bibliography-title {
  margin: 0 0 15px 0;
  font-size: 18px;
  font-weight: 600;
  color: #111827;
}

.bibliography-list {
  list-style: none;
  padding: 0;
  margin: 0;
  counter-reset: bib-item;
}

.bibliography-item {
  margin: 10px 0;
  padding-left: 30px;
  position: relative;
  color: #374151;
  font-size: 14px;
  line-height: 1.6;
}

.bibliography-item::before {
  counter-increment: bib-item;
  content: counter(bib-item) ".";
  position: absolute;
  left: 0;
  font-weight: 600;
  color: #6b7280;
}

.bibliography-empty {
  color: #9ca3af;
  font-style: italic;
  padding: 10px;
}

.editor-container.dark .bibliography {
  background: #1f2937;
  border-color: #374151;
}

.editor-container.dark .bibliography-title {
  color: #f9fafb;
}

.editor-container.dark .bibliography-item {
  color: #d1d5db;
}

.editor-container.dark .bibliography-item::before {
  color: #9ca3af;
}
`;
