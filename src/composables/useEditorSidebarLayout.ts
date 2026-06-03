/**
 * Editor Sidebar Layout Composable
 *
 * Coordinates left/right sidebar panels so they always render at full height
 * inside the editor workspace while the document canvas compresses (flex: 1).
 */

import { computed, type Ref } from 'vue';

/** Fixed panel widths in CSS pixels (must match panel component styles). */
export const EDITOR_SIDEBAR_WIDTHS = {
  outline: 280,
  splitPreview: 400,
  ai: 360,
  comments: 350,
  revision: 400,
  spreadsheetMax: 800,
  spreadsheetMin: 320,
} as const;

/** Minimum width reserved for the document canvas before horizontal scroll appears. */
export const EDITOR_CANVAS_MIN_WIDTH = 160;

export interface EditorSidebarVisibility {
  showDocumentOutline: Ref<boolean>;
  showSplitView: Ref<boolean>;
  showAISidebar: Ref<boolean>;
  showCommentsPanel: Ref<boolean>;
  showRevisionPanel: Ref<boolean>;
  showSpreadsheet: Ref<boolean>;
  showUniverSpreadsheet: Ref<boolean>;
}

/**
 * Compute layout flags and CSS classes for the editor workspace row.
 */
export function useEditorSidebarLayout(visibility: EditorSidebarVisibility) {
  const hasLeftPanels = computed(() => visibility.showDocumentOutline.value);

  const hasRightPanels = computed(
    () =>
      visibility.showSplitView.value ||
      visibility.showAISidebar.value ||
      visibility.showCommentsPanel.value ||
      visibility.showRevisionPanel.value ||
      visibility.showSpreadsheet.value ||
      visibility.showUniverSpreadsheet.value
  );

  const leftPanelCount = computed(() => {
    return visibility.showDocumentOutline.value ? 1 : 0;
  });

  const rightPanelCount = computed(() => {
    let count = 0;
    if (visibility.showSplitView.value) {
      count += 1;
    }
    if (visibility.showCommentsPanel.value) {
      count += 1;
    }
    if (visibility.showRevisionPanel.value) {
      count += 1;
    }
    if (visibility.showAISidebar.value) {
      count += 1;
    }
    if (visibility.showSpreadsheet.value) {
      count += 1;
    }
    if (visibility.showUniverSpreadsheet.value) {
      count += 1;
    }
    return count;
  });

  const layoutClasses = computed(() => ({
    'editor-workspace--left-open': hasLeftPanels.value,
    'editor-workspace--right-open': hasRightPanels.value,
    'editor-workspace--outline-open': visibility.showDocumentOutline.value,
    'editor-workspace--split-open': visibility.showSplitView.value,
    'editor-workspace--ai-open': visibility.showAISidebar.value,
    'editor-workspace--comments-open': visibility.showCommentsPanel.value,
    'editor-workspace--revision-open': visibility.showRevisionPanel.value,
    'editor-workspace--spreadsheet-open':
      visibility.showSpreadsheet.value || visibility.showUniverSpreadsheet.value,
  }));

  const layoutStyle = computed(() => ({
    '--editor-canvas-min-width': `${EDITOR_CANVAS_MIN_WIDTH}px`,
    '--editor-sidebar-outline-width': `${EDITOR_SIDEBAR_WIDTHS.outline}px`,
    '--editor-sidebar-split-width': `${EDITOR_SIDEBAR_WIDTHS.splitPreview}px`,
    '--editor-sidebar-ai-width': `${EDITOR_SIDEBAR_WIDTHS.ai}px`,
    '--editor-sidebar-comments-width': `${EDITOR_SIDEBAR_WIDTHS.comments}px`,
    '--editor-sidebar-revision-width': `${EDITOR_SIDEBAR_WIDTHS.revision}px`,
    '--editor-sidebar-spreadsheet-max-width': `${EDITOR_SIDEBAR_WIDTHS.spreadsheetMax}px`,
    '--editor-sidebar-spreadsheet-min-width': `${EDITOR_SIDEBAR_WIDTHS.spreadsheetMin}px`,
  }));

  return {
    hasLeftPanels,
    hasRightPanels,
    leftPanelCount,
    rightPanelCount,
    layoutClasses,
    layoutStyle,
  };
}
