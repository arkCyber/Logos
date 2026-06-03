import { describe, it, expect, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import BubbleMenu from '../BubbleMenu.vue';

// Mock editor
const mockEditor = {
  chain: () => ({
    focus: () => ({
      toggleBold: () => ({ run: () => {} }),
      toggleItalic: () => ({ run: () => {} }),
      toggleUnderline: () => ({ run: () => {} }),
      toggleStrike: () => ({ run: () => {} }),
      toggleCode: () => ({ run: () => {} }),
      setLink: () => ({ run: () => {} }),
      unsetLink: () => ({ run: () => {} }),
      toggleBulletList: () => ({ run: () => {} }),
      toggleOrderedList: () => ({ run: () => {} }),
      toggleBlockquote: () => ({ run: () => {} }),
      toggleHeading: () => ({ run: () => {} })
    })
  }),
  isActive: () => false
};

describe('BubbleMenu', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(BubbleMenu, {
      props: {
        editor: mockEditor,
        show: true,
        x: 100,
        y: 100
      }
    });
  });

  it('should render bubble menu when show is true', () => {
    expect(wrapper.find('.bubble-menu').exists()).toBe(true);
  });

  it('should not render bubble menu when show is false', async () => {
    await wrapper.setProps({ show: false });
    expect(wrapper.find('.bubble-menu').exists()).toBe(false);
  });

  it('should emit close event', async () => {
    await wrapper.vm.$emit('close');
    expect(wrapper.emitted('close')).toBeTruthy();
  });

  it('should have correct position based on props', () => {
    const menu = wrapper.find('.bubble-menu');
    expect(menu.attributes('style')).toContain('left: 100px');
    expect(menu.attributes('style')).toContain('top: 100px');
  });

  it('should render all format buttons', () => {
    const buttons = wrapper.findAll('.bubble-button');
    expect(buttons.length).toBeGreaterThan(0);
  });
});
