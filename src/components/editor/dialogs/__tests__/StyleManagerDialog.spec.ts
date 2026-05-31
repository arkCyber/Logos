import { describe, it, expect, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import { nextTick } from 'vue';
import StyleManagerDialog from '../StyleManagerDialog.vue';

describe('StyleManagerDialog', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(StyleManagerDialog, {
      props: {
        show: false
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

  it('displays correct title', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    expect(wrapper.find('.dialog-title').text()).toBe('样式');
  });

  it('has style tabs', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const tabs = wrapper.findAll('.style-tab');
    expect(tabs.length).toBe(2);
    expect(tabs[0].text()).toBe('段落样式');
    expect(tabs[1].text()).toBe('字符样式');
  });

  it('can switch between paragraph and character tabs', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const tabs = wrapper.findAll('.style-tab');
    await tabs[1].trigger('click');
    await nextTick();
    expect(tabs[1].classes()).toContain('active');
  });

  it('has style list', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    expect(wrapper.find('.style-list').exists()).toBe(true);
  });

  it('has style items', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const styleItems = wrapper.findAll('.style-item');
    expect(styleItems.length).toBeGreaterThan(0);
  });

  it('can select a style', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const styleItems = wrapper.findAll('.style-item');
    await styleItems[0].trigger('click');
    await nextTick();
    expect(styleItems[0].classes()).toContain('selected');
  });

  it('has new style button', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    expect(wrapper.find('.style-list-btn').exists()).toBe(true);
  });

  it('can create new style', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const newStyleBtn = wrapper.find('.style-list-btn');
    await newStyleBtn.trigger('click');
    await nextTick();
    expect(wrapper.find('.style-editor').exists()).toBe(true);
  });

  it('has style details panel', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    expect(wrapper.find('.style-details').exists()).toBe(true);
  });

  it('can edit selected style', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const styleItems = wrapper.findAll('.style-item');
    await styleItems[0].trigger('click');
    await nextTick();
    const editBtn = wrapper.find('.style-action-btn');
    await editBtn.trigger('click');
    await nextTick();
    expect(wrapper.find('.style-editor').exists()).toBe(true);
  });

  it('can delete selected style', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const styleItems = wrapper.findAll('.style-item');
    const initialCount = styleItems.length;
    await styleItems[0].trigger('click');
    await nextTick();
    const deleteBtn = wrapper.findAll('.style-action-btn')[1];
    // Mock window.confirm
    global.confirm = () => true;
    await deleteBtn.trigger('click');
    await nextTick();
    // Verify style was deleted
    const newStyleItems = wrapper.findAll('.style-item');
    expect(newStyleItems.length).toBeLessThan(initialCount);
  });

  it('has style preview', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    expect(wrapper.find('.style-preview-large').exists()).toBe(true);
  });

  it('has style properties display', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const styleItems = wrapper.findAll('.style-item');
    await styleItems[0].trigger('click');
    await nextTick();
    expect(wrapper.find('.style-properties').exists()).toBe(true);
  });

  it('emits apply-style event when apply button is clicked', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const styleItems = wrapper.findAll('.style-item');
    await styleItems[0].trigger('click');
    await nextTick();
    const applyBtn = wrapper.find('.apply-style-btn');
    await applyBtn.trigger('click');
    await nextTick();
    expect(wrapper.emitted('apply-style')).toBeTruthy();
  });

  it('emits update:show event when cancel is clicked', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const cancelBtn = wrapper.find('.dialog-btn.secondary');
    await cancelBtn.trigger('click');
    await nextTick();
    expect(wrapper.emitted('update:show')).toBeTruthy();
  });

  it('has correct ARIA attributes', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const dialog = wrapper.find('.dialog-mask');
    expect(dialog.attributes('role')).toBe('dialog');
  });
});
