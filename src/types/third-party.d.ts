/**
 * 第三方库类型声明
 * 为没有 TypeScript 定义的库提供类型支持
 */

// html-to-rtf
declare module 'html-to-rtf' {
  /**
   * 将 HTML 转换为 RTF 格式
   * @param html - HTML 字符串
   * @returns RTF 格式字符串
   */
  function htmlToRtf(html: string): string;
  export default htmlToRtf;
}

// mammoth
declare module 'mammoth' {
  interface ConvertOptions {
    arrayBuffer?: ArrayBuffer;
    path?: string;
    buffer?: Buffer;
  }

  interface ConvertResult {
    value: string;
    messages: Array<{
      type: string;
      message: string;
    }>;
  }

  interface ImageOptions {
    convertImage?: (image: any) => Promise<{ src: string }>;
  }

  /**
   * 将 DOCX 转换为 HTML
   */
  export function convertToHtml(
    options: ConvertOptions,
    imageOptions?: ImageOptions
  ): Promise<ConvertResult>;

  /**
   * 提取纯文本
   */
  export function extractRawText(options: ConvertOptions): Promise<ConvertResult>;

  /**
   * 转换为 Markdown
   */
  export function convertToMarkdown(options: ConvertOptions): Promise<ConvertResult>;
}

// katex
declare module 'katex' {
  interface KatexOptions {
    displayMode?: boolean;
    throwOnError?: boolean;
    errorColor?: string;
    macros?: Record<string, string>;
    trust?: boolean;
    strict?: boolean | 'warn' | 'error';
    output?: 'html' | 'mathml' | 'htmlAndMathml';
    fleqn?: boolean;
    leqno?: boolean;
    minRuleThickness?: number;
    colorIsTextColor?: boolean;
    maxSize?: number;
    maxExpand?: number;
  }

  /**
   * 渲染数学公式为 HTML 字符串
   */
  export function renderToString(tex: string, options?: KatexOptions): string;

  /**
   * 渲染数学公式到 DOM 元素
   */
  export function render(tex: string, element: HTMLElement, options?: KatexOptions): void;

  /**
   * KaTeX 版本
   */
  export const version: string;
}

// typo-js
declare module 'typo-js' {
  export default class Typo {
    /**
     * 创建拼写检查器
     * @param dictionary - 字典名称 (如 'en_US')
     * @param affData - 词缀数据
     * @param dicData - 字典数据
     */
    constructor(dictionary: string, affData?: string, dicData?: string);

    /**
     * 检查单词拼写
     * @param word - 要检查的单词
     * @returns 拼写是否正确
     */
    check(word: string): boolean;

    /**
     * 获取拼写建议
     * @param word - 拼写错误的单词
     * @param limit - 建议数量限制
     * @returns 建议的单词列表
     */
    suggest(word: string, limit?: number): string[];
  }
}

// luckysheet
declare module 'luckysheet' {
  interface LuckysheetOptions {
    container?: string;
    title?: string;
    lang?: string;
    data?: any[];
    plugins?: string[];
    showtoolbar?: boolean;
    showinfobar?: boolean;
    showsheetbar?: boolean;
    showstatisticBar?: boolean;
    sheetFormulaBar?: boolean;
    enableAddRow?: boolean;
    enableAddCol?: boolean;
    userInfo?: string | boolean;
    myFolderUrl?: string;
    loadUrl?: string;
    updateUrl?: string;
  }

  export function create(options: LuckysheetOptions): void;
  export function destroy(): void;
  export function getAllSheets(): any[];
  export function getSheet(index?: number): any;
}

// file-saver
declare module 'file-saver' {
  /**
   * 保存文件
   * @param data - 文件数据 (Blob 或 File)
   * @param filename - 文件名
   * @param options - 保存选项
   */
  export function saveAs(
    data: Blob | File,
    filename?: string,
    options?: { autoBom?: boolean }
  ): void;
}
