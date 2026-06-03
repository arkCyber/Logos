/**
 * 航空航天级 Typst 模板系统
 * 提供预定义的 Typst 文档模板
 * 支持本地文件系统持久化存储
 */

import { logger, LogCategory } from './logger';

// Typst官方模板库URL
export const TYPST_TEMPLATE_MARKET_URL = 'https://typst.app/universe/search/?kind=templates';

// Check if Tauri is available
let isTauriAvailable = false;
let invokeTauri: any = null;

try {
  const tauriModule = await import('@tauri-apps/api/core');
  isTauriAvailable = tauriModule.isTauri();
  invokeTauri = tauriModule.invoke;
} catch (error) {
  logger.debug('Tauri not available, running in web mode', {}, LogCategory.SYSTEM);
}

export interface TypstTemplate {
  id: string;
  name: string;
  description: string;
  category: 'academic' | 'business' | 'technical' | 'creative' | 'custom';
  content: string;
  preview?: string;
}

/**
 * Typst 模板管理器类
 */
export class TypstTemplateManager {
  private templates: Map<string, TypstTemplate> = new Map();
  private loadedFromDisk: boolean = false;

  constructor() {
    this.initializeDefaultTemplates();
    // 异步加载磁盘模板，不阻塞构造函数
    this.loadTemplatesFromDisk().catch((error) => logger.error('Failed to load templates from disk in constructor', error, LogCategory.SYSTEM));
  }

  /**
   * 从磁盘加载模板
   */
  private async loadTemplatesFromDisk(): Promise<void> {
    try {
      if (isTauriAvailable && invokeTauri) {
        const templatesJson = await invokeTauri('list_templates');
        const diskTemplates: TypstTemplate[] = JSON.parse(templatesJson);
        
        diskTemplates.forEach(template => {
          this.templates.set(template.id, template);
        });
        
        this.loadedFromDisk = true;
        logger.debug('Loaded templates from disk', { count: diskTemplates.length }, LogCategory.SYSTEM);
      }
    } catch (error) {
      logger.error('Failed to load templates from disk', error, LogCategory.SYSTEM);
      // 继续使用默认模板
    }
  }

  /**
   * 初始化默认模板
   */
  private initializeDefaultTemplates(): void {
    // 学术论文模板
    this.addTemplate({
      id: 'academic-paper',
      name: '学术论文',
      description: '标准学术论文模板，包含标题、摘要、章节、参考文献',
      category: 'academic',
      content: `#set page(
  paper: "a4",
  margin: (x: 2.5cm, y: 2.5cm)
)

#set text(
  font: "Times New Roman",
  size: 12pt,
  lang: "zh"
)

#set par(
  justify: true,
  leading: 1.5em,
  spacing: 0.5em
)

#set heading(
  numbering: "1.1",
  outlined: true
)

= 论文标题

== 摘要

在此处输入摘要内容...

== 关键词

关键词1, 关键词2, 关键词3

== 1. 引言

在此处输入引言内容...

== 2. 正文

在此处输入正文内容...

== 3. 结论

在此处输入结论内容...

== 参考文献

在此处输入参考文献...`
    });

    // 技术文档模板
    this.addTemplate({
      id: 'technical-doc',
      name: '技术文档',
      description: '技术文档模板，包含代码块、表格、图表支持',
      category: 'technical',
      content: `#set page(
  paper: "a4",
  margin: (x: 2cm, y: 2cm)
)

#set text(
  font: "Consolas",
  size: 11pt,
  lang: "zh"
)

#set par(
  justify: false,
  leading: 1.4em,
  spacing: 0.3em
)

#set heading(
  numbering: "1.1",
  outlined: true
)

= 技术文档标题

== 概述

在此处输入概述内容...

== 安装指南

\`\`\`
# 安装命令示例
npm install package-name
\`\`\`

== API 文档

在此处输入 API 文档内容...

== 故障排除

在此处输入故障排除内容...`
    });

    // 商业报告模板
    this.addTemplate({
      id: 'business-report',
      name: '商业报告',
      description: '商业报告模板，包含执行摘要、数据分析、建议',
      category: 'business',
      content: `#set page(
  paper: "a4",
  margin: (x: 2cm, y: 2.5cm)
)

#set text(
  font: "Arial",
  size: 11pt,
  lang: "zh"
)

#set par(
  justify: true,
  leading: 1.3em,
  spacing: 0.4em
)

#set heading(
  numbering: "1.1",
  outlined: true
)

= 商业报告

== 执行摘要

在此处输入执行摘要内容...

== 市场分析

在此处输入市场分析内容...

== 财务数据

在此处输入财务数据内容...

== 建议与结论

在此处输入建议与结论内容...`
    });

    // 简历模板
    this.addTemplate({
      id: 'resume',
      name: '简历',
      description: '个人简历模板，包含个人信息、工作经历、技能',
      category: 'business',
      content: `#set page(
  paper: "a4",
  margin: (x: 2cm, y: 2cm)
)

#set text(
  font: "Arial",
  size: 11pt,
  lang: "zh"
)

#set par(
  justify: true,
  leading: 1.2em,
  spacing: 0.3em
)

= 个人简历

== 个人信息

*姓名：* 您的姓名
*邮箱：* your.email@example.com
*电话：* +86 123 4567 8900

== 工作经历

*公司名称 - 职位*
*时间：* 2020 - 至今

在此处描述您的工作职责和成就...

== 教育背景

*学校名称 - 学位*
*时间：* 2016 - 2020

在此处描述您的教育经历...

== 技能

*编程语言：* Python, JavaScript, TypeScript
*工具：* Git, Docker, VS Code
*语言：* 中文（母语），英语（流利）`
    });

    // 创意写作模板
    this.addTemplate({
      id: 'creative-writing',
      name: '创意写作',
      description: '创意写作模板，包含章节、对话、场景描述',
      category: 'creative',
      content: `#set page(
  paper: "a4",
  margin: (x: 2.5cm, y: 2.5cm)
)

#set text(
  font: "Georgia",
  size: 12pt,
  lang: "zh"
)

#set par(
  justify: true,
  leading: 1.6em,
  spacing: 0.8em
)

= 作品标题

== 第一章

在此处输入第一章内容...

== 第二章

在此处输入第二章内容...

== 第三章

在此处输入第三章内容...`
    });

    // 实验报告模板
    this.addTemplate({
      id: 'lab-report',
      name: '实验报告',
      description: '实验报告模板，包含实验目的、方法、结果、讨论',
      category: 'academic',
      content: `#set page(
  paper: "a4",
  margin: (x: 2cm, y: 2.5cm)
)

#set text(
  font: "Times New Roman",
  size: 12pt,
  lang: "zh"
)

#set par(
  justify: true,
  leading: 1.5em,
  spacing: 0.5em
)

#set heading(
  numbering: "1.1",
  outlined: true
)

= 实验报告

== 实验目的

在此处输入实验目的...

== 实验原理

在此处输入实验原理...

== 实验方法

在此处输入实验方法...

== 实验结果

在此处输入实验结果...

== 讨论

在此处输入讨论内容...

== 结论

在此处输入结论...`
    });
  }

  /**
   * 添加模板
   */
  async addTemplate(template: TypstTemplate): Promise<void> {
    this.templates.set(template.id, template);
    
    // 保存到磁盘 (仅在 Tauri 环境中)
    try {
      if (isTauriAvailable && invokeTauri) {
        await invokeTauri('save_template', {
          id: template.id,
          name: template.name,
          description: template.description,
          category: template.category,
          content: template.content,
          preview: template.preview
        });
        logger.debug('Saved template to disk', { id: template.id }, LogCategory.SYSTEM);
      }
    } catch (error) {
      logger.error('Failed to save template to disk', error, LogCategory.SYSTEM);
    }
  }

  /**
   * 获取模板
   */
  getTemplate(id: string): TypstTemplate | undefined {
    return this.templates.get(id);
  }

  /**
   * 获取所有模板
   */
  getAllTemplates(): TypstTemplate[] {
    return Array.from(this.templates.values());
  }

  /**
   * 按类别获取模板
   */
  getTemplatesByCategory(category: TypstTemplate['category']): TypstTemplate[] {
    return this.getAllTemplates().filter(t => t.category === category);
  }

  /**
   * 删除模板
   */
  async removeTemplate(id: string): Promise<boolean> {
    // 从内存中删除
    const deleted = this.templates.delete(id);
    
    // 从磁盘删除 (仅在 Tauri 环境中)
    if (deleted) {
      try {
        if (isTauriAvailable && invokeTauri) {
          await invokeTauri('delete_template', { id });
          logger.debug('Deleted template from disk', { id }, LogCategory.SYSTEM);
        }
      } catch (error) {
        logger.error('Failed to delete template from disk', error, LogCategory.SYSTEM);
      }
    }
    
    return deleted;
  }

  /**
   * 更新模板
   */
  updateTemplate(id: string, updates: Partial<TypstTemplate>): boolean {
    const template = this.templates.get(id);
    if (!template) {
      return false;
    }
    this.templates.set(id, { ...template, ...updates });
    return true;
  }

  /**
   * 应用模板到内容
   */
  applyTemplate(id: string, _content: string): string {
    const template = this.getTemplate(id);
    if (!template) {
      throw new Error(`Template with id "${id}" not found`);
    }
    return template.content;
  }

  /**
   * 搜索模板
   */
  searchTemplates(query: string): TypstTemplate[] {
    const lowerQuery = query.toLowerCase();
    return this.getAllTemplates().filter(
      t =>
        t.name.toLowerCase().includes(lowerQuery) ||
        t.description.toLowerCase().includes(lowerQuery)
    );
  }

  /**
   * 获取模板存储目录
   */
  async getTemplatesDirectory(): Promise<string> {
    try {
      if (isTauriAvailable && invokeTauri) {
        return await invokeTauri('get_templates_directory');
      }
      return '';
    } catch (error) {
      logger.error('Failed to get templates directory', error, LogCategory.SYSTEM);
      return '';
    }
  }

  /**
   * 从URL下载模板
   */
  async downloadTemplateFromUrl(url: string): Promise<TypstTemplate> {
    try {
      if (isTauriAvailable && invokeTauri) {
        const templateJson = await invokeTauri('download_template_from_url', { url });
        const template: TypstTemplate = JSON.parse(templateJson);
        
        // 添加到内存中的模板
        this.templates.set(template.id, template);
        
        logger.debug('Downloaded template from URL', { id: template.id, url }, LogCategory.SYSTEM);
        return template;
      }
      throw new Error('Tauri not available');
    } catch (error) {
      logger.error('Failed to download template from URL', error, LogCategory.SYSTEM);
      throw error;
    }
  }
}

// 导出单例
export const typstTemplateManager = new TypstTemplateManager();

// 导出便捷函数
export const getTypstTemplate = (id: string) => typstTemplateManager.getTemplate(id);
export const getAllTypstTemplates = () => typstTemplateManager.getAllTemplates();
export const getTypstTemplatesByCategory = (category: TypstTemplate['category']) =>
  typstTemplateManager.getTemplatesByCategory(category);
export const applyTypstTemplate = (id: string, content: string) =>
  typstTemplateManager.applyTemplate(id, content);
export const downloadTypstTemplate = (url: string) =>
  typstTemplateManager.downloadTemplateFromUrl(url);
