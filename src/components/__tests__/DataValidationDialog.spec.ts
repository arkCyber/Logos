import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import DataValidationDialog from '../DataValidationDialog.vue';

describe('DataValidationDialog', () => {
  it('renders correctly', () => {
    const wrapper = mount(DataValidationDialog);
    expect(wrapper.exists()).toBe(true);
  });

  it('emits close event when close button is clicked', async () => {
    const wrapper = mount(DataValidationDialog);
    await wrapper.find('.btn-close').trigger('click');
    expect(wrapper.emitted('close')).toBeTruthy();
  });

  it('emits apply event when apply button is clicked', async () => {
    const wrapper = mount(DataValidationDialog);
    await wrapper.find('.btn-apply').trigger('click');
    expect(wrapper.emitted('apply')).toBeTruthy();
  });

  it('has input field for cell range', () => {
    const wrapper = mount(DataValidationDialog);
    const rangeInput = wrapper.find('.input-range');
    expect(rangeInput.exists()).toBe(true);
  });

  it('has select for validation type', () => {
    const wrapper = mount(DataValidationDialog);
    const typeSelect = wrapper.find('.select-rule');
    expect(typeSelect.exists()).toBe(true);
  });
});
