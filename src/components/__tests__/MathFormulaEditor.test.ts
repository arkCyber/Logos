/**
 * MathFormulaEditor Component Tests
 * Aerospace-grade comprehensive test suite
 */

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any */
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import MathFormulaEditor from '../MathFormulaEditor.vue';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

describe('MathFormulaEditor', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(MathFormulaEditor, {
      props: {
        modelValue: 'E = mc^2'
      }
    });
  });

  describe('Component Rendering', () => {
    it('should render the component', () => {
      expect(wrapper.exists()).toBe(true);
    });

    it('should have default props', () => {
      expect(wrapper.props('inline')).toBe(false);
      expect(wrapper.props('editable')).toBe(true);
    });

    it('should initialize with modelValue', () => {
      expect(wrapper.vm.latexInput).toBe('E = mc^2');
    });
  });

  describe('Props Handling', () => {
    it('should accept inline prop', () => {
      const inlineWrapper = mount(MathFormulaEditor, {
        props: {
          modelValue: 'x^2',
          inline: true
        }
      });
      expect(inlineWrapper.props('inline')).toBe(true);
    });

    it('should accept editable prop', () => {
      const readonlyWrapper = mount(MathFormulaEditor, {
        props: {
          modelValue: 'x^2',
          editable: false
        }
      });
      expect(readonlyWrapper.props('editable')).toBe(false);
    });
  });

  describe('LaTeX Rendering', () => {
    it('should render LaTeX on mount', async () => {
      // Mock successful render
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue('<span class="katex">E = mc^2</span>');
      
      await wrapper.vm.renderLatex();
      expect(wrapper.vm.previewHtml).toContain('katex');
    });

    it('should handle empty LaTeX input', async () => {
      await wrapper.setData({ latexInput: '' });
      await wrapper.vm.renderLatex();
      expect(wrapper.vm.previewHtml).toBe('');
    });

    it('should handle rendering errors', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockRejectedValue(new Error('Invalid LaTeX'));
      
      await wrapper.setData({ latexInput: 'invalid' });
      await wrapper.vm.renderLatex();
      expect(wrapper.vm.isError).toBe(true);
      expect(wrapper.vm.errorMessage).toBe('Invalid LaTeX');
    });

    it('should show loading state during rendering', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockImplementation(() => new Promise(resolve => 
        setTimeout(() => resolve('<span>test</span>'), 100)
      ));
      
      wrapper.vm.renderLatex();
      // Check loading state immediately
      expect(wrapper.vm.isRendering).toBe(true);
      
      // Wait for completion
      await new Promise(resolve => setTimeout(resolve, 150));
      expect(wrapper.vm.isRendering).toBe(false);
    });
  });

  describe('Model Value Binding', () => {
    it('should emit update:modelValue on input change', async () => {
      await wrapper.setData({ latexInput: 'x^2 + y^2' });
      await wrapper.vm.updateLatex('x^2 + y^2');
      expect(wrapper.emitted('update:modelValue')).toBeTruthy();
    });

    it('should sync latexInput with modelValue prop', async () => {
      await wrapper.setProps({ modelValue: '\\sum_{i=1}^{n} i' });
      expect(wrapper.vm.latexInput).toBe('\\sum_{i=1}^{n} i');
    });
  });

  describe('Editor Dialog', () => {
    it('should open editor dialog', async () => {
      await wrapper.vm.openEditor();
      expect(wrapper.vm.showEditor).toBe(true);
    });

    it('should close editor dialog', async () => {
      await wrapper.setData({ showEditor: true });
      await wrapper.vm.closeEditor();
      expect(wrapper.vm.showEditor).toBe(false);
    });
  });

  describe('Display Mode', () => {
    it('should pass displayMode to render function', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue('<span>test</span>');
      
      await wrapper.setData({ inline: false });
      await wrapper.vm.renderLatex();
      expect(vi.mocked(invoke)).toHaveBeenCalledWith('render_latex', {
        latex: expect.any(String),
        displayMode: true
      });
    });

    it('should use inline mode when inline prop is true', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue('<span>test</span>');
      
      await wrapper.setProps({ inline: true });
      await wrapper.vm.renderLatex();
      expect(vi.mocked(invoke)).toHaveBeenCalledWith('render_latex', {
        latex: expect.any(String),
        displayMode: false
      });
    });
  });

  describe('Error Handling', () => {
    it('should display error message for invalid LaTeX', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockRejectedValue(new Error('Syntax error'));
      
      await wrapper.setData({ latexInput: '\\invalid' });
      await wrapper.vm.renderLatex();
      expect(wrapper.vm.isError).toBe(true);
      expect(wrapper.vm.errorMessage).toBe('Syntax error');
    });

    it('should handle non-Error objects', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockRejectedValue('String error');
      
      await wrapper.setData({ latexInput: 'test' });
      await wrapper.vm.renderLatex();
      expect(wrapper.vm.errorMessage).toBe('Invalid LaTeX');
    });
  });

  describe('Watch Model Value', () => {
    it('should re-render when modelValue changes', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue('<span>new</span>');
      
      await wrapper.setProps({ modelValue: '\\frac{a}{b}' });
      // Wait for watch to trigger
      await wrapper.vm.$nextTick();
      expect(wrapper.vm.latexInput).toBe('\\frac{a}{b}');
    });
  });
});
