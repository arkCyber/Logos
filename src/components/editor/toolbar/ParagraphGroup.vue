<script setup lang="ts">
import { ref } from 'vue';
import {
  AlignLeft, AlignCenter, AlignRight, AlignJustify,
  List, ListOrdered, IndentDecrease, IndentIncrease,
  Square, Layers, SortAsc, Eye, MoveDiagonal2,
  CheckSquare
} from 'lucide-vue-next';

interface Emits {
  (e: 'set-text-align', alignment: 'left' | 'center' | 'right' | 'justify'): void;
  (e: 'toggle-bullet-list'): void;
  (e: 'toggle-ordered-list'): void;
  (e: 'toggle-task-list'): void;
  (e: 'decrease-indent'): void;
  (e: 'increase-indent'): void;
  (e: 'set-heading', level: 1 | 2 | 3 | 4 | 5 | 6): void;
  (e: 'toggle-blockquote'): void;
  (e: 'toggle-code-block'): void;
  (e: 'insert-horizontal-rule'): void;
  (e: 'clear-formatting'): void;
  (e: 'set-line-spacing', spacing: 1 | 1.15 | 1.5 | 2 | 2.5 | 3): void;
  (e: 'set-paragraph-spacing', before: number, after: number): void;
  (e: 'add-border'): void;
  (e: 'add-shading'): void;
  (e: 'toggle-multilevel-list'): void;
  (e: 'sort-paragraph'): void;
  (e: 'toggle-format-marks'): void;
}

const emit = defineEmits<Emits>();

// Paragraph group expansion state
const isParagraphExpanded = ref(false);

const setTextAlign = (alignment: 'left' | 'center' | 'right' | 'justify') => {
  emit('set-text-align', alignment);
};

const toggleBulletList = () => {
  emit('toggle-bullet-list');
};

const toggleOrderedList = () => {
  emit('toggle-ordered-list');
};

const toggleTaskList = () => {
  emit('toggle-task-list');
};

const decreaseIndent = () => {
  emit('decrease-indent');
};

const increaseIndent = () => {
  emit('increase-indent');
};

const setHeading = (level: 1 | 2 | 3 | 4 | 5 | 6) => {
  emit('set-heading', level);
};

const toggleBlockquote = () => {
  emit('toggle-blockquote');
};

const toggleCodeBlock = () => {
  emit('toggle-code-block');
};

const insertHorizontalRule = () => {
  emit('insert-horizontal-rule');
};

const clearFormatting = () => {
  emit('clear-formatting');
};

const setLineSpacing = (spacing: 1 | 1.15 | 1.5 | 2 | 2.5 | 3) => {
  emit('set-line-spacing', spacing);
};

const setParagraphSpacing = (before: number, after: number) => {
  emit('set-paragraph-spacing', before, after);
};

const addBorder = () => {
  emit('add-border');
};

const addShading = () => {
  emit('add-shading');
};

const toggleMultilevelList = () => {
  emit('toggle-multilevel-list');
};

const sortParagraph = () => {
  emit('sort-paragraph');
};

const toggleFormatMarks = () => {
  emit('toggle-format-marks');
};
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <!-- Row 1: Alignment & Headings -->
      <div class="paragraph-buttons-row">
        <div class="alignment-buttons-compact">
          <button class="ribbon-button-small" title="左对齐" @click="setTextAlign('left')">
            <AlignLeft :size="16" />
          </button>
          <button class="ribbon-button-small" title="居中" @click="setTextAlign('center')">
            <AlignCenter :size="16" />
          </button>
          <button class="ribbon-button-small" title="右对齐" @click="setTextAlign('right')">
            <AlignRight :size="16" />
          </button>
          <button class="ribbon-button-small" title="两端对齐" @click="setTextAlign('justify')">
            <AlignJustify :size="16" />
          </button>
          <button class="ribbon-button style-compact" title="标题4" @click="setHeading(4)">
            <span class="heading-text">H4</span>
          </button>
          <button class="ribbon-button style-compact" title="标题5" @click="setHeading(5)">
            <span class="heading-text">H5</span>
          </button>
          <button class="ribbon-button style-compact" title="标题6" @click="setHeading(6)">
            <span class="heading-text">H6</span>
          </button>
          <button class="ribbon-button style-compact" title="引用块" @click="toggleBlockquote">
            <span class="heading-text">"</span>
          </button>
        </div>
      </div>

      <!-- Row 2: Lists & Indents & More -->
      <div class="paragraph-buttons-row">
        <div class="list-buttons-compact">
          <button class="ribbon-button-small" title="无序列表" @click="toggleBulletList">
            <List :size="16" />
          </button>
          <button class="ribbon-button-small" title="有序列表" @click="toggleOrderedList">
            <ListOrdered :size="16" />
          </button>
          <button class="ribbon-button-small" title="任务列表" @click="toggleTaskList">
            <CheckSquare :size="16" />
          </button>
          <button class="ribbon-button-small" title="减少缩进" @click="decreaseIndent">
            <IndentDecrease :size="16" />
          </button>
          <button class="ribbon-button-small" title="增加缩进" @click="increaseIndent">
            <IndentIncrease :size="16" />
          </button>
          <button class="ribbon-button style-compact" title="代码块" @click="toggleCodeBlock">
            <span class="heading-text">&lt;/&gt;</span>
          </button>
          <button class="ribbon-button style-compact" title="分隔线" @click="insertHorizontalRule">
            <span class="heading-text">—</span>
          </button>
          <button class="ribbon-button style-compact" title="清除格式" @click="clearFormatting">
            <span class="heading-text">A</span>
          </button>
          <button class="ribbon-button style-compact" title="添加边框" @click="addBorder">
            <Square :size="16" />
          </button>
        </div>
      </div>

      <!-- Row 3: Headings & More -->
      <div class="paragraph-buttons-row">
        <div class="font-buttons-compact">
          <button class="ribbon-button style-compact" title="标题1" @click="setHeading(1)">
            <span class="heading-text">H1</span>
          </button>
          <button class="ribbon-button style-compact" title="标题2" @click="setHeading(2)">
            <span class="heading-text">H2</span>
          </button>
          <button class="ribbon-button style-compact" title="标题3" @click="setHeading(3)">
            <span class="heading-text">H3</span>
          </button>
          <button class="ribbon-button style-compact" title="添加底纹" @click="addShading">
            <Layers :size="16" />
          </button>
          <button class="ribbon-button style-compact" title="多级列表" @click="toggleMultilevelList">
            <ListOrdered :size="16" />
          </button>
          <button class="ribbon-button style-compact" title="排序段落" @click="sortParagraph">
            <SortAsc :size="16" />
          </button>
          <button class="ribbon-button style-compact" title="显示格式标记" @click="toggleFormatMarks">
            <Eye :size="16" />
          </button>
        </div>
      </div>
      <!-- Expanded paragraph options -->
      <div v-if="isParagraphExpanded" class="paragraph-expanded">
        <div class="expanded-row">
          <button class="ribbon-button style-compact" title="标题4" @click="setHeading(4)">
            <span class="heading-text">H4</span>
          </button>
          <button class="ribbon-button style-compact" title="标题5" @click="setHeading(5)">
            <span class="heading-text">H5</span>
          </button>
          <button class="ribbon-button style-compact" title="标题6" @click="setHeading(6)">
            <span class="heading-text">H6</span>
          </button>
          <button class="ribbon-button style-compact" title="引用块" @click="toggleBlockquote">
            <span class="heading-text">"</span>
          </button>
          <button class="ribbon-button style-compact" title="代码块" @click="toggleCodeBlock">
            <span class="heading-text">&lt;/&gt;</span>
          </button>
        </div>
        <div class="expanded-row">
          <button class="ribbon-button style-compact" title="分隔线" @click="insertHorizontalRule">
            <span class="heading-text">—</span>
          </button>
          <button class="ribbon-button style-compact" title="清除格式" @click="clearFormatting">
            <span class="heading-text">A</span>
          </button>
        </div>
        <div class="expanded-section">
          <div class="expanded-section-title">行距</div>
          <div class="expanded-row">
            <button class="ribbon-button style-compact" title="1.0" @click="setLineSpacing(1)">
              <span class="heading-text">1.0</span>
            </button>
            <button class="ribbon-button style-compact" title="1.15" @click="setLineSpacing(1.15)">
              <span class="heading-text">1.15</span>
            </button>
            <button class="ribbon-button style-compact" title="1.5" @click="setLineSpacing(1.5)">
              <span class="heading-text">1.5</span>
            </button>
            <button class="ribbon-button style-compact" title="2.0" @click="setLineSpacing(2)">
              <span class="heading-text">2.0</span>
            </button>
            <button class="ribbon-button style-compact" title="2.5" @click="setLineSpacing(2.5)">
              <span class="heading-text">2.5</span>
            </button>
            <button class="ribbon-button style-compact" title="3.0" @click="setLineSpacing(3)">
              <span class="heading-text">3.0</span>
            </button>
          </div>
        </div>
        <div class="expanded-section">
          <div class="expanded-section-title">段落间距</div>
          <div class="expanded-row">
            <button class="ribbon-button style-compact" title="段前0" @click="setParagraphSpacing(0, 0)">
              <span class="heading-text">前0</span>
            </button>
            <button class="ribbon-button style-compact" title="段前6pt" @click="setParagraphSpacing(6, 0)">
              <span class="heading-text">前6</span>
            </button>
            <button class="ribbon-button style-compact" title="段前12pt" @click="setParagraphSpacing(12, 0)">
              <span class="heading-text">前12</span>
            </button>
            <button class="ribbon-button style-compact" title="段后0" @click="setParagraphSpacing(0, 0)">
              <span class="heading-text">后0</span>
            </button>
            <button class="ribbon-button style-compact" title="段后6pt" @click="setParagraphSpacing(0, 6)">
              <span class="heading-text">后6</span>
            </button>
            <button class="ribbon-button style-compact" title="段后12pt" @click="setParagraphSpacing(0, 12)">
              <span class="heading-text">后12</span>
            </button>
          </div>
        </div>
        <div class="expanded-section">
          <div class="expanded-section-title">边框和底纹</div>
          <div class="expanded-row">
            <button class="ribbon-button style-compact" title="添加边框" @click="addBorder">
              <Square :size="16" />
              <span class="heading-text">边框</span>
            </button>
            <button class="ribbon-button style-compact" title="添加底纹" @click="addShading">
              <Layers :size="16" />
              <span class="heading-text">底纹</span>
            </button>
          </div>
        </div>
        <div class="expanded-section">
          <div class="expanded-section-title">其他</div>
          <div class="expanded-row">
            <button class="ribbon-button style-compact" title="多级列表" @click="toggleMultilevelList">
              <ListOrdered :size="16" />
              <span class="heading-text">多级</span>
            </button>
            <button class="ribbon-button style-compact" title="排序段落" @click="sortParagraph">
              <SortAsc :size="16" />
              <span class="heading-text">排序</span>
            </button>
            <button class="ribbon-button style-compact" title="显示格式标记" @click="toggleFormatMarks">
              <Eye :size="16" />
              <span class="heading-text">标记</span>
            </button>
          </div>
        </div>
      </div>
    </div>
    <div class="group-label" @click="isParagraphExpanded = !isParagraphExpanded">
      <span>段落</span>
      <MoveDiagonal2 :size="12" />
    </div>
  </div>
</template>

<style scoped>
.group-content {
  flex-direction: column;
  flex-wrap: nowrap;
  gap: 0;
}

.paragraph-buttons-row {
  display: flex;
  gap: 4px;
  align-items: center;
  justify-content: flex-start;
  width: 100%;
}

.alignment-buttons-compact,
.list-buttons-compact,
.font-buttons-compact {
  display: flex;
  gap: 1px;
}

.expand-button {
  padding: 0 2px;
  min-width: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 24px;
}

.expand-button:hover {
  background: var(--word-button-hover);
}

.paragraph-expanded {
  border-top: 1px solid var(--word-divider);
  padding-top: 8px;
  margin-top: 8px;
  max-height: 400px;
  overflow-y: auto;
  animation: fadeIn 0.2s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.expanded-row {
  display: flex;
  gap: 4px;
  margin-bottom: 4px;
  flex-wrap: wrap;
}

.expanded-row:last-child {
  margin-bottom: 0;
}

.expanded-section {
  margin-bottom: 12px;
}

.expanded-section:last-child {
  margin-bottom: 0;
}

.expanded-section-title {
  font-size: 11px;
  color: var(--word-text-secondary);
  margin-bottom: 6px;
  font-weight: 500;
}

.heading-text {
  font-weight: bold;
  font-size: 12px;
}

.spacing-label {
  font-size: 10px;
  color: var(--word-text-secondary);
  margin-right: 2px;
}

.paragraph-expanded .ribbon-button {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  min-width: 50px;
  min-height: 28px;
  height: 28px;
  justify-content: center;
  transition: background-color 0.15s ease;
}

.paragraph-expanded .ribbon-button svg {
  flex-shrink: 0;
  width: 14px;
  height: 14px;
}

.paragraph-expanded .ribbon-button:hover {
  background: var(--word-button-hover);
}
</style>
