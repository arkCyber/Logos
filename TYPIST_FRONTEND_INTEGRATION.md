# Typst Service 前端集成指南

## 概述

Typst Service 通过 Tauri 提供航空航天级别的排版功能，前端可以通过 Tauri 的 IPC 机制调用后端服务。

## Tauri 命令

### render_typst

渲染 Typst 源代码为 PDF、SVG 或 PNG。

```typescript
import { invoke } from '@tauri-apps/api/core';

interface TypstRenderRequest {
  source: string;
  format: 'pdf' | 'svg' | 'png';
}

interface TypstRenderResponse {
  success: boolean;
  output?: string;  // Base64 编码的输出
  error?: string;
}

async function renderTypst(source: string, format: 'pdf' | 'svg' | 'png') {
  const request: TypstRenderRequest = {
    source,
    format
  };
  
  const response: TypstRenderResponse = await invoke('render_typst', { request });
  
  if (response.success && response.output) {
    // 解码 Base64
    const binaryString = atob(response.output);
    const bytes = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
    }
    
    // 创建 Blob
    const blob = new Blob([bytes], { 
      type: format === 'pdf' ? 'application/pdf' : 
            format === 'svg' ? 'image/svg+xml' : 'image/png' 
    });
    
    return URL.createObjectURL(blob);
  } else {
    throw new Error(response.error || 'Rendering failed');
  }
}
```

### check_typst_availability

检查 Typst 服务是否可用。

```typescript
async function checkTypstAvailability(): Promise<boolean> {
  return await invoke('check_typst_availability');
}
```

## 高级排版功能集成

### 1. 字体排版系统

```typescript
// 前端配置字体排版
interface TypographyConfig {
  enableKerning: boolean;
  enableLigatures: boolean;
  enableSmallCaps: boolean;
  opticalSize?: number;
  letterSpacing?: number;
  wordSpacing?: number;
}

function generateTypographyConfig(config: TypographyConfig): string {
  let typst = '';
  
  if (config.enableKerning) {
    typst += '#set text(kerning: true)\n';
  }
  
  if (config.enableLigatures) {
    typst += '#set text(features: (liga: true))\n';
  }
  
  if (config.enableSmallCaps) {
    typst += '#set text(features: (smcp: true))\n';
  }
  
  if (config.opticalSize) {
    typst += `#set text(optical-size: ${config.opticalSize})\n`;
  }
  
  if (config.letterSpacing) {
    typst += `#set text(tracking: ${config.letterSpacing}pt)\n`;
  }
  
  if (config.wordSpacing) {
    typst += `#set text(spacing: ${config.wordSpacing}pt)\n`;
  }
  
  return typst;
}
```

### 2. 网格系统

```typescript
interface GridConfig {
  enabled: boolean;
  spacing: number;
  showGrid: boolean;
}

function generateGridConfig(config: GridConfig): string {
  if (!config.enabled) return '';
  
  return `
#set page(
  width: 210mm,
  height: 297mm,
  margin: (x: 20mm, y: 20mm)
)
`;
}
```

### 3. CJK 排版

```typescript
interface CJKConfig {
  language: 'zh-CN' | 'zh-TW' | 'ja' | 'ko';
  writingMode: 'horizontal' | 'vertical';
  enablePunctuationCompression: boolean;
}

function generateCJKConfig(config: CJKConfig): string {
  let typst = '';
  
  // 设置语言
  const langMap = {
    'zh-CN': 'zh',
    'zh-TW': 'zh-TW',
    'ja': 'ja',
    'ko': 'ko'
  };
  
  typst += `#set text(lang: "${langMap[config.language]}")\n`;
  
  // 设置书写模式
  if (config.writingMode === 'vertical') {
    typst += '#set text(writing-mode: "vertical")\n';
  }
  
  // CJK 标点压缩
  if (config.enablePunctuationCompression) {
    typst += '#set text(cjk-punctuation: "compress")\n';
  }
  
  return typst;
}
```

### 4. 色彩管理

```typescript
interface ColorConfig {
  cmykMode: boolean;
  iccProfile?: string;
  renderingIntent?: 'perceptual' | 'relative' | 'saturation' | 'absolute';
}

function generateColorConfig(config: ColorConfig): string {
  let typst = '';
  
  if (config.cmykMode) {
    typst += '#set text(fill: cmyk(0%, 0%, 0%, 100%))\n';
  }
  
  if (config.iccProfile) {
    typst += `#set text(icc-profile: "${config.iccProfile}")\n`;
  }
  
  if (config.renderingIntent) {
    typst += `#set text(rendering-intent: "${config.renderingIntent}")\n`;
  }
  
  return typst;
}
```

### 5. 主控页面

```typescript
interface MasterPageConfig {
  headerContent?: string;
  footerContent?: string;
  showPageNumber: boolean;
  pageNumberAlign?: 'left' | 'center' | 'right';
}

function generateMasterPageConfig(config: MasterPageConfig): string {
  let typst = '';
  
  typst += '#set page(header: locate(loc => {\n';
  
  if (config.headerContent) {
    typst += `  ${config.headerContent}\n`;
  }
  
  typst += '}), footer: locate(loc => {\n';
  
  if (config.footerContent) {
    typst += `  ${config.footerContent}\n`;
  }
  
  if (config.showPageNumber) {
    const align = config.pageNumberAlign || 'right';
    typst += `  align(${align}, counter(page).display())\n`;
  }
  
  typst += '}))\n';
  
  return typst;
}
```

## Vue 组件集成示例

### TypstEditor 组件

```vue
<template>
  <div class="typst-editor">
    <div class="toolbar">
      <button @click="renderPdf">Export PDF</button>
      <button @click="renderSvg">Export SVG</button>
      <button @click="renderPng">Export PNG</button>
    </div>
    
    <div class="editor-container">
      <textarea 
        v-model="source" 
        @input="debouncedRender"
        placeholder="Enter Typst code..."
      ></textarea>
      
      <div class="preview" v-if="previewUrl">
        <iframe v-if="outputFormat === 'pdf'" :src="previewUrl"></iframe>
        <img v-else :src="previewUrl" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { debounce } from 'lodash-es';

const source = ref('#set page(paper: "a4")\nHello, World!');
const previewUrl = ref<string | null>(null);
const outputFormat = ref<'pdf' | 'svg' | 'png'>('pdf');

const render = async (format: 'pdf' | 'svg' | 'png') => {
  try {
    const response = await invoke('render_typst', {
      request: {
        source: source.value,
        format
      }
    });
    
    if (response.success && response.output) {
      const binaryString = atob(response.output);
      const bytes = new Uint8Array(binaryString.length);
      for (let i = 0; i < binaryString.length; i++) {
        bytes[i] = binaryString.charCodeAt(i);
      }
      
      const blob = new Blob([bytes], { 
        type: format === 'pdf' ? 'application/pdf' : 
              format === 'svg' ? 'image/svg+xml' : 'image/png' 
      });
      
      previewUrl.value = URL.createObjectURL(blob);
      outputFormat.value = format;
    }
  } catch (error) {
    console.error('Render failed:', error);
  }
};

const renderPdf = () => render('pdf');
const renderSvg = () => render('svg');
const renderPng = () => render('png');

const debouncedRender = debounce(() => renderPdf(), 500);

watch(source, () => {
  debouncedRender();
});
</script>

<style scoped>
.typst-editor {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.toolbar {
  padding: 10px;
  border-bottom: 1px solid #ccc;
}

.editor-container {
  display: flex;
  flex: 1;
}

textarea {
  flex: 1;
  padding: 10px;
  font-family: monospace;
  resize: none;
}

.preview {
  flex: 1;
  border-left: 1px solid #ccc;
  padding: 10px;
}

.preview iframe,
.preview img {
  width: 100%;
  height: 100%;
  border: none;
}
</style>
```

## 高级功能集成

### 增量编译

```typescript
class IncrementalTypstCompiler {
  private cache = new Map<string, string>();
  private hashCache = new Map<string, string>();
  
  async computeHash(content: string): Promise<string> {
    // 简化的哈希计算，实际应该使用 Web Crypto API
    let hash = 0;
    for (let i = 0; i < content.length; i++) {
      const char = content.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash;
    }
    return hash.toString(36);
  }
  
  async render(key: string, content: string, format: 'pdf' | 'svg' | 'png') {
    const hash = await this.computeHash(content);
    
    // 检查缓存
    if (this.hashCache.get(key) === hash && this.cache.has(key)) {
      return this.cache.get(key)!;
    }
    
    // 渲染
    const response = await invoke('render_typst', {
      request: { source: content, format }
    });
    
    if (response.success && response.output) {
      this.hashCache.set(key, hash);
      this.cache.set(key, response.output);
      return response.output;
    }
    
    throw new Error(response.error || 'Rendering failed');
  }
  
  clearCache() {
    this.cache.clear();
    this.hashCache.clear();
  }
}
```

### 并行渲染

```typescript
async function parallelRender(
  tasks: Array<{ key: string; content: string; format: 'pdf' | 'svg' | 'png' }>
): Promise<Map<string, string>> {
  const results = new Map<string, string>();
  
  const promises = tasks.map(async task => {
    try {
      const response = await invoke('render_typst', {
        request: { source: task.content, format: task.format }
      });
      
      if (response.success && response.output) {
        results.set(task.key, response.output);
      }
    } catch (error) {
      console.error(`Failed to render ${task.key}:`, error);
    }
  });
  
  await Promise.all(promises);
  return results;
}
```

## 错误处理

```typescript
interface TypstError {
  code: string;
  message: string;
  source: string;
}

async function renderWithErrorHandling(source: string, format: 'pdf' | 'svg' | 'png') {
  try {
    const response = await invoke('render_typst', {
      request: { source, format }
    });
    
    if (!response.success) {
      const error: TypstError = {
        code: 'RENDER_ERROR',
        message: response.error || 'Unknown error',
        source: 'render_typst'
      };
      
      // 记录错误
      console.error('Typst render error:', error);
      
      // 显示用户友好的错误消息
      throw new Error(`渲染失败: ${error.message}`);
    }
    
    return response.output;
  } catch (error) {
    if (error instanceof Error) {
      throw error;
    }
    throw new Error('未知错误');
  }
}
```

## 性能优化

### 防抖渲染

```typescript
import { debounce } from 'lodash-es';

const debouncedRender = debounce(async (source: string) => {
  await renderWithErrorHandling(source, 'pdf');
}, 500);
```

### 虚拟滚动

对于大型文档，使用虚拟滚动来提高性能。

```typescript
import { useVirtualizer } from '@tanstack/vue-virtual';

const virtualizer = useVirtualizer({
  count: pages.length,
  getScrollElement: () => scrollContainer.value,
  estimateSize: () => 300,
  overscan: 5
});
```

## 测试

### 单元测试

```typescript
import { describe, it, expect, vi } from 'vitest';
import { renderTypst } from './typstService';

describe('Typst Service', () => {
  it('should render PDF successfully', async () => {
    const mockInvoke = vi.fn().mockResolvedValue({
      success: true,
      output: 'base64-encoded-pdf'
    });
    
    vi.mock('@tauri-apps/api/core', () => ({
      invoke: mockInvoke
    }));
    
    const result = await renderTypst('# Hello', 'pdf');
    expect(result).toBeDefined();
  });
});
```

## 最佳实践

1. **错误处理**: 始终处理渲染错误并向用户显示友好的错误消息
2. **性能优化**: 使用防抖和缓存来减少不必要的渲染
3. **用户体验**: 显示加载状态和进度指示器
4. **内存管理**: 及时释放 Blob URL 以避免内存泄漏
5. **类型安全**: 使用 TypeScript 接口确保类型安全

## 示例项目

完整的示例项目可以参考 `src/components/Editor.vue` 中的实现。

## 故障排除

### 渲染失败

1. 检查 Typst 源代码语法
2. 验证输入大小限制（10MB）
3. 检查字体是否已安装
4. 查看错误日志获取详细信息

### 性能问题

1. 启用增量编译
2. 使用防抖减少渲染次数
3. 分批处理大型文档
4. 检查系统资源使用情况

## 相关文档

- API 文档: `TYPIST_API_DOCUMENTATION.md`
- 用户指南: `TYPIST_USER_GUIDE.md`
- 架构文档: `TYPIST_ARCHITECTURE.md`

## License

Proprietary - Aerospace Grade License
