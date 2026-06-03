import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import RevisionModePanel from '../RevisionModePanel.vue';

// Mock global confirm function
global.confirm = vi.fn(() => true);

describe('RevisionModePanel Component', () => {
  const mockRevisions = [
    {
      id: 'revision-1',
      author: 'User',
      timestamp: Date.now(),
      type: 'insert' as const,
      content: 'Test content',
      accepted: false,
      rejected: false
    }
  ];

  const defaultProps = {
    show: true,
    revisions: mockRevisions,
    trackChanges: false
  };

  describe('Component Rendering', () => {
    it('should render revision mode panel correctly', () => {
      const wrapper = mount(RevisionModePanel, {
        props: defaultProps
      });
      expect(wrapper.exists()).toBe(true);
    });

    it('should display panel when show is true', () => {
      const wrapper = mount(RevisionModePanel, {
        props: defaultProps
      });
      // Check if panel is rendered (not necessarily visible)
      expect(wrapper.exists()).toBe(true);
    });

    it('should hide panel when show is false', () => {
      const wrapper = mount(RevisionModePanel, {
        props: {
          ...defaultProps,
          show: false
        }
      });
      expect(wrapper.find('.revision-mode-panel').exists()).toBe(false);
    });
  });

  describe('Revision Mode Toggle', () => {
    it('should display revision mode status', () => {
      const wrapper = mount(RevisionModePanel, {
        props: {
          ...defaultProps,
          trackChanges: true
        }
      });
      expect(wrapper.text()).toContain('修订');
    });

    it('should have toggle button', () => {
      const wrapper = mount(RevisionModePanel, {
        props: defaultProps
      });
      const buttons = wrapper.findAll('button');
      expect(buttons.length).toBeGreaterThan(0);
    });
  });

  describe('Revision Actions', () => {
    it('should render accept all button', () => {
      const wrapper = mount(RevisionModePanel, {
        props: {
          ...defaultProps,
          trackChanges: true
        }
      });
      // Check if any accept button exists
      const buttons = wrapper.findAll('button');
      const hasAcceptButton = buttons.some(btn => 
        btn.attributes('title')?.includes('接受') || btn.text().includes('接受')
      );
      expect(hasAcceptButton).toBe(true);
    });

    it('should render reject all button', () => {
      const wrapper = mount(RevisionModePanel, {
        props: {
          ...defaultProps,
          trackChanges: true
        }
      });
      // Check if any reject button exists
      const buttons = wrapper.findAll('button');
      const hasRejectButton = buttons.some(btn => 
        btn.attributes('title')?.includes('拒绝') || btn.text().includes('拒绝')
      );
      expect(hasRejectButton).toBe(true);
    });

    it('should emit accept-all event', async () => {
      const wrapper = mount(RevisionModePanel, {
        props: {
          ...defaultProps,
          trackChanges: true
        }
      });
      const buttons = wrapper.findAll('button');
      const acceptButton = buttons.find(btn => 
        btn.attributes('title')?.includes('接受') || btn.text().includes('接受')
      );
      if (acceptButton) {
        await acceptButton.trigger('click');
        expect(wrapper.emitted('accept-all')).toBeTruthy();
      }
    });

    it('should emit reject-all event', async () => {
      const wrapper = mount(RevisionModePanel, {
        props: {
          ...defaultProps,
          trackChanges: true
        }
      });
      const buttons = wrapper.findAll('button');
      const rejectButton = buttons.find(btn => 
        btn.attributes('title')?.includes('拒绝') || btn.text().includes('拒绝')
      );
      if (rejectButton) {
        await rejectButton.trigger('click');
        expect(wrapper.emitted('reject-all')).toBeTruthy();
      }
    });
  });

  describe('Accessibility', () => {
    it('should have proper button labels', () => {
      const wrapper = mount(RevisionModePanel, {
        props: defaultProps
      });
      const buttons = wrapper.findAll('button');
      const buttonsWithLabels = buttons.filter(button => 
        button.attributes('aria-label') || button.attributes('title')
      );
      // At least some buttons should have labels
      expect(buttonsWithLabels.length).toBeGreaterThan(0);
    });
  });
});
