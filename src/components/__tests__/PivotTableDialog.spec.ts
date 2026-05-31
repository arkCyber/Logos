import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import PivotTableDialog from '../PivotTableDialog.vue';

describe('PivotTableDialog', () => {
  it('renders correctly', () => {
    const wrapper = mount(PivotTableDialog);
    expect(wrapper.exists()).toBe(true);
  });

  it('emits close event when close button is clicked', async () => {
    const wrapper = mount(PivotTableDialog);
    await wrapper.find('.btn-close').trigger('click');
    expect(wrapper.emitted('close')).toBeTruthy();
  });

  it('emits apply event when apply button is clicked', async () => {
    const wrapper = mount(PivotTableDialog);
    await wrapper.find('.btn-apply').trigger('click');
    expect(wrapper.emitted('apply')).toBeTruthy();
  });

  it('has input field for pivot table name', () => {
    const wrapper = mount(PivotTableDialog);
    const nameInput = wrapper.find('.input-value');
    expect(nameInput.exists()).toBe(true);
  });

  it('has input field for source range', () => {
    const wrapper = mount(PivotTableDialog);
    const rangeInput = wrapper.findAll('.input-value')[1];
    expect(rangeInput.exists()).toBe(true);
  });
});
