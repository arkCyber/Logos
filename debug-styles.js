// Tauri 样式调试脚本
// 在 Tauri 开发者工具的 Console 中粘贴并运行此脚本

console.log('='.repeat(60));
console.log('LOGOS Word 纸张效果诊断工具');
console.log('='.repeat(60));

// 1. 检查 DOM 元素
console.log('\n1. DOM 元素检查:');
const wrapper = document.querySelector('.editor-content-wrapper');
const canvas = document.querySelector('.document-canvas');
const mount = document.querySelector('.editor-mount');
const prosemirror = document.querySelector('.ProseMirror');

console.log('  ✓ editor-content-wrapper:', wrapper ? '存在' : '❌ 不存在');
console.log('  ✓ document-canvas:', canvas ? '存在' : '❌ 不存在');
console.log('  ✓ editor-mount:', mount ? '存在' : '❌ 不存在');
console.log('  ✓ ProseMirror:', prosemirror ? '存在' : '❌ 不存在');

// 2. 检查 wrapper 样式
if (wrapper) {
  console.log('\n2. editor-content-wrapper 样式:');
  const wrapperStyles = window.getComputedStyle(wrapper);
  const bgColor = wrapperStyles.backgroundColor;
  const expectedBg = 'rgb(165, 165, 165)';
  
  console.log('  - background:', bgColor, bgColor === expectedBg ? '✅' : '❌ 应该是 ' + expectedBg);
  console.log('  - display:', wrapperStyles.display, wrapperStyles.display === 'flex' ? '✅' : '❌');
  console.log('  - flex-direction:', wrapperStyles.flexDirection);
  console.log('  - overflow-y:', wrapperStyles.overflowY);
  console.log('  - overflow-x:', wrapperStyles.overflowX);
} else {
  console.log('\n2. ❌ editor-content-wrapper 不存在，无法检查样式');
}

// 3. 检查 canvas 样式
if (canvas) {
  console.log('\n3. document-canvas 样式:');
  const canvasStyles = window.getComputedStyle(canvas);
  
  console.log('  - display:', canvasStyles.display, canvasStyles.display === 'flex' ? '✅' : '❌');
  console.log('  - flex-direction:', canvasStyles.flexDirection);
  console.log('  - align-items:', canvasStyles.alignItems, canvasStyles.alignItems === 'center' ? '✅' : '❌');
  console.log('  - padding:', canvasStyles.padding);
} else {
  console.log('\n3. ❌ document-canvas 不存在，无法检查样式');
}

// 4. 检查 mount 样式
if (mount) {
  console.log('\n4. editor-mount (A4 纸张) 样式:');
  const mountStyles = window.getComputedStyle(mount);
  const width = mountStyles.width;
  const bgColor = mountStyles.backgroundColor;
  const expectedWidth = '794px';
  const expectedBg = 'rgb(255, 255, 255)';
  
  console.log('  - width:', width, width === expectedWidth ? '✅' : '❌ 应该是 ' + expectedWidth);
  console.log('  - min-height:', mountStyles.minHeight);
  console.log('  - background:', bgColor, bgColor === expectedBg ? '✅' : '❌ 应该是白色');
  console.log('  - box-shadow:', mountStyles.boxShadow ? '✅ 有阴影' : '❌ 无阴影');
  console.log('  - padding:', mountStyles.padding);
  console.log('  - margin-bottom:', mountStyles.marginBottom);
  console.log('  - position:', mountStyles.position);
  console.log('  - flex-shrink:', mountStyles.flexShrink);
} else {
  console.log('\n4. ❌ editor-mount 不存在，无法检查样式');
}

// 5. 检查 ProseMirror 样式
if (prosemirror) {
  console.log('\n5. ProseMirror (编辑器) 样式:');
  const pmStyles = window.getComputedStyle(prosemirror);
  
  console.log('  - font-family:', pmStyles.fontFamily);
  console.log('  - font-size:', pmStyles.fontSize);
  console.log('  - line-height:', pmStyles.lineHeight);
  console.log('  - color:', pmStyles.color);
  console.log('  - min-height:', pmStyles.minHeight);
} else {
  console.log('\n5. ❌ ProseMirror 不存在，无法检查样式');
}

// 6. 视觉检查
console.log('\n6. 视觉效果检查:');
if (wrapper && mount) {
  const wrapperRect = wrapper.getBoundingClientRect();
  const mountRect = mount.getBoundingClientRect();
  
  console.log('  - wrapper 可见:', wrapperRect.width > 0 && wrapperRect.height > 0 ? '✅' : '❌');
  console.log('  - mount 可见:', mountRect.width > 0 && mountRect.height > 0 ? '✅' : '❌');
  console.log('  - wrapper 尺寸:', Math.round(wrapperRect.width) + 'x' + Math.round(wrapperRect.height));
  console.log('  - mount 尺寸:', Math.round(mountRect.width) + 'x' + Math.round(mountRect.height));
  
  // 检查居中
  const wrapperCenterX = wrapperRect.left + wrapperRect.width / 2;
  const mountCenterX = mountRect.left + mountRect.width / 2;
  const isCentered = Math.abs(wrapperCenterX - mountCenterX) < 50;
  console.log('  - 纸张居中:', isCentered ? '✅' : '❌ 偏移 ' + Math.round(Math.abs(wrapperCenterX - mountCenterX)) + 'px');
}

// 7. 诊断建议
console.log('\n7. 诊断结果:');
let issues = [];

if (!wrapper) issues.push('editor-content-wrapper 元素不存在');
if (!canvas) issues.push('document-canvas 元素不存在');
if (!mount) issues.push('editor-mount 元素不存在');

if (wrapper) {
  const wrapperStyles = window.getComputedStyle(wrapper);
  if (wrapperStyles.backgroundColor !== 'rgb(165, 165, 165)') {
    issues.push('灰色背景未应用 (当前: ' + wrapperStyles.backgroundColor + ')');
  }
  if (wrapperStyles.display !== 'flex') {
    issues.push('wrapper display 不是 flex (当前: ' + wrapperStyles.display + ')');
  }
}

if (mount) {
  const mountStyles = window.getComputedStyle(mount);
  if (mountStyles.width !== '794px') {
    issues.push('A4 纸张宽度不正确 (当前: ' + mountStyles.width + ')');
  }
  if (mountStyles.backgroundColor !== 'rgb(255, 255, 255)') {
    issues.push('纸张背景不是白色 (当前: ' + mountStyles.backgroundColor + ')');
  }
  if (!mountStyles.boxShadow || mountStyles.boxShadow === 'none') {
    issues.push('纸张阴影未应用');
  }
}

if (issues.length === 0) {
  console.log('  ✅ 所有检查通过！Word 纸张效果应该正常显示。');
  console.log('  如果你看不到效果，请检查:');
  console.log('    1. 窗口是否足够大 (至少 914px 宽)');
  console.log('    2. 是否有其他元素遮挡');
  console.log('    3. 尝试滚动页面');
} else {
  console.log('  ❌ 发现以下问题:');
  issues.forEach((issue, i) => {
    console.log('    ' + (i + 1) + '. ' + issue);
  });
  console.log('\n  建议操作:');
  console.log('    1. 刷新页面 (Cmd+R 或 F5)');
  console.log('    2. 检查 CSS 文件是否正确加载');
  console.log('    3. 查看 TAURI_PAPER_TROUBLESHOOTING.md 获取详细帮助');
}

// 8. 快速修复
console.log('\n8. 快速修复选项:');
console.log('  如果样式未应用，运行以下命令强制应用:');
console.log('  > applyStyles()');

window.applyStyles = function() {
  console.log('正在强制应用样式...');
  
  if (wrapper) {
    wrapper.style.background = '#a5a5a5';
    wrapper.style.display = 'flex';
    wrapper.style.flexDirection = 'column';
    wrapper.style.alignItems = 'stretch';
    wrapper.style.overflowY = 'auto';
    wrapper.style.overflowX = 'auto';
    wrapper.style.padding = '0';
    console.log('  ✓ wrapper 样式已应用');
  }
  
  if (canvas) {
    canvas.style.display = 'flex';
    canvas.style.flexDirection = 'column';
    canvas.style.alignItems = 'center';
    canvas.style.padding = '40px 60px';
    canvas.style.flex = '1';
    console.log('  ✓ canvas 样式已应用');
  }
  
  if (mount) {
    mount.style.width = '794px';
    mount.style.minHeight = '1123px';
    mount.style.background = '#ffffff';
    mount.style.boxShadow = '0 1px 3px rgba(0, 0, 0, 0.22), 0 4px 20px rgba(0, 0, 0, 0.16)';
    mount.style.marginBottom = '40px';
    mount.style.padding = '96px 120px';
    mount.style.position = 'relative';
    mount.style.flexShrink = '0';
    mount.style.boxSizing = 'border-box';
    console.log('  ✓ mount 样式已应用');
  }
  
  console.log('✅ 样式强制应用完成！检查效果。');
};

console.log('\n' + '='.repeat(60));
console.log('诊断完成。如需帮助，请查看 TAURI_PAPER_TROUBLESHOOTING.md');
console.log('='.repeat(60) + '\n');
