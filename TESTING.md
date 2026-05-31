# Logos Testing Guide

This document provides comprehensive testing instructions for Logos, the AI-powered word processing software.

## Prerequisites

- Ensure the application is running: `bun run tauri dev`
- For AI features testing, configure your API key in `src-tauri/src/lib.rs`

## Test Categories

### 1. Basic Editor Formatting Features

#### Test 1.1: Text Formatting
- **Bold**: Type some text, select it, click the Bold button (B) or use Ctrl/Cmd+B
- **Italic**: Type some text, select it, click the Italic button (I) or use Ctrl/Cmd+I
- **Strikethrough**: Type some text, select it, click the Strikethrough button
- **Code**: Type some text, select it, click the Code button

**Expected Result**: Text should display with the applied formatting

#### Test 1.2: Headings
- Type some text
- Select it and click H1, H2, or H3 buttons
- Verify different heading sizes are applied

**Expected Result**: Text should display as heading with appropriate size

#### Test 1.3: Lists
- **Bullet List**: Click the bullet list button, type items, press Enter for new items
- **Ordered List**: Click the ordered list button, type items, press Enter for new items

**Expected Result**: Lists should display correctly with proper numbering/bullets

#### Test 1.4: Blockquote
- Type some text
- Select it and click the quote button
- Verify blockquote styling is applied

**Expected Result**: Text should display as a blockquote with left border

### 2. Table Operations

#### Test 2.1: Insert Table
- Click the table button in the toolbar
- A 3x3 table with header row should be inserted

**Expected Result**: Table appears with 3 rows and 3 columns, first row as header

#### Test 2.2: Add/Remove Columns
- Click in a table cell
- Click "Add Column" button - a new column should appear
- Click "Delete Column" button - the column should be removed

**Expected Result**: Columns are added/removed correctly

#### Test 2.3: Add/Remove Rows
- Click in a table cell
- Click "Add Row" button - a new row should appear
- Click "Delete Row" button - the row should be removed

**Expected Result**: Rows are added/removed correctly

#### Test 2.4: Toggle Headers
- Click in a table cell
- Click "Toggle Header Row" - the row should become a header
- Click "Toggle Header Column" - the column should become a header
- Click "Toggle Header Cell" - the cell should become a header

**Expected Result**: Header styling is applied to selected rows/columns/cells

#### Test 2.5: Resize Columns
- Hover over the right edge of a column
- Drag to resize the column width

**Expected Result**: Column width changes smoothly

#### Test 2.6: Delete Table
- Click anywhere in the table
- Click the "Delete Table" button

**Expected Result**: Entire table is removed

### 3. AI Features

**Note**: Requires API key configuration in `src-tauri/src/lib.rs`

#### Test 3.1: AI Polish (润色)
- Type some casual text: "这个文章写得挺好的"
- Select the text
- AI bubble menu should appear above selection
- Click "润色" button
- Wait for AI response

**Expected Result**: Text is replaced with more professional/academic version

#### Test 3.2: AI Expand (扩写)
- Type a short sentence: "人工智能很重要"
- Select the text
- Click "扩写" button in bubble menu
- Wait for AI response

**Expected Result**: Text is expanded with more details and context

#### Test 3.3: AI Rewrite (重写)
- Type a sentence: "今天天气很好"
- Select the text
- Click "重写" button in bubble menu
- Wait for AI response

**Expected Result**: Text is rewritten with different wording but same meaning

#### Test 3.4: AI Summarize (总结)
- Type a longer paragraph with multiple sentences
- Select the text
- Click "总结" button in bubble menu
- Wait for AI response

**Expected Result**: Text is replaced with a concise summary

#### Test 3.5: AI Translate (翻译)
- Type some Chinese text
- Select the text
- Click "翻译" button in bubble menu
- Wait for AI response

**Expected Result**: Text is translated to English

### 4. Keyboard Shortcuts

#### Test 4.1: Save (Ctrl/Cmd + S)
- Type some text
- Press Ctrl/Cmd + S
- Alert should appear indicating save function was triggered

**Expected Result**: Save function is called (logs to console)

#### Test 4.2: Open (Ctrl/Cmd + O)
- Press Ctrl/Cmd + O
- Alert should appear indicating load function was triggered

**Expected Result**: Load function is called (logs to console)

#### Test 4.3: Bold (Ctrl/Cmd + B)
- Type some text, select it
- Press Ctrl/Cmd + B

**Expected Result**: Text becomes bold

#### Test 4.4: Italic (Ctrl/Cmd + I)
- Type some text, select it
- Press Ctrl/Cmd + I

**Expected Result**: Text becomes italic

#### Test 4.5: Undo/Redo
- Type some text
- Press Ctrl/Cmd + Z to undo
- Press Ctrl/Cmd + Shift + Z to redo

**Expected Result**: Undo/redo works correctly

### 5. Export Features

#### Test 5.1: Markdown Export
- Create content with various formatting
- Click the download button in toolbar
- Check console for exported Markdown

**Expected Result**: Markdown content is logged to console

### 6. File Operations

#### Test 6.1: Save Document
- Click the save button (floppy disk icon)
- Alert should appear

**Expected Result**: Save function is triggered (logs HTML content to console)

#### Test 6.2: Load Document
- Click the open button (folder icon)
- Alert should appear

**Expected Result**: Load function is triggered (logs to console)

### 7. UI/UX Tests

#### Test 7.1: Toolbar Responsiveness
- Resize the window
- Verify toolbar buttons remain accessible

**Expected Result**: Toolbar adapts to window size

#### Test 7.2: Bubble Menu Positioning
- Select text at different positions (top, middle, bottom of document)
- Verify bubble menu appears above selection

**Expected Result**: Bubble menu appears correctly positioned

#### Test 7.3: Editor Scrolling
- Add enough content to require scrolling
- Scroll through the document
- Verify smooth scrolling

**Expected Result**: Editor scrolls smoothly without issues

## Performance Tests

### Test 8.1: Large Document
- Create a document with 1000+ words
- Test editing, formatting, and AI operations
- Verify performance remains acceptable

**Expected Result**: No significant lag or freezing

### Test 8.2: Multiple Tables
- Insert 5+ tables in a document
- Test editing in different tables
- Verify no performance degradation

**Expected Result**: All tables function correctly

## Known Limitations

1. **File Dialog**: Save/Load currently use placeholder alerts - full file picker requires additional Tauri dialog plugin configuration
2. **AI API**: Requires valid API key - replace `YOUR_API_KEY` in `src-tauri/src/lib.rs`
3. **Streaming**: AI responses are not currently streamed - they appear all at once after completion
4. **Slash Menu**: Not yet implemented - would require additional TipTap extensions

## Bug Reporting

If you encounter any issues during testing:

1. Note the specific test case that failed
2. Record the expected vs actual behavior
3. Check the browser console for error messages
4. Check the terminal for Rust/Tauri errors
5. Report with steps to reproduce

## Test Checklist

Use this checklist to track your testing progress:

- [ ] Basic text formatting (bold, italic, strikethrough, code)
- [ ] Headings (H1, H2, H3)
- [ ] Lists (bullet, ordered)
- [ ] Blockquotes
- [ ] Table insertion
- [ ] Table column operations (add, delete, toggle header)
- [ ] Table row operations (add, delete, toggle header)
- [ ] Table cell operations (toggle header)
- [ ] Table column resizing
- [ ] Table deletion
- [ ] AI Polish (with API key)
- [ ] AI Expand (with API key)
- [ ] AI Rewrite (with API key)
- [ ] AI Summarize (with API key)
- [ ] AI Translate (with API key)
- [ ] Keyboard shortcuts (Ctrl/Cmd+S, O, B, I, Z)
- [ ] Markdown export
- [ ] Save document (placeholder)
- [ ] Load document (placeholder)
- [ ] UI responsiveness
- [ ] Bubble menu positioning
- [ ] Large document performance
- [ ] Multiple tables performance

## Next Steps

After completing these tests, consider:

1. Configuring a real API key for full AI testing
2. Implementing streaming AI responses
3. Adding slash menu for quick commands
4. Implementing full file dialog integration
5. Adding PDF export with Typst
