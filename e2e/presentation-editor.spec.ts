/**
 * Aerospace-grade E2E tests for Presentation Editor
 * Tests the complete user workflow with Playwright
 */

import { test, expect } from '@playwright/test';

test.describe.skip('Presentation Editor E2E Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the presentation editor
    await page.goto('/presentation-editor');
    
    // Wait for the editor to load
    await page.waitForSelector('.presentation-editor');
  });

  test('should load presentation editor', async ({ page }) => {
    // Check that the editor toolbar is visible
    await expect(page.locator('.editor-toolbar')).toBeVisible();
    
    // Check that the canvas is visible
    await expect(page.locator('.editor-canvas')).toBeVisible();
    
    // Check that the properties panel is visible
    await expect(page.locator('.properties-panel')).toBeVisible();
  });

  test('should add a new slide', async ({ page }) => {
    // Click the add slide button
    await page.click('.toolbar-btn[title="Add Slide"]');
    
    // Check that a new slide thumbnail appears
    const thumbnails = page.locator('.slide-thumbnail');
    await expect(thumbnails).toHaveCount(2);
  });

  test('should add a text element', async ({ page }) => {
    // Click the add text button
    await page.click('.toolbar-btn[title="Add Text"]');
    
    // Check that a text element appears on the canvas
    await expect(page.locator('.slide-element')).toBeVisible();
    
    // Check that the element is selected
    await expect(page.locator('.slide-element.selected')).toBeVisible();
  });

  test('should add an image element', async ({ page }) => {
    // Click the add image button
    await page.click('.toolbar-btn[title="Add Image"]');
    
    // Check that an image element appears
    await expect(page.locator('.element-image')).toBeVisible();
  });

  test('should add a shape element', async ({ page }) => {
    // Click the add shape button
    await page.click('.toolbar-btn[title="Add Shape"]');
    
    // Check that a shape element appears
    await expect(page.locator('.element-shape')).toBeVisible();
  });

  test('should add a table element', async ({ page }) => {
    // Click the add table button
    await page.click('.toolbar-btn[title="Add Table"]');
    
    // Check that a table element appears
    await expect(page.locator('.element-table')).toBeVisible();
  });

  test('should select and deselect elements', async ({ page }) => {
    // Add a text element
    await page.click('.toolbar-btn[title="Add Text"]');
    
    // Check that it's selected
    await expect(page.locator('.slide-element.selected')).toBeVisible();
    
    // Click on the canvas to deselect
    await page.click('.slide-canvas');
    
    // Check that nothing is selected
    await expect(page.locator('.slide-element.selected')).not.toBeVisible();
  });

  test('should edit element properties', async ({ page }) => {
    // Add a text element
    await page.click('.toolbar-btn[title="Add Text"]');
    
    // Change the font size
    const fontSizeInput = page.locator('input[type="number"][max="72"]');
    await fontSizeInput.fill('36');
    
    // Verify the change
    await expect(fontSizeInput).toHaveValue('36');
  });

  test('should change element color', async ({ page }) => {
    // Add a text element
    await page.click('.toolbar-btn[title="Add Text"]');
    
    // Change the color
    const colorInput = page.locator('input[type="color"]');
    await colorInput.fill('#ff0000');
    
    // Verify the change
    await expect(colorInput).toHaveValue('#ff0000');
  });

  test('should delete a slide', async ({ page }) => {
    // Add a new slide first
    await page.click('.toolbar-btn[title="Add Slide"]');
    
    // Click the delete slide button
    await page.click('.toolbar-btn[title="Delete Slide"]');
    
    // Check that only one slide remains
    const thumbnails = page.locator('.slide-thumbnail');
    await expect(thumbnails).toHaveCount(1);
  });

  test('should navigate between slides', async ({ page }) => {
    // Add a new slide
    await page.click('.toolbar-btn[title="Add Slide"]');
    
    // Click on the first slide thumbnail
    await page.locator('.slide-thumbnail').first().click();
    
    // Check that the first slide is active
    await expect(page.locator('.slide-thumbnail').first()).toHaveClass(/active/);
    
    // Click on the second slide thumbnail
    await page.locator('.slide-thumbnail').nth(1).click();
    
    // Check that the second slide is active
    await expect(page.locator('.slide-thumbnail').nth(1)).toHaveClass(/active/);
  });

  test('should export to Slidev', async ({ page }) => {
    // Click the export Slidev button
    const downloadPromise = page.waitForEvent('download');
    await page.click('button:has-text("Export Slidev")');
    const download = await downloadPromise;
    
    // Verify the download
    expect(download.suggestedFilename()).toMatch(/\.md$/);
  });

  test('should export to Typst', async ({ page }) => {
    // Click the export Typst button
    const downloadPromise = page.waitForEvent('download');
    await page.click('button:has-text("Export Typst")');
    const download = await downloadPromise;
    
    // Verify the download
    expect(download.suggestedFilename()).toMatch(/\.typ$/);
  });

  test('should undo and redo actions', async ({ page }) => {
    // Add a text element
    await page.click('.toolbar-btn[title="Add Text"]');
    
    // Undo the action
    await page.click('.toolbar-btn[title="Undo"]');
    
    // Check that the element is removed
    await expect(page.locator('.slide-element')).not.toBeVisible();
    
    // Redo the action
    await page.click('.toolbar-btn[title="Redo"]');
    
    // Check that the element reappears
    await expect(page.locator('.slide-element')).toBeVisible();
  });

  test('should disable undo when no history', async ({ page }) => {
    // Check that undo button is disabled initially
    const undoButton = page.locator('.toolbar-btn[title="Undo"]');
    await expect(undoButton).toBeDisabled();
  });

  test('should disable redo when at latest state', async ({ page }) => {
    // Add an element
    await page.click('.toolbar-btn[title="Add Text"]');
    
    // Check that redo button is disabled
    const redoButton = page.locator('.toolbar-btn[title="Redo"]');
    await expect(redoButton).toBeDisabled();
  });

  test('should change slide layout', async ({ page }) => {
    // Select a layout from the properties panel
    const layoutSelect = page.locator('select');
    await layoutSelect.selectOption('title');
    
    // Verify the selection
    await expect(layoutSelect).toHaveValue('title');
  });

  test('should change slide background', async ({ page }) => {
    // Change the background color
    const colorInput = page.locator('.property-row:has-text("Background") input[type="color"]');
    await colorInput.fill('#ff0000');
    
    // Verify the change
    await expect(colorInput).toHaveValue('#ff0000');
  });

  test('should show element properties when selected', async ({ page }) => {
    // Add a text element
    await page.click('.toolbar-btn[title="Add Text"]');
    
    // Check that element properties section is visible
    await expect(page.locator('.property-section:has-text("Element Properties")')).toBeVisible();
  });

  test('should hide element properties when nothing selected', async ({ page }) => {
    // Click on canvas to deselect
    await page.click('.slide-canvas');
    
    // Check that element properties section shows message
    await expect(page.locator('.property-section:has-text("Select an element")')).toBeVisible();
  });

  test('should display slide count correctly', async ({ page }) => {
    // Check initial slide count
    const counter = page.locator('.slide-counter');
    await expect(counter).toHaveText('1 / 1');
    
    // Add a slide
    await page.click('.toolbar-btn[title="Add Slide"]');
    
    // Check updated slide count
    await expect(counter).toHaveText('1 / 2');
  });

  test('should handle keyboard shortcuts', async ({ page }) => {
    // Add a text element
    await page.click('.toolbar-btn[title="Add Text"]');
    
    // Press Ctrl+Z to undo
    await page.keyboard.press('Control+Z');
    
    // Check that element is removed
    await expect(page.locator('.slide-element')).not.toBeVisible();
  });
});

test.describe.skip('Slidev Integration E2E Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the Slidev integration page
    await page.goto('/slidev-integration');
    
    // Wait for the integration to load
    await page.waitForSelector('.slidev-integration');
  });

  test('should load Slidev integration', async ({ page }) => {
    // Check that the header is visible
    await expect(page.locator('.slidev-header')).toBeVisible();
    
    // Check that the main slide view is visible
    await expect(page.locator('.slidev-main')).toBeVisible();
  });

  test('should navigate slides with keyboard', async ({ page }) => {
    // Press right arrow to go to next slide
    await page.keyboard.press('ArrowRight');
    
    // Wait for navigation
    await page.waitForTimeout(100);
    
    // Check that slide counter updated
    const counter = page.locator('.slide-counter');
    await expect(counter).toHaveText('2 /');
  });

  test('should navigate slides with buttons', async ({ page }) => {
    // Click next button
    await page.locator('.control-btn').nth(1).click();
    
    // Check that slide counter updated
    const counter = page.locator('.slide-counter');
    await expect(counter).toHaveText('2 /');
  });

  test('should toggle presenter mode', async ({ page }) => {
    // Click presenter mode button
    await page.locator('.control-btn[title="Presenter Mode"]').click();
    
    // Check that presenter view is visible
    await expect(page.locator('.slidev-presenter')).toBeVisible();
  });

  test('should show presenter notes', async ({ page }) => {
    // Enable presenter mode
    await page.locator('.control-btn[title="Presenter Mode"]').click();
    
    // Check that notes section is visible
    await expect(page.locator('.presenter-notes')).toBeVisible();
  });

  test('should show next slide preview', async ({ page }) => {
    // Enable presenter mode
    await page.locator('.control-btn[title="Presenter Mode"]').click();
    
    // Check that next slide section is visible
    await expect(page.locator('.presenter-next')).toBeVisible();
  });

  test('should show timer', async ({ page }) => {
    // Enable presenter mode
    await page.locator('.control-btn[title="Presenter Mode"]').click();
    
    // Check that timer is visible
    await expect(page.locator('.presenter-timer')).toBeVisible();
  });

  test('should toggle fullscreen', async ({ page }) => {
    // Click fullscreen button
    await page.locator('.control-btn[title="Fullscreen"]').click();
    
    // Note: Fullscreen behavior varies by browser
    // Just verify the button is clickable
    await expect(page.locator('.control-btn[title="Fullscreen"]')).toBeVisible();
  });

  test('should toggle thumbnails panel', async ({ page }) => {
    // Click the toggle button
    await page.locator('.thumbnail-toggle').click();
    
    // Check that thumbnails are collapsed
    await expect(page.locator('.slidev-thumbnails')).toHaveClass(/collapsed/);
    
    // Click again to expand
    await page.locator('.thumbnail-toggle').click();
    
    // Check that thumbnails are expanded
    await expect(page.locator('.slidev-thumbnails')).not.toHaveClass(/collapsed/);
  });

  test('should navigate to specific slide from thumbnails', async ({ page }) => {
    // Click on a thumbnail
    await page.locator('.thumbnail-item').nth(1).click();
    
    // Check that slide counter updated
    const counter = page.locator('.slide-counter');
    await expect(counter).toHaveText('2 /');
  });

  test('should exit preview', async ({ page }) => {
    // Click exit button
    await page.locator('.control-btn[title="Exit Preview"]').click();
    
    // Check that we navigated away
    await expect(page).not.toHaveURL(/slidev-integration/);
  });
});

test.describe.skip('Presentation Format Conversion E2E Tests', () => {
  test('should convert to Slidev format', async ({ page }) => {
    // Create a test presentation
    await page.goto('/presentation-editor');
    
    // Add some content
    await page.click('.toolbar-btn[title="Add Text"]');
    
    // Export to Slidev
    const downloadPromise = page.waitForEvent('download');
    await page.click('button:has-text("Export Slidev")');
    const download = await downloadPromise;
    
    // Verify download
    expect(download.suggestedFilename()).toMatch(/\.md$/);
  });

  test('should convert to Typst format', async ({ page }) => {
    // Create a test presentation
    await page.goto('/presentation-editor');
    
    // Add some content
    await page.click('.toolbar-btn[title="Add Text"]');
    
    // Export to Typst
    const downloadPromise = page.waitForEvent('download');
    await page.click('button:has-text("Export Typst")');
    const download = await downloadPromise;
    
    // Verify download
    expect(download.suggestedFilename()).toMatch(/\.typ$/);
  });
});
