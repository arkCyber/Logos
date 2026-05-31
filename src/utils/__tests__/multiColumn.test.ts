/**
 * Multi-Column Layout Manager Tests
 * Aerospace-grade comprehensive test suite
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { multiColumnManager } from '../multiColumn';

describe('MultiColumnManager', () => {
  beforeEach(() => {
    multiColumnManager.reset();
  });

  describe('Configuration', () => {
    it('should set column count', () => {
      multiColumnManager.setColumnCount(2);
      expect(multiColumnManager.getColumnCount()).toBe(2);
    });

    it('should throw error for invalid column count', () => {
      expect(() => multiColumnManager.setColumnCount(0)).toThrow();
      expect(() => multiColumnManager.setColumnCount(7)).toThrow();
    });

    it('should set column gap', () => {
      multiColumnManager.setColumnGap('30px');
      const config = multiColumnManager.getConfig();
      expect(config.columnGap).toBe('30px');
    });

    it('should throw error for empty column gap', () => {
      expect(() => multiColumnManager.setColumnGap('')).toThrow();
    });

    it('should set column rule', () => {
      multiColumnManager.setColumnRule('solid');
      const config = multiColumnManager.getConfig();
      expect(config.columnRule).toBe('solid');
    });

    it('should throw error for invalid column rule', () => {
      expect(() => multiColumnManager.setColumnRule('invalid')).toThrow();
    });

    it('should set column rule color', () => {
      multiColumnManager.setColumnRuleColor('#ff0000');
      const config = multiColumnManager.getConfig();
      expect(config.columnRuleColor).toBe('#ff0000');
    });

    it('should set column rule width', () => {
      multiColumnManager.setColumnRuleWidth('2px');
      const config = multiColumnManager.getConfig();
      expect(config.columnRuleWidth).toBe('2px');
    });

    it('should set balance', () => {
      multiColumnManager.setBalance(true);
      const config = multiColumnManager.getConfig();
      expect(config.balance).toBe(true);
    });

    it('should set full configuration', () => {
      multiColumnManager.setConfig({
        columnCount: 3,
        columnGap: '25px',
        balance: true
      });
      const config = multiColumnManager.getConfig();
      expect(config.columnCount).toBe(3);
      expect(config.columnGap).toBe('25px');
      expect(config.balance).toBe(true);
    });
  });

  describe('CSS Generation', () => {
    it('should generate CSS', () => {
      multiColumnManager.setColumnCount(2);
      const css = multiColumnManager.generateCSS();
      expect(css).toContain('column-count: 2');
      expect(css).toContain('column-gap');
    });

    it('should include balance in CSS', () => {
      multiColumnManager.setBalance(true);
      const css = multiColumnManager.generateCSS();
      expect(css).toContain('column-fill: balance');
    });

    it('should include column rule in CSS', () => {
      multiColumnManager.setColumnRule('solid');
      const css = multiColumnManager.generateCSS();
      expect(css).toContain('column-rule');
    });
  });

  describe('Layout Application', () => {
    it('should apply multi-column layout to HTML', () => {
      const html = '<p>Test content</p>';
      multiColumnManager.setColumnCount(2);
      const result = multiColumnManager.applyLayout(html);
      expect(result).toContain('multi-column-layout');
      expect(result).toContain('Test content');
    });

    it('should not apply layout for single column', () => {
      const html = '<p>Test content</p>';
      multiColumnManager.setColumnCount(1);
      const result = multiColumnManager.applyLayout(html);
      expect(result).toBe(html);
    });

    it('should remove multi-column layout from HTML', () => {
      const html = '<p>Test content</p>';
      const withLayout = multiColumnManager.applyLayout(html);
      const withoutLayout = multiColumnManager.removeLayout(withLayout);
      expect(withoutLayout).toBe(html);
    });
  });

  describe('Configuration Validation', () => {
    it('should validate column count in setConfig', () => {
      expect(() => multiColumnManager.setConfig({ columnCount: 0 })).toThrow();
    });

    it('should validate column gap in setConfig', () => {
      expect(() => multiColumnManager.setConfig({ columnGap: '' })).toThrow();
    });

    it('should validate column rule in setConfig', () => {
      expect(() => multiColumnManager.setConfig({ columnRule: 'invalid' as any })).toThrow();
    });
  });

  describe('Reset', () => {
    it('should reset to default configuration', () => {
      multiColumnManager.setColumnCount(3);
      multiColumnManager.setBalance(true);
      multiColumnManager.reset();

      const config = multiColumnManager.getConfig();
      expect(config.columnCount).toBe(1);
      expect(config.balance).toBe(false);
    });
  });

  describe('Import/Export', () => {
    it('should export configuration to JSON', () => {
      multiColumnManager.setColumnCount(2);
      const json = multiColumnManager.exportToJSON();
      expect(json).toBeTruthy();
      const data = JSON.parse(json);
      expect(data.columnCount).toBe(2);
    });

    it('should import configuration from JSON', () => {
      const json = JSON.stringify({ columnCount: 3, columnGap: '30px' });
      multiColumnManager.importFromJSON(json);

      const config = multiColumnManager.getConfig();
      expect(config.columnCount).toBe(3);
      expect(config.columnGap).toBe('30px');
    });

    it('should throw error for invalid JSON', () => {
      expect(() => multiColumnManager.importFromJSON('invalid json')).toThrow();
    });

    it('should validate imported configuration', () => {
      const json = JSON.stringify({ columnCount: 0 });
      expect(() => multiColumnManager.importFromJSON(json)).toThrow();
    });
  });

  describe('Default Configuration', () => {
    it('should have sensible defaults', () => {
      const config = multiColumnManager.getConfig();
      expect(config.columnCount).toBe(1);
      expect(config.columnGap).toBe('20px');
      expect(config.columnRule).toBe('none');
      expect(config.balance).toBe(false);
    });
  });
});
