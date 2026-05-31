/**
 * 测试辅助工具
 * 提供常用的测试工具函数和 Mock 对象
 */

import { Editor } from '@tiptap/core';

import { VueWrapper } from '@vue/test-utils';
import { vi } from 'vitest';

/**
 * 等待指定时间
 */
export const wait = (ms: number): Promise<void> => {
  return new Promise(resolve => setTimeout(resolve, ms));
};

/**
 * 等待 Vue 更新完成
 */
export const waitForUpdate = async (): Promise<void> => {
  await wait(0);
};

/**
 * 等待条件满足
 */
export const waitFor = async (
  condition: () => boolean,
  options: {
    timeout?: number;
    interval?: number;
  } = {}
): Promise<void> => {
  const { timeout = 5000, interval = 100 } = options;
  const startTime = Date.now();

  while (!condition()) {
    if (Date.now() - startTime > timeout) {
      throw new Error('Timeout waiting for condition');
    }
    await wait(interval);
  }
};

/**
 * 创建 Mock Editor
 */
export const createMockEditor = (): Partial<Editor> => {
  const commands = {
    setContent: vi.fn().mockReturnThis(),
    focus: vi.fn().mockReturnThis(),
    blur: vi.fn().mockReturnThis(),
    toggleBold: vi.fn().mockReturnThis(),
    toggleItalic: vi.fn().mockReturnThis(),
    toggleUnderline: vi.fn().mockReturnThis(),
    toggleStrike: vi.fn().mockReturnThis(),
    setHeading: vi.fn().mockReturnThis(),
    setParagraph: vi.fn().mockReturnThis(),
    toggleBulletList: vi.fn().mockReturnThis(),
    toggleOrderedList: vi.fn().mockReturnThis(),
    toggleTaskList: vi.fn().mockReturnThis(),
    setTextAlign: vi.fn().mockReturnThis(),
    insertTable: vi.fn().mockReturnThis(),
    deleteTable: vi.fn().mockReturnThis(),
    addColumnBefore: vi.fn().mockReturnThis(),
    addColumnAfter: vi.fn().mockReturnThis(),
    deleteColumn: vi.fn().mockReturnThis(),
    addRowBefore: vi.fn().mockReturnThis(),
    addRowAfter: vi.fn().mockReturnThis(),
    deleteRow: vi.fn().mockReturnThis(),
    mergeCells: vi.fn().mockReturnThis(),
    splitCell: vi.fn().mockReturnThis(),
    setImage: vi.fn().mockReturnThis(),
    setLink: vi.fn().mockReturnThis(),
    unsetLink: vi.fn().mockReturnThis(),
    setColor: vi.fn().mockReturnThis(),
    setHighlight: vi.fn().mockReturnThis(),
    clearNodes: vi.fn().mockReturnThis(),
    undo: vi.fn().mockReturnThis(),
    redo: vi.fn().mockReturnThis(),
    run: vi.fn()
  };

  return {
    commands: commands as any,
    chain: () =>
      ({
        ...commands
      }) as any,
    isActive: vi.fn().mockReturnValue(false),
    can: () =>
      ({
        run: vi.fn().mockReturnValue(true)
      }) as any,
    getHTML: vi.fn().mockReturnValue('<p>Test content</p>'),
    getText: vi.fn().mockReturnValue('Test content'),
    getJSON: vi.fn().mockReturnValue({ type: 'doc', content: [] }),
    isEmpty: false,
    destroy: vi.fn(),
    on: vi.fn(),
    off: vi.fn(),
    emit: vi.fn()
  };
};

/**
 * 创建 Mock File
 */
export const createMockFile = (
  name: string,
  content: string,
  type: string = 'text/plain'
): File => {
  const blob = new Blob([content], { type });
  return new File([blob], name, { type });
};

/**
 * 创建 Mock Blob
 */
export const createMockBlob = (content: string, type: string = 'text/plain'): Blob => {
  return new Blob([content], { type });
};

/**
 * 模拟文件读取
 */
export const mockFileReader = (content: string): void => {
  const mockReader = {
    readAsText: vi.fn(function (this: any) {
      this.onload({ target: { result: content } });
    }),
    readAsDataURL: vi.fn(function (this: any) {
      this.onload({ target: { result: `data:text/plain;base64,${btoa(content)}` } });
    }),
    readAsArrayBuffer: vi.fn(function (this: any) {
      const buffer = new ArrayBuffer(content.length);
      const view = new Uint8Array(buffer);
      for (let i = 0; i < content.length; i++) {
        view[i] = content.charCodeAt(i);
      }
      this.onload({ target: { result: buffer } });
    })
  };

  global.FileReader = vi.fn(() => mockReader) as any;
};

/**
 * 模拟 Tauri API
 */
export const mockTauriAPI = () => {
  return {
    invoke: vi.fn().mockResolvedValue(null),
    dialog: {
      save: vi.fn().mockResolvedValue('/path/to/file.docx'),
      open: vi.fn().mockResolvedValue('/path/to/file.docx')
    },
    fs: {
      readTextFile: vi.fn().mockResolvedValue('Mock file content'),
      writeTextFile: vi.fn().mockResolvedValue(undefined)
    },
    event: {
      listen: vi.fn().mockResolvedValue(() => {})
    }
  };
};

/**
 * 触发组件事件
 */
export const triggerEvent = async (
  wrapper: VueWrapper<any>,
  selector: string,
  event: string,
  data?: any
): Promise<void> => {
  const element = wrapper.find(selector);
  if (!element.exists()) {
    throw new Error(`Element not found: ${selector}`);
  }
  await element.trigger(event, data);
  await waitForUpdate();
};

/**
 * 设置输入值
 */
export const setInputValue = async (
  wrapper: VueWrapper<any>,
  selector: string,
  value: string
): Promise<void> => {
  const input = wrapper.find(selector);
  if (!input.exists()) {
    throw new Error(`Input not found: ${selector}`);
  }
  await input.setValue(value);
  await waitForUpdate();
};

/**
 * 点击按钮
 */
export const clickButton = async (wrapper: VueWrapper<any>, selector: string): Promise<void> => {
  await triggerEvent(wrapper, selector, 'click');
};

/**
 * 检查元素是否可见
 */
export const isVisible = (wrapper: VueWrapper<any>, selector: string): boolean => {
  const element = wrapper.find(selector);
  return element.exists() && element.isVisible();
};

/**
 * 获取元素文本
 */
export const getText = (wrapper: VueWrapper<any>, selector: string): string => {
  const element = wrapper.find(selector);
  if (!element.exists()) {
    throw new Error(`Element not found: ${selector}`);
  }
  return element.text();
};

/**
 * 检查元素是否有类名
 */
export const hasClass = (
  wrapper: VueWrapper<any>,
  selector: string,
  className: string
): boolean => {
  const element = wrapper.find(selector);
  if (!element.exists()) {
    throw new Error(`Element not found: ${selector}`);
  }
  return element.classes().includes(className);
};

/**
 * 模拟键盘事件
 */
export const pressKey = async (
  wrapper: VueWrapper<any>,
  selector: string,
  key: string,
  modifiers?: {
    ctrl?: boolean;
    shift?: boolean;
    alt?: boolean;
    meta?: boolean;
  }
): Promise<void> => {
  const element = wrapper.find(selector);
  if (!element.exists()) {
    throw new Error(`Element not found: ${selector}`);
  }

  await element.trigger('keydown', {
    key,
    ctrlKey: modifiers?.ctrl,
    shiftKey: modifiers?.shift,
    altKey: modifiers?.alt,
    metaKey: modifiers?.meta
  });

  await waitForUpdate();
};

/**
 * 创建测试数据
 */
export const createTestDocument = () => {
  return {
    type: 'doc',
    content: [
      {
        type: 'heading',
        attrs: { level: 1 },
        content: [{ type: 'text', text: '测试标题' }]
      },
      {
        type: 'paragraph',
        content: [{ type: 'text', text: '测试段落内容' }]
      },
      {
        type: 'bulletList',
        content: [
          {
            type: 'listItem',
            content: [
              {
                type: 'paragraph',
                content: [{ type: 'text', text: '列表项 1' }]
              }
            ]
          },
          {
            type: 'listItem',
            content: [
              {
                type: 'paragraph',
                content: [{ type: 'text', text: '列表项 2' }]
              }
            ]
          }
        ]
      }
    ]
  };
};

/**
 * 性能测试工具
 */
export class PerformanceTester {
  private marks: Map<string, number> = new Map();

  start(name: string): void {
    this.marks.set(name, performance.now());
  }

  end(name: string): number {
    const start = this.marks.get(name);
    if (!start) {
      throw new Error(`No start mark found for: ${name}`);
    }
    const duration = performance.now() - start;
    this.marks.delete(name);
    return duration;
  }

  measure(name: string, fn: () => void): number {
    this.start(name);
    fn();
    return this.end(name);
  }

  async measureAsync(name: string, fn: () => Promise<void>): Promise<number> {
    this.start(name);
    await fn();
    return this.end(name);
  }
}

export const perfTester = new PerformanceTester();
