<template>
  <div class="wallpaper-selector">
    <button
      @click="toggleDialog"
      class="wallpaper-button"
      title="选择墙纸"
      aria-label="选择墙纸"
      :disabled="isUploading"
    >
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
        <circle cx="8.5" cy="8.5" r="1.5"></circle>
        <polyline points="21 15 16 10 5 21"></polyline>
      </svg>
    </button>

    <div v-if="showDialog" class="wallpaper-dialog-overlay" @click="toggleDialog">
      <div class="wallpaper-dialog" @click.stop>
        <div class="dialog-header">
          <h3>选择墙纸</h3>
          <button @click="toggleDialog" class="close-button" aria-label="关闭">×</button>
        </div>
        <div v-if="errorMessage" class="error-message">
          {{ errorMessage }}
          <button @click="errorMessage = null" class="error-close" aria-label="关闭错误">×</button>
        </div>
        <div class="wallpaper-grid" role="list" aria-label="墙纸列表">
          <div
            v-for="wallpaper in wallpapers"
            :key="wallpaper.name"
            class="wallpaper-item"
            :class="{ active: selectedWallpaper === wallpaper.name }"
            @click="selectWallpaper(wallpaper.name)"
          >
            <img :src="wallpaper.path" :alt="wallpaper.label" />
            <span class="wallpaper-label">{{ wallpaper.label }}</span>
          </div>
          <div
            v-for="customWallpaper in customWallpapers"
            :key="customWallpaper.name"
            class="wallpaper-item"
            :class="{ active: selectedWallpaper === customWallpaper.name }"
            @click="selectWallpaper(customWallpaper.name)"
          >
            <img :src="customWallpaper.path" :alt="customWallpaper.label" />
            <span class="wallpaper-label">{{ customWallpaper.label }}</span>
            <button
              @click.stop="removeCustomWallpaper(customWallpaper.name)"
              class="remove-button"
              title="删除"
            >×</button>
          </div>
          <div
            class="wallpaper-item"
            :class="{ active: selectedWallpaper === null }"
            @click="selectWallpaper(null)"
          >
            <div class="no-wallpaper">无墙纸</div>
            <span class="wallpaper-label">无</span>
          </div>
          <div
            class="wallpaper-item upload-item"
            @click="uploadCustomWallpaper"
          >
            <div class="upload-icon">
              <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                <polyline points="17 8 12 3 7 8"></polyline>
                <line x1="12" y1="3" x2="12" y2="15"></line>
              </svg>
            </div>
            <span class="wallpaper-label">上传</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';

interface Wallpaper {
  name: string;
  path: string;
  label: string;
}

const wallpapers: Wallpaper[] = [
  { name: 'jr-korpa-6uEtb1fLX7E-unsplash.jpg', path: '/jr-korpa-6uEtb1fLX7E-unsplash.jpg', label: '简约' },
  { name: 'alexey-o-jTf_h6TtR9Y-unsplash.jpg', path: '/alexey-o-jTf_h6TtR9Y-unsplash.jpg', label: '自然' },
  { name: 'liana-s-k7RLGSA471U-unsplash.jpg', path: '/liana-s-k7RLGSA471U-unsplash.jpg', label: '花卉' },
  { name: 'max-bvp-580uEbTATOw-unsplash.jpg', path: '/max-bvp-580uEbTATOw-unsplash.jpg', label: '抽象' },
  { name: 'sascha-roder-zb3r_kTcVbU-unsplash.jpg', path: '/sascha-roder-zb3r_kTcVbU-unsplash.jpg', label: '科技' },
  { name: 'bernd-dittrich-3MdsHe3IIOk-unsplash.jpg', path: '/bernd-dittrich-3MdsHe3IIOk-unsplash.jpg', label: '城市' },
  { name: 'ines-alvarez-fdez-788PTAxvGnQ-unsplash.jpg', path: '/ines-alvarez-fdez-788PTAxvGnQ-unsplash.jpg', label: '海洋' },
  { name: 'kamil-molendys-YgTL_6KJqMo-unsplash.jpg', path: '/kamil-molendys-YgTL_6KJqMo-unsplash.jpg', label: '山脉' },
  { name: 'zoltan-tasi-B_fv8i18lBI-unsplash.jpg', path: '/zoltan-tasi-B_fv8i18lBI-unsplash.jpg', label: '森林' },
];

const showDialog = ref(false);
const selectedWallpaper = ref<string | null>(null);
const customWallpapers = ref<Wallpaper[]>([]);
const isUploading = ref(false);
const errorMessage = ref<string | null>(null);

// Configuration
const MAX_FILE_SIZE = 5 * 1024 * 1024; // 5MB
const ALLOWED_FILE_TYPES = ['image/jpeg', 'image/jpg', 'image/png', 'image/webp', 'image/gif'];
const MAX_IMAGE_WIDTH = 4096;
const MAX_IMAGE_HEIGHT = 4096;

const emit = defineEmits<{
  (e: 'select', wallpaper: string | null): void;
  (e: 'error', message: string): void;
}>();

const toggleDialog = () => {
  showDialog.value = !showDialog.value;
};

const selectWallpaper = (wallpaper: string | null) => {
  selectedWallpaper.value = wallpaper;
  emit('select', wallpaper);
  saveSelectedWallpaper(wallpaper);
  toggleDialog();
};

const uploadCustomWallpaper = () => {
  errorMessage.value = null;
  const input = document.createElement('input');
  input.type = 'file';
  input.accept = 'image/*';
  input.onchange = async (e) => {
    const file = (e.target as HTMLInputElement).files?.[0];
    if (file) {
      // Validate file size
      if (file.size > MAX_FILE_SIZE) {
        errorMessage.value = `文件大小超过限制 (最大 ${MAX_FILE_SIZE / 1024 / 1024}MB)`;
        emit('error', errorMessage.value);
        return;
      }

      // Validate file type
      if (!ALLOWED_FILE_TYPES.includes(file.type)) {
        errorMessage.value = `不支持的文件类型: ${file.type}`;
        emit('error', errorMessage.value);
        return;
      }

      isUploading.value = true;
      
      try {
        const dataUrl = await validateAndReadImage(file);
        const name = `custom-${Date.now()}-${file.name}`;
        const customWallpaper: Wallpaper = {
          name: name,
          path: dataUrl,
          label: file.name.substring(0, 10)
        };
        customWallpapers.value.push(customWallpaper);
        saveCustomWallpapers();
        selectWallpaper(name);
      } catch (error) {
        errorMessage.value = error instanceof Error ? error.message : '图片上传失败';
        emit('error', errorMessage.value);
      } finally {
        isUploading.value = false;
      }
    }
  };
  input.click();
};

const validateAndReadImage = (file: File): Promise<string> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (event) => {
      const dataUrl = event.target?.result as string;
      
      // Validate image dimensions
      const img = new Image();
      img.onload = () => {
        if (img.width > MAX_IMAGE_WIDTH || img.height > MAX_IMAGE_HEIGHT) {
          reject(new Error(`图片尺寸过大 (最大 ${MAX_IMAGE_WIDTH}x${MAX_IMAGE_HEIGHT})`));
        } else {
          resolve(dataUrl);
        }
      };
      img.onerror = () => {
        reject(new Error('图片加载失败'));
      };
      img.src = dataUrl;
    };
    reader.onerror = () => {
      reject(new Error('文件读取失败'));
    };
    reader.readAsDataURL(file);
  });
};

const removeCustomWallpaper = (name: string) => {
  customWallpapers.value = customWallpapers.value.filter(w => w.name !== name);
  if (selectedWallpaper.value === name) {
    selectWallpaper(null);
  }
  saveCustomWallpapers();
};

const saveCustomWallpapers = () => {
  try {
    localStorage.setItem('custom-wallpapers', JSON.stringify(customWallpapers.value));
  } catch (e) {
    console.error('Failed to save custom wallpapers:', e);
    emit('error', '保存自定义墙纸失败');
  }
};

const saveSelectedWallpaper = (wallpaper: string | null) => {
  try {
    localStorage.setItem('selected-wallpaper', JSON.stringify(wallpaper));
  } catch (e) {
    console.error('Failed to save selected wallpaper:', e);
  }
};

const loadCustomWallpapers = () => {
  try {
    const saved = localStorage.getItem('custom-wallpapers');
    if (saved) {
      customWallpapers.value = JSON.parse(saved);
    }
  } catch (e) {
    console.error('Failed to load custom wallpapers:', e);
  }
};

const loadSelectedWallpaper = () => {
  try {
    const saved = localStorage.getItem('selected-wallpaper');
    if (saved) {
      selectedWallpaper.value = JSON.parse(saved);
    }
  } catch (e) {
    console.error('Failed to load selected wallpaper:', e);
  }
};

onMounted(() => {
  loadCustomWallpapers();
  loadSelectedWallpaper();
  // Initialize with saved wallpaper after loading
  emit('select', selectedWallpaper.value);
});
</script>

<style scoped>
.wallpaper-selector {
  position: relative;
}

.wallpaper-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: 1px solid var(--word-border);
  background: var(--word-bg);
  color: var(--word-text-primary);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.wallpaper-button:hover {
  background: var(--word-button-hover);
  border-color: var(--word-accent);
}

.wallpaper-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.error-message {
  background: rgba(220, 38, 38, 0.1);
  border: 1px solid rgba(220, 38, 38, 0.3);
  color: #dc2626;
  padding: 12px 16px;
  margin: 0 20px;
  border-radius: 4px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 14px;
}

.error-close {
  background: none;
  border: none;
  color: #dc2626;
  cursor: pointer;
  font-size: 18px;
  padding: 0 8px;
  opacity: 0.7;
  transition: opacity 0.2s;
}

.error-close:hover {
  opacity: 1;
}

.wallpaper-dialog-overlay {
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

.wallpaper-dialog {
  background: var(--word-bg);
  border-radius: 8px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  max-width: 600px;
  width: 90%;
  max-height: 80vh;
  overflow: hidden;
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--word-border);
}

.dialog-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 500;
  color: var(--word-text-primary);
}

.close-button {
  background: none;
  border: none;
  font-size: 24px;
  color: var(--word-text-secondary);
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: background 0.2s;
}

.close-button:hover {
  background: var(--word-button-hover);
  color: var(--word-text-primary);
}

.wallpaper-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 12px;
  padding: 20px;
  max-height: 60vh;
  overflow-y: auto;
}

.wallpaper-item {
  cursor: pointer;
  border: 2px solid transparent;
  border-radius: 8px;
  overflow: hidden;
  transition: all 0.2s;
  background: var(--word-bg);
}

.wallpaper-item:hover {
  border-color: var(--word-accent);
  transform: scale(1.05);
}

.wallpaper-item.active {
  border-color: var(--word-accent);
  box-shadow: 0 0 0 2px var(--word-accent);
}

.wallpaper-item img {
  width: 100%;
  height: 80px;
  object-fit: cover;
  display: block;
}

.no-wallpaper {
  width: 100%;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--word-bg);
  color: var(--word-text-tertiary);
  font-size: 12px;
}

.wallpaper-label {
  display: block;
  text-align: center;
  padding: 8px;
  font-size: 12px;
  color: var(--word-text-secondary);
  background: var(--word-bg);
}

.wallpaper-item.active .wallpaper-label {
  color: var(--word-accent);
  font-weight: 500;
}

.upload-item {
  border: 2px dashed var(--word-border);
}

.upload-item:hover {
  border-color: var(--word-accent);
  background: var(--word-button-hover);
}

.upload-icon {
  width: 100%;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--word-text-tertiary);
}

.remove-button {
  position: absolute;
  top: 4px;
  right: 4px;
  width: 20px;
  height: 20px;
  background: rgba(0, 0, 0, 0.6);
  color: white;
  border: none;
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: bold;
  opacity: 0;
  transition: opacity 0.2s;
}

.wallpaper-item:hover .remove-button {
  opacity: 1;
}

.remove-button:hover {
  background: rgba(220, 38, 38, 0.9);
}
</style>
