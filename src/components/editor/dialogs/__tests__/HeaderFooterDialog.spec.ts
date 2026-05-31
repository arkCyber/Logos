import { describe, it, expect, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import { nextTick } from 'vue';
import HeaderFooterDialog from '../HeaderFooterDialog.vue';

describe('HeaderFooterDialog', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(HeaderFooterDialog, {
      props: {
        show: false,
        type: 'header'
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

  it('displays correct title for header', async () => {
    await wrapper.setProps({ show: true, type: 'header' });
    await nextTick();
    expect(wrapper.find('.dialog-title').text()).toBe('页眉');
  });

  it('displays correct title for footer', async () => {
    await wrapper.setProps({ show: true, type: 'footer' });
    await nextTick();
    expect(wrapper.find('.dialog-title').text()).toBe('页脚');
  });

  it('has options section', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    expect(wrapper.find('.options-grid').exists()).toBe(true);
  });

  it('has different first page option', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const checkboxes = wrapper.findAll('.checkbox-label');
    expect(checkboxes.length).toBe(2);
  });

  it('can toggle different first page', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const checkbox = wrapper.find('.checkbox-label input');
    await checkbox.setChecked(true);
    await nextTick();
    expect(checkbox.element.checked).toBe(true);
  });

  it('has content tabs', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const tabs = wrapper.findAll('.content-tab');
    expect(tabs.length).toBe(1); // Only odd tab by default
  });

  it('shows even tab when different odd even is enabled', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const checkbox = wrapper.findAll('.checkbox-label input')[1];
    await checkbox.setChecked(true);
    await nextTick();
    const tabs = wrapper.findAll('.content-tab');
    expect(tabs.length).toBe(2);
  });

  it('has content editor', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    expect(wrapper.find('.content-textarea').exists()).toBe(true);
  });

  it('can edit content', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const textarea = wrapper.find('.content-textarea');
    await textarea.setValue('Test content');
    await nextTick();
    expect(textarea.element.value).toBe('Test content');
  });

  it('has field buttons', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const fieldBtns = wrapper.findAll('.field-btn');
    expect(fieldBtns.length).toBe(6);
  });

  it('can insert page number field', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const pageBtn = wrapper.findAll('.field-btn')[0];
    await pageBtn.trigger('click');
    await nextTick();
    const textarea = wrapper.find('.content-textarea');
    expect(textarea.element.value).toContain('{PAGE}');
  });

  it('can insert date field', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const dateBtn = wrapper.findAll('.field-btn')[2];
    await dateBtn.trigger('click');
    await nextTick();
    const textarea = wrapper.find('.content-textarea');
    expect(textarea.element.value).toContain('{DATE}');
  });

  it('has position options', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const positionBtns = wrapper.findAll('.position-btn');
    expect(positionBtns.length).toBe(3);
  });

  it('can select position', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const positionBtns = wrapper.findAll('.position-btn');
    await positionBtns[1].trigger('click');
    await nextTick();
    expect(positionBtns[1].classes()).toContain('active');
  });

  it('has distance input', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    expect(wrapper.find('.distance-input input').exists()).toBe(true);
  });

  it('can set distance from edge', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const distanceInput = wrapper.find('.distance-input input');
    await distanceInput.setValue('2.0');
    await nextTick();
    expect(distanceInput.element.value).toBe('2.0');
  });

  it('has preview area', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    expect(wrapper.find('.preview-area').exists()).toBe(true);
  });

  it('emits apply event with correct content', async () => {
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

  it('can clear content', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const textarea = wrapper.find('.content-textarea');
    await textarea.setValue('Test content');
    await nextTick();
    const clearBtn = wrapper.findAll('.dialog-btn.secondary')[0];
    await clearBtn.trigger('click');
    await nextTick();
    expect(textarea.element.value).toBe('');
  });

  it('has correct ARIA attributes', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const dialog = wrapper.find('.dialog-mask');
    expect(dialog.attributes('role')).toBe('dialog');
  });
});
