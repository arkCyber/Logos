# Logos - AI-Powered Word Processing Software with Typst Integration

[English](./README.md) | [中文](./README.zh-CN.md)

Logos is a modern word processing software built with Tauri, Vue3, TipTap, and Tailwind CSS. It features AI-powered text editing capabilities including polish, expand, and rewrite functions. **The key innovation is the integration with Typst compilation engine, enabling real-time professional typesetting preview.**

## Features

- **Rich Text Editor**: Built on TipTap with ProseMirror, supporting:
  - Bold, italic, strikethrough, underline, code formatting
  - Text highlighting/mark functionality
  - Text transformation (uppercase/lowercase)
  - Headings (H1, H2, H3)
  - Bullet and ordered lists
  - Blockquotes
  - Resizable tables with full editing capabilities
  - Horizontal rules
  - Image insertion (base64 encoded)
  - Link insertion and management
  - Text alignment (left, center, right, justify)
  - Text indentation controls
  - Adjustable font size (12px - 32px)
  - Adjustable line height (1.0 - 2.5)
  - Code blocks with syntax highlighting (100+ languages)
  - Improved table styling with rounded corners and better borders
- **AI Integration**: Select text and use AI to:
  - Polish text for academic/professional tone (润色)
  - Expand content with more details (扩写)
  - Rewrite text while maintaining meaning (重写)
  - Summarize content concisely (总结)
  - Translate text to English (翻译)
  - **Streaming Responses**: AI responses appear in real-time with typewriter effect
- **Slash Commands**: Type `/` to access quick commands:
  - Headings (H1, H2, H3, Plain text)
  - Bullet and ordered lists
  - Blockquotes
  - Code blocks and inline code
  - Tables (2x2, 3x3, 4x4)
  - Text formatting (Bold, Italic, Strikethrough)
  - Horizontal rules
- **Document Statistics**: Real-time word, character, paragraph, and line count display
- **Reading Time**: Automatic reading time estimation (based on 200 words/minute)
- **Status Bar**: Bottom status bar showing document info, cursor position, and settings
- **Search and Replace**: Full-text search with replace functionality
- **Auto-Save**: Automatic saving to localStorage every 30 seconds (toggleable)
- **Dark/Light Theme**: Toggle between dark and light modes with full UI adaptation
- **Fullscreen Mode**: Toggle fullscreen editing mode
- **Print Support**: Native browser printing functionality
- **Clear Document**: Option to clear document with confirmation
- **Modern UI**: Clean, Word-style toolbar with Tailwind CSS styling
- **Keyboard Shortcuts**:
  - Ctrl/Cmd + S: Save document
  - Ctrl/Cmd + O: Open document
  - Ctrl/Cmd + N: New document
  - Ctrl/Cmd + B: Bold
  - Ctrl/Cmd + I: Italic
  - Ctrl/Cmd + U: Underline
  - Ctrl/Cmd + Z: Undo
  - Ctrl/Cmd + Shift + Z: Redo
  - Ctrl/Cmd + Y: Redo (alternative)
  - Ctrl/Cmd + F: Find
  - Ctrl/Cmd + H: Replace
  - Ctrl/Cmd + K: Insert link
  - Ctrl/Cmd + A: Select all
- **Export**: Export documents to Markdown, HTML, and plain text formats with proper styling
- **File Operations**: Save and load documents (HTML format) with native file dialogs
- **Recent Files**: Quick access to recently opened files (up to 10 files)
- **Document Templates**: Pre-built templates for common document types (blank, article, meeting notes, study notes, project plan, diary, book notes)
- **Keyboard Shortcuts Help**: Built-in keyboard shortcuts reference panel
- **Typst Integration**: Real-time professional typesetting preview
  - Dual-pane interface with live Typst rendering
  - HTML to Typst markup translation
  - Rust-powered compilation for millisecond-speed rendering
  - Debounced compilation to prevent performance issues
  - Base64-encoded PNG output for instant preview

## Tech Stack

- **Frontend**: Vue 3 + TypeScript + Vite
- **Desktop Framework**: Tauri 2
- **Editor**: TipTap (ProseMirror-based)
- **Styling**: Tailwind CSS 4
- **Backend**: Rust with reqwest for API calls
- **AI Provider**: DeepSeek API (configurable)

## Prerequisites

- Node.js (v18 or higher)
- Bun (recommended) or npm/yarn
- Rust and Cargo
- macOS, Windows, or Linux

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd LOGOS
```

2. Install dependencies:
```bash
bun install
```

3. Install Rust dependencies (if not already installed):
```bash
cd src-tauri
cargo build
cd ..
```

## Development

Run the development server:

```bash
bun run tauri dev
```

This will:
- Start the Vite dev server on port 1420
- Build and run the Tauri application
- Enable hot-reload for both frontend and backend changes

## Building for Production

Build the application for your platform:

```bash
bun run tauri build
```

The built application will be in `src-tauri/target/release/bundle/`.

## Configuration

### AI API Key

To use AI features, you need to configure your API key:

1. Copy the `.env.example` file to `.env`:
```bash
cp .env.example .env
```

2. In the `.env` file, replace `your_api_key_here` with your actual DeepSeek API key:
```env
AI_API_KEY=your_actual_api_key_here
```

3. To use a different AI provider, modify the `AiConfig` in `src-tauri/src/ai_service/mod.rs` to match your provider's API format.

**Note**: The `.env` file is included in `.gitignore` to prevent committing sensitive API keys to the repository.

## Usage

### Basic Editing

- Use the toolbar buttons to format text
- Insert tables using the table button
- Resize table columns by dragging the column borders

### AI Features

1. Select any text in the editor
2. A bubble menu will appear above the selection
3. Click on:
   - **润色 (Polish)**: Improve text for academic/professional tone
   - **扩写 (Expand)**: Add more details and content
   - **重写 (Rewrite)**: Rephrase while keeping the meaning

### Export

Click the download button in the toolbar to export your document as Markdown.

## Project Structure

```
LOGOS/
├── src/
│   ├── components/
│   │   └── Editor.vue       # Main TipTap editor component
│   ├── App.vue              # Root component
│   ├── main.ts              # Application entry point
│   └── style.css            # Global styles
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs           # Rust backend with AI integration
│   │   ├── ai_service/      # AI service module
│   │   ├── typist_service/  # Typst compilation service
│   │   ├── editing_engine_service/  # Editing engine service
│   │   └── tiptap_service/  # TipTap configuration service
│   └── Cargo.toml           # Rust dependencies
├── package.json             # Node.js dependencies
├── vite.config.ts           # Vite configuration
├── .env.example             # Environment variables template
└── README.zh-CN.md          # Chinese documentation
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
- [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Future Enhancements

- Streaming AI responses (typewriter effect)
- Slash menu for quick commands (like Notion)
- Typst integration for PDF export
- Real-time collaboration
- More AI features (summarize, translate, etc.)

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
