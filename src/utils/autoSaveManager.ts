/**
 * Auto Save Manager Utility
 * Manages automatic document saving functionality
 */

import { pathManager } from './pathManager';

interface AutoSaveConfig {
  enabled: boolean;
  interval: number; // in milliseconds
  maxAutoSaves: number;
}

class AutoSaveManager {
  private config: AutoSaveConfig;
  private autoSaveIntervalId: number | null = null;
  private lastSaveTime: number = 0;

  constructor() {
    this.config = {
      enabled: true,
      interval: 30 * 1000, // 30 seconds
      maxAutoSaves: 5
    };
  }

  /**
   * Auto save the document
   */
  async autoSave(content: string, filename: string): Promise<string> {
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    const autoSavePath = pathManager.getSystemPath('autosave');
    const autoSaveFilename = `${filename}_autosave_${timestamp}.json`;
    const fullPath = pathManager.getFullPath(autoSavePath, autoSaveFilename);

    // In a real implementation, this would use Tauri's file system API
    // For now, we'll store in localStorage as a fallback
    const autoSaveData = {
      content,
      timestamp: new Date().toISOString(),
      filename
    };

    try {
      localStorage.setItem(`autosave_${filename}`, JSON.stringify(autoSaveData));
      this.lastSaveTime = Date.now();
      console.log(`Auto saved: ${filename} at ${new Date().toISOString()}`);
      return autoSaveFilename;
    } catch (error) {
      console.error('Failed to auto save:', error);
      throw error;
    }
  }

  /**
   * Restore from auto save
   */
  async restoreAutoSave(filename: string): Promise<string | null> {
    try {
      const autoSaveData = localStorage.getItem(`autosave_${filename}`);
      if (autoSaveData) {
        const parsed = JSON.parse(autoSaveData);
        return parsed.content;
      }
      return null;
    } catch (error) {
      console.error('Failed to restore auto save:', error);
      return null;
    }
  }

  /**
   * Check if there's an auto save available
   */
  hasAutoSave(filename: string): boolean {
    return localStorage.getItem(`autosave_${filename}`) !== null;
  }

  /**
   * Get last auto save time
   */
  getLastSaveTime(): number {
    return this.lastSaveTime;
  }

  /**
   * Enable auto save with interval
   */
  enableAutoSave(callback: () => void): void {
    if (this.autoSaveIntervalId) {
      this.disableAutoSave();
    }

    if (this.config.enabled) {
      this.autoSaveIntervalId = window.setInterval(() => {
        callback();
      }, this.config.interval);
      console.log(`Auto save enabled with interval: ${this.config.interval}ms`);
    }
  }

  /**
   * Disable auto save
   */
  disableAutoSave(): void {
    if (this.autoSaveIntervalId) {
      clearInterval(this.autoSaveIntervalId);
      this.autoSaveIntervalId = null;
      console.log('Auto save disabled');
    }
  }

  /**
   * Clear auto save for a specific file
   */
  clearAutoSave(filename: string): void {
    localStorage.removeItem(`autosave_${filename}`);
  }

  /**
   * Clear all auto saves
   */
  clearAllAutoSaves(): void {
    const keys: string[] = [];
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key && key.startsWith('autosave_')) {
        keys.push(key);
      }
    }
    keys.forEach(key => localStorage.removeItem(key));
    console.log(`Cleared ${keys.length} auto saves`);
  }

  /**
   * Update auto save configuration
   */
  updateConfig(config: Partial<AutoSaveConfig>): void {
    this.config = { ...this.config, ...config };
    
    // Restart auto save if interval changed
    if (config.interval && this.autoSaveIntervalId) {
      // The caller should restart auto save with new config
      this.disableAutoSave();
    }
  }

  /**
   * Get auto save configuration
   */
  getConfig(): AutoSaveConfig {
    return { ...this.config };
  }

  /**
   * Check if auto save is enabled
   */
  isEnabled(): boolean {
    return this.config.enabled && this.autoSaveIntervalId !== null;
  }
}

export const autoSaveManager = new AutoSaveManager();
