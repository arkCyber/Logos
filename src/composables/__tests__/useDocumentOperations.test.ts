import { describe, it, expect, beforeEach, vi } from 'vitest';
import { useDocumentOperations } from '../useDocumentOperations';

// Mock Tiptap editor
const mockEditor = {
  chain: vi.fn(() => ({
    focus: vi.fn(() => ({
      toggleBold: vi.fn(() => ({ run: vi.fn() })),
      toggleItalic: vi.fn(() => ({ run: vi.fn() })),
      toggleUnderline: vi.fn(() => ({ run: vi.fn() })),
      toggleStrike: vi.fn(() => ({ run: vi.fn() })),
      toggleCode: vi.fn(() => ({ run: vi.fn() })),
      toggleHeading: vi.fn(() => ({ run: vi.fn() })),
      toggleBulletList: vi.fn(() => ({ run: vi.fn() })),
      toggleOrderedList: vi.fn(() => ({ run: vi.fn() })),
      toggleBlockquote: vi.fn(() => ({ run: vi.fn() })),
      setTextAlign: vi.fn(() => ({ run: vi.fn() })),
      undo: vi.fn(() => ({ run: vi.fn() })),
      redo: vi.fn(() => ({ run: vi.fn() })),
      unsetAllMarks: vi.fn(() => ({ run: vi.fn() })),
      clearNodes: vi.fn(() => ({ run: vi.fn() })),
      insertContent: vi.fn(() => ({ run: vi.fn() })),
      selectAll: vi.fn(() => ({ run: vi.fn() })),
      setImage: vi.fn(() => ({ run: vi.fn() })),
      updateAttributes: vi.fn(() => ({ run: vi.fn() })),
      setLink: vi.fn(() => ({ run: vi.fn() })),
      unsetLink: vi.fn(() => ({ run: vi.fn() })),
      toggleHighlight: vi.fn(() => ({ run: vi.fn() })),
      toggleTaskList: vi.fn(() => ({ run: vi.fn() })),
      setFontFamily: vi.fn(() => ({ run: vi.fn() })),
      toggleSubscript: vi.fn(() => ({ run: vi.fn() })),
      toggleSuperscript: vi.fn(() => ({ run: vi.fn() })),
      insertTable: vi.fn(() => ({ run: vi.fn() })),
      deleteTable: vi.fn(() => ({ run: vi.fn() })),
      addColumnBefore: vi.fn(() => ({ run: vi.fn() })),
      addColumnAfter: vi.fn(() => ({ run: vi.fn() })),
      deleteColumn: vi.fn(() => ({ run: vi.fn() })),
      addRowBefore: vi.fn(() => ({ run: vi.fn() })),
      addRowAfter: vi.fn(() => ({ run: vi.fn() })),
      deleteRow: vi.fn(() => ({ run: vi.fn() })),
      mergeCells: vi.fn(() => ({ run: vi.fn() })),
      splitCell: vi.fn(() => ({ run: vi.fn() })),
      toggleHeaderColumn: vi.fn(() => ({ run: vi.fn() })),
      toggleHeaderRow: vi.fn(() => ({ run: vi.fn() })),
      toggleHeaderCell: vi.fn(() => ({ run: vi.fn() }))
    }))
  })),
  getHTML: vi.fn(() => '<p>Test content</p>'),
  getText: vi.fn(() => 'Test content'),
  commands: {
    setContent: vi.fn()
  },
  isActive: vi.fn(() => false)
};

// Mock useEditor hook
vi.mock('@tiptap/vue-3', () => ({
  useEditor: vi.fn(() => mockEditor)
}));

describe('useDocumentOperations', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Basic Text Formatting', () => {
    it('should toggle bold text', () => {
      const { toggleBold } = useDocumentOperations();
      toggleBold();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should toggle italic text', () => {
      const { toggleItalic } = useDocumentOperations();
      toggleItalic();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should toggle underline', () => {
      const { toggleUnderline } = useDocumentOperations();
      toggleUnderline();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should toggle strike', () => {
      const { toggleStrike } = useDocumentOperations();
      toggleStrike();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should toggle code', () => {
      const { toggleCode } = useDocumentOperations();
      toggleCode();
      expect(mockEditor.chain).toHaveBeenCalled();
    });
  });

  describe('Heading and Lists', () => {
    it('should toggle heading with level', () => {
      const { toggleHeading } = useDocumentOperations();
      toggleHeading(1);
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should toggle bullet list', () => {
      const { toggleBulletList } = useDocumentOperations();
      toggleBulletList();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should toggle ordered list', () => {
      const { toggleOrderedList } = useDocumentOperations();
      toggleOrderedList();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should toggle blockquote', () => {
      const { toggleBlockquote } = useDocumentOperations();
      toggleBlockquote();
      expect(mockEditor.chain).toHaveBeenCalled();
    });
  });

  describe('Text Alignment', () => {
    it('should set text alignment to left', () => {
      const { setTextAlign } = useDocumentOperations();
      setTextAlign('left');
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should set text alignment to center', () => {
      const { setTextAlign } = useDocumentOperations();
      setTextAlign('center');
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should set text alignment to right', () => {
      const { setTextAlign } = useDocumentOperations();
      setTextAlign('right');
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should set text alignment to justify', () => {
      const { setTextAlign } = useDocumentOperations();
      setTextAlign('justify');
      expect(mockEditor.chain).toHaveBeenCalled();
    });
  });

  describe('History Operations', () => {
    it('should undo', () => {
      const { undo } = useDocumentOperations();
      undo();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should redo', () => {
      const { redo } = useDocumentOperations();
      redo();
      expect(mockEditor.chain).toHaveBeenCalled();
    });
  });

  describe('Clipboard Operations', () => {
    it('should cut selection', () => {
      const { cutSelection } = useDocumentOperations();
      cutSelection();
      expect(document.execCommand).toHaveBeenCalledWith('cut');
    });

    it('should copy selection', () => {
      const { copySelection } = useDocumentOperations();
      copySelection();
      expect(document.execCommand).toHaveBeenCalledWith('copy');
    });
  });

  describe('Image Operations', () => {
    it('should add image with src', () => {
      const { addImage } = useDocumentOperations();
      addImage('https://example.com/image.jpg', 'Test image');
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should set image size', () => {
      const { setImageSize } = useDocumentOperations();
      setImageSize(300, 200);
      expect(mockEditor.chain).toHaveBeenCalled();
    });
  });

  describe('Link Operations', () => {
    it('should set link', () => {
      const { setLink } = useDocumentOperations();
      setLink('https://example.com');
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should unset link when href is empty', () => {
      const { setLink } = useDocumentOperations();
      setLink('');
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should unset link', () => {
      const { unsetLink } = useDocumentOperations();
      unsetLink();
      expect(mockEditor.chain).toHaveBeenCalled();
    });
  });

  describe('Highlight Operations', () => {
    it('should toggle highlight with color', () => {
      const { toggleHighlight } = useDocumentOperations();
      toggleHighlight('#ffff00');
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should toggle highlight without color', () => {
      const { toggleHighlight } = useDocumentOperations();
      toggleHighlight();
      expect(mockEditor.chain).toHaveBeenCalled();
    });
  });

  describe('Task List Operations', () => {
    it('should toggle task list', () => {
      const { toggleTaskList } = useDocumentOperations();
      toggleTaskList();
      expect(mockEditor.chain).toHaveBeenCalled();
    });
  });

  describe('Font Family Operations', () => {
    it('should set font family', () => {
      const { setFontFamily } = useDocumentOperations();
      setFontFamily('Arial, sans-serif');
      expect(mockEditor.chain).toHaveBeenCalled();
    });
  });

  describe('Subscript and Superscript', () => {
    it('should toggle subscript', () => {
      const { toggleSubscript } = useDocumentOperations();
      toggleSubscript();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should toggle superscript', () => {
      const { toggleSuperscript } = useDocumentOperations();
      toggleSuperscript();
      expect(mockEditor.chain).toHaveBeenCalled();
    });
  });

  describe('Table Operations', () => {
    it('should insert table with default options', () => {
      const { insertTable } = useDocumentOperations();
      insertTable();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should insert table with custom options', () => {
      const { insertTable } = useDocumentOperations();
      insertTable({ rows: 5, cols: 4, withHeaderRow: true });
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should delete table', () => {
      const { deleteTable } = useDocumentOperations();
      deleteTable();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should add column before', () => {
      const { addColumnBefore } = useDocumentOperations();
      addColumnBefore();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should add column after', () => {
      const { addColumnAfter } = useDocumentOperations();
      addColumnAfter();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should delete column', () => {
      const { deleteColumn } = useDocumentOperations();
      deleteColumn();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should add row before', () => {
      const { addRowBefore } = useDocumentOperations();
      addRowBefore();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should add row after', () => {
      const { addRowAfter } = useDocumentOperations();
      addRowAfter();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should delete row', () => {
      const { deleteRow } = useDocumentOperations();
      deleteRow();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should merge cells', () => {
      const { mergeCells } = useDocumentOperations();
      mergeCells();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should split cell', () => {
      const { splitCell } = useDocumentOperations();
      splitCell();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should toggle header column', () => {
      const { toggleHeaderColumn } = useDocumentOperations();
      toggleHeaderColumn();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should toggle header row', () => {
      const { toggleHeaderRow } = useDocumentOperations();
      toggleHeaderRow();
      expect(mockEditor.chain).toHaveBeenCalled();
    });

    it('should toggle header cell', () => {
      const { toggleHeaderCell } = useDocumentOperations();
      toggleHeaderCell();
      expect(mockEditor.chain).toHaveBeenCalled();
    });
  });

  describe('Content Operations', () => {
    it('should get HTML content', () => {
      const { getHTML } = useDocumentOperations();
      const html = getHTML();
      expect(html).toBe('<p>Test content</p>');
    });

    it('should set content', () => {
      const { setContent } = useDocumentOperations();
      setContent('<p>New content</p>');
      expect(mockEditor.commands.setContent).toHaveBeenCalledWith('<p>New content</p>');
    });

    it('should get word count', () => {
      const { getWordCount } = useDocumentOperations();
      const count = getWordCount();
      expect(count).toBe(2);
    });

    it('should get character count', () => {
      const { getCharCount } = useDocumentOperations();
      const count = getCharCount();
      expect(count).toBe(12);
    });
  });

  describe('Clear Formatting', () => {
    it('should clear formatting', () => {
      const { clearFormatting } = useDocumentOperations();
      clearFormatting();
      expect(mockEditor.chain).toHaveBeenCalled();
    });
  });
});
