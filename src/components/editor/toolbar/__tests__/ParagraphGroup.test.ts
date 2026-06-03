import { describe, it, expect, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import ParagraphGroup from '../ParagraphGroup.vue';

describe('ParagraphGroup', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(ParagraphGroup, {
      props: {}
    });
  });

  it('should render paragraph group', () => {
    expect(wrapper.find('.ribbon-group').exists()).toBe(true);
  });

  it('should emit set-text-align event when left align button is clicked', async () => {
    await wrapper.find('button[title="左对齐"]').trigger('click');
    expect(wrapper.emitted('set-text-align')).toBeTruthy();
    expect(wrapper.emitted('set-text-align')[0]).toEqual(['left']);
  });

  it('should emit set-text-align event when center align button is clicked', async () => {
    await wrapper.find('button[title="居中"]').trigger('click');
    expect(wrapper.emitted('set-text-align')).toBeTruthy();
    expect(wrapper.emitted('set-text-align')[0]).toEqual(['center']);
  });

  it('should emit set-text-align event when right align button is clicked', async () => {
    await wrapper.find('button[title="右对齐"]').trigger('click');
    expect(wrapper.emitted('set-text-align')).toBeTruthy();
    expect(wrapper.emitted('set-text-align')[0]).toEqual(['right']);
  });

  it('should emit set-text-align event when justify align button is clicked', async () => {
    await wrapper.find('button[title="两端对齐"]').trigger('click');
    expect(wrapper.emitted('set-text-align')).toBeTruthy();
    expect(wrapper.emitted('set-text-align')[0]).toEqual(['justify']);
  });

  it('should emit toggle-bullet-list event when bullet list button is clicked', async () => {
    await wrapper.find('button[title="无序列表"]').trigger('click');
    expect(wrapper.emitted('toggle-bullet-list')).toBeTruthy();
  });

  it('should emit toggle-ordered-list event when ordered list button is clicked', async () => {
    await wrapper.find('button[title="有序列表"]').trigger('click');
    expect(wrapper.emitted('toggle-ordered-list')).toBeTruthy();
  });

  it('should emit toggle-task-list event when task list button is clicked', async () => {
    await wrapper.find('button[title="任务列表"]').trigger('click');
    expect(wrapper.emitted('toggle-task-list')).toBeTruthy();
  });

  it('should emit decrease-indent event when decrease indent button is clicked', async () => {
    await wrapper.find('button[title="减少缩进"]').trigger('click');
    expect(wrapper.emitted('decrease-indent')).toBeTruthy();
  });

  it('should emit increase-indent event when increase indent button is clicked', async () => {
    await wrapper.find('button[title="增加缩进"]').trigger('click');
    expect(wrapper.emitted('increase-indent')).toBeTruthy();
  });

  it('should emit set-heading event when heading button is clicked', async () => {
    await wrapper.find('button[title="标题1"]').trigger('click');
    expect(wrapper.emitted('set-heading')).toBeTruthy();
    expect(wrapper.emitted('set-heading')[0]).toEqual([1]);
  });

  it('should emit toggle-blockquote event when quote button is clicked', async () => {
    await wrapper.find('button[title="引用块"]').trigger('click');
    expect(wrapper.emitted('toggle-blockquote')).toBeTruthy();
  });

  it('should emit toggle-code-block event when code block button is clicked', async () => {
    await wrapper.find('button[title="代码块"]').trigger('click');
    expect(wrapper.emitted('toggle-code-block')).toBeTruthy();
  });

  it('should emit insert-horizontal-rule event when horizontal rule button is clicked', async () => {
    await wrapper.find('button[title="分隔线"]').trigger('click');
    expect(wrapper.emitted('insert-horizontal-rule')).toBeTruthy();
  });

  it('should emit clear-formatting event when clear formatting button is clicked', async () => {
    await wrapper.find('button[title="清除格式"]').trigger('click');
    expect(wrapper.emitted('clear-formatting')).toBeTruthy();
  });
});
