import { ref } from 'vue';
import { useEditor, EditorContent } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import TextAlign from '@tiptap/extension-text-align';
import type { Level } from '@tiptap/extension-heading';

export function useDocumentOperations() {
  const editor = useEditor({
    extensions: [
      StarterKit,
      TextAlign.configure({
        types: ['heading', 'paragraph']
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
      console.error('Failed to paste:', err);
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
    getCharCount
  };
}
