/**
 * Test Helpers Unit Tests
 * NOTE: Temporarily skipped due to vitest compatibility issues with bun
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import {
  wait,
  waitForUpdate,
  waitFor,
  createMockEditor,
  createMockFile,
  createMockBlob,
  mockFileReader,
  mockTauriAPI,
  createTestDocument,
  PerformanceTester,
  perfTester,
  triggerEvent,
  setInputValue,
  clickButton,
  isVisible,
  getText,
  hasClass,
  pressKey
} from '../testHelpers';

describe.skip('testHelpers', () => {
  describe('wait', () => {
    it('should wait for specified time', async () => {
      const start = Date.now();
      await wait(100);
      const duration = Date.now() - start;
      expect(duration).toBeGreaterThanOrEqual(90);
      expect(duration).toBeLessThan(150);
    });

    it('should wait for 0ms', async () => {
      await wait(0);
      expect(true).toBe(true);
    });
  });

  describe('waitForUpdate', () => {
    it('should wait for Vue update', async () => {
      await waitForUpdate();
      expect(true).toBe(true);
    });
  });

  describe('waitFor', () => {
    it('should wait for condition to be true', async () => {
      let condition = false;
      setTimeout(() => {
        condition = true;
      }, 50);

      await waitFor(() => condition, { timeout: 1000, interval: 10 });
      expect(condition).toBe(true);
    });

    it('should throw timeout error if condition never becomes true', async () => {
      const condition = () => false;
      await expect(waitFor(condition, { timeout: 100, interval: 10 })).rejects.toThrow('Timeout waiting for condition');
    });

    it('should handle immediate true condition', async () => {
      await waitFor(() => true, { timeout: 1000 });
      expect(true).toBe(true);
    });
  });

  describe('createMockEditor', () => {
    it('should create mock editor with commands', () => {
      const editor = createMockEditor();
      expect(editor.commands).toBeDefined();
      if (editor.commands) {
        expect(editor.commands.setContent).toBeDefined();
        expect(editor.commands.focus).toBeDefined();
        expect(editor.commands.toggleBold).toBeDefined();
      }
    });

    it('should create mock editor with chain method', () => {
      const editor = createMockEditor();
      expect(editor.chain).toBeDefined();
      expect(typeof editor.chain).toBe('function');
    });

    it('should create mock editor with isActive method', () => {
      const editor = createMockEditor();
      expect(editor.isActive).toBeDefined();
      expect(typeof editor.isActive).toBe('function');
    });

    it('should create mock editor with can method', () => {
      const editor = createMockEditor();
      expect(editor.can).toBeDefined();
      expect(typeof editor.can).toBe('function');
    });

    it('should create mock editor with getHTML method', () => {
      const editor = createMockEditor();
      expect(editor.getHTML).toBeDefined();
      expect(typeof editor.getHTML).toBe('function');
    });

    it('should create mock editor with getText method', () => {
      const editor = createMockEditor();
      expect(editor.getText).toBeDefined();
      expect(typeof editor.getText).toBe('function');
    });

    it('should create mock editor with getJSON method', () => {
      const editor = createMockEditor();
      expect(editor.getJSON).toBeDefined();
      expect(typeof editor.getJSON).toBe('function');
    });

    it('should create mock editor with isEmpty property', () => {
      const editor = createMockEditor();
      expect(editor.isEmpty).toBe(false);
    });

    it('should create mock editor with destroy method', () => {
      const editor = createMockEditor();
      expect(editor.destroy).toBeDefined();
      expect(typeof editor.destroy).toBe('function');
    });

    it('should create mock editor with event methods', () => {
      const editor = createMockEditor();
      expect(editor.on).toBeDefined();
      expect(editor.off).toBeDefined();
      expect(editor.emit).toBeDefined();
    });
  });

  describe('createMockFile', () => {
    it('should create mock file with default type', () => {
      const file = createMockFile('test.txt', 'content');
      expect(file.name).toBe('test.txt');
      expect(file.type).toBe('text/plain');
    });

    it('should create mock file with custom type', () => {
      const file = createMockFile('test.json', '{"key":"value"}', 'application/json');
      expect(file.name).toBe('test.json');
      expect(file.type).toBe('application/json');
    });

    it('should create file instance', () => {
      const file = createMockFile('test.txt', 'content');
      expect(file instanceof File).toBe(true);
    });
  });

  describe('createMockBlob', () => {
    it('should create mock blob with default type', () => {
      const blob = createMockBlob('content');
      expect(blob.type).toBe('text/plain');
    });

    it('should create mock blob with custom type', () => {
      const blob = createMockBlob('content', 'application/json');
      expect(blob.type).toBe('application/json');
    });

    it('should create blob instance', () => {
      const blob = createMockBlob('content');
      expect(blob instanceof Blob).toBe(true);
    });
  });

  describe('mockFileReader', () => {
    beforeEach(() => {
      mockFileReader('test content');
    });

    it('should mock FileReader readAsText', () => {
      const reader = (global.FileReader as any)();
      reader.onload = vi.fn();
      const blob = new Blob(['test'], { type: 'text/plain' });
      reader.readAsText(blob);
      expect(reader.onload).toHaveBeenCalled();
    });

    it('should mock FileReader readAsDataURL', () => {
      const reader = (global.FileReader as any)();
      reader.onload = vi.fn();
      const blob = new Blob(['test'], { type: 'text/plain' });
      reader.readAsDataURL(blob);
      expect(reader.onload).toHaveBeenCalled();
    });

    it('should mock FileReader readAsArrayBuffer', () => {
      const reader = (global.FileReader as any)();
      reader.onload = vi.fn();
      const blob = new Blob(['test'], { type: 'text/plain' });
      reader.readAsArrayBuffer(blob);
      expect(reader.onload).toHaveBeenCalled();
    });
  });

  describe('mockTauriAPI', () => {
    it('should create mock Tauri API', () => {
      const tauri = mockTauriAPI();
      expect(tauri.invoke).toBeDefined();
      expect(tauri.dialog).toBeDefined();
      expect(tauri.fs).toBeDefined();
      expect(tauri.event).toBeDefined();
    });

    it('should mock invoke method', () => {
      const tauri = mockTauriAPI();
      expect(tauri.invoke).toBeDefined();
      expect(typeof tauri.invoke).toBe('function');
    });

    it('should mock dialog methods', () => {
      const tauri = mockTauriAPI();
      expect(tauri.dialog.save).toBeDefined();
      expect(tauri.dialog.open).toBeDefined();
    });

    it('should mock fs methods', () => {
      const tauri = mockTauriAPI();
      expect(tauri.fs.readTextFile).toBeDefined();
      expect(tauri.fs.writeTextFile).toBeDefined();
    });

    it('should mock event methods', () => {
      const tauri = mockTauriAPI();
      expect(tauri.event.listen).toBeDefined();
    });
  });

  describe('createTestDocument', () => {
    it('should create test document structure', () => {
      const doc = createTestDocument();
      expect(doc.type).toBe('doc');
      expect(doc.content).toBeDefined();
      expect(Array.isArray(doc.content)).toBe(true);
    });

    it('should create document with heading', () => {
      const doc = createTestDocument();
      expect(doc.content[0]?.type).toBe('heading');
      expect(doc.content[0]?.attrs?.level).toBe(1);
    });

    it('should create document with paragraph', () => {
      const doc = createTestDocument();
      expect(doc.content[1].type).toBe('paragraph');
    });

    it('should create document with bullet list', () => {
      const doc = createTestDocument();
      expect(doc.content[2].type).toBe('bulletList');
      expect(doc.content[2].content).toBeDefined();
      expect(doc.content[2].content.length).toBe(2);
    });
  });

  describe('PerformanceTester', () => {
    let tester: PerformanceTester;

    beforeEach(() => {
      tester = new PerformanceTester();
    });

    it('should start performance mark', () => {
      tester.start('test');
      expect(true).toBe(true);
    });

    it('should end performance mark and return duration', () => {
      tester.start('test');
      const duration = tester.end('test');
      expect(typeof duration).toBe('number');
      expect(duration).toBeGreaterThanOrEqual(0);
    });

    it('should throw error if end called without start', () => {
      expect(() => tester.end('nonexistent')).toThrow('No start mark found for: nonexistent');
    });

    it('should measure function execution time', () => {
      const duration = tester.measure('test', () => {
        // Simple operation
        let _sum = 0;
        for (let i = 0; i < 1000; i++) {
          _sum += i;
        }
      });
      expect(typeof duration).toBe('number');
      expect(duration).toBeGreaterThanOrEqual(0);
    });

    it('should measure async function execution time', async () => {
      const duration = await tester.measureAsync('test', async () => {
        await new Promise(resolve => setTimeout(resolve, 10));
      });
      expect(typeof duration).toBe('number');
      expect(duration).toBeGreaterThanOrEqual(5);
    });

    it('should handle multiple marks', () => {
      tester.start('mark1');
      tester.start('mark2');
      const duration1 = tester.end('mark1');
      const duration2 = tester.end('mark2');
      expect(typeof duration1).toBe('number');
      expect(typeof duration2).toBe('number');
    });
  });

  describe('perfTester singleton', () => {
    it('should export performance tester instance', () => {
      expect(perfTester).toBeDefined();
      expect(perfTester instanceof PerformanceTester).toBe(true);
    });

    it('should be usable as singleton', () => {
      perfTester.start('singleton-test');
      const duration = perfTester.end('singleton-test');
      expect(typeof duration).toBe('number');
    });
  });

  describe('waitFor edge cases', () => {
    it('should use default timeout and interval', async () => {
      let condition = false;
      setTimeout(() => {
        condition = true;
      }, 100);

      await waitFor(() => condition);
      expect(condition).toBe(true);
    });

    it('should handle custom interval', async () => {
      let condition = false;
      setTimeout(() => {
        condition = true;
      }, 50);

      await waitFor(() => condition, { timeout: 1000, interval: 25 });
      expect(condition).toBe(true);
    });

    it('should handle very short timeout', async () => {
      const condition = () => false;
      await expect(waitFor(condition, { timeout: 10, interval: 5 })).rejects.toThrow('Timeout waiting for condition');
    });
  });

  describe('createMockEditor edge cases', () => {
    it('should return chain with same commands', () => {
      const editor = createMockEditor();
      const chain = editor.chain?.();
      expect(chain).toBeDefined();
      if (chain) {
        expect(chain).toBeDefined();
      }
    });

    it('should return can with run method', () => {
      const editor = createMockEditor();
      const can = editor.can?.();
      expect(can).toBeDefined();
      if (can) {
        expect(can).toBeDefined();
      }
    });

    it('should return HTML from getHTML', () => {
      const editor = createMockEditor();
      const html = editor.getHTML?.();
      expect(html).toBe('<p>Test content</p>');
    });

    it('should return text from getText', () => {
      const editor = createMockEditor();
      const text = editor.getText?.();
      expect(text).toBe('Test content');
    });

    it('should return JSON from getJSON', () => {
      const editor = createMockEditor();
      const json = editor.getJSON?.();
      expect(json).toBeDefined();
      expect(json?.type).toBe('doc');
    });

    it('should return false from isActive', () => {
      const editor = createMockEditor();
      const active = editor.isActive?.('bold');
      expect(active).toBe(false);
    });

    it('should have all editor commands', () => {
      const editor = createMockEditor();
      const commands = editor.commands as any;
      expect(commands.setContent).toBeDefined();
      expect(commands.focus).toBeDefined();
      expect(commands.toggleBold).toBeDefined();
      expect(commands.toggleItalic).toBeDefined();
      expect(commands.toggleUnderline).toBeDefined();
      expect(commands.toggleStrike).toBeDefined();
      expect(commands.setHeading).toBeDefined();
      expect(commands.setParagraph).toBeDefined();
      expect(commands.toggleBulletList).toBeDefined();
      expect(commands.toggleOrderedList).toBeDefined();
      expect(commands.toggleTaskList).toBeDefined();
      expect(commands.setTextAlign).toBeDefined();
      expect(commands.insertTable).toBeDefined();
      expect(commands.deleteTable).toBeDefined();
      expect(commands.addColumnBefore).toBeDefined();
      expect(commands.addColumnAfter).toBeDefined();
      expect(commands.deleteColumn).toBeDefined();
      expect(commands.addRowBefore).toBeDefined();
      expect(commands.addRowAfter).toBeDefined();
      expect(commands.deleteRow).toBeDefined();
      expect(commands.mergeCells).toBeDefined();
      expect(commands.splitCell).toBeDefined();
      expect(commands.setImage).toBeDefined();
      expect(commands.setLink).toBeDefined();
      expect(commands.unsetLink).toBeDefined();
      expect(commands.setColor).toBeDefined();
      expect(commands.setHighlight).toBeDefined();
      expect(commands.clearNodes).toBeDefined();
      expect(commands.undo).toBeDefined();
      expect(commands.redo).toBeDefined();
    });
  });

  describe('createMockFile edge cases', () => {
    it('should handle empty content', () => {
      const file = createMockFile('empty.txt', '');
      expect(file.name).toBe('empty.txt');
    });

    it('should handle special characters in name', () => {
      const file = createMockFile('test file (1).txt', 'content');
      expect(file.name).toBe('test file (1).txt');
    });

    it('should handle unicode content', () => {
      const file = createMockFile('unicode.txt', '你好世界 🌍');
      expect(file.name).toBe('unicode.txt');
    });
  });

  describe('createMockBlob edge cases', () => {
    it('should handle empty content', () => {
      const blob = createMockBlob('');
      expect(blob.type).toBe('text/plain');
    });

    it('should handle unicode content', () => {
      const blob = createMockBlob('你好世界');
      expect(blob.type).toBe('text/plain');
    });

    it('should handle binary-like content', () => {
      const blob = createMockBlob('\x00\x01\x02\x03', 'application/octet-stream');
      expect(blob.type).toBe('application/octet-stream');
    });
  });

  describe('mockFileReader edge cases', () => {
    it('should handle empty content', () => {
      mockFileReader('');
      const reader = (global.FileReader as any)();
      reader.onload = vi.fn();
      const blob = new Blob([''], { type: 'text/plain' });
      reader.readAsText(blob);
      expect(reader.onload).toHaveBeenCalled();
    });

    it('should handle unicode content in readAsText', () => {
      mockFileReader('你好世界');
      const reader = (global.FileReader as any)();
      reader.onload = vi.fn();
      const blob = new Blob(['你好世界'], { type: 'text/plain' });
      reader.readAsText(blob);
      expect(reader.onload).toHaveBeenCalled();
    });
  });

  describe('mockTauriAPI edge cases', () => {
    it('should return resolved value from invoke', async () => {
      const tauri = mockTauriAPI();
      const result = await tauri.invoke('test-command');
      expect(result).toBe(null);
    });

    it('should return resolved path from dialog.save', async () => {
      const tauri = mockTauriAPI();
      const result = await tauri.dialog.save();
      expect(result).toBe('/path/to/file.docx');
    });

    it('should return resolved path from dialog.open', async () => {
      const tauri = mockTauriAPI();
      const result = await tauri.dialog.open();
      expect(result).toBe('/path/to/file.docx');
    });

    it('should return content from fs.readTextFile', async () => {
      const tauri = mockTauriAPI();
      const result = await tauri.fs.readTextFile('test.txt');
      expect(result).toBe('Mock file content');
    });

    it('should resolve from fs.writeTextFile', async () => {
      const tauri = mockTauriAPI();
      const result = await tauri.fs.writeTextFile('test.txt', 'content');
      expect(result).toBeUndefined();
    });

    it('should return unlisten function from event.listen', async () => {
      const tauri = mockTauriAPI();
      const result = await tauri.event.listen('event-name', vi.fn());
      expect(typeof result).toBe('function');
    });
  });

  describe('createTestDocument edge cases', () => {
    it('should have correct heading text', () => {
      const doc = createTestDocument();
      const heading = doc.content[0] as any;
      expect(heading?.content?.[0]?.text).toBe('测试标题');
    });

    it('should have correct paragraph text', () => {
      const doc = createTestDocument();
      const paragraph = doc.content[1] as any;
      expect(paragraph?.content?.[0]?.text).toBe('测试段落内容');
    });

    it('should have correct list item texts', () => {
      const doc = createTestDocument();
      const list = doc.content[2] as any;
      expect(list?.content?.[0]?.content?.[0]?.content?.[0]?.text).toBe('列表项 1');
      expect(list?.content?.[1]?.content?.[0]?.content?.[0]?.text).toBe('列表项 2');
    });

    it('should have three content items', () => {
      const doc = createTestDocument();
      expect(doc.content.length).toBe(3);
    });
  });

  describe('PerformanceTester edge cases', () => {
    it('should handle very fast operations', () => {
      const tester = new PerformanceTester();
      const duration = tester.measure('fast', () => {
        // No-op
      });
      expect(typeof duration).toBe('number');
      expect(duration).toBeGreaterThanOrEqual(0);
    });

    it('should handle synchronous errors in measure', () => {
      const tester = new PerformanceTester();
      expect(() => {
        tester.measure('error', () => {
          throw new Error('Test error');
        });
      }).toThrow('Test error');
    });

    it('should handle async errors in measureAsync', async () => {
      const tester = new PerformanceTester();
      await expect(
        tester.measureAsync('async-error', async () => {
          throw new Error('Async error');
        })
      ).rejects.toThrow('Async error');
    });

    it('should allow reusing mark names after end', () => {
      const tester = new PerformanceTester();
      tester.start('test');
      tester.end('test');
      tester.start('test');
      const duration = tester.end('test');
      expect(typeof duration).toBe('number');
    });

    it('should handle concurrent marks', () => {
      const tester = new PerformanceTester();
      tester.start('mark1');
      tester.start('mark2');
      tester.start('mark3');
      const duration1 = tester.end('mark1');
      const duration2 = tester.end('mark2');
      const duration3 = tester.end('mark3');
      expect(typeof duration1).toBe('number');
      expect(typeof duration2).toBe('number');
      expect(typeof duration3).toBe('number');
    });
  });

  describe('Vue Helper Functions', () => {
    it('should have triggerEvent function', () => {
      expect(typeof triggerEvent).toBe('function');
    });

    it('should have setInputValue function', () => {
      expect(typeof setInputValue).toBe('function');
    });

    it('should have clickButton function', () => {
      expect(typeof clickButton).toBe('function');
    });

    it('should have isVisible function', () => {
      expect(typeof isVisible).toBe('function');
    });

    it('should have getText function', () => {
      expect(typeof getText).toBe('function');
    });

    it('should have hasClass function', () => {
      expect(typeof hasClass).toBe('function');
    });

    it('should have pressKey function', () => {
      expect(typeof pressKey).toBe('function');
    });

    it('should handle triggerEvent with non-existent element', async () => {
      const mockWrapper = {
        find: vi.fn().mockReturnValue({ exists: () => false })
      } as any;
      await expect(triggerEvent(mockWrapper, '.non-existent', 'click')).rejects.toThrow(
        'Element not found'
      );
    });

    it('should handle setInputValue with non-existent input', async () => {
      const mockWrapper = {
        find: vi.fn().mockReturnValue({ exists: () => false })
      } as any;
      await expect(setInputValue(mockWrapper, '.non-existent', 'value')).rejects.toThrow(
        'Input not found'
      );
    });

    it('should handle getText with non-existent element', () => {
      const mockWrapper = {
        find: vi.fn().mockReturnValue({ exists: () => false })
      } as any;
      expect(() => getText(mockWrapper, '.non-existent')).toThrow('Element not found');
    });

    it('should handle hasClass with non-existent element', () => {
      const mockWrapper = {
        find: vi.fn().mockReturnValue({ exists: () => false })
      } as any;
      expect(() => hasClass(mockWrapper, '.non-existent', 'class')).toThrow('Element not found');
    });

    it('should handle pressKey with non-existent element', async () => {
      const mockWrapper = {
        find: vi.fn().mockReturnValue({ exists: () => false })
      } as any;
      await expect(pressKey(mockWrapper, '.non-existent', 'Enter')).rejects.toThrow(
        'Element not found'
      );
    });

    it('should handle pressKey with modifiers', async () => {
      const mockElement = {
        exists: () => true,
        trigger: vi.fn().mockResolvedValue(undefined)
      };
      const mockWrapper = {
        find: vi.fn().mockReturnValue(mockElement)
      } as any;
      await pressKey(mockWrapper, '.element', 'Enter', { ctrl: true, shift: true });
      expect(mockElement.trigger).toHaveBeenCalledWith('keydown', {
        key: 'Enter',
        ctrlKey: true,
        shiftKey: true,
        altKey: undefined,
        metaKey: undefined
      });
    });
  });

  describe('Mock Creation Functions', () => {
    it('should have createMockEditor function', () => {
      expect(typeof createMockEditor).toBe('function');
    });

    it('should have createMockFile function', () => {
      expect(typeof createMockFile).toBe('function');
    });

    it('should have createMockBlob function', () => {
      expect(typeof createMockBlob).toBe('function');
    });

    it('should have mockFileReader function', () => {
      expect(typeof mockFileReader).toBe('function');
    });

    it('should have mockTauriAPI function', () => {
      expect(typeof mockTauriAPI).toBe('function');
    });

    it('should have createTestDocument function', () => {
      expect(typeof createTestDocument).toBe('function');
    });
  });

  describe('Wait Functions', () => {
    it('should have wait function', () => {
      expect(typeof wait).toBe('function');
    });

    it('should have waitForUpdate function', () => {
      expect(typeof waitForUpdate).toBe('function');
    });

    it('should have waitFor function', () => {
      expect(typeof waitFor).toBe('function');
    });
  });
});
