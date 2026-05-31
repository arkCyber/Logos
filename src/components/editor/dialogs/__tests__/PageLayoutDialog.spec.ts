import { describe, it, expect, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import { nextTick } from 'vue';
import PageLayoutDialog from '../PageLayoutDialog.vue';

describe('PageLayoutDialog', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(PageLayoutDialog, {
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
    expect(wrapper.find('.dialog-title').text()).toBe('页面设置');
  });

  it('has orientation options', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const orientationBtns = wrapper.findAll('.orientation-btn');
    expect(orientationBtns.length).toBe(2);
  });

  it('can select portrait orientation', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const portraitBtn = wrapper.findAll('.orientation-btn')[0];
    await portraitBtn.trigger('click');
    await nextTick();
    expect(portraitBtn.classes()).toContain('active');
  });

  it('can select landscape orientation', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const landscapeBtn = wrapper.findAll('.orientation-btn')[1];
    await landscapeBtn.trigger('click');
    await nextTick();
    expect(landscapeBtn.classes()).toContain('active');
  });

  it('has page size options', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const pageSizeBtns = wrapper.findAll('.page-size-btn');
    expect(pageSizeBtns.length).toBe(4);
  });

  it('can select page size', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const pageSizeBtns = wrapper.findAll('.page-size-btn');
    await pageSizeBtns[0].trigger('click');
    await nextTick();
    expect(pageSizeBtns[0].classes()).toContain('active');
  });

  it('has margin presets', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const marginPresets = wrapper.findAll('.margin-preset-btn');
    expect(marginPresets.length).toBe(4);
  });

  it('can select margin preset', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const marginPresets = wrapper.findAll('.margin-preset-btn');
    await marginPresets[0].trigger('click');
    await nextTick();
    expect(marginPresets[0].classes()).toContain('active');
  });

  it('can set custom margins', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const marginInputs = wrapper.findAll('.margin-input-group input');
    await marginInputs[0].setValue('3.0');
    await nextTick();
    expect(marginInputs[0].element.value).toBe('3.0');
  });

  it('emits apply event with correct settings', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const applyBtn = wrapper.find('.dialog-btn.primary');
    await applyBtn.trigger('click');
    await nextTick();
    expect(wrapper.emitted('apply')).toBeTruthy();
  });

  it('emits update:show event when cancel is clicked', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const cancelBtn = wrapper.find('.dialog-btn.secondary');
    await cancelBtn.trigger('click');
    await nextTick();
    expect(wrapper.emitted('update:show')).toBeTruthy();
    expect(wrapper.emitted('update:show')![0]).toEqual([false]);
  });

  it('has preview area', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    expect(wrapper.find('.page-preview').exists()).toBe(true);
  });

  it('has correct ARIA attributes', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const dialog = wrapper.find('.dialog-mask');
    expect(dialog.attributes('role')).toBe('dialog');
  });
});
