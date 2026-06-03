import { test, expect } from '@playwright/test';

/**
 * E2E tests for Typst split-view sidebar placement.
 * Preview panel must render in the right workspace column (adjacent to editor).
 */
test.describe('Typst Split View Sidebar', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:1425');
    await page.waitForSelector('.ProseMirror', { timeout: 10000 });
  });

  test('should open Typst preview in the right workspace column', async ({ page }) => {
    const splitButton = page.locator('.qat-right-btn.share-btn');
    await splitButton.click();

    const sidebar = page.locator('[data-testid="split-view-sidebar"]');
    await expect(sidebar).toBeVisible({ timeout: 5000 });

    const rightColumn = page.locator('.editor-workspace__right');
    await expect(rightColumn).toBeVisible();
    await expect(rightColumn.locator('[data-testid="split-view-sidebar"]')).toBeVisible();
  });

  test('should place editor canvas before the right column in layout order', async ({ page }) => {
    const splitButton = page.locator('.qat-right-btn.share-btn');
    await splitButton.click();

    await expect(page.locator('[data-testid="split-view-sidebar"]')).toBeVisible({ timeout: 5000 });

    const order = await page.locator('.editor-workspace').evaluate((workspace) => {
      const children = Array.from(workspace.children);
      const centerIdx = children.findIndex((el) => el.classList.contains('editor-workspace__center'));
      const rightIdx = children.findIndex((el) => el.classList.contains('editor-workspace__right'));
      return { centerIdx, rightIdx };
    });

    expect(order.centerIdx).toBeGreaterThanOrEqual(0);
    expect(order.rightIdx).toBeGreaterThan(order.centerIdx);
  });

  test('should not place Typst preview in the left workspace column', async ({ page }) => {
    const splitButton = page.locator('.qat-right-btn.share-btn');
    await splitButton.click();

    await expect(page.locator('[data-testid="split-view-sidebar"]')).toBeVisible({ timeout: 5000 });

    const leftColumn = page.locator('.editor-workspace__left');
    if (await leftColumn.count()) {
      await expect(leftColumn.locator('[data-testid="split-view-sidebar"]')).toHaveCount(0);
    }
  });

  test('should close Typst preview from panel close button', async ({ page }) => {
    const splitButton = page.locator('.qat-right-btn.share-btn');
    await splitButton.click();

    const sidebar = page.locator('[data-testid="split-view-sidebar"]');
    await expect(sidebar).toBeVisible({ timeout: 5000 });

    await page.locator('button[aria-label="Close Typst preview"]').click();
    await expect(sidebar).not.toBeVisible();
  });
});
