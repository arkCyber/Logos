# Tauri 纸张效果故障排除指南

## 问题描述
网页预览中看到了 Word 风格的纸张效果（灰色背景 + 白色 A4 纸），但在 Tauri 应用中没有显示。

## 诊断步骤

### 1. 打开 Tauri 开发者工具
在 Tauri 应用窗口中：
- **macOS**: 右键点击 → "Inspect Element" 或按 `Cmd + Option + I`
- **Windows/Linux**: 右键点击 → "Inspect" 或按 `F12`

### 2. 检查控制台输出
在 Console 标签页中，查找以下调试信息：

```
[Editor] DOM Structure Check:
  - editor-content-wrapper exists: true/false
  - document-canvas exists: true/false
  - editor-mount exists: true/false
  - wrapper background: rgb(165, 165, 165)  ← 应该是灰色
  - wrapper display: flex
  - mount width: 794px  ← 应该是 A4 宽度
  - mount background: rgb(255, 255, 255)  ← 应该是白色
  - mount box-shadow: ...  ← 应该有阴影
  - mount padding: 96px 120px  ← 应该有边距
```

### 3. 检查 DOM 结构
在 Elements 标签页中，查找以下结构：

```html
<div class="editor-container">
  <div class="quick-access-toolbar">...</div>
  <div class="ribbon-tabs">...</div>
  <div class="ribbon-panels">...</div>
  
  <div class="editor-content-wrapper">  ← 灰色背景容器
    <div class="document-canvas">  ← 居中容器
      <div class="editor-mount">  ← A4 白纸
        <div class="ProseMirror">...</div>
      </div>
    </div>
  </div>
</div>
```

### 4. 检查 CSS 样式
选中 `.editor-content-wrapper` 元素，在 Styles 面板中检查：

**应该看到的样式：**
```css
.editor-content-wrapper {
  flex: 1;
  overflow-y: auto;
  overflow-x: auto;
  background: #a5a5a5;  ← 灰色
  padding: 0;
  display: flex;
  flex-direction: column;
  align-items: stretch;
  position: relative;
}
```

选中 `.editor-mount` 元素，检查：

```css
.editor-mount {
  width: 794px;  ← A4 宽度
  min-height: 1123px;  ← A4 高度
  background: #ffffff;  ← 白色
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.22), 0 4px 20px rgba(0, 0, 0, 0.16);
  margin-bottom: 40px;
  padding: 96px 120px;  ← 页边距
  position: relative;
  flex-shrink: 0;
  box-sizing: border-box;
}
```

## 常见问题及解决方案

### 问题 1: 看不到灰色背景

**可能原因：**
- CSS 未正确加载
- 其他样式覆盖了背景色

**解决方案：**
1. 在开发者工具中检查 `.editor-content-wrapper` 的 `background` 属性
2. 如果是 `transparent` 或其他颜色，检查是否有其他 CSS 规则覆盖
3. 尝试在 Styles 面板中手动添加 `background: #a5a5a5 !important;` 测试

### 问题 2: 看不到白色纸张

**可能原因：**
- `.editor-mount` 元素不存在
- 宽度或高度为 0
- 被其他元素遮挡

**解决方案：**
1. 在 Elements 面板中搜索 `editor-mount`
2. 检查元素的 Computed 样式，确认 width 和 height
3. 检查 `display` 属性不是 `none`
4. 检查父元素 `.document-canvas` 的 `align-items: center`

### 问题 3: 纸张不居中

**可能原因：**
- `.document-canvas` 的 flex 布局未生效
- 父容器宽度不足

**解决方案：**
1. 检查 `.document-canvas` 的 `display: flex` 和 `align-items: center`
2. 检查 `.editor-content-wrapper` 的 `display: flex`
3. 确认窗口宽度大于 914px (794px 纸张 + 120px 边距)

### 问题 4: 看不到阴影

**可能原因：**
- `box-shadow` 属性未应用
- 阴影被背景色吞没

**解决方案：**
1. 检查 `.editor-mount` 的 `box-shadow` 属性
2. 确认背景是灰色 (#a5a5a5)，不是白色
3. 尝试增加阴影强度测试：`box-shadow: 0 0 20px rgba(0,0,0,0.5) !important;`

### 问题 5: 热重载后样式消失

**可能原因：**
- Vite HMR 问题
- 样式作用域问题

**解决方案：**
1. 完全刷新页面（Cmd+R 或 F5）
2. 重启 Tauri 开发服务器：
   ```bash
   # 停止当前服务器
   pkill -9 -f "tauri dev"
   
   # 重新启动
   bun tauri dev
   ```

## 手动验证步骤

### 步骤 1: 检查元素是否存在
在 Console 中运行：
```javascript
console.log('wrapper:', document.querySelector('.editor-content-wrapper'));
console.log('canvas:', document.querySelector('.document-canvas'));
console.log('mount:', document.querySelector('.editor-mount'));
```

所有三个都应该返回 DOM 元素，而不是 `null`。

### 步骤 2: 检查计算样式
```javascript
const wrapper = document.querySelector('.editor-content-wrapper');
const styles = window.getComputedStyle(wrapper);
console.log('background:', styles.backgroundColor);
console.log('display:', styles.display);
```

应该看到：
- `background: rgb(165, 165, 165)` 或 `rgba(165, 165, 165, 1)`
- `display: flex`

### 步骤 3: 强制应用样式
如果样式未应用，尝试在 Console 中强制设置：
```javascript
const wrapper = document.querySelector('.editor-content-wrapper');
wrapper.style.background = '#a5a5a5';
wrapper.style.display = 'flex';
wrapper.style.flexDirection = 'column';

const mount = document.querySelector('.editor-mount');
mount.style.width = '794px';
mount.style.background = '#ffffff';
mount.style.boxShadow = '0 1px 3px rgba(0,0,0,0.22), 0 4px 20px rgba(0,0,0,0.16)';
mount.style.padding = '96px 120px';
```

如果这样能看到效果，说明 CSS 文件未正确加载。

## CSS 加载检查

### 检查 style.css 是否加载
在 Network 标签页中，刷新页面，查找：
- `style.css`
- `Editor.vue` (包含 scoped styles)

确认状态码是 200，不是 404。

### 检查 CSS 内容
在 Sources 标签页中，找到 `Editor.vue`，搜索 `.editor-content-wrapper`，确认样式代码存在。

## 对比网页和 Tauri

### 网页预览 (http://localhost:1425)
1. 打开浏览器开发者工具
2. 检查 `.editor-content-wrapper` 的样式
3. 截图保存

### Tauri 应用
1. 打开 Tauri 开发者工具
2. 检查相同元素的样式
3. 对比两者差异

如果网页中有样式但 Tauri 中没有，可能是：
- Tauri 的 CSP (Content Security Policy) 阻止了某些样式
- Tauri 的 WebView 版本不支持某些 CSS 特性

## 最终解决方案

如果以上都无效，尝试：

### 1. 清理并重新构建
```bash
# 清理缓存
rm -rf node_modules/.vite
rm -rf dist

# 重新安装依赖
bun install

# 重启开发服务器
bun tauri dev
```

### 2. 检查 Tauri 配置
确认 `src-tauri/tauri.conf.json` 中没有禁用样式：
```json
{
  "app": {
    "security": {
      "csp": null  ← 应该是 null 或允许 style-src
    }
  }
}
```

### 3. 使用内联样式测试
临时在 `Editor.vue` 模板中添加内联样式：
```vue
<div 
  class="editor-content-wrapper"
  style="background: #a5a5a5; display: flex; flex-direction: column;"
>
  <div 
    class="document-canvas"
    style="display: flex; flex-direction: column; align-items: center; padding: 40px 60px;"
  >
    <div 
      class="editor-mount"
      style="width: 794px; min-height: 1123px; background: white; box-shadow: 0 4px 20px rgba(0,0,0,0.2); padding: 96px 120px;"
    >
      <EditorContent :editor="editor" />
    </div>
  </div>
</div>
```

如果内联样式有效，说明 scoped CSS 有问题。

## 联系支持

如果问题仍未解决，请提供：
1. Tauri 开发者工具的 Console 截图
2. Elements 面板中 `.editor-content-wrapper` 的 HTML 结构
3. Styles 面板中 `.editor-content-wrapper` 和 `.editor-mount` 的完整样式
4. 你的操作系统和 Tauri 版本信息

运行以下命令获取版本信息：
```bash
bun tauri info
```
