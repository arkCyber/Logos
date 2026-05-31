/**
 * 航空航天级 Typst 文档转换器
 * 提供完整的 HTML/Markdown 到 Typst 转换功能
 */

import { logger, LogCategory } from './logger';
import { createError, ErrorCode, ErrorSeverity, ErrorCategory } from './errorHandler';

/**
 * Typst 文档配置接口
 */
export interface TypstConfig {
  paper: string;
  margin: { top: string; bottom: string; left: string; right: string };
  font: string;
  fontSize: string;
  lineHeight: string;
  paragraphSpacing: string;
  headingFont: string;
  headingBaseSize: string;
  headingScale: number;
}

/**
 * 默认 Typst 配置
 */
const DEFAULT_CONFIG: TypstConfig = {
  paper: 'a4',
  margin: { top: '25mm', bottom: '25mm', left: '20mm', right: '20mm' },
  font: 'SimSun',
  fontSize: '11pt',
  lineHeight: '17pt',
  paragraphSpacing: '6pt',
  headingFont: 'SimHei',
  headingBaseSize: '16pt',
  headingScale: 1
};

/**
 * Typst 转换器类
 */
export class TypstConverter {
  private config: TypstConfig;

  constructor(config: Partial<TypstConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    logger.info('Typst converter initialized', { config: this.config }, LogCategory.BUSINESS);
  }

  /**
   * 生成 Typst 文档头部
   */
  private generateHeader(): string {
    const header = `#set page(
  paper: "${this.config.paper}",
  margin: (left: ${this.config.margin.left}, right: ${this.config.margin.right}, top: ${this.config.margin.top}, bottom: ${this.config.margin.bottom})
)

#set text(
  font: "${this.config.font}",
  size: ${this.config.fontSize},
  lang: "zh"
)

`;
    return header;
  }

  /**
   * 转换 HTML 到 Typst
   */
  htmlToTypst(html: string): string {
    try {
      logger.info('Converting HTML to Typst', { htmlLength: html.length }, LogCategory.BUSINESS);

      let typst = html;

      // 转换标题
      typst = this.convertHeadings(typst);

      // 转换文本格式
      typst = this.convertTextFormatting(typst);

      // 转换代码块
      typst = this.convertCodeBlocks(typst);

      // 转换引用块
      typst = this.convertBlockquotes(typst);

      // 转换列表
      typst = this.convertLists(typst);

      // 转换表格
      typst = this.convertTables(typst);

      // 转换水平线
      typst = this.convertHorizontalRules(typst);

      // 转换段落
      typst = this.convertParagraphs(typst);

      // 转换链接
      typst = this.convertLinks(typst);

      // 转换图片
      typst = this.convertImages(typst);

      // 转换数学公式
      typst = this.convertMath(typst);

      // 转换换行符（必须在清理HTML之前）
      typst = this.convertLineBreaks(typst);

      // 清理HTML标签
      typst = this.cleanupHTML(typst);

      // 清理多余空白
      typst = this.cleanupWhitespace(typst);

      // 添加头部
      const header = this.generateHeader();
      const finalTypst = header + typst;

      logger.info('HTML to Typst conversion completed', { typstLength: finalTypst.length }, LogCategory.BUSINESS);

      return finalTypst;
    } catch (error) {
      logger.error('Failed to convert HTML to Typst', error, LogCategory.BUSINESS);
      throw createError(
        ErrorCode.CONVERSION_ERROR,
        'HTML to Typst conversion failed',
        ErrorSeverity.ERROR,
        ErrorCategory.BUSINESS
      );
    }
  }

  /**
   * 转换标题
   */
  private convertHeadings(html: string): string {
    const sizes = [
      { level: 1, typst: '=', size: this.config.headingBaseSize },
      { level: 2, typst: '==', size: `${Math.round(parseFloat(this.config.headingBaseSize) * this.config.headingScale)}pt` },
      { level: 3, typst: '===', size: `${Math.round(parseFloat(this.config.headingBaseSize) * Math.pow(this.config.headingScale, 2))}pt` },
      { level: 4, typst: '====', size: `${Math.round(parseFloat(this.config.headingBaseSize) * Math.pow(this.config.headingScale, 3))}pt` },
      { level: 5, typst: '=====', size: `${Math.round(parseFloat(this.config.headingBaseSize) * Math.pow(this.config.headingScale, 4))}pt` },
      { level: 6, typst: '======', size: `${Math.round(parseFloat(this.config.headingBaseSize) * Math.pow(this.config.headingScale, 5))}pt` }
    ];

    let result = html;
    sizes.forEach(({ level, typst }) => {
      const regex = new RegExp(`<h${level}[^>]*>(.*?)</h${level}>`, 'gs');
      result = result.replace(regex, (_, content) => {
        return `${typst} ${content}\n`;
      });
    });

    return result;
  }

  /**
   * 转换文本格式
   */
  private convertTextFormatting(html: string): string {
    let result = html;

    // 粗体
    result = result
      .replace(/<strong[^>]*>(.*?)<\/strong>/gs, '*$1*')
      .replace(/<b[^>]*>(.*?)<\/b>/gs, '*$1*');

    // 斜体
    result = result
      .replace(/<em[^>]*>(.*?)<\/em>/gs, '_$1_')
      .replace(/<i[^>]*>(.*?)<\/i>/gs, '_$1_');

    // 下划线
    result = result
      .replace(/<u[^>]*>(.*?)<\/u>/gs, '#underline[$1]');

    // 删除线
    result = result
      .replace(/<s[^>]*>(.*?)<\/s>/gs, '#strike[$1]')
      .replace(/<strike[^>]*>(.*?)<\/strike>/gs, '#strike[$1]')
      .replace(/<del[^>]*>(.*?)<\/del>/gs, '#strike[$1]');

    // 高亮
    result = result
      .replace(/<mark[^>]*>(.*?)<\/mark>/gs, '#highlight[$1]');

    // 上标
    result = result
      .replace(/<sup[^>]*>(.*?)<\/sup>/gs, '#super[$1]');

    // 下标
    result = result
      .replace(/<sub[^>]*>(.*?)<\/sub>/gs, '#sub[$1]');

    return result;
  }

  /**
   * 转换代码块
   */
  private convertCodeBlocks(html: string): string {
    let result = html;

    // 行内代码
    result = result.replace(/<code[^>]*>(.*?)<\/code>/gs, '`$1`');

    // 代码块
    result = result.replace(/<pre[^>]*><code[^>]*>(.*?)<\/code><\/pre>/gs, (_, content) => {
      return `\`\`\`\n${content}\n\`\`\`\n\n`;
    });

    result = result.replace(/<pre[^>]*>(.*?)<\/pre>/gs, (_, content) => {
      return `\`\`\`\n${content}\n\`\`\`\n\n`;
    });

    return result;
  }

  /**
   * 转换引用块
   */
  private convertBlockquotes(html: string): string {
    return html.replace(
      /<blockquote[^>]*>(.*?)<\/blockquote>/gs,
      (_, content) => {
        return `#block(fill: rgb("f5f5f5"), inset: 8pt, radius: 4pt)[\n  ${content}\n]\n\n`;
      }
    );
  }

  /**
   * 转换列表
   */
  private convertLists(html: string): string {
    let result = html;

    // 无序列表
    result = result.replace(/<ul[^>]*>(.*?)<\/ul>/gs, (_: string, listContent: string) => {
      const items = listContent.match(/<li[^>]*>(.*?)<\/li>/gs) || [];
      const typstItems = items
        .map((item: string) => {
          const content = item.replace(/<\/?li[^>]*>/g, '').trim();
          return `- ${content}`;
        })
        .join('\n');
      return `${typstItems}\n\n`;
    });

    // 有序列表
    result = result.replace(/<ol[^>]*>(.*?)<\/ol>/gs, (_: string, listContent: string) => {
      const items = listContent.match(/<li[^>]*>(.*?)<\/li>/gs) || [];
      const typstItems = items
        .map((item: string, index: number) => {
          const content = item.replace(/<\/?li[^>]*>/g, '').trim();
          return `${index + 1}. ${content}`;
        })
        .join('\n');
      return `${typstItems}\n\n`;
    });

    return result;
  }

  /**
   * 转换表格
   */
  private convertTables(html: string): string {
    return html.replace(/<table[^>]*>(.*?)<\/table>/gs, (_: string, tableContent: string) => {
      const rows = tableContent.match(/<tr[^>]*>(.*?)<\/tr>/gs) || [];
      if (rows.length === 0) {
        return '';
      }

      // 提取表头
      const headerRow = rows[0];
      if (!headerRow) {
        return '';
      }
      const headerCells = headerRow.match(/<(th|td)[^>]*>(.*?)<\/(th|td)>/gs) || [];
      const colCount = headerCells.length;

      // 构建表格
      let tableTypst = `#table(\n  columns: (${Array(colCount).fill('auto').join(', ')}),\n  stroke: 0.5pt,\n`;

      rows.forEach((row: string) => {
        const cells = row.match(/<(td|th)[^>]*>(.*?)<\/(td|th)>/gs) || [];
        const cellContents = cells.map((cell: string) => {
          const content = cell.replace(/<\/?(td|th)[^>]*>/g, '').trim();
          return `[${content}]`;
        });
        tableTypst += `  ${cellContents.join(', ')}\n`;
      });

      tableTypst += ')\n\n';
      return tableTypst;
    });
  }

  /**
   * 转换水平线
   */
  private convertHorizontalRules(html: string): string {
    return html.replace(/<hr\s*\/?>/g, '#line(length: 100%, stroke: 0.5pt)\n\n');
  }

  /**
   * 转换段落
   */
  private convertParagraphs(html: string): string {
    return html.replace(/<p[^>]*>([\s\S]*?)<\/p>/gs, (_, content) => {
      if (content.trim() === '') {
        return '';
      }
      // 保留段落内的换行符
      return `${content}\n\n`;
    });
  }

  /**
   * 转换链接
   */
  private convertLinks(html: string): string {
    return html.replace(/<a[^>]*href="([^"]*)"[^>]*>(.*?)<\/a>/gs, (_, url, text) => {
      return `#link("${url}")[${text}]`;
    });
  }

  /**
   * 转换图片
   */
  private convertImages(html: string): string {
    return html.replace(/<img[^>]*src="([^"]*)"[^>]*>/gs, (_, src) => {
      return `#image("${src}", width: 100%)\n\n`;
    });
  }

  /**
   * 转换换行符
   */
  private convertLineBreaks(html: string): string {
    // 将 <br> 转换为换行符
    return html.replace(/<br\s*\/?>/gi, '\n');
  }

  /**
   * 转换数学公式
   */
  private convertMath(html: string): string {
    let result = html;

    // LaTeX 行内公式
    result = result.replace(/\$\$([^$]+)\$\$/g, (_, math) => {
      return `$ ${math} $`;
    });

    result = result.replace(/\$([^$]+)\$/g, (_, math) => {
      return `$ ${math} $`;
    });

    return result;
  }

  /**
   * 清理HTML标签
   */
  private cleanupHTML(html: string): string {
    return html.replace(/<[^>]*>/g, '');
  }

  /**
   * 清理多余空白
   */
  private cleanupWhitespace(html: string): string {
    return html
      .replace(/\n{4,}/g, '\n\n\n')
      .replace(/^\s+|\s+$/gm, '');
  }

  /**
   * 转换 Markdown 到 Typst
   */
  markdownToTypst(markdown: string): string {
    try {
      logger.info('Converting Markdown to Typst', { markdownLength: markdown.length }, LogCategory.BUSINESS);

      let typst = markdown;

      // 转换标题
      typst = typst
        .replace(/^###### (.+)$/gm, '====== $1')
        .replace(/^##### (.+)$/gm, '===== $1')
        .replace(/^#### (.+)$/gm, '==== $1')
        .replace(/^### (.+)$/gm, '=== $1')
        .replace(/^## (.+)$/gm, '== $1')
        .replace(/^# (.+)$/gm, '= $1');

      // 转换粗体
      typst = typst.replace(/\*\*(.+?)\*\*/g, '*$1*');

      // 转换斜体
      typst = typst.replace(/\*(.+?)\*/g, '_$1_');

      // 转换代码
      typst = typst.replace(/`([^`]+)`/g, '`$1`');

      // 转换代码块
      typst = typst.replace(/```(\w+)?\n([\s\S]*?)```/g, (_, lang, code) => {
        return `\`\`\`\n${code}\n\`\`\``;
      });

      // 转换引用
      typst = typst.replace(/^> (.+)$/gm, '#block(fill: rgb("f5f5f5"), inset: 8pt, radius: 4pt)[$1]');

      // 转换列表
      typst = typst.replace(/^- (.+)$/gm, '- $1');
      typst = typst.replace(/^\d+\. (.+)$/gm, (match, content) => {
        const num = match.match(/^\d+/)?.[0] || '1';
        return `${num}. ${content}`;
      });

      // 转换水平线
      typst = typst.replace(/^---$/gm, '#line(length: 100%, stroke: 0.5pt)');

      // 转换链接
      typst = typst.replace(/\[([^\]]+)\]\(([^)]+)\)/g, '#link("$2")[$1]');

      // 转换图片
      typst = typst.replace(/!\[([^\]]*)\]\(([^)]+)\)/g, '#image("$2", width: 100%)');

      // 添加头部
      const header = this.generateHeader();
      const finalTypst = header + typst;

      logger.info('Markdown to Typst conversion completed', { typstLength: finalTypst.length }, LogCategory.BUSINESS);

      return finalTypst;
    } catch (error) {
      logger.error('Failed to convert Markdown to Typst', error, LogCategory.BUSINESS);
      throw createError(
        ErrorCode.CONVERSION_ERROR,
        'Markdown to Typst conversion failed',
        ErrorSeverity.ERROR,
        ErrorCategory.BUSINESS
      );
    }
  }

  /**
   * 更新配置
   */
  updateConfig(config: Partial<TypstConfig>): void {
    this.config = { ...this.config, ...config };
    logger.info('Typst converter configuration updated', { config: this.config }, LogCategory.BUSINESS);
  }

  /**
   * 获取当前配置
   */
  getConfig(): TypstConfig {
    return { ...this.config };
  }
}

// 导出单例
export const typstConverter = new TypstConverter();

// 导出便捷函数
export const typst = {
  convertHTML: (html: string) => typstConverter.htmlToTypst(html),
  convertMarkdown: (markdown: string) => typstConverter.markdownToTypst(markdown),
  updateConfig: (config: Partial<TypstConfig>) => typstConverter.updateConfig(config),
  getConfig: () => typstConverter.getConfig()
};
