/**
 * Unified SVG Export API
 *
 * Routes HTML and Typst export requests to the correct Rust backend services:
 * - HTML content -> export_document (svg_service)
 * - Typst source -> render_typst (TypstSvgExporter)
 */

import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';

export type SvgExportSource = 'html' | 'typst';

export interface SvgExportResult {
  success: boolean;
  data?: Uint8Array;
  text?: string;
  error?: string;
}

export interface TypstRenderResponse {
  success: boolean;
  output?: string;
  error?: string;
}

export interface ExportDocumentResponse {
  success: boolean;
  output_data: number[];
  error?: string | null;
}

/** Detect Tauri desktop runtime availability. */
export function isTauriEnvironment(): boolean {
  return typeof window !== 'undefined' && '__TAURI__' in window;
}

/** Decode base64 payload into UTF-8 text. */
export function decodeBase64ToText(base64: string): string {
  return atob(base64);
}

/** Decode base64 payload into bytes. */
export function decodeBase64ToBytes(base64: string): Uint8Array {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) {
    bytes[i] = binary.charCodeAt(i);
  }
  return bytes;
}

/** Validate minimal SVG structure before preview or save. */
export function validateSvgStructure(svg: string): boolean {
  const trimmed = svg.trim();
  return trimmed.includes('<svg') && trimmed.includes('</svg>');
}

/** Build export_document config for SVG output. */
export function buildSvgExportConfig(title = 'SVG Export') {
  const now = new Date().toISOString();
  return {
    format: 'svg' as const,
    metadata: {
      title,
      author: 'LOGOS',
      subject: '',
      keywords: [] as string[],
      created: now,
      modified: now,
    },
    include_toc: false,
    include_page_numbers: false,
    compress_images: false,
    embed_fonts: false,
    use_typst_rendering: false,
    typst_quality: 'standard' as const,
  };
}

/**
 * Export HTML editor content to SVG via svg_service/export_service.
 */
export async function exportHtmlToSvg(html: string): Promise<SvgExportResult> {
  try {
    const result = await invoke<ExportDocumentResponse>('export_document', {
      content: html,
      config: buildSvgExportConfig('HTML SVG Export'),
    });

    if (!result.success) {
      return {
        success: false,
        error: result.error ?? 'HTML to SVG export failed',
      };
    }

    const data = new Uint8Array(result.output_data);
    const text = new TextDecoder().decode(data);
    if (!validateSvgStructure(text)) {
      return {
        success: false,
        error: 'Generated SVG failed structural validation',
      };
    }

    return { success: true, data, text };
  } catch (error) {
    return {
      success: false,
      error: error instanceof Error ? error.message : String(error),
    };
  }
}

/**
 * Export Typst source to SVG via typist_service TypstSvgExporter.
 */
export async function exportTypstToSvg(
  source: string,
  page = 0
): Promise<SvgExportResult> {
  try {
    const result = await invoke<TypstRenderResponse>('render_typst', {
      request: {
        source,
        format: 'svg',
        page,
      },
    });

    if (!result.success || !result.output) {
      return {
        success: false,
        error: result.error ?? 'Typst to SVG export failed',
      };
    }

    const text = decodeBase64ToText(result.output);
    if (!validateSvgStructure(text)) {
      return {
        success: false,
        error: 'Typst SVG output failed structural validation',
      };
    }

    return {
      success: true,
      data: decodeBase64ToBytes(result.output),
      text,
    };
  } catch (error) {
    return {
      success: false,
      error: error instanceof Error ? error.message : String(error),
    };
  }
}

/**
 * Preview Typst-rendered SVG from HTML via html_to_typst + render_typst.
 */
export async function previewTypstSvgFromHtml(
  html: string,
  htmlToTypst: (content: string) => string,
  page = 0
): Promise<SvgExportResult> {
  const typstSource = htmlToTypst(html);
  return exportTypstToSvg(typstSource, page);
}

/** Create an object URL for inline SVG preview. */
export function createSvgObjectUrl(svgText: string): string {
  const blob = new Blob([svgText], { type: 'image/svg+xml' });
  return URL.createObjectURL(blob);
}

/** Prompt user and save SVG text to disk. */
export async function promptSaveSvgFile(
  svgText: string,
  defaultPath = 'document.svg'
): Promise<boolean> {
  const filePath = await save({
    defaultPath,
    filters: [
      {
        name: 'SVG Image',
        extensions: ['svg'],
      },
    ],
  });

  if (!filePath) {
    return false;
  }

  await invoke('save_file', { filePath, content: svgText });
  return true;
}

/**
 * Unified SVG export entry point.
 */
export async function exportToSvg(
  content: string,
  source: SvgExportSource,
  options?: { page?: number; typstSource?: string }
): Promise<SvgExportResult> {
  if (source === 'html') {
    return exportHtmlToSvg(content);
  }

  const typstSource = options?.typstSource ?? content;
  return exportTypstToSvg(typstSource, options?.page ?? 0);
}
