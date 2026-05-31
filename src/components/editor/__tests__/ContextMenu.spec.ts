import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { mount } from '@vue/test-utils';
import { nextTick } from 'vue';
import ContextMenu from '../ContextMenu.vue';

describe('ContextMenu', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(ContextMenu, {
      props: {
        show: false,
        x: 100,
        y: 100,
        context: 'general'
      }
    });
  });

  afterEach(() => {
    wrapper.unmount();
  });

  it('renders correctly when show is false', () => {
    expect(wrapper.find('.context-menu').exists()).toBe(false);
  });

  it('renders correctly when show is true', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    expect(wrapper.find('.context-menu').exists()).toBe(true);
  });

  it('displays correct menu items for general context', async () => {
    await wrapper.setProps({ show: true, context: 'general' });
    await nextTick();
    const menuItems = wrapper.findAll('.context-menu-item');
    expect(menuItems.length).toBeGreaterThan(0);
  });

  it('displays correct menu items for text context', async () => {
    await wrapper.setProps({ show: true, context: 'text' });
    await nextTick();
    const menuItems = wrapper.findAll('.context-menu-item');
    expect(menuItems.length).toBeGreaterThan(0);
  });

  it('displays correct menu items for table context', async () => {
    await wrapper.setProps({ show: true, context: 'table' });
    await nextTick();
    const menuItems = wrapper.findAll('.context-menu-item');
    expect(menuItems.length).toBeGreaterThan(0);
  });

  it('displays correct menu items for image context', async () => {
    await wrapper.setProps({ show: true, context: 'image' });
    await nextTick();
    const menuItems = wrapper.findAll('.context-menu-item');
    expect(menuItems.length).toBeGreaterThan(0);
  });

  it('has correct position', async () => {
    await wrapper.setProps({ show: true, x: 200, y: 300 });
    await nextTick();
    const menu = wrapper.find('.context-menu');
    expect(menu.attributes('style')).toContain('left: 200px');
    expect(menu.attributes('style')).toContain('top: 300px');
  });

  it('emits action event when menu item is clicked', async () => {
    await wrapper.setProps({ show: true, context: 'general' });
    await nextTick();
    const menuItems = wrapper.findAll('.context-menu-item');
    await menuItems[0].trigger('click');
    await nextTick();
    expect(wrapper.emitted('action')).toBeTruthy();
  });

  it('emits update:show event when menu item is clicked', async () => {
    await wrapper.setProps({ show: true, context: 'general' });
    await nextTick();
    const menuItems = wrapper.findAll('.context-menu-item');
    await menuItems[0].trigger('click');
    await nextTick();
    expect(wrapper.emitted('update:show')).toBeTruthy();
    expect(wrapper.emitted('update:show')![0]).toEqual([false]);
  });

  it('has separator items', async () => {
    await wrapper.setProps({ show: true, context: 'text' });
    await nextTick();
    const separators = wrapper.findAll('.context-menu-item.separator');
    expect(separators.length).toBeGreaterThan(0);
  });

  it('has menu icons', async () => {
    await wrapper.setProps({ show: true, context: 'general' });
    await nextTick();
    const icons = wrapper.findAll('.menu-icon');
    expect(icons.length).toBeGreaterThan(0);
  });

  it('has menu labels', async () => {
    await wrapper.setProps({ show: true, context: 'general' });
    await nextTick();
    const labels = wrapper.findAll('.menu-label');
    expect(labels.length).toBeGreaterThan(0);
  });

  it('has keyboard shortcuts display', async () => {
    await wrapper.setProps({ show: true, context: 'general' });
    await nextTick();
    const shortcuts = wrapper.findAll('.menu-shortcut');
    expect(shortcuts.length).toBeGreaterThan(0);
  });

  it('has correct ARIA attributes', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    const menu = wrapper.find('.context-menu');
    expect(menu.attributes('role')).toBe('menu');
    expect(menu.attributes('aria-label')).toBe('上下文菜单');
  });

  it('menu items have correct role', async () => {
    await wrapper.setProps({ show: true, context: 'general' });
    await nextTick();
    const menuItems = wrapper.findAll('.context-menu-item');
    menuItems.forEach((item: any) => {
      if (!item.classes().includes('separator')) {
        expect(item.attributes('role')).toBe('menuitem');
      }
    });
  });

  it('closes on escape key', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }));
    await nextTick();
    expect(wrapper.emitted('update:show')).toBeTruthy();
  });

  it('closes on click outside', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    document.body.click();
    await nextTick();
    expect(wrapper.emitted('update:show')).toBeTruthy();
  });
});
