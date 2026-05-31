<script setup lang="ts">
import { 
  Paintbrush, Clipboard, Scissors, Copy, TrendingUp, Search, FileText, SquareCheck,
  AlignLeft, AlignCenter, AlignRight, AlignJustify,
  List, ListOrdered, IndentDecrease, IndentIncrease,
  Heading1, Heading2, Heading3
} from 'lucide-vue-next';

interface Props {
  activeTab: string;
  fontFamily: string;
  fontSize: number;
}

interface Emits {
  (e: 'set-active-tab', tab: string): void;
  (e: 'toggle-bold'): void;
  (e: 'toggle-italic'): void;
  (e: 'toggle-underline'): void;
  (e: 'toggle-strike'): void;
  (e: 'toggle-subscript'): void;
  (e: 'toggle-superscript'): void;
  (e: 'toggle-highlight'): void;
  (e: 'set-text-color'): void;
  (e: 'set-text-align', alignment: 'left' | 'center' | 'right' | 'justify'): void;
  (e: 'toggle-bullet-list'): void;
  (e: 'toggle-ordered-list'): void;
  (e: 'increase-indent'): void;
  (e: 'decrease-indent'): void;
  (e: 'set-heading', level: 1 | 2 | 3): void;
  (e: 'update-font-family', font: string): void;
  (e: 'update-font-size', size: number): void;
  (e: 'paste'): void;
  (e: 'cut'): void;
  (e: 'copy'): void;
  (e: 'format-painter'): void;
  (e: 'change-styles'): void;
  (e: 'find-text'): void;
  (e: 'replace-text'): void;
  (e: 'select-all'): void;
  (e: 'insert-table'): void;
  (e: 'insert-image'): void;
  (e: 'insert-link'): void;
  (e: 'insert-shape'): void;
  (e: 'insert-icon'): void;
  (e: 'insert-symbol'): void;
  (e: 'insert-page-break'): void;
  (e: 'insert-section-break'): void;
  (e: 'insert-header'): void;
  (e: 'insert-footer'): void;
  (e: 'insert-page-number'): void;
  (e: 'set-orientation', orientation: 'portrait' | 'landscape'): void;
  (e: 'set-columns', count: number): void;
  (e: 'set-margins'): void;
  (e: 'insert-toc'): void;
  (e: 'insert-citation'): void;
  (e: 'insert-bibliography'): void;
  (e: 'insert-footnote'): void;
  (e: 'insert-endnote'): void;
  (e: 'add-cross-reference'): void;
  (e: 'add-comment'): void;
  (e: 'track-changes'): void;
  (e: 'accept-change'): void;
  (e: 'reject-change'): void;
  (e: 'toggle-spell-check'): void;
  (e: 'toggle-fullscreen'): void;
  (e: 'toggle-web-layout'): void;
  (e: 'toggle-print-layout'): void;
  (e: 'toggle-navigation-pane'): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

const tabs = [
  { id: 'home', label: '开始' },
  { id: 'insert', label: '插入' },
  { id: 'layout', label: '布局' },
  { id: 'references', label: '引用' },
  { id: 'review', label: '审阅' },
  { id: 'view', label: '视图' }
];
</script>

<template>
  <div role="toolbar" aria-label="功能区工具栏">
    <!-- Ribbon Tabs -->
    <div class="ribbon-tabs" role="tablist" aria-label="功能区选项卡">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        class="ribbon-tab"
        :class="{ active: activeTab === tab.id }"
        :aria-selected="activeTab === tab.id"
        :aria-controls="`panel-${tab.id}`"
        role="tab"
        tabindex="0"
        @click="emit('set-active-tab', tab.id)"
        @keydown.enter="emit('set-active-tab', tab.id)"
        @keydown.space.prevent="emit('set-active-tab', tab.id)"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- Ribbon Panels -->
    <div class="ribbon-panels" role="tabpanel" :aria-labelledby="`panel-${activeTab}`">
      <!-- Home Tab Panel -->
      <div v-if="activeTab === 'home'" :id="`panel-home`" class="ribbon-panel">
        <!-- Clipboard Group -->
        <div class="ribbon-group" role="group" aria-label="剪贴板">
          <div class="group-content">
            <button class="ribbon-button-large" title="粘贴" aria-label="粘贴" @click="emit('paste')">
              <Clipboard :size="32" />
              <span>粘贴</span>
            </button>
            <div class="font-buttons-compact">
              <button class="ribbon-button-small" title="剪切" aria-label="剪切" @click="emit('cut')">
                <Scissors :size="16" />
              </button>
              <button class="ribbon-button-small" title="复制" aria-label="复制" @click="emit('copy')">
                <Copy :size="16" />
              </button>
              <button class="ribbon-button-small" title="格式刷" aria-label="格式刷" @click="emit('format-painter')">
                <Paintbrush :size="16" />
              </button>
            </div>
          </div>
          <div class="group-label">剪贴板</div>
        </div>

        <!-- Font Group -->
        <div class="ribbon-group">
          <div class="group-content font-group">
            <select 
              class="ribbon-select compact" 
              :value="fontFamily"
              @change="emit('update-font-family', ($event.target as HTMLSelectElement).value)"
            >
              <option value="Calibri, 'Microsoft YaHei', '微软雅黑', 'Segoe UI', sans-serif">Calibri</option>
              <option value="'Microsoft YaHei', '微软雅黑', sans-serif">微软雅黑</option>
              <option value="'SimSun', '宋体', serif">宋体</option>
              <option value="'Arial', sans-serif">Arial</option>
              <option value="'Times New Roman', serif">Times New Roman</option>
            </select>
            <select 
              class="ribbon-select compact" 
              :value="fontSize"
              @change="emit('update-font-size', Number(($event.target as HTMLSelectElement).value))"
            >
              <option :value="11">11</option>
              <option :value="12">12</option>
              <option :value="14">14</option>
              <option :value="16">16</option>
              <option :value="18">18</option>
              <option :value="24">24</option>
              <option :value="28">28</option>
              <option :value="36">36</option>
            </select>
            <div class="font-buttons-compact">
              <button class="ribbon-button-small" title="加粗" @click="emit('toggle-bold')">
                <strong>B</strong>
              </button>
              <button class="ribbon-button-small" title="斜体" @click="emit('toggle-italic')">
                <em>I</em>
              </button>
              <button class="ribbon-button-small" title="下划线" @click="emit('toggle-underline')">
                <u>U</u>
              </button>
              <button class="ribbon-button-small" title="删除线" @click="emit('toggle-strike')">
                <s>S</s>
              </button>
              <button class="ribbon-button-small" title="下标" @click="emit('toggle-subscript')">
                X₂
              </button>
              <button class="ribbon-button-small" title="上标" @click="emit('toggle-superscript')">
                X²
              </button>
              <button class="ribbon-button-small" title="高亮" @click="emit('toggle-highlight')">
                🖊
              </button>
              <button class="ribbon-button-small" title="字体颜色" @click="emit('set-text-color')">
                <span style="color: #dc2626;">A</span>
              </button>
            </div>
          </div>
          <div class="group-label">字体</div>
        </div>

        <!-- Paragraph Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <div class="alignment-buttons-compact">
              <button class="ribbon-button-small" title="左对齐" @click="emit('set-text-align', 'left')">
                <AlignLeft :size="16" />
              </button>
              <button class="ribbon-button-small" title="居中" @click="emit('set-text-align', 'center')">
                <AlignCenter :size="16" />
              </button>
              <button class="ribbon-button-small" title="右对齐" @click="emit('set-text-align', 'right')">
                <AlignRight :size="16" />
              </button>
              <button class="ribbon-button-small" title="两端对齐" @click="emit('set-text-align', 'justify')">
                <AlignJustify :size="16" />
              </button>
            </div>
            <div class="list-buttons-compact">
              <button class="ribbon-button-small" title="无序列表" @click="emit('toggle-bullet-list')">
                <List :size="16" />
              </button>
              <button class="ribbon-button-small" title="有序列表" @click="emit('toggle-ordered-list')">
                <ListOrdered :size="16" />
              </button>
              <button class="ribbon-button-small" title="减少缩进" @click="emit('decrease-indent')">
                <IndentDecrease :size="16" />
              </button>
              <button class="ribbon-button-small" title="增加缩进" @click="emit('increase-indent')">
                <IndentIncrease :size="16" />
              </button>
            </div>
            <div class="font-buttons-compact">
              <button class="ribbon-button style-compact" title="标题1" @click="emit('set-heading', 1)">
                <Heading1 :size="16" />
              </button>
              <button class="ribbon-button style-compact" title="标题2" @click="emit('set-heading', 2)">
                <Heading2 :size="16" />
              </button>
              <button class="ribbon-button style-compact" title="标题3" @click="emit('set-heading', 3)">
                <Heading3 :size="16" />
              </button>
            </div>
          </div>
          <div class="group-label">段落</div>
        </div>

        <!-- Styles Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <select class="ribbon-select compact">
              <option>正常</option>
              <option>无间距</option>
              <option>标题1</option>
              <option>标题2</option>
              <option>引用</option>
            </select>
            <button class="ribbon-button" title="更改样式" @click="emit('change-styles')">
              <TrendingUp :size="20" />
              <span>更改样式</span>
            </button>
          </div>
          <div class="group-label">样式</div>
        </div>

        <!-- Editing Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="查找" @click="emit('find-text')">
              <Search :size="20" />
              <span>查找</span>
            </button>
            <button class="ribbon-button" title="替换" @click="emit('replace-text')">
              <FileText :size="20" />
              <span>替换</span>
            </button>
            <button class="ribbon-button" title="全选" @click="emit('select-all')">
              <SquareCheck :size="20" />
              <span>全选</span>
            </button>
          </div>
          <div class="group-label">编辑</div>
        </div>
      </div>

      <!-- Insert Tab Panel -->
      <div v-if="activeTab === 'insert'" class="ribbon-panel">
        <!-- Pages Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="插入空白页" aria-label="插入空白页" @click="emit('insert-page-break')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                <polyline points="14 2 14 8 20 8" />
                <line x1="12" y1="18" x2="12" y2="12" />
                <line x1="9" y1="15" x2="15" y2="15" />
              </svg>
              <span>空白页</span>
            </button>
            <button class="ribbon-button" title="插入分页符" aria-label="插入分页符" @click="emit('insert-page-break')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="5" y1="12" x2="19" y2="12" />
                <polyline points="12 5 19 12 12 19" />
              </svg>
              <span>分页符</span>
            </button>
          </div>
          <div class="group-label">页面</div>
        </div>

        <!-- Tables Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="插入表格" aria-label="插入表格" @click="emit('insert-table')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                <line x1="3" y1="9" x2="21" y2="9" />
                <line x1="3" y1="15" x2="21" y2="15" />
                <line x1="9" y1="3" x2="9" y2="21" />
                <line x1="15" y1="3" x2="15" y2="21" />
              </svg>
              <span>表格</span>
            </button>
          </div>
          <div class="group-label">表格</div>
        </div>

        <!-- Illustrations Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="插入图片" aria-label="插入图片" @click="emit('insert-image')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                <circle cx="8.5" cy="8.5" r="1.5" />
                <polyline points="21 15 16 10 5 21" />
              </svg>
              <span>图片</span>
            </button>
            <button class="ribbon-button" title="插入形状" aria-label="插入形状" @click="emit('insert-shape')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
              </svg>
              <span>形状</span>
            </button>
            <button class="ribbon-button" title="插入图标" aria-label="插入图标" @click="emit('insert-icon')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10" />
                <path d="M12 8v8" />
                <path d="M8 12h8" />
              </svg>
              <span>图标</span>
            </button>
          </div>
          <div class="group-label">插图</div>
        </div>

        <!-- Links Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="插入链接" aria-label="插入链接" @click="emit('insert-link')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
                <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
              </svg>
              <span>链接</span>
            </button>
          </div>
          <div class="group-label">链接</div>
        </div>

        <!-- Symbols Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="插入符号" aria-label="插入符号" @click="emit('insert-symbol')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M6 9l6 6 6-6" />
              </svg>
              <span>符号</span>
            </button>
          </div>
          <div class="group-label">符号</div>
        </div>
      </div>

      <!-- Layout Tab Panel -->
      <div v-if="activeTab === 'layout'" class="ribbon-panel">
        <!-- Page Setup Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="纵向" aria-label="纵向" @click="emit('set-orientation', 'portrait')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="4" y="2" width="16" height="20" rx="2" />
              </svg>
              <span>纵向</span>
            </button>
            <button class="ribbon-button" title="横向" aria-label="横向" @click="emit('set-orientation', 'landscape')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="2" y="4" width="20" height="16" rx="2" />
              </svg>
              <span>横向</span>
            </button>
          </div>
          <div class="group-label">页面设置</div>
        </div>

        <!-- Columns Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="一栏" aria-label="一栏" @click="emit('set-columns', 1)">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="3" y="3" width="18" height="18" rx="2" />
              </svg>
              <span>一栏</span>
            </button>
            <button class="ribbon-button" title="两栏" aria-label="两栏" @click="emit('set-columns', 2)">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="3" y="3" width="18" height="18" rx="2" />
                <line x1="12" y1="3" x2="12" y2="21" />
              </svg>
              <span>两栏</span>
            </button>
            <button class="ribbon-button" title="三栏" aria-label="三栏" @click="emit('set-columns', 3)">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="3" y="3" width="18" height="18" rx="2" />
                <line x1="8" y1="3" x2="8" y2="21" />
                <line x1="16" y1="3" x2="16" y2="21" />
              </svg>
              <span>三栏</span>
            </button>
          </div>
          <div class="group-label">分栏</div>
        </div>

        <!-- Margins Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="页边距" aria-label="页边距" @click="emit('set-margins')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="3" y="3" width="18" height="18" rx="2" />
                <rect x="5" y="5" width="14" height="14" rx="1" />
              </svg>
              <span>页边距</span>
            </button>
          </div>
          <div class="group-label">页边距</div>
        </div>
      </div>

      <!-- References Tab Panel -->
      <div v-if="activeTab === 'references'" class="ribbon-panel">
        <!-- Table of Contents Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="目录" aria-label="目录" @click="emit('insert-toc')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="4" y1="6" x2="20" y2="6" />
                <line x1="4" y1="12" x2="20" y2="12" />
                <line x1="4" y1="18" x2="20" y2="18" />
              </svg>
              <span>目录</span>
            </button>
          </div>
          <div class="group-label">目录</div>
        </div>

        <!-- Citations Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="插入引用" aria-label="插入引用" @click="emit('insert-citation')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
                <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" />
              </svg>
              <span>引用</span>
            </button>
            <button class="ribbon-button" title="参考文献" aria-label="参考文献" @click="emit('insert-bibliography')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
                <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" />
                <line x1="12" y1="6" x2="12" y2="18" />
              </svg>
              <span>参考文献</span>
            </button>
          </div>
          <div class="group-label">引用</div>
        </div>

        <!-- Footnotes Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="插入脚注" aria-label="插入脚注" @click="emit('insert-footnote')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 19V5" />
                <path d="M12 19a3 3 0 0 1-3-3" />
                <path d="M12 19a3 3 0 0 0 3-3" />
              </svg>
              <span>脚注</span>
            </button>
            <button class="ribbon-button" title="插入尾注" aria-label="插入尾注" @click="emit('insert-endnote')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 5v14" />
                <path d="M12 5a3 3 0 0 0-3 3" />
                <path d="M12 5a3 3 0 0 1 3 3" />
              </svg>
              <span>尾注</span>
            </button>
          </div>
          <div class="group-label">脚注</div>
        </div>

        <!-- Cross Reference Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="交叉引用" aria-label="交叉引用" @click="emit('add-cross-reference')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
                <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
              </svg>
              <span>交叉引用</span>
            </button>
          </div>
          <div class="group-label">交叉引用</div>
        </div>
      </div>

      <!-- Review Tab Panel -->
      <div v-if="activeTab === 'review'" class="ribbon-panel">
        <!-- Comments Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="新建批注" aria-label="新建批注" @click="emit('add-comment')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
              </svg>
              <span>新建批注</span>
            </button>
          </div>
          <div class="group-label">批注</div>
        </div>

        <!-- Changes Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="修订" aria-label="修订" @click="emit('track-changes')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
                <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
              </svg>
              <span>修订</span>
            </button>
            <button class="ribbon-button" title="接受" aria-label="接受" @click="emit('accept-change')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12" />
              </svg>
              <span>接受</span>
            </button>
            <button class="ribbon-button" title="拒绝" aria-label="拒绝" @click="emit('reject-change')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18" />
                <line x1="6" y1="6" x2="18" y2="18" />
              </svg>
              <span>拒绝</span>
            </button>
          </div>
          <div class="group-label">更改</div>
        </div>

        <!-- Proofing Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="拼写检查" aria-label="拼写检查" @click="emit('toggle-spell-check')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 2l-9 20h18L12 2z" />
                <path d="M12 2v20" />
              </svg>
              <span>拼写检查</span>
            </button>
          </div>
          <div class="group-label">校对</div>
        </div>
      </div>

      <!-- View Tab Panel -->
      <div v-if="activeTab === 'view'" class="ribbon-panel">
        <!-- Views Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="页面视图" aria-label="页面视图" @click="emit('toggle-print-layout')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="3" y="3" width="18" height="18" rx="2" />
              </svg>
              <span>页面视图</span>
            </button>
            <button class="ribbon-button" title="Web版式" aria-label="Web版式" @click="emit('toggle-web-layout')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10" />
                <line x1="2" y1="12" x2="22" y2="12" />
                <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" />
              </svg>
              <span>Web版式</span>
            </button>
          </div>
          <div class="group-label">视图</div>
        </div>

        <!-- Show Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="导航窗格" aria-label="导航窗格" @click="emit('toggle-navigation-pane')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="3" y="3" width="18" height="18" rx="2" />
                <line x1="9" y1="3" x2="9" y2="21" />
              </svg>
              <span>导航窗格</span>
            </button>
          </div>
          <div class="group-label">显示</div>
        </div>

        <!-- Zoom Group -->
        <div class="ribbon-group">
          <div class="group-content">
            <button class="ribbon-button" title="全屏" aria-label="全屏" @click="emit('toggle-fullscreen')">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3" />
              </svg>
              <span>全屏</span>
            </button>
          </div>
          <div class="group-label">缩放</div>
        </div>
      </div>

      <!-- Other tabs placeholder - will be implemented in subsequent iterations -->
      <div v-else class="ribbon-panel">
        <div class="ribbon-group">
          <div class="group-content">
            <p style="padding: 20px; color: var(--word-text-secondary);" aria-live="polite">
              {{ tabs.find(t => t.id === activeTab)?.label }} 标签页内容待实现
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
