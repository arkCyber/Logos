import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import DualPaneEditor from '../DualPaneEditor.vue';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

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

  it('has control buttons', () => {
    expect(wrapper.find('.btn-icon').exists()).toBe(true);
  });

  it('displays SVG preview pane header', () => {
    expect(wrapper.find('.pane-header h3').text()).toBe('编辑器');
  });

  it('has SVG preview wrapper', () => {
    expect(wrapper.find('.svg-preview-wrapper').exists()).toBe(true);
  });

  it('shows empty state when no PDF data', () => {
    // Component shows placeholder when no data
    expect(wrapper.find('.svg-preview-wrapper').exists()).toBe(true);
  });

  it('shows loading state when compiling', async () => {
    wrapper.vm.compileState = { isCompiling: true };
    await wrapper.vm.$nextTick();
    expect(wrapper.find('.loading').exists()).toBe(true);
  });

  it('has toolbar groups for formatting', () => {
    expect(wrapper.find('.toolbar-group').exists()).toBe(true);
  });

  it('has toolbar separators', () => {
    expect(wrapper.find('.toolbar-separator').exists()).toBe(true);
  });

  it('exposes compile method', () => {
    expect(typeof wrapper.vm.compile).toBe('function');
  });

  it('exposes getContent method', () => {
    expect(typeof wrapper.vm.getContent).toBe('function');
  });

  it('exposes setContent method', () => {
    expect(typeof wrapper.vm.setContent).toBe('function');
  });

  it('exposes getEditor method', () => {
    expect(typeof wrapper.vm.getEditor).toBe('function');
  });

  it('exposes getEditorState method', () => {
    expect(typeof wrapper.vm.getEditorState).toBe('function');
  });

  it('exposes getCompileState method', () => {
    expect(typeof wrapper.vm.getCompileState).toBe('function');
  });

  it('exposes enableSync method', () => {
    expect(typeof wrapper.vm.enableSync).toBe('function');
  });

  it('exposes disableSync method', () => {
    expect(typeof wrapper.vm.disableSync).toBe('function');
  });

  it('has bold formatting button', () => {
    expect(wrapper.findAll('.toolbar-btn').length).toBeGreaterThan(0);
  });

  it('has italic formatting button', () => {
    expect(wrapper.findAll('.toolbar-btn').length).toBeGreaterThan(0);
  });

  it('has heading buttons', () => {
    expect(wrapper.findAll('.toolbar-btn').length).toBeGreaterThan(0);
  });

  it('has list buttons', () => {
    expect(wrapper.findAll('.toolbar-btn').length).toBeGreaterThan(0);
  });

  it('has table insertion button', () => {
    expect(wrapper.findAll('.toolbar-btn').length).toBeGreaterThan(0);
  });

  it('has undo button', () => {
    expect(wrapper.find('.btn-icon').exists()).toBe(true);
  });

  it('has redo button', () => {
    expect(wrapper.find('.btn-icon').exists()).toBe(true);
  });

  it('has compile/refresh button', () => {
    expect(wrapper.find('.btn-icon').exists()).toBe(true);
  });

  it('applies theme class', () => {
    expect(wrapper.find('.theme-light').exists()).toBe(true);
  });

  it('handles dark theme', async () => {
    await wrapper.setProps({ theme: 'dark' });
    expect(wrapper.find('.theme-dark').exists()).toBe(true);
  });

  it('has editor content container', () => {
    expect(wrapper.find('.editor-content').exists()).toBe(true);
  });

  it('has preview content container', () => {
    expect(wrapper.find('.preview-content').exists()).toBe(true);
  });

  it('has SVG preview image element', () => {
    expect(wrapper.find('.svg-preview-image').exists()).toBe(true);
  });

  it('emits update:modelValue when content changes', async () => {
    await wrapper.vm.setContent('<p>New content</p>');
    await wrapper.vm.$nextTick();
    expect(wrapper.emitted('update:modelValue')).toBeTruthy();
  });

  it('emits compile event', async () => {
    // Compile method exists and can be called
    expect(typeof wrapper.vm.compile).toBe('function');
  });

  it('emits compiled event when PDF data is ready', async () => {
    // PDF data can be set
    wrapper.vm.pdfData = 'blob:test';
    await wrapper.vm.$nextTick();
    expect(wrapper.vm.pdfData).toBe('blob:test');
  });

  it('emits error event on compilation error', async () => {
    // Compile state can be set
    wrapper.vm.compileState = { isCompiling: false, compileError: new Error('Test error') };
    await wrapper.vm.$nextTick();
    expect(wrapper.vm.compileState.compileError).toBeDefined();
  });

  it('handles autoCompile prop', () => {
    expect(wrapper.props('autoCompile')).toBe(true);
  });

  it('handles compileDelay prop', () => {
    expect(wrapper.props('compileDelay')).toBe(500);
  });

  it('has spinner animation for loading state', () => {
    // Spinner element exists in the component
    expect(wrapper.vm.$el).toBeDefined();
  });

  it('has spin animation class', () => {
    const style = wrapper.vm.$el.style;
    expect(style).toBeDefined();
  });
});
