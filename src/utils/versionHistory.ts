/**
 * Document Version History Manager
 * Aerospace-grade implementation with comprehensive version tracking and validation
 */

export interface DocumentVersion {
  id: string;
  version: number;
  timestamp: number;
  author: string;
  content: string;
  description: string;
  tags: string[];
  size: number;
  checksum: string;
  isAutoSave: boolean;
  parentVersionId?: string;
}

export interface VersionDiff {
  versionId: string;
  changes: {
    additions: number;
    deletions: number;
    modifications: number;
  };
  summary: string;
}

class VersionHistoryManager {
  private static instance: VersionHistoryManager;
  private versions: DocumentVersion[] = [];
  private currentVersion: number = 0;
  private maxVersions: number = 100;
  private autoSaveInterval: number = 5 * 60 * 1000; // 5 minutes
  private autoSaveTimer: ReturnType<typeof setInterval> | null = null;
  private currentAuthor: string = 'Anonymous';
  private lastAutoSaveChecksum: string = '';
  private autoSaveEnabled: boolean = true;

  private constructor() {}

  static getInstance(): VersionHistoryManager {
    if (!VersionHistoryManager.instance) {
      VersionHistoryManager.instance = new VersionHistoryManager();
    }
    return VersionHistoryManager.instance;
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
   * Create a new version
   * @param content - Document content
   * @param description - Version description
   * @param isAutoSave - Whether this is an auto-save
   * @param tags - Version tags
   * @returns Version ID
   */
  createVersion(
    content: string,
    description: string = '',
    isAutoSave: boolean = false,
    tags: string[] = []
  ): string {
    if (!content) {
      throw new Error('Content cannot be empty');
    }

    this.currentVersion++;
    const checksum = this.calculateChecksum(content);

    const version: DocumentVersion = {
      id: this.generateId(),
      version: this.currentVersion,
      timestamp: Date.now(),
      author: this.currentAuthor,
      content,
      description: description || (isAutoSave ? 'Auto-save' : `Version ${this.currentVersion}`),
      tags,
      size: content.length,
      checksum,
      isAutoSave,
      parentVersionId:
        this.versions.length > 0 ? this.versions[this.versions.length - 1].id : undefined
    };

    this.versions.push(version);

    // Enforce max versions limit
    if (this.versions.length > this.maxVersions) {
      this.versions.shift();
    }

    return version.id;
  }

  /**
   * Get a version by ID
   * @param versionId - Version ID
   * @returns Version or null
   */
  getVersion(versionId: string): DocumentVersion | null {
    return this.versions.find(v => v.id === versionId) || null;
  }

  /**
   * Get a version by version number
   * @param versionNumber - Version number
   * @returns Version or null
   */
  getVersionByNumber(versionNumber: number): DocumentVersion | null {
    return this.versions.find(v => v.version === versionNumber) || null;
  }

  /**
   * Get the latest version
   * @returns Latest version or null
   */
  getLatestVersion(): DocumentVersion | null {
    return this.versions.length > 0 ? this.versions[this.versions.length - 1] : null;
  }

  /**
   * Get all versions
   * @returns Array of versions
   */
  getAllVersions(): DocumentVersion[] {
    return [...this.versions];
  }

  /**
   * Restore a version
   * @param versionId - Version ID
   * @returns Document content
   */
  restoreVersion(versionId: string): string | null {
    const version = this.getVersion(versionId);
    if (!version) {
      return null;
    }

    // Verify checksum
    const currentChecksum = this.calculateChecksum(version.content);
    if (currentChecksum !== version.checksum) {
      console.warn('Checksum mismatch when restoring version');
    }

    return version.content;
  }

  /**
   * Delete a version
   * @param versionId - Version ID
   */
  deleteVersion(versionId: string): void {
    const index = this.versions.findIndex(v => v.id === versionId);
    if (index !== -1) {
      this.versions.splice(index, 1);
    }
  }

  /**
   * Add a tag to a version
   * @param versionId - Version ID
   * @param tag - Tag name
   */
  addTag(versionId: string, tag: string): void {
    const version = this.getVersion(versionId);
    if (version && !version.tags.includes(tag)) {
      version.tags.push(tag);
    }
  }

  /**
   * Remove a tag from a version
   * @param versionId - Version ID
   * @param tag - Tag name
   */
  removeTag(versionId: string, tag: string): void {
    const version = this.getVersion(versionId);
    if (version) {
      version.tags = version.tags.filter(t => t !== tag);
    }
  }

  /**
   * Get versions by tag
   * @param tag - Tag name
   * @returns Array of versions
   */
  getVersionsByTag(tag: string): DocumentVersion[] {
    return this.versions.filter(v => v.tags.includes(tag));
  }

  /**
   * Get versions by author
   * @param author - Author name
   * @returns Array of versions
   */
  getVersionsByAuthor(author: string): DocumentVersion[] {
    return this.versions.filter(v => v.author === author);
  }

  /**
   * Get versions in a time range
   * @param startTime - Start timestamp
   * @param endTime - End timestamp
   * @returns Array of versions
   */
  getVersionsByTimeRange(startTime: number, endTime: number): DocumentVersion[] {
    return this.versions.filter(v => v.timestamp >= startTime && v.timestamp <= endTime);
  }

  /**
   * Compare two versions
   * @param versionId1 - First version ID
   * @param versionId2 - Second version ID
   * @returns Version diff
   */
  compareVersions(versionId1: string, versionId2: string): VersionDiff | null {
    const version1 = this.getVersion(versionId1);
    const version2 = this.getVersion(versionId2);

    if (!version1 || !version2) {
      return null;
    }

    const diff = this.calculateDiff(version1.content, version2.content);

    return {
      versionId: versionId2,
      changes: diff,
      summary: this.generateDiffSummary(diff)
    };
  }

  /**
   * Calculate diff between two contents
   */
  private calculateDiff(
    content1: string,
    content2: string
  ): {
    additions: number;
    deletions: number;
    modifications: number;
  } {
    // Simplified diff calculation
    // In production, use a proper diff library like diff-match-patch
    const lines1 = content1.split('\n');
    const lines2 = content2.split('\n');

    let additions = 0;
    let deletions = 0;
    let modifications = 0;

    const maxLength = Math.max(lines1.length, lines2.length);

    for (let i = 0; i < maxLength; i++) {
      const line1 = lines1[i] || '';
      const line2 = lines2[i] || '';

      if (line1 === '' && line2 !== '') {
        additions++;
      } else if (line1 !== '' && line2 === '') {
        deletions++;
      } else if (line1 !== line2) {
        modifications++;
      }
    }

    return { additions, deletions, modifications };
  }

  /**
   * Generate diff summary
   */
  private generateDiffSummary(diff: {
    additions: number;
    deletions: number;
    modifications: number;
  }): string {
    const parts: string[] = [];

    if (diff.additions > 0) {
      parts.push(`${diff.additions} addition${diff.additions > 1 ? 's' : ''}`);
    }

    if (diff.deletions > 0) {
      parts.push(`${diff.deletions} deletion${diff.deletions > 1 ? 's' : ''}`);
    }

    if (diff.modifications > 0) {
      parts.push(`${diff.modifications} modification${diff.modifications > 1 ? 's' : ''}`);
    }

    return parts.length > 0 ? parts.join(', ') : 'No changes';
  }

  /**
   * Enable auto-save
   */
  enableAutoSave(): void {
    if (this.autoSaveTimer) {
      return;
    }

    this.autoSaveTimer = setInterval(() => {
      // Auto-save is triggered by document change events via triggerAutoSave()
      // This timer is kept for periodic cleanup or backup if needed
    }, this.autoSaveInterval);
  }

  /**
   * Trigger auto-save manually (called when document changes)
   * @param content - Current document content
   */
  triggerAutoSave(content: string): void {
    if (!this.autoSaveEnabled) {
      return;
    }

    if (!content || content.trim() === '') {
      // Skip auto-save for empty content
      return;
    }

    // Calculate checksum to detect actual changes
    const currentChecksum = this.calculateChecksum(content);

    // Skip if content hasn't changed
    if (this.lastAutoSaveChecksum === currentChecksum) {
      return;
    }

    // Create auto-save version
    try {
      const _version = this.createVersion(content, 'Auto-save', true, ['auto-save']);

      this.lastAutoSaveChecksum = currentChecksum;

      // Enforce max versions limit by removing old auto-saves
      if (this.maxVersions > 0) {
        this.enforceMaxVersions();
      }
    } catch (error) {
      console.error('Auto-save failed:', error);
    }
  }

  /**
   * Disable auto-save
   */
  disableAutoSave(): void {
    if (this.autoSaveTimer) {
      clearInterval(this.autoSaveTimer);
      this.autoSaveTimer = null;
    }
  }

  /**
   * Set auto-save interval
   * @param interval - Interval in milliseconds
   */
  setAutoSaveInterval(interval: number): void {
    if (interval < 1000) {
      throw new Error('Auto-save interval must be at least 1000ms');
    }

    this.autoSaveInterval = interval;

    // Restart timer if it's running
    if (this.autoSaveTimer) {
      this.disableAutoSave();
      this.enableAutoSave();
    }
  }

  /**
   * Calculate checksum for content
   */
  private calculateChecksum(content: string): string {
    // Simple hash function
    let hash = 0;
    for (let i = 0; i < content.length; i++) {
      const char = content.charCodeAt(i);
      hash = (hash << 5) - hash + char;
      hash = hash & hash; // Convert to 32bit integer
    }
    return hash.toString(16);
  }

  /**
   * Set maximum versions limit
   * @param max - Maximum number of versions
   */
  setMaxVersions(max: number): void {
    if (max < 1) {
      throw new Error('Max versions must be at least 1');
    }
    this.maxVersions = max;

    // Trim existing versions if necessary
    while (this.versions.length > this.maxVersions) {
      this.versions.shift();
    }
  }

  /**
   * Enforce maximum versions limit by removing old versions
   */
  private enforceMaxVersions(): void {
    while (this.versions.length > this.maxVersions) {
      // Remove oldest auto-save versions first
      const autoSaveIndex = this.versions.findIndex(v => v.isAutoSave);
      if (autoSaveIndex !== -1) {
        this.versions.splice(autoSaveIndex, 1);
      } else {
        this.versions.shift();
      }
    }
  }

  /**
   * Clear all versions
   */
  clearAll(): void {
    this.versions = [];
    this.currentVersion = 0;
  }

  /**
   * Get statistics
   */
  getStatistics(): {
    totalVersions: number;
    currentVersion: number;
    totalSize: number;
    byAuthor: Record<string, number>;
    byTag: Record<string, number>;
    autoSaveCount: number;
  } {
    const byAuthor: Record<string, number> = {};
    const byTag: Record<string, number> = {};
    let totalSize = 0;
    let autoSaveCount = 0;

    this.versions.forEach(version => {
      byAuthor[version.author] = (byAuthor[version.author] || 0) + 1;
      totalSize += version.size;

      if (version.isAutoSave) {
        autoSaveCount++;
      }

      version.tags.forEach(tag => {
        byTag[tag] = (byTag[tag] || 0) + 1;
      });
    });

    return {
      totalVersions: this.versions.length,
      currentVersion: this.currentVersion,
      totalSize,
      byAuthor,
      byTag,
      autoSaveCount
    };
  }

  /**
   * Generate unique ID
   */
  private generateId(): string {
    return `ver-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  /**
   * Export to JSON
   * @returns JSON string
   */
  exportToJSON(): string {
    const data = {
      versions: this.versions,
      currentVersion: this.currentVersion,
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
      if (data.versions && Array.isArray(data.versions)) {
        this.versions = data.versions;
      }
      if (data.currentVersion) {
        this.currentVersion = data.currentVersion;
      }
    } catch (error) {
      throw new Error('Failed to import version history: Invalid JSON format');
    }
  }
}

export const versionHistoryManager = VersionHistoryManager.getInstance();
