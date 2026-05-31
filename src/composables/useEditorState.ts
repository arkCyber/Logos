import { ref, computed } from 'vue';

export interface EditorState {
  isDarkMode: boolean;
  autoSaveEnabled: boolean;
  isFullscreen: boolean;
  zoomLevel: number;
  fontSize: number;
  lineHeight: number;
  fontFamily: string;
  textColor: string;
  backgroundColor: string;
  highlightColor: string;
  wordCount: number;
  charCount: number;
  currentPage: number;
  totalPages: number;
}

export function useEditorState() {
  const isDarkMode = ref(false);
  const autoSaveEnabled = ref(true);
  const isFullscreen = ref(false);
  const zoomLevel = ref(100);
  const fontSize = ref(11);
  const lineHeight = ref(1.15);
  const fontFamily = ref('Calibri, "Microsoft YaHei", "微软雅黑", "Segoe UI", sans-serif');
  const textColor = ref('#000000');
  const backgroundColor = ref('#ffffff');
  const highlightColor = ref('#ffff00');
  const wordCount = ref(0);
  const charCount = ref(0);
  const currentPage = ref(1);
  const totalPages = ref(1);

  const toggleTheme = () => {
    isDarkMode.value = !isDarkMode.value;
  };

  const toggleAutoSave = () => {
    autoSaveEnabled.value = !autoSaveEnabled.value;
  };

  const toggleFullscreen = () => {
    isFullscreen.value = !isFullscreen.value;
    if (isFullscreen.value) {
      document.documentElement.requestFullscreen();
    } else {
      document.exitFullscreen();
    }
  };

  const setZoom = (level: number) => {
    if (level >= 25 && level <= 400) {
      zoomLevel.value = level;
    }
  };

  const zoomIn = () => {
    setZoom(zoomLevel.value + 25);
  };

  const zoomOut = () => {
    setZoom(zoomLevel.value - 25);
  };

  const zoom100 = () => {
    setZoom(100);
  };

  const state = computed<EditorState>(() => ({
    isDarkMode: isDarkMode.value,
    autoSaveEnabled: autoSaveEnabled.value,
    isFullscreen: isFullscreen.value,
    zoomLevel: zoomLevel.value,
    fontSize: fontSize.value,
    lineHeight: lineHeight.value,
    fontFamily: fontFamily.value,
    textColor: textColor.value,
    backgroundColor: backgroundColor.value,
    highlightColor: highlightColor.value,
    wordCount: wordCount.value,
    charCount: charCount.value,
    currentPage: currentPage.value,
    totalPages: totalPages.value
  }));

  return {
    // State
    isDarkMode,
    autoSaveEnabled,
    isFullscreen,
    zoomLevel,
    fontSize,
    lineHeight,
    fontFamily,
    textColor,
    backgroundColor,
    highlightColor,
    wordCount,
    charCount,
    currentPage,
    totalPages,
    // Computed
    state,
    // Actions
    toggleTheme,
    toggleAutoSave,
    toggleFullscreen,
    setZoom,
    zoomIn,
    zoomOut,
    zoom100
  };
}
