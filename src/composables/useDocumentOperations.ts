import { useEditor } from '@tiptap/vue-3';
import { logger, LogCategory } from '../utils/logger';
import StarterKit from '@tiptap/starter-kit';
import TextAlign from '@tiptap/extension-text-align';
import Image from '@tiptap/extension-image';
import Link from '@tiptap/extension-link';
import Highlight from '@tiptap/extension-highlight';
import CodeBlockLowlight from '@tiptap/extension-code-block-lowlight';
import TaskList from '@tiptap/extension-task-list';
import TaskItem from '@tiptap/extension-task-item';
import FontFamily from '@tiptap/extension-font-family';
import Typography from '@tiptap/extension-typography';
import { TextStyle } from '@tiptap/extension-text-style';
import Placeholder from '@tiptap/extension-placeholder';
import Underline from '@tiptap/extension-underline';
import Strike from '@tiptap/extension-strike';
import Subscript from '@tiptap/extension-subscript';
import Superscript from '@tiptap/extension-superscript';
import { Table } from '@tiptap/extension-table';
import TableRow from '@tiptap/extension-table-row';
import TableCell from '@tiptap/extension-table-cell';
import TableHeader from '@tiptap/extension-table-header';
import { common, createLowlight } from 'lowlight';
import type { Level } from '@tiptap/extension-heading';
import ListKeymap from '@tiptap/extension-list-keymap';
import TableOfContents from '@tiptap/extension-table-of-contents';
import Emoji from '@tiptap/extension-emoji';

export function useDocumentOperations() {
  const lowlight = createLowlight(common);

  const editor = useEditor({
    extensions: [
      StarterKit.configure({
        codeBlock: false // Disable default code block to use CodeBlockLowlight
      }),
      TextAlign.configure({
        types: ['heading', 'paragraph']
      }),
      Image.configure({
        inline: true,
        allowBase64: true
      }),
      Link.configure({
        openOnClick: false,
        HTMLAttributes: {
          class: 'text-blue-500 underline'
        }
      }),
      Highlight.configure({
        multicolor: true
      }),
      CodeBlockLowlight.configure({
        lowlight
      }),
      TaskList,
      TaskItem.configure({
        nested: true
      }),
      FontFamily,
      Typography,
      TextStyle,
      Placeholder.configure({
        placeholder: '开始输入内容...'
      }),
      Underline,
      Strike,
      Subscript,
      Superscript,
      Table.configure({
        resizable: true
      }),
      TableRow,
      TableHeader,
      TableCell,
      ListKeymap,
      TableOfContents,
      Emoji.configure({
        suggestion: {
          items: ({ query }) => {
            const emojis = [
              { emoji: '😀', name: 'grinning face' },
              { emoji: '😃', name: 'grinning face with big eyes' },
              { emoji: '😄', name: 'grinning face with smiling eyes' },
              { emoji: '😁', name: 'beaming face with smiling eyes' },
              { emoji: '😅', name: 'grinning face with sweat' },
              { emoji: '😂', name: 'face with tears of joy' },
              { emoji: '🤣', name: 'rolling on the floor laughing' },
              { emoji: '😊', name: 'smiling face with smiling eyes' },
              { emoji: '😇', name: 'smiling face with halo' },
              { emoji: '🙂', name: 'slightly smiling face' },
              { emoji: '🙃', name: 'upside-down face' },
              { emoji: '😉', name: 'winking face' },
              { emoji: '😌', name: 'relieved face' },
              { emoji: '😍', name: 'smiling face with heart-eyes' },
              { emoji: '🥰', name: 'smiling face with hearts' },
              { emoji: '😘', name: 'face blowing a kiss' },
              { emoji: '😎', name: 'smiling face with sunglasses' },
              { emoji: '🤩', name: 'star-struck' },
              { emoji: '🥳', name: 'partying face' },
              { emoji: '😢', name: 'crying face' },
              { emoji: '😭', name: 'loudly crying face' },
              { emoji: '😡', name: 'pouting face' },
              { emoji: '👍', name: 'thumbs up' },
              { emoji: '👎', name: 'thumbs down' },
              { emoji: '❤️', name: 'red heart' },
              { emoji: '✨', name: 'sparkles' },
              { emoji: '🎉', name: 'party popper' }
            ];
            return emojis.filter(item => 
              item.emoji.includes(query) || item.name.includes(query)
            ).slice(0, 10);
          }
        }
      })
    ],
    content: '',
    editorProps: {
      attributes: {
        class: 'prose prose-sm sm:prose lg:prose-lg xl:prose-2xl mx-auto focus:outline-none'
      }
    }
  });

  const isActive = (name: string, attributes = {}) => {
    return editor.value?.isActive(name, attributes) || false;
  };

  const toggleBold = () => {
    editor.value?.chain().focus().toggleBold().run();
  };

  const toggleItalic = () => {
    editor.value?.chain().focus().toggleItalic().run();
  };

  const toggleUnderline = () => {
    editor.value?.chain().focus().toggleUnderline().run();
  };

  const toggleStrike = () => {
    editor.value?.chain().focus().toggleStrike().run();
  };

  const toggleCode = () => {
    editor.value?.chain().focus().toggleCode().run();
  };

  const toggleHeading = (level: Level) => {
    editor.value?.chain().focus().toggleHeading({ level }).run();
  };

  const toggleBulletList = () => {
    editor.value?.chain().focus().toggleBulletList().run();
  };

  const toggleOrderedList = () => {
    editor.value?.chain().focus().toggleOrderedList().run();
  };

  const toggleBlockquote = () => {
    editor.value?.chain().focus().toggleBlockquote().run();
  };

  const setTextAlign = (alignment: 'left' | 'center' | 'right' | 'justify') => {
    editor.value?.chain().focus().setTextAlign(alignment).run();
  };

  const undo = () => {
    editor.value?.chain().focus().undo().run();
  };

  const redo = () => {
    editor.value?.chain().focus().redo().run();
  };

  const cutSelection = () => {
    document.execCommand('cut');
  };

  const copySelection = () => {
    document.execCommand('copy');
  };

  const pasteFromClipboard = async () => {
    try {
      const text = await navigator.clipboard.readText();
      editor.value?.chain().focus().insertContent(text).run();
    } catch (err) {
      logger.error('Failed to paste', err, LogCategory.SYSTEM);
    }
  };

  const selectAll = () => {
    editor.value?.chain().focus().selectAll().run();
  };

  const clearFormatting = () => {
    editor.value?.chain().focus().unsetAllMarks().run();
    editor.value?.chain().focus().clearNodes().run();
  };

  const getHTML = () => {
    return editor.value?.getHTML() || '';
  };

  const setContent = (content: string) => {
    editor.value?.commands.setContent(content);
  };

  const getWordCount = () => {
    const text = editor.value?.getText() || '';
    return text.trim().split(/\s+/).filter(word => word.length > 0).length;
  };

  const getCharCount = () => {
    return editor.value?.getText().length || 0;
  };

  // Image functions
  const addImage = (src: string, alt?: string) => {
    editor.value?.chain().focus().setImage({ src, alt }).run();
  };

  const setImageSize = (width?: number, height?: number) => {
    editor.value?.chain().focus().updateAttributes('image', { width, height }).run();
  };

  // Link functions
  const setLink = (href: string, _text?: string) => {
    if (href === '') {
      editor.value?.chain().focus().unsetLink().run();
    } else {
      editor.value?.chain().focus().setLink({ href }).run();
    }
  };

  const unsetLink = () => {
    editor.value?.chain().focus().unsetLink().run();
  };

  // Highlight functions
  const toggleHighlight = (color?: string) => {
    if (color) {
      editor.value?.chain().focus().toggleHighlight({ color }).run();
    } else {
      editor.value?.chain().focus().toggleHighlight().run();
    }
  };

  // Task list functions
  const toggleTaskList = () => {
    editor.value?.chain().focus().toggleTaskList().run();
  };

  // Font family functions
  const setFontFamily = (fontFamily: string) => {
    editor.value?.chain().focus().setFontFamily(fontFamily).run();
  };

  // Subscript/Superscript functions
  const toggleSubscript = () => {
    editor.value?.chain().focus().toggleSubscript().run();
  };

  const toggleSuperscript = () => {
    editor.value?.chain().focus().toggleSuperscript().run();
  };

  // Table functions
  const insertTable = ({ rows = 3, cols = 3, withHeaderRow = true } = {}) => {
    editor.value?.chain().focus().insertTable({ rows, cols, withHeaderRow }).run();
  };

  const deleteTable = () => {
    editor.value?.chain().focus().deleteTable().run();
  };

  const addColumnBefore = () => {
    editor.value?.chain().focus().addColumnBefore().run();
  };

  const addColumnAfter = () => {
    editor.value?.chain().focus().addColumnAfter().run();
  };

  const deleteColumn = () => {
    editor.value?.chain().focus().deleteColumn().run();
  };

  const addRowBefore = () => {
    editor.value?.chain().focus().addRowBefore().run();
  };

  const addRowAfter = () => {
    editor.value?.chain().focus().addRowAfter().run();
  };

  const deleteRow = () => {
    editor.value?.chain().focus().deleteRow().run();
  };

  const mergeCells = () => {
    editor.value?.chain().focus().mergeCells().run();
  };

  const splitCell = () => {
    editor.value?.chain().focus().splitCell().run();
  };

  const toggleHeaderColumn = () => {
    editor.value?.chain().focus().toggleHeaderColumn().run();
  };

  const toggleHeaderRow = () => {
    editor.value?.chain().focus().toggleHeaderRow().run();
  };

  const toggleHeaderCell = () => {
    editor.value?.chain().focus().toggleHeaderCell().run();
  };

  // Table of contents functions
  const insertTableOfContents = () => {
    editor.value?.chain().focus().insertContent('<div class="table-of-contents" data-type="tableOfContents"></div>').run();
  };

  // Emoji functions
  const insertEmoji = (emoji: string) => {
    editor.value?.chain().focus().insertContent(emoji).run();
  };

  return {
    editor,
    isActive,
    toggleBold,
    toggleItalic,
    toggleUnderline,
    toggleStrike,
    toggleCode,
    toggleHeading,
    toggleBulletList,
    toggleOrderedList,
    toggleBlockquote,
    setTextAlign,
    undo,
    redo,
    cutSelection,
    copySelection,
    pasteFromClipboard,
    selectAll,
    clearFormatting,
    getHTML,
    setContent,
    getWordCount,
    getCharCount,
    // Image functions
    addImage,
    setImageSize,
    // Link functions
    setLink,
    unsetLink,
    // Highlight functions
    toggleHighlight,
    // Task list functions
    toggleTaskList,
    // Font family functions
    setFontFamily,
    // Subscript/Superscript functions
    toggleSubscript,
    toggleSuperscript,
    // Table functions
    insertTable,
    deleteTable,
    addColumnBefore,
    addColumnAfter,
    deleteColumn,
    addRowBefore,
    addRowAfter,
    deleteRow,
    mergeCells,
    splitCell,
    toggleHeaderColumn,
    toggleHeaderRow,
    toggleHeaderCell,
    // Table of contents functions
    insertTableOfContents,
    // Emoji functions
    insertEmoji
  };
}
