/**
 * 状态纽带 - Tiptap驱动Ribbon工具栏和右键菜单
 * 
 * 功能：
 * 1. 实时感知Tiptap编辑器光标位置和当前状态
 * 2. 根据编辑器状态动态控制Ribbon工具栏按钮的亮灭
 * 3. 根据编辑器状态动态显示右键菜单选项
 * 4. 支持表格、列表、标题等特殊状态的检测
 */

import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import { Editor } from '@tiptap/vue-3';
import { Table } from '@tiptap/extension-table';
import { TableRow } from '@tiptap/extension-table-row';
import { TableCell } from '@tiptap/extension-table-cell';
import { TableHeader } from '@tiptap/extension-table-header';
import { logger, LogCategory } from '../utils/logger';

// 编辑器状态类型
export interface EditorState {
  // 文本格式状态
  isBold: boolean;
  isItalic: boolean;
  isUnderline: boolean;
  isStrike: boolean;
  isCode: boolean;
  
  // 标题状态
  headingLevel: number | null;
  
  // 列表状态
  isBulletList: boolean;
  isOrderedList: boolean;
  isTaskList: boolean;
  
  // 引用状态
  isBlockquote: boolean;
  
  // 代码块状态
  isCodeBlock: boolean;
  
  // 表格状态
  isInTable: boolean;
  isInTableHeader: boolean;
  isInTableCell: boolean;
  tableSelectedCells: { from: number; to: number } | null;
  
  // 对齐状态
  textAlign: 'left' | 'center' | 'right' | 'justify' | null;
  
  // 链接状态
  isLink: boolean;
  linkUrl: string | null;
  
  // 光标位置
  cursorPosition: number | null;
  selectionRange: { from: number; to: number } | null;
  
  // 当前节点类型
  nodeType: string | null;
}

// Ribbon工具栏按钮状态
export interface RibbonButtonState {
  id: string;
  label: string;
  icon: string;
  enabled: boolean;
  active: boolean;
  shortcut?: string;
}

// 右键菜单项
export interface ContextMenuItem {
  id: string;
  label: string;
  icon: string;
  enabled: boolean;
  separator?: boolean;
  children?: ContextMenuItem[];
  action?: () => void;
}

export function useStateSync(editor: Editor | null) {
  // 编辑器状态
  const editorState = ref<EditorState>({
    isBold: false,
    isItalic: false,
    isUnderline: false,
    isStrike: false,
    isCode: false,
    headingLevel: null,
    isBulletList: false,
    isOrderedList: false,
    isTaskList: false,
    isBlockquote: false,
    isCodeBlock: false,
    isInTable: false,
    isInTableHeader: false,
    isInTableCell: false,
    tableSelectedCells: null,
    textAlign: null,
    isLink: false,
    linkUrl: null,
    cursorPosition: null,
    selectionRange: null,
    nodeType: null
  });

  // 右键菜单显示状态
  const showContextMenu = ref(false);
  const contextMenuPosition = ref({ x: 0, y: 0 });

  // 更新编辑器状态
  const updateEditorState = () => {
    if (!editor) {
return;
}

    const state = editorState.value;
    
    // 文本格式状态
    state.isBold = editor.isActive('bold');
    state.isItalic = editor.isActive('italic');
    state.isUnderline = editor.isActive('underline');
    state.isStrike = editor.isActive('strike');
    state.isCode = editor.isActive('code');
    
    // 标题状态
    state.headingLevel = null;
    for (let level = 1; level <= 6; level++) {
      if (editor.isActive('heading', { level })) {
        state.headingLevel = level;
        break;
      }
    }
    
    // 列表状态
    state.isBulletList = editor.isActive('bulletList');
    state.isOrderedList = editor.isActive('orderedList');
    state.isTaskList = editor.isActive('taskList');
    
    // 引用状态
    state.isBlockquote = editor.isActive('blockquote');
    
    // 代码块状态
    state.isCodeBlock = editor.isActive('codeBlock');
    
    // 表格状态
    state.isInTable = editor.isActive('table');
    state.isInTableHeader = editor.isActive('tableHeader');
    state.isInTableCell = editor.isActive('tableCell');
    
    // 获取表格选中的单元格
    if (state.isInTable) {
      try {
        const { from, to } = editor.state.selection;
        state.tableSelectedCells = { from, to };
      } catch (error) {
        state.tableSelectedCells = null;
      }
    } else {
      state.tableSelectedCells = null;
    }
    
    // 对齐状态
    state.textAlign = null;
    if (editor.isActive({ textAlign: 'left' })) {
state.textAlign = 'left';
} else if (editor.isActive({ textAlign: 'center' })) {
state.textAlign = 'center';
} else if (editor.isActive({ textAlign: 'right' })) {
state.textAlign = 'right';
} else if (editor.isActive({ textAlign: 'justify' })) {
state.textAlign = 'justify';
}
    
    // 链接状态
    state.isLink = editor.isActive('link');
    if (state.isLink) {
      const linkMark = editor.state.doc.rangeHasMark(
        editor.state.selection.from,
        editor.state.selection.to,
        editor.state.schema.marks.link
      );
      if (linkMark) {
        state.linkUrl = editor.getAttributes('link').href;
      }
    } else {
      state.linkUrl = null;
    }
    
    // 光标位置
    state.cursorPosition = editor.state.selection.from;
    state.selectionRange = {
      from: editor.state.selection.from,
      to: editor.state.selection.to
    };
    
    // 当前节点类型
    const { from } = editor.state.selection;
    const node = editor.state.doc.nodeAt(from);
    state.nodeType = node?.type.name || null;
    
    logger.debug('Editor state updated', { state }, LogCategory.UI);
  };

  // Ribbon工具栏按钮状态
  const ribbonButtons = computed<RibbonButtonState[]>(() => {
    const state = editorState.value;
    
    return [
      // 文本格式
      {
        id: 'bold',
        label: '粗体',
        icon: 'B',
        enabled: true,
        active: state.isBold,
        shortcut: 'Ctrl+B'
      },
      {
        id: 'italic',
        label: '斜体',
        icon: 'I',
        enabled: true,
        active: state.isItalic,
        shortcut: 'Ctrl+I'
      },
      {
        id: 'underline',
        label: '下划线',
        icon: 'U',
        enabled: true,
        active: state.isUnderline,
        shortcut: 'Ctrl+U'
      },
      {
        id: 'strike',
        label: '删除线',
        icon: 'S',
        enabled: true,
        active: state.isStrike
      },
      
      // 标题
      {
        id: 'heading1',
        label: '标题1',
        icon: 'H1',
        enabled: true,
        active: state.headingLevel === 1
      },
      {
        id: 'heading2',
        label: '标题2',
        icon: 'H2',
        enabled: true,
        active: state.headingLevel === 2
      },
      {
        id: 'heading3',
        label: '标题3',
        icon: 'H3',
        enabled: true,
        active: state.headingLevel === 3
      },
      
      // 列表
      {
        id: 'bulletList',
        label: '无序列表',
        icon: '•',
        enabled: true,
        active: state.isBulletList
      },
      {
        id: 'orderedList',
        label: '有序列表',
        icon: '1.',
        enabled: true,
        active: state.isOrderedList
      },
      
      // 引用
      {
        id: 'blockquote',
        label: '引用',
        icon: '"',
        enabled: true,
        active: state.isBlockquote
      },
      
      // 对齐
      {
        id: 'alignLeft',
        label: '左对齐',
        icon: '⬅',
        enabled: true,
        active: state.textAlign === 'left'
      },
      {
        id: 'alignCenter',
        label: '居中',
        icon: '⬌',
        enabled: true,
        active: state.textAlign === 'center'
      },
      {
        id: 'alignRight',
        label: '右对齐',
        icon: '➡',
        enabled: true,
        active: state.textAlign === 'right'
      },
      
      // 表格（仅在表格中启用）
      {
        id: 'insertTable',
        label: '插入表格',
        icon: '⊞',
        enabled: !state.isInTable,
        active: false
      },
      {
        id: 'addRow',
        label: '添加行',
        icon: '+行',
        enabled: state.isInTable,
        active: false
      },
      {
        id: 'deleteRow',
        label: '删除行',
        icon: '-行',
        enabled: state.isInTable,
        active: false
      },
      {
        id: 'addColumn',
        label: '添加列',
        icon: '+列',
        enabled: state.isInTable,
        active: false
      },
      {
        id: 'deleteColumn',
        label: '删除列',
        icon: '-列',
        enabled: state.isInTable,
        active: false
      },
      
      // 链接
      {
        id: 'link',
        label: '链接',
        icon: '🔗',
        enabled: true,
        active: state.isLink
      }
    ];
  });

  // 右键菜单项
  const contextMenuItems = computed<ContextMenuItem[]>(() => {
    const state = editorState.value;
    const items: ContextMenuItem[] = [];
    
    // 基础编辑操作
    items.push(
      { id: 'undo', label: '撤销', icon: '↶', enabled: true },
      { id: 'redo', label: '重做', icon: '↷', enabled: true },
      { id: 'separator1', label: '', icon: '', enabled: true, separator: true },
      { id: 'cut', label: '剪切', icon: '✂', enabled: true },
      { id: 'copy', label: '复制', icon: '📋', enabled: true },
      { id: 'paste', label: '粘贴', icon: '📄', enabled: true },
      { id: 'separator2', label: '', icon: '', enabled: true, separator: true }
    );
    
    // 文本格式
    items.push(
      { id: 'bold', label: '粗体', icon: 'B', enabled: true, action: () => editor?.chain().focus().toggleBold().run() },
      { id: 'italic', label: '斜体', icon: 'I', enabled: true, action: () => editor?.chain().focus().toggleItalic().run() },
      { id: 'underline', label: '下划线', icon: 'U', enabled: true, action: () => editor?.chain().focus().toggleUnderline().run() },
      { id: 'separator3', label: '', icon: '', enabled: true, separator: true }
    );
    
    // 标题
    items.push({
      id: 'heading',
      label: '标题',
      icon: 'H',
      enabled: true,
      children: [
        { id: 'heading1', label: '标题1', icon: 'H1', enabled: true, action: () => editor?.chain().focus().toggleHeading({ level: 1 }).run() },
        { id: 'heading2', label: '标题2', icon: 'H2', enabled: true, action: () => editor?.chain().focus().toggleHeading({ level: 2 }).run() },
        { id: 'heading3', label: '标题3', icon: 'H3', enabled: true, action: () => editor?.chain().focus().toggleHeading({ level: 3 }).run() }
      ]
    });
    
    // 列表
    items.push(
      { id: 'bulletList', label: '无序列表', icon: '•', enabled: true, action: () => editor?.chain().focus().toggleBulletList().run() },
      { id: 'orderedList', label: '有序列表', icon: '1.', enabled: true, action: () => editor?.chain().focus().toggleOrderedList().run() },
      { id: 'separator4', label: '', icon: '', enabled: true, separator: true }
    );
    
    // 表格操作（仅在表格中显示）
    if (state.isInTable) {
      items.push(
        { id: 'tableHeader', label: '表格工具', icon: '⊞', enabled: true },
        { id: 'addRow', label: '插入行', icon: '+行', enabled: true, action: () => editor?.chain().focus().addRowAfter().run() },
        { id: 'deleteRow', label: '删除行', icon: '-行', enabled: true, action: () => editor?.chain().focus().deleteRow().run() },
        { id: 'addColumn', label: '插入列', icon: '+列', enabled: true, action: () => editor?.chain().focus().addColumnAfter().run() },
        { id: 'deleteColumn', label: '删除列', icon: '-列', enabled: true, action: () => editor?.chain().focus().deleteColumn().run() },
        { id: 'mergeCells', label: '合并单元格', icon: '⊞', enabled: true, action: () => editor?.chain().focus().mergeCells().run() },
        { id: 'splitCell', label: '拆分单元格', icon: '⊟', enabled: true, action: () => editor?.chain().focus().splitCell().run() },
        { id: 'separator5', label: '', icon: '', enabled: true, separator: true }
      );
    }
    
    // 对齐
    items.push({
      id: 'align',
      label: '对齐',
      icon: '⬌',
      enabled: true,
      children: [
        { id: 'alignLeft', label: '左对齐', icon: '⬅', enabled: true, action: () => editor?.chain().focus().setTextAlign('left').run() },
        { id: 'alignCenter', label: '居中', icon: '⬌', enabled: true, action: () => editor?.chain().focus().setTextAlign('center').run() },
        { id: 'alignRight', label: '右对齐', icon: '➡', enabled: true, action: () => editor?.chain().focus().setTextAlign('right').run() },
        { id: 'alignJustify', label: '两端对齐', icon: '≡', enabled: true, action: () => editor?.chain().focus().setTextAlign('justify').run() }
      ]
    });
    
    // 链接
    if (state.isLink) {
      items.push(
        { id: 'editLink', label: '编辑链接', icon: '✏', enabled: true },
        { id: 'removeLink', label: '移除链接', icon: '✕', enabled: true, action: () => editor?.chain().focus().unsetLink().run() }
      );
    } else {
      items.push({ id: 'addLink', label: '添加链接', icon: '🔗', enabled: true });
    }
    
    return items;
  });

  // 处理右键菜单
  const handleContextMenu = (event: MouseEvent) => {
    event.preventDefault();
    
    // 更新编辑器状态
    updateEditorState();
    
    // 显示右键菜单
    contextMenuPosition.value = {
      x: event.clientX,
      y: event.clientY
    };
    showContextMenu.value = true;
    
    logger.debug('Context menu shown', { position: contextMenuPosition.value }, LogCategory.UI);
  };

  // 隐藏右键菜单
  const hideContextMenu = () => {
    showContextMenu.value = false;
  };

  // 执行右键菜单项操作
  const executeContextMenuItem = (item: ContextMenuItem) => {
    if (item.action) {
      item.action();
    }
    hideContextMenu();
  };

  // 监听编辑器选择变化
  const handleSelectionUpdate = () => {
    updateEditorState();
  };

  // 生命周期
  onMounted(() => {
    if (editor) {
      editor.on('selectionUpdate', handleSelectionUpdate);
      editor.on('transaction', handleSelectionUpdate);
      
      // 初始状态更新
      updateEditorState();
      
      logger.info('State sync mounted', {}, LogCategory.UI);
    }
  });

  onUnmounted(() => {
    if (editor) {
      editor.off('selectionUpdate', handleSelectionUpdate);
      editor.off('transaction', handleSelectionUpdate);
      
      logger.info('State sync unmounted', {}, LogCategory.UI);
    }
  });

  return {
    // 状态
    editorState,
    showContextMenu,
    contextMenuPosition,
    
    // 计算属性
    ribbonButtons,
    contextMenuItems,
    
    // 方法
    updateEditorState,
    handleContextMenu,
    hideContextMenu,
    executeContextMenuItem
  };
}
