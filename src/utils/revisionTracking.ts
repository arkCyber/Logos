/**
 * Revision Tracking System
 * Aerospace-grade implementation with comprehensive change tracking and validation
 */

export interface Revision {
  id: string;
  timestamp: number;
  author: string;
  type: 'insert' | 'delete' | 'format' | 'replace';
  position: number;
  length: number;
  content: string;
  previousContent?: string;
  accepted: boolean;
  rejected: boolean;
}

export interface RevisionSnapshot {
  id: string;
  timestamp: number;
  author: string;
  content: string;
  revisions: Revision[];
  description?: string;
}

class RevisionTrackingSystem {
  private static instance: RevisionTrackingSystem;
  private revisions: Revision[] = [];
  private snapshots: RevisionSnapshot[] = [];
  private currentAuthor: string = 'Anonymous';
  private trackingEnabled: boolean = true;
  private maxRevisions: number = 1000;
  private maxSnapshots: number = 50;

  private constructor() {}

  static getInstance(): RevisionTrackingSystem {
    if (!RevisionTrackingSystem.instance) {
      RevisionTrackingSystem.instance = new RevisionTrackingSystem();
    }
    return RevisionTrackingSystem.instance;
  }

  /**
   * Set current author
   * @param author - Author name
   */
  setAuthor(author: string): void {
    if (!author || author.trim().length === 0) {
      throw new Error('Author name cannot be empty');
    }
    this.currentAuthor = author.trim();
  }

  /**
   * Get current author
   * @returns Current author name
   */
  getAuthor(): string {
    return this.currentAuthor;
  }

  /**
   * Enable or disable revision tracking
   * @param enabled - Enable tracking
   */
  setTrackingEnabled(enabled: boolean): void {
    this.trackingEnabled = enabled;
  }

  /**
   * Check if tracking is enabled
   * @returns True if tracking is enabled
   */
  isTrackingEnabled(): boolean {
    return this.trackingEnabled;
  }

  /**
   * Record an insertion revision
   * @param position - Position of insertion
   * @param content - Inserted content
   * @returns Revision ID
   */
  recordInsert(position: number, content: string): string {
    if (!this.trackingEnabled) {
      return '';
    }

    const revision: Revision = {
      id: this.generateId(),
      timestamp: Date.now(),
      author: this.currentAuthor,
      type: 'insert',
      position,
      length: content.length,
      content,
      accepted: false,
      rejected: false
    };

    this.addRevision(revision);
    return revision.id;
  }

  /**
   * Record a deletion revision
   * @param position - Position of deletion
   * @param content - Deleted content
   * @returns Revision ID
   */
  recordDelete(position: number, content: string): string {
    if (!this.trackingEnabled) {
      return '';
    }

    const revision: Revision = {
      id: this.generateId(),
      timestamp: Date.now(),
      author: this.currentAuthor,
      type: 'delete',
      position,
      length: content.length,
      content,
      accepted: false,
      rejected: false
    };

    this.addRevision(revision);
    return revision.id;
  }

  /**
   * Record a format change revision
   * @param position - Position of format change
   * @param length - Length of formatted text
   * @param content - Formatted content
   * @param previousContent - Previous content before format change
   * @returns Revision ID
   */
  recordFormat(
    position: number,
    length: number,
    content: string,
    previousContent?: string
  ): string {
    if (!this.trackingEnabled) {
      return '';
    }

    const revision: Revision = {
      id: this.generateId(),
      timestamp: Date.now(),
      author: this.currentAuthor,
      type: 'format',
      position,
      length,
      content,
      previousContent,
      accepted: false,
      rejected: false
    };

    this.addRevision(revision);
    return revision.id;
  }

  /**
   * Record a replace revision
   * @param position - Position of replacement
   * @param length - Length of replaced text
   * @param content - New content
   * @param previousContent - Previous content
   * @returns Revision ID
   */
  recordReplace(
    position: number,
    length: number,
    content: string,
    previousContent: string
  ): string {
    if (!this.trackingEnabled) {
      return '';
    }

    const revision: Revision = {
      id: this.generateId(),
      timestamp: Date.now(),
      author: this.currentAuthor,
      type: 'replace',
      position,
      length,
      content,
      previousContent,
      accepted: false,
      rejected: false
    };

    this.addRevision(revision);
    return revision.id;
  }

  /**
   * Add revision to tracking system
   */
  private addRevision(revision: Revision): void {
    this.revisions.push(revision);

    // Enforce max revisions limit
    if (this.revisions.length > this.maxRevisions) {
      this.revisions.shift();
    }
  }

  /**
   * Accept a revision
   * @param revisionId - Revision ID
   */
  acceptRevision(revisionId: string): void {
    const revision = this.findRevision(revisionId);
    if (revision) {
      revision.accepted = true;
      revision.rejected = false;
    }
  }

  /**
   * Reject a revision
   * @param revisionId - Revision ID
   */
  rejectRevision(revisionId: string): void {
    const revision = this.findRevision(revisionId);
    if (revision) {
      revision.rejected = true;
      revision.accepted = false;
    }
  }

  /**
   * Find revision by ID
   * @param revisionId - Revision ID
   * @returns Revision or null
   */
  findRevision(revisionId: string): Revision | null {
    return this.revisions.find(r => r.id === revisionId) || null;
  }

  /**
   * Get all revisions
   * @returns Array of revisions
   */
  getRevisions(): Revision[] {
    return [...this.revisions];
  }

  /**
   * Get pending revisions (not accepted or rejected)
   * @returns Array of pending revisions
   */
  getPendingRevisions(): Revision[] {
    return this.revisions.filter(r => !r.accepted && !r.rejected);
  }

  /**
   * Get revisions by author
   * @param author - Author name
   * @returns Array of revisions
   */
  getRevisionsByAuthor(author: string): Revision[] {
    return this.revisions.filter(r => r.author === author);
  }

  /**
   * Get revisions by type
   * @param type - Revision type
   * @returns Array of revisions
   */
  getRevisionsByType(type: Revision['type']): Revision[] {
    return this.revisions.filter(r => r.type === type);
  }

  /**
   * Get revisions in a time range
   * @param startTime - Start timestamp
   * @param endTime - End timestamp
   * @returns Array of revisions
   */
  getRevisionsByTimeRange(startTime: number, endTime: number): Revision[] {
    return this.revisions.filter(r => r.timestamp >= startTime && r.timestamp <= endTime);
  }

  /**
   * Create a snapshot of current document state
   * @param content - Current document content
   * @param description - Snapshot description
   * @returns Snapshot ID
   */
  createSnapshot(content: string, description?: string): string {
    const snapshot: RevisionSnapshot = {
      id: this.generateId(),
      timestamp: Date.now(),
      author: this.currentAuthor,
      content,
      revisions: [...this.revisions],
      description
    };

    this.snapshots.push(snapshot);

    // Enforce max snapshots limit
    if (this.snapshots.length > this.maxSnapshots) {
      this.snapshots.shift();
    }

    return snapshot.id;
  }

  /**
   * Get snapshot by ID
   * @param snapshotId - Snapshot ID
   * @returns Snapshot or null
   */
  getSnapshot(snapshotId: string): RevisionSnapshot | null {
    return this.snapshots.find(s => s.id === snapshotId) || null;
  }

  /**
   * Get all snapshots
   * @returns Array of snapshots
   */
  getSnapshots(): RevisionSnapshot[] {
    return [...this.snapshots];
  }

  /**
   * Restore document from snapshot
   * @param snapshotId - Snapshot ID
   * @returns Document content
   */
  restoreSnapshot(snapshotId: string): string | null {
    const snapshot = this.getSnapshot(snapshotId);
    if (snapshot) {
      this.revisions = [...snapshot.revisions];
      return snapshot.content;
    }
    return null;
  }

  /**
   * Delete a snapshot
   * @param snapshotId - Snapshot ID
   */
  deleteSnapshot(snapshotId: string): void {
    const index = this.snapshots.findIndex(s => s.id === snapshotId);
    if (index !== -1) {
      this.snapshots.splice(index, 1);
    }
  }

  /**
   * Clear all revisions
   */
  clearRevisions(): void {
    this.revisions = [];
  }

  /**
   * Clear all snapshots
   */
  clearSnapshots(): void {
    this.snapshots = [];
  }

  /**
   * Clear all tracking data
   */
  clearAll(): void {
    this.revisions = [];
    this.snapshots = [];
  }

  /**
   * Get revision statistics
   * @returns Statistics object
   */
  getStatistics(): {
    totalRevisions: number;
    pendingRevisions: number;
    acceptedRevisions: number;
    rejectedRevisions: number;
    byType: Record<string, number>;
    byAuthor: Record<string, number>;
  } {
    const byType: Record<string, number> = {};
    const byAuthor: Record<string, number> = {};

    this.revisions.forEach(r => {
      byType[r.type] = (byType[r.type] || 0) + 1;
      byAuthor[r.author] = (byAuthor[r.author] || 0) + 1;
    });

    return {
      totalRevisions: this.revisions.length,
      pendingRevisions: this.getPendingRevisions().length,
      acceptedRevisions: this.revisions.filter(r => r.accepted).length,
      rejectedRevisions: this.revisions.filter(r => r.rejected).length,
      byType,
      byAuthor
    };
  }

  /**
   * Export revisions to JSON
   * @returns JSON string
   */
  exportToJSON(): string {
    const data = {
      revisions: this.revisions,
      snapshots: this.snapshots,
      exportedAt: Date.now()
    };
    return JSON.stringify(data, null, 2);
  }

  /**
   * Import revisions from JSON
   * @param json - JSON string
   */
  importFromJSON(json: string): void {
    try {
      const data = JSON.parse(json);
      if (data.revisions && Array.isArray(data.revisions)) {
        this.revisions = data.revisions;
      }
      if (data.snapshots && Array.isArray(data.snapshots)) {
        this.snapshots = data.snapshots;
      }
    } catch (error) {
      throw new Error('Failed to import revisions: Invalid JSON format');
    }
  }

  /**
   * Generate unique ID
   */
  private generateId(): string {
    return `rev-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  /**
   * Set maximum revisions limit
   * @param max - Maximum number of revisions
   */
  setMaxRevisions(max: number): void {
    if (max < 1) {
      throw new Error('Max revisions must be at least 1');
    }
    this.maxRevisions = max;

    // Trim existing revisions if necessary
    while (this.revisions.length > this.maxRevisions) {
      this.revisions.shift();
    }
  }

  /**
   * Set maximum snapshots limit
   * @param max - Maximum number of snapshots
   */
  setMaxSnapshots(max: number): void {
    if (max < 1) {
      throw new Error('Max snapshots must be at least 1');
    }
    this.maxSnapshots = max;

    // Trim existing snapshots if necessary
    while (this.snapshots.length > this.maxSnapshots) {
      this.snapshots.shift();
    }
  }
}

export const revisionTracking = RevisionTrackingSystem.getInstance();

/**
 * Revision display styles for CSS injection
 */
export const REVISION_STYLES = `
.revision-highlight-insert {
  background-color: #d1fae5;
  text-decoration: none;
  border-bottom: 2px solid #10b981;
}

.revision-highlight-delete {
  background-color: #fee2e2;
  text-decoration: line-through;
  color: #ef4444;
}

.revision-highlight-format {
  background-color: #dbeafe;
  border-bottom: 2px solid #3b82f6;
}

.revision-highlight-replace {
  background-color: #fef3c7;
  border-bottom: 2px solid #f59e0b;
}

.revision-accepted {
  background-color: transparent !important;
  text-decoration: none !important;
  border-bottom: none !important;
}

.revision-rejected {
  display: none;
}

.revision-tooltip {
  position: absolute;
  background: #1f2937;
  color: #f9fafb;
  padding: 8px 12px;
  border-radius: 4px;
  font-size: 12px;
  z-index: 1000;
  pointer-events: none;
  white-space: nowrap;
}

.revision-tooltip::after {
  content: '';
  position: absolute;
  top: 100%;
  left: 50%;
  margin-left: -5px;
  border-width: 5px;
  border-style: solid;
  border-color: #1f2937 transparent transparent transparent;
}

.editor-container.dark .revision-highlight-insert {
  background-color: #065f46;
  border-bottom-color: #34d399;
}

.editor-container.dark .revision-highlight-delete {
  background-color: #7f1d1d;
  color: #fca5a5;
}

.editor-container.dark .revision-highlight-format {
  background-color: #1e3a8a;
  border-bottom-color: #60a5fa;
}

.editor-container.dark .revision-highlight-replace {
  background-color: #78350f;
  border-bottom-color: #fbbf24;
}
`;
