import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import QuickAccessToolbar from '../QuickAccessToolbar.vue';

describe('QuickAccessToolbar Component', () => {
  describe('Component Rendering', () => {
    it('should render quick access toolbar correctly', () => {
      const wrapper = mount(QuickAccessToolbar);
      expect(wrapper.exists()).toBe(true);
    });

    it('should have toolbar container', () => {
      const wrapper = mount(QuickAccessToolbar);
      expect(wrapper.find('.quick-access-toolbar').exists()).toBe(true);
    });
  });

  describe('Common Actions', () => {
    it('should render file backstage button', () => {
      const wrapper = mount(QuickAccessToolbar, {
        props: {
          showFileBackstage: false,
          documentTitle: 'Test Document'
        }
      });
      expect(wrapper.find('[data-testid="qat-file-button"]').exists()).toBe(true);
    });

    it('should mark file button active when backstage is open', () => {
      const wrapper = mount(QuickAccessToolbar, {
        props: {
          showFileBackstage: true,
          documentTitle: 'Test Document'
        }
      });
      expect(wrapper.find('[data-testid="qat-file-button"].is-active').exists()).toBe(true);
    });

    it('should render save button', () => {
      const wrapper = mount(QuickAccessToolbar, {
        props: {
          showFileBackstage: false,
          documentTitle: 'Test Document'
        }
      });
      expect(wrapper.find('button[title="保存 (Ctrl+S)"]').exists()).toBe(true);
    });

    it('should render undo button', () => {
      const wrapper = mount(QuickAccessToolbar, {
        props: {
          showFileBackstage: false,
          documentTitle: 'Test Document'
        }
      });
      expect(wrapper.find('button[title="撤销 (Ctrl+Z)"]').exists()).toBe(true);
    });

    it('should render redo button', () => {
      const wrapper = mount(QuickAccessToolbar, {
        props: {
          showFileBackstage: false,
          documentTitle: 'Test Document'
        }
      });
      expect(wrapper.find('button[title="重做 (Ctrl+Y)"]').exists()).toBe(true);
    });
  });

  describe('Event Emission', () => {
    it('should emit toggle-file-backstage when file button is clicked', async () => {
      const wrapper = mount(QuickAccessToolbar, {
        props: {
          showFileBackstage: false,
          documentTitle: 'Test Document'
        }
      });
      await wrapper.find('[data-testid="qat-file-button"]').trigger('click');
      expect(wrapper.emitted('toggle-file-backstage')).toBeTruthy();
    });

    it('should emit save event when save button is clicked', async () => {
      const wrapper = mount(QuickAccessToolbar, {
        props: {
          showFileBackstage: false,
          documentTitle: 'Test Document'
        }
      });
      const saveButton = wrapper.find('button[title="保存 (Ctrl+S)"]');
      await saveButton.trigger('click');
      expect(wrapper.emitted('save')).toBeTruthy();
    });

    it('should emit undo event when undo button is clicked', async () => {
      const wrapper = mount(QuickAccessToolbar, {
        props: {
          showFileBackstage: false,
          documentTitle: 'Test Document'
        }
      });
      const undoButton = wrapper.find('button[title="撤销 (Ctrl+Z)"]');
      await undoButton.trigger('click');
      expect(wrapper.emitted('undo')).toBeTruthy();
    });

    it('should emit redo event when redo button is clicked', async () => {
      const wrapper = mount(QuickAccessToolbar, {
        props: {
          showFileBackstage: false,
          documentTitle: 'Test Document'
        }
      });
      const redoButton = wrapper.find('button[title="重做 (Ctrl+Y)"]');
      await redoButton.trigger('click');
      expect(wrapper.emitted('redo')).toBeTruthy();
    });
  });

  describe('Accessibility', () => {
    it('should have proper ARIA labels', () => {
      const wrapper = mount(QuickAccessToolbar);
      const buttons = wrapper.findAll('button');
      buttons.forEach(button => {
        expect(button.attributes('title') || button.attributes('aria-label')).toBeTruthy();
      });
    });
  });

  describe('Icon Rendering', () => {
    it('should render icons for buttons', () => {
      const wrapper = mount(QuickAccessToolbar);
      const buttons = wrapper.findAll('button');
      expect(buttons.length).toBeGreaterThan(0);
    });
  });
});
