/**
 * SpreadsheetApiService unit tests
 *
 * Tests the HTTP API client by mocking globalThis.fetch. Covers all three
 * resource groups (conditional formats, charts, pivot tables) plus the two
 * export methods and error-path handling.
 *
 * Each test manages its own fetch mock to avoid the happy-dom global.fetch
 * being undefined on the shared beforeEach spy (which causes mockReset
 * between tests to corrupt the chain).
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { spreadsheetApi as service, SpreadsheetApiService, type Chart, type ConditionalFormatRule, type PivotTable } from '../spreadsheetApi';

function mockResponse(data: unknown, ok = true) {
  return {
    ok,
    statusText: ok ? 'OK' : 'Not Found',
    json: () => Promise.resolve(data)
  };
}

describe('SpreadsheetApiService', () => {
  beforeEach(() => {
    // Stub globalThis.fetch so the service's bare `fetch` call resolves.
    // Each test re-assigns this so there is no shared mock-state pollution.
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    vi.stubGlobal('fetch', vi.fn<typeof fetch>().mockResolvedValue(mockResponse({}) as Response));
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  // ── Conditional Formats ──────────────────────────────────────────────────

  describe('listConditionalFormats', () => {
    it('GETs the correct endpoint and returns parsed JSON', async () => {
      const rules: ConditionalFormatRule[] = [
        { id: 'r1', sheet_id: 's1', range: 'A1:C5', rule_type: 'cellValue', rule_data: '>0', format_data: 'green', priority: 1, created_at: '' }
      ];
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(rules) as Response);
      vi.stubGlobal('fetch', fetchSpy);

      const result = await service.listConditionalFormats('s1');

      expect(fetchSpy).toHaveBeenCalledTimes(1);
      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/sheets/s1/conditional-formats',
        expect.objectContaining({ headers: { 'Content-Type': 'application/json' } })
      );
      expect(result).toEqual(rules);
    });
  });

  describe('createConditionalFormat', () => {
    it('POSTs the payload to the correct endpoint', async () => {
      const rule: ConditionalFormatRule = { id: 'r1', sheet_id: 's1', range: 'A1', rule_type: 'containsText', rule_data: 'foo', format_data: 'red', priority: 1, created_at: '2026-01-01' };
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(rule) as Response);
      vi.stubGlobal('fetch', fetchSpy);

      const data = { sheet_id: 's1', range: 'A1', rule_type: 'containsText', rule_data: 'foo', format_data: 'red' };
      const result = await service.createConditionalFormat(data);

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/sheets/conditional-formats',
        expect.objectContaining({ method: 'POST', body: JSON.stringify(data) })
      );
      expect(result).toEqual(rule);
    });

    it('includes optional priority in the POST body', async () => {
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse({ id: 'r1', sheet_id: 's1', range: '', rule_type: '', rule_data: '', format_data: '', priority: 99, created_at: '' }) as Response);
      vi.stubGlobal('fetch', fetchSpy);

      await service.createConditionalFormat({ sheet_id: 's1', range: 'A1', rule_type: 'x', rule_data: '', format_data: '', priority: 99 });
      const [, opts] = fetchSpy.mock.calls[0] as [string, RequestInit];
      expect(JSON.parse(opts.body as string)).toMatchObject({ priority: 99 });
    });
  });

  describe('updateConditionalFormat', () => {
    it('PUTs the partial update to the correct endpoint', async () => {
      const rule: ConditionalFormatRule = { id: 'r1', sheet_id: 's1', range: 'B2', rule_type: 'cellValue', rule_data: '>10', format_data: 'blue', priority: 2, created_at: '' };
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(rule) as Response);
      vi.stubGlobal('fetch', fetchSpy);

      const result = await service.updateConditionalFormat('r1', { range: 'B2' });
      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/conditional-formats/r1',
        expect.objectContaining({ method: 'PUT', body: JSON.stringify({ range: 'B2' }) })
      );
      expect(result).toEqual(rule);
    });
  });

  describe('deleteConditionalFormat', () => {
    it('DELETEs the correct endpoint', async () => {
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(undefined) as Response);
      vi.stubGlobal('fetch', fetchSpy);
      await service.deleteConditionalFormat('r1');
      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/conditional-formats/r1',
        expect.objectContaining({ method: 'DELETE' })
      );
    });
  });

  // ── Charts ───────────────────────────────────────────────────────────────

  describe('listCharts', () => {
    it('GETs the charts endpoint', async () => {
      const charts: Chart[] = [{ id: 'c1', sheet_id: 's1', name: 'Q1 Sales', chart_type: 'bar', data_range: 'A1:B4', style_data: '{}', created_at: '', updated_at: '' }];
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(charts) as Response);
      vi.stubGlobal('fetch', fetchSpy);

      const result = await service.listCharts('s1');
      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/sheets/s1/charts',
        expect.objectContaining({ headers: { 'Content-Type': 'application/json' } })
      );
      expect(result).toEqual(charts);
    });
  });

  describe('createChart', () => {
    it('POSTs the chart data', async () => {
      const chart: Chart = { id: 'c1', sheet_id: 's1', name: 'Monthly', chart_type: 'line', data_range: 'A1:C12', title: 'Revenue', style_data: '{}', created_at: '', updated_at: '' };
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(chart) as Response);
      vi.stubGlobal('fetch', fetchSpy);

      const data = { sheet_id: 's1', name: 'Monthly', chart_type: 'line', data_range: 'A1:C12', title: 'Revenue', style_data: '{}' };
      const result = await service.createChart(data);
      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/sheets/charts',
        expect.objectContaining({ method: 'POST', body: JSON.stringify(data) })
      );
      expect(result).toEqual(chart);
    });
  });

  describe('updateChart', () => {
    it('PUTs the partial update', async () => {
      const chart: Chart = { id: 'c1', sheet_id: 's1', name: 'Updated', chart_type: 'bar', data_range: 'A1', style_data: '{}', created_at: '', updated_at: '' };
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(chart) as Response);
      vi.stubGlobal('fetch', fetchSpy);

      const result = await service.updateChart('c1', { name: 'Updated' });
      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/charts/c1',
        expect.objectContaining({ method: 'PUT', body: JSON.stringify({ name: 'Updated' }) })
      );
      expect(result).toEqual(chart);
    });
  });

  describe('deleteChart', () => {
    it('DELETEs the chart endpoint', async () => {
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(undefined) as Response);
      vi.stubGlobal('fetch', fetchSpy);
      await service.deleteChart('c1');
      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/charts/c1',
        expect.objectContaining({ method: 'DELETE' })
      );
    });
  });

  // ── Pivot Tables ────────────────────────────────────────────────────────

  describe('listPivotTables', () => {
    it('GETs the pivot tables endpoint', async () => {
      const tables: PivotTable[] = [{ id: 'p1', sheet_id: 's1', name: 'Summary', source_range: 'A1:Z99', row_fields: 'Region', column_fields: 'Product', value_fields: 'Sales', filter_fields: '', created_at: '', updated_at: '' }];
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(tables) as Response);
      vi.stubGlobal('fetch', fetchSpy);

      const result = await service.listPivotTables('s1');
      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/sheets/s1/pivot-tables',
        expect.objectContaining({ headers: { 'Content-Type': 'application/json' } })
      );
      expect(result).toEqual(tables);
    });
  });

  describe('createPivotTable', () => {
    it('POSTs the pivot table data', async () => {
      const table: PivotTable = { id: 'p1', sheet_id: 's1', name: 'Sales Pivot', source_range: 'A1:D10', row_fields: 'Region', column_fields: 'Year', value_fields: 'Revenue', filter_fields: '', created_at: '', updated_at: '' };
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(table) as Response);
      vi.stubGlobal('fetch', fetchSpy);

      const data = { sheet_id: 's1', name: 'Sales Pivot', source_range: 'A1:D10', row_fields: 'Region', column_fields: 'Year', value_fields: 'Revenue', filter_fields: '' };
      const result = await service.createPivotTable(data);
      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/sheets/pivot-tables',
        expect.objectContaining({ method: 'POST', body: JSON.stringify(data) })
      );
      expect(result).toEqual(table);
    });
  });

  describe('updatePivotTable', () => {
    it('PUTs the partial update', async () => {
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse({}) as Response);
      vi.stubGlobal('fetch', fetchSpy);

      await service.updatePivotTable('p1', { name: 'New Name', row_fields: 'City' });
      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/pivot-tables/p1',
        expect.objectContaining({ method: 'PUT', body: JSON.stringify({ name: 'New Name', row_fields: 'City' }) })
      );
    });
  });

  describe('deletePivotTable', () => {
    it('DELETEs the pivot table endpoint', async () => {
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(undefined) as Response);
      vi.stubGlobal('fetch', fetchSpy);
      await service.deletePivotTable('p1');
      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/pivot-tables/p1',
        expect.objectContaining({ method: 'DELETE' })
      );
    });
  });

  // ── Error handling ──────────────────────────────────────────────────────

  describe('request error handling', () => {
    it('throws when GET returns non-ok', async () => {
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(null, false) as Response);
      vi.stubGlobal('fetch', fetchSpy);
      await expect(service.listConditionalFormats('s1')).rejects.toThrow('API request failed: Not Found');
    });

    it('throws when POST returns non-ok', async () => {
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(null, false) as Response);
      vi.stubGlobal('fetch', fetchSpy);
      await expect(service.createChart({ sheet_id: 's1', name: 'x', chart_type: 'bar', data_range: 'A1', style_data: '{}' }))
        .rejects.toThrow('API request failed: Not Found');
    });

    it('throws when PUT returns non-ok', async () => {
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(null, false) as Response);
      vi.stubGlobal('fetch', fetchSpy);
      await expect(service.updateChart('c1', { name: 'x' })).rejects.toThrow('API request failed: Not Found');
    });

    it('throws when DELETE returns non-ok', async () => {
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(null, false) as Response);
      vi.stubGlobal('fetch', fetchSpy);
      await expect(service.deleteChart('c1')).rejects.toThrow('API request failed: Not Found');
    });
  });

  // ── exportToTypst ───────────────────────────────────────────────────────

  describe('exportToTypst', () => {
    it('GETs the export endpoint and returns the JSON', async () => {
      const payload = { typst_code: '= table(...)', success: true };
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(payload) as Response);
      vi.stubGlobal('fetch', fetchSpy);

      const result = await service.exportToTypst('s1');
      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/sheets/s1/export/typst',
        { method: 'GET', headers: { 'Content-Type': 'application/json' } }
      );
      expect(result).toEqual(payload);
    });

    it('throws when the export endpoint returns non-ok', async () => {
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse(null, false) as Response);
      vi.stubGlobal('fetch', fetchSpy);
      await expect(service.exportToTypst('s1')).rejects.toThrow('Failed to export to Typst: Not Found');
    });
  });

  // ── exportToTypstTauri ─────────────────────────────────────────────────

  describe('exportToTypstTauri', () => {
    it('throws when Tauri is not available', async () => {
      // happy-dom does not define __TAURI__; the check passes typeof but __TAURI__ is undefined
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse({}) as Response);
      vi.stubGlobal('fetch', fetchSpy);
      await expect(service.exportToTypstTauri('s1')).rejects.toThrow('Tauri not available');
    });

    // Skipped: mocking dynamic `import('@tauri-apps/api/core')` in happy-dom
    // requires elaborate module-mock setup (e.g. vi.mock with import.meta.url)
    // that is fragile in this environment. Covered by integration/E2E tests.
    it.skip('calls Tauri invoke when __TAURI__ is present', () => {
      // Placeholder — replace with vi.mock setup if Tauri integration tests are added
    });
  });

  // ── Custom base URL ─────────────────────────────────────────────────────

  describe('custom base URL', () => {
    it('uses the provided base URL instead of the default', async () => {
      const svc = new SpreadsheetApiService('https://custom-host:3000/api');
      const fetchSpy = vi.fn<typeof fetch>().mockResolvedValue(mockResponse([]) as Response);
      vi.stubGlobal('fetch', fetchSpy);

      await svc.listConditionalFormats('s1');
      expect(fetchSpy).toHaveBeenCalledWith(
        'https://custom-host:3000/api/sheets/s1/conditional-formats',
        expect.objectContaining({ headers: { 'Content-Type': 'application/json' } })
      );
    });
  });
});
