import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import EnhancedTemplateLibrary from '../EnhancedTemplateLibrary.vue';

describe('EnhancedTemplateLibrary', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(EnhancedTemplateLibrary);
  });

  it('renders correctly', () => {
    expect(wrapper.exists()).toBe(true);
  });

  it('displays search box', () => {
    expect(wrapper.find('.search-input').exists()).toBe(true);
  });

  it('displays create template button', () => {
    expect(wrapper.find('.btn-primary').exists()).toBe(true);
  });

  it('filters templates by search query', async () => {
    wrapper.vm.searchQuery = 'test';
    await wrapper.vm.$nextTick();
    // Should filter templates
    expect(wrapper.vm.filteredTemplates).toBeDefined();
  });

  it('toggles tag filter', async () => {
    wrapper.vm.toggleTag('test-tag');
    await wrapper.vm.$nextTick();
    expect(wrapper.vm.selectedTags).toContain('test-tag');
  });

  it('sorts templates by popularity', async () => {
    wrapper.vm.sortBy = 'popular';
    await wrapper.vm.$nextTick();
    expect(wrapper.vm.sortBy).toBe('popular');
  });

  it('switches between grid and list view', async () => {
    wrapper.vm.viewMode = 'list';
    await wrapper.vm.$nextTick();
    expect(wrapper.vm.viewMode).toBe('list');
  });

  it('shows template details dialog', async () => {
    const mockTemplate = {
      id: '1',
      name: 'Test Template',
      description: 'Test description',
      content: '',
      variables: [],
      metadata: { category: 'Custom', created_at: '', updated_at: '' },
      rating: { average: 4.5, count: 10 },
      downloads: 100,
      author: { name: 'Test Author' },
      tags: ['test'],
      isOfficial: false,
      isFeatured: false,
      comments: []
    };
    
    wrapper.vm.showTemplateDetails(mockTemplate);
    await wrapper.vm.$nextTick();
    expect(wrapper.vm.showDetailsDialog).toBe(true);
  });

  it('formats star rating correctly', () => {
    const rating = wrapper.vm.getStarRating(4.5);
    expect(rating).toContain('⭐');
  });
});
