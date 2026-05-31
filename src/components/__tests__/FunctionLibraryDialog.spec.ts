import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import FunctionLibraryDialog from '../FunctionLibraryDialog.vue';

describe('FunctionLibraryDialog', () => {
  it('renders correctly', () => {
    const wrapper = mount(FunctionLibraryDialog);
    expect(wrapper.exists()).toBe(true);
  });

  it('emits close event when close button is clicked', async () => {
    const wrapper = mount(FunctionLibraryDialog);
    await wrapper.find('.btn-close').trigger('click');
    expect(wrapper.emitted('close')).toBeTruthy();
  });

  it('has search input', () => {
    const wrapper = mount(FunctionLibraryDialog);
    const searchInput = wrapper.find('.input-search');
    expect(searchInput.exists()).toBe(true);
  });

  it('has category list', () => {
    const wrapper = mount(FunctionLibraryDialog);
    const categoryList = wrapper.find('.category-list');
    expect(categoryList.exists()).toBe(true);
  });

  it('has function list', () => {
    const wrapper = mount(FunctionLibraryDialog);
    const functionList = wrapper.find('.function-list');
    expect(functionList.exists()).toBe(true);
  });
});
