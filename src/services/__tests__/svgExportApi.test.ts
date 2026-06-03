/**
 * svgExportApi unit tests
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import {
  buildSvgExportConfig,
  decodeBase64ToText,
  decodeBase64ToBytes,
  exportHtmlToSvg,
  exportTypstToSvg,
  previewTypstSvgFromHtml,
  exportToSvg,
  createSvgObjectUrl,
  validateSvgStructure,
} from '../svgExportApi';

const invokeMock = vi.fn();

vi.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: unknown[]) => invokeMock(...args),
}));

vi.mock('@tauri-apps/plugin-dialog', () => ({
  save: vi.fn(),
}));

describe('svgExportApi', () => {
  beforeEach(() => {
    invokeMock.mockReset();
  });

  it('validateSvgStructure accepts well-formed SVG', () => {
    const svg = '<?xml version="1.0"?><svg xmlns="http://www.w3.org/2000/svg"></svg>';
    expect(validateSvgStructure(svg)).toBe(true);
  });

  it('validateSvgStructure rejects malformed SVG', () => {
    expect(validateSvgStructure('<div>not svg</div>')).toBe(false);
  });

  it('buildSvgExportConfig uses svg format', () => {
    const config = buildSvgExportConfig('Test');
    expect(config.format).toBe('svg');
    expect(config.metadata.title).toBe('Test');
  });

  it('decodeBase64 helpers round-trip simple payload', () => {
    const original = '<svg></svg>';
    const encoded = btoa(original);
    expect(decodeBase64ToText(encoded)).toBe(original);
    expect(new TextDecoder().decode(decodeBase64ToBytes(encoded))).toBe(original);
  });

  it('exportHtmlToSvg validates export_document output', async () => {
    invokeMock.mockResolvedValueOnce({
      success: true,
      output_data: Array.from(new TextEncoder().encode('<svg xmlns="http://www.w3.org/2000/svg"></svg>')),
      error: null,
    });

    const result = await exportHtmlToSvg('<h1>Title</h1>');
    expect(result.success).toBe(true);
    expect(result.text).toContain('<svg');
    expect(invokeMock).toHaveBeenCalledWith(
      'export_document',
      expect.objectContaining({ content: '<h1>Title</h1>' })
    );
  });

  it('exportTypstToSvg validates render_typst output', async () => {
    invokeMock.mockResolvedValueOnce({
      success: true,
      output: btoa('<svg xmlns="http://www.w3.org/2000/svg"><text>Hello</text></svg>'),
      error: null,
    });

    const result = await exportTypstToSvg('#set page(width: 100pt)\nHello');
    expect(result.success).toBe(true);
    expect(result.text).toContain('Hello');
  });

  it('exportHtmlToSvg rejects invalid SVG payloads', async () => {
    invokeMock.mockResolvedValueOnce({
      success: true,
      output_data: Array.from(new TextEncoder().encode('not svg')),
      error: null,
    });

    const result = await exportHtmlToSvg('<p>Hello</p>');
    expect(result.success).toBe(false);
    expect(result.error).toContain('structural validation');
  });

  it('previewTypstSvgFromHtml converts HTML then exports Typst SVG', async () => {
    invokeMock.mockResolvedValueOnce({
      success: true,
      output: btoa('<svg xmlns="http://www.w3.org/2000/svg"><text>Preview</text></svg>'),
      error: null,
    });

    const htmlToTypst = vi.fn((html: string) => `# converted\n${html}`);
    const result = await previewTypstSvgFromHtml('<p>Preview</p>', htmlToTypst);

    expect(htmlToTypst).toHaveBeenCalledWith('<p>Preview</p>');
    expect(result.success).toBe(true);
    expect(result.text).toContain('Preview');
    expect(invokeMock).toHaveBeenCalledWith(
      'render_typst',
      expect.objectContaining({ request: expect.objectContaining({ format: 'svg' }) })
    );
  });

  it('exportToSvg routes html source to export_document', async () => {
    invokeMock.mockResolvedValueOnce({
      success: true,
      output_data: Array.from(new TextEncoder().encode('<svg xmlns="http://www.w3.org/2000/svg"></svg>')),
      error: null,
    });

    const result = await exportToSvg('<div>Route</div>', 'html');
    expect(result.success).toBe(true);
    expect(invokeMock).toHaveBeenCalledWith('export_document', expect.any(Object));
  });

  it('exportToSvg routes typst source to render_typst', async () => {
    invokeMock.mockResolvedValueOnce({
      success: true,
      output: btoa('<svg xmlns="http://www.w3.org/2000/svg"></svg>'),
      error: null,
    });

    const result = await exportToSvg('', 'typst', { typstSource: '# Hello' });
    expect(result.success).toBe(true);
    expect(invokeMock).toHaveBeenCalledWith(
      'render_typst',
      expect.objectContaining({
        request: expect.objectContaining({ source: '# Hello', format: 'svg' }),
      })
    );
  });

  it('createSvgObjectUrl returns blob URL for valid SVG text', () => {
    const url = createSvgObjectUrl('<svg xmlns="http://www.w3.org/2000/svg"></svg>');
    expect(url).toMatch(/^blob:/);
    URL.revokeObjectURL(url);
  });
});
