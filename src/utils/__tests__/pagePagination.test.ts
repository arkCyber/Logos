/**
 * Page Pagination Utility Tests
 */

import { describe, expect, it } from 'vitest';
import {
  EMPTY_PAGE_HTML,
  PAGE_BREAK_MARKER,
  PAGE_BREAK_MARKER_AUTO,
  PAGE_BREAK_MARKER_MANUAL,
  PAGE_GAP_PX,
  buildDocumentPlainTextIndex,
  compactPagesByHeight,
  compactPagesLayout,
  computePageBodyHeightPx,
  computePageBodyWidthPx,
  countWordsAndCharsFromPages,
  evaluatePagePaginationConditions,
  findHeadingLocation,
  getPlainTextFromPages,
  htmlExceedsPageHeight,
  isAtomicHtmlBlock,
  isEmptyPageHtml,
  joinPagesToHtml,
  joinPagesToHtmlWithBreaks,
  measureHtmlHeight,
  mergePageIntoNeighbor,
  normalizePageHtml,
  pagesLayoutWouldChange,
  pullContentPrefixUpToHeight,
  reflowPagesByHeight,
  reflowPagesLayout,
  remapHardBreakAfterRemovePage,
  removePageAtIndex,
  resolveGlobalOffsetToPage,
  searchPlainTextDocument,
  splitHtmlByHeight,
  splitHtmlAfterFixedPrefix,
  splitTableHtmlAfterRowIndex,
  splitTableHtmlByRows,
  splitOverflowingTableCellHtml,
  splitTableCellHtmlAfterFixedPrefix,
  containsRichPasteStructure,
  containsImagePasteStructure,
  containsOfficePasteStructure,
  evaluateClipboardHtmlPaste,
  plainTextFileToEditorHtml,
  isTextFileDataTransfer,
  sanitizeRichPasteHtml,
  normalizeTableTheadInHtml,
  tableHtmlHasMergedCells,
  isPasteBulkContent,
  isPasteBulkDropContent,
  PASTE_BULK_CHAR_THRESHOLD,
  PASTE_BULK_DROP_CHAR_THRESHOLD,
  splitHtmlIntoPages,
  splitHtmlIntoPagesWithBreaks,
  splitOverflowingParagraphHtml,
  trimTrailingEmptyPages,
  trimTrailingEmptyPagesLayout,
  isPageNearCapacity,
  peelTrailingEmptyParagraphs
} from '../pagePagination';

describe('pagePagination', () => {
  it('exports Word-style page gap constant', () => {
    expect(PAGE_GAP_PX).toBe(40);
  });

  it('computes A4 body width and height from margins', () => {
    const width = computePageBodyWidthPx(210, 25 * 3.78, 25 * 3.78);
    const height = computePageBodyHeightPx(297, 25 * 3.78, 25 * 3.78);
    expect(width).toBeGreaterThan(600);
    expect(height).toBeGreaterThan(800);
    expect(height).toBeLessThan(980);
  });

  it('normalizes blank html to empty page', () => {
    expect(normalizePageHtml('')).toBe(EMPTY_PAGE_HTML);
    expect(normalizePageHtml('   ')).toBe(EMPTY_PAGE_HTML);
  });

  it('returns empty page array input as single empty page', () => {
    expect(splitHtmlIntoPages('')).toEqual([EMPTY_PAGE_HTML]);
    expect(joinPagesToHtml([])).toBe(EMPTY_PAGE_HTML);
  });

  it('splits and joins pages with break markers (round-trip)', () => {
    const pages = ['<p>Page one</p>', '<p>Page two</p>', '<p>Page three</p>'];
    const combined = joinPagesToHtml(pages);
    expect(combined).toContain('data-logos-page-break="auto"');
    expect(splitHtmlIntoPages(combined)).toEqual(pages);
    expect(combined.match(/data-logos-page-break="auto"/g)?.length).toBe(2);
  });

  it('preserves manual page breaks through join and split round-trip', () => {
    const pages = ['<p>Before break</p>', '<p>After break</p>'];
    const { html } = joinPagesToHtmlWithBreaks(pages, [true]);
    expect(html).toContain(PAGE_BREAK_MARKER_MANUAL);
    const loaded = splitHtmlIntoPagesWithBreaks(html);
    expect(loaded.pages).toEqual(pages);
    expect(loaded.hardBreakAfter).toEqual([true]);
  });

  it('does not compact across manual page breaks', () => {
    const pages = ['<p>Short A</p>', '<p>Short B</p>'];
    const compacted = compactPagesLayout(pages, 500, 400, 4, [true]);
    expect(compacted.pages).toHaveLength(2);
    expect(compacted.hardBreakAfter).toEqual([true]);
  });

  it('reflows segments independently when manual breaks are present', () => {
    const longBlock = Array.from(
      { length: 30 },
      (_, index) => `<p>Segment ${index} ${'word '.repeat(12)}</p>`
    ).join('');
    const layout = reflowPagesLayout(['<p>Intro</p>', longBlock], 500, 120, 4, [true]);
    expect(layout.pages.length).toBeGreaterThan(1);
    expect(layout.hardBreakAfter.some((flag) => flag)).toBe(true);
    expect(layout.pages[0]).toContain('Intro');
  });

  it('reflow produces more pages when printable body height shrinks', () => {
    const longBlock = Array.from(
      { length: 36 },
      (_, index) => `<p>Reflow height ${index} ${'word '.repeat(14)}</p>`
    ).join('');
    const tallBody = reflowPagesLayout([longBlock], 500, 200, 4);
    const shortBody = reflowPagesLayout([longBlock], 500, 120, 4);
    expect(shortBody.pages.length).toBeGreaterThanOrEqual(tallBody.pages.length);
    expect(tallBody.pages.length).toBeGreaterThan(1);
    expect(shortBody.pages.length).toBeGreaterThanOrEqual(tallBody.pages.length);
  });

  it('trims trailing empty pages with hard-break metadata', () => {
    const trimmed = trimTrailingEmptyPagesLayout(
      ['<p>A</p>', '<p><br></p>', '<p><br></p>'],
      [false, false]
    );
    expect(trimmed.pages).toEqual(['<p>A</p>']);
    expect(trimmed.hardBreakAfter).toEqual([]);
  });

  it('classifies legacy true markers as auto breaks', () => {
    const legacy = `<p>A</p>${PAGE_BREAK_MARKER_AUTO.replace('auto', 'true')}<p>B</p>`;
    const loaded = splitHtmlIntoPagesWithBreaks(legacy);
    expect(loaded.pages).toEqual(['<p>A</p>', '<p>B</p>']);
    expect(loaded.hardBreakAfter).toEqual([false]);
  });

  it('web join/split round-trip preserves mixed manual and auto breaks', () => {
    const pages = ['<p>A</p>', '<p>B</p>', '<p>C</p>'];
    const joined = joinPagesToHtmlWithBreaks(pages, [true, false]);
    const loaded = splitHtmlIntoPagesWithBreaks(joined.html);
    expect(loaded.pages).toEqual(pages);
    expect(loaded.hardBreakAfter).toEqual([true, false]);
  });

  it('pagesLayoutWouldChange returns false when manual break blocks compact', () => {
    const pages = ['<p>Short A</p>', '<p>Short B</p>'];
    expect(pagesLayoutWouldChange(pages, 500, 400, 4, [true])).toBe(false);
    expect(pagesLayoutWouldChange(pages, 500, 400, 4, [false])).toBe(true);
  });

  it('remaps hard-break flags after page deletion', () => {
    expect(remapHardBreakAfterRemovePage([false, true], 1)).toEqual([true]);
    expect(remapHardBreakAfterRemovePage([true], 0)).toEqual([]);
    expect(remapHardBreakAfterRemovePage([false, false, true], 2)).toEqual([false, true]);
  });

  it('resolveGlobalOffsetToPage maps cursor positions across page boundaries', () => {
    const index = buildDocumentPlainTextIndex(['<p>AB</p>', '<p>CDE</p>']);
    expect(resolveGlobalOffsetToPage(0, index.pageStarts)).toEqual({ pageIndex: 0, localOffset: 0 });
    expect(resolveGlobalOffsetToPage(2, index.pageStarts)).toEqual({ pageIndex: 0, localOffset: 2 });
    expect(resolveGlobalOffsetToPage(3, index.pageStarts)).toEqual({ pageIndex: 1, localOffset: 0 });
    expect(resolveGlobalOffsetToPage(5, index.pageStarts)).toEqual({ pageIndex: 1, localOffset: 2 });
  });

  it('detects empty pages', () => {
    expect(isEmptyPageHtml('<p><br></p>')).toBe(true);
    expect(isEmptyPageHtml('<p></p>')).toBe(true);
    expect(isEmptyPageHtml('<p>Hello</p>')).toBe(false);
  });

  it('counts words and characters across pages', () => {
    const stats = countWordsAndCharsFromPages(['<p>Hello world</p>', '<p>Second page</p>']);
    expect(stats.wordCount).toBe(4);
    expect(stats.charCount).toBe('Hello world Second page'.length);
  });

  it('supports legacy page-break-container markers', () => {
    const legacy = '<p>A</p><div class="page-break-container"><hr class="page-break"></div><p>B</p>';
    expect(splitHtmlIntoPages(legacy)).toEqual(['<p>A</p>', '<p>B</p>']);
    expect(joinPagesToHtml(['<p>A</p>', '<p>B</p>'])).toContain(PAGE_BREAK_MARKER);
  });

  it('measures html height in jsdom', () => {
    const short = measureHtmlHeight('<p>Short</p>', 400);
    const long = measureHtmlHeight(`<p>${'Word '.repeat(200)}</p>`, 400);
    expect(short).toBeGreaterThan(0);
    expect(long).toBeGreaterThan(short);
  });

  it('detects overflow against max height', () => {
    const html = `<p>${'Overflow content '.repeat(80)}</p>`;
    expect(htmlExceedsPageHeight(html, 400, 40)).toBe(true);
    expect(htmlExceedsPageHeight('<p>Small</p>', 400, 400)).toBe(false);
  });

  it('splits html by height into head and tail', () => {
    const blocks = Array.from({ length: 12 }, (_, index) => `<p>Paragraph ${index + 1} with content.</p>`).join('');
    const split = splitHtmlByHeight(blocks, 500, 120);
    expect(split).not.toBeNull();
    expect(split?.head).toContain('Paragraph 1');
    expect(split?.tail).toContain('Paragraph');
    expect(measureHtmlHeight(split!.head, 500)).toBeLessThanOrEqual(124);
  });

  it('splits a single overflowing paragraph by word boundary', () => {
    const html = `<p>${'Word '.repeat(200)}</p>`;
    const split = splitOverflowingParagraphHtml(html, 400, 80);
    expect(split).not.toBeNull();
    expect(measureHtmlHeight(split!.head, 400)).toBeLessThanOrEqual(84);
    expect(split?.tail).toContain('Word');
  });

  it('peels trailing empty paragraphs for Enter-at-bottom flow', () => {
    const peeled = peelTrailingEmptyParagraphs('<p>Content</p><p><br></p>');
    expect(peeled?.head).toContain('Content');
    expect(peeled?.tail).toContain('<br>');

    const multiBlank = peelTrailingEmptyParagraphs(
      '<p>Content</p><p><br></p><p></p><p><br class="ProseMirror-trailingBreak"></p>'
    );
    expect(multiBlank?.head).toContain('Content');
    expect(multiBlank?.tail.match(/<p/g)?.length).toBeGreaterThanOrEqual(2);
  });

  it('splits overflow only from tail when prefix before caret is fixed', () => {
    const prefix = `<p>Before ${'word '.repeat(8)}</p>`;
    const tail = Array.from(
      { length: 24 },
      (_, index) => `<p>After block ${index} ${'text '.repeat(22)}</p>`
    ).join('');
    const combinedHeight = measureHtmlHeight(`${prefix}${tail}`, 400);
    expect(combinedHeight).toBeGreaterThan(120);
    const split = splitHtmlAfterFixedPrefix(prefix, tail, 400, 120);
    expect(split).not.toBeNull();
    expect(split!.head).toContain('Before');
    expect(split!.tail).toContain('After block');
    expect(measureHtmlHeight(split!.head, 400)).toBeLessThanOrEqual(124);
  });

  it('splits table HTML after a fixed row index for cursor-in-table edits', () => {
    const rows = Array.from(
      { length: 6 },
      (_, index) => `<tr><td>Row ${index} ${'cell '.repeat(12)}</td></tr>`
    ).join('');
    const tableHtml = `<table><tbody>${rows}</tbody></table>`;
    const split = splitTableHtmlAfterRowIndex(tableHtml, 2);
    expect(split).not.toBeNull();
    expect(split!.head).toContain('Row 2');
    expect(split!.tail).toContain('Row 3');
    expect(split!.head).not.toContain('Row 5');
  });

  it('detects bulk paste thresholds for immediate pagination', () => {
    expect(PASTE_BULK_CHAR_THRESHOLD).toBeGreaterThan(100);
    expect(PASTE_BULK_DROP_CHAR_THRESHOLD).toBeLessThan(PASTE_BULK_CHAR_THRESHOLD);
    expect(isPasteBulkContent(10, 50)).toBe(false);
    expect(isPasteBulkContent(80, 250)).toBe(true);
    expect(isPasteBulkContent(60, 50)).toBe(true);
    expect(isPasteBulkDropContent(10, 150)).toBe(true);
    expect(isPasteBulkDropContent(10, 80)).toBe(false);
    expect(isPasteBulkDropContent(60, 50)).toBe(true);
  });

  it('detects merged cells in table HTML', () => {
    expect(tableHtmlHasMergedCells('<table><tr><td colspan="2">A</td></tr></table>')).toBe(true);
    expect(tableHtmlHasMergedCells('<table><tr><td>A</td><td>B</td></tr></table>')).toBe(false);
  });

  it('detects near-capacity pages', () => {
    const tall = `<p>${'Near full '.repeat(60)}</p>`;
    expect(isPageNearCapacity(tall, 400, 80, 0.5)).toBe(true);
    expect(isPageNearCapacity('<p>Short</p>', 400, 400, 0.5)).toBe(false);
  });

  it('reflows pages when combined content exceeds height', () => {
    const longPage = Array.from(
      { length: 40 },
      (_, index) => `<p>Line ${index + 1} with enough text to consume vertical space.</p>`
    ).join('');
    const reflowed = reflowPagesByHeight([longPage], 500, 120);
    expect(reflowed.length).toBeGreaterThan(1);
    reflowed.forEach((page) => {
      expect(measureHtmlHeight(page, 500)).toBeLessThanOrEqual(124);
    });
  });

  it('pulls prefix content that fits remaining page space', () => {
    const nextPage = Array.from(
      { length: 6 },
      (_, index) => `<p>Next block ${index + 1} with text.</p>`
    ).join('');
    const pulled = pullContentPrefixUpToHeight(nextPage, 500, 45);
    expect(pulled).not.toBeNull();
    expect(pulled!.head).toContain('Next block 1');
    expect(measureHtmlHeight(pulled!.head, 500)).toBeLessThanOrEqual(49);
    if (pulled!.tail) {
      expect(pulled!.tail).toContain('Next block');
    } else {
      expect(measureHtmlHeight(nextPage, 500)).toBeLessThanOrEqual(49);
    }
  });

  it('compacts pages after delete frees space on earlier pages', () => {
    const pageOne = `<p>${'First page content. '.repeat(8)}</p>`;
    const pageTwo = Array.from(
      { length: 10 },
      (_, index) => `<p>Second page block ${index + 1} ${'word '.repeat(10)}</p>`
    ).join('');
    const pages = reflowPagesByHeight([`${pageOne}${pageTwo}`], 500, 160);
    expect(pages.length).toBeGreaterThan(1);

    const shrunkFirst = '<p>Short</p>';
    const compacted = compactPagesByHeight([shrunkFirst, ...pages.slice(1)], 500, 160);
    expect(compacted.length).toBeLessThan(pages.length);
    const combinedText = compacted.join('');
    expect(combinedText).toContain('Short');
    expect(combinedText).toContain('Second page block');
  });

  it('detects when compacting would change stored page layout', () => {
    const pages = ['<p>A</p>', '<p>B</p>'];
    expect(pagesLayoutWouldChange(pages, 500, 400)).toBe(true);
    expect(pagesLayoutWouldChange(['<p>A only</p>'], 500, 400)).toBe(false);
  });

  it('evaluates pagination conditions for overflow and enter-at-end', () => {
    const tall = `<p>${'Overflow '.repeat(80)}</p>`;
    const overflow = evaluatePagePaginationConditions(tall, 400, 80);
    expect(overflow.shouldSplitOverflow).toBe(true);
    expect(overflow.hasOverflow).toBe(true);

    const nearFull = `<p>${'Near full '.repeat(60)}</p>`;
    const enterAtEnd = evaluatePagePaginationConditions(nearFull, 400, 80, null, {
      enterAtDocumentEnd: true
    });
    expect(enterAtEnd.shouldContinueOnNextPageAfterEnter).toBe(true);

    const partial = '<p>Only a few words on the page.</p>';
    const partialEnter = evaluatePagePaginationConditions(partial, 400, 400, null, {
      enterAtDocumentEnd: true
    });
    expect(partialEnter.hasOverflow).toBe(false);
    expect(partialEnter.isNearCapacity).toBe(false);
    expect(partialEnter.shouldContinueOnNextPageAfterEnter).toBe(false);
  });

  it('trims trailing empty pages but keeps minimum page count', () => {
    expect(trimTrailingEmptyPages(['<p>A</p>', '<p><br></p>', '<p><br></p>'])).toEqual(['<p>A</p>']);
    expect(trimTrailingEmptyPages(['<p><br></p>', '<p><br></p>'])).toEqual(['<p><br></p>']);
    expect(trimTrailingEmptyPages(['<p>A</p>', '<p><br></p>'], 2)).toEqual(['<p>A</p>', '<p><br></p>']);
  });

  it('handles repeated split calls without regex state bugs', () => {
    const combined = joinPagesToHtml(['<p>1</p>', '<p>2</p>']);
    expect(splitHtmlIntoPages(combined)).toEqual(['<p>1</p>', '<p>2</p>']);
    expect(splitHtmlIntoPages(combined)).toEqual(['<p>1</p>', '<p>2</p>']);
  });

  it('extracts plain text across pages', () => {
    const text = getPlainTextFromPages(['<p>Hello</p>', '<p>World</p>']);
    expect(text).toContain('Hello');
    expect(text).toContain('World');
  });

  it('removes a page by index while keeping minimum one page', () => {
    expect(removePageAtIndex(['<p>A</p>', '<p>B</p>', '<p>C</p>'], 1)).toEqual(['<p>A</p>', '<p>C</p>']);
    expect(removePageAtIndex(['<p>Only</p>'], 0)).toEqual(['<p>Only</p>']);
  });

  it('merges deleted page into neighbor', () => {
    const result = mergePageIntoNeighbor(['<p>A</p>', '<p>B</p>', '<p>C</p>'], 1);
    expect(result.activeIndex).toBe(0);
    expect(result.pages).toHaveLength(2);
    expect(result.pages[0]).toContain('A');
    expect(result.pages[0]).toContain('B');
    expect(result.pages[1]).toBe('<p>C</p>');
  });

  it('merges first page into second when deleting page one', () => {
    const result = mergePageIntoNeighbor(['<p>First</p>', '<p>Second</p>'], 0);
    expect(result.activeIndex).toBe(0);
    expect(result.pages).toHaveLength(1);
    expect(result.pages[0]).toContain('First');
    expect(result.pages[0]).toContain('Second');
  });

  it('builds stable plain-text index for cross-page search', () => {
    const index = buildDocumentPlainTextIndex(['<p>Hello</p>', '<p>World</p>']);
    expect(index.text).toBe('Hello\nWorld');
    expect(index.pageStarts).toEqual([0, 6, 11]);
    expect(resolveGlobalOffsetToPage(7, index.pageStarts)).toEqual({
      pageIndex: 1,
      localOffset: 1
    });
  });

  it('splits tables with rowspan at row boundaries', () => {
    const rows = Array.from(
      { length: 10 },
      (_, index) =>
        index === 0
          ? `<tr><td rowspan="4">Merged header</td><td>Row ${index} ${'x '.repeat(8)}</td></tr>`
          : `<tr><td>Row ${index} ${'x '.repeat(8)}</td></tr>`
    ).join('');
    const table = `<table><tbody>${rows}</tbody></table>`;
    const split = splitTableHtmlByRows(table, 500, 60);
    expect(split).not.toBeNull();
    expect(split!.head).toContain('rowspan');
    expect(split!.tail).toContain('<tr>');
  });

  it('splits tables with colspan and rowspan merged cells', () => {
    const rows = [
      `<tr><td colspan="2" rowspan="3">Section title</td><td>Col A ${'x '.repeat(10)}</td></tr>`,
      `<tr><td>Col B ${'x '.repeat(10)}</td></tr>`,
      `<tr><td>Col C ${'x '.repeat(10)}</td></tr>`,
      ...Array.from(
        { length: 10 },
        (_, index) => `<tr><td colspan="2">Row ${index + 3} ${'cell '.repeat(12)}</td><td>Extra</td></tr>`
      )
    ].join('');
    const table = `<table><tbody>${rows}</tbody></table>`;
    const split = splitTableHtmlByRows(table, 500, 100);
    expect(split).not.toBeNull();
    expect(split!.head).toMatch(/colspan|rowspan/);
    expect(split!.tail).toContain('<tr>');
    expect(split!.head.match(/<tr/g)?.length).toBeLessThan(rows.match(/<tr/g)?.length || 0);
  });

  it('identifies atomic html blocks for keep-together pagination', () => {
    expect(isAtomicHtmlBlock(document.createElement('table'))).toBe(false);
    expect(isAtomicHtmlBlock(document.createElement('img'))).toBe(true);
    expect(isAtomicHtmlBlock(document.createElement('figure'))).toBe(true);
    expect(isAtomicHtmlBlock(document.createElement('p'))).toBe(false);

    const wrapper = document.createElement('div');
    wrapper.innerHTML = '<img src="test.png" alt="test">';
    expect(isAtomicHtmlBlock(wrapper)).toBe(true);
  });

  it('splits oversized tables by row boundaries', () => {
    const rows = Array.from(
      { length: 14 },
      (_, index) => `<tr><td>Row ${index} ${'cell '.repeat(12)}</td></tr>`
    ).join('');
    const table = `<table><tbody>${rows}</tbody></table>`;
    const split = splitTableHtmlByRows(table, 500, 140);

    expect(split).not.toBeNull();
    expect(split!.head).toContain('<tr>');
    expect(split!.tail).toContain('<tr>');
    expect(split!.head.match(/<tr/g)?.length).toBeLessThan(rows.match(/<tr/g)?.length || 0);
  });

  it('keeps small tables on one page or moves them intact', () => {
    const paragraphs = Array.from(
      { length: 8 },
      (_, index) => `<p>P${index} ${'word '.repeat(24)}</p>`
    ).join('');
    const html = `${paragraphs}<table><tr><td>Table cell content</td></tr></table><p>After table</p>`;
    const split = splitHtmlByHeight(html, 500, 180);

    expect(split).not.toBeNull();
    const headHasTable = split!.head.includes('<table');
    const tailHasTable = split!.tail.includes('<table');
    expect(headHasTable && tailHasTable).toBe(false);

    if (headHasTable) {
      expect(split!.head).toContain('</table>');
    } else if (tailHasTable) {
      expect(split!.tail).toContain('</table>');
    }
  });

  it('moves standalone images to the next page when preceding content fills the page', () => {
    const paragraphs = Array.from(
      { length: 12 },
      (_, index) => `<p>P${index} ${'word '.repeat(24)}</p>`
    ).join('');
    const img =
      '<img src="data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%22400%22 height=%22200%22/%3E" width="400" height="200" alt="block" />';
    const html = `${paragraphs}${img}<p>After image</p>`;
    const split = splitHtmlByHeight(html, 500, 180);

    expect(split).not.toBeNull();
    const headHasImg = split!.head.includes('<img');
    const tailHasImg = split!.tail.includes('<img');
    expect(headHasImg && tailHasImg).toBe(false);
    if (tailHasImg) {
      expect(split!.head).not.toContain('<img');
    }
  });

  it('finds heading location across pages', () => {
    const pages = ['<h1>Title</h1><p>Intro</p>', '<h2>Section</h2><p>Body</p>'];
    expect(findHeadingLocation(pages, 0)).toEqual({
      pageIndex: 0,
      localHeadingIndex: 0,
      headingText: 'Title'
    });
    expect(findHeadingLocation(pages, 1)).toEqual({
      pageIndex: 1,
      localHeadingIndex: 0,
      headingText: 'Section'
    });
    expect(findHeadingLocation(pages, 99)).toBeNull();
  });

  it('aligns getPlainTextFromPages with buildDocumentPlainTextIndex', () => {
    const pages = ['<p>Alpha</p>', '<p>Beta</p>', '<p>Gamma</p>'];
    const index = buildDocumentPlainTextIndex(pages);
    expect(getPlainTextFromPages(pages)).toBe(index.text.trim());
    expect(index.segments).toHaveLength(3);
  });

  it('resolves global offset at page boundaries', () => {
    const index = buildDocumentPlainTextIndex(['<p>AB</p>', '<p>CD</p>']);
    expect(resolveGlobalOffsetToPage(0, index.pageStarts)).toEqual({ pageIndex: 0, localOffset: 0 });
    expect(resolveGlobalOffsetToPage(2, index.pageStarts)).toEqual({ pageIndex: 0, localOffset: 2 });
    expect(resolveGlobalOffsetToPage(3, index.pageStarts)).toEqual({ pageIndex: 1, localOffset: 0 });
    expect(resolveGlobalOffsetToPage(5, index.pageStarts)).toEqual({ pageIndex: 1, localOffset: 2 });
  });

  it('searches plain text across page boundaries in the browser', () => {
    const index = buildDocumentPlainTextIndex([
      '<p>Alpha</p>',
      '<p>boundary-search-target-2</p>',
      '<p>boundary-search-target-3</p>'
    ]);
    const result = searchPlainTextDocument(index.text, 'boundary-search-target-3', {
      case_sensitive: false,
      whole_word: false,
      use_regex: false
    });
    expect(result?.total_count).toBe(1);
    expect(result?.matches[0].position).toBeGreaterThan(0);
    expect(resolveGlobalOffsetToPage(result!.matches[0].position, index.pageStarts).pageIndex).toBe(2);
  });

  it('reflows long documents with embedded tables into multiple pages', () => {
    const tableRows = Array.from(
      { length: 8 },
      (_, index) => `<tr><td>Cell ${index} ${'data '.repeat(10)}</td></tr>`
    ).join('');
    const longPage = [
      `<p>${'Intro paragraph. '.repeat(20)}</p>`,
      `<table><tbody>${tableRows}</tbody></table>`,
      `<p>${'Outro paragraph. '.repeat(20)}</p>`
    ].join('');
    const reflowed = reflowPagesByHeight([longPage], 500, 160);
    expect(reflowed.length).toBeGreaterThan(1);
    const combined = reflowed.join('');
    expect(combined).toContain('<table');
    expect(combined).toContain('Intro paragraph');
    expect(combined).toContain('Outro paragraph');
  });

  it('preserves multi-page content through join and split round-trip', () => {
    const pages = [
      '<p>Page A content</p>',
      '<p>Page B with table</p><table><tr><td>X</td></tr></table>',
      '<p>Page C ending</p>'
    ];
    const combined = joinPagesToHtml(pages);
    expect(splitHtmlIntoPages(combined)).toEqual(pages);
    expect(buildDocumentPlainTextIndex(pages).text).toContain('Page A');
    expect(buildDocumentPlainTextIndex(pages).text).toContain('Page C ending');
  });

  it('handles delete and merge workflow without losing text', () => {
    const pages = ['<p>First</p>', '<p>Second</p>', '<p>Third</p>'];
    const afterRemove = removePageAtIndex(pages, 1);
    expect(afterRemove).toEqual(['<p>First</p>', '<p>Third</p>']);
    const merged = mergePageIntoNeighbor(afterRemove, 0);
    expect(merged.pages).toHaveLength(1);
    expect(merged.pages[0]).toContain('First');
    expect(merged.pages[0]).toContain('Third');
  });

  it('splits overflowing single-cell table content at paragraph boundaries', () => {
    const cellParagraphs = Array.from(
      { length: 24 },
      (_, index) => `<p>Cell line ${index} ${'word '.repeat(28)}</p>`
    ).join('');
    const table = `<table><tbody><tr><td>${cellParagraphs}</td></tr></tbody></table>`;
    expect(tableHtmlHasMergedCells(table)).toBe(false);

    const split = splitOverflowingTableCellHtml(table, 500, 120);
    expect(split).not.toBeNull();
    expect(split!.head).toContain('<table');
    expect(split!.tail).toContain('<table');
    expect(split!.head).toContain('Cell line 0');
    expect(split!.tail).toMatch(/Cell line \d+/);
    expect(split!.head.match(/Cell line/g)?.length).toBeLessThan(
      cellParagraphs.match(/Cell line/g)?.length || 0
    );
  });

  it('returns null for single-cell split when merged cells are present', () => {
    const table =
      '<table><tbody><tr><td rowspan="2">Merged</td><td>Data</td></tr></tbody></table>';
    expect(tableHtmlHasMergedCells(table)).toBe(true);
    expect(splitOverflowingTableCellHtml(table, 500, 80)).toBeNull();
  });

  it('detects rich paste structure and sanitizes unsafe HTML', () => {
    const richHtml =
      '<p>Intro</p><table><tr><td>Cell</td></tr></table><img src="x.png" alt="x" onerror="alert(1)">';
    expect(containsRichPasteStructure(richHtml)).toBe(true);
    expect(containsRichPasteStructure('<p>Plain only</p>')).toBe(false);

    const sanitized = sanitizeRichPasteHtml(
      '<script>alert(1)</script><p onclick="bad()">Safe text</p><style>.x{}</style>'
    );
    expect(sanitized).not.toMatch(/script|onclick|<style/i);
    expect(sanitized).toContain('Safe text');
  });

  it('evaluates clipboard HTML paste interception decisions', () => {
    const plain = evaluateClipboardHtmlPaste('');
    expect(plain.shouldIntercept).toBe(false);

    const safeRich = evaluateClipboardHtmlPaste('<table><tr><td>Cell</td></tr></table>');
    expect(safeRich.shouldIntercept).toBe(true);
    expect(safeRich.richPaste).toBe(true);

    const unsafe = evaluateClipboardHtmlPaste(
      '<script>alert(1)</script><p onclick="bad()">Text</p><table><tr><td>Cell</td></tr></table>'
    );
    expect(unsafe.shouldIntercept).toBe(true);
    expect(unsafe.wasSanitized).toBe(true);
    expect(unsafe.sanitizedHtml).not.toMatch(/script|onclick/i);
    expect(unsafe.sanitizedHtml).toContain('Cell');

    const benign = evaluateClipboardHtmlPaste('<p>Hello world</p>');
    expect(benign.shouldIntercept).toBe(false);
    expect(benign.richPaste).toBe(false);
  });

  it('detects and sanitizes Microsoft Word paste markup', () => {
    const wordHtml =
      '<html xmlns:o="urn:schemas-microsoft-com:office:office">' +
      '<body><!--StartFragment--><p class=MsoNormal>Word line</p><o:p></o:p><!--EndFragment--></body></html>';
    expect(containsOfficePasteStructure(wordHtml)).toBe(true);

    const decision = evaluateClipboardHtmlPaste(wordHtml);
    expect(decision.shouldIntercept).toBe(true);
    expect(decision.officePaste).toBe(true);
    expect(decision.sanitizedHtml).toContain('Word line');
    expect(decision.sanitizedHtml).not.toMatch(/MsoNormal|StartFragment|<o:p/i);
  });

  it('maps Word headings, lists, and preserves bold/italic inline styles', () => {
    const wordHtml =
      '<p class=MsoHeading1><span style="font-weight:bold;font-size:16.0pt;mso-fareast-font-family:Calibri">Title</span></p>' +
      '<p class=MsoNormal><span style="font-weight:bold">Bold</span> and <i>italic</i></p>' +
      '<p class=MsoListParagraph><span style="mso-list:Ignore">·</span>First item</p>' +
      '<p class=MsoListParagraph><span style="mso-list:Ignore">·</span>Second item</p>';
    const sanitized = sanitizeRichPasteHtml(wordHtml);

    expect(sanitized).toMatch(/<h1[\s>]/i);
    expect(sanitized).toContain('Title');
    expect(sanitized).toMatch(/<ul[\s>]/i);
    expect(sanitized).toMatch(/<li[\s>]/i);
    expect(sanitized).toContain('First item');
    expect(sanitized).toContain('Second item');
    expect(sanitized).toContain('Bold');
    expect(sanitized).toMatch(/font-weight:\s*bold/i);
    expect(sanitized).toMatch(/<i>italic<\/i>/i);
    expect(sanitized).not.toMatch(/MsoHeading|MsoListParagraph|mso-list/i);
  });

  it('maps Word alpha and roman ordered lists to ol type attributes', () => {
    const alphaHtml =
      '<p class=MsoListParagraph><span style="mso-list:Ignore">a.</span>Alpha one</p>' +
      '<p class=MsoListParagraph><span style="mso-list:Ignore">b.</span>Alpha two</p>';
    const alphaSanitized = sanitizeRichPasteHtml(alphaHtml);
    expect(alphaSanitized).toMatch(/<ol[^>]*type="a"/i);
    expect(alphaSanitized).toContain('Alpha one');

    const romanHtml =
      '<p class=MsoListParagraph><span style="mso-list:Ignore">i.</span>Roman one</p>' +
      '<p class=MsoListParagraph><span style="mso-list:Ignore">ii.</span>Roman two</p>';
    const romanSanitized = sanitizeRichPasteHtml(romanHtml);
    expect(romanSanitized).toMatch(/<ol[^>]*type="i"/i);
    expect(romanSanitized).toContain('Roman two');

    const decimalHtml =
      '<p class=MsoListParagraph><span style="mso-list:Ignore">1.</span>One</p>' +
      '<p class=MsoListParagraph><span style="mso-list:Ignore">2.</span>Two</p>';
    const decimalSanitized = sanitizeRichPasteHtml(decimalHtml);
    expect(decimalSanitized).toMatch(/<ol[\s>]/i);
    expect(decimalSanitized).not.toMatch(/type="[aAiI]"/i);
  });

  it('preserves Word table borders and cell padding styles', () => {
    const tableHtml =
      '<table class=MsoTableGrid border=1 cellspacing=0 cellpadding=0>' +
      '<tr><td style="border:solid 1.0pt;padding:0in 5.4pt;mso-border-alt:solid">Cell A</td></tr>' +
      '</table>';
    const sanitized = sanitizeRichPasteHtml(tableHtml);
    expect(sanitized).toContain('<tbody>');
    expect(sanitized).toContain('Cell A');
    expect(sanitized).toMatch(/border/i);
    expect(sanitized).toMatch(/padding/i);
    expect(sanitized).not.toMatch(/MsoTableGrid|mso-border/i);
  });

  it('converts plain text file content to editor paragraphs', () => {
    const html = plainTextFileToEditorHtml('Line one\nLine two\n\nSecond paragraph');
    expect(html).toContain('<p>Line one<br>Line two</p>');
    expect(html).toContain('<p>Second paragraph</p>');
    expect(plainTextFileToEditorHtml('<script>alert(1)</script>')).toContain('&lt;script&gt;');
  });

  it('detects text file data transfer payloads', () => {
    const dataTransfer = {
      files: [{ name: 'notes.txt', type: 'text/plain', text: async () => 'hello' }]
    } as unknown as DataTransfer;
    expect(isTextFileDataTransfer(dataTransfer)).toBe(true);
    expect(isTextFileDataTransfer({ files: [] } as unknown as DataTransfer)).toBe(false);
  });

  it('splits single-row overflowing tables via cell paragraph split during reflow', () => {
    const cellParagraphs = Array.from(
      { length: 24 },
      (_, index) => `<p>Reflow cell ${index} ${'word '.repeat(28)}</p>`
    ).join('');
    const html = `<p>${'Intro '.repeat(40)}</p><table><tbody><tr><td>${cellParagraphs}</td></tr></tbody></table>`;
    const reflowed = reflowPagesByHeight([html], 500, 160);
    expect(reflowed.length).toBeGreaterThan(1);
    const combined = reflowed.join('');
    expect(combined).toMatch(/Reflow cell 0/);
    expect(combined).toMatch(/Reflow cell 2[0-9]/);
    expect(reflowed.filter((page) => page.includes('<table')).length).toBeGreaterThanOrEqual(2);
  });

  it('repeats thead on tail fragment when splitting rowspan tables', () => {
    const rows = Array.from({ length: 12 }, (_, index) =>
      index === 0
        ? `<tr><td rowspan="4">Merged</td><td>Row ${index} ${'x '.repeat(12)}</td></tr>`
        : `<tr><td>Row ${index} ${'x '.repeat(12)}</td></tr>`
    ).join('');
    const table = `<table><thead><tr><th>H1</th><th>H2</th></tr></thead><tbody>${rows}</tbody></table>`;
    const split = splitTableHtmlByRows(table, 500, 90);
    expect(split).not.toBeNull();
    expect(split!.head).toContain('<thead');
    expect(split!.tail).toContain('<thead');
    expect(split!.head).toMatch(/rowspan/i);
  });

  it('promotes TipTap first th row into thead for continuation splits', () => {
    const dataRows = Array.from(
      { length: 10 },
      (_, index) => `<tr><td>Data ${index} ${'cell '.repeat(14)}</td><td>More ${'x '.repeat(10)}</td></tr>`
    ).join('');
    const tipTapTable =
      '<table><tbody>' +
      '<tr><th>Header A</th><th>Header B</th></tr>' +
      dataRows +
      '</tbody></table>';
    const normalized = normalizeTableTheadInHtml(tipTapTable);
    expect(normalized).toContain('<thead');
    expect(normalized).toContain('<th>Header A</th>');
    const split = splitTableHtmlByRows(normalized, 500, 90);
    expect(split).not.toBeNull();
    expect(split!.tail).toContain('<thead');
  });

  it('splits table cell content at caret prefix inside a cell', () => {
    const cellHtml =
      `<p>Before anchor</p><p>Anchor tail ${'word '.repeat(40)}</p>` +
      Array.from({ length: 20 }, (_, index) => `<p>Line ${index} ${'word '.repeat(32)}</p>`).join('');
    const table = `<table><tbody><tr><td>${cellHtml}</td></tr></tbody></table>`;
    const prefix = '<p>Before anchor</p><p>Anchor ';
    const tail =
      `<p>tail ${'word '.repeat(40)}</p>` +
      Array.from({ length: 20 }, (_, index) => `<p>Line ${index} ${'word '.repeat(32)}</p>`).join('');
    const split = splitTableCellHtmlAfterFixedPrefix(table, 0, 0, prefix, tail, 500, 120);
    expect(split).not.toBeNull();
    expect(split!.head).toContain('Before anchor');
    expect(split!.head).toContain('Anchor');
    expect(split!.tail).toMatch(/tail|Line/);
    expect(split!.head).toContain('<table');
    expect(split!.tail).toContain('<table');
  });

  it('detects image paste structure and sanitizes unsafe figure HTML', () => {
    const html = '<figure><img src="x.png" alt="x" onerror="alert(1)"></figure>';
    expect(containsImagePasteStructure(html)).toBe(true);
    expect(containsRichPasteStructure(html)).toBe(true);
    const sanitized = sanitizeRichPasteHtml(`${html}<script>x</script>`);
    expect(sanitized).not.toMatch(/script|onerror/i);
    expect(sanitized).toContain('<img');
  });

  it('normalizePageHtml normalizes plain tables without thead promotion', () => {
    const plain = '<p>Intro</p><table><tr><td>X</td></tr></table>';
    const normalized = normalizePageHtml(plain);
    expect(normalized).toContain('<td>X</td>');
    expect(normalized).not.toContain('<thead');
  });

  it('applyTableLegacyAttributes maps cellpadding to cell padding styles', async () => {
    const { applyTableLegacyAttributes } = await import('../pagePagination');
    const container = document.createElement('div');
    container.innerHTML =
      '<table border="1" cellpadding="10"><tr><td>Cell</td><th>Head</th></tr></table>';
    const table = container.querySelector('table') as HTMLTableElement;
    expect(applyTableLegacyAttributes(table)).toBe(true);
    expect(table.querySelector('td')?.getAttribute('style')).toMatch(/padding:\s*10px/i);
    expect(table.querySelector('th')?.getAttribute('style')).toMatch(/padding:\s*10px/i);
  });

  it('prepareHtmlForEditorInsert expands table legacy attributes for TipTap insert', async () => {
    const { prepareHtmlForEditorInsert, tableHtmlNeedsEditorNormalization } = await import(
      '../pagePagination'
    );
    const raw =
      '<table border="1" cellpadding="12"><tbody><tr><td>Cell</td></tr></tbody></table>';
    expect(tableHtmlNeedsEditorNormalization(raw)).toBe(true);
    const prepared = prepareHtmlForEditorInsert(raw);
    expect(prepared).toMatch(/padding:\s*12px/i);
    expect(prepared).toContain('Cell');
  });

  it('applyDefaultTableBorderAttribute adds border="1" to borderless tables', async () => {
    const { applyDefaultTableBorderAttribute, prepareInsertedTableHtml } = await import(
      '../pagePagination'
    );
    const raw = '<table><tbody><tr><td>No border</td></tr></tbody></table>';
    expect(applyDefaultTableBorderAttribute(raw)).toMatch(/border="1"/);
    const prepared = prepareInsertedTableHtml(raw);
    expect(prepared).toMatch(/border:\s*1px/i);
    expect(prepared).toContain('No border');
  });

  it('prepareInsertedTableHtml expands cellspacing into border-spacing styles', async () => {
    const { prepareInsertedTableHtml } = await import('../pagePagination');
    const raw =
      '<table border="1" cellspacing="6"><tbody><tr><td>Spaced</td></tr></tbody></table>';
    const prepared = prepareInsertedTableHtml(raw);
    expect(prepared).toMatch(/border-spacing:\s*6px/i);
    expect(prepared).toContain('Spaced');
  });

  it('resyncEditorHtmlAfterTableInsert updates borderless insertTable HTML in editor', async () => {
    const { resyncEditorHtmlAfterTableInsert } = await import('../pagePagination');
    const raw =
      '<table><tbody><tr><th><p>H</p></th></tr><tr><td><p>Cell</p></td></tr></tbody></table>';
    let currentHtml = raw;
    const editorStub = {
      getHTML: () => currentHtml,
      commands: {
        setContent: (html: string) => {
          currentHtml = html;
          return true;
        }
      },
      isDestroyed: false
    };

    expect(resyncEditorHtmlAfterTableInsert(editorStub)).toBe(true);
    expect(currentHtml).toMatch(/border="1"/);
    expect(currentHtml).toMatch(/border:\s*1px/i);
    expect(resyncEditorHtmlAfterTableInsert(editorStub)).toBe(false);
  });

  it('runInsertTableWithResync inserts table and applies Word-style border resync', async () => {
    const { runInsertTableWithResync } = await import('../pagePagination');
    let currentHtml = '<p>Before</p>';
    const editorStub = {
      chain: () => ({
        focus: () => ({
          insertTable: () => ({
            run: () => {
              currentHtml =
                '<p>Before</p><table><tbody><tr><th><p>H</p></th></tr><tr><td><p>Cell</p></td></tr></tbody></table>';
              return true;
            }
          })
        })
      }),
      getHTML: () => currentHtml,
      commands: {
        setContent: (html: string) => {
          currentHtml = html;
          return true;
        }
      },
      isDestroyed: false
    };

    expect(runInsertTableWithResync(editorStub, { rows: 2, cols: 2, withHeaderRow: true })).toBe(true);
    expect(currentHtml).toMatch(/border="1"/);
    expect(currentHtml).toMatch(/border:\s*1px/i);
  });

  it('insertPreparedHtmlIntoEditor normalizes table HTML before insertContent', async () => {
    const { insertPreparedHtmlIntoEditor } = await import('../pagePagination');
    const inserted: string[] = [];
    const editorStub = {
      chain: () => ({
        focus: () => ({
          insertContent: (html: string) => {
            inserted.push(html);
            return { run: () => true };
          }
        })
      }),
      isDestroyed: false
    };
    const raw =
      '<table border="1" cellpadding="8"><tbody><tr><td>Padded</td></tr></tbody></table>';
    expect(insertPreparedHtmlIntoEditor(editorStub, raw)).toBe(true);
    expect(inserted[0]).toMatch(/padding:\s*8px/i);
  });

  it('insertSanitizedClipboardPaste inserts only when interception is recommended', async () => {
    const { insertSanitizedClipboardPaste, evaluateClipboardHtmlPaste } = await import(
      '../pagePagination'
    );
    const inserted: string[] = [];
    const editorStub = {
      chain: () => ({
        focus: () => ({
          insertContent: (html: string) => {
            inserted.push(html);
            return { run: () => true };
          }
        })
      }),
      isDestroyed: false
    };
    const decision = evaluateClipboardHtmlPaste(
      '<script>x</script><table><tr><td>Safe</td></tr></table>'
    );
    expect(insertSanitizedClipboardPaste(editorStub, decision)).toBe(true);
    expect(inserted[0]).toContain('Safe');
    expect(inserted[0]).not.toContain('<script');
    expect(
      insertSanitizedClipboardPaste(editorStub, {
        shouldIntercept: false,
        sanitizedHtml: '<p>plain</p>'
      })
    ).toBe(false);
  });

  it('insertSanitizedClipboardPaste supports docPos for anchor insertion', async () => {
    const { insertSanitizedClipboardPaste, evaluateClipboardHtmlPaste } = await import(
      '../pagePagination'
    );
    const inserted: Array<{ html: string; docPos?: number }> = [];
    const editorStub = {
      chain: () => ({
        focus: () => ({
          setTextSelection: (docPos: number) => ({
            insertContent: (html: string) => {
              inserted.push({ html, docPos });
              return { run: () => true };
            }
          }),
          insertContent: (html: string) => {
            inserted.push({ html });
            return { run: () => true };
          }
        })
      }),
      isDestroyed: false
    };
    const decision = evaluateClipboardHtmlPaste(
      '<table><tr><td>AnchorCell</td></tr></table>'
    );
    expect(insertSanitizedClipboardPaste(editorStub, decision, { docPos: 42 })).toBe(true);
    expect(inserted[0]?.docPos).toBe(42);
    expect(inserted[0]?.html).toContain('AnchorCell');
  });

  it('insertPlainTextFileHtmlIntoEditor converts paragraphs before insert', async () => {
    const { insertPlainTextFileHtmlIntoEditor } = await import('../pagePagination');
    const inserted: string[] = [];
    const editorStub = {
      chain: () => ({
        focus: () => ({
          insertContent: (html: string) => {
            inserted.push(html);
            return { run: () => true };
          }
        })
      }),
      isDestroyed: false
    };
    expect(insertPlainTextFileHtmlIntoEditor(editorStub, 'Line one\nLine two')).toBe(true);
    expect(inserted[0]).toContain('Line one');
    expect(inserted[0]).toContain('Line two');
    expect(inserted[0]).toMatch(/<p>/);
  });

  it('applyClipboardPastePayload sanitizes rich HTML or falls back to plain text', async () => {
    const { applyClipboardPastePayload, evaluateClipboardHtmlPaste } = await import(
      '../pagePagination'
    );
    const inserted: string[] = [];
    const editorStub = {
      chain: () => ({
        focus: () => ({
          insertContent: (content: string) => {
            inserted.push(content);
            return { run: () => true };
          }
        })
      }),
      isDestroyed: false
    };
    const richDecision = evaluateClipboardHtmlPaste(
      '<script>x</script><table><tr><td>MenuPaste</td></tr></table>'
    );
    const richResult = applyClipboardPastePayload(editorStub, {
      ...richDecision,
      plainText: 'fallback'
    });
    expect(richResult.handled).toBe(true);
    expect(richResult.sanitized).toBe(true);
    expect(richResult.bulkSuggested).toBe(true);
    expect(inserted[0]).toContain('MenuPaste');

    inserted.length = 0;
    const plainResult = applyClipboardPastePayload(editorStub, {
      shouldIntercept: false,
      sanitizedHtml: '',
      imagePaste: false,
      richPaste: false,
      officePaste: false,
      wasSanitized: false,
      plainText: 'Plain menu paste'
    });
    expect(plainResult.handled).toBe(true);
    expect(plainResult.sanitized).toBe(false);
    expect(inserted[0]).toBe('Plain menu paste');
  });

  it('normalizePageHtml promotes header rows when persisting page snapshots', () => {
    const tipTapTable =
      '<table><tbody><tr><th>H1</th><th>H2</th></tr><tr><td>A</td><td>B</td></tr></tbody></table>';
    const normalized = normalizePageHtml(tipTapTable);
    expect(normalized).toContain('<thead');
    expect(normalized).toContain('<td>A</td>');
  });
});
