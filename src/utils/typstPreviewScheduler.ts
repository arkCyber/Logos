/**
 * Debounced scheduler for Typst/SVG preview refresh.
 *
 * Ensures every content change reschedules compilation and the timer handle
 * is cleared after each run so later edits keep syncing.
 */

export interface TypstPreviewScheduler {
  schedule: () => void;
  cancel: () => void;
  isPending: () => boolean;
}

/**
 * Create a debounced async task runner for preview compilation.
 */
export function createTypstPreviewScheduler(
  debounceMs: number,
  run: () => void | Promise<void>
): TypstPreviewScheduler {
  let timer: ReturnType<typeof setTimeout> | null = null;

  const schedule = () => {
    if (timer !== null) {
      clearTimeout(timer);
    }
    timer = setTimeout(async () => {
      timer = null;
      await run();
    }, debounceMs);
  };

  const cancel = () => {
    if (timer !== null) {
      clearTimeout(timer);
      timer = null;
    }
  };

  const isPending = () => timer !== null;

  return { schedule, cancel, isPending };
}
