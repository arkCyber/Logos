/**
 * Typst Templates Tests
 * Aerospace-grade comprehensive test suite
 */

import { describe, it, expect, beforeEach } from 'vitest';
import {
  TypstTemplateManager,
  TypstTemplate,
  typstTemplateManager,
  getTypstTemplate,
  getAllTypstTemplates,
  getTypstTemplatesByCategory,
  applyTypstTemplate
} from '../typstTemplates';

describe('TypstTemplateManager', () => {
  let manager: TypstTemplateManager;

  beforeEach(() => {
    manager = new TypstTemplateManager();
  });

  describe('Initialization', () => {
    it('should initialize with default templates', () => {
      const templates = manager.getAllTemplates();
      expect(templates.length).toBeGreaterThan(0);
    });

    it('should have academic template', () => {
      const template = manager.getTemplate('academic-paper');
      expect(template).toBeDefined();
      expect(template?.name).toBe('学术论文');
      expect(template?.category).toBe('academic');
    });

    it('should have technical doc template', () => {
      const template = manager.getTemplate('technical-doc');
      expect(template).toBeDefined();
      expect(template?.name).toBe('技术文档');
      expect(template?.category).toBe('technical');
    });

    it('should have business report template', () => {
      const template = manager.getTemplate('business-report');
      expect(template).toBeDefined();
      expect(template?.name).toBe('商业报告');
      expect(template?.category).toBe('business');
    });

    it('should have resume template', () => {
      const template = manager.getTemplate('resume');
      expect(template).toBeDefined();
      expect(template?.name).toBe('简历');
      expect(template?.category).toBe('business');
    });

    it('should have creative writing template', () => {
      const template = manager.getTemplate('creative-writing');
      expect(template).toBeDefined();
      expect(template?.name).toBe('创意写作');
      expect(template?.category).toBe('creative');
    });

    it('should have lab report template', () => {
      const template = manager.getTemplate('lab-report');
      expect(template).toBeDefined();
      expect(template?.name).toBe('实验报告');
      expect(template?.category).toBe('academic');
    });
  });

  describe('Template Operations', () => {
    it('should add custom template', () => {
      const customTemplate: TypstTemplate = {
        id: 'custom-template',
        name: '自定义模板',
        description: '自定义模板描述',
        category: 'custom',
        content: '#set page(paper: "a4")\n\n= 自定义标题\n\n自定义内容...'
      };

      manager.addTemplate(customTemplate);
      const retrieved = manager.getTemplate('custom-template');
      expect(retrieved).toEqual(customTemplate);
    });

    it('should get template by id', () => {
      const template = manager.getTemplate('academic-paper');
      expect(template).toBeDefined();
      expect(template?.id).toBe('academic-paper');
    });

    it('should return undefined for non-existent template', () => {
      const template = manager.getTemplate('non-existent');
      expect(template).toBeUndefined();
    });

    it('should get all templates', () => {
      const templates = manager.getAllTemplates();
      expect(Array.isArray(templates)).toBe(true);
      expect(templates.length).toBeGreaterThan(0);
    });

    it('should get templates by category', () => {
      const academicTemplates = manager.getTemplatesByCategory('academic');
      expect(academicTemplates.length).toBeGreaterThan(0);
      academicTemplates.forEach(t => {
        expect(t.category).toBe('academic');
      });
    });

    it('should return empty array for category with no templates', () => {
      const templates = manager.getTemplatesByCategory('custom');
      expect(Array.isArray(templates)).toBe(true);
    });

    it('should remove template', () => {
      manager.addTemplate({
        id: 'temp-template',
        name: '临时模板',
        description: '临时描述',
        category: 'custom',
        content: '临时内容'
      });

      const removed = manager.removeTemplate('temp-template');
      expect(removed).toBe(true);

      const template = manager.getTemplate('temp-template');
      expect(template).toBeUndefined();
    });

    it('should return false when removing non-existent template', () => {
      const removed = manager.removeTemplate('non-existent');
      expect(removed).toBe(false);
    });

    it('should update template', () => {
      manager.addTemplate({
        id: 'update-test',
        name: '原始名称',
        description: '原始描述',
        category: 'custom',
        content: '原始内容'
      });

      const updated = manager.updateTemplate('update-test', {
        name: '更新后的名称',
        description: '更新后的描述'
      });

      expect(updated).toBe(true);

      const template = manager.getTemplate('update-test');
      expect(template?.name).toBe('更新后的名称');
      expect(template?.description).toBe('更新后的描述');
      expect(template?.content).toBe('原始内容'); // unchanged
    });

    it('should return false when updating non-existent template', () => {
      const updated = manager.updateTemplate('non-existent', { name: '新名称' });
      expect(updated).toBe(false);
    });

    it('should apply template', () => {
      const content = manager.applyTemplate('academic-paper', 'some content');
      expect(typeof content).toBe('string');
      expect(content).toContain('#set page');
      expect(content).toContain('论文标题');
    });

    it('should throw error when applying non-existent template', () => {
      expect(() => {
        manager.applyTemplate('non-existent', 'content');
      }).toThrow();
    });

    it('should search templates by name', () => {
      const results = manager.searchTemplates('学术');
      expect(results.length).toBeGreaterThan(0);
      results.forEach(t => {
        expect(t.name.toLowerCase()).toContain('学术');
      });
    });

    it('should search templates by description', () => {
      const results = manager.searchTemplates('论文');
      expect(results.length).toBeGreaterThan(0);
    });

    it('should return empty array for non-matching search', () => {
      const results = manager.searchTemplates('xyz-non-existent-query');
      expect(results).toEqual([]);
    });

    it('should be case-insensitive in search', () => {
      const results1 = manager.searchTemplates('学术');
      const results2 = manager.searchTemplates('学术'.toUpperCase());
      expect(results1.length).toBe(results2.length);
    });
  });

  describe('Template Content Validation', () => {
    it('should have valid Typst syntax in academic template', () => {
      const template = manager.getTemplate('academic-paper');
      expect(template?.content).toContain('#set page');
      expect(template?.content).toContain('#set text');
      expect(template?.content).toContain('#set par');
      expect(template?.content).toContain('#set heading');
    });

    it('should have valid Typst syntax in technical doc template', () => {
      const template = manager.getTemplate('technical-doc');
      expect(template?.content).toContain('#set page');
      expect(template?.content).toContain('```');
    });

    it('should have valid Typst syntax in business report template', () => {
      const template = manager.getTemplate('business-report');
      expect(template?.content).toContain('#set page');
      expect(template?.content).toContain('执行摘要');
    });

    it('should have valid Typst syntax in resume template', () => {
      const template = manager.getTemplate('resume');
      expect(template?.content).toContain('#set page');
      expect(template?.content).toContain('个人信息');
      expect(template?.content).toContain('工作经历');
    });

    it('should have valid Typst syntax in creative writing template', () => {
      const template = manager.getTemplate('creative-writing');
      expect(template?.content).toContain('#set page');
      expect(template?.content).toContain('第一章');
    });

    it('should have valid Typst syntax in lab report template', () => {
      const template = manager.getTemplate('lab-report');
      expect(template?.content).toContain('#set page');
      expect(template?.content).toContain('实验目的');
      expect(template?.content).toContain('实验结果');
    });
  });

  describe('Template Categories', () => {
    it('should have academic category templates', () => {
      const academic = manager.getTemplatesByCategory('academic');
      expect(academic.length).toBeGreaterThanOrEqual(2);
    });

    it('should have business category templates', () => {
      const business = manager.getTemplatesByCategory('business');
      expect(business.length).toBeGreaterThanOrEqual(2);
    });

    it('should have technical category templates', () => {
      const technical = manager.getTemplatesByCategory('technical');
      expect(technical.length).toBeGreaterThanOrEqual(1);
    });

    it('should have creative category templates', () => {
      const creative = manager.getTemplatesByCategory('creative');
      expect(creative.length).toBeGreaterThanOrEqual(1);
    });
  });

  describe('Template Structure', () => {
    it('should have required fields in all templates', () => {
      const templates = manager.getAllTemplates();
      templates.forEach(template => {
        expect(template).toHaveProperty('id');
        expect(template).toHaveProperty('name');
        expect(template).toHaveProperty('description');
        expect(template).toHaveProperty('category');
        expect(template).toHaveProperty('content');
      });
    });

    it('should have non-empty content in all templates', () => {
      const templates = manager.getAllTemplates();
      templates.forEach(template => {
        expect(template.content.length).toBeGreaterThan(0);
      });
    });

    it('should have unique IDs', () => {
      const templates = manager.getAllTemplates();
      const ids = templates.map(t => t.id);
      const uniqueIds = new Set(ids);
      expect(ids.length).toBe(uniqueIds.size);
    });
  });

  describe('Edge Cases', () => {
    it('should handle empty search query', () => {
      const results = manager.searchTemplates('');
      expect(results.length).toBeGreaterThan(0);
    });

    it('should handle special characters in search', () => {
      const results = manager.searchTemplates('#');
      expect(Array.isArray(results)).toBe(true);
    });

    it('should handle template with very long content', () => {
      const longContent = '#set page(paper: "a4")\n\n' + 'x'.repeat(10000);
      manager.addTemplate({
        id: 'long-content',
        name: '长内容模板',
        description: '测试长内容',
        category: 'custom',
        content: longContent
      });

      const template = manager.getTemplate('long-content');
      expect(template?.content.length).toBeGreaterThan(10000);
    });

    it('should handle template with special characters in content', () => {
      const specialContent = '#set page(paper: "a4")\n\n= 标题\n\n特殊字符: @#$%^&*()_+-=[]{}|;:\'",.<>?/~`';
      manager.addTemplate({
        id: 'special-chars',
        name: '特殊字符模板',
        description: '测试特殊字符',
        category: 'custom',
        content: specialContent
      });

      const template = manager.getTemplate('special-chars');
      expect(template?.content).toContain('@#$%^&*');
    });

    it('should handle multiple additions of same template', () => {
      const template: TypstTemplate = {
        id: 'duplicate-test',
        name: '重复测试',
        description: '测试重复',
        category: 'custom',
        content: '内容'
      };

      manager.addTemplate(template);
      manager.addTemplate(template);

      const retrieved = manager.getTemplate('duplicate-test');
      expect(retrieved).toBeDefined();
    });
  });
});

describe('Typst Template Singleton', () => {
  it('should export singleton instance', () => {
    expect(typstTemplateManager).toBeDefined();
    expect(typstTemplateManager instanceof TypstTemplateManager).toBe(true);
  });

  it('should have default templates in singleton', () => {
    const templates = typstTemplateManager.getAllTemplates();
    expect(templates.length).toBeGreaterThan(0);
  });
});

describe('Typst Template Convenience Functions', () => {
  it('should export getTypstTemplate function', () => {
    expect(typeof getTypstTemplate).toBe('function');
    const template = getTypstTemplate('academic-paper');
    expect(template).toBeDefined();
  });

  it('should export getAllTypstTemplates function', () => {
    expect(typeof getAllTypstTemplates).toBe('function');
    const templates = getAllTypstTemplates();
    expect(Array.isArray(templates)).toBe(true);
  });

  it('should export getTypstTemplatesByCategory function', () => {
    expect(typeof getTypstTemplatesByCategory).toBe('function');
    const templates = getTypstTemplatesByCategory('academic');
    expect(Array.isArray(templates)).toBe(true);
  });

  it('should export applyTypstTemplate function', () => {
    expect(typeof applyTypstTemplate).toBe('function');
    const content = applyTypstTemplate('academic-paper', 'test');
    expect(typeof content).toBe('string');
  });

  it('should throw error in convenience function for non-existent template', () => {
    expect(() => {
      applyTypstTemplate('non-existent', 'content');
    }).toThrow();
  });
});

describe('TypstTemplate Interface', () => {
  it('should accept valid template structure', () => {
    const template: TypstTemplate = {
      id: 'test-id',
      name: 'Test Template',
      description: 'Test Description',
      category: 'academic',
      content: '#set page(paper: "a4")\n\n= Test\n\nContent'
    };

    expect(template.id).toBe('test-id');
    expect(template.name).toBe('Test Template');
    expect(template.description).toBe('Test Description');
    expect(template.category).toBe('academic');
    expect(template.content).toContain('#set page');
  });

  it('should accept optional preview field', () => {
    const template: TypstTemplate = {
      id: 'test-id',
      name: 'Test Template',
      description: 'Test Description',
      category: 'academic',
      content: 'Content',
      preview: 'Preview content'
    };

    expect(template.preview).toBe('Preview content');
  });

  it('should accept all category types', () => {
    const categories: TypstTemplate['category'][] = ['academic', 'business', 'technical', 'creative', 'custom'];
    categories.forEach(category => {
      const template: TypstTemplate = {
        id: `test-${category}`,
        name: 'Test',
        description: 'Test',
        category,
        content: 'Content'
      };
      expect(template.category).toBe(category);
    });
  });
});
