/**
 * Backup Manager Utility
 * Manages document backup functionality
 */

import { pathManager } from './pathManager';

interface BackupConfig {
  maxBackups: number;
  autoBackup: boolean;
  backupInterval: number; // in milliseconds
}

class BackupManager {
  private config: BackupConfig;
  private backupIntervalId: number | null = null;

  constructor() {
    this.config = {
      maxBackups: 10,
      autoBackup: true,
      backupInterval: 5 * 60 * 1000 // 5 minutes
    };
  }

  /**
   * Create a backup of the document
   */
  async createBackup(content: string, filename: string): Promise<string> {
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    const backupPath = pathManager.getSystemPath('backups');
    const backupFilename = `${filename}_${timestamp}.json`;
    const fullPath = pathManager.getFullPath(backupPath, backupFilename);

    // In a real implementation, this would use Tauri's file system API
    // For now, we'll store in localStorage as a fallback
    const backupData = {
      content,
      timestamp: new Date().toISOString(),
      filename
    };

    try {
      localStorage.setItem(`backup_${backupFilename}`, JSON.stringify(backupData));
      console.log(`Backup created: ${backupFilename}`);
      return backupFilename;
    } catch (error) {
      console.error('Failed to create backup:', error);
      throw error;
    }
  }

  /**
   * Restore a backup
   */
  async restoreBackup(backupFilename: string): Promise<string | null> {
    try {
      const backupData = localStorage.getItem(`backup_${backupFilename}`);
      if (backupData) {
        const parsed = JSON.parse(backupData);
        return parsed.content;
      }
      return null;
    } catch (error) {
      console.error('Failed to restore backup:', error);
      return null;
    }
  }

  /**
   * List all available backups
   */
  listBackups(): string[] {
    const backups: string[] = [];
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key && key.startsWith('backup_')) {
        backups.push(key.replace('backup_', ''));
      }
    }
    return backups.sort().reverse(); // Most recent first
  }

  /**
   * Delete a backup
   */
  deleteBackup(backupFilename: string): void {
    localStorage.removeItem(`backup_${backupFilename}`);
  }

  /**
   * Clean old backups (keep only maxBackups)
   */
  cleanOldBackups(): number {
    const backups = this.listBackups();
    const toDelete = backups.slice(this.config.maxBackups);
    
    toDelete.forEach(backup => {
      this.deleteBackup(backup);
    });

    return toDelete.length;
  }

  /**
   * Enable auto backup
   */
  enableAutoBackup(callback: () => void): void {
    if (this.backupIntervalId) {
      this.disableAutoBackup();
    }

    this.backupIntervalId = window.setInterval(() => {
      if (this.config.autoBackup) {
        callback();
      }
    }, this.config.backupInterval);
  }

  /**
   * Disable auto backup
   */
  disableAutoBackup(): void {
    if (this.backupIntervalId) {
      clearInterval(this.backupIntervalId);
      this.backupIntervalId = null;
    }
  }

  /**
   * Update backup configuration
   */
  updateConfig(config: Partial<BackupConfig>): void {
    this.config = { ...this.config, ...config };
  }

  /**
   * Get backup configuration
   */
  getConfig(): BackupConfig {
    return { ...this.config };
  }
}

export const backupManager = new BackupManager();
