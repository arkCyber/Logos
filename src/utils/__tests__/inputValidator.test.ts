import { describe, it, expect, beforeEach } from 'vitest';
import {
  InputValidator,
  InputSanitizer,
  ValidationRules,
  SanitizationRules,
  TextValidator,
  createSafeTextValidator,
  createHTMLValidator,
  createFilenameValidator,
  validateObject,
  sanitizeObject,
  validateAndSanitizeObject
} from '../inputValidator';

describe('InputValidator (Aerospace Grade)', () => {
  describe('InputValidator', () => {
    let validator: InputValidator;

    beforeEach(() => {
      validator = new InputValidator();
    });

    it('should validate successfully with no rules', () => {
      const result = validator.validate('test');
      expect(result.valid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it('should validate with passing rule', () => {
      validator.addRule(ValidationRules.notEmpty('Test field'));
      const result = validator.validate('test');

      expect(result.valid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it('should fail validation with failing rule', () => {
      validator.addRule(ValidationRules.notEmpty('Test field'));
      const result = validator.validate('');

      expect(result.valid).toBe(false);
      expect(result.errors).toHaveLength(1);
      expect(result.errors[0]).toContain('不能为空');
    });

    it('should throw error on validation failure', () => {
      validator.addRule(ValidationRules.notEmpty('Test field'));

      expect(() => {
        validator.validateOrThrow('');
      }).toThrow();
    });

    it('should clear rules', () => {
      validator.addRule(ValidationRules.notEmpty('Test field'));
      validator.clearRules();

      const result = validator.validate('');
      expect(result.valid).toBe(true);
    });
  });

  describe('InputSanitizer', () => {
    let sanitizer: InputSanitizer;

    beforeEach(() => {
      sanitizer = new InputSanitizer();
    });

    it('should return original value with no rules', () => {
      const result = sanitizer.sanitize('  test  ');
      expect(result).toBe('  test  ');
    });

    it('should apply trim rule', () => {
      sanitizer.addRule(SanitizationRules.trim());
      const result = sanitizer.sanitize('  test  ');

      expect(result).toBe('test');
    });

    it('should apply multiple rules', () => {
      sanitizer.addRule(SanitizationRules.trim());
      sanitizer.addRule(SanitizationRules.toLowerCase());

      const result = sanitizer.sanitize('  TEST  ');
      expect(result).toBe('test');
    });

    it('should clear rules', () => {
      sanitizer.addRule(SanitizationRules.trim());
      sanitizer.clearRules();

      const result = sanitizer.sanitize('  test  ');
      expect(result).toBe('  test  ');
    });
  });

  describe('ValidationRules', () => {
    it('should validate not empty', () => {
      const rule = ValidationRules.notEmpty('Field');
      expect(rule.validate('test')).toBe(true);
      expect(rule.validate('')).toBe(false);
      expect(rule.validate(null)).toBe(false);
      expect(rule.validate(undefined)).toBe(false);
    });

    it('should validate length', () => {
      const rule = ValidationRules.length(3, 10, 'Field');
      expect(rule.validate('test')).toBe(true);
      expect(rule.validate('ab')).toBe(false);
      expect(rule.validate('abcdefghijk')).toBe(false);
    });

    it('should validate number', () => {
      const rule = ValidationRules.isNumber('Field');
      expect(rule.validate(123)).toBe(true);
      expect(rule.validate('123')).toBe(false);
      expect(rule.validate(NaN)).toBe(false);
    });

    it('should validate integer', () => {
      const rule = ValidationRules.isInteger('Field');
      expect(rule.validate(123)).toBe(true);
      expect(rule.validate(123.45)).toBe(false);
    });

    it('should validate positive', () => {
      const rule = ValidationRules.isPositive('Field');
      expect(rule.validate(123)).toBe(true);
      expect(rule.validate(-123)).toBe(false);
      expect(rule.validate(0)).toBe(false);
    });

    it('should validate range', () => {
      const rule = ValidationRules.range(1, 10, 'Field');
      expect(rule.validate(5)).toBe(true);
      expect(rule.validate(0)).toBe(false);
      expect(rule.validate(11)).toBe(false);
    });

    it('should validate email', () => {
      const rule = ValidationRules.email('Email');
      expect(rule.validate('test@example.com')).toBe(true);
      expect(rule.validate('invalid')).toBe(false);
    });

    it('should validate URL', () => {
      const rule = ValidationRules.url('URL');
      expect(rule.validate('https://example.com')).toBe(true);
      expect(rule.validate('invalid')).toBe(false);
    });

    it('should validate pattern', () => {
      const rule = ValidationRules.pattern(/^[a-z]+$/, 'Field');
      expect(rule.validate('test')).toBe(true);
      expect(rule.validate('Test')).toBe(false);
      expect(rule.validate('test123')).toBe(false);
    });

    it('should validate with custom rule', () => {
      const rule = ValidationRules.custom(
        (value: any) => typeof value === 'string' && value.startsWith('prefix'),
        'Must start with prefix'
      );

      expect(rule.validate('prefix-test')).toBe(true);
      expect(rule.validate('test')).toBe(false);
    });
  });

  describe('SanitizationRules', () => {
    it('should trim whitespace', () => {
      const rule = SanitizationRules.trim();
      expect(rule.sanitize('  test  ')).toBe('test');
    });

    it('should remove all whitespace', () => {
      const rule = SanitizationRules.removeAllWhitespace();
      expect(rule.sanitize('  t e s t  ')).toBe('test');
    });

    it('should convert to lowercase', () => {
      const rule = SanitizationRules.toLowerCase();
      expect(rule.sanitize('TEST')).toBe('test');
    });

    it('should convert to uppercase', () => {
      const rule = SanitizationRules.toUpperCase();
      expect(rule.sanitize('test')).toBe('TEST');
    });

    it('should escape HTML', () => {
      const rule = SanitizationRules.escapeHTML();
      expect(rule.sanitize('<script>alert("xss")</script>')).toContain('&lt;');
    });

    it('should strip HTML tags', () => {
      const rule = SanitizationRules.stripHTML();
      expect(rule.sanitize('<p>test</p>')).toBe('test');
    });

    it('should remove scripts', () => {
      const rule = SanitizationRules.removeScripts();
      const result = rule.sanitize('<script>alert("xss")</script><p>test</p>');
      expect(result).not.toContain('<script>');
    });

    it('should remove event handlers', () => {
      const rule = SanitizationRules.removeEventHandlers();
      const result = rule.sanitize('<div onclick="alert()">test</div>');
      expect(result).not.toContain('onclick');
    });

    it('should limit max length', () => {
      const rule = SanitizationRules.maxLength(5);
      expect(rule.sanitize('123456789')).toBe('12345');
    });

    it('should apply custom sanitization', () => {
      const rule = SanitizationRules.custom((value: any) => {
        if (typeof value === 'string') {
          return value.replace(/foo/g, 'bar');
        }
        return value;
      });

      expect(rule.sanitize('foo test foo')).toBe('bar test bar');
    });
  });

  describe('TextValidator', () => {
    let validator: TextValidator;

    beforeEach(() => {
      validator = new TextValidator();
    });

    it('should validate and sanitize text', () => {
      validator.addValidationRule(ValidationRules.notEmpty('Text'));
      validator.addSanitizationRule(SanitizationRules.trim());

      const result = validator.validateAndSanitize('  test  ');

      expect(result.valid).toBe(true);
      expect(result.sanitized).toBe('test');
    });

    it('should throw on validation failure', () => {
      validator.addValidationRule(ValidationRules.notEmpty('Text'));

      expect(() => {
        validator.validateAndSanitizeOrThrow('');
      }).toThrow();
    });

    it('should return sanitized text on success', () => {
      validator.addValidationRule(ValidationRules.notEmpty('Text'));
      validator.addSanitizationRule(SanitizationRules.trim());

      const result = validator.validateAndSanitizeOrThrow('  test  ');

      expect(result).toBe('test');
    });
  });

  describe('Predefined Validators', () => {
    it('should create safe text validator', () => {
      const validator = createSafeTextValidator();
      const result = validator.validateAndSanitize('  <script>alert("xss")</script>test  ');

      expect(result.valid).toBe(true);
      expect(result.sanitized).not.toContain('<script>');
    });

    it('should create HTML validator', () => {
      const validator = createHTMLValidator();
      const result = validator.validateAndSanitize('<p>test</p>');

      expect(result.valid).toBe(true);
      expect(result.sanitized).not.toContain('<script>');
    });

    it('should create filename validator', () => {
      const validator = createFilenameValidator();
      const result = validator.validateAndSanitize('test<>file');

      expect(result.valid).toBe(true);
      expect(result.sanitized).not.toContain('<>');
    });
  });

  describe('Object Validation and Sanitization', () => {
    it('should validate object', () => {
      const schema = {
        name: [ValidationRules.notEmpty('Name'), ValidationRules.length(1, 50, 'Name')],
        age: [ValidationRules.isNumber('Age'), ValidationRules.range(0, 120, 'Age')]
      };

      const obj = { name: 'John', age: 30 };
      const result = validateObject(obj, schema);

      expect(result.valid).toBe(true);
      expect(result.errors).toEqual({});
    });

    it('should fail object validation', () => {
      const schema = {
        name: [ValidationRules.notEmpty('Name')],
        age: [ValidationRules.isNumber('Age')]
      };

      const obj = { name: '', age: 'invalid' };
      const result = validateObject(obj, schema);

      expect(result.valid).toBe(false);
      expect(Object.keys(result.errors)).toHaveLength(2);
    });

    it('should sanitize object', () => {
      const schema = {
        name: [SanitizationRules.trim(), SanitizationRules.toLowerCase()],
        email: [SanitizationRules.trim()]
      };

      const obj = { name: '  JOHN  ', email: '  test@example.com  ' };
      const result = sanitizeObject(obj, schema);

      expect(result.name).toBe('john');
      expect(result.email).toBe('test@example.com');
    });

    it('should validate and sanitize object', () => {
      const validationSchema = {
        name: [ValidationRules.notEmpty('Name')],
        age: [ValidationRules.isNumber('Age')]
      };

      const sanitizationSchema = {
        name: [SanitizationRules.trim()],
        age: [SanitizationRules.custom((v: any) => parseInt(v) || 0)]
      };

      const obj = { name: '  John  ', age: '30' };
      const result = validateAndSanitizeObject(obj, validationSchema, sanitizationSchema);

      expect(result.valid).toBe(true);
      expect(result.sanitized.name).toBe('John');
      expect(result.sanitized.age).toBe(30);
    });
  });
});
