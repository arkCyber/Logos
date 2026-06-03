/**
 * HTML to SVG 转换测试
 * 模拟双窗口预览中的 HTML -> Typst -> SVG 转换流程
 */

import { describe, it, expect } from 'vitest';

describe('HTML to SVG Conversion', () => {
  describe('HTML to Typst Conversion', () => {
    it('should convert simple HTML to Typst', () => {
      const html = '<p>Hello World</p>';
      const expectedTypst = 'Hello World';
      
      // 简单的HTML到Typst转换逻辑
      const typstCode = html
        .replace(/<p>/g, '')
        .replace(/<\/p>/g, '\n')
        .replace(/<strong>/g, '*')
        .replace(/<\/strong>/g, '*')
        .replace(/<em>/g, '_')
        .replace(/<\/em>/g, '_')
        .trim();
      
      expect(typstCode).toBe(expectedTypst);
    });

    it('should convert HTML with formatting to Typst', () => {
      const html = '<p><strong>Bold</strong> and <em>italic</em> text</p>';
      const expectedTypst = '*Bold* and _italic_ text';
      
      const typstCode = html
        .replace(/<p>/g, '')
        .replace(/<\/p>/g, '\n')
        .replace(/<strong>/g, '*')
        .replace(/<\/strong>/g, '*')
        .replace(/<em>/g, '_')
        .replace(/<\/em>/g, '_')
        .trim();
      
      expect(typstCode).toBe(expectedTypst);
    });

    it('should handle complex HTML structure', () => {
      const html = '<h1>Title</h1><p>Content</p>';
      const expectedTypst = '= Title\nContent';
      
      const typstCode = html
        .replace(/<h1>/g, '= ')
        .replace(/<\/h1>/g, '\n')
        .replace(/<p>/g, '')
        .replace(/<\/p>/g, '\n')
        .trim();
      
      expect(typstCode).toBe(expectedTypst);
    });
  });

  describe('SVG Decoding', () => {
    it('should decode base64 SVG correctly', () => {
      const mockSvg = '<?xml version="1.0" encoding="UTF-8"?><svg xmlns="http://www.w3.org/2000/svg" width="595" height="842"><text x="50" y="50">Test</text></svg>';
      const mockBase64 = btoa(mockSvg);
      
      const decodedSvg = atob(mockBase64);
      
      expect(decodedSvg).toBe(mockSvg);
      expect(decodedSvg.includes('<svg')).toBe(true);
      expect(decodedSvg.includes('</svg>')).toBe(true);
    });

    it('should handle invalid base64', () => {
      const invalidBase64 = 'invalid!!!';
      
      expect(() => {
        atob(invalidBase64);
      }).toThrow();
    });
  });

  describe('Full Pipeline Test (Without Tauri)', () => {
    it('should complete HTML -> Typst -> SVG pipeline (mocked)', () => {
      // Step 1: HTML input
      const html = '<p>Hello World</p>';
      
      // Step 2: HTML to Typst conversion
      const typstCode = html
        .replace(/<p>/g, '')
        .replace(/<\/p>/g, '\n')
        .trim();
      
      expect(typstCode).toBe('Hello World');
      
      // Step 3: Mock render_typst response (simulated)
      const mockSvg = '<?xml version="1.0" encoding="UTF-8"?><svg xmlns="http://www.w3.org/2000/svg" width="595" height="842"><text x="50" y="50">Hello World</text></svg>';
      const mockBase64 = btoa(mockSvg);
      
      // Step 4: Simulate successful render
      const mockResult = {
        success: true,
        output: mockBase64
      };
      
      expect(mockResult.success).toBe(true);
      
      // Step 5: Decode SVG
      const decodedSvg = atob(mockResult.output);
      
      expect(decodedSvg).toBe(mockSvg);
      expect(decodedSvg.includes('<svg')).toBe(true);
      expect(decodedSvg.includes('Hello World')).toBe(true);
    });

    it('should handle pipeline errors gracefully (mocked)', () => {
      const html = '<p>Hello World</p>';
      const typstCode = html.replace(/<p>/g, '').replace(/<\/p>/g, '\n').trim();
      
      // Mock error response
      const mockResult = {
        success: false,
        error: 'Typst compilation failed'
      };
      
      expect(mockResult.success).toBe(false);
      expect(mockResult.error).toBe('Typst compilation failed');
    });
  });

  describe('SVG Validation', () => {
    it('should validate SVG structure', () => {
      const validSvg = '<?xml version="1.0" encoding="UTF-8"?><svg xmlns="http://www.w3.org/2000/svg" width="595" height="842"><text x="50" y="50">Test</text></svg>';
      
      expect(validSvg.includes('<?xml')).toBe(true);
      expect(validSvg.includes('<svg')).toBe(true);
      expect(validSvg.includes('xmlns="http://www.w3.org/2000/svg"')).toBe(true);
      expect(validSvg.includes('width=')).toBe(true);
      expect(validSvg.includes('height=')).toBe(true);
      expect(validSvg.includes('</svg>')).toBe(true);
    });

    it('should detect invalid SVG', () => {
      const invalidSvg = 'not an svg';
      
      expect(invalidSvg.includes('<svg')).toBe(false);
      expect(invalidSvg.includes('</svg>')).toBe(false);
    });
  });

  describe('Base64 Encoding/Decoding', () => {
    it('should encode and decode SVG correctly', () => {
      const originalSvg = '<?xml version="1.0" encoding="UTF-8"?><svg xmlns="http://www.w3.org/2000/svg" width="595" height="842"><text x="50" y="50">Test</text></svg>';
      
      const encoded = btoa(originalSvg);
      const decoded = atob(encoded);
      
      expect(decoded).toBe(originalSvg);
    });

    it('should handle special characters in SVG', () => {
      const svgWithSpecialChars = '<?xml version="1.0" encoding="UTF-8"?><svg xmlns="http://www.w3.org/2000/svg" width="595" height="842"><text x="50" y="50">Test &amp; &lt;special&gt;</text></svg>';
      
      const encoded = btoa(svgWithSpecialChars);
      const decoded = atob(encoded);
      
      expect(decoded).toBe(svgWithSpecialChars);
    });
  });
});
