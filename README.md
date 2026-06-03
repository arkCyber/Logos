# Logos智道办公软件 - AI-Powered Office Suite

[English](./README.md) | [中文](./README.zh-CN.md)

<div align="center">

**Logos智道办公软件** is a modern, feature-rich office suite built with Tauri, Vue 3, and Rust. It combines AI-powered text editing, professional typesetting with Typst, spreadsheet capabilities, presentation tools, and more in a single desktop application.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.5-green)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.80-orange)](https://www.rust-lang.org/)

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [Development](#development) • [Contributing](#contributing)

</div>

## Table of Contents

- [Features](#features)
- [Screenshots](#screenshots)
- [Tech Stack](#tech-stack)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Development](#development)
- [Building for Production](#building-for-production)
- [Configuration](#configuration)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Testing](#testing)
- [Architecture](#architecture)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Features

### 📝 Rich Text Editor
Built on TipTap with ProseMirror, supporting:
- **Text Formatting**: Bold, italic, strikethrough, underline, code formatting
- **Text Highlighting**: Mark functionality for emphasis
- **Text Transformation**: Uppercase/lowercase conversion
- **Headings**: H1, H2, H3 support
- **Lists**: Bullet and ordered lists with full customization
- **Blockquotes**: Nested blockquote support
- **Tables**: Resizable tables with full editing capabilities
- **Horizontal Rules**: Visual separators
- **Images**: Base64 encoded image insertion
- **Links**: Link insertion and management
- **Text Alignment**: Left, center, right, justify
- **Text Indentation**: Precise indentation controls
- **Font Size**: Adjustable (12px - 32px)
- **Line Height**: Adjustable (1.0 - 2.5)
- **Code Blocks**: Syntax highlighting for 100+ languages
- **Task Lists**: Interactive todo lists

### 🤖 AI Integration
Select text and use AI to:
- **Polish (润色)**: Improve text for academic/professional tone
- **Expand (扩写)**: Add more details and content
- **Rewrite (重写)**: Rephrase while keeping the meaning
- **Summarize (总结)**: Concise content summarization
- **Translate (翻译)**: Translate text to English
- **Streaming Responses**: Real-time typewriter effect for AI responses

### ⚡ Slash Commands
Type `/` to access quick commands:
- Headings (H1, H2, H3, Plain text)
- Bullet and ordered lists
- Blockquotes
- Code blocks and inline code
- Tables (2x2, 3x3, 4x4)
- Text formatting (Bold, Italic, Strikethrough)
- Horizontal rules

### 📊 Spreadsheet Module
- Full spreadsheet functionality with UniverJS
- Formula engine support
- Chart generation and visualization
- Import/Export Excel files
- Cell formatting and styling

### 🎯 Presentation Module
- PPT creation and editing
- Slide templates
- Export to various formats
- Animation support

### 📄 Document Processing
- **Typst Integration**: Real-time professional typesetting preview
  - Dual-pane interface with live Typst rendering
  - HTML to Typst markup translation
  - Rust-powered compilation for millisecond-speed rendering
  - Debounced compilation for performance
  - Base64-encoded PNG output for instant preview
- **Export Formats**: Markdown, HTML, RTF, DOCX, PDF
- **Import Formats**: DOCX, HTML, Markdown
- **PDF Conversion**: Built-in PDF generation
- **OCR**: Text extraction from images using Tesseract

### 🎨 UI/UX Features
- **Modern UI**: Clean, Word-style toolbar with Tailwind CSS
- **Lucide Icons**: Beautiful, consistent icon system
- **Ribbon Toolbar**: Professional ribbon interface with categorized tools
- **Dark/Light Theme**: Toggle between dark and light modes
- **Wallpaper System**: Customizable background wallpapers
- **Fullscreen Mode**: Distraction-free editing
- **Status Bar**: Document info, cursor position, and settings
- **Keyboard Shortcuts**: Comprehensive shortcut system

### 🔧 Advanced Features
- **Document Statistics**: Word, character, paragraph, line count
- **Reading Time**: Automatic estimation (200 words/minute)
- **Search and Replace**: Full-text search with replace
- **Auto-Save**: Automatic saving to localStorage (toggleable)
- **Recent Files**: Quick access to recently opened files
- **Document Templates**: Pre-built templates for common document types
- **Spell Check**: Built-in spell checking
- **Collaboration**: Real-time collaboration support
- **Voice Recognition**: Speech-to-text input
- **Text-to-Speech**: Audio playback of documents
- **Accessibility**: Screen reader support and keyboard navigation

## Screenshots

*Coming soon - Add screenshots of the application*

## Tech Stack

### Frontend
- **Framework**: Vue 3.5 with Composition API
- **Language**: TypeScript 5.6
- **Build Tool**: Vite 6.0
- **Editor**: TipTap 3.23 (ProseMirror-based)
- **Styling**: Tailwind CSS 4.3
- **Icons**: Lucide Vue Next
- **Spreadsheet**: UniverJS 0.25
- **Charts**: Chart.js 4.5
- **Math**: KaTeX 0.16
- **PDF**: PDF.js 6.0

### Backend
- **Framework**: Tauri 2.0
- **Language**: Rust 1.80+
- **AI Integration**: DeepSeek API (configurable)
- **OCR**: Tesseract
- **Typst**: Rust-powered compilation
- **Services**:
  - AI Service
  - Spreadsheet Service
  - PPT Service
  - PDF Service
  - OCR Service
  - Voice Service
  - Collaboration Service
  - Spell Check Service
  - And 40+ more microservices

### Development Tools
- **Testing**: Vitest, Playwright
- **Linting**: ESLint, Prettier
- **Type Checking**: vue-tsc
- **Package Manager**: Bun (recommended) or npm

## Prerequisites

- **Node.js**: v18 or higher
- **Bun**: Latest version (recommended) or npm/yarn
- **Rust**: 1.80 or higher with Cargo
- **Operating System**: macOS, Windows, or Linux

## Installation

1. **Clone the repository**:
```bash
git clone https://github.com/arkCyber/Logos.git
cd Logos
```

2. **Install frontend dependencies**:
```bash
bun install
# or
npm install
```

3. **Install Rust dependencies** (if not already installed):
```bash
cd src-tauri
cargo build
cd ..
```

4. **Configure environment variables**:
```bash
cp .env.example .env
```

Edit `.env` and add your API keys:
```env
AI_API_KEY=your_actual_api_key_here
```

## Development

Run the development server:

```bash
bun run tauri dev
# or
npm run tauri dev
```

This will:
- Start the Vite dev server on port 1420
- Build and run the Tauri application
- Enable hot-reload for both frontend and backend changes

### Available Scripts

- `bun run dev` - Start Vite dev server
- `bun run build` - Build for production
- `bun run tauri dev` - Start Tauri development mode
- `bun run tauri build` - Build Tauri application
- `bun run test` - Run unit tests
- `bun run test:ui` - Run tests with UI
- `bun run test:e2e` - Run end-to-end tests
- `bun run lint` - Run ESLint
- `bun run lint:fix` - Fix ESLint issues
- `bun run format` - Format code with Prettier
- `bun run type-check` - Run TypeScript type checking
- `bun run audit` - Run full audit (lint + type-check + test)

## Building for Production

Build the application for your platform:

```bash
bun run tauri build
# or
npm run tauri build
```

The built application will be in `src-tauri/target/release/bundle/`.

### Platform-specific builds:

**macOS**:
```bash
bun run tauri build --target universal-apple-darwin
```

**Windows**:
```bash
bun run tauri build --target x86_64-pc-windows-msvc
```

**Linux**:
```bash
bun run tauri build --target x86_64-unknown-linux-gnu
```

## Configuration

### AI API Key

To use AI features, configure your API key in `.env`:

```env
AI_API_KEY=your_actual_api_key_here
```

To use a different AI provider, modify the `AiConfig` in `src-tauri/src/ai_service/mod.rs`.

**Note**: The `.env` file is included in `.gitignore` to prevent committing sensitive API keys.

### Typst Configuration

Typst compilation settings can be configured in `src-tauri/src/typist_service/`.

### Wallpaper Configuration

Wallpapers are stored in `public/` directory. Add new wallpapers by placing image files there and updating the wallpaper list in `src/components/WallpaperSelector.vue`.

## Usage

### Basic Editing

1. **Launch the application** using `bun run tauri dev` or the built executable
2. **Use the toolbar** to format text, insert tables, and more
3. **Resize table columns** by dragging the column borders
4. **Use keyboard shortcuts** for common operations

### AI Features

1. **Select text** in the editor
2. **A bubble menu** will appear above the selection
3. **Click on**:
   - **润色 (Polish)**: Improve text for academic/professional tone
   - **扩写 (Expand)**: Add more details and content
   - **重写 (Rewrite)**: Rephrase while keeping the meaning
   - **总结 (Summarize)**: Get a concise summary
   - **翻译 (Translate)**: Translate to English

### Typst Preview

1. **Click the Typst tab** in the dual-pane interface
2. **Edit in the left pane** (HTML editor)
3. **See real-time preview** in the right pane (Typst rendering)
4. **Export** the compiled result as PNG

### Export Documents

Click the download button in the toolbar to export your document in various formats:
- Markdown (.md)
- HTML (.html)
- RTF (.rtf)
- DOCX (.docx)
- PDF (.pdf)

## Project Structure

```
LOGOS/
├── src/                          # Frontend source code
│   ├── components/               # Vue components
│   │   ├── Editor.vue           # Main editor component
│   │   ├── WallpaperSelector.vue # Wallpaper selector
│   │   ├── editor/              # Editor-specific components
│   │   └── ...
│   ├── composables/             # Vue composables
│   ├── services/                # Frontend services
│   ├── utils/                   # Utility functions
│   ├── types/                   # TypeScript type definitions
│   ├── App.vue                  # Root component
│   ├── main.ts                  # Application entry point
│   └── style.css                # Global styles
├── src-tauri/                   # Rust backend
│   ├── src/
│   │   ├── lib.rs              # Main Rust library
│   │   ├── ai_service/         # AI integration service
│   │   ├── spreadsheet_service/ # Spreadsheet functionality
│   │   ├── ppt_service/        # Presentation module
│   │   ├── pdf_service/        # PDF processing
│   │   ├── ocr_service/        # OCR functionality
│   │   ├── voice_service/      # Voice recognition & TTS
│   │   ├── collaboration_service/ # Real-time collaboration
│   │   ├── spell_check_service/ # Spell checking
│   │   ├── tiptap_service/     # TipTap configuration (437 modules)
│   │   ├── typist_service/     # Typst compilation
│   │   └── ...                 # 40+ more services
│   ├── Cargo.toml              # Rust dependencies
│   └── tauri.conf.json         # Tauri configuration
├── public/                      # Static assets
│   └── wallpapers/             # Wallpaper images
├── assets/                      # Additional assets
├── tests/                       # Test files
├── docs/                        # Documentation
├── package.json                 # Node.js dependencies
├── vite.config.ts              # Vite configuration
├── tsconfig.json               # TypeScript configuration
├── .env.example                # Environment variables template
├── .gitignore                  # Git ignore rules
├── README.md                   # This file
└── README.zh-CN.md             # Chinese documentation
```

## Testing

### Unit Tests

Run unit tests with Vitest:

```bash
bun run test
# or with UI
bun run test:ui
# or with coverage
bun run test:coverage
```

### End-to-End Tests

Run E2E tests with Playwright:

```bash
bun run test:e2e
# or with UI
bun run test:e2e:ui
```

### Type Checking

Run TypeScript type checking:

```bash
bun run type-check
```

### Linting

Run ESLint:

```bash
bun run lint
# or fix issues
bun run lint:fix
```

### Full Audit

Run all checks:

```bash
bun run audit
```

## Architecture

### Frontend Architecture

The frontend follows a component-based architecture with Vue 3 Composition API:

- **Components**: Reusable Vue components for UI elements
- **Composables**: Shared logic using Vue composables
- **Services**: Frontend service layer for API calls
- **State Management**: Reactive state with Vue refs and computed properties
- **Routing**: Component-based navigation (no router needed for single-page app)

### Backend Architecture

The backend is organized as microservices in Rust:

- **Tauri Commands**: Exposed functions for frontend-backend communication
- **Service Modules**: Independent service modules for each feature
- **Error Handling**: Centralized error handling with circuit breakers
- **Configuration**: Service-based configuration management
- **API Integration**: External API clients (AI, OCR, etc.)

### Data Flow

1. **User Action** → Frontend Component
2. **Component** → Tauri Command (if backend needed)
3. **Tauri Command** → Rust Service
4. **Service** → External API / Processing
5. **Response** → Frontend Component
6. **Component** → UI Update

## Roadmap

### Phase 1: Core Features (Completed ✅)
- Rich text editor with TipTap
- AI integration (polish, expand, rewrite)
- Typst integration
- Basic export formats
- Dark/Light theme

### Phase 2: Advanced Features (In Progress 🚧)
- Full spreadsheet module
- Presentation module
- Advanced PDF features
- OCR integration
- Voice recognition
- Real-time collaboration

### Phase 3: Polish & Optimization (Planned 📋)
- Performance optimization
- More AI features
- Additional export formats
- Mobile responsiveness
- Plugin system
- Cloud sync

### Phase 4: Enterprise Features (Future 🔮)
- Team collaboration
- Version control
- Advanced security
- Enterprise integrations
- API for third-party apps

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**
4. **Run tests**: `bun run audit`
5. **Commit your changes**: `git commit -m 'Add amazing feature'`
6. **Push to the branch**: `git push origin feature/amazing-feature`
7. **Open a Pull Request**

### Code Style

- Follow the existing code style
- Use TypeScript for type safety
- Write tests for new features
- Update documentation as needed
- Keep commits clear and concise

### Issue Reporting

When reporting issues, please include:
- OS and version
- Node.js and Rust versions
- Steps to reproduce
- Expected behavior
- Actual behavior
- Screenshots if applicable

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Tauri](https://tauri.app/) - Desktop application framework
- [Vue.js](https://vuejs.org/) - Progressive JavaScript framework
- [TipTap](https://tiptap.dev/) - Rich text editor framework
- [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework
- [UniverJS](https://univer.ai/) - Spreadsheet engine
- [DeepSeek](https://www.deepseek.com/) - AI API provider
- [Typst](https://typst.app/) - Typesetting system

## Support

- 📧 Email: support@arkcyber.com
- 💬 Discord: [Join our Discord server](https://discord.gg/example)
- 🐛 Issues: [Report bugs on GitHub](https://github.com/arkCyber/Logos/issues)
- 📖 Documentation: [Full documentation](https://github.com/arkCyber/Logos/wiki)

---

<div align="center">

**Built with ❤️ by the Logos Team**

[⬆ Back to top](#logos智道办公软件---ai-powered-office-suite)

</div>
