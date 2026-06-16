// Spreadsheet API Service
// Connects to the spreadsheet microservice

const SPREADSHEET_API_BASE = 'http://localhost:8080/api';

export interface ConditionalFormatRule {
  id: string;
  sheet_id: string;
  range: string;
  rule_type: string;
  rule_data: string;
  format_data: string;
  priority: number;
  created_at: string;
}

export interface Chart {
  id: string;
  sheet_id: string;
  name: string;
  chart_type: string;
  data_range: string;
  title?: string;
  x_axis_title?: string;
  y_axis_title?: string;
  legend_position?: string;
  style_data: string;
  created_at: string;
  updated_at: string;
}

export interface PivotTable {
  id: string;
  sheet_id: string;
  name: string;
  source_range: string;
  row_fields: string;
  column_fields: string;
  value_fields: string;
  filter_fields: string;
  created_at: string;
  updated_at: string;
}

export class SpreadsheetApiService {
  private baseUrl: string;

  constructor(baseUrl: string = SPREADSHEET_API_BASE) {
    this.baseUrl = baseUrl;
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.baseUrl}${endpoint}`;
    const response = await fetch(url, {
      headers: {
        'Content-Type': 'application/json',
        ...options.headers
      },
      ...options
    });

    if (!response.ok) {
      throw new Error(`API request failed: ${response.statusText}`);
    }

    return response.json();
  }

  // Conditional Formatting
  async listConditionalFormats(sheetId: string): Promise<ConditionalFormatRule[]> {
    return this.request<ConditionalFormatRule[]>(`/sheets/${sheetId}/conditional-formats`);
  }

  async createConditionalFormat(data: {
    sheet_id: string;
    range: string;
    rule_type: string;
    rule_data: string;
    format_data: string;
    priority?: number;
  }): Promise<ConditionalFormatRule> {
    return this.request<ConditionalFormatRule>('/sheets/conditional-formats', {
      method: 'POST',
      body: JSON.stringify(data)
    });
  }

  async updateConditionalFormat(
    id: string,
    data: Partial<{
      range: string;
      rule_type: string;
      rule_data: string;
      format_data: string;
      priority: number;
    }>
  ): Promise<ConditionalFormatRule> {
    return this.request<ConditionalFormatRule>(`/conditional-formats/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data)
    });
  }

  async deleteConditionalFormat(id: string): Promise<void> {
    return this.request<void>(`/conditional-formats/${id}`, {
      method: 'DELETE'
    });
  }

  // Charts
  async listCharts(sheetId: string): Promise<Chart[]> {
    return this.request<Chart[]>(`/sheets/${sheetId}/charts`);
  }

  async createChart(data: {
    sheet_id: string;
    name: string;
    chart_type: string;
    data_range: string;
    title?: string;
    x_axis_title?: string;
    y_axis_title?: string;
    legend_position?: string;
    style_data: string;
  }): Promise<Chart> {
    return this.request<Chart>('/sheets/charts', {
      method: 'POST',
      body: JSON.stringify(data)
    });
  }

  async updateChart(
    id: string,
    data: Partial<{
      name: string;
      chart_type: string;
      data_range: string;
      title: string;
      x_axis_title: string;
      y_axis_title: string;
      legend_position: string;
      style_data: string;
    }>
  ): Promise<Chart> {
    return this.request<Chart>(`/charts/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data)
    });
  }

  async deleteChart(id: string): Promise<void> {
    return this.request<void>(`/charts/${id}`, {
      method: 'DELETE'
    });
  }

  // Pivot Tables
  async listPivotTables(sheetId: string): Promise<PivotTable[]> {
    return this.request<PivotTable[]>(`/sheets/${sheetId}/pivot-tables`);
  }

  async createPivotTable(data: {
    sheet_id: string;
    name: string;
    source_range: string;
    row_fields: string;
    column_fields: string;
    value_fields: string;
    filter_fields: string;
  }): Promise<PivotTable> {
    return this.request<PivotTable>('/sheets/pivot-tables', {
      method: 'POST',
      body: JSON.stringify(data)
    });
  }

  async updatePivotTable(
    id: string,
    data: Partial<{
      name: string;
      source_range: string;
      row_fields: string;
      column_fields: string;
      value_fields: string;
      filter_fields: string;
    }>
  ): Promise<PivotTable> {
    return this.request<PivotTable>(`/pivot-tables/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data)
    });
  }

  async deletePivotTable(id: string): Promise<void> {
    return this.request<void>(`/pivot-tables/${id}`, {
      method: 'DELETE'
    });
  }

  // Export to Typst format
  async exportToTypst(sheetId: string): Promise<{ typst_code: string; success: boolean; error?: string }> {
    const response = await fetch(`${this.baseUrl}/sheets/${sheetId}/export/typst`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      }
    });

    if (!response.ok) {
      throw new Error(`Failed to export to Typst: ${response.statusText}`);
    }

    return response.json();
  }

  // Tauri backend integration for Typst export
  async exportToTypstTauri(sheetId: string): Promise<{ typst_code: string; success: boolean; error?: string }> {
    if (typeof window !== 'undefined' && (window as any).__TAURI__) {
      const { invoke } = await import('@tauri-apps/api/core');
      return invoke('export_spreadsheet_to_typst', { sheetId });
    }
    throw new Error('Tauri not available');
  }
}

export const spreadsheetApi = new SpreadsheetApiService();
