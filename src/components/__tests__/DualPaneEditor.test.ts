import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import DualPaneEditor from '../DualPaneEditor.vue';

describe('DualPaneEditor', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(DualPaneEditor, {
      props: {
        modelValue: '<p>Test content</p>',
        theme: 'light',
        autoCompile: true,
        compileDelay: 500
      }
    });
  });

  it('renders correctly', () => {
    expect(wrapper.exists()).toBe(true);
  });

  it('displays editor and preview panes', () => {
    expect(wrapper.find('.editor-pane').exists()).toBe(true);
    expect(wrapper.find('.preview-pane').exists()).toBe(true);
  });

  it('has toolbar with formatting buttons', () => {
    expect(wrapper.find('.toolbar-btn').exists()).toBe(true);
  });

  it('has PDF control buttons', () => {
    expect(wrapper.find('.btn-icon').exists()).toBe(true);
  });

  it('emits update:modelValue when content changes', async () => {
    // Simulate editor content change
    wrapper.vm.editorContent = '<p>New content</p>';
    await wrapper.vm.$nextTick();
    expect(wrapper.emitted('update:modelValue')).toBeTruthy();
  });

  it('zooms in PDF', async () => {
    wrapper.vm.zoomIn();
    await wrapper.vm.$nextTick();
    expect(wrapper.vm.pdfScale).toBeGreaterThan(1.0);
  });

  it('zooms out PDF', async () => {
    wrapper.vm.zoomOut();
    await wrapper.vm.$nextTick();
    expect(wrapper.vm.pdfScale).toBeLessThan(1.0);
  });

  it('rotates PDF clockwise', async () => {
    wrapper.vm.rotateClockwise();
    await wrapper.vm.$nextTick();
    expect(wrapper.vm.pdfRotation).toBe(90);
  });

  it('rotates PDF counter-clockwise', async () => {
    wrapper.vm.rotateCounterClockwise();
    await wrapper.vm.$nextTick();
    expect(wrapper.vm.pdfRotation).toBe(270);
  });

  it('fits PDF to width', async () => {
    wrapper.vm.fitToWidth();
    await wrapper.vm.$nextTick();
    expect(wrapper.vm.pdfScale).toBe(1.0);
  });

  it('shows loading state when compiling', async () => {
    wrapper.vm.isCompiling = true;
    await wrapper.vm.$nextTick();
    expect(wrapper.find('.loading').exists()).toBe(true);
  });

  it('displays error when compilation fails', async () => {
    wrapper.vm.compileError = 'Test error';
    await wrapper.vm.$nextTick();
    expect(wrapper.find('.error-panel').exists()).toBe(true);
  });
});
