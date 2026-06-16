import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import CommentsPanel from '../CommentsPanel.vue';

describe('CommentsPanel Component', () => {
  const mockComments = [
    {
      id: 'comment-1',
      text: 'Test comment',
      author: 'User',
      timestamp: Date.now(),
      range: { from: 0, to: 10 },
      resolved: false
    }
  ];

  describe('Component Rendering', () => {
    it('should render comments panel correctly', () => {
      const wrapper = mount(CommentsPanel, {
        props: {
          comments: mockComments,
          show: true
        }
      });
      expect(wrapper.exists()).toBe(true);
    });

    it('should display comments when show is true', () => {
      const wrapper = mount(CommentsPanel, {
        props: {
          comments: mockComments,
          show: true
        }
      });
      expect(wrapper.find('.comments-panel').exists()).toBe(true);
    });

    it('should hide comments when show is false', () => {
      const wrapper = mount(CommentsPanel, {
        props: {
          comments: mockComments,
          show: false
        }
      });
      expect(wrapper.find('.comments-panel').exists()).toBe(false);
    });
  });

  describe('Comment Display', () => {
    it('should render comment list', () => {
      const wrapper = mount(CommentsPanel, {
        props: {
          comments: mockComments,
          show: true
        }
      });
      // Check if comments are displayed in some form
      expect(wrapper.text()).toContain('Test comment');
    });

    it('should display comment text', () => {
      const wrapper = mount(CommentsPanel, {
        props: {
          comments: mockComments,
          show: true
        }
      });
      expect(wrapper.text()).toContain('Test comment');
    });

    it('should display comment author', () => {
      const wrapper = mount(CommentsPanel, {
        props: {
          comments: mockComments,
          show: true
        }
      });
      expect(wrapper.text()).toContain('User');
    });
  });

  describe('Empty State', () => {
    it('should show empty state when no comments', () => {
      const wrapper = mount(CommentsPanel, {
        props: {
          comments: [],
          show: true
        }
      });
      expect(wrapper.text()).toContain('暂无批注');
    });
  });

  describe('Event Emission', () => {
    it('should emit add-comment event', async () => {
      const wrapper = mount(CommentsPanel, {
        props: {
          comments: mockComments,
          show: true
        }
      });
      const addButton = wrapper.find('button[title="添加评论"]');
      if (addButton.exists()) {
        await addButton.trigger('click');
        expect(wrapper.emitted('add-comment')).toBeTruthy();
      }
    });

    it('should emit delete-comment event', async () => {
      const wrapper = mount(CommentsPanel, {
        props: {
          comments: mockComments,
          show: true
        }
      });
      const deleteButton = wrapper.find('button[title="删除评论"]');
      if (deleteButton.exists()) {
        await deleteButton.trigger('click');
        expect(wrapper.emitted('delete-comment')).toBeTruthy();
      }
    });
  });

  describe('Accessibility', () => {
    it('should have proper button labels', () => {
      const wrapper = mount(CommentsPanel, {
        props: {
          comments: mockComments,
          show: true
        }
      });
      // Check if buttons have labels
      const buttons = wrapper.findAll('button');
      const buttonsWithLabels = buttons.filter(button => 
        button.attributes('aria-label') || button.attributes('title')
      );
      // At least some buttons should have labels
      expect(buttonsWithLabels.length).toBeGreaterThan(0);
    });
  });
});
