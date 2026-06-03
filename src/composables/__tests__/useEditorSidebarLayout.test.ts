/**
 * useEditorSidebarLayout unit tests
 */

import { describe, it, expect } from 'vitest';
import { ref } from 'vue';
import {
  useEditorSidebarLayout,
  EDITOR_SIDEBAR_WIDTHS,
  EDITOR_CANVAS_MIN_WIDTH,
} from '../useEditorSidebarLayout';

describe('useEditorSidebarLayout', () => {
  function createVisibility(overrides: Partial<Record<string, boolean>> = {}) {
    return {
      showDocumentOutline: ref(overrides.showDocumentOutline ?? false),
      showSplitView: ref(overrides.showSplitView ?? false),
      showAISidebar: ref(overrides.showAISidebar ?? false),
      showCommentsPanel: ref(overrides.showCommentsPanel ?? false),
      showRevisionPanel: ref(overrides.showRevisionPanel ?? false),
      showSpreadsheet: ref(overrides.showSpreadsheet ?? false),
      showUniverSpreadsheet: ref(overrides.showUniverSpreadsheet ?? false),
    };
  }

  it('exposes sidebar width constants', () => {
    expect(EDITOR_SIDEBAR_WIDTHS.outline).toBe(280);
    expect(EDITOR_SIDEBAR_WIDTHS.ai).toBe(360);
    expect(EDITOR_CANVAS_MIN_WIDTH).toBeGreaterThan(0);
  });

  it('detects left panels only when document outline is open', () => {
    const layout = useEditorSidebarLayout(
      createVisibility({ showDocumentOutline: true })
    );
    expect(layout.hasLeftPanels.value).toBe(true);
    expect(layout.hasRightPanels.value).toBe(false);
    expect(layout.leftPanelCount.value).toBe(1);
  });

  it('places Typst split preview on the right column', () => {
    const layout = useEditorSidebarLayout(
      createVisibility({ showSplitView: true })
    );
    expect(layout.hasLeftPanels.value).toBe(false);
    expect(layout.hasRightPanels.value).toBe(true);
    expect(layout.rightPanelCount.value).toBe(1);
    expect(layout.layoutClasses.value['editor-workspace--split-open']).toBe(true);
  });

  it('detects right panels when comments, revision, or AI is open', () => {
    const layout = useEditorSidebarLayout(
      createVisibility({ showCommentsPanel: true, showAISidebar: true })
    );
    expect(layout.hasRightPanels.value).toBe(true);
    expect(layout.rightPanelCount.value).toBe(2);
    expect(layout.layoutClasses.value['editor-workspace--comments-open']).toBe(true);
    expect(layout.layoutClasses.value['editor-workspace--ai-open']).toBe(true);
  });

  it('sets CSS variables for panel widths', () => {
    const layout = useEditorSidebarLayout(createVisibility());
    expect(layout.layoutStyle.value['--editor-sidebar-outline-width']).toBe('280px');
    expect(layout.layoutStyle.value['--editor-canvas-min-width']).toBe(`${EDITOR_CANVAS_MIN_WIDTH}px`);
  });

  it('counts multiple open right panels independently', () => {
    const layout = useEditorSidebarLayout(
      createVisibility({
        showSplitView: true,
        showCommentsPanel: true,
        showRevisionPanel: true,
        showSpreadsheet: true,
      })
    );
    expect(layout.rightPanelCount.value).toBe(4);
    expect(layout.layoutClasses.value['editor-workspace--split-open']).toBe(true);
    expect(layout.layoutClasses.value['editor-workspace--spreadsheet-open']).toBe(true);
  });
});
