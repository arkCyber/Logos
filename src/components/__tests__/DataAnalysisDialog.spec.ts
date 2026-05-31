import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import DataAnalysisDialog from '../DataAnalysisDialog.vue';

describe('DataAnalysisDialog', () => {
  it('renders correctly', () => {
    const wrapper = mount(DataAnalysisDialog);
    expect(wrapper.exists()).toBe(true);
  });

  it('emits close event when close button is clicked', async () => {
    const wrapper = mount(DataAnalysisDialog);
    await wrapper.find('.btn-close').trigger('click');
    expect(wrapper.emitted('close')).toBeTruthy();
  });

  it('emits apply event when apply button is clicked', async () => {
    const wrapper = mount(DataAnalysisDialog);
    await wrapper.find('.btn-apply').trigger('click');
    expect(wrapper.emitted('apply')).toBeTruthy();
  });

  it('has input field for analysis name', () => {
    const wrapper = mount(DataAnalysisDialog);
    const nameInput = wrapper.find('.input-value');
    expect(nameInput.exists()).toBe(true);
  });

  it('has analysis type grid', () => {
    const wrapper = mount(DataAnalysisDialog);
    const typeGrid = wrapper.find('.analysis-type-grid');
    expect(typeGrid.exists()).toBe(true);
  });
});
