/**
 * Cross-References Manager
 * Aerospace-grade implementation with comprehensive reference tracking and validation
 */

export type ReferenceType =
  | 'heading'
  | 'bookmark'
  | 'footnote'
  | 'endnote'
  | 'figure'
  | 'table'
  | 'equation'
  | 'section'
  | 'page';

export interface CrossReference {
  id: string;
  type: ReferenceType;
  targetId: string;
  label: string;
  format: 'page' | 'number' | 'text' | 'above' | 'below';
  position: number;
  createdAt: number;
  updatedAt: number;
}

export interface ReferenceTarget {
  id: string;
  type: ReferenceType;
  label: string;
  position: number;
  number?: number;
  caption?: string;
}

class CrossReferencesManager {
  private static instance: CrossReferencesManager;
  private references: CrossReference[] = [];
  private targets: Map<string, ReferenceTarget> = new Map();

  private constructor() {}

  static getInstance(): CrossReferencesManager {
    if (!CrossReferencesManager.instance) {
      CrossReferencesManager.instance = new CrossReferencesManager();
    }
    return CrossReferencesManager.instance;
  }

  /**
   * Register a reference target
   * @param target - Reference target
   * @returns Target ID
   */
  registerTarget(target: Omit<ReferenceTarget, 'id'>): string {
    const id = this.generateId();
    const fullTarget: ReferenceTarget = {
      ...target,
      id
    };

    this.targets.set(id, fullTarget);
    return id;
  }

  /**
   * Unregister a reference target
   * @param targetId - Target ID
   */
  unregisterTarget(targetId: string): void {
    this.targets.delete(targetId);
    // Remove references to this target
    this.references = this.references.filter(r => r.targetId !== targetId);
  }

  /**
   * Get a reference target
   * @param targetId - Target ID
   * @returns Target or null
   */
  getTarget(targetId: string): ReferenceTarget | null {
    return this.targets.get(targetId) || null;
  }

  /**
   * Get all reference targets
   * @returns Array of targets
   */
  getAllTargets(): ReferenceTarget[] {
    return Array.from(this.targets.values());
  }

  /**
   * Get targets by type
   * @param type - Target type
   * @returns Array of targets
   */
  getTargetsByType(type: ReferenceType): ReferenceTarget[] {
    return this.getAllTargets().filter(t => t.type === type);
  }

  /**
   * Add a cross-reference
   * @param reference - Cross-reference
   * @returns Reference ID
   */
  addReference(reference: Omit<CrossReference, 'id' | 'createdAt' | 'updatedAt'>): string {
    if (!this.targets.has(reference.targetId)) {
      throw new Error(`Target with id ${reference.targetId} not found`);
    }

    const id = this.generateId();
    const now = Date.now();

    const fullReference: CrossReference = {
      ...reference,
      id,
      createdAt: now,
      updatedAt: now
    };

    this.references.push(fullReference);
    return id;
  }

  /**
   * Update a cross-reference
   * @param id - Reference ID
   * @param updates - Partial updates
   */
  updateReference(id: string, updates: Partial<CrossReference>): void {
    const reference = this.references.find(r => r.id === id);
    if (!reference) {
      throw new Error(`Reference with id ${id} not found`);
    }

    if (updates.targetId && !this.targets.has(updates.targetId)) {
      throw new Error(`Target with id ${updates.targetId} not found`);
    }

    Object.assign(reference, updates, { updatedAt: Date.now() });
  }

  /**
   * Remove a cross-reference
   * @param id - Reference ID
   */
  removeReference(id: string): void {
    const index = this.references.findIndex(r => r.id === id);
    if (index !== -1) {
      this.references.splice(index, 1);
    }
  }

  /**
   * Get a cross-reference
   * @param id - Reference ID
   * @returns Reference or null
   */
  getReference(id: string): CrossReference | null {
    return this.references.find(r => r.id === id) || null;
  }

  /**
   * Get all cross-references
   * @returns Array of references
   */
  getReferences(): CrossReference[] {
    return [...this.references];
  }

  /**
   * Get references for a target
   * @param targetId - Target ID
   * @returns Array of references
   */
  getReferencesForTarget(targetId: string): CrossReference[] {
    return this.references.filter(r => r.targetId === targetId);
  }

  /**
   * Generate HTML for a cross-reference
   * @param id - Reference ID
   * @returns HTML string
   */
  generateReferenceHTML(id: string): string {
    const reference = this.getReference(id);
    if (!reference) {
      return '<span class="cross-reference broken">[Broken Reference]</span>';
    }

    const target = this.getTarget(reference.targetId);
    if (!target) {
      return '<span class="cross-reference broken">[Broken Reference]</span>';
    }

    const text = this.generateReferenceText(reference, target);
    return `<a href="#${target.id}" class="cross-reference" data-type="${reference.type}">${text}</a>`;
  }

  /**
   * Generate reference text based on format
   */
  private generateReferenceText(reference: CrossReference, target: ReferenceTarget): string {
    switch (reference.format) {
      case 'page':
        return `Page ${this.estimatePageNumber(target.position)}`;
      case 'number':
        return target.number ? target.number.toString() : target.label;
      case 'text':
        return target.caption || target.label;
      case 'above':
        return 'see above';
      case 'below':
        return 'see below';
      default:
        return target.label;
    }
  }

  /**
   * Estimate page number (simplified - in production would use actual page layout)
   */
  private estimatePageNumber(position: number): number {
    // Assume 2500 characters per page
    return Math.floor(position / 2500) + 1;
  }

  /**
   * Update all references after document changes
   * @param positionChanges - Map of old positions to new positions
   */
  updateReferencesAfterChanges(positionChanges: Map<number, number>): void {
    // Update target positions
    this.targets.forEach(target => {
      const newPosition = positionChanges.get(target.position);
      if (newPosition !== undefined) {
        target.position = newPosition;
      }
    });

    // Update reference positions
    this.references.forEach(reference => {
      const newPosition = positionChanges.get(reference.position);
      if (newPosition !== undefined) {
        reference.position = newPosition;
      }
    });
  }

  /**
   * Validate all references
   * @returns Array of broken reference IDs
   */
  validateReferences(): string[] {
    const brokenIds: string[] = [];

    this.references.forEach(reference => {
      if (!this.targets.has(reference.targetId)) {
        brokenIds.push(reference.id);
      }
    });

    return brokenIds;
  }

  /**
   * Clear all references and targets
   */
  clearAll(): void {
    this.references = [];
    this.targets.clear();
  }

  /**
   * Get statistics
   */
  getStatistics(): {
    totalReferences: number;
    totalTargets: number;
    byType: Record<string, number>;
    brokenReferences: number;
  } {
    const byType: Record<string, number> = {};

    this.references.forEach(r => {
      byType[r.type] = (byType[r.type] || 0) + 1;
    });

    return {
      totalReferences: this.references.length,
      totalTargets: this.targets.size,
      byType,
      brokenReferences: this.validateReferences().length
    };
  }

  /**
   * Generate unique ID
   */
  private generateId(): string {
    return `ref-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  /**
   * Export to JSON
   * @returns JSON string
   */
  exportToJSON(): string {
    const data = {
      references: this.references,
      targets: Array.from(this.targets.values()),
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
      if (data.references && Array.isArray(data.references)) {
        this.references = data.references;
      }
      if (data.targets && Array.isArray(data.targets)) {
        this.targets = new Map(data.targets.map((t: ReferenceTarget) => [t.id, t]));
      }
    } catch (error) {
      throw new Error('Failed to import cross-references: Invalid JSON format');
    }
  }
}

export const crossReferencesManager = CrossReferencesManager.getInstance();

/**
 * Cross-reference styles for CSS injection
 */
export const CROSS_REFERENCE_STYLES = `
.cross-reference {
  color: #3b82f6;
  text-decoration: none;
  font-weight: 500;
  border-bottom: 1px dotted #3b82f6;
  transition: all 0.2s;
}

.cross-reference:hover {
  color: #2563eb;
  border-bottom-style: solid;
  text-decoration: none;
}

.cross-reference.broken {
  color: #ef4444;
  border-bottom-color: #ef4444;
  font-style: italic;
}

.cross-reference.broken:hover {
  color: #dc2626;
}

.editor-container.dark .cross-reference {
  color: #60a5fa;
  border-bottom-color: #60a5fa;
}

.editor-container.dark .cross-reference:hover {
  color: #3b82f6;
}

.editor-container.dark .cross-reference.broken {
  color: #f87171;
  border-bottom-color: #f87171;
}

.editor-container.dark .cross-reference.broken:hover {
  color: #ef4444;
}

@media print {
  .cross-reference {
    color: #000;
    border-bottom: none;
    font-weight: 600;
  }

  .cross-reference::after {
    content: " (p. " attr(data-page) ")";
    font-size: 0.8em;
    font-weight: normal;
  }
}
`;
