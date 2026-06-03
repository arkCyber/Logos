<script setup lang="ts">
import { ref } from 'vue';
import { Layers, Sparkles, Star, Plus, Layout, MoveDiagonal2, Quote, List, Book, Zap, FileText, Text } from 'lucide-vue-next';

interface Props {
  selectedStyle: string;
}

interface Emits {
  (e: 'update:selectedStyle', value: string): void;
  (e: 'change-style-set'): void;
  (e: 'apply-emphasis'): void;
  (e: 'apply-strong-emphasis'): void;
  (e: 'apply-quote'): void;
  (e: 'apply-list-paragraph'): void;
  (e: 'apply-intense-quote'): void;
  (e: 'apply-subtle-reference'): void;
  (e: 'apply-book-title'): void;
  (e: 'apply-intense-emphasis'): void;
  (e: 'new-style'): void;
  (e: 'style-pane'): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const isStylesExpanded = ref(false);

const changeStyleSet = () => {
  emit('change-style-set');
};

const applyEmphasis = () => {
  emit('apply-emphasis');
};

const applyStrongEmphasis = () => {
  emit('apply-strong-emphasis');
};

const applyQuote = () => {
  emit('apply-quote');
};

const applyListParagraph = () => {
  emit('apply-list-paragraph');
};

const applyIntenseQuote = () => {
  emit('apply-intense-quote');
};

const applySubtleReference = () => {
  emit('apply-subtle-reference');
};

const applyBookTitle = () => {
  emit('apply-book-title');
};

const applyIntenseEmphasis = () => {
  emit('apply-intense-emphasis');
};

const newStyle = () => {
  emit('new-style');
};

const stylePane = () => {
  emit('style-pane');
};
</script>

<template>
  <div class="ribbon-group">
    <div class="group-content">
      <!-- Row 1: Style Select & Style Set -->
      <div class="styles-row">
        <select 
          :value="selectedStyle" 
          class="ribbon-select compact" 
          @change="emit('update:selectedStyle', ($event.target as HTMLSelectElement).value)"
        >
          <option value="normal">正常</option>
          <option value="no-spacing">无间距</option>
          <option value="heading1">标题1</option>
          <option value="heading2">标题2</option>
          <option value="heading3">标题3</option>
          <option value="heading4">标题4</option>
          <option value="heading5">标题5</option>
          <option value="heading6">标题6</option>
          <option value="heading7">标题7</option>
          <option value="heading8">标题8</option>
          <option value="heading9">标题9</option>
          <option value="title">标题</option>
          <option value="subtitle">副标题</option>
        </select>
        <button class="ribbon-button-small" title="样式集" @click="changeStyleSet">
          <Layers :size="16" />
        </button>
      </div>

      <!-- Row 2: Emphasis & Quote Styles -->
      <div class="styles-row styles-buttons-compact">
        <button class="ribbon-button-small" title="强调" @click="applyEmphasis">
          <Sparkles :size="16" />
        </button>
        <button class="ribbon-button-small" title="明显强调" @click="applyStrongEmphasis">
          <Star :size="16" />
        </button>
        <button class="ribbon-button-small" title="引用" @click="applyQuote">
          <Quote :size="16" />
        </button>
        <button class="ribbon-button-small" title="列表段落" @click="applyListParagraph">
          <List :size="16" />
        </button>
        <button class="ribbon-button-small" title="明显引用" @click="applyIntenseQuote">
          <FileText :size="16" />
        </button>
      </div>

      <!-- Row 3: More Styles & Management -->
      <div class="styles-row styles-buttons-compact">
        <button class="ribbon-button-small" title="微妙引用" @click="applySubtleReference">
          <Text :size="16" />
        </button>
        <button class="ribbon-button-small" title="书题" @click="applyBookTitle">
          <Book :size="16" />
        </button>
        <button class="ribbon-button-small" title="强烈强调" @click="applyIntenseEmphasis">
          <Zap :size="16" />
        </button>
        <button class="ribbon-button-small" title="新建样式" @click="newStyle">
          <Plus :size="16" />
        </button>
        <button class="ribbon-button-small" title="样式窗格" @click="stylePane">
          <Layout :size="16" />
        </button>
      </div>
    </div>
    <div class="group-label" @click="isStylesExpanded = !isStylesExpanded">
      <span>样式</span>
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

.styles-row {
  display: flex;
  gap: 1px;
  align-items: center;
}

.styles-buttons-compact {
  display: flex;
  gap: 1px;
}
</style>
