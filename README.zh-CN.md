# Logos智道办公软件 - 集成 Typst 的 AI 智能办公软件

[English](./README.md) | [中文](./README.zh-CN.md)

Logos智道办公软件是一款基于 Tauri、Vue3、TipTap 和 Tailwind CSS 构建的现代化办公软件。它具备 AI 驱动的文本编辑功能，包括润色、扩写和重写功能。**核心创新在于集成了 Typst 编译引擎，实现了实时专业排版预览。**

## 功能特性

- **富文本编辑器**：基于 TipTap 和 ProseMirror 构建，支持：
  - 粗体、斜体、删除线、下划线、代码格式
  - 文本高亮/标记功能
  - 文本转换（大写/小写）
  - 标题（H1、H2、H3）
  - 无序和有序列表
  - 引用块
  - 可调整大小的表格，具备完整编辑功能
  - 水平分割线
  - 图片插入（base64 编码）
  - 链接插入和管理
  - 文本对齐（左对齐、居中、右对齐、两端对齐）
  - 文本缩进控制
  - 可调节字体大小（12px - 32px）
  - 可调节行高（1.0 - 2.5）
  - 带语法高亮的代码块（支持 100+ 种语言）
  - 改进的表格样式，带有圆角和更好的边框
- **AI 集成**：选中文本后可使用 AI 功能：
  - 润色文本以获得学术/专业语调
  - 扩写内容，添加更多细节
  - 重写文本，保持原意不变
  - 简洁总结内容
  - 翻译文本为英文
  - **流式响应**：AI 响应以打字机效果实时显示
- **斜杠命令**：输入 `/` 访问快速命令：
  - 标题（H1、H2、H3、纯文本）
  - 无序和有序列表
  - 引用块
  - 代码块和行内代码
  - 表格（2x2、3x3、4x4）
  - 文本格式（粗体、斜体、删除线）
  - 水平分割线
- **文档统计**：实时显示字数、字符数、段落数和行数
- **阅读时间**：自动估算阅读时间（基于 200 字/分钟）
- **状态栏**：底部状态栏显示文档信息、光标位置和设置
- **查找和替换**：全文搜索和替换功能
- **自动保存**：每 30 秒自动保存到 localStorage（可切换）
- **深色/浅色主题**：在深色和浅色模式之间切换，UI 完全适配
- **全屏模式**：切换全屏编辑模式
- **打印支持**：原生浏览器打印功能
- **清空文档**：带有确认提示的清空文档选项
- **现代 UI**：使用 Tailwind CSS 样式的 Word 风格工具栏
- **键盘快捷键**：
  - Ctrl/Cmd + S：保存文档
  - Ctrl/Cmd + O：打开文档
  - Ctrl/Cmd + N：新建文档
  - Ctrl/Cmd + B：粗体
  - Ctrl/Cmd + I：斜体
  - Ctrl/Cmd + U：下划线
  - Ctrl/Cmd + Z：撤销
  - Ctrl/Cmd + Shift + Z：重做
  - Ctrl/Cmd + Y：重做（备选）
  - Ctrl/Cmd + F：查找
  - Ctrl/Cmd + H：替换
  - Ctrl/Cmd + K：插入链接
  - Ctrl/Cmd + A：全选
- **导出功能**：将文档导出为 Markdown、HTML 和纯文本格式，带有正确的样式
- **文件操作**：使用原生文件对话框保存和加载文档（HTML 格式）
- **最近文件**：快速访问最近打开的文件（最多 10 个文件）
- **文档模板**：常见文档类型的预构建模板（空白、文章、会议记录、学习笔记、项目计划、日记、读书笔记）
- **键盘快捷键帮助**：内置键盘快捷键参考面板
- **Typst 集成**：实时专业排版预览
  - 双窗格界面，实时 Typst 渲染
  - HTML 到 Typst 标记转换
  - Rust 驱动的编译，毫秒级渲染速度
  - 防抖编译以防止性能问题
  - Base64 编码的 PNG 输出，即时预览

## 技术栈

- **前端**：Vue 3 + TypeScript + Vite
- **桌面框架**：Tauri 2
- **编辑器**：TipTap（基于 ProseMirror）
- **样式**：Tailwind CSS 4
- **后端**：Rust，使用 reqwest 进行 API 调用
- **AI 提供商**：DeepSeek API（可配置）

## 环境要求

- Node.js（v18 或更高版本）
- Bun（推荐）或 npm/yarn
- Rust 和 Cargo
- macOS、Windows 或 Linux

## 安装

1. 克隆仓库：
```bash
git clone <repository-url>
cd LOGOS
```

2. 安装依赖：
```bash
bun install
```

3. 安装 Rust 依赖（如果尚未安装）：
```bash
cd src-tauri
cargo build
cd ..
```

## 开发

运行开发服务器：

```bash
bun run tauri dev
```

这将：
- 在端口 1420 上启动 Vite 开发服务器
- 构建并运行 Tauri 应用程序
- 为前端和后端更改启用热重载

## 生产构建

为您的平台构建应用程序：

```bash
bun run tauri build
```

构建的应用程序将位于 `src-tauri/target/release/bundle/` 目录中。

## 配置

### AI API 密钥

要使用 AI 功能，您需要配置您的 API 密钥：

1. 复制 `.env.example` 文件为 `.env`：
```bash
cp .env.example .env
```

2. 在 `.env` 文件中，将 `your_api_key_here` 替换为您的实际 DeepSeek API 密钥：
```env
AI_API_KEY=your_actual_api_key_here
```

3. 要使用不同的 AI 提供商，请修改 `src-tauri/src/ai_service/mod.rs` 中的 `AiConfig` 以匹配您提供商的 API 格式。

## 使用方法

### 基本编辑

- 使用工具栏按钮格式化文本
- 使用表格按钮插入表格
- 通过拖动列边框调整表格列大小

### AI 功能

1. 在编辑器中选择任意文本
2. 选择区域上方会出现气泡菜单
3. 点击：
   - **润色**：改善文本以获得学术/专业语调
   - **扩写**：添加更多细节和内容
   - **重写**：在保持原意的同时重新表述
   - **总结**：简洁总结内容
   - **翻译**：翻译文本为英文

### 导出

点击工具栏中的下载按钮将文档导出为 Markdown。

## 项目结构

```
LOGOS/
├── src/
│   ├── components/
│   │   └── Editor.vue       # 主要 TipTap 编辑器组件
│   ├── App.vue              # 根组件
│   ├── main.ts              # 应用程序入口点
│   └── style.css            # 全局样式
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs           # Rust 后端，包含 AI 集成
│   │   ├── ai_service/      # AI 服务模块
│   │   ├── typist_service/  # Typst 编译服务
│   │   ├── editing_engine_service/  # 编辑引擎服务
│   │   └── tiptap_service/  # TipTap 配置服务
│   └── Cargo.toml           # Rust 依赖
├── package.json             # Node.js 依赖
└── vite.config.ts           # Vite 配置
```

## 推荐的 IDE 设置

- [VS Code](https://code.visualstudio.com/)
- [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 未来增强

- 流式 AI 响应（打字机效果）
- 快速命令的斜杠菜单（类似 Notion）
- Typst 集成用于 PDF 导出
- 实时协作
- 更多 AI 功能（总结、翻译等）

## 许可证

MIT

## 贡献

欢迎贡献！请随时提交 Pull Request。

## 测试

详细的测试指南请参阅 [TESTING.md](./TESTING.md)。
