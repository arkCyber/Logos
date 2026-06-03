# Tiptap功能迁移到Rust后端计划

## 已审计的可迁移功能模块

### 1. 搜索替换功能 (Search & Replace)
**当前实现**: 前端JavaScript实现
- `findNext()` - 查找下一个匹配
- `findPrevious()` - 查找上一个匹配
- `replaceCurrent()` - 替换当前匹配
- `replaceAll()` - 替换所有匹配
- 支持正则表达式、大小写敏感、全词匹配

**迁移优势**:
- Rust的正则表达式性能更高
- 大文档搜索更快
- 可以实现更复杂的搜索算法

**Rust服务**: `search_replace_service`
- `find_text(text: String, pattern: String, options: SearchOptions) -> SearchResult`
- `replace_text(text: String, pattern: String, replacement: String, options: ReplaceOptions) -> ReplaceResult`

### 2. 目录生成功能 (Table of Contents)
**当前实现**: 前端JavaScript实现
- `tocGenerator.generateFromHTML()` - 从HTML生成目录
- 解析h1-h6标题
- 生成嵌套目录结构

**迁移优势**:
- Rust的HTML解析更稳定
- 可以支持更复杂的目录格式
- 性能更好

**Rust服务**: `toc_service`
- `generate_toc(html: String) -> TocResult`
- `insert_toc(html: String, toc: TocResult) -> String`

### 3. 页眉页脚功能 (Header & Footer)
**当前实现**: 前端JavaScript实现
- `_applyHeaderFooter()` - 应用页眉页脚
- `removeHeaderFooter()` - 移除页眉页脚
- `_applyPageNumbers()` - 应用页码

**迁移优势**:
- Rust可以更好地处理文档结构
- 支持更复杂的页眉页脚布局
- 与导出功能集成更好

**Rust服务**: `header_footer_service`
- `apply_header_footer(html: String, header: HeaderConfig, footer: FooterConfig) -> String`
- `remove_header_footer(html: String) -> String`
- `apply_page_numbers(html: String, config: PageNumberConfig) -> String`

### 4. 水印功能 (Watermark)
**当前实现**: 前端JavaScript实现
- `applyWatermark()` - 应用水印
- `_removeWatermark()` - 移除水印

**迁移优势**:
- Rust可以生成更复杂的水印
- 支持图片水印
- 与PDF导出集成更好

**Rust服务**: `watermark_service`
- `apply_watermark(html: String, config: WatermarkConfig) -> String`
- `remove_watermark(html: String) -> String`

## 迁移优先级

### 高优先级 (立即迁移)
1. **搜索替换功能** - 使用频繁，性能提升明显
2. **目录生成功能** - 核心功能，稳定性重要

### 中优先级 (后续迁移)
3. **页眉页脚功能** - 重要但使用频率较低
4. **水印功能** - 辅助功能

## 实施步骤

### 阶段1: 搜索替换服务
1. 创建 `src-tauri/src/search_replace_service/` 模块
2. 实现 `SearchOptions`, `SearchResult`, `ReplaceOptions`, `ReplaceResult` 结构
3. 实现搜索和替换逻辑
4. 注册Tauri命令
5. 更新前端 `useHybridServices.ts`
6. 更新 `Editor.vue` 调用
7. 编写单元测试
8. 测试功能

### 阶段2: 目录生成服务
1. 创建 `src-tauri/src/toc_service/` 模块
2. 实现 `TocItem`, `TocResult` 结构
3. 实现目录生成逻辑
4. 注册Tauri命令
5. 更新前端调用
6. 编写单元测试
7. 测试功能

### 阶段3: 页眉页脚服务
1. 创建 `src-tauri/src/header_footer_service/` 模块
2. 实现相关结构
3. 实现页眉页脚逻辑
4. 注册Tauri命令
5. 更新前端调用
6. 编写单元测试
7. 测试功能

### 阶段4: 水印服务
1. 创建 `src-tauri/src/watermark_service/` 模块
2. 实现相关结构
3. 实现水印逻辑
4. 注册Tauri命令
5. 更新前端调用
6. 编写单元测试
7. 测试功能

## 预期收益

### 性能提升
- 搜索替换: 50-70% 性能提升
- 目录生成: 40-60% 性能提升
- 页眉页脚: 30-50% 性能提升
- 水印: 30-50% 性能提升

### 代码质量
- 类型安全
- 更好的错误处理
- 更易于测试
- 更易于维护

### 功能增强
- 支持更复杂的搜索模式
- 支持更多目录格式
- 支持更复杂的页眉页脚布局
- 支持图片水印

## 风险评估

### 低风险
- 搜索替换: 纯计算任务，无副作用
- 目录生成: 只读操作，无副作用

### 中风险
- 页眉页脚: 修改HTML结构，需要仔细测试
- 水印: 修改HTML结构，需要仔细测试

## 回退计划

如果迁移出现问题，可以快速回退到前端实现：
1. 保留前端代码作为后备
2. 使用特性开关控制使用哪个实现
3. 添加错误处理，失败时回退到前端
