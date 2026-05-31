import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import { nextTick } from 'vue';
import BaseDialog from '../BaseDialog.vue';

describe('BaseDialog', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(BaseDialog, {
      props: {
        show: false,
        title: 'Test Dialog'
      }
    });
  });

  it('renders correctly when show is false', () => {
    expect(wrapper.find('.dialog-mask').exists()).toBe(false);
  });

  it('renders correctly when show is true', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    expect(wrapper.find('.dialog-mask').exists()).toBe(true);
  });

  it('displays the title', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    expect(wrapper.find('.dialog-title').text()).toBe('Test Dialog');
  });

  it('emits close event when close button is clicked', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await wrapper.find('.dialog-close').trigger('click');
    expect(wrapper.emitted('update:show')).toBeTruthy();
    expect(wrapper.emitted('update:show')![0]).toEqual([false]);
  });

  it('emits close event when mask is clicked', async () => {
    await wrapper.setProps({ show: true, maskClosable: true });
    await nextTick();
    await wrapper.find('.dialog-mask').trigger('click');
    expect(wrapper.emitted('update:show')).toBeTruthy();
  });

  it('does not close when maskClosable is false', async () => {
    await wrapper.setProps({ show: true, maskClosable: false });
    await nextTick();
    await wrapper.find('.dialog-mask').trigger('click');
    expect(wrapper.emitted('update:show')).toBeFalsy();
  });

  it('emits close event when ESC key is pressed', async () => {
    await wrapper.setProps({ show: true, closeOnEscape: true });
    await nextTick();
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }));
    await nextTick();
    expect(wrapper.emitted('update:show')).toBeTruthy();
  });

  it('does not close when closeOnEscape is false', async () => {
    await wrapper.setProps({ show: true, closeOnEscape: false });
    await nextTick();
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }));
    await nextTick();
    expect(wrapper.emitted('update:show')).toBeFalsy();
  });

  it('renders footer slot when provided', async () => {
    wrapper = mount(BaseDialog, {
      props: {
        show: true,
        title: 'Test Dialog'
      },
      slots: {
        footer: '<button>Custom Footer</button>'
      }
    });
    await nextTick();
    expect(wrapper.find('.dialog-footer').exists()).toBe(true);
    expect(wrapper.find('.dialog-footer').html()).toContain('Custom Footer');
  });

  it('has correct ARIA attributes', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const mask = wrapper.find('.dialog-mask');
    expect(mask.attributes('role')).toBe('dialog');
    expect(mask.attributes('aria-modal')).toBe('true');
    expect(mask.attributes('aria-labelledby')).toBe('dialog-title');
  });

  it('applies custom width and height', async () => {
    await wrapper.setProps({ show: true, width: '800px', height: '600px' });
    await nextTick();
    const container = wrapper.find('.dialog-container');
    expect(container.attributes('style')).toContain('width: 800px');
    expect(container.attributes('style')).toContain('height: 600px');
  });

  it('applies custom zIndex', async () => {
    await wrapper.setProps({ show: true, zIndex: 2000 });
    await nextTick();
    const mask = wrapper.find('.dialog-mask');
    expect(mask.attributes('style')).toContain('z-index: 2000');
  });

  it('hides close button when closable is false', async () => {
    await wrapper.setProps({ show: true, closable: false });
    await nextTick();
    expect(wrapper.find('.dialog-close').exists()).toBe(false);
  });

  it('renders default content slot', async () => {
    wrapper = mount(BaseDialog, {
      props: {
        show: true,
        title: 'Test Dialog'
      },
      slots: {
        default: '<div class="custom-content">Test Content</div>'
      }
    });
    await nextTick();
    expect(wrapper.find('.custom-content').exists()).toBe(true);
  });
});
