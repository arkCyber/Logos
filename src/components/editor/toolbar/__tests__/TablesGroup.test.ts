import { describe, it, expect, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import TablesGroup from '../TablesGroup.vue';

describe('TablesGroup', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(TablesGroup, {
      props: {}
    });
  });

  it('should render table group', () => {
    expect(wrapper.find('.ribbon-group').exists()).toBe(true);
  });

  it('should emit insert-table event when table button is clicked', async () => {
    await wrapper.find('button[title="插入表格"]').trigger('click');
    expect(wrapper.emitted('insert-table')).toBeTruthy();
  });

  it('should emit delete-table event when delete button is clicked', async () => {
    await wrapper.find('button[title="删除表格"]').trigger('click');
    expect(wrapper.emitted('delete-table')).toBeTruthy();
  });

  it('should emit add-column-before event when left arrow button is clicked', async () => {
    await wrapper.find('button[title="在左侧插入列"]').trigger('click');
    expect(wrapper.emitted('add-column-before')).toBeTruthy();
  });

  it('should emit add-column-after event when right arrow button is clicked', async () => {
    await wrapper.find('button[title="在右侧插入列"]').trigger('click');
    expect(wrapper.emitted('add-column-after')).toBeTruthy();
  });

  it('should emit delete-column event when delete column button is clicked', async () => {
    await wrapper.find('button[title="删除列"]').trigger('click');
    expect(wrapper.emitted('delete-column')).toBeTruthy();
  });

  it('should emit add-row-before event when up arrow button is clicked', async () => {
    await wrapper.find('button[title="在上方插入行"]').trigger('click');
    expect(wrapper.emitted('add-row-before')).toBeTruthy();
  });

  it('should emit add-row-after event when down arrow button is clicked', async () => {
    await wrapper.find('button[title="在下方插入行"]').trigger('click');
    expect(wrapper.emitted('add-row-after')).toBeTruthy();
  });

  it('should emit delete-row event when delete row button is clicked', async () => {
    await wrapper.find('button[title="删除行"]').trigger('click');
    expect(wrapper.emitted('delete-row')).toBeTruthy();
  });

  it('should emit merge-cells event when merge button is clicked', async () => {
    await wrapper.find('button[title="合并单元格"]').trigger('click');
    expect(wrapper.emitted('merge-cells')).toBeTruthy();
  });

  it('should emit split-cell event when split button is clicked', async () => {
    await wrapper.find('button[title="拆分单元格"]').trigger('click');
    expect(wrapper.emitted('split-cell')).toBeTruthy();
  });

  it('should emit toggle-header-row event when header row button is clicked', async () => {
    await wrapper.find('button[title="切换表头行"]').trigger('click');
    expect(wrapper.emitted('toggle-header-row')).toBeTruthy();
  });

  it('should emit toggle-header-column event when header column button is clicked', async () => {
    await wrapper.find('button[title="切换表头列"]').trigger('click');
    expect(wrapper.emitted('toggle-header-column')).toBeTruthy();
  });

  it('should emit toggle-header-cell event when header cell button is clicked', async () => {
    await wrapper.find('button[title="切换表头单元格"]').trigger('click');
    expect(wrapper.emitted('toggle-header-cell')).toBeTruthy();
  });
});
