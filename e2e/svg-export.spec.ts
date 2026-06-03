import { test, expect } from '@playwright/test';

/**
 * E2E tests for SVG export UI entry points in File Backstage.
 * Validates visibility of aerospace-hardened export controls (no backend invoke in browser CI).
 */
test.describe('SVG Export UI', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:1425');
    await page.waitForSelector('.ProseMirror', { timeout: 10000 });
  });

  async function openInfoTab(page: import('@playwright/test').Page) {
    const fileButton = page.locator('.qat-button.file-button');
    await fileButton.click();

    const backstage = page.locator('.file-backstage');
    await expect(backstage).toBeVisible({ timeout: 5000 });

    const infoTab = page.locator('.sidebar-item:has-text("信息")');
    await infoTab.click();

    await expect(page.locator('.document-info')).toBeVisible();
  }

  test('should show Export SVG (Typst) button on info tab', async ({ page }) => {
    await openInfoTab(page);

    const typstButton = page.locator('button[aria-label="Export SVG (Typst)"]');
    await expect(typstButton).toBeVisible();
    await expect(typstButton).toContainText('Export SVG (Typst)');
  });

  test('should show Export SVG (HTML) button on info tab', async ({ page }) => {
    await openInfoTab(page);

    const htmlButton = page.locator('button[aria-label="Export SVG (HTML)"]');
    await expect(htmlButton).toBeVisible();
    await expect(htmlButton).toContainText('Export SVG (HTML)');
  });

  test('should list SVG export buttons alongside other export formats', async ({ page }) => {
    await openInfoTab(page);

    const exportSection = page.locator('.info-section:has-text("导出")');
    await expect(exportSection).toBeVisible();
    await expect(exportSection.locator('button[aria-label="Export SVG (Typst)"]')).toBeVisible();
    await expect(exportSection.locator('button[aria-label="Export SVG (HTML)"]')).toBeVisible();
    await expect(exportSection.locator('button[aria-label="Export Typst"]')).toBeVisible();
  });

  test('should close file backstage after clicking Export SVG (HTML)', async ({ page }) => {
    await openInfoTab(page);

    const htmlButton = page.locator('button[aria-label="Export SVG (HTML)"]');
    await htmlButton.click();

    await expect(page.locator('.file-backstage')).not.toBeVisible({ timeout: 5000 });
  });

  test('should close file backstage after clicking Export SVG (Typst)', async ({ page }) => {
    await openInfoTab(page);

    const typstButton = page.locator('button[aria-label="Export SVG (Typst)"]');
    await typstButton.click();

    await expect(page.locator('.file-backstage')).not.toBeVisible({ timeout: 5000 });
  });
});
