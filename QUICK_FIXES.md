# 快速修复方案

## 🚨 立即可执行的修复

### 1. 优化 Vite 配置 - 代码分割

**文件**: `vite.config.ts`

```typescript
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [vue()],
  server: {
    port: 1420,
    strictPort: false,
  },
  
  // 新增: 优化依赖
  optimizeDeps: {
    exclude: ['events', 'fs', 'buffer']
  },
  
  // 新增: 代码分割
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          // Vue 核心
          'vue-vendor': ['vue', '@tiptap/vue-3'],
          
          // TipTap 核心
          'editor-core': [
            '@tiptap/core',
            '@tiptap/starter-kit',
            '@tiptap/pm'
          ],
          
          // TipTap 扩展
          'editor-extensions': [
            '@tiptap/extension-table',
            '@tiptap/extension-table-row',
            '@tiptap/extension-table-cell',
            '@tiptap/extension-table-header',
            '@tiptap/extension-image',
            '@tiptap/extension-link',
            '@tiptap/extension-text-align',
            '@tiptap/extension-code-block-lowlight',
            '@tiptap/extension-underline',
            '@tiptap/extension-task-list',
            '@tiptap/extension-task-item',
            '@tiptap/extension-text-style',
            '@tiptap/extension-highlight',
            '@tiptap/extension-placeholder',
            '@tiptap/extension-typography'
          ],
          
          // 数学公式
          'katex': ['katex'],
          
          // 文档处理
          'docx': ['docx'],
          
          // 代码高亮
          'lowlight': ['lowlight'],
          
          // Tauri
          'tauri': ['@tauri-apps/api', '@tauri-apps/plugin-dialog', '@tauri-apps/plugin-opener']
        }
      }
    },
    
    // 提高警告阈值
    chunkSizeWarningLimit: 1000,
    
    // 生产环境优化
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true
      }
    }
  }
});
```

**预期效果**:
- 包大小从 9MB 减少到 ~3-4MB
- 首次加载时间减少 50%
- 更好的缓存策略

---

### 2. 创建日志工具

**文件**: `src/utils/logger.ts` (新建)

```typescript
type LogLevel = 'debug' | 'info' | 'warn' | 'error';

interface LogEntry {
  level: LogLevel;
  message: string;
  timestamp: number;
  data?: any;
}

class Logger {
  private logs: LogEntry[] = [];
  private maxLogs = 100;

  debug(message: string, data?: any) {
    this.log('debug', message, data);
    if (import.meta.env.DEV) {
      console.log(`%c[DEBUG] ${message}`, 'color: #888', data);
    }
  }

  info(message: string, data?: any) {
    this.log('info', message, data);
    if (import.meta.env.DEV) {
      console.info(`%c[INFO] ${message}`, 'color: #0078D4', data);
    }
  }

  warn(message: string, data?: any) {
    this.log('warn', message, data);
    console.warn(`[WARN] ${message}`, data);
  }

  error(message: string, error?: Error | any) {
    this.log('error', message, error);
    console.error(`[ERROR] ${message}`, error);
    
    // 可以发送到错误追踪服务
    if (import.meta.env.PROD) {
      this.sendToErrorTracking(message, error);
    }
  }

  private log(level: LogLevel, message: string, data?: any) {
    const entry: LogEntry = {
      level,
      message,
      timestamp: Date.now(),
      data
    };
    
    this.logs.push(entry);
    
    // 限制日志数量
    if (this.logs.length > this.maxLogs) {
      this.logs.shift();
    }
  }

  private sendToErrorTracking(message: string, error?: any) {
    // TODO: 集成 Sentry 或其他错误追踪服务
    // Sentry.captureException(error);
  }

  getLogs(): LogEntry[] {
    return [...this.logs];
  }

  clearLogs() {
    this.logs = [];
  }

  exportLogs(): string {
    return JSON.stringify(this.logs, null, 2);
  }
}

export const logger = new Logger();
```

**使用方法**:
```typescript
// 替换所有 console.log
import { logger } from '@/utils/logger';

// ❌ console.log('Saving document...')
// ✅ logger.debug('Saving document...')

// ❌ console.error('Failed to save:', error)
// ✅ logger.error('Failed to save document', error)
```

---

### 3. 添加类型声明文件

**文件**: `src/types/third-party.d.ts` (新建)

```typescript
// html-to-rtf
declare module 'html-to-rtf' {
  function htmlToRtf(html: string): string;
  export default htmlToRtf;
}

// mammoth
declare module 'mammoth' {
  interface ConvertOptions {
    arrayBuffer?: ArrayBuffer;
    path?: string;
  }
  
  interface ConvertResult {
    value: string;
    messages: any[];
  }
  
  export function convertToHtml(options: ConvertOptions): Promise<ConvertResult>;
  export function extractRawText(options: ConvertOptions): Promise<ConvertResult>;
}

// katex (补充)
declare module 'katex' {
  interface KatexOptions {
    displayMode?: boolean;
    throwOnError?: boolean;
    errorColor?: string;
    macros?: Record<string, string>;
    trust?: boolean;
  }
  
  export function renderToString(tex: string, options?: KatexOptions): string;
  export function render(tex: string, element: HTMLElement, options?: KatexOptions): void;
}

// typo-js
declare module 'typo-js' {
  export default class Typo {
    constructor(dictionary: string, affData?: string, dicData?: string);
    check(word: string): boolean;
    suggest(word: string, limit?: number): string[];
  }
}
```

---

### 4. 改进错误处理

**文件**: `src/utils/errorHandler.ts` (新建)

```typescript
import { logger } from './logger';

export class AppError extends Error {
  constructor(
    message: string,
    public code: string,
    public severity: 'low' | 'medium' | 'high' = 'medium'
  ) {
    super(message);
    this.name = 'AppError';
  }
}

export function handleError(error: unknown, context: string): string {
  let message = '发生未知错误';
  
  if (error instanceof AppError) {
    message = error.message;
    logger.error(`[${context}] ${message}`, { code: error.code, severity: error.severity });
  } else if (error instanceof Error) {
    message = error.message;
    logger.error(`[${context}] ${message}`, error);
  } else if (typeof error === 'string') {
    message = error;
    logger.error(`[${context}] ${message}`);
  } else {
    logger.error(`[${context}] Unknown error`, error);
  }
  
  return message;
}

export async function withErrorHandling<T>(
  fn: () => Promise<T>,
  context: string,
  fallback?: T
): Promise<T | undefined> {
  try {
    return await fn();
  } catch (error) {
    const message = handleError(error, context);
    
    // 显示用户友好的错误消息
    // showNotification(message, 'error');
    
    return fallback;
  }
}
```

**使用示例**:
```typescript
import { withErrorHandling, AppError } from '@/utils/errorHandler';

// 包装异步函数
const loadDocument = async () => {
  return withErrorHandling(async () => {
    const filePath = await open({ ... });
    if (!filePath) {
      throw new AppError('未选择文件', 'NO_FILE_SELECTED', 'low');
    }
    
    const content = await readTextFile(filePath);
    editor.value?.commands.setContent(content);
    
    return content;
  }, 'loadDocument');
};
```

---

### 5. 修复 Rust unwrap/expect

**优先修复文件**: `src-tauri/src/ai_service/conversation.rs`

```rust
// ❌ 危险代码
pub fn get_message(&self, index: usize) -> &Message {
    self.messages.get(index).unwrap()
}

// ✅ 安全代码
pub fn get_message(&self, index: usize) -> Result<&Message, String> {
    self.messages
        .get(index)
        .ok_or_else(|| format!("Message at index {} not found", index))
}

// 或使用 Option
pub fn get_message(&self, index: usize) -> Option<&Message> {
    self.messages.get(index)
}
```

**批量修复脚本**:
```bash
# 查找所有 unwrap
cd src-tauri
rg "\.unwrap\(\)" --type rust

# 查找所有 expect
rg "\.expect\(" --type rust

# 建议使用 ? 操作符或 match
```

---

## 📋 执行清单

### 立即执行 (今天)
```bash
# 1. 更新 vite.config.ts
# 复制上面的配置

# 2. 创建日志工具
mkdir -p src/utils
# 创建 logger.ts

# 3. 创建类型声明
mkdir -p src/types
# 创建 third-party.d.ts

# 4. 测试构建
bun run build:check

# 5. 检查包大小
du -sh dist/
```

### 本周执行
```bash
# 1. 创建错误处理工具
# 创建 errorHandler.ts

# 2. 替换 console.log
# 使用 VS Code 全局搜索替换

# 3. 修复 Rust unwrap
cd src-tauri
cargo clippy -- -W clippy::unwrap_used

# 4. 运行测试
bun run test
cargo test
```

---

## 🎯 预期改进

| 指标 | 修复前 | 修复后 | 改进 |
|------|--------|--------|------|
| JS 包大小 | 9 MB | ~3 MB | -67% |
| 首次加载 | 5 秒 | 2 秒 | -60% |
| console.log | 61 处 | 0 处 | -100% |
| @ts-ignore | 7 处 | 0 处 | -100% |
| 类型安全 | 85% | 95% | +10% |

---

## ⚠️ 注意事项

1. **备份代码**: 修改前先提交当前代码
2. **逐步测试**: 每个修复后都要测试
3. **性能监控**: 使用 Lighthouse 验证改进
4. **用户测试**: 确保功能正常

---

**创建时间**: 2026-05-28 23:40  
**优先级**: P0 (立即执行)  
**预计时间**: 2-3 小时
