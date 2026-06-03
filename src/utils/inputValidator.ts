/**
 * 航空航天级输入验证和清理系统
 * 提供全面的输入验证、清理和安全防护
 */

import { createError, ErrorCode, ErrorSeverity, ErrorCategory } from './errorHandler';

/**
 * 验证规则接口
 */
export interface ValidationRule {
  name: string;
  validate: (value: any) => boolean;
  errorMessage: string;
  severity?: ErrorSeverity;
}

/**
 * 清理规则接口
 */
export interface SanitizationRule {
  name: string;
  sanitize: (value: any) => any;
}

/**
 * 输入验证器类
 */
export class InputValidator {
  private rules: ValidationRule[] = [];

  /**
   * 添加验证规则
   */
  addRule(rule: ValidationRule): void {
    this.rules.push(rule);
  }

  /**
   * 验证输入
   */
  validate(value: any, _context?: string): { valid: boolean; errors: string[] } {
    const errors: string[] = [];

    for (const rule of this.rules) {
      if (!rule.validate(value)) {
        errors.push(rule.errorMessage);
      }
    }

    return {
      valid: errors.length === 0,
      errors
    };
  }

  /**
   * 验证并抛出错误
   */
  validateOrThrow(value: any, context?: string): void {
    const result = this.validate(value, context);
    if (!result.valid) {
      throw createError(
        ErrorCode.VALIDATION_ERROR,
        result.errors.join('; '),
        ErrorSeverity.ERROR,
        ErrorCategory.VALIDATION,
        {
          timestamp: Date.now(),
          component: context,
          additionalData: { value: typeof value === 'string' ? value.substring(0, 100) : value }
        }
      );
    }
  }

  /**
   * 清除所有规则
   */
  clearRules(): void {
    this.rules = [];
  }
}

/**
 * 输入清理器类
 */
export class InputSanitizer {
  private rules: SanitizationRule[] = [];

  /**
   * 添加清理规则
   */
  addRule(rule: SanitizationRule): void {
    this.rules.push(rule);
  }

  /**
   * 清理输入
   */
  sanitize(value: any): any {
    let result = value;

    for (const rule of this.rules) {
      result = rule.sanitize(result);
    }

    return result;
  }

  /**
   * 清除所有规则
   */
  clearRules(): void {
    this.rules = [];
  }
}

/**
 * 常用验证规则
 */
export const ValidationRules = {
  /**
   * 非空验证
   */
  notEmpty: (fieldName: string = '字段'): ValidationRule => ({
    name: 'notEmpty',
    validate: (value: any) => {
      if (value === null || value === undefined) {
return false;
}
      if (typeof value === 'string') {
return value.trim().length > 0;
}
      if (Array.isArray(value)) {
return value.length > 0;
}
      return true;
    },
    errorMessage: `${fieldName}不能为空`,
    severity: ErrorSeverity.ERROR
  }),

  /**
   * 长度验证
   */
  length: (min: number, max: number, fieldName: string = '字段'): ValidationRule => ({
    name: 'length',
    validate: (value: any) => {
      if (typeof value !== 'string') {
return false;
}
      const len = value.length;
      return len >= min && len <= max;
    },
    errorMessage: `${fieldName}长度必须在${min}到${max}之间`,
    severity: ErrorSeverity.ERROR
  }),

  /**
   * 数字验证
   */
  isNumber: (fieldName: string = '字段'): ValidationRule => ({
    name: 'isNumber',
    validate: (value: any) => {
      return typeof value === 'number' && !isNaN(value);
    },
    errorMessage: `${fieldName}必须是数字`,
    severity: ErrorSeverity.ERROR
  }),

  /**
   * 整数验证
   */
  isInteger: (fieldName: string = '字段'): ValidationRule => ({
    name: 'isInteger',
    validate: (value: any) => {
      return Number.isInteger(value);
    },
    errorMessage: `${fieldName}必须是整数`,
    severity: ErrorSeverity.ERROR
  }),

  /**
   * 正数验证
   */
  isPositive: (fieldName: string = '字段'): ValidationRule => ({
    name: 'isPositive',
    validate: (value: any) => {
      return typeof value === 'number' && value > 0;
    },
    errorMessage: `${fieldName}必须是正数`,
    severity: ErrorSeverity.ERROR
  }),

  /**
   * 范围验证
   */
  range: (min: number, max: number, fieldName: string = '字段'): ValidationRule => ({
    name: 'range',
    validate: (value: any) => {
      return typeof value === 'number' && value >= min && value <= max;
    },
    errorMessage: `${fieldName}必须在${min}到${max}之间`,
    severity: ErrorSeverity.ERROR
  }),

  /**
   * 邮箱验证
   */
  email: (fieldName: string = '邮箱'): ValidationRule => ({
    name: 'email',
    validate: (value: any) => {
      if (typeof value !== 'string') {
return false;
}
      const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
      return emailRegex.test(value);
    },
    errorMessage: `${fieldName}格式不正确`,
    severity: ErrorSeverity.ERROR
  }),

  /**
   * URL验证
   */
  url: (fieldName: string = 'URL'): ValidationRule => ({
    name: 'url',
    validate: (value: any) => {
      if (typeof value !== 'string') {
return false;
}
      try {
        new URL(value);
        return true;
      } catch {
        return false;
      }
    },
    errorMessage: `${fieldName}格式不正确`,
    severity: ErrorSeverity.ERROR
  }),

  /**
   * 正则表达式验证
   */
  pattern: (regex: RegExp, fieldName: string = '字段'): ValidationRule => ({
    name: 'pattern',
    validate: (value: any) => {
      if (typeof value !== 'string') {
return false;
}
      return regex.test(value);
    },
    errorMessage: `${fieldName}格式不正确`,
    severity: ErrorSeverity.ERROR
  }),

  /**
   * 自定义验证
   */
  custom: (validator: (value: any) => boolean, errorMessage: string): ValidationRule => ({
    name: 'custom',
    validate: validator,
    errorMessage,
    severity: ErrorSeverity.ERROR
  })
};

/**
 * 常用清理规则
 */
export const SanitizationRules = {
  /**
   * 去除首尾空格
   */
  trim: (): SanitizationRule => ({
    name: 'trim',
    sanitize: (value: any) => {
      if (typeof value === 'string') {
        return value.trim();
      }
      return value;
    }
  }),

  /**
   * 去除所有空格
   */
  removeAllWhitespace: (): SanitizationRule => ({
    name: 'removeAllWhitespace',
    sanitize: (value: any) => {
      if (typeof value === 'string') {
        return value.replace(/\s+/g, '');
      }
      return value;
    }
  }),

  /**
   * 转换为小写
   */
  toLowerCase: (): SanitizationRule => ({
    name: 'toLowerCase',
    sanitize: (value: any) => {
      if (typeof value === 'string') {
        return value.toLowerCase();
      }
      return value;
    }
  }),

  /**
   * 转换为大写
   */
  toUpperCase: (): SanitizationRule => ({
    name: 'toUpperCase',
    sanitize: (value: any) => {
      if (typeof value === 'string') {
        return value.toUpperCase();
      }
      return value;
    }
  }),

  /**
   * HTML转义（防止XSS）
   */
  escapeHTML: (): SanitizationRule => ({
    name: 'escapeHTML',
    sanitize: (value: any) => {
      if (typeof value === 'string') {
        const escapeMap: Record<string, string> = {
          '&': '&amp;',
          '<': '&lt;',
          '>': '&gt;',
          '"': '&quot;',
          "'": '&#39;'
        };
        return value.replace(/[&<>"']/g, char => escapeMap[char]);
      }
      return value;
    }
  }),

  /**
   * 移除HTML标签
   */
  stripHTML: (): SanitizationRule => ({
    name: 'stripHTML',
    sanitize: (value: any) => {
      if (typeof value === 'string') {
        return value.replace(/<[^>]*>/g, '');
      }
      return value;
    }
  }),

  /**
   * 移除脚本标签
   */
  removeScripts: (): SanitizationRule => ({
    name: 'removeScripts',
    sanitize: (value: any) => {
      if (typeof value === 'string') {
        return value.replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '');
      }
      return value;
    }
  }),

  /**
   * 移除危险的事件处理器
   */
  removeEventHandlers: (): SanitizationRule => ({
    name: 'removeEventHandlers',
    sanitize: (value: any) => {
      if (typeof value === 'string') {
        return value.replace(/\s*on\w+\s*=\s*["'][^"']*["']/gi, '');
      }
      return value;
    }
  }),

  /**
   * SQL注入防护
   */
  preventSQLInjection: (): SanitizationRule => ({
    name: 'preventSQLInjection',
    sanitize: (value: any) => {
      if (typeof value === 'string') {
        // 移除常见的SQL注入模式
        const sqlPatterns = [
          /(\b(SELECT|INSERT|UPDATE|DELETE|DROP|ALTER|CREATE|EXEC|UNION)\b)/gi,
          /(--|#|\/\*|\*\/)/g,
          /(\bOR\b|\bAND\b).*=.*=/gi,
          /['";]/g
        ];

        let result = value;
        for (const pattern of sqlPatterns) {
          result = result.replace(pattern, '');
        }
        return result;
      }
      return value;
    }
  }),

  /**
   * 移除控制字符
   */
  removeControlChars: (): SanitizationRule => ({
    name: 'removeControlChars',
    sanitize: (value: any) => {
      if (typeof value === 'string') {
        // eslint-disable-next-line no-control-regex
        return value.replace(/[\x00-\x1F\x7F]/g, '');
      }
      return value;
    }
  }),

  /**
   * 限制最大长度
   */
  maxLength: (max: number): SanitizationRule => ({
    name: 'maxLength',
    sanitize: (value: any) => {
      if (typeof value === 'string' && value.length > max) {
        return value.substring(0, max);
      }
      return value;
    }
  }),

  /**
   * 自定义清理
   */
  custom: (sanitizer: (value: any) => any): SanitizationRule => ({
    name: 'custom',
    sanitize: sanitizer
  })
};

/**
 * 文本验证器（航空航天标准）
 */
export class TextValidator {
  private validator: InputValidator;
  private sanitizer: InputSanitizer;

  constructor() {
    this.validator = new InputValidator();
    this.sanitizer = new InputSanitizer();
  }

  /**
   * 添加验证规则
   */
  addValidationRule(rule: ValidationRule): void {
    this.validator.addRule(rule);
  }

  /**
   * 添加清理规则
   */
  addSanitizationRule(rule: SanitizationRule): void {
    this.sanitizer.addRule(rule);
  }

  /**
   * 验证文本
   */
  validate(text: string): { valid: boolean; errors: string[] } {
    return this.validator.validate(text);
  }

  /**
   * 清理文本
   */
  sanitize(text: string): string {
    return this.sanitizer.sanitize(text);
  }

  /**
   * 验证并清理文本
   */
  validateAndSanitize(text: string): { valid: boolean; errors: string[]; sanitized: string } {
    const sanitized = this.sanitize(text);
    const validation = this.validator.validate(sanitized);
    return {
      ...validation,
      sanitized
    };
  }

  /**
   * 验证并清理文本，如果验证失败则抛出错误
   */
  validateAndSanitizeOrThrow(text: string): string {
    const result = this.validateAndSanitize(text);
    if (!result.valid) {
      throw createError(
        ErrorCode.VALIDATION_ERROR,
        result.errors.join('; '),
        ErrorSeverity.ERROR,
        ErrorCategory.VALIDATION
      );
    }
    return result.sanitized;
  }
}

/**
 * 创建安全的文本验证器（默认配置）
 */
export function createSafeTextValidator(): TextValidator {
  const validator = new TextValidator();

  // 添加默认的安全清理规则
  validator.addSanitizationRule(SanitizationRules.trim());
  validator.addSanitizationRule(SanitizationRules.removeScripts());
  validator.addSanitizationRule(SanitizationRules.removeEventHandlers());
  validator.addSanitizationRule(SanitizationRules.removeControlChars());

  // 添加默认的验证规则
  validator.addValidationRule(ValidationRules.notEmpty('文本'));
  validator.addValidationRule(ValidationRules.length(1, 100000, '文本'));

  return validator;
}

/**
 * 创建HTML内容验证器
 */
export function createHTMLValidator(): TextValidator {
  const validator = new TextValidator();

  // HTML特定的清理规则
  validator.addSanitizationRule(SanitizationRules.trim());
  validator.addSanitizationRule(SanitizationRules.removeScripts());
  validator.addSanitizationRule(SanitizationRules.removeEventHandlers());

  // HTML验证规则
  validator.addValidationRule(ValidationRules.notEmpty('HTML内容'));
  validator.addValidationRule(ValidationRules.length(1, 1000000, 'HTML内容'));

  return validator;
}

/**
 * 创建文件名验证器
 */
export function createFilenameValidator(): TextValidator {
  const validator = new TextValidator();

  // 文件名特定的清理规则
  validator.addSanitizationRule(SanitizationRules.trim());
  validator.addSanitizationRule(SanitizationRules.removeControlChars());

  // 移除非法字符
  validator.addSanitizationRule({
    name: 'removeIllegalChars',
    sanitize: (value: any) => {
      if (typeof value === 'string') {
        return value.replace(/[<>:"/\\|?*]/g, '');
      }
      return value;
    }
  });

  // 文件名验证规则
  validator.addValidationRule(ValidationRules.notEmpty('文件名'));
  validator.addValidationRule(ValidationRules.length(1, 255, '文件名'));
  validator.addValidationRule(ValidationRules.pattern(/^[^<>:"/\\|?*]+$/, '文件名'));

  return validator;
}

/**
 * 验证对象属性
 */
export function validateObject(
  obj: Record<string, any>,
  schema: Record<string, ValidationRule[]>
): { valid: boolean; errors: Record<string, string[]> } {
  const errors: Record<string, string[]> = {};
  let valid = true;

  for (const [key, rules] of Object.entries(schema)) {
    const value = obj[key];
    const fieldErrors: string[] = [];

    for (const rule of rules) {
      if (!rule.validate(value)) {
        fieldErrors.push(rule.errorMessage);
      }
    }

    if (fieldErrors.length > 0) {
      errors[key] = fieldErrors;
      valid = false;
    }
  }

  return { valid, errors };
}

/**
 * 清理对象属性
 */
export function sanitizeObject(
  obj: Record<string, any>,
  schema: Record<string, SanitizationRule[]>
): Record<string, any> {
  const result: Record<string, any> = { ...obj };

  for (const [key, rules] of Object.entries(schema)) {
    let value = result[key];
    for (const rule of rules) {
      value = rule.sanitize(value);
    }
    result[key] = value;
  }

  return result;
}

/**
 * 验证并清理对象
 */
export function validateAndSanitizeObject(
  obj: Record<string, any>,
  validationSchema: Record<string, ValidationRule[]>,
  sanitizationSchema: Record<string, SanitizationRule[]>
): { valid: boolean; errors: Record<string, string[]>; sanitized: Record<string, any> } {
  const sanitized = sanitizeObject(obj, sanitizationSchema);
  const validation = validateObject(sanitized, validationSchema);

  return {
    ...validation,
    sanitized
  };
}
