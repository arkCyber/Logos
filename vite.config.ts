import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  
  server: {
    port: 1425,
    strictPort: true,
  },
  
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
    }
  },
  
  // 测试配置
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    include: ['src/**/*.{test,spec}.{js,ts,jsx,tsx}'],
    exclude: ['node_modules', 'dist', 'src-tauri'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html', 'lcov'],
      exclude: [
        'node_modules/',
        'src/test/',
        '**/*.spec.ts',
        '**/*.test.ts',
        '**/types/',
        '**/*.d.ts',
        'src-tauri/',
        'dist/'
      ]
    }
  },
  
  optimizeDeps: {
    exclude: ['events', 'fs', 'buffer']
  },
  
  build: {
    // 代码分割优化
    rollupOptions: {
      output: {
        manualChunks: {
          // Vue 核心
          'vue-vendor': ['vue', '@tiptap/vue-3'],
          
          // TipTap 核心（合并以避免循环依赖）
          'editor-core': [
            '@tiptap/core',
            '@tiptap/starter-kit',
            '@tiptap/extension-table',
            '@tiptap/extension-table-row',
            '@tiptap/extension-table-cell',
            '@tiptap/extension-table-header',
            '@tiptap/extension-image',
            '@tiptap/extension-link',
            '@tiptap/extension-text-align',
            '@tiptap/extension-code-block-lowlight',
            '@tiptap/extension-underline',
            '@tiptap/extension-task-list',
            '@tiptap/extension-task-item',
            '@tiptap/extension-text-style',
            '@tiptap/extension-highlight',
            '@tiptap/extension-placeholder',
            '@tiptap/extension-typography',
            '@tiptap/extension-bubble-menu',
            '@tiptap/suggestion',
            '@tiptap/extension-font-family',
            '@tiptap/extension-strike',
            '@tiptap/extension-subscript',
            '@tiptap/extension-superscript'
          ],
          
          // 数学公式
          'katex': ['katex'],
          
          // 文档处理
          'docx': ['docx'],
          
          // 代码高亮
          'lowlight': ['lowlight'],
          
          // Tauri
          'tauri': [
            '@tauri-apps/api',
            '@tauri-apps/plugin-dialog',
            '@tauri-apps/plugin-opener'
          ]
        }
      }
    },
    
    // 提高警告阈值
    chunkSizeWarningLimit: 1000,
    
    // 生产环境优化
    minify: false,
    
    // 源码映射
    sourcemap: false,
    
    // 目标浏览器
    target: 'esnext'
  }
});
