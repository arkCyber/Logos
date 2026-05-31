import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import ConditionalFormattingDialog from '../ConditionalFormattingDialog.vue';

describe('ConditionalFormattingDialog', () => {
  it('renders correctly', () => {
    const wrapper = mount(ConditionalFormattingDialog);
    expect(wrapper.exists()).toBe(true);
  });

  it('emits close event when close button is clicked', async () => {
    const wrapper = mount(ConditionalFormattingDialog);
    await wrapper.find('.btn-close').trigger('click');
    expect(wrapper.emitted('close')).toBeTruthy();
  });

  it('emits apply event when apply button is clicked', async () => {
    const wrapper = mount(ConditionalFormattingDialog);
    await wrapper.find('.btn-apply').trigger('click');
    expect(wrapper.emitted('apply')).toBeTruthy();
  });

  it('has input fields for cell range', () => {
    const wrapper = mount(ConditionalFormattingDialog);
    const rangeInput = wrapper.find('.input-range');
    expect(rangeInput.exists()).toBe(true);
  });

  it('has select for rule type', () => {
    const wrapper = mount(ConditionalFormattingDialog);
    const ruleSelect = wrapper.find('.select-rule');
    expect(ruleSelect.exists()).toBe(true);
  });
});
