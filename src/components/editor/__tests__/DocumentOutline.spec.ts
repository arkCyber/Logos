/**
 * DocumentOutline layout tests
 */

import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import DocumentOutline from '../DocumentOutline.vue';

describe('DocumentOutline layout', () => {
  it('uses in-flow side panel classes when visible', () => {
    const wrapper = mount(DocumentOutline, {
      props: {
        show: true,
        headings: [{ id: 'h1', level: 1, text: 'Title' }],
      },
    });

    const panel = wrapper.find('.document-outline');
    expect(panel.exists()).toBe(true);
    expect(panel.classes()).toContain('editor-side-panel');
    expect(panel.classes()).toContain('editor-side-panel--left');
  });

  it('does not render when show is false', () => {
    const wrapper = mount(DocumentOutline, {
      props: {
        show: false,
        headings: [],
      },
    });

    expect(wrapper.find('.document-outline').exists()).toBe(false);
  });
});
