import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import TypstPreviewEditor from '../TypstPreviewEditor.vue';

describe('TypstPreviewEditor', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(TypstPreviewEditor, {
      props: {
        modelValue: '',
        theme: 'light',
        fontSize: 14,
        showLineNumbers: true,
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

  it('has toolbar with compile button', () => {
    expect(wrapper.find('.btn-primary').exists()).toBe(true);
  });

  it('emits update:modelValue when content changes', async () => {
    const textarea = wrapper.find('textarea');
    await textarea.setValue('test content');
    expect(wrapper.emitted('update:modelValue')).toBeTruthy();
  });

  it('applies theme class correctly', () => {
    const wrapperDark = mount(TypstPreviewEditor, {
      props: {
        modelValue: '',
        theme: 'dark'
      }
    });
    expect(wrapperDark.find('.theme-dark').exists()).toBe(true);
  });

  it('shows loading state when compiling', async () => {
    wrapper.vm.isCompiling = true;
    await wrapper.vm.$nextTick();
    expect(wrapper.find('.compiling').exists()).toBe(true);
  });

  it('displays error message when compilation fails', async () => {
    wrapper.vm.compileError = 'Test error';
    await wrapper.vm.$nextTick();
    expect(wrapper.find('.error-panel').exists()).toBe(true);
  });
});
