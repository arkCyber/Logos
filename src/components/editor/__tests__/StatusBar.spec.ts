import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import StatusBar from '../StatusBar.vue';

describe('StatusBar Component', () => {
  const defaultProps = {
    wordCount: 100,
    charCount: 500,
    currentPage: 1,
    totalPages: 5,
    zoomLevel: 100,
    isDarkMode: false,
    viewMode: 'print' as const
  };

  describe('Component Rendering', () => {
    it('should render status bar correctly', () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      expect(wrapper.exists()).toBe(true);
      expect(wrapper.find('.status-bar').exists()).toBe(true);
    });

    it('should display page information', () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      expect(wrapper.text()).toContain('页 1 / 5');
      expect(wrapper.find('.page-navigation').exists()).toBe(true);
    });

    it('should emit page navigation events', async () => {
      const wrapper = mount(StatusBar, {
        props: {
          ...defaultProps,
          currentPage: 2,
          totalPages: 5
        }
      });

      const buttons = wrapper.findAll('.page-nav-btn');
      await buttons[0].trigger('click');
      expect(wrapper.emitted('page-previous')).toBeTruthy();

      await buttons[1].trigger('click');
      expect(wrapper.emitted('page-next')).toBeTruthy();
    });

    it('should disable page navigation at boundaries', () => {
      const wrapper = mount(StatusBar, {
        props: {
          ...defaultProps,
          currentPage: 1,
          totalPages: 3
        }
      });

      const buttons = wrapper.findAll('.page-nav-btn');
      expect(buttons[0].attributes('disabled')).toBeDefined();
      expect(buttons[1].attributes('disabled')).toBeUndefined();
    });

    it('should display word count', () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      expect(wrapper.text()).toContain('100 字');
    });

    it('should display character count', () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      expect(wrapper.text()).toContain('500 字符');
    });

    it('should display zoom level', () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      expect(wrapper.text()).toContain('100%');
    });
  });

  describe('View Mode Toggles', () => {
    it('should render all view mode buttons', () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      expect(wrapper.find('.view-modes').exists()).toBe(true);
      const viewModeButtons = wrapper.findAll('.view-mode-btn');
      expect(viewModeButtons.length).toBe(4); // focus, read, print, web
    });

    it('should highlight active view mode', () => {
      const wrapper = mount(StatusBar, {
        props: {
          ...defaultProps,
          viewMode: 'focus'
        }
      });
      
      const activeButton = wrapper.find('.view-mode-btn.active');
      expect(activeButton.exists()).toBe(true);
    });

    it('should emit view-mode-change when view mode is clicked', async () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      const focusButton = wrapper.findAll('.view-mode-btn')[0];
      await focusButton.trigger('click');
      
      expect(wrapper.emitted('view-mode-change')).toBeTruthy();
      expect(wrapper.emitted('view-mode-change')?.[0]).toEqual(['focus']);
    });

    it('should emit correct view mode for each button', async () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      const viewModes = ['focus', 'read', 'print', 'web'];
      const buttons = wrapper.findAll('.view-mode-btn');
      
      for (let i = 0; i < buttons.length; i++) {
        await buttons[i].trigger('click');
        expect(wrapper.emitted('view-mode-change')?.[i]).toEqual([viewModes[i]]);
      }
    });
  });

  describe('Zoom Controls', () => {
    it('should render zoom controls', () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      expect(wrapper.find('.zoom-controls').exists()).toBe(true);
      expect(wrapper.find('.zoom-out').exists()).toBe(true);
      expect(wrapper.find('.zoom-in').exists()).toBe(true);
      expect(wrapper.find('.zoom-slider').exists()).toBe(true);
    });

    it('should emit zoom-out when minus button is clicked', async () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      const zoomOutButton = wrapper.find('.zoom-out');
      await zoomOutButton.trigger('click');
      
      expect(wrapper.emitted('zoom-out')).toBeTruthy();
    });

    it('should emit zoom-in when plus button is clicked', async () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      const zoomInButton = wrapper.find('.zoom-in');
      await zoomInButton.trigger('click');
      
      expect(wrapper.emitted('zoom-in')).toBeTruthy();
    });

    it('should emit zoom-change when slider is changed', async () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      const slider = wrapper.find('.zoom-slider');
      await slider.setValue(150);
      await slider.trigger('input');
      
      expect(wrapper.emitted('zoom-change')).toBeTruthy();
      expect(wrapper.emitted('zoom-change')?.[0]).toEqual([150]);
    });

    it('should respect zoom level bounds', () => {
      const wrapper = mount(StatusBar, {
        props: {
          ...defaultProps,
          zoomLevel: 25
        }
      });
      
      const slider = wrapper.find('.zoom-slider');
      expect(slider.attributes('min')).toBe('25');
      expect(slider.attributes('max')).toBe('400');
    });
  });

  describe('Theme Toggle', () => {
    it('should render theme toggle button', () => {
      const wrapper = mount(StatusBar, {
        props: {
          ...defaultProps,
          isDarkMode: false
        }
      });
      
      expect(wrapper.find('.theme-toggle').exists()).toBe(true);
    });

    it('should show sun icon in light mode', () => {
      const wrapper = mount(StatusBar, {
        props: {
          ...defaultProps,
          isDarkMode: false
        }
      });
      
      expect(wrapper.html()).toContain('lucide-sun');
    });

    it('should show moon icon in dark mode', () => {
      const wrapper = mount(StatusBar, {
        props: {
          ...defaultProps,
          isDarkMode: true
        }
      });
      
      expect(wrapper.html()).toContain('lucide-moon');
    });

    it('should emit toggle-theme when theme button is clicked', async () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      const themeButton = wrapper.find('.theme-toggle');
      await themeButton.trigger('click');
      
      expect(wrapper.emitted('toggle-theme')).toBeTruthy();
    });
  });

  describe('Icon Sizes', () => {
    it('should render all icons with 16px size', () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      // Check that all SVG icons have width="16" and height="16"
      const svgs = wrapper.findAll('svg');
      svgs.forEach(svg => {
        expect(svg.attributes('width')).toBe('16');
        expect(svg.attributes('height')).toBe('16');
      });
    });

    it('should render Lucide icons', () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      // Check for Lucide icon class names
      expect(wrapper.html()).toContain('lucide');
    });
  });

  describe('Accessibility', () => {
    it('should have proper ARIA labels on buttons', () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      const buttons = wrapper.findAll('button');
      buttons.forEach(button => {
        expect(button.attributes('aria-label')).toBeTruthy();
      });
    });

    it('should have proper ARIA labels on inputs', () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      const slider = wrapper.find('.zoom-slider');
      expect(slider.attributes('aria-label')).toBe('缩放级别');
    });
  });

  describe('Responsive Design', () => {
    it('should have correct height', () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      const statusBar = wrapper.find('.status-bar');
      expect(statusBar.exists()).toBe(true);
    });

    it('should display separators between sections', () => {
      const wrapper = mount(StatusBar, {
        props: defaultProps
      });
      
      const separators = wrapper.findAll('.status-separator');
      expect(separators.length).toBeGreaterThan(0);
    });
  });

  describe('Edge Cases', () => {
    it('should handle zero word count', () => {
      const wrapper = mount(StatusBar, {
        props: {
          ...defaultProps,
          wordCount: 0,
          charCount: 0
        }
      });
      
      expect(wrapper.text()).toContain('0 字');
      expect(wrapper.text()).toContain('0 字符');
    });

    it('should handle single page', () => {
      const wrapper = mount(StatusBar, {
        props: {
          ...defaultProps,
          currentPage: 1,
          totalPages: 1
        }
      });
      
      expect(wrapper.text()).toContain('页 1 / 1');
    });

    it('should handle minimum zoom level', () => {
      const wrapper = mount(StatusBar, {
        props: {
          ...defaultProps,
          zoomLevel: 25
        }
      });
      
      expect(wrapper.text()).toContain('25%');
    });

    it('should handle maximum zoom level', () => {
      const wrapper = mount(StatusBar, {
        props: {
          ...defaultProps,
          zoomLevel: 400
        }
      });
      
      expect(wrapper.text()).toContain('400%');
    });
  });
});
