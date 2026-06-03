import { describe, it, expect, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import FontGroup from '../FontGroup.vue';

describe('FontGroup', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(FontGroup, {
      props: {
        fontFamily: 'Arial, sans-serif',
        fontSize: 14
      }
    });
  });

  it('should render font group', () => {
    expect(wrapper.find('.ribbon-group').exists()).toBe(true);
  });

  it('should emit update:font-family when font family changes', async () => {
    const select = wrapper.find('select');
    await select.setValue('Times New Roman, serif');
    expect(wrapper.emitted('update:font-family')).toBeTruthy();
  });

  it('should emit update:font-size when font size changes', async () => {
    const select = wrapper.findAll('select')[1];
    await select.setValue('16');
    expect(wrapper.emitted('update:font-size')).toBeTruthy();
  });

  it('should emit toggle-bold when bold button is clicked', async () => {
    await wrapper.find('button[title="加粗"]').trigger('click');
    expect(wrapper.emitted('toggle-bold')).toBeTruthy();
  });

  it('should emit toggle-italic when italic button is clicked', async () => {
    await wrapper.find('button[title="斜体"]').trigger('click');
    expect(wrapper.emitted('toggle-italic')).toBeTruthy();
  });

  it('should emit toggle-underline when underline button is clicked', async () => {
    await wrapper.find('button[title="下划线"]').trigger('click');
    expect(wrapper.emitted('toggle-underline')).toBeTruthy();
  });

  it('should emit toggle-strike when strike button is clicked', async () => {
    await wrapper.find('button[title="删除线"]').trigger('click');
    expect(wrapper.emitted('toggle-strike')).toBeTruthy();
  });

  it('should emit toggle-subscript when subscript button is clicked', async () => {
    await wrapper.find('button[title="下标"]').trigger('click');
    expect(wrapper.emitted('toggle-subscript')).toBeTruthy();
  });

  it('should emit toggle-superscript when superscript button is clicked', async () => {
    await wrapper.find('button[title="上标"]').trigger('click');
    expect(wrapper.emitted('toggle-superscript')).toBeTruthy();
  });

  it('should emit toggle-highlight when highlight button is clicked', async () => {
    await wrapper.find('button[title="高亮"]').trigger('click');
    expect(wrapper.emitted('toggle-highlight')).toBeTruthy();
  });

  it('should emit set-text-color when text color button is clicked', async () => {
    await wrapper.find('button[title="字体颜色"]').trigger('click');
    expect(wrapper.emitted('set-text-color')).toBeTruthy();
  });

  it('should emit clear-formatting when clear formatting button is clicked', async () => {
    await wrapper.find('button[title="清除格式"]').trigger('click');
    expect(wrapper.emitted('clear-formatting')).toBeTruthy();
  });
});
