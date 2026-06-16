/**
 * Backup Manager tests
 */
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { backupManager } from '../backupManager';

describe('BackupManager', () => {
  beforeEach(() => {
    localStorage.clear();
    backupManager.disableAutoBackup();
  });

  afterEach(() => {
    backupManager.disableAutoBackup();
  });

  it('creates a backup entry with a timestamped filename', async () => {
    const filename = await backupManager.createBackup('content-A', 'doc');
    expect(filename).toMatch(/^doc_.+\.json$/);

    const stored = localStorage.getItem(`backup_${filename}`);
    expect(stored).not.toBeNull();
    const parsed = JSON.parse(stored as string);
    expect(parsed.content).toBe('content-A');
    expect(parsed.filename).toBe('doc');
  });

  it('restores the saved content', async () => {
    const filename = await backupManager.createBackup('payload', 'doc');
    const restored = await backupManager.restoreBackup(filename);
    expect(restored).toBe('payload');
  });

  it('returns null for a missing backup', async () => {
    expect(await backupManager.restoreBackup('does-not-exist.json')).toBeNull();
  });

  it('lists all backups sorted newest first', async () => {
    const f1 = await backupManager.createBackup('a', 'doc');
    await new Promise(resolve => setTimeout(resolve, 5));
    await backupManager.createBackup('b', 'doc');
    await new Promise(resolve => setTimeout(resolve, 5));
    const f3 = await backupManager.createBackup('c', 'doc');

    const list = backupManager.listBackups();
    expect(list).toHaveLength(3);
    expect(list[0]).toBe(f3);
    expect(list[2]).toBe(f1);
  });

  it('deletes a backup by name', async () => {
    const f = await backupManager.createBackup('x', 'doc');
    backupManager.deleteBackup(f);
    expect(await backupManager.restoreBackup(f)).toBeNull();
  });

  it('keeps only the most recent maxBackups entries', async () => {
    backupManager.updateConfig({ maxBackups: 2 });
    const f1 = await backupManager.createBackup('a', 'doc');
    await new Promise(resolve => setTimeout(resolve, 5));
    const f2 = await backupManager.createBackup('b', 'doc');
    await new Promise(resolve => setTimeout(resolve, 5));
    const f3 = await backupManager.createBackup('c', 'doc');

    const removed = backupManager.cleanOldBackups();
    expect(removed).toBe(1);
    expect(await backupManager.restoreBackup(f1)).toBeNull();
    expect(await backupManager.restoreBackup(f2)).not.toBeNull();
    expect(await backupManager.restoreBackup(f3)).not.toBeNull();
  });

  it('runs the callback on a schedule when auto backup is enabled', () => {
    vi.useFakeTimers();
    backupManager.updateConfig({ backupInterval: 1000 });
    const cb = vi.fn();
    backupManager.enableAutoBackup(cb);
    vi.advanceTimersByTime(3500);
    backupManager.disableAutoBackup();
    expect(cb).toHaveBeenCalledTimes(3);
    vi.useRealTimers();
  });

  it('skips the callback when autoBackup is disabled', () => {
    vi.useFakeTimers();
    backupManager.updateConfig({ autoBackup: false, backupInterval: 1000 });
    const cb = vi.fn();
    backupManager.enableAutoBackup(cb);
    vi.advanceTimersByTime(2500);
    backupManager.disableAutoBackup();
    expect(cb).toHaveBeenCalledTimes(0);
    vi.useRealTimers();
  });

  it('updates configuration immutably', () => {
    backupManager.updateConfig({ maxBackups: 10 });
    const original = backupManager.getConfig();
    backupManager.updateConfig({ maxBackups: 3 });
    expect(backupManager.getConfig().maxBackups).toBe(3);
    expect(original.maxBackups).toBe(10);
  });
});
