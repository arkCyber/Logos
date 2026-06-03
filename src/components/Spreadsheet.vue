<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed } from 'vue'; // eslint-disable-line @typescript-eslint/no-unused-vars

// Load jQuery first and make it globally available BEFORE importing luckysheet
import $ from 'jquery';
(window as any).$ = $;
(window as any).jQuery = $;

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore - luckysheet doesn't have TypeScript definitions
import luckysheet from 'luckysheet';
import 'luckysheet/dist/plugins/css/pluginsCss.css';
import 'luckysheet/dist/plugins/plugins.css';
import 'luckysheet/dist/css/luckysheet.css';
import 'luckysheet/dist/luckysheet.umd';
import { invoke } from '@tauri-apps/api/core';
import { logger, LogCategory } from '../utils/logger';

// ============================================================================
// 组件定义和事件
// ============================================================================
const emit = defineEmits<{
  insertContent: [code: string];
}>();

// ============================================================================
// 组件状态
// ============================================================================
let luckysheetInstance: any = null; // Luckysheet 实例
const currentDocId = ref('default'); // 当前文档 ID
const isLoading = ref(false); // eslint-disable-line @typescript-eslint/no-unused-vars
const lastAction = ref(''); // 最后操作
const showFormulaBar = ref(true); // 显示公式栏
const selectedCellInfo = ref({ row: 0, col: 0, value: '' }); // 选中的单元格信息

// ============================================================================
// 默认数据
// ============================================================================
const defaultData = [
  {
    name: 'Sheet1',
    color: '',
    status: 1,
    order: 0,
    data: [
      { r: 0, c: 0, v: { v: '年份', m: '年份', ct: { fa: '@', t: 's' } } },
      { r: 0, c: 1, v: { v: '营收 (亿)', m: '营收 (亿)', ct: { fa: '@', t: 's' } } },
      { r: 0, c: 2, v: { v: '利润 (亿)', m: '利润 (亿)', ct: { fa: '@', t: 's' } } },
      { r: 1, c: 0, v: { v: 2025, m: '2025', ct: { fa: 'General', t: 'n' } } },
      { r: 1, c: 1, v: { v: 12.5, m: '12.5', ct: { fa: '0.0', t: 'n' } } },
      { r: 1, c: 2, v: { v: 3.2, m: '3.2', ct: { fa: '0.0', t: 'n' } } },
      { r: 2, c: 0, v: { v: 2026, m: '2026', ct: { fa: 'General', t: 'n' } } },
      { r: 2, c: 1, v: { v: 18.2, m: '18.2', ct: { fa: '0.0', t: 'n' } } },
      { r: 2, c: 2, v: { v: 5.8, m: '5.8', ct: { fa: '0.0', t: 'n' } } },
      { r: 3, c: 0, v: { v: 2027, m: '2027', ct: { fa: 'General', t: 'n' } } },
      { r: 3, c: 1, v: { v: 25.6, m: '25.6', ct: { fa: '0.0', t: 'n' } } },
      { r: 3, c: 2, v: { v: 8.9, m: '8.9', ct: { fa: '0.0', t: 'n' } } }
    ],
    config: {},
    index: 0
  }
];

// ============================================================================
// 初始化 Luckysheet
// ============================================================================
const initLuckysheet = () => {
  luckysheet.create({
    container: 'luckysheet',
    data: defaultData,
    title: 'Exodus Data',
    lang: 'zh',
    showinfobar: false,
    showsheetbar: true,
    showstatisticBar: true,
    enableAddRow: true,
    enableAddCol: true,
    userInfo: false,
    showConfigWindowResize: true,
    forceCalculation: true,
    rowHeaderWidth: 46,
    columnHeaderHeight: 25,
    defaultColWidth: 120,
    defaultRowHeight: 28,
    hook: {
      cellMousedown: (cell: any) => {
        selectedCellInfo.value = {
          row: cell.r,
          col: cell.c,
          value: cell.v?.v || ''
        };
      },
      cellUpdated: (r: number, c: number, oldValue: any, newValue: any, isRefresh: boolean) => {
        if (!isRefresh) {
          lastAction.value = `更新单元格 (${r + 1}, ${c + 1})`;
        }
      }
    }
  });

  luckysheetInstance = luckysheet;
};

// ============================================================================
// 导出选中区域到 Logos 表格
// ============================================================================
const exportSelectionToLogos = () => {
  if (!luckysheetInstance) {
    return;
  }

  const selection = luckysheetInstance.getRange();
  if (!selection || selection.length === 0) {
    alert('请先选择要导出的单元格区域');
    return;
  }

  const sheet = luckysheetInstance.getSheet();
  const selectionData = selection[0];
  if (!selectionData) {
    alert('选择数据无效');
    return;
  }
  
  const { row, column } = selectionData;
  if (!row || !column || row.length < 2 || column.length < 2) {
    alert('选择区域无效');
    return;
  }

  const matrixData: any[][] = [];
  for (let r = row[0]; r <= row[1]; r++) {
    const rowData: any[] = [];
    for (let c = column[0]; c <= column[1]; c++) {
      const cell = sheet.data[r]?.[c];
      rowData.push(cell ? cell.m || cell.v || '' : '');
    }
    matrixData.push(rowData);
  }

  const typstTableCode = translateMatrixToTypstTable(matrixData);
  emit('insertContent', typstTableCode);
  lastAction.value = '导出表格到 Logos';
};

// ============================================================================
// 导出选中区域到图表
// ============================================================================
const exportSelectionToChart = () => {
  if (!luckysheetInstance) {
    return;
  }

  const selection = luckysheetInstance.getRange();
  if (!selection || selection.length === 0) {
    alert('请先选择要生成图表的数据区域');
    return;
  }

  const sheet = luckysheetInstance.getSheet();
  const selectionData = selection[0];
  if (!selectionData) {
    alert('选择数据无效');
    return;
  }
  
  const { row, column } = selectionData;
  if (!row || !column || row.length < 2 || column.length < 2) {
    alert('选择区域无效');
    return;
  }

  const matrixData: any[][] = [];
  for (let r = row[0]; r <= row[1]; r++) {
    const rowData: any[] = [];
    for (let c = column[0]; c <= column[1]; c++) {
      const cell = sheet.data[r]?.[c];
      rowData.push(cell ? cell.m || cell.v || '' : '');
    }
    matrixData.push(rowData);
  }

  const typstChartCode = translateToTypstLineChart(matrixData, '营收趋势图');
  emit('insertContent', typstChartCode);
  lastAction.value = '生成图表';
};

// ============================================================================
// 导出选中区域到柱状图
// ============================================================================
const exportToBarChart = () => {
  if (!luckysheetInstance) {
    return;
  }

  const selection = luckysheetInstance.getRange();
  if (!selection || selection.length === 0) {
    alert('请先选择要生成图表的数据区域');
    return;
  }

  const sheet = luckysheetInstance.getSheet();
  const selectionData = selection[0];
  if (!selectionData) {
    alert('选择数据无效');
    return;
  }
  
  const { row, column } = selectionData;
  if (!row || !column || row.length < 2 || column.length < 2) {
    alert('选择区域无效');
    return;
  }

  const matrixData: any[][] = [];
  for (let r = row[0]; r <= row[1]; r++) {
    const rowData: any[] = [];
    for (let c = column[0]; c <= column[1]; c++) {
      const cell = sheet.data[r]?.[c];
      rowData.push(cell ? cell.m || cell.v || '' : '');
    }
    matrixData.push(rowData);
  }

  const typstChartCode = translateToTypstBarChart(matrixData, '数据对比');
  emit('insertContent', typstChartCode);
  lastAction.value = '生成柱状图';
};

// ============================================================================
// 行/列操作函数
// ============================================================================
const addRow = () => {
  if (!luckysheetInstance) {
    return;
  }
  luckysheetInstance.insertRow();
  lastAction.value = '插入行';
};

const addColumn = () => {
  if (!luckysheetInstance) {
    return;
  }
  luckysheetInstance.insertColumn();
  lastAction.value = '插入列';
};

const deleteRow = () => {
  if (!luckysheetInstance) {
    return;
  }
  luckysheetInstance.deleteRow();
  lastAction.value = '删除行';
};

const deleteColumn = () => {
  if (!luckysheetInstance) {
    return;
  }
  luckysheetInstance.deleteColumn();
  lastAction.value = '删除列';
};

// ============================================================================
// 冻结行/列函数
// ============================================================================
const freezeRow = () => {
  if (!luckysheetInstance) {
    return;
  }
  const selection = luckysheetInstance.getRange();
  if (selection && selection.length > 0) {
    const selectionData = selection[0];
    if (selectionData && selectionData.row && selectionData.row.length >= 1) {
      const { row } = selectionData;
      luckysheetInstance.setFrozen({ row: row[0] });
      lastAction.value = `冻结行 ${row[0] + 1}`;
    }
  }
};

const freezeColumn = () => {
  if (!luckysheetInstance) {
    return;
  }
  const selection = luckysheetInstance.getRange();
  if (selection && selection.length > 0) {
    const selectionData = selection[0];
    if (selectionData && selectionData.column && selectionData.column.length >= 1) {
      const { column } = selectionData;
      luckysheetInstance.setFrozen({ column: column[0] });
      lastAction.value = `冻结列 ${column[0] + 1}`;
    }
  }
};

// ============================================================================
// 清空表格
// ============================================================================
const clearSheet = () => {
  if (!luckysheetInstance) {
    return;
  }
  luckysheetInstance.setCellValue(0, 0, '');
  luckysheetInstance.setCellValue(0, 1, '');
  luckysheetInstance.setCellValue(0, 2, '');
  for (let i = 1; i < 10; i++) {
    for (let j = 0; j < 3; j++) {
      luckysheetInstance.setCellValue(i, j, '');
    }
  }
};

// ============================================================================
// 数据同步函数
// ============================================================================
// 同步电子表格数据到后端
const syncSheetData = async () => {
  if (!luckysheetInstance) {
    return;
  }

  try {
    const sheet = luckysheetInstance.getSheet();
    const sheetDataJson = JSON.stringify(sheet);
    
    await invoke('sync_sheet_data', {
      docId: currentDocId.value,
      sheetDataJson
    });
    
    logger.debug('Spreadsheet data synced successfully', {}, LogCategory.SYSTEM);
  } catch (error) {
    logger.error('Failed to sync spreadsheet data', error, LogCategory.SYSTEM);
  }
};

// ============================================================================
// 保存电子表格为Typst格式
// ============================================================================
const saveSpreadsheetAsTypst = async () => {
  if (!luckysheetInstance) {
    return;
  }

  try {
    const { save } = await import('@tauri-apps/plugin-dialog');
    
    const filePath = await save({
      filters: [
        {
          name: 'Typst Document',
          extensions: ['typ']
        }
      ]
    });

    if (filePath) {
      const sheet = luckysheetInstance.getSheet();
      const typstContent = convertSheetToTypst(sheet);
      
      await invoke('save_file', { filePath, content: typstContent });
      
      lastAction.value = '保存为Typst文件';
      logger.debug('Spreadsheet saved as Typst successfully', {}, LogCategory.SYSTEM);
    }
  } catch (error) {
    logger.error('Failed to save spreadsheet as Typst', error, LogCategory.SYSTEM);
  }
};

// ============================================================================
// 将电子表格数据转换为Typst格式
// ============================================================================
const convertSheetToTypst = (sheet: any): string => {
  if (!sheet || !sheet.data) {
    return '';
  }

  let typstCode = '#set page(paper: "a4", margin: (x: 2cm, y: 2.5cm))\n';
  typstCode += '#set text(font: "SimSun", size: 11pt)\n\n';
  typstCode += `= ${sheet.name || '电子表格'}\n\n`;

  // 找出数据的实际范围
  let maxRow = 0;
  let maxCol = 0;
  
  sheet.data.forEach((row: any) => {
    if (row) {
      Object.keys(row).forEach((colStr) => {
        const col = parseInt(colStr);
        if (col > maxCol) {
maxCol = col;
}
        if (col > maxRow) {
maxRow = col;
}
      });
    }
  });

  // 构建表格数据
  const tableData: string[][] = [];
  
  for (let r = 0; r <= maxRow; r++) {
    const rowData: string[] = [];
    for (let c = 0; c <= maxCol; c++) {
      const cell = sheet.data[r]?.[c];
      const value = cell ? (cell.m || cell.v || '') : '';
      rowData.push(String(value));
    }
    tableData.push(rowData);
  }

  // 生成Typst表格代码
  if (tableData.length > 0) {
    typstCode += '#table(\n';
    typstCode += `  columns: ${tableData[0].length},\n`;
    
    tableData.forEach((row, rowIndex) => {
      const rowStr = row.map(cell => `[${cell}]`).join(', ');
      typstCode += `  ${rowStr}`;
      if (rowIndex < tableData.length - 1) {
        typstCode += ',\n';
      } else {
        typstCode += '\n';
      }
    });
    
    typstCode += ')\n';
  }

  return typstCode;
};

// 从后端加载电子表格数据
const loadSheetData = async () => {
  if (!luckysheetInstance) {
    return;
  }

  try {
    const sheetDataJson = await invoke('get_sheet_data', { docId: currentDocId.value });
    
    if (sheetDataJson) {
      const sheetData = JSON.parse(sheetDataJson as string);
      luckysheetInstance.loadData(sheetData);
      logger.debug('Spreadsheet data loaded successfully', {}, LogCategory.SYSTEM);
    }
  } catch (error) {
    logger.error('Failed to load spreadsheet data', error, LogCategory.SYSTEM);
  }
};

// ============================================================================
// Typst 代码生成函数
// ============================================================================
// 将矩阵数据转换为 Typst 表格代码
const translateMatrixToTypstTable = (matrix: any[][]): string => {
  if (matrix.length === 0) {
    return '';
  }
  const colCount = matrix[0].length;

  let typstCode = '\n// --- 由 Exodus Data 智能生成的三线表 ---\n';
  typstCode += '#table(\n';
  typstCode += `  columns: ${colCount},\n`;
  typstCode += '  stroke: none,\n';
  typstCode += '  fill: (x, y) => if y == 0 { rgb("e0e7ff") } else { none },\n';
  typstCode += `  table.hline(start: 0, end: ${colCount}, stroke: 1.5pt + black),\n`;
  typstCode += `  table.hline(y: 1, start: 0, end: ${colCount}, stroke: 1pt + black),\n`;

  matrix.forEach((row, rowIndex) => {
    const cellsFormatted = row.map(cell => `[${cell}]`).join(', ');
    typstCode += `  ${cellsFormatted},\n`;

    if (rowIndex === matrix.length - 1) {
      typstCode += `  table.hline(y: ${rowIndex + 1}, start: 0, end: ${colCount}, stroke: 1.5pt + black)\n`;
    }
  });

  typstCode += ')\n';
  return typstCode;
};

// 将矩阵数据转换为 Typst 折线图代码
const translateToTypstLineChart = (matrix: any[][], title: string): string => {
  if (matrix.length < 2 || !matrix[0] || !matrix[1]) {
    return '';
  }

  const labels = matrix[0].slice(1);
  const values = matrix[1].slice(1);

  let plotCode = '\n// --- 由 Exodus Data 智能生成的矢量图表 ---\n';
  plotCode += '#import "@preview/cetz:0.3.1"\n';
  plotCode += '#import "@preview/cetz-plot:0.1.1": plot\n\n';
  plotCode += '#cetz.canvas({\n';
  plotCode += '  plot.plot(size: (12, 6), x-tick-step: 1, y-tick-step: 5, {\n';

  const dataPoints = values.map((v, i) => `(${labels[i]}, ${v})`).join(', ');
  plotCode += `    plot.add(( ${dataPoints} ), label: [${title}])\n`;

  plotCode += '  })\n';
  plotCode += '})\n';
  return plotCode;
};

// 将矩阵数据转换为 Typst 柱状图代码
const translateToTypstBarChart = (matrix: any[][], title: string): string => {
  if (matrix.length < 2 || !matrix[0] || !matrix[1]) {
    return '';
  }

  const labels = matrix[0].slice(1);
  const values = matrix[1].slice(1);

  let plotCode = '\n// --- 由 Exodus Data 智能生成的柱状图 ---\n';
  plotCode += '#import "@preview/cetz:0.3.1"\n';
  plotCode += '#import "@preview/cetz-plot:0.1.1": plot\n\n';
  plotCode += '#cetz.canvas({\n';
  plotCode += '  plot.plot(size: (12, 6), x-tick-step: 1, y-tick-step: 5, {\n';

  const dataPoints = values.map((v, i) => `(${labels[i]}, ${v})`).join(', ');
  plotCode += `    plot.add(( ${dataPoints} ), label: [${title}]),\n`;
  plotCode += '    plot.bar-style(mark: (x, y) => {\n';
  plotCode += '      (rect((x - 0.4, 0), (x + 0.4, y)), fill: blue)\n';
  plotCode += '    })\n';

  plotCode += '  })\n';
  plotCode += '})\n';
  return plotCode;
};

// ============================================================================
// 生命周期钩子
// ============================================================================
onMounted(() => {
  initLuckysheet();
});

onUnmounted(() => {
  if (luckysheetInstance) {
    luckysheet.destroy();
  }
});
</script>

<template>
  <div class="spreadsheet-container">
    <!-- Formula Bar -->
    <div v-if="showFormulaBar" class="formula-bar">
      <div class="formula-bar-label">单元格:</div>
      <div class="formula-bar-cell">{{ selectedCellInfo.row + 1 }}, {{ selectedCellInfo.col + 1 }}</div>
      <div class="formula-bar-value">{{ selectedCellInfo.value }}</div>
    </div>

    <!-- Main Toolbar -->
    <div class="spreadsheet-toolbar">
      <div class="toolbar-group">
        <button class="btn-sync" title="导出选中区域到 Logos" @click="exportSelectionToLogos">
          📄 导出表格
        </button>
        <button class="btn-chart" title="生成折线图" @click="exportSelectionToChart">
          � 折线图
        </button>
        <button class="btn-bar" title="生成柱状图" @click="exportToBarChart">
          📊 柱状图
        </button>
      </div>
      
      <div class="toolbar-divider"></div>
      
      <div class="toolbar-group">
        <button class="btn-add-row" title="插入行" @click="addRow">
          ➕ 行
        </button>
        <button class="btn-add-col" title="插入列" @click="addColumn">
          ➕ 列
        </button>
        <button class="btn-del-row" title="删除行" @click="deleteRow">
          ➖ 行
        </button>
        <button class="btn-del-col" title="删除列" @click="deleteColumn">
          ➖ 列
        </button>
      </div>
      
      <div class="toolbar-divider"></div>
      
      <div class="toolbar-group">
        <button class="btn-freeze-row" title="冻结行" @click="freezeRow">
          🧊 冻结行
        </button>
        <button class="btn-freeze-col" title="冻结列" @click="freezeColumn">
          🧊 冻结列
        </button>
      </div>
      
      <div class="toolbar-divider"></div>
      
      <div class="toolbar-group">
        <button class="btn-load" title="加载数据" @click="loadSheetData">
          📥 加载
        </button>
        <button class="btn-save" title="保存数据" @click="syncSheetData">
          💾 保存
        </button>
        <button class="btn-save-typst" title="保存为Typst文件" @click="saveSpreadsheetAsTypst">
          📝 保存Typst
        </button>
        <button class="btn-clear" title="清空表格" @click="clearSheet">🗑️ 清空</button>
      </div>
    </div>

    <!-- Status Bar -->
    <div class="status-bar">
      <div class="status-left">
        <span v-if="lastAction" class="last-action">✓ {{ lastAction }}</span>
      </div>
      <div class="status-right">
        <button class="btn-toggle-formula" @click="showFormulaBar = !showFormulaBar">
          {{ showFormulaBar ? '隐藏' : '显示' }}公式栏
        </button>
      </div>
    </div>

    <!-- Spreadsheet Container -->
    <div id="luckysheet" class="luckysheet-container"></div>
  </div>
</template>

<style scoped>
.spreadsheet-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #f8f9fa;
}

/* Formula Bar */
.formula-bar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 16px;
  background: white;
  border-bottom: 1px solid #e0e0e0;
  font-size: 13px;
}

.formula-bar-label {
  color: #666;
  font-weight: 500;
  min-width: 60px;
}

.formula-bar-cell {
  background: #f0f0f0;
  padding: 4px 8px;
  border-radius: 4px;
  min-width: 80px;
  text-align: center;
  font-family: 'Consolas', monospace;
  color: #333;
}

.formula-bar-value {
  flex: 1;
  background: #fff;
  border: 1px solid #ddd;
  padding: 4px 8px;
  border-radius: 4px;
  font-family: 'Consolas', monospace;
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* Main Toolbar */
.spreadsheet-toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: white;
  border-bottom: 1px solid #e0e0e0;
  flex-wrap: wrap;
}

.toolbar-group {
  display: flex;
  gap: 6px;
  align-items: center;
}

.toolbar-divider {
  width: 1px;
  height: 24px;
  background: #e0e0e0;
  margin: 0 4px;
}

.btn-sync,
.btn-chart,
.btn-bar,
.btn-add-row,
.btn-add-col,
.btn-del-row,
.btn-del-col,
.btn-freeze-row,
.btn-freeze-col,
.btn-load,
.btn-save,
.btn-clear {
  padding: 6px 12px;
  border: 1px solid #d0d0d0;
  background: white;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 4px;
  font-weight: 500;
}

.btn-sync:hover,
.btn-chart:hover,
.btn-bar:hover,
.btn-add-row:hover,
.btn-add-col:hover,
.btn-del-row:hover,
.btn-del-col:hover,
.btn-freeze-row:hover,
.btn-freeze-col:hover,
.btn-load:hover,
.btn-save:hover,
.btn-clear:hover {
  background: #f0f0f0;
  border-color: #b0b0b0;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.btn-sync:active,
.btn-chart:active,
.btn-bar:active,
.btn-add-row:active,
.btn-add-col:active,
.btn-del-row:active,
.btn-del-col:active,
.btn-freeze-row:active,
.btn-freeze-col:active,
.btn-load:active,
.btn-save:active,
.btn-clear:active {
  transform: translateY(0);
  box-shadow: none;
}

.btn-sync {
  border-color: #4CAF50;
  color: #4CAF50;
}

.btn-sync:hover {
  background: #4CAF50;
  color: white;
}

.btn-chart,
.btn-bar {
  border-color: #2196F3;
  color: #2196F3;
}

.btn-chart:hover,
.btn-bar:hover {
  background: #2196F3;
  color: white;
}

.btn-add-row,
.btn-add-col {
  border-color: #FF9800;
  color: #FF9800;
}

.btn-add-row:hover,
.btn-add-col:hover {
  background: #FF9800;
  color: white;
}

.btn-del-row,
.btn-del-col {
  border-color: #f44336;
  color: #f44336;
}

.btn-del-row:hover,
.btn-del-col:hover {
  background: #f44336;
  color: white;
}

.btn-freeze-row,
.btn-freeze-col {
  border-color: #9C27B0;
  color: #9C27B0;
}

.btn-freeze-row:hover,
.btn-freeze-col:hover {
  background: #9C27B0;
  color: white;
}

.btn-save {
  border-color: #4CAF50;
  color: #4CAF50;
}

.btn-save:hover {
  background: #4CAF50;
  color: white;
}

/* Status Bar */
.status-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 12px;
  background: #f0f0f0;
  border-bottom: 1px solid #e0e0e0;
  font-size: 12px;
}

.status-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.last-action {
  color: #4CAF50;
  font-weight: 500;
  animation: fadeIn 0.3s ease-in;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-4px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.status-right {
  display: flex;
  align-items: center;
}

.btn-toggle-formula {
  padding: 4px 8px;
  border: 1px solid #d0d0d0;
  background: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 11px;
  transition: all 0.2s;
}

.btn-toggle-formula:hover {
  background: #e0e0e0;
}

/* Spreadsheet Container */
.luckysheet-container {
  flex: 1;
  width: 100%;
  overflow: hidden;
  background: white;
}
</style>
