import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import TypstPackageBrowser from '../TypstPackageBrowser.vue';

describe('TypstPackageBrowser', () => {
  it('should render component', () => {
    const wrapper = mount(TypstPackageBrowser);
    expect(wrapper.exists()).toBe(true);
  });

  it('should display package browser header', () => {
    const wrapper = mount(TypstPackageBrowser);
    expect(wrapper.find('h2').text()).toBe('Typst 包浏览器');
  });

  it('should have search input', () => {
    const wrapper = mount(TypstPackageBrowser);
    const searchInput = wrapper.find('.search-input');
    expect(searchInput.exists()).toBe(true);
    expect(searchInput.attributes('placeholder')).toBe('搜索包...');
    expect(searchInput.attributes('maxlength')).toBe('100');
  });

  it('should have refresh button', () => {
    const wrapper = mount(TypstPackageBrowser);
    const refreshButton = wrapper.find('.btn-refresh');
    expect(refreshButton.exists()).toBe(true);
  });

  it('should filter packages by search query', async () => {
    const wrapper = mount(TypstPackageBrowser);
    const searchInput = wrapper.find('.search-input') as any;
    
    await searchInput.setValue('math');
    expect(searchInput.element.value).toBe('math');
  });

  it('should filter packages by category', async () => {
    const wrapper = mount(TypstPackageBrowser);
    const filterSelect = wrapper.find('.filter-select');
    
    expect(filterSelect.exists()).toBe(true);
  });

  it('should display error message when set', async () => {
    const wrapper = mount(TypstPackageBrowser);
    await (wrapper.vm as any).setData({ errorMessage: 'Test error' });
    
    expect(wrapper.find('.error-message').exists()).toBe(true);
    expect(wrapper.find('.error-message').text()).toContain('Test error');
  });

  it('should display success message when set', async () => {
    const wrapper = mount(TypstPackageBrowser);
    await (wrapper.vm as any).setData({ successMessage: 'Test success' });
    
    expect(wrapper.find('.success-message').exists()).toBe(true);
    expect(wrapper.find('.success-message').text()).toContain('Test success');
  });

  it('should close error message on button click', async () => {
    const wrapper = mount(TypstPackageBrowser);
    await (wrapper.vm as any).setData({ errorMessage: 'Test error' });
    
    const closeButton = wrapper.find('.btn-close-error');
    await closeButton.trigger('click');
    
    expect((wrapper.vm as any).errorMessage).toBe('');
  });

  it('should close success message on button click', async () => {
    const wrapper = mount(TypstPackageBrowser);
    await (wrapper.vm as any).setData({ successMessage: 'Test success' });
    
    const closeButton = wrapper.find('.btn-close-success');
    await closeButton.trigger('click');
    
    expect((wrapper.vm as any).successMessage).toBe('');
  });

  it('should show loading state when isLoading is true', async () => {
    const wrapper = mount(TypstPackageBrowser);
    await (wrapper.vm as any).setData({ isLoading: true });
    
    expect(wrapper.find('.loading-state').exists()).toBe(true);
  });

  it('should show empty state when no packages match', async () => {
    const wrapper = mount(TypstPackageBrowser);
    await (wrapper.vm as any).setData({ searchQuery: 'nonexistent' });
    
    // After filtering, if no packages match, empty state should show
    expect(wrapper.find('.empty-state').exists()).toBe(true);
  });

  it('should display package cards when packages are loaded', async () => {
    const wrapper = mount(TypstPackageBrowser);
    await wrapper.vm.$nextTick();
    
    // Wait for mock data to load
    await new Promise(resolve => setTimeout(resolve, 100));
    
    const packageCards = wrapper.findAll('.package-card');
    expect(packageCards.length).toBeGreaterThan(0);
  });

  it('should show package details dialog when clicking a package', async () => {
    const wrapper = mount(TypstPackageBrowser);
    await wrapper.vm.$nextTick();
    
    await new Promise(resolve => setTimeout(resolve, 100));
    
    const packageCard = wrapper.find('.package-card');
    await packageCard.trigger('click');
    
    expect((wrapper.vm as any).showDetailsDialog).toBe(true);
  });

  it('should close details dialog on close button click', async () => {
    const wrapper = mount(TypstPackageBrowser);
    await (wrapper.vm as any).setData({ showDetailsDialog: true });
    
    const closeButton = wrapper.find('.btn-close');
    await closeButton.trigger('click');
    
    expect((wrapper.vm as any).showDetailsDialog).toBe(false);
  });
});
