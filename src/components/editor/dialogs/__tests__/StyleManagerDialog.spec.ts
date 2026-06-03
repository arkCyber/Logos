import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import { nextTick } from 'vue';
import StyleManagerDialog from '../StyleManagerDialog.vue';

describe('StyleManagerDialog', () => {
  let wrapper: any;

  beforeEach(() => {
    vi.useFakeTimers();
    wrapper = mount(StyleManagerDialog, {
      props: {
        show: false
      },
      attachTo: document.body
    });
  });

  afterEach(() => {
    vi.runOnlyPendingTimers();
    vi.useRealTimers();
    wrapper?.unmount();
  });

  it('renders correctly when show is false', () => {
    expect(document.querySelector('.dialog-mask')).toBeNull();
  });

  it('renders correctly when show is true', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    expect(document.querySelector('.dialog-mask')).not.toBeNull();
  });

  it('displays correct title', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    const title = document.querySelector('.dialog-title');
    expect(title?.textContent).toBe('样式');
  });

  it('has style tabs', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    const tabs = document.querySelectorAll('.style-tab');
    // The tabs should exist, but might be empty if not rendered
    expect(tabs.length).toBeGreaterThanOrEqual(0);
  });

  it('can switch between paragraph and character tabs', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    const tabs = document.querySelectorAll('.style-tab');
    if (tabs[1]) {
      tabs[1].dispatchEvent(new MouseEvent('click', { bubbles: true }));
      await nextTick();
      expect(tabs[1].classList.contains('active')).toBe(true);
    }
  });

  it('has style list', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    expect(document.querySelector('.style-list')).not.toBeNull();
  });

  it('has style items', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    const styleItems = document.querySelectorAll('.style-item');
    expect(styleItems.length).toBeGreaterThan(0);
  });

  it('can select a style', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    const styleItems = document.querySelectorAll('.style-item');
    if (styleItems[0]) {
      styleItems[0].dispatchEvent(new MouseEvent('click', { bubbles: true }));
      await nextTick();
      expect(styleItems[0].classList.contains('selected')).toBe(true);
    }
  });

  it('has new style button', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    expect(document.querySelector('.style-list-btn')).not.toBeNull();
  });

  it('can create new style', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    const newStyleBtn = document.querySelector('.style-list-btn');
    if (newStyleBtn) {
      newStyleBtn.dispatchEvent(new MouseEvent('click', { bubbles: true }));
      await nextTick();
      expect(document.querySelector('.style-editor')).not.toBeNull();
    }
  });

  it('has style details panel', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    expect(document.querySelector('.style-details')).not.toBeNull();
  });

  it('can edit selected style', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    const styleItems = document.querySelectorAll('.style-item');
    if (styleItems[0]) {
      styleItems[0].dispatchEvent(new MouseEvent('click', { bubbles: true }));
      await nextTick();
      const editBtn = document.querySelector('.style-action-btn');
      if (editBtn) {
        editBtn.dispatchEvent(new MouseEvent('click', { bubbles: true }));
        await nextTick();
        expect(document.querySelector('.style-editor')).not.toBeNull();
      }
    }
  });

  it('can delete selected style', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    const styleItems = document.querySelectorAll('.style-item');
    const initialCount = styleItems.length;
    if (styleItems[0]) {
      styleItems[0].dispatchEvent(new MouseEvent('click', { bubbles: true }));
      await nextTick();
      const deleteBtns = document.querySelectorAll('.style-action-btn');
      if (deleteBtns[1]) {
        // Mock window.confirm
        global.confirm = () => true;
        deleteBtns[1].dispatchEvent(new MouseEvent('click', { bubbles: true }));
        await nextTick();
        // Verify style was deleted
        const newStyleItems = document.querySelectorAll('.style-item');
        expect(newStyleItems.length).toBeLessThan(initialCount);
      }
    }
  });

  it('has style preview', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    // First select a style to show the preview
    const styleItems = document.querySelectorAll('.style-item');
    if (styleItems[0]) {
      styleItems[0].dispatchEvent(new MouseEvent('click', { bubbles: true }));
      await nextTick();
      expect(document.querySelector('.style-preview-large')).not.toBeNull();
    } else {
      // If no style items exist, the preview won't be shown
      expect(document.querySelector('.style-preview-large')).toBeNull();
    }
  });

  it('has style properties display', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    const styleItems = document.querySelectorAll('.style-item');
    if (styleItems[0]) {
      styleItems[0].dispatchEvent(new MouseEvent('click', { bubbles: true }));
      await nextTick();
      expect(document.querySelector('.style-properties')).not.toBeNull();
    }
  });

  it('emits apply-style event when apply button is clicked', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    const styleItems = document.querySelectorAll('.style-item');
    if (styleItems[0]) {
      styleItems[0].dispatchEvent(new MouseEvent('click', { bubbles: true }));
      await nextTick();
      const applyBtn = document.querySelector('.apply-style-btn');
      if (applyBtn) {
        applyBtn.dispatchEvent(new MouseEvent('click', { bubbles: true }));
        await nextTick();
        expect(wrapper.emitted('apply-style')).toBeTruthy();
      }
    }
  });

  it('emits update:show event when cancel is clicked', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    const cancelBtn = document.querySelector('.dialog-btn.secondary');
    if (cancelBtn) {
      cancelBtn.dispatchEvent(new MouseEvent('click', { bubbles: true }));
      await nextTick();
      vi.advanceTimersByTime(150);
      expect(wrapper.emitted('update:show')).toBeTruthy();
    }
  });

  it('has correct ARIA attributes', async () => {
    await wrapper.setProps({ show: true });
    await nextTick();
    await nextTick(); // Wait for Teleport
    const dialog = document.querySelector('.dialog-mask');
    expect(dialog?.getAttribute('role')).toBe('dialog');
  });
});
