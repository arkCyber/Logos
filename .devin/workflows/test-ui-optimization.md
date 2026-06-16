---
description: 测试 UI 优化效果
---

# 测试 UI 优化效果

本工作流程帮助您验证 Word 风格 UI 优化是否成功应用。

## 步骤 1: 启动开发服务器

```bash
cd /Users/arksong/LOGOS
bun run tauri dev
```

## 步骤 2: 检查样式文件

验证所有样式文件是否存在：

```bash
ls -la src/styles/word-*.css
```

应该看到：
- word-colors.css
- word-typography.css
- word-ribbon.css
- word-components.css

## 步骤 3: 验证样式导入

检查 `src/style.css` 是否正确导入：

```bash
head -10 src/style.css
```

应该看到 `@import "./styles/word-*.css"` 语句。

## 步骤 4: 浏览器开发者工具检查

1. 打开应用后，按 F12 打开开发者工具
2. 选择 Elements 标签
3. 检查以下元素的计算样式：

### Quick Access Toolbar
- 选择 `.quick-access-toolbar`
- 验证 `background-color` 是否为 `rgb(243, 243, 243)` (#F3F3F3)
- 验证 `height` 是否为 `32px`

### Ribbon Tabs
- 选择 `.ribbon-tab.active`
- 验证 `border-bottom-color` 是否为 `rgb(0, 120, 212)` (#0078D4)
- 验证 `background-color` 是否为 `rgb(255, 255, 255)` (#FFFFFF)

### Ribbon Buttons
- Hover 一个按钮
- 验证 `background-color` 是否变为 `rgb(204, 228, 247)` (#CCE4F7)

### 字体
- 选择任意 UI 元素
- 验证 `font-family` 包含 "Segoe UI"
- 验证 `font-size` 为 `11px`

## 步骤 5: 交互测试

### 测试 Ribbon 选项卡切换
1. 点击不同的选项卡（开始、插入、布局等）
2. 验证激活状态正确显示（蓝色底边）
3. 验证面板内容正确切换

### 测试按钮 Hover 效果
1. 将鼠标悬停在各种按钮上
2. 验证背景色变为浅蓝色 (#CCE4F7)
3. 验证过渡动画流畅

### 测试深色模式
1. 点击主题切换按钮
2. 验证所有颜色正确反转
3. 验证文字可读性良好

## 步骤 6: 性能检查

在开发者工具中：
1. 打开 Performance 标签
2. 录制页面交互
3. 检查是否有性能警告
4. 验证动画帧率 > 60 FPS

## 步骤 7: 响应式测试

1. 调整窗口大小
2. 验证 Ribbon 可以横向滚动
3. 验证按钮不会被截断
4. 验证状态栏信息正确显示

## 步骤 8: 截图对比

1. 截取当前界面
2. 与 Word 2019/2021 界面对比
3. 检查颜色、间距、字体是否一致

## 常见问题排查

### 样式没有生效？
```bash
# 清除缓存并重启
rm -rf node_modules/.vite
bun run tauri dev
```

### 颜色不对？
```bash
# 检查 CSS 变量定义
grep -n "word-accent" src/styles/word-colors.css
```

### 字体不对？
```bash
# 检查字体定义
grep -n "word-font-ui" src/styles/word-typography.css
```

## 成功标准

✅ Ribbon 背景色为 #F3F3F3  
✅ 按钮 hover 为 #CCE4F7  
✅ 激活选项卡有蓝色底边  
✅ 字体为 Segoe UI  
✅ 所有动画流畅  
✅ 深色模式正常工作  

## 下一步

如果测试通过，可以：
1. 提交代码
2. 进行更详细的功能测试
3. 准备发布

如果测试失败，查看：
- `IMPLEMENTATION_GUIDE.md` - 实施指南
- `UI_AUDIT_REPORT.md` - 审计报告
- `OPTIMIZATION_SUMMARY.md` - 优化总结
