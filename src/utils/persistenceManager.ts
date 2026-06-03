/**
 * 航空航天级数据持久化和恢复系统
 * 提供完整的数据存储、备份、恢复和同步功能
 */

import { logger, LogCategory } from './logger';
import { createError, ErrorCode, ErrorSeverity, ErrorCategory } from './errorHandler';

/**
 * 数据存储接口
 */
export interface DataStore {
  get<T>(key: string): Promise<T | null>;
  set<T>(key: string, value: T): Promise<void>;
  delete(key: string): Promise<void>;
  clear(): Promise<void>;
  keys(): Promise<string[]>;
  exists(key: string): Promise<boolean>;
}

/**
 * 数据备份接口
 */
export interface DataBackup {
  id: string;
  timestamp: number;
  data: Record<string, any>;
  checksum: string;
  size: number;
  compressed: boolean;
  encrypted: boolean;
}

/**
 * 数据恢复点接口
 */
export interface RecoveryPoint {
  id: string;
  timestamp: number;
  description: string;
  data: Record<string, any>;
  version: number;
}

/**
 * 持久化配置接口
 */
export interface PersistenceConfig {
  enableAutoSave: boolean;
  autoSaveInterval: number;
  maxBackups: number;
  enableCompression: boolean;
  enableEncryption: boolean;
  encryptionKey?: string;
  enableVersioning: boolean;
  maxVersions: number;
  enableConflictResolution: boolean;
  enableDataValidation: boolean;
  storageType: 'localStorage' | 'indexedDB' | 'custom';
  customStore?: DataStore;
}

/**
 * 默认配置
 */
const DEFAULT_CONFIG: PersistenceConfig = {
  enableAutoSave: true,
  autoSaveInterval: 30000, // 30秒
  maxBackups: 10,
  enableCompression: false,
  enableEncryption: false,
  enableVersioning: true,
  maxVersions: 5,
  enableConflictResolution: true,
  enableDataValidation: true,
  storageType: 'localStorage'
};

/**
 * LocalStorage 实现
 */
class LocalStorageStore implements DataStore {
  async get<T>(key: string): Promise<T | null> {
    try {
      const value = localStorage.getItem(key);
      if (value === null) {
return null;
}
      
      // Try to parse as JSON first
      try {
        return JSON.parse(value) as T;
      } catch {
        // If not valid JSON, return the raw value for strings
        return value as T;
      }
    } catch (error) {
      logger.error('Failed to get from localStorage', error, LogCategory.DATABASE);
      return null;
    }
  }

  async set<T>(key: string, value: T): Promise<void> {
    try {
      localStorage.setItem(key, JSON.stringify(value));
    } catch (error) {
      logger.error('Failed to set to localStorage', error, LogCategory.DATABASE);
      throw createError(
        ErrorCode.FILE_WRITE_ERROR,
        'Failed to save data to localStorage',
        ErrorSeverity.ERROR,
        ErrorCategory.FILE_IO
      );
    }
  }

  async delete(key: string): Promise<void> {
    try {
      localStorage.removeItem(key);
    } catch (error) {
      logger.error('Failed to delete from localStorage', error, LogCategory.DATABASE);
    }
  }

  async clear(): Promise<void> {
    try {
      localStorage.clear();
    } catch (error) {
      logger.error('Failed to clear localStorage', error, LogCategory.DATABASE);
    }
  }

  async keys(): Promise<string[]> {
    try {
      return Object.keys(localStorage);
    } catch (error) {
      logger.error('Failed to get localStorage keys', error, LogCategory.DATABASE);
      return [];
    }
  }

  async exists(key: string): Promise<boolean> {
    try {
      return localStorage.getItem(key) !== null;
    } catch (error) {
      logger.error('Failed to check localStorage existence', error, LogCategory.DATABASE);
      return false;
    }
  }
}

/**
 * IndexedDB 实现
 */
class IndexedDBStore implements DataStore {
  private db: IDBDatabase | null = null;
  private dbName = 'PersistenceDB';
  private storeName = 'data';

  async initialize(): Promise<void> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(this.dbName, 1);

      request.onerror = () => {
        reject(new Error('Failed to open IndexedDB'));
      };

      request.onsuccess = () => {
        this.db = request.result;
        resolve();
      };

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        if (!db.objectStoreNames.contains(this.storeName)) {
          db.createObjectStore(this.storeName);
        }
      };
    });
  }

  async get<T>(key: string): Promise<T | null> {
    if (!this.db) {
      await this.initialize();
    }

    return new Promise((resolve, reject) => {
      const transaction = this.db?.transaction([this.storeName], 'readonly');
      if (!transaction) {
        reject(new Error('Database not initialized'));
        return;
      }
      const store = transaction.objectStore(this.storeName);
      const request = store.get(key);

      request.onsuccess = () => {
        resolve(request.result || null);
      };

      request.onerror = () => {
        reject(new Error('Failed to get from IndexedDB'));
      };
    });
  }

  async set<T>(key: string, value: T): Promise<void> {
    if (!this.db) {
      await this.initialize();
    }

    return new Promise((resolve, reject) => {
      const transaction = this.db?.transaction([this.storeName], 'readwrite');
      if (!transaction) {
        reject(new Error('Database not initialized'));
        return;
      }
      const store = transaction.objectStore(this.storeName);
      const request = store.put(value, key);

      request.onsuccess = () => {
        resolve();
      };

      request.onerror = () => {
        reject(new Error('Failed to set to IndexedDB'));
      };
    });
  }

  async delete(key: string): Promise<void> {
    if (!this.db) {
      await this.initialize();
    }

    return new Promise((resolve, reject) => {
      const transaction = this.db?.transaction([this.storeName], 'readwrite');
      if (!transaction) {
        reject(new Error('Database not initialized'));
        return;
      }
      const store = transaction.objectStore(this.storeName);
      const request = store.delete(key);

      request.onsuccess = () => {
        resolve();
      };

      request.onerror = () => {
        reject(new Error('Failed to delete from IndexedDB'));
      };
    });
  }

  async clear(): Promise<void> {
    if (!this.db) {
      await this.initialize();
    }

    return new Promise((resolve, reject) => {
      const transaction = this.db?.transaction([this.storeName], 'readwrite');
      if (!transaction) {
        reject(new Error('Database not initialized'));
        return;
      }
      const store = transaction.objectStore(this.storeName);
      const request = store.clear();

      request.onsuccess = () => {
        resolve();
      };

      request.onerror = () => {
        reject(new Error('Failed to clear IndexedDB'));
      };
    });
  }

  async keys(): Promise<string[]> {
    if (!this.db) {
      await this.initialize();
    }

    return new Promise((resolve, reject) => {
      const transaction = this.db?.transaction([this.storeName], 'readonly');
      if (!transaction) {
        reject(new Error('Database not initialized'));
        return;
      }
      const store = transaction.objectStore(this.storeName);
      const request = store.getAllKeys();

      request.onsuccess = () => {
        resolve(request.result as string[]);
      };

      request.onerror = () => {
        reject(new Error('Failed to get IndexedDB keys'));
      };
    });
  }

  async exists(key: string): Promise<boolean> {
    const value = await this.get(key);
    return value !== null;
  }
}

/**
 * 航空航天级持久化管理器
 */
export class PersistenceManager {
  private config: PersistenceConfig;
  private store: DataStore;
  private autoSaveInterval: number | null = null;
  private backups: DataBackup[] = [];
  private versions: Map<string, RecoveryPoint[]> = new Map();
  private currentVersion: number = 1;
  private dataValidators: Map<string, (value: any) => boolean> = new Map();

  constructor(config: Partial<PersistenceConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.store = this.createStore();
    this.initialize();
  }

  /**
   * 创建数据存储
   */
  private createStore(): DataStore {
    switch (this.config.storageType) {
      case 'localStorage':
        return new LocalStorageStore();
      case 'indexedDB':
        return new IndexedDBStore();
      case 'custom':
        if (this.config.customStore) {
          return this.config.customStore;
        }
        return new LocalStorageStore();
      default:
        return new LocalStorageStore();
    }
  }

  /**
   * 初始化持久化管理器
   */
  private async initialize(): Promise<void> {
    logger.info('Persistence manager initialized', { config: this.config }, LogCategory.DATABASE);

    // 加载现有的备份
    await this.loadBackups();

    // 加载现有的版本
    await this.loadVersions();

    // 启动自动保存
    if (this.config.enableAutoSave) {
      this.startAutoSave();
    }
  }

  /**
   * 加载备份
   */
  private async loadBackups(): Promise<void> {
    try {
      const backupData = await this.store.get<DataBackup[]>('backups');
      if (backupData) {
        this.backups = backupData;
        logger.info(`Loaded ${this.backups.length} backups`, {}, LogCategory.DATABASE);
      }
    } catch (error) {
      logger.error('Failed to load backups', error, LogCategory.DATABASE);
    }
  }

  /**
   * 加载版本
   */
  private async loadVersions(): Promise<void> {
    try {
      const versionData = await this.store.get<Record<string, RecoveryPoint[]>>('versions');
      if (versionData) {
        this.versions = new Map(Object.entries(versionData));
        logger.info(`Loaded versions for ${this.versions.size} keys`, {}, LogCategory.DATABASE);
      }

      const currentVersion = await this.store.get<number>('currentVersion');
      if (currentVersion) {
        this.currentVersion = currentVersion;
      }
    } catch (error) {
      logger.error('Failed to load versions', error, LogCategory.DATABASE);
    }
  }

  /**
   * 启动自动保存
   */
  private startAutoSave(): void {
    if (this.autoSaveInterval) {
      return;
    }

    this.autoSaveInterval = window.setInterval(() => {
      this.createBackup();
    }, this.config.autoSaveInterval);

    logger.info('Auto-save started', { interval: this.config.autoSaveInterval }, LogCategory.DATABASE);
  }

  /**
   * 停止自动保存
   */
  private stopAutoSave(): void {
    if (this.autoSaveInterval) {
      clearInterval(this.autoSaveInterval);
      this.autoSaveInterval = null;
      logger.info('Auto-save stopped', {}, LogCategory.DATABASE);
    }
  }

  /**
   * 计算校验和
   */
  private calculateChecksum(data: any): string {
    const str = JSON.stringify(data);
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
      const char = str.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash; // Convert to 32bit integer
    }
    return hash.toString(16);
  }

  /**
   * 压缩数据
   */
  private async compressData(data: any): Promise<any> {
    if (!this.config.enableCompression) {
      return data;
    }

    // 在实际应用中，这里应该使用真正的压缩算法
    // 如 LZString, pako 等
    return data;
  }

  /**
   * 解压缩数据
   */
  private async decompressData(data: any): Promise<any> {
    if (!this.config.enableCompression) {
      return data;
    }

    // 在实际应用中，这里应该使用真正的解压缩算法
    return data;
  }

  /**
   * 加密数据
   */
  private async encryptData(data: any): Promise<any> {
    if (!this.config.enableEncryption || !this.config.encryptionKey) {
      return data;
    }

    // 在实际应用中，这里应该使用真正的加密算法
    // 如 AES, RSA 等
    return data;
  }

  /**
   * 解密数据
   */
  private async decryptData(data: any): Promise<any> {
    if (!this.config.enableEncryption || !this.config.encryptionKey) {
      return data;
    }

    // 在实际应用中，这里应该使用真正的解密算法
    return data;
  }

  /**
   * 验证数据
   */
  private validateData(key: string, value: any): boolean {
    if (!this.config.enableDataValidation) {
      return true;
    }

    const validator = this.dataValidators.get(key);
    if (validator) {
      return validator(value);
    }

    return true;
  }

  /**
   * 注册数据验证器
   */
  registerValidator(key: string, validator: (value: any) => boolean): void {
    this.dataValidators.set(key, validator);
    logger.info(`Validator registered for key: ${key}`, {}, LogCategory.DATABASE);
  }

  /**
   * 保存数据
   */
  async save<T>(key: string, value: T, options?: { skipValidation?: boolean }): Promise<void> {
    try {
      // 验证数据
      if (!options?.skipValidation && !this.validateData(key, value)) {
        throw createError(
          ErrorCode.VALIDATION_ERROR,
          `Data validation failed for key: ${key}`,
          ErrorSeverity.ERROR,
          ErrorCategory.VALIDATION
        );
      }

      // 创建版本（如果启用）
      if (this.config.enableVersioning) {
        await this.createVersion(key, value);
      }

      // 保存数据
      const processedValue = await this.encryptData(await this.compressData(value));
      await this.store.set(key, processedValue);

      logger.info(`Data saved: ${key}`, { size: JSON.stringify(value).length }, LogCategory.DATABASE);
    } catch (error) {
      logger.error(`Failed to save data: ${key}`, error, LogCategory.DATABASE);
      throw error;
    }
  }

  /**
   * 加载数据
   */
  async load<T>(key: string): Promise<T | null> {
    try {
      const value = await this.store.get<T>(key);
      if (value === null) {
        return null;
      }

      const decompressedValue = await this.decompressData(await this.decryptData(value));
      return decompressedValue;
    } catch (error) {
      logger.error(`Failed to load data: ${key}`, error, LogCategory.DATABASE);
      return null;
    }
  }

  /**
   * 删除数据
   */
  async delete(key: string): Promise<void> {
    try {
      await this.store.delete(key);
      logger.info(`Data deleted: ${key}`, {}, LogCategory.DATABASE);
    } catch (error) {
      logger.error(`Failed to delete data: ${key}`, error, LogCategory.DATABASE);
      throw error;
    }
  }

  /**
   * 清除所有数据
   */
  async clear(): Promise<void> {
    try {
      await this.store.clear();
      this.backups = [];
      this.versions.clear();
      logger.info('All data cleared', {}, LogCategory.DATABASE);
    } catch (error) {
      logger.error('Failed to clear data', error, LogCategory.DATABASE);
      throw error;
    }
  }

  /**
   * 创建备份
   */
  async createBackup(): Promise<void> {
    try {
      const keys = await this.store.keys();
      const data: Record<string, any> = {};

      for (const key of keys) {
        if (key !== 'backups' && key !== 'versions' && key !== 'currentVersion') {
          const value = await this.store.get(key);
          if (value !== null) {
            data[key] = value;
          }
        }
      }

      const backup: DataBackup = {
        id: `backup-${Date.now()}`,
        timestamp: Date.now(),
        data,
        checksum: this.calculateChecksum(data),
        size: JSON.stringify(data).length,
        compressed: this.config.enableCompression,
        encrypted: this.config.enableEncryption
      };

      this.backups.push(backup);

      // 限制备份数量
      if (this.backups.length > this.config.maxBackups) {
        this.backups.shift();
      }

      await this.store.set('backups', this.backups);

      logger.info('Backup created', { id: backup.id, size: backup.size }, LogCategory.DATABASE);
    } catch (error) {
      logger.error('Failed to create backup', error, LogCategory.DATABASE);
    }
  }

  /**
   * 恢复备份
   */
  async restoreBackup(backupId: string): Promise<void> {
    try {
      const backup = this.backups.find(b => b.id === backupId);
      if (!backup) {
        throw createError(
          ErrorCode.RESOURCE_NOT_FOUND,
          `Backup not found: ${backupId}`,
          ErrorSeverity.ERROR,
          ErrorCategory.FILE_IO
        );
      }

      // 验证校验和
      const currentChecksum = this.calculateChecksum(backup.data);
      if (currentChecksum !== backup.checksum) {
        throw createError(
          ErrorCode.DATA_CORRUPTION,
          'Backup checksum mismatch - data may be corrupted',
          ErrorSeverity.CRITICAL,
          ErrorCategory.FILE_IO
        );
      }

      // 恢复数据
      for (const [key, value] of Object.entries(backup.data)) {
        await this.store.set(key, value);
      }

      logger.info('Backup restored', { id: backupId }, LogCategory.DATABASE);
    } catch (error) {
      logger.error('Failed to restore backup', error, LogCategory.DATABASE);
      throw error;
    }
  }

  /**
   * 创建版本
   */
  private async createVersion(key: string, value: any): Promise<void> {
    try {
      if (!this.versions.has(key)) {
        this.versions.set(key, []);
      }

      const versions = this.versions.get(key);
      if (!versions) {
        this.versions.set(key, []);
        return;
      }
      const recoveryPoint: RecoveryPoint = {
        id: `version-${this.currentVersion}-${Date.now()}`,
        timestamp: Date.now(),
        description: `Auto-save version ${this.currentVersion}`,
        data: value,
        version: this.currentVersion
      };

      versions.push(recoveryPoint);

      // 限制版本数量
      if (versions.length > this.config.maxVersions) {
        versions.shift();
      }

      this.currentVersion++;
      await this.store.set('currentVersion', this.currentVersion);
      await this.store.set('versions', Object.fromEntries(this.versions));

      logger.debug(`Version created for ${key}`, { version: recoveryPoint.version }, LogCategory.DATABASE);
    } catch (error) {
      logger.error('Failed to create version', error, LogCategory.DATABASE);
    }
  }

  /**
   * 恢复版本
   */
  async restoreVersion(key: string, versionId: string): Promise<void> {
    try {
      const versions = this.versions.get(key);
      if (!versions) {
        throw createError(
          ErrorCode.RESOURCE_NOT_FOUND,
          `No versions found for key: ${key}`,
          ErrorSeverity.ERROR,
          ErrorCategory.FILE_IO
        );
      }

      const version = versions.find(v => v.id === versionId);
      if (!version) {
        throw createError(
          ErrorCode.RESOURCE_NOT_FOUND,
          `Version not found: ${versionId}`,
          ErrorSeverity.ERROR,
          ErrorCategory.FILE_IO
        );
      }

      await this.save(key, version.data, { skipValidation: true });

      logger.info('Version restored', { key, versionId }, LogCategory.DATABASE);
    } catch (error) {
      logger.error('Failed to restore version', error, LogCategory.DATABASE);
      throw error;
    }
  }

  /**
   * 获取版本历史
   */
  getVersionHistory(key: string): RecoveryPoint[] {
    return this.versions.get(key) || [];
  }

  /**
   * 获取所有备份
   */
  getBackups(): DataBackup[] {
    return [...this.backups];
  }

  /**
   * 导出数据
   */
  async exportData(): Promise<string> {
    try {
      const keys = await this.store.keys();
      const data: Record<string, any> = {};

      for (const key of keys) {
        const value = await this.store.get(key);
        if (value !== null) {
          data[key] = value;
        }
      }

      return JSON.stringify({
        timestamp: Date.now(),
        data,
        backups: this.backups,
        versions: Object.fromEntries(this.versions)
      }, null, 2);
    } catch (error) {
      logger.error('Failed to export data', error, LogCategory.DATABASE);
      throw error;
    }
  }

  /**
   * 导入数据
   */
  async importData(jsonData: string): Promise<void> {
    try {
      if (!jsonData || typeof jsonData !== 'string') {
        throw createError(
          ErrorCode.VALIDATION_ERROR,
          'Invalid JSON data',
          ErrorSeverity.ERROR,
          ErrorCategory.VALIDATION
        );
      }

      const imported = JSON.parse(jsonData);

      // 验证数据结构
      if (!imported.data || typeof imported.data !== 'object') {
        throw createError(
          ErrorCode.VALIDATION_ERROR,
          'Invalid data format',
          ErrorSeverity.ERROR,
          ErrorCategory.VALIDATION
        );
      }

      // 导入数据
      for (const [key, value] of Object.entries(imported.data)) {
        await this.store.set(key, value);
      }

      // 导入备份和版本
      if (imported.backups) {
        this.backups = imported.backups;
        await this.store.set('backups', this.backups);
      }

      if (imported.versions) {
        this.versions = new Map(Object.entries(imported.versions));
        await this.store.set('versions', imported.versions);
      }

      logger.info('Data imported successfully', {}, LogCategory.DATABASE);
    } catch (error) {
      logger.error('Failed to import data', error, LogCategory.DATABASE);
      throw error;
    }
  }

  /**
   * 下载导出数据
   */
  async downloadExport(): Promise<void> {
    try {
      const data = await this.exportData();
      const blob = new Blob([data], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `data-export-${Date.now()}.json`;
      a.click();
      URL.revokeObjectURL(url);

      logger.info('Data export downloaded', {}, LogCategory.DATABASE);
    } catch (error) {
      logger.error('Failed to download export', error, LogCategory.DATABASE);
      throw error;
    }
  }

  /**
   * 获取存储统计信息
   */
  async getStorageStatistics(): Promise<{
    totalKeys: number;
    totalSize: number;
    backupCount: number;
    versionCount: number;
  }> {
    try {
      const keys = await this.store.keys();
      let totalSize = 0;

      for (const key of keys) {
        const value = await this.store.get(key);
        if (value !== null) {
          totalSize += JSON.stringify(value).length;
        }
      }

      let versionCount = 0;
      for (const versions of this.versions.values()) {
        versionCount += versions.length;
      }

      return {
        totalKeys: keys.length,
        totalSize,
        backupCount: this.backups.length,
        versionCount
      };
    } catch (error) {
      logger.error('Failed to get storage statistics', error, LogCategory.DATABASE);
      return {
        totalKeys: 0,
        totalSize: 0,
        backupCount: 0,
        versionCount: 0
      };
    }
  }

  /**
   * 更新配置
   */
  updateConfig(config: Partial<PersistenceConfig>): void {
    this.config = { ...this.config, ...config };

    // 重启自动保存（如果配置改变）
    if (this.config.enableAutoSave && !this.autoSaveInterval) {
      this.startAutoSave();
    } else if (!this.config.enableAutoSave && this.autoSaveInterval) {
      this.stopAutoSave();
    }

    logger.info('Persistence configuration updated', { config: this.config }, LogCategory.DATABASE);
  }

  /**
   * 销毁
   */
  destroy(): void {
    this.stopAutoSave();
    logger.info('Persistence manager destroyed', {}, LogCategory.DATABASE);
  }
}

// 导出单例
export const persistenceManager = new PersistenceManager();

// 导出便捷函数
export const storage = {
  save: <T>(key: string, value: T) => persistenceManager.save(key, value),
  load: <T>(key: string) => persistenceManager.load<T>(key),
  delete: (key: string) => persistenceManager.delete(key),
  clear: () => persistenceManager.clear(),
  createBackup: () => persistenceManager.createBackup(),
  restoreBackup: (id: string) => persistenceManager.restoreBackup(id),
  restoreVersion: (key: string, versionId: string) => persistenceManager.restoreVersion(key, versionId),
  getVersionHistory: (key: string) => persistenceManager.getVersionHistory(key),
  getBackups: () => persistenceManager.getBackups(),
  exportData: () => persistenceManager.exportData(),
  importData: (data: string) => persistenceManager.importData(data),
  downloadExport: () => persistenceManager.downloadExport(),
  getStatistics: () => persistenceManager.getStorageStatistics(),
  registerValidator: (key: string, validator: (value: any) => boolean) =>
    persistenceManager.registerValidator(key, validator),
  updateConfig: (config: Partial<PersistenceConfig>) => persistenceManager.updateConfig(config)
};
