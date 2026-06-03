<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import DOMPurify from 'dompurify';

interface Props {
  modelValue: string;
  inline?: boolean;
  editable?: boolean;
}

interface Emits {
  (e: 'update:modelValue', value: string): void;
}

const props = withDefaults(defineProps<Props>(), {
  inline: false,
  editable: true
});

const emit = defineEmits<Emits>();

const latexInput = ref(props.modelValue);
const previewHtml = ref('');
const isError = ref(false);
const errorMessage = ref('');
const showEditor = ref(false);
const isRendering = ref(false);

// Render LaTeX using Rust backend
const renderLatex = async () => {
  if (!latexInput.value.trim()) {
    previewHtml.value = '';
    return;
  }

  isRendering.value = true;
  try {
    const html = await invoke<string>('render_latex', {
      latex: latexInput.value,
      displayMode: !props.inline
    });
    // Sanitize HTML to prevent XSS attacks
    const sanitizedHtml = DOMPurify.sanitize(html, {
      USE_PROFILES: { html: true, mathMl: true }
    });
    previewHtml.value = sanitizedHtml;
    isError.value = false;
    errorMessage.value = '';
  } catch (error) {
    isError.value = true;
    errorMessage.value = error instanceof Error ? error.message : 'Invalid LaTeX';
    previewHtml.value = `<span class="katex-error">${errorMessage.value}</span>`;
  } finally {
    isRendering.value = false;
  }
};

// Update LaTeX and emit
const updateLatex = (value: string) => {
  latexInput.value = value;
  emit('update:modelValue', value);
  renderLatex();
};

// Open editor
const openEditor = () => {
  showEditor.value = true;
  latexInput.value = props.modelValue;
  renderLatex();
};

// Close editor
const closeEditor = () => {
  showEditor.value = false;
};

// Insert symbol
const insertSymbol = (symbol: string) => {
  const textarea = document.querySelector('.latex-input') as HTMLTextAreaElement;
  if (textarea) {
    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;
    const text = latexInput.value;
    const before = text.substring(0, start);
    const after = text.substring(end);
    latexInput.value = before + symbol + after;
    textarea.selectionStart = textarea.selectionEnd = start + symbol.length;
    updateLatex(latexInput.value);
    textarea.focus();
  }
};

// Common LaTeX symbols
const commonSymbols = [
  { label: 'α', value: '\\alpha' },
  { label: 'β', value: '\\beta' },
  { label: 'γ', value: '\\gamma' },
  { label: 'δ', value: '\\delta' },
  { label: 'θ', value: '\\theta' },
  { label: 'π', value: '\\pi' },
  { label: 'Σ', value: '\\sum' },
  { label: '∫', value: '\\int' },
  { label: '√', value: '\\sqrt{}' },
  { label: '∞', value: '\\infty' },
  { label: '≠', value: '\\neq' },
  { label: '≤', value: '\\leq' },
  { label: '≥', value: '\\geq' },
  { label: '±', value: '\\pm' },
  { label: '→', value: '\\rightarrow' }
];

// Common templates
const commonTemplates = [
  { label: 'Fraction', value: '\\frac{a}{b}' },
  { label: 'Power', value: 'x^{n}' },
  { label: 'Subscript', value: 'x_{n}' },
  { label: 'Sum', value: '\\sum_{i=1}^{n}' },
  { label: 'Integral', value: '\\int_{a}^{b}' },
  { label: 'Matrix', value: '\\begin{pmatrix} a & b \\\\ c & d \\end{pmatrix}' },
  { label: 'Binomial', value: '\\binom{n}{k}' },
  { label: 'Cases', value: '\\begin{cases} x & x > 0 \\\\ -x & x < 0 \\end{cases}' }
];

// Watch for model value changes
watch(
  () => props.modelValue,
  newValue => {
    latexInput.value = newValue;
    renderLatex();
  }
);

onMounted(() => {
  renderLatex();
});
</script>

<template>
  <div class="math-formula-editor">
    <!-- Preview -->
    <div
      class="math-preview"
      :class="{ 'inline-mode': inline, error: isError, rendering: isRendering }"
      @click="editable ? openEditor() : null"
    >
      <span v-if="isRendering" class="loading-indicator">Rendering...</span>
      <span v-else class="katex-render" v-html="previewHtml"></span>
      <span v-if="editable && !showEditor" class="edit-hint">Click to edit</span>
    </div>

    <!-- Editor Modal -->
    <div v-if="showEditor" class="editor-modal">
      <div class="editor-content">
        <div class="editor-header">
          <h3>Math Formula Editor</h3>
          <button class="close-button" @click="closeEditor">&times;</button>
        </div>

        <div class="editor-body">
          <!-- LaTeX Input -->
          <div class="input-section">
            <label>LaTeX Code:</label>
            <textarea
              v-model="latexInput"
              class="latex-input"
              placeholder="Enter LaTeX formula..."
              rows="4"
              @input="updateLatex(latexInput)"
            ></textarea>
          </div>

          <!-- Live Preview -->
          <div class="preview-section">
            <label>Preview:</label>
            <div class="preview-box" :class="{ error: isError, rendering: isRendering }">
              <span v-if="isRendering" class="loading-indicator">Rendering...</span>
              <span v-else class="katex-render" v-html="previewHtml"></span>
            </div>
            <div v-if="isError" class="error-message">{{ errorMessage }}</div>
          </div>

          <!-- Common Symbols -->
          <div class="symbols-section">
            <label>Common Symbols:</label>
            <div class="symbol-grid">
              <button
                v-for="symbol in commonSymbols"
                :key="symbol.value"
                class="symbol-button"
                :title="symbol.value"
                @click="insertSymbol(symbol.value)"
              >
                {{ symbol.label }}
              </button>
            </div>
          </div>

          <!-- Common Templates -->
          <div class="templates-section">
            <label>Templates:</label>
            <div class="template-grid">
              <button
                v-for="template in commonTemplates"
                :key="template.value"
                class="template-button"
                :title="template.value"
                @click="insertSymbol(template.value)"
              >
                {{ template.label }}
              </button>
            </div>
          </div>
        </div>

        <div class="editor-footer">
          <button class="cancel-button" @click="closeEditor">Cancel</button>
          <button class="insert-button" @click="closeEditor">Insert</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.math-formula-editor {
  position: relative;
  display: inline-block;
}

.math-preview {
  display: inline-block;
  padding: 8px 12px;
  border: 1px solid #e5e7eb;
  border-radius: 4px;
  background: #f9fafb;
  cursor: pointer;
  transition: all 0.2s;
  min-width: 50px;
  min-height: 30px;
}

.math-preview:hover {
  border-color: #3b82f6;
  background: #eff6ff;
}

.math-preview.inline-mode {
  display: inline;
  padding: 2px 6px;
  vertical-align: middle;
}

.math-preview.error {
  border-color: #ef4444;
  background: #fef2f2;
}

.math-preview.rendering {
  opacity: 0.6;
}

.edit-hint {
  margin-left: 8px;
  font-size: 12px;
  color: #6b7280;
  opacity: 0;
  transition: opacity 0.2s;
}

.math-preview:hover .edit-hint {
  opacity: 1;
}

.katex-render {
  font-size: 16px;
}

.katex-error {
  color: #ef4444;
  font-style: italic;
}

.loading-indicator {
  color: #6b7280;
  font-style: italic;
}

/* Editor Modal */
.editor-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.editor-content {
  background: white;
  border-radius: 8px;
  width: 90%;
  max-width: 800px;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid #e5e7eb;
}

.editor-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #111827;
}

.close-button {
  background: none;
  border: none;
  font-size: 24px;
  color: #6b7280;
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s;
}

.close-button:hover {
  background: #f3f4f6;
  color: #111827;
}

.editor-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.input-section,
.preview-section,
.symbols-section,
.templates-section {
  margin-bottom: 20px;
}

label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: #374151;
  margin-bottom: 8px;
}

.latex-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  font-size: 14px;
  resize: vertical;
  transition: border-color 0.2s;
}

.latex-input:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.preview-box {
  min-height: 80px;
  padding: 16px;
  border: 1px solid #e5e7eb;
  border-radius: 4px;
  background: #f9fafb;
  display: flex;
  align-items: center;
  justify-content: center;
}

.preview-box.error {
  border-color: #ef4444;
  background: #fef2f2;
}

.preview-box.rendering {
  opacity: 0.6;
}

.error-message {
  margin-top: 8px;
  font-size: 12px;
  color: #ef4444;
}

.symbol-grid,
.template-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(60px, 1fr));
  gap: 8px;
}

.symbol-button,
.template-button {
  padding: 8px 12px;
  border: 1px solid #d1d5db;
  background: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.symbol-button:hover,
.template-button:hover {
  background: #f3f4f6;
  border-color: #9ca3af;
}

.editor-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid #e5e7eb;
}

.cancel-button,
.insert-button {
  padding: 8px 16px;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.cancel-button {
  background: white;
  border: 1px solid #d1d5db;
  color: #374151;
}

.cancel-button:hover {
  background: #f3f4f6;
}

.insert-button {
  background: #3b82f6;
  border: 1px solid #3b82f6;
  color: white;
}

.insert-button:hover {
  background: #2563eb;
}

/* Dark mode */
.editor-container.dark .math-preview {
  background: #1f2937;
  border-color: #374151;
}

.editor-container.dark .math-preview:hover {
  background: #374151;
  border-color: #60a5fa;
}

.editor-container.dark .math-preview.error {
  background: #450a0a;
  border-color: #dc2626;
}

.editor-container.dark .edit-hint {
  color: #9ca3af;
}

.editor-container.dark .editor-content {
  background: #1f2937;
}

.editor-container.dark .editor-header {
  border-bottom-color: #374151;
}

.editor-container.dark .editor-header h3 {
  color: #f9fafb;
}

.editor-container.dark .close-button {
  color: #9ca3af;
}

.editor-container.dark .close-button:hover {
  background: #374151;
  color: #f9fafb;
}

.editor-container.dark label {
  color: #d1d5db;
}

.editor-container.dark .latex-input {
  background: #374151;
  border-color: #4b5563;
  color: #f9fafb;
}

.editor-container.dark .latex-input:focus {
  border-color: #60a5fa;
}

.editor-container.dark .preview-box {
  background: #374151;
  border-color: #4b5563;
}

.editor-container.dark .preview-box.error {
  background: #450a0a;
  border-color: #dc2626;
}

.editor-container.dark .symbol-button,
.editor-container.dark .template-button {
  background: #374151;
  border-color: #4b5563;
  color: #f9fafb;
}

.editor-container.dark .symbol-button:hover,
.editor-container.dark .template-button:hover {
  background: #4b5563;
  border-color: #6b7280;
}

.editor-container.dark .editor-footer {
  border-top-color: #374151;
}

.editor-container.dark .cancel-button {
  background: #374151;
  border-color: #4b5563;
  color: #f9fafb;
}

.editor-container.dark .cancel-button:hover {
  background: #4b5563;
}
</style>
