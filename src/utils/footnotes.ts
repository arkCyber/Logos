/**
 * Footnotes and Endnotes Management
 * Aerospace-grade implementation with comprehensive note tracking and validation
 */

export type NoteType = 'footnote' | 'endnote';

export interface Footnote {
  id: string;
  type: NoteType;
  number: number;
  content: string;
  position: number;
  createdAt: number;
  updatedAt: number;
}

class FootnoteManager {
  private static instance: FootnoteManager;
  private footnotes: Footnote[] = [];
  private endnotes: Footnote[] = [];
  private nextFootnoteNumber: number = 1;
  private nextEndnoteNumber: number = 1;

  private constructor() {}

  static getInstance(): FootnoteManager {
    if (!FootnoteManager.instance) {
      FootnoteManager.instance = new FootnoteManager();
    }
    return FootnoteManager.instance;
  }

  /**
   * Add a footnote
   * @param content - Footnote content
   * @param position - Position in document
   * @returns Footnote ID
   */
  addFootnote(content: string, position: number): string {
    if (!content || content.trim().length === 0) {
      throw new Error('Footnote content cannot be empty');
    }

    const footnote: Footnote = {
      id: this.generateId(),
      type: 'footnote',
      number: this.nextFootnoteNumber++,
      content: content.trim(),
      position,
      createdAt: Date.now(),
      updatedAt: Date.now()
    };

    this.footnotes.push(footnote);
    return footnote.id;
  }

  /**
   * Add an endnote
   * @param content - Endnote content
   * @param position - Position in document
   * @returns Endnote ID
   */
  addEndnote(content: string, position: number): string {
    if (!content || content.trim().length === 0) {
      throw new Error('Endnote content cannot be empty');
    }

    const endnote: Footnote = {
      id: this.generateId(),
      type: 'endnote',
      number: this.nextEndnoteNumber++,
      content: content.trim(),
      position,
      createdAt: Date.now(),
      updatedAt: Date.now()
    };

    this.endnotes.push(endnote);
    return endnote.id;
  }

  /**
   * Update a footnote or endnote
   * @param id - Note ID
   * @param content - New content
   */
  updateNote(id: string, content: string): void {
    if (!content || content.trim().length === 0) {
      throw new Error('Note content cannot be empty');
    }

    const footnote = this.footnotes.find(f => f.id === id);
    if (footnote) {
      footnote.content = content.trim();
      footnote.updatedAt = Date.now();
      return;
    }

    const endnote = this.endnotes.find(e => e.id === id);
    if (endnote) {
      endnote.content = content.trim();
      endnote.updatedAt = Date.now();
      return;
    }

    throw new Error(`Note with id ${id} not found`);
  }

  /**
   * Delete a footnote or endnote
   * @param id - Note ID
   */
  deleteNote(id: string): void {
    const footnoteIndex = this.footnotes.findIndex(f => f.id === id);
    if (footnoteIndex !== -1) {
      this.footnotes.splice(footnoteIndex, 1);
      this.renumberFootnotes();
      return;
    }

    const endnoteIndex = this.endnotes.findIndex(e => e.id === id);
    if (endnoteIndex !== -1) {
      this.endnotes.splice(endnoteIndex, 1);
      this.renumberEndnotes();
      return;
    }

    throw new Error(`Note with id ${id} not found`);
  }

  /**
   * Get a note by ID
   * @param id - Note ID
   * @returns Note or null
   */
  getNote(id: string): Footnote | null {
    return this.footnotes.find(f => f.id === id) || this.endnotes.find(e => e.id === id) || null;
  }

  /**
   * Get all footnotes
   * @returns Array of footnotes
   */
  getFootnotes(): Footnote[] {
    return [...this.footnotes];
  }

  /**
   * Get all endnotes
   * @returns Array of endnotes
   */
  getEndnotes(): Footnote[] {
    return [...this.endnotes];
  }

  /**
   * Get all notes (footnotes and endnotes)
   * @returns Array of all notes
   */
  getAllNotes(): Footnote[] {
    return [...this.footnotes, ...this.endnotes];
  }

  /**
   * Get footnotes by position range
   * @param start - Start position
   * @param end - End position
   * @returns Array of footnotes
   */
  getFootnotesByRange(start: number, end: number): Footnote[] {
    return this.footnotes.filter(f => f.position >= start && f.position <= end);
  }

  /**
   * Generate HTML for footnotes
   * @returns HTML string
   */
  generateFootnotesHTML(): string {
    if (this.footnotes.length === 0) {
      return '';
    }

    let html = '<div class="footnotes-section">';
    html += '<h4 class="footnotes-title">Footnotes</h4>';
    html += '<ol class="footnotes-list">';

    this.footnotes.forEach(footnote => {
      html += `<li class="footnote-item" id="footnote-${footnote.id}">`;
      html += `<a href="#footnote-ref-${footnote.id}" class="footnote-backlink">↩</a> `;
      html += `<span class="footnote-content">${footnote.content}</span>`;
      html += '</li>';
    });

    html += '</ol></div>';
    return html;
  }

  /**
   * Generate HTML for endnotes
   * @returns HTML string
   */
  generateEndnotesHTML(): string {
    if (this.endnotes.length === 0) {
      return '';
    }

    let html = '<div class="endnotes-section">';
    html += '<h4 class="endnotes-title">Endnotes</h4>';
    html += '<ol class="endnotes-list">';

    this.endnotes.forEach(endnote => {
      html += `<li class="endnote-item" id="endnote-${endnote.id}">`;
      html += `<a href="#endnote-ref-${endnote.id}" class="endnote-backlink">↩</a> `;
      html += `<span class="endnote-content">${endnote.content}</span>`;
      html += '</li>';
    });

    html += '</ol></div>';
    return html;
  }

  /**
   * Generate footnote reference marker
   * @param id - Footnote ID
   * @returns HTML string
   */
  generateFootnoteReference(id: string): string {
    const footnote = this.footnotes.find(f => f.id === id);
    if (!footnote) {
      return '';
    }

    return (
      `<sup class="footnote-ref" id="footnote-ref-${id}">` +
      `<a href="#footnote-${id}" class="footnote-link">${footnote.number}</a>` +
      '</sup>'
    );
  }

  /**
   * Generate endnote reference marker
   * @param id - Endnote ID
   * @returns HTML string
   */
  generateEndnoteReference(id: string): string {
    const endnote = this.endnotes.find(e => e.id === id);
    if (!endnote) {
      return '';
    }

    return (
      `<sup class="endnote-ref" id="endnote-ref-${id}">` +
      `<a href="#endnote-${id}" class="endnote-link">${endnote.number}</a>` +
      '</sup>'
    );
  }

  /**
   * Renumber footnotes after deletion
   */
  private renumberFootnotes(): void {
    this.footnotes.forEach((footnote, index) => {
      footnote.number = index + 1;
    });
    this.nextFootnoteNumber = this.footnotes.length + 1;
  }

  /**
   * Renumber endnotes after deletion
   */
  private renumberEndnotes(): void {
    this.endnotes.forEach((endnote, index) => {
      endnote.number = index + 1;
    });
    this.nextEndnoteNumber = this.endnotes.length + 1;
  }

  /**
   * Clear all footnotes
   */
  clearFootnotes(): void {
    this.footnotes = [];
    this.nextFootnoteNumber = 1;
  }

  /**
   * Clear all endnotes
   */
  clearEndnotes(): void {
    this.endnotes = [];
    this.nextEndnoteNumber = 1;
  }

  /**
   * Clear all notes
   */
  clearAll(): void {
    this.clearFootnotes();
    this.clearEndnotes();
  }

  /**
   * Get statistics
   */
  getStatistics(): {
    totalFootnotes: number;
    totalEndnotes: number;
    totalNotes: number;
  } {
    return {
      totalFootnotes: this.footnotes.length,
      totalEndnotes: this.endnotes.length,
      totalNotes: this.footnotes.length + this.endnotes.length
    };
  }

  /**
   * Generate unique ID
   */
  private generateId(): string {
    return `note-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  /**
   * Export notes to JSON
   * @returns JSON string
   */
  exportToJSON(): string {
    const data = {
      footnotes: this.footnotes,
      endnotes: this.endnotes,
      exportedAt: Date.now()
    };
    return JSON.stringify(data, null, 2);
  }

  /**
   * Import notes from JSON
   * @param json - JSON string
   */
  importFromJSON(json: string): void {
    try {
      const data = JSON.parse(json);
      if (data.footnotes && Array.isArray(data.footnotes)) {
        this.footnotes = data.footnotes;
        this.nextFootnoteNumber = this.footnotes.length + 1;
      }
      if (data.endnotes && Array.isArray(data.endnotes)) {
        this.endnotes = data.endnotes;
        this.nextEndnoteNumber = this.endnotes.length + 1;
      }
    } catch (error) {
      throw new Error('Failed to import notes: Invalid JSON format');
    }
  }
}

export const footnoteManager = FootnoteManager.getInstance();

/**
 * Footnote styles for CSS injection
 */
export const FOOTNOTE_STYLES = `
.footnotes-section,
.endnotes-section {
  margin-top: 40px;
  padding-top: 20px;
  border-top: 1px solid #e5e7eb;
}

.footnotes-title,
.endnotes-title {
  font-size: 16px;
  font-weight: 600;
  color: #111827;
  margin: 0 0 15px 0;
}

.footnotes-list,
.endnotes-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.footnote-item,
.endnote-item {
  margin: 10px 0;
  padding-left: 20px;
  position: relative;
  font-size: 12px;
  line-height: 1.6;
  color: #374151;
}

.footnote-item::before,
.endnote-item::before {
  content: attr(data-number);
  position: absolute;
  left: 0;
  font-weight: 600;
  color: #6b7280;
}

.footnote-ref,
.endnote-ref {
  font-size: 10px;
  vertical-align: super;
  line-height: 0;
}

.footnote-link,
.endnote-link {
  color: #3b82f6;
  text-decoration: none;
  font-weight: 600;
}

.footnote-link:hover,
.endnote-link:hover {
  text-decoration: underline;
}

.footnote-backlink,
.endnote-backlink {
  color: #3b82f6;
  text-decoration: none;
  font-weight: 600;
  margin-right: 5px;
}

.footnote-backlink:hover,
.endnote-backlink:hover {
  text-decoration: underline;
}

.footnote-content,
.endnote-content {
  flex: 1;
}

.editor-container.dark .footnotes-section,
.editor-container.dark .endnotes-section {
  border-top-color: #374151;
}

.editor-container.dark .footnotes-title,
.editor-container.dark .endnotes-title {
  color: #f9fafb;
}

.editor-container.dark .footnote-item,
.editor-container.dark .endnote-item {
  color: #d1d5db;
}

.editor-container.dark .footnote-item::before,
.editor-container.dark .endnote-item::before {
  color: #9ca3af;
}

.editor-container.dark .footnote-link,
.editor-container.dark .endnote-link,
.editor-container.dark .footnote-backlink,
.editor-container.dark .endnote-backlink {
  color: #60a5fa;
}
`;
