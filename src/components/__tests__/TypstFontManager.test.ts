import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import TypstFontManager from '../TypstFontManager.vue';

describe('TypstFontManager', () => {
  it('should render component', () => {
    const wrapper = mount(TypstFontManager);
    expect(wrapper.exists()).toBe(true);
  });

  it('should display font manager header', () => {
    const wrapper = mount(TypstFontManager);
    expect(wrapper.find('h2').text()).toBe('Typst 字体管理');
  });

  it('should have search input', () => {
    const wrapper = mount(TypstFontManager);
    const searchInput = wrapper.find('.search-input');
    expect(searchInput.exists()).toBe(true);
    expect(searchInput.attributes('placeholder')).toBe('搜索字体...');
    expect(searchInput.attributes('maxlength')).toBe('100');
  });

  it('should have upload button', () => {
    const wrapper = mount(TypstFontManager);
    const uploadButton = wrapper.find('.btn-primary');
    expect(uploadButton.exists()).toBe(true);
    expect(uploadButton.text()).toBe('上传字体');
  });

  it('should have refresh button', () => {
    const wrapper = mount(TypstFontManager);
    const refreshButton = wrapper.find('.btn-refresh');
    expect(refreshButton.exists()).toBe(true);
  });

  it('should filter fonts by search query', async () => {
    const wrapper = mount(TypstFontManager);
    const searchInput = wrapper.find('.search-input') as any;
    
    await searchInput.setValue('Arial');
    expect(searchInput.element.value).toBe('Arial');
  });

  it('should filter fonts by category', async () => {
    const wrapper = mount(TypstFontManager);
    const filterSelect = wrapper.find('.filter-select');
    
    expect(filterSelect.exists()).toBe(true);
  });

  it('should display error message when set', async () => {
    const wrapper = mount(TypstFontManager);
    await (wrapper.vm as any).setData({ errorMessage: 'Test error' });
    
    expect(wrapper.find('.error-message').exists()).toBe(true);
    expect(wrapper.find('.error-message').text()).toContain('Test error');
  });

  it('should display success message when set', async () => {
    const wrapper = mount(TypstFontManager);
    await (wrapper.vm as any).setData({ successMessage: 'Test success' });
    
    expect(wrapper.find('.success-message').exists()).toBe(true);
    expect(wrapper.find('.success-message').text()).toContain('Test success');
  });

  it('should close error message on button click', async () => {
    const wrapper = mount(TypstFontManager);
    await (wrapper.vm as any).setData({ errorMessage: 'Test error' });
    
    const closeButton = wrapper.find('.btn-close-error');
    await closeButton.trigger('click');
    
    expect((wrapper.vm as any).errorMessage).toBe('');
  });

  it('should close success message on button click', async () => {
    const wrapper = mount(TypstFontManager);
    await (wrapper.vm as any).setData({ successMessage: 'Test success' });
    
    const closeButton = wrapper.find('.btn-close-success');
    await closeButton.trigger('click');
    
    expect((wrapper.vm as any).successMessage).toBe('');
  });

  it('should show loading state when isLoading is true', async () => {
    const wrapper = mount(TypstFontManager);
    await (wrapper.vm as any).setData({ isLoading: true });
    
    expect(wrapper.find('.loading-state').exists()).toBe(true);
  });

  it('should show empty state when no fonts match', async () => {
    const wrapper = mount(TypstFontManager);
    await (wrapper.vm as any).setData({ searchQuery: 'nonexistent' });
    
    expect(wrapper.find('.empty-state').exists()).toBe(true);
  });

  it('should display font cards when fonts are loaded', async () => {
    const wrapper = mount(TypstFontManager);
    await wrapper.vm.$nextTick();
    
    await new Promise(resolve => setTimeout(resolve, 100));
    
    const fontCards = wrapper.findAll('.font-card');
    expect(fontCards.length).toBeGreaterThan(0);
  });

  it('should show font preview dialog when clicking preview button', async () => {
    const wrapper = mount(TypstFontManager);
    await wrapper.vm.$nextTick();
    
    await new Promise(resolve => setTimeout(resolve, 100));
    
    const previewButton = wrapper.find('.btn-preview');
    await previewButton.trigger('click');
    
    expect((wrapper.vm as any).showPreviewDialog).toBe(true);
  });

  it('should close preview dialog on close button click', async () => {
    const wrapper = mount(TypstFontManager);
    await (wrapper.vm as any).setData({ showPreviewDialog: true });
    
    const closeButton = wrapper.find('.btn-close');
    await closeButton.trigger('click');
    
    expect((wrapper.vm as any).showPreviewDialog).toBe(false);
  });

  it('should show upload dialog when clicking upload button', async () => {
    const wrapper = mount(TypstFontManager);
    const uploadButton = wrapper.find('.btn-primary');
    await uploadButton.trigger('click');
    
    expect((wrapper.vm as any).showUploadDialog).toBe(true);
  });

  it('should close upload dialog on close button click', async () => {
    const wrapper = mount(TypstFontManager);
    await (wrapper.vm as any).setData({ showUploadDialog: true });
    
    const closeButton = wrapper.find('.btn-close');
    await closeButton.trigger('click');
    
    expect((wrapper.vm as any).showUploadDialog).toBe(false);
  });

  it('should disable upload button when uploading', async () => {
    const wrapper = mount(TypstFontManager);
    await (wrapper.vm as any).setData({ isUploading: true });
    
    const uploadButton = wrapper.find('.btn-primary');
    expect(uploadButton.attributes('disabled')).toBeDefined();
    expect(uploadButton.text()).toBe('上传中...');
  });

  it('should format file size correctly', () => {
    const wrapper = mount(TypstFontManager);
    const vm = wrapper.vm as any;
    
    expect(vm.formatSize(500)).toBe('500 B');
    expect(vm.formatSize(2048)).toBe('2.0 KB');
    expect(vm.formatSize(2097152)).toBe('2.0 MB');
  });

  it('should get weight label correctly', () => {
    const wrapper = mount(TypstFontManager);
    const vm = wrapper.vm as any;
    
    expect(vm.getWeightLabel(200)).toBe('Light');
    expect(vm.getWeightLabel(400)).toBe('Regular');
    expect(vm.getWeightLabel(450)).toBe('Medium');
    expect(vm.getWeightLabel(550)).toBe('SemiBold');
    expect(vm.getWeightLabel(650)).toBe('Bold');
    expect(vm.getWeightLabel(750)).toBe('ExtraBold');
  });
});
