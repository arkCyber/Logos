import { test, expect } from '@playwright/test';

test.describe('Tiptap Features Integration Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:5173');
  });

  test('should configure and use all Tiptap extensions', async ({ page }) => {
    // Wait for editor to load
    await page.waitForSelector('.ProseMirror');

    // Test basic text formatting
    await page.click('button[title="加粗"]');
    await page.keyboard.type('Bold text');
    expect(await page.locator('strong').count()).toBeGreaterThan(0);

    await page.click('button[title="斜体"]');
    await page.keyboard.type('Italic text');
    expect(await page.locator('em').count()).toBeGreaterThan(0);

    // Test subscript and superscript
    await page.click('button[title="下标"]');
    await page.keyboard.type('Subscript');
    expect(await page.locator('sub').count()).toBeGreaterThan(0);

    await page.click('button[title="上标"]');
    await page.keyboard.type('Superscript');
    expect(await page.locator('sup').count()).toBeGreaterThan(0);
  });

  test('should insert and manipulate tables', async ({ page }) => {
    await page.waitForSelector('.ProseMirror');

    // Insert table
    await page.click('button[title="表格"]');
    await page.waitForTimeout(500);

    // Check if table was inserted
    const table = page.locator('table');
    await expect(table).toBeVisible();

    // Test adding column
    await page.click('button[title="在右侧插入列"]');
    await page.waitForTimeout(200);

    // Test adding row
    await page.click('button[title="在下方插入行"]');
    await page.waitForTimeout(200);

    // Test merging cells
    await page.click('button[title="合并单元格"]');
    await page.waitForTimeout(200);
  });

  test('should use task list feature', async ({ page }) => {
    await page.waitForSelector('.ProseMirror');

    // Toggle task list
    await page.click('button[title="任务列表"]');
    await page.waitForTimeout(200);

    // Check if task list was created
    const taskList = page.locator('ul[data-type="taskList"]');
    await expect(taskList).toBeVisible();
  });

  test('should use text highlight feature', async ({ page }) => {
    await page.waitForSelector('.ProseMirror');

    // Type some text
    await page.keyboard.type('Highlight this text');

    // Select the text
    await page.keyboard.press('Control+A');

    // Toggle highlight
    await page.click('button[title="高亮"]');
    await page.waitForTimeout(200);

    // Check if text is highlighted
    const highlight = page.locator('mark');
    await expect(highlight).toBeVisible();
  });

  test('should use code block with syntax highlighting', async ({ page }) => {
    await page.waitForSelector('.ProseMirror');

    // Insert code block
    await page.click('button[title="代码块"]');
    await page.waitForTimeout(200);

    // Type code
    await page.keyboard.type('const x = 5;');

    // Check if code block exists
    const codeBlock = page.locator('pre');
    await expect(codeBlock).toBeVisible();
  });

  test('should show bubble menu on text selection', async ({ page }) => {
    await page.waitForSelector('.ProseMirror');

    // Type text
    await page.keyboard.type('Select this text');

    // Select text
    await page.keyboard.press('Control+A');

    // Wait for bubble menu to appear
    await page.waitForSelector('.bubble-menu', { timeout: 1000 });

    // Check if bubble menu is visible
    const bubbleMenu = page.locator('.bubble-menu');
    await expect(bubbleMenu).toBeVisible();
  });

  test('should use font family feature', async ({ page }) => {
    await page.waitForSelector('.ProseMirror');

    // Type text
    await page.keyboard.type('Test text');

    // Select text
    await page.keyboard.press('Control+A');

    // Change font family
    const fontSelect = page.locator('select').first();
    await fontSelect.selectOption('Times New Roman, serif');
    await page.waitForTimeout(200);

    // Check if font was applied
    const text = page.locator('.ProseMirror');
    const fontFamily = await text.evaluate(el => getComputedStyle(el).fontFamily);
    expect(fontFamily).toContain('Times New Roman');
  });

  test('should use link feature', async ({ page }) => {
    await page.waitForSelector('.ProseMirror');

    // Type text
    await page.keyboard.type('Click here');

    // Select text
    await page.keyboard.press('Control+A');

    // Insert link
    await page.click('button[title="链接"]');
    await page.waitForTimeout(200);

    // Check if link was created
    const link = page.locator('a');
    await expect(link).toBeVisible();
  });

  test('should use placeholder feature', async ({ page }) => {
    await page.waitForSelector('.ProseMirror');

    // Clear editor
    await page.keyboard.press('Control+A');
    await page.keyboard.press('Backspace');

    // Check if placeholder is visible
    const placeholder = page.locator('.p.is-editor-empty:first-child::before');
    await expect(placeholder).toBeVisible();
  });

  test('should handle image operations', async ({ page }) => {
    await page.waitForSelector('.ProseMirror');

    // Click insert image button
    await page.click('button[title="图片"]');
    await page.waitForTimeout(200);

    // Enter image URL (mock)
    await page.fill('input[type="text"]', 'https://via.placeholder.com/300');
    await page.click('button:has-text("插入")');
    await page.waitForTimeout(500);

    // Check if image was inserted
    const image = page.locator('img');
    await expect(image).toBeVisible();
  });

  test('should use typography features', async ({ page }) => {
    await page.waitForSelector('.ProseMirror');

    // Type text with quotes
    await page.keyboard.type('"Smart quotes"');

    // Check if smart quotes were applied
    const text = await page.locator('.ProseMirror').textContent();
    expect(text).toContain('""');
  });

  test('should maintain undo/redo history', async ({ page }) => {
    await page.waitForSelector('.ProseMirror');

    // Type text
    await page.keyboard.type('Original text');

    // Modify text
    await page.keyboard.press('Control+A');
    await page.keyboard.type('Modified text');

    // Undo
    await page.click('button[title="撤销"]');
    await page.waitForTimeout(200);

    // Check if text was restored
    const text = await page.locator('.ProseMirror').textContent();
    expect(text).toContain('Original text');

    // Redo
    await page.click('button[title="重做"]');
    await page.waitForTimeout(200);

    // Check if modification was reapplied
    const textAfterRedo = await page.locator('.ProseMirror').textContent();
    expect(textAfterRedo).toContain('Modified text');
  });
});
