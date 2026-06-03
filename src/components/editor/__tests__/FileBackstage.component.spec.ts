/**
 * FileBackstage component mount tests
 *
 * Validates SVG export button rendering and event emission at component level.
 */

import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import FileBackstage from '../FileBackstage.vue';

vi.mock('@tauri-apps/plugin-dialog', () => ({
  save: vi.fn(),
  open: vi.fn(),
}));

vi.mock('../../utils/logger', () => ({
  logger: {
    info: vi.fn(),
    warn: vi.fn(),
    error: vi.fn(),
    debug: vi.fn(),
  },
  LogCategory: {
    SYSTEM: 'system',
    UI: 'ui',
  },
}));

describe('FileBackstage Component', () => {
  const defaultProps = {
    show: true,
    recentFiles: [] as string[],
    documentTitle: 'Test Document',
  };

  it('should render SVG export buttons on info tab', async () => {
    const wrapper = mount(FileBackstage, { props: defaultProps });

    const infoTab = wrapper.findAll('.sidebar-item').find((item) => item.text().includes('信息'));
    expect(infoTab).toBeDefined();
    await infoTab!.trigger('click');

    expect(wrapper.text()).toContain('Export SVG (Typst)');
    expect(wrapper.text()).toContain('Export SVG (HTML)');
  });

  it('should emit export-svg-typst and close when Typst SVG button clicked', async () => {
    const wrapper = mount(FileBackstage, { props: defaultProps });

    const infoTab = wrapper.findAll('.sidebar-item').find((item) => item.text().includes('信息'));
    await infoTab!.trigger('click');

    const button = wrapper.find('button[aria-label="Export SVG (Typst)"]');
    await button.trigger('click');

    expect(wrapper.emitted('export-svg-typst')).toBeTruthy();
    expect(wrapper.emitted('close')).toBeTruthy();
  });

  it('should emit export-svg-html and close when HTML SVG button clicked', async () => {
    const wrapper = mount(FileBackstage, { props: defaultProps });

    const infoTab = wrapper.findAll('.sidebar-item').find((item) => item.text().includes('信息'));
    await infoTab!.trigger('click');

    const button = wrapper.find('button[aria-label="Export SVG (HTML)"]');
    await button.trigger('click');

    expect(wrapper.emitted('export-svg-html')).toBeTruthy();
    expect(wrapper.emitted('close')).toBeTruthy();
  });

  it('should not render backstage content when show is false', () => {
    const wrapper = mount(FileBackstage, {
      props: { ...defaultProps, show: false },
    });

    expect(wrapper.find('.file-backstage').exists()).toBe(false);
  });
});
