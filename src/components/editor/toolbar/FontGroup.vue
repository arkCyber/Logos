<script setup lang="ts">
import { ref } from 'vue';
import { Eraser, Sparkles, Type, Languages, Circle, RotateCw, Minus, Square, Palette, Minimize, Text, ZoomIn, ArrowUp, MoveDiagonal2 } from 'lucide-vue-next';

interface Props {
  fontFamily: string;
  fontSize: number;
}

interface Emits {
  (e: 'update:font-family', font: string): void;
  (e: 'update:font-size', size: number): void;
  (e: 'toggle-bold'): void;
  (e: 'toggle-italic'): void;
  (e: 'toggle-underline'): void;
  (e: 'toggle-strike'): void;
  (e: 'toggle-subscript'): void;
  (e: 'toggle-superscript'): void;
  (e: 'toggle-highlight', color?: string): void;
  (e: 'set-text-color'): void;
  (e: 'clear-formatting'): void;
  (e: 'text-effects'): void;
  (e: 'change-case'): void;
  (e: 'pinyin-guide'): void;
  (e: 'enclosed-characters'): void;
  (e: 'vertical-text'): void;
  (e: 'double-strikethrough'): void;
  (e: 'full-half-width'): void;
  (e: 'text-border'): void;
  (e: 'text-shading'): void;
  (e: 'character-spacing'): void;
  (e: 'drop-cap'): void;
  (e: 'character-scale'): void;
  (e: 'small-caps'): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const isFontExpanded = ref(false);

const toggleBold = () => {
  emit('toggle-bold');
};

const toggleItalic = () => {
  emit('toggle-italic');
};

const toggleUnderline = () => {
  emit('toggle-underline');
};

const toggleStrike = () => {
  emit('toggle-strike');
};

const toggleSubscript = () => {
  emit('toggle-subscript');
};

const toggleSuperscript = () => {
  emit('toggle-superscript');
};

const toggleHighlight = (color?: string) => {
  emit('toggle-highlight', color);
};

const setTextColor = () => {
  emit('set-text-color');
};

const clearFormatting = () => {
  emit('clear-formatting');
};

const textEffects = () => {
  emit('text-effects');
};

const changeCase = () => {
  emit('change-case');
};

const pinyinGuide = () => {
  emit('pinyin-guide');
};

const enclosedCharacters = () => {
  emit('enclosed-characters');
};

const verticalText = () => {
  emit('vertical-text');
};

const doubleStrikethrough = () => {
  emit('double-strikethrough');
};

const fullHalfWidth = () => {
  emit('full-half-width');
};

const textBorder = () => {
  emit('text-border');
};

const textShading = () => {
  emit('text-shading');
};

const characterSpacing = () => {
  emit('character-spacing');
};

const dropCap = () => {
  emit('drop-cap');
};

const characterScale = () => {
  emit('character-scale');
};

const smallCaps = () => {
  emit('small-caps');
};
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content font-group">
      <!-- Row 1: Font Selects & Text Effects -->
      <div class="font-row">
        <select 
          :value="fontFamily" 
          class="ribbon-select compact"
          @input="emit('update:font-family', ($event.target as HTMLSelectElement).value)"
        >
          <option value="Calibri, 'Microsoft YaHei', '微软雅黑', 'Segoe UI', sans-serif">Calibri</option>
          <option value="'Microsoft YaHei', '微软雅黑', sans-serif">微软雅黑</option>
          <option value="'SimSun', '宋体', serif">宋体</option>
          <option value="'Arial', sans-serif">Arial</option>
          <option value="'Times New Roman', serif">Times New Roman</option>
        </select>
        <select 
          :value="fontSize" 
          class="ribbon-select compact"
          @input="emit('update:font-size', Number(($event.target as HTMLSelectElement).value))"
        >
          <option value="11">11</option>
          <option value="12">12</option>
          <option value="14">14</option>
          <option value="16">16</option>
          <option value="18">18</option>
          <option value="24">24</option>
          <option value="28">28</option>
          <option value="36">36</option>
        </select>
        <button class="ribbon-button-small" title="文本效果" @click="textEffects">
          <Sparkles :size="14" />
        </button>
        <button class="ribbon-button-small" title="更改大小写" @click="changeCase">
          <Type :size="14" />
        </button>
      </div>

      <!-- Row 2: Standard Styles & Colors -->
      <div class="font-row font-buttons-compact">
        <button class="ribbon-button-small" title="加粗" @click="toggleBold">
          <strong>B</strong>
        </button>
        <button class="ribbon-button-small" title="斜体" @click="toggleItalic">
          <em>I</em>
        </button>
        <button class="ribbon-button-small" title="下划线" @click="toggleUnderline">
          <u>U</u>
        </button>
        <button class="ribbon-button-small" title="删除线" @click="toggleStrike">
          <s>S</s>
        </button>
        <button class="ribbon-button-small" title="下标" @click="toggleSubscript">
          X₂
        </button>
        <button class="ribbon-button-small" title="上标" @click="toggleSuperscript">
          X²
        </button>
        <button class="ribbon-button-small" title="高亮" @click="() => toggleHighlight()">
          🖊
        </button>
        <button class="ribbon-button-small" title="字体颜色" @click="setTextColor">
          <span style="color: #dc2626;">A</span>
        </button>
        <button class="ribbon-button-small" title="字符缩放" @click="characterScale">
          <ZoomIn :size="16" />
        </button>
        <button class="ribbon-button-small" title="小型大写字母" @click="smallCaps">
          <ArrowUp :size="16" />
        </button>
      </div>

      <!-- Row 3: Advanced Text Features -->
      <div class="font-row font-buttons-compact">
        <button class="ribbon-button-small" title="清除格式" @click="clearFormatting">
          <Eraser :size="16" />
        </button>
        <button class="ribbon-button-small" title="拼音指南" @click="pinyinGuide">
          <Languages :size="16" />
        </button>
        <button class="ribbon-button-small" title="带圈字符" @click="enclosedCharacters">
          <Circle :size="16" />
        </button>
        <button class="ribbon-button-small" title="纵向文字" @click="verticalText">
          <RotateCw :size="16" />
        </button>
        <button class="ribbon-button-small" title="双删除线" @click="doubleStrikethrough">
          <Minus :size="16" />
        </button>
        <button class="ribbon-button-small" title="全角/半角" @click="fullHalfWidth">
          <span style="font-size: 12px;">全</span>
        </button>
        <button class="ribbon-button-small" title="字符间距" @click="characterSpacing">
          <Minimize :size="16" />
        </button>
        <button class="ribbon-button-small" title="首字下沉" @click="dropCap">
          <Text :size="16" />
        </button>
        <button class="ribbon-button-small" title="文字边框" @click="textBorder">
          <Square :size="16" />
        </button>
        <button class="ribbon-button-small" title="底纹" @click="textShading">
          <Palette :size="16" />
        </button>
      </div>
    </div>
    <div class="group-label" @click="isFontExpanded = !isFontExpanded">
      <span>字体</span>
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

.font-row {
  display: flex;
  gap: 1px;
  align-items: center;
}

.font-buttons-compact {
  display: flex;
  gap: 1px;
}
</style>
