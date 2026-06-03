/**
 * typstPreviewScheduler unit tests
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { createTypstPreviewScheduler } from '../typstPreviewScheduler';

describe('createTypstPreviewScheduler', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('runs the preview task after debounce delay', async () => {
    const run = vi.fn();
    const scheduler = createTypstPreviewScheduler(200, run);

    scheduler.schedule();
    expect(run).not.toHaveBeenCalled();

    vi.advanceTimersByTime(200);
    expect(run).toHaveBeenCalledTimes(1);
    expect(scheduler.isPending()).toBe(false);
  });

  it('reschedules when called repeatedly and only runs once', async () => {
    const run = vi.fn();
    const scheduler = createTypstPreviewScheduler(200, run);

    scheduler.schedule();
    vi.advanceTimersByTime(100);
    scheduler.schedule();
    vi.advanceTimersByTime(100);
    expect(run).not.toHaveBeenCalled();

    vi.advanceTimersByTime(100);
    expect(run).toHaveBeenCalledTimes(1);
  });

  it('allows subsequent schedules after a run completes', async () => {
    const run = vi.fn();
    const scheduler = createTypstPreviewScheduler(200, run);

    scheduler.schedule();
    vi.advanceTimersByTime(200);
    scheduler.schedule();
    vi.advanceTimersByTime(200);

    expect(run).toHaveBeenCalledTimes(2);
  });

  it('cancel clears pending timer', () => {
    const run = vi.fn();
    const scheduler = createTypstPreviewScheduler(200, run);

    scheduler.schedule();
    scheduler.cancel();
    vi.advanceTimersByTime(200);

    expect(run).not.toHaveBeenCalled();
    expect(scheduler.isPending()).toBe(false);
  });
});
