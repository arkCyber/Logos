/**
 * Cache Manager Utility
 * Manages application cache for improved performance
 */

interface CacheItem {
  data: any;
  timestamp: number;
  ttl?: number; // Time to live in milliseconds
}

class CacheManager {
  private cache: Map<string, CacheItem>;
  private defaultTTL: number = 5 * 60 * 1000; // 5 minutes default

  constructor() {
    this.cache = new Map();
  }

  /**
   * Set a cache item
   */
  set(key: string, data: any, ttl?: number): void {
    const item: CacheItem = {
      data,
      timestamp: Date.now(),
      ttl: ttl || this.defaultTTL
    };
    this.cache.set(key, item);
  }

  /**
   * Get a cache item
   */
  get(key: string): any | null {
    const item = this.cache.get(key);
    if (!item) {
      return null;
    }

    // Check if item has expired
    if (item.ttl && Date.now() - item.timestamp > item.ttl) {
      this.cache.delete(key);
      return null;
    }

    return item.data;
  }

  /**
   * Check if a key exists and is not expired
   */
  has(key: string): boolean {
    return this.get(key) !== null;
  }

  /**
   * Delete a cache item
   */
  delete(key: string): void {
    this.cache.delete(key);
  }

  /**
   * Clear all cache
   */
  clear(): void {
    this.cache.clear();
  }

  /**
   * Clean expired items
   */
  cleanExpired(): number {
    let cleaned = 0;
    const now = Date.now();

    for (const [key, item] of this.cache.entries()) {
      if (item.ttl && now - item.timestamp > item.ttl) {
        this.cache.delete(key);
        cleaned++;
      }
    }

    return cleaned;
  }

  /**
   * Get cache size
   */
  size(): number {
    return this.cache.size;
  }
}

export const cacheManager = new CacheManager();
