import { describe, it, expect, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import RibbonToolbar from '../RibbonToolbar.vue';

describe('RibbonToolbar Component', () => {
  describe('Component Rendering', () => {
    it('should render ribbon tabs correctly', () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      expect(wrapper.exists()).toBe(true);
      expect(wrapper.find('.ribbon-tabs').exists()).toBe(true);
    });

    it('should render all expected tabs', () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const tabs = wrapper.findAll('.ribbon-tab');
      expect(tabs.length).toBe(8);
    });

    it('should highlight active tab', () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const activeTab = wrapper.find('.ribbon-tab.active');
      expect(activeTab.exists()).toBe(true);
      expect(activeTab.text()).toContain('开始');
    });
  });

  describe('Tab Switching', () => {
    it('should emit set-active-tab when tab is clicked', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const tabs = wrapper.findAll('.ribbon-tab');
      await tabs[2].trigger('click'); // Third tab: insert (after file, home)

      expect(wrapper.emitted('set-active-tab')).toBeTruthy();
      expect(wrapper.emitted('set-active-tab')?.[0]).toEqual(['insert']);
    });
  });

  describe('Home Tab Panel', () => {
    it('should render clipboard group when home tab is active', () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      expect(wrapper.find('.ribbon-group[aria-label="剪贴板"]').exists()).toBe(true);
    });

    it('should emit clipboard events', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const pasteButton = wrapper.find('button[aria-label="粘贴"]');
      await pasteButton.trigger('click');
      
      expect(wrapper.emitted('paste')).toBeTruthy();
    });

    it('should emit format-painter event', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const formatPainterButton = wrapper.find('button[aria-label="格式刷"]');
      await formatPainterButton.trigger('click');
      
      expect(wrapper.emitted('format-painter')).toBeTruthy();
    });
  });

  describe('Paragraph Group', () => {
    it('should render alignment buttons', () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      expect(wrapper.find('button[title="左对齐"]').exists()).toBe(true);
      expect(wrapper.find('button[title="居中"]').exists()).toBe(true);
      expect(wrapper.find('button[title="右对齐"]').exists()).toBe(true);
      expect(wrapper.find('button[title="两端对齐"]').exists()).toBe(true);
    });

    it('should emit text-align events', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const leftAlignButton = wrapper.find('button[title="左对齐"]');
      await leftAlignButton.trigger('click');
      
      expect(wrapper.emitted('set-text-align')).toBeTruthy();
      expect(wrapper.emitted('set-text-align')?.[0]).toEqual(['left']);
    });

    it('should emit list events', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const bulletListButton = wrapper.find('button[title="无序列表"]');
      await bulletListButton.trigger('click');
      
      expect(wrapper.emitted('toggle-bullet-list')).toBeTruthy();
    });

    it('should emit heading events', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const heading1Button = wrapper.find('button[title="标题1"]');
      await heading1Button.trigger('click');
      
      expect(wrapper.emitted('set-heading')).toBeTruthy();
      expect(wrapper.emitted('set-heading')?.[0]).toEqual([1]);
    });

    it('should render expand button', () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      expect(wrapper.find('button[title="更多段落选项"]').exists()).toBe(true);
    });

    it('should toggle expanded state when expand button is clicked', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const expandButton = wrapper.find('button[title="更多段落选项"]');
      await expandButton.trigger('click');
      
      // After clicking, the expanded section should be visible
      expect(wrapper.find('.paragraph-expanded').exists()).toBe(true);
    });

    it('should render expanded paragraph options when expanded', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const expandButton = wrapper.find('button[title="更多段落选项"]');
      await expandButton.trigger('click');
      
      expect(wrapper.find('button[title="标题4"]').exists()).toBe(true);
      expect(wrapper.find('button[title="标题5"]').exists()).toBe(true);
      expect(wrapper.find('button[title="标题6"]').exists()).toBe(true);
      expect(wrapper.find('button[title="引用块"]').exists()).toBe(true);
      expect(wrapper.find('button[title="代码块"]').exists()).toBe(true);
      expect(wrapper.find('button[title="分隔线"]').exists()).toBe(true);
      expect(wrapper.find('button[title="清除格式"]').exists()).toBe(true);
    });

    it('should emit events for expanded paragraph options', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const expandButton = wrapper.find('button[title="更多段落选项"]');
      await expandButton.trigger('click');
      
      const heading4Button = wrapper.find('button[title="标题4"]');
      await heading4Button.trigger('click');
      
      expect(wrapper.emitted('set-heading')).toBeTruthy();
    });
  });

  describe('Editing Group', () => {
    it('should render editing buttons', () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      expect(wrapper.find('button[title="查找"]').exists()).toBe(true);
      expect(wrapper.find('button[title="替换"]').exists()).toBe(true);
      expect(wrapper.find('button[title="全选"]').exists()).toBe(true);
    });

    it('should emit find-text event', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const findButton = wrapper.find('button[title="查找"]');
      await findButton.trigger('click');
      
      expect(wrapper.emitted('find-text')).toBeTruthy();
    });
  });

  describe('Font Group', () => {
    it('should render font controls', () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      expect(wrapper.find('.ribbon-select').exists()).toBe(true);
    });

    it('should emit font-family change event', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const fontSelect = wrapper.find('.ribbon-select');
      await fontSelect.setValue('"Arial", sans-serif');
      await fontSelect.trigger('change');
      
      expect(wrapper.emitted('update-font-family')).toBeTruthy();
    });

    it('should emit font-size change event', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const sizeSelect = wrapper.findAll('.ribbon-select')[1];
      await sizeSelect.setValue(14);
      await sizeSelect.trigger('change');
      
      expect(wrapper.emitted('update-font-size')).toBeTruthy();
    });
  });

  describe('Lucide Icons', () => {
    it('should render Lucide icons in clipboard group', () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      // Check for Lucide icon components
      expect(wrapper.html()).toContain('lucide');
    });

    it('should render Lucide icons in paragraph group', () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      expect(wrapper.html()).toContain('lucide');
    });
  });

  describe('Accessibility', () => {
    it('should have proper ARIA labels', () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      expect(wrapper.find('[role="toolbar"]').exists()).toBe(true);
      expect(wrapper.find('[role="tablist"]').exists()).toBe(true);
    });

    it('should have proper button labels', () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'home',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12
        }
      });
      
      const buttons = wrapper.findAll('button');
      // Filter out buttons without labels (some might be decorative)
      const buttonsWithLabels = buttons.filter(button => 
        button.attributes('aria-label') || button.attributes('title')
      );
      // At least some buttons should have labels
      expect(buttonsWithLabels.length).toBeGreaterThan(0);
    });
  });

  describe('File Tab Export', () => {
    it('should render SVG export buttons on file tab', () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'file',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12,
        },
      });

      expect(wrapper.text()).toContain('Export SVG (Typst)');
      expect(wrapper.text()).toContain('Export SVG (HTML)');
    });

    it('should emit export-svg-typst from file tab', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'file',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12,
        },
      });

      const button = wrapper.find('button[aria-label="Export SVG Typst"]');
      await button.trigger('click');
      expect(wrapper.emitted('export-svg-typst')).toBeTruthy();
    });

    it('should emit export-svg-html from file tab', async () => {
      const wrapper = mount(RibbonToolbar, {
        props: {
          activeTab: 'file',
          fontFamily: 'Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif',
          fontSize: 12,
        },
      });

      const button = wrapper.find('button[aria-label="Export SVG HTML"]');
      await button.trigger('click');
      expect(wrapper.emitted('export-svg-html')).toBeTruthy();
    });
  });
});
