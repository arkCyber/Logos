# LOGOS 测试指南

## 📋 测试概览

本项目使用 **Vitest** 作为测试框架，提供全面的单元测试和集成测试。

---

## 🚀 快速开始

### 运行所有测试
```bash
bun run test
```

### 运行测试并生成覆盖率报告
```bash
bun run test:coverage
```

### 以 UI 模式运行测试
```bash
bun run test:ui
```

### 监听模式（开发时使用）
```bash
bun run test:watch
```

### 运行一次测试（CI 环境）
```bash
bun run test:run
```

---

## 📁 测试文件结构

```
src/
├── utils/
│   ├── __tests__/
│   │   ├── logger.test.ts          # 日志工具测试
│   │   ├── errorHandler.test.ts   # 错误处理测试
│   │   ├── tableOfContents.test.ts # 目录生成测试
│   │   └── ...
│   ├── logger.ts
│   ├── errorHandler.ts
│   └── testHelpers.ts              # 测试辅助工具
├── components/
│   └── __tests__/
│       ├── Editor.spec.ts          # 编辑器组件测试
│       ├── translator.spec.ts      # 翻译器测试
│       └── slideTranslator.spec.ts # 幻灯片翻译器测试
└── test/
    └── setup.ts                    # 测试环境设置
```

---

## 🧪 编写测试

### 基本测试结构

```typescript
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { myFunction } from '../myModule';

describe('MyModule', () => {
  beforeEach(() => {
    // 每个测试前的设置
    vi.clearAllMocks();
  });

  describe('myFunction', () => {
    it('should return expected result', () => {
      const result = myFunction('input');
      expect(result).toBe('expected');
    });

    it('should handle edge cases', () => {
      expect(() => myFunction(null)).toThrow();
    });
  });
});
```

### Vue 组件测试

```typescript
import { mount } from '@vue/test-utils';
import { describe, it, expect } from 'vitest';
import MyComponent from '../MyComponent.vue';

describe('MyComponent', () => {
  it('should render correctly', () => {
    const wrapper = mount(MyComponent, {
      props: {
        title: 'Test Title'
      }
    });

    expect(wrapper.text()).toContain('Test Title');
  });

  it('should emit event on click', async () => {
    const wrapper = mount(MyComponent);
    
    await wrapper.find('button').trigger('click');
    
    expect(wrapper.emitted('click')).toBeTruthy();
  });
});
```

### 使用测试辅助工具

```typescript
import { 
  wait, 
  waitFor, 
  createMockEditor,
  clickButton,
  setInputValue
} from '@/utils/testHelpers';

describe('Editor Integration', () => {
  it('should update content', async () => {
    const editor = createMockEditor();
    const wrapper = mount(EditorComponent, {
      global: {
        provide: {
          editor
        }
      }
    });

    await setInputValue(wrapper, 'input.content', 'New content');
    await clickButton(wrapper, 'button.save');

    await waitFor(() => editor.commands.setContent.mock.calls.length > 0);

    expect(editor.commands.setContent).toHaveBeenCalledWith('New content');
  });
});
```

---

## 📊 测试覆盖率

### 覆盖率目标

| 类型 | 目标 | 当前 |
|------|------|------|
| 语句覆盖率 | 80% | - |
| 分支覆盖率 | 80% | - |
| 函数覆盖率 | 80% | - |
| 行覆盖率 | 80% | - |

### 查看覆盖率报告

运行测试后，覆盖率报告会生成在 `coverage/` 目录：

```bash
bun run test:coverage

# 打开 HTML 报告
open coverage/index.html
```

---

## 🔧 Mock 和 Stub

### Mock Tauri API

```typescript
import { mockTauriAPI } from '@/utils/testHelpers';

describe('Document Operations', () => {
  it('should save document', async () => {
    const tauri = mockTauriAPI();
    
    await saveDocument('content');
    
    expect(tauri.invoke).toHaveBeenCalledWith('save_document', {
      content: 'content'
    });
  });
});
```

### Mock 文件操作

```typescript
import { createMockFile, mockFileReader } from '@/utils/testHelpers';

describe('File Upload', () => {
  it('should read file content', async () => {
    const file = createMockFile('test.txt', 'Test content');
    mockFileReader('Test content');
    
    const content = await readFile(file);
    
    expect(content).toBe('Test content');
  });
});
```

---

## 🎯 测试最佳实践

### 1. 测试命名
```typescript
// ✅ 好的命名
it('should save document when save button is clicked', () => {});

// ❌ 不好的命名
it('test save', () => {});
```

### 2. 单一职责
```typescript
// ✅ 每个测试只测试一个功能
it('should validate email format', () => {});
it('should show error for invalid email', () => {});

// ❌ 一个测试做太多事
it('should validate and save email', () => {});
```

### 3. 使用 beforeEach 清理
```typescript
describe('MyComponent', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // 重置状态
  });

  it('test 1', () => {});
  it('test 2', () => {});
});
```

### 4. 避免测试实现细节
```typescript
// ✅ 测试行为
it('should display error message', () => {
  expect(wrapper.find('.error').text()).toBe('Error occurred');
});

// ❌ 测试实现
it('should set errorMessage variable', () => {
  expect(wrapper.vm.errorMessage).toBe('Error occurred');
});
```

### 5. 使用有意义的断言
```typescript
// ✅ 具体的断言
expect(result).toEqual({ id: 1, name: 'Test' });

// ❌ 模糊的断言
expect(result).toBeTruthy();
```

---

## 🐛 调试测试

### 使用 console.log
```typescript
it('should work', () => {
  const result = myFunction();
  console.log('Result:', result); // 调试输出
  expect(result).toBe('expected');
});
```

### 使用 test.only
```typescript
// 只运行这一个测试
it.only('should work', () => {
  // ...
});
```

### 使用 test.skip
```typescript
// 跳过这个测试
it.skip('should work', () => {
  // ...
});
```

### 使用 Vitest UI
```bash
bun run test:ui
```
在浏览器中查看测试结果，支持交互式调试。

---

## 📈 性能测试

### 使用 PerformanceTester

```typescript
import { perfTester } from '@/utils/testHelpers';

describe('Performance', () => {
  it('should complete within time limit', () => {
    const duration = perfTester.measure('operation', () => {
      // 执行操作
      heavyOperation();
    });

    expect(duration).toBeLessThan(100); // 应在 100ms 内完成
  });

  it('should handle async operations', async () => {
    const duration = await perfTester.measureAsync('async-op', async () => {
      await asyncOperation();
    });

    expect(duration).toBeLessThan(500);
  });
});
```

---

## 🔄 CI/CD 集成

### GitHub Actions 示例

```yaml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Bun
        uses: oven-sh/setup-bun@v1
      
      - name: Install dependencies
        run: bun install
      
      - name: Run tests
        run: bun run test:coverage
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./coverage/lcov.info
```

---

## 📝 测试清单

### 新功能开发
- [ ] 编写单元测试
- [ ] 编写集成测试
- [ ] 测试边界情况
- [ ] 测试错误处理
- [ ] 达到 80% 覆盖率
- [ ] 所有测试通过
- [ ] 代码审查

### Bug 修复
- [ ] 编写复现 bug 的测试
- [ ] 修复 bug
- [ ] 确认测试通过
- [ ] 添加回归测试

---

## 🎓 学习资源

- [Vitest 官方文档](https://vitest.dev/)
- [Vue Test Utils 文档](https://test-utils.vuejs.org/)
- [Testing Library 最佳实践](https://testing-library.com/docs/guiding-principles)

---

## 💡 常见问题

### Q: 测试运行很慢怎么办？
A: 
1. 使用 `test.concurrent` 并行运行测试
2. 减少不必要的 `beforeEach` 操作
3. 使用 mock 替代真实的 API 调用

### Q: 如何测试 Tauri 命令？
A: 使用 `mockTauriAPI()` 创建 mock 对象

### Q: 如何测试异步操作？
A: 使用 `async/await` 和 `waitFor` 辅助函数

### Q: 测试覆盖率不够怎么办？
A: 
1. 查看覆盖率报告找出未覆盖的代码
2. 添加针对性的测试用例
3. 测试边界情况和错误路径

---

**最后更新**: 2026-05-28  
**维护者**: LOGOS 开发团队
