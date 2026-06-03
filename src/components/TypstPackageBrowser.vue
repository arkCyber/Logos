<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { invoke } from '@tauri-apps/api/core'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { logger, LogCategory } from '../utils/logger';
import { debounce } from '../utils/debounce';
import { auditLogger, AuditAction } from '../utils/auditLogger';

interface TypstPackage {
  name: string;
  version: string;
  description: string;
  author: string;
  license: string;
  homepage: string;
  repository: string;
  keywords: string[];
  installed: boolean;
  installedVersion?: string;
  dependencies: string[];
  downloads: number;
  rating: number;
}

const packages = ref<TypstPackage[]>([]);
const searchQuery = ref('');
const selectedCategory = ref('');
const isLoading = ref(false);
const showDetailsDialog = ref(false);
const selectedPackage = ref<TypstPackage | null>(null);
const errorMessage = ref('');
const successMessage = ref('');
const isInstalling = ref(false);
const isUninstalling = ref(false);
const isUpdating = ref(false);
const showContextMenu = ref(false);
const contextMenuPosition = ref({ x: 0, y: 0 });
const contextMenuPackage = ref<TypstPackage | null>(null);

const categories = computed(() => {
  const categorySet = new Set<string>();
  packages.value.forEach(pkg => {
    pkg.keywords.forEach(keyword => categorySet.add(keyword));
  });
  return Array.from(categorySet).sort();
});

const filteredPackages = computed(() => {
  let result = packages.value;

  if (selectedCategory.value) {
    result = result.filter(pkg => 
      pkg.keywords.includes(selectedCategory.value)
    );
  }

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    result = result.filter(pkg =>
      pkg.name.toLowerCase().includes(query) ||
      pkg.description.toLowerCase().includes(query) ||
      pkg.keywords.some(k => k.toLowerCase().includes(query))
    );
  }

  return result;
});

const installedPackages = computed(() => {
  return packages.value.filter(pkg => pkg.installed);
});

onMounted(async () => {
  await loadPackages();
  setupKeyboardShortcuts();
});

// 键盘快捷键
function setupKeyboardShortcuts() {
  const handleKeyDown = (e: KeyboardEvent) => {
    // Ctrl/Cmd + F: 聚焦搜索框
    if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
      e.preventDefault();
      const searchInput = document.querySelector('.search-input') as HTMLInputElement;
      if (searchInput) {
        searchInput.focus();
      }
    }
    
    // Ctrl/Cmd + R: 刷新包列表
    if ((e.ctrlKey || e.metaKey) && e.key === 'r') {
      e.preventDefault();
      loadPackages();
    }
    
    // Escape: 关闭对话框
    if (e.key === 'Escape') {
      if (showDetailsDialog.value) {
        showDetailsDialog.value = false;
      }
      if (showContextMenu.value) {
        closeContextMenu();
      }
      if (errorMessage.value) {
        errorMessage.value = '';
      }
      if (successMessage.value) {
        successMessage.value = '';
      }
    }
  };
  
  window.addEventListener('keydown', handleKeyDown);
  window.addEventListener('click', () => {
    if (showContextMenu.value) {
      closeContextMenu();
    }
  });
  
  // 清理函数
  return () => {
    window.removeEventListener('keydown', handleKeyDown);
  };
}

// 防抖搜索
const debouncedSearch = debounce((query: string) => { // eslint-disable-line @typescript-eslint/no-unused-vars
  searchQuery.value = query;
}, 300);

// 验证包数据
function validatePackage(pkg: any): pkg is TypstPackage {
  return (
    typeof pkg.name === 'string' &&
    pkg.name.length > 0 &&
    pkg.name.length <= 100 &&
    typeof pkg.version === 'string' &&
    pkg.version.length > 0 &&
    pkg.version.length <= 50 &&
    typeof pkg.description === 'string' &&
    pkg.description.length <= 500 &&
    typeof pkg.author === 'string' &&
    pkg.author.length <= 100 &&
    typeof pkg.license === 'string' &&
    pkg.license.length <= 50 &&
    Array.isArray(pkg.keywords) &&
    pkg.keywords.every((k: any) => typeof k === 'string' && k.length <= 50) &&
    typeof pkg.installed === 'boolean' &&
    typeof pkg.downloads === 'number' &&
    pkg.downloads >= 0 &&
    typeof pkg.rating === 'number' &&
    pkg.rating >= 0 &&
    pkg.rating <= 5
  );
}

// XSS防护：转义HTML
function escapeHtml(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

// 验证URL
function _isValidUrl(url: string): boolean { // eslint-disable-line @typescript-eslint/no-unused-vars
  try {
    new URL(url);
    return true;
  } catch {
    return false;
  }
}

async function loadPackages() {
  isLoading.value = true;
  errorMessage.value = '';
  try {
    // TODO: 调用后端API获取包列表
    // const rawData = await invoke('get_typst_packages');
    // packages.value = rawData.filter(validatePackage);
    
    // 临时：模拟数据
    const mockData = [
      {
        name: 'typst-cite',
        version: '0.1.0',
        description: 'Citations and bibliography for Typst',
        author: 'Typst Authors',
        license: 'MIT',
        homepage: 'https://typst.app',
        repository: 'https://github.com/typst/packages',
        keywords: ['bibliography', 'citations', 'academic'],
        installed: false,
        dependencies: [],
        downloads: 1250,
        rating: 4.5
      },
      {
        name: 'typst-math',
        version: '0.2.0',
        description: 'Advanced mathematical formulas',
        author: 'Math Team',
        license: 'Apache-2.0',
        homepage: 'https://typst.app',
        repository: 'https://github.com/typst/packages',
        keywords: ['math', 'formulas', 'scientific'],
        installed: true,
        installedVersion: '0.1.5',
        dependencies: [],
        downloads: 3400,
        rating: 4.8
      },
      {
        name: 'typst-code',
        version: '0.3.0',
        description: 'Code highlighting and formatting',
        author: 'Code Team',
        license: 'MIT',
        homepage: 'https://typst.app',
        repository: 'https://github.com/typst/packages',
        keywords: ['code', 'syntax', 'programming'],
        installed: false,
        dependencies: [],
        downloads: 890,
        rating: 4.2
      }
    ];
    
    // 验证数据
    packages.value = mockData.filter(validatePackage);
    
    logger.info('Typst packages loaded', { count: packages.value.length }, LogCategory.BUSINESS);
    auditLogger.log(AuditAction.REFRESH, { count: packages.value.length }, true);
  } catch (error) {
    const errorMsg = '加载包列表失败';
    errorMessage.value = errorMsg;
    logger.error(errorMsg, error as Error, LogCategory.SYSTEM);
    auditLogger.log(AuditAction.REFRESH, {}, false, errorMsg);
  } finally {
    isLoading.value = false;
  }
}

async function installPackage(pkg: TypstPackage) {
  if (isInstalling.value) {
return;
}
  
  errorMessage.value = '';
  isInstalling.value = true;
  
  try {
    // TODO: 调用后端API安装包
    // await invoke('install_typst_package', { name: pkg.name, version: pkg.version });
    
    pkg.installed = true;
    pkg.installedVersion = pkg.version;
    successMessage.value = `包 ${escapeHtml(pkg.name)} 安装成功`;
    
    logger.info('Package installed', { name: pkg.name, version: pkg.version }, LogCategory.BUSINESS);
    auditLogger.log(AuditAction.PACKAGE_INSTALL, { name: pkg.name, version: pkg.version }, true);
    
    // 3秒后清除成功消息
    setTimeout(() => {
      successMessage.value = '';
    }, 3000);
  } catch (error) {
    const errorMsg = `安装包 ${escapeHtml(pkg.name)} 失败`;
    errorMessage.value = errorMsg;
    logger.error(errorMsg, error as Error, LogCategory.SYSTEM);
    auditLogger.log(AuditAction.PACKAGE_INSTALL, { name: pkg.name, version: pkg.version }, false, errorMsg);
  } finally {
    isInstalling.value = false;
  }
}

async function uninstallPackage(pkg: TypstPackage) {
  if (isUninstalling.value) {
return;
}
  
  errorMessage.value = '';
  isUninstalling.value = true;
  
  try {
    // TODO: 调用后端API卸载包
    // await invoke('uninstall_typst_package', { name: pkg.name });
    
    pkg.installed = false;
    pkg.installedVersion = undefined;
    successMessage.value = `包 ${escapeHtml(pkg.name)} 卸载成功`;
    
    logger.info('Package uninstalled', { name: pkg.name }, LogCategory.BUSINESS);
    auditLogger.log(AuditAction.PACKAGE_UNINSTALL, { name: pkg.name }, true);
    
    // 3秒后清除成功消息
    setTimeout(() => {
      successMessage.value = '';
    }, 3000);
  } catch (error) {
    const errorMsg = `卸载包 ${escapeHtml(pkg.name)} 失败`;
    errorMessage.value = errorMsg;
    logger.error(errorMsg, error as Error, LogCategory.SYSTEM);
    auditLogger.log(AuditAction.PACKAGE_UNINSTALL, { name: pkg.name }, false, errorMsg);
  } finally {
    isUninstalling.value = false;
  }
}

async function updatePackage(pkg: TypstPackage) {
  if (isUpdating.value) {
return;
}
  
  errorMessage.value = '';
  isUpdating.value = true;
  
  try {
    // TODO: 调用后端API更新包
    // await invoke('update_typst_package', { name: pkg.name });
    
    pkg.installedVersion = pkg.version;
    successMessage.value = `包 ${escapeHtml(pkg.name)} 更新成功`;
    
    logger.info('Package updated', { name: pkg.name, version: pkg.version }, LogCategory.BUSINESS);
    auditLogger.log(AuditAction.PACKAGE_UPDATE, { name: pkg.name, version: pkg.version }, true);
    
    // 3秒后清除成功消息
    setTimeout(() => {
      successMessage.value = '';
    }, 3000);
  } catch (error) {
    const errorMsg = `更新包 ${escapeHtml(pkg.name)} 失败`;
    errorMessage.value = errorMsg;
    logger.error(errorMsg, error as Error, LogCategory.SYSTEM);
    auditLogger.log(AuditAction.PACKAGE_UPDATE, { name: pkg.name, version: pkg.version }, false, errorMsg);
  } finally {
    isUpdating.value = false;
  }
}

function showPackageDetails(pkg: TypstPackage) {
  selectedPackage.value = pkg;
  showDetailsDialog.value = true;
  auditLogger.log(AuditAction.PACKAGE_VIEW, { name: pkg.name, version: pkg.version }, true);
}

// 右键菜单处理
function handleContextMenu(event: MouseEvent, pkg: TypstPackage) {
  event.preventDefault();
  contextMenuPackage.value = pkg;
  
  // 边界检测，防止菜单超出屏幕
  const menuWidth = 180;
  const menuHeight = 200; // 估计高度
  const padding = 8;
  
  let x = event.clientX;
  let y = event.clientY;
  
  // 检查右边界
  if (x + menuWidth > window.innerWidth - padding) {
    x = window.innerWidth - menuWidth - padding;
  }
  
  // 检查下边界
  if (y + menuHeight > window.innerHeight - padding) {
    y = window.innerHeight - menuHeight - padding;
  }
  
  // 确保不超出左边界和上边界
  x = Math.max(padding, x);
  y = Math.max(padding, y);
  
  contextMenuPosition.value = { x, y };
  showContextMenu.value = true;
}

function closeContextMenu() {
  showContextMenu.value = false;
  contextMenuPackage.value = null;
}

function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text).then(() => {
    successMessage.value = '已复制到剪贴板';
    setTimeout(() => {
      successMessage.value = '';
    }, 2000);
  }).catch((error) => {
    errorMessage.value = '复制失败';
    logger.error('Failed to copy to clipboard', error as Error, LogCategory.SYSTEM);
  });
}

function copyPackageName() {
  if (contextMenuPackage.value) {
    copyToClipboard(contextMenuPackage.value.name);
  }
}

function copyPackageVersion() {
  if (contextMenuPackage.value) {
    copyToClipboard(contextMenuPackage.value.version);
  }
}

function contextMenuInstall() {
  if (contextMenuPackage.value) {
    installPackage(contextMenuPackage.value);
    closeContextMenu();
  }
}

function contextMenuUninstall() {
  if (contextMenuPackage.value) {
    uninstallPackage(contextMenuPackage.value);
    closeContextMenu();
  }
}

function contextMenuUpdate() {
  if (contextMenuPackage.value) {
    updatePackage(contextMenuPackage.value);
    closeContextMenu();
  }
}

function contextMenuViewDetails() {
  if (contextMenuPackage.value) {
    showPackageDetails(contextMenuPackage.value);
    closeContextMenu();
  }
}

function getStarRating(rating: number): string {
  const fullStars = Math.floor(rating);
  const hasHalfStar = rating % 1 >= 0.5;
  let stars = '⭐'.repeat(fullStars);
  if (hasHalfStar) {
stars += '⭐';
}
  return stars || '暂无评分';
}
</script>

<template>
  <div class="typst-package-browser">
    <div class="browser-header">
      <h2>Typst 包浏览器</h2>
      <div class="header-actions">
        <div class="search-box">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索包... (Ctrl+F)"
            class="search-input"
            maxlength="100"
            aria-label="搜索包"
            role="searchbox"
          />
        </div>
        <button class="btn-refresh" :disabled="isLoading" aria-label="刷新包列表" title="快捷键: Ctrl+R" @click="loadPackages">
          {{ isLoading ? '加载中...' : '刷新' }}
        </button>
      </div>
      
      <div v-if="errorMessage" class="error-message">
        {{ errorMessage }}
        <button class="btn-close-error" @click="errorMessage = ''">✕</button>
      </div>
      
      <div v-if="successMessage" class="success-message">
        {{ successMessage }}
        <button class="btn-close-success" @click="successMessage = ''">✕</button>
      </div>
    </div>

    <div class="browser-filters">
      <div class="filter-group">
        <label>分类:</label>
        <select v-model="selectedCategory" class="filter-select">
          <option value="">全部</option>
          <option v-for="category in categories" :key="category" :value="category">
            {{ category }}
          </option>
        </select>
      </div>
      
      <div class="filter-group">
        <label>已安装:</label>
        <button class="filter-btn" :class="{ active: selectedCategory === '' }" @click="selectedCategory = ''">
          全部
        </button>
        <button class="filter-btn" :class="{ active: selectedCategory === 'installed' }" @click="selectedCategory = 'installed'">
          已安装 ({{ installedPackages.length }})
        </button>
      </div>
    </div>

    <div class="packages-container">
      <div v-if="isLoading" class="loading-state">
        <div class="spinner"></div>
        <p>加载包列表...</p>
      </div>
      
      <div v-else-if="filteredPackages.length === 0" class="empty-state">
        <p>没有找到匹配的包</p>
      </div>
      
      <div v-else class="packages-grid">
        <div
          v-for="pkg in filteredPackages"
          :key="pkg.name"
          class="package-card"
          @click="showPackageDetails(pkg)"
          @contextmenu="handleContextMenu($event, pkg)"
        >
          <div class="package-header">
            <h3>{{ pkg.name }}</h3>
            <span class="version">v{{ pkg.version }}</span>
          </div>
          
          <p class="description">{{ pkg.description }}</p>
          
          <div class="package-meta">
            <span class="author">{{ pkg.author }}</span>
            <span class="license">{{ pkg.license }}</span>
          </div>
          
          <div class="package-stats">
            <span class="rating" :title="`评分: ${pkg.rating}`">
              {{ getStarRating(pkg.rating) }}
            </span>
            <span class="downloads" :title="`下载量: ${pkg.downloads}`">
              ⬇️ {{ pkg.downloads }}
            </span>
          </div>
          
          <div class="package-tags">
            <span v-for="keyword in pkg.keywords.slice(0, 3)" :key="keyword" class="tag">
              {{ keyword }}
            </span>
          </div>
          
          <div class="package-actions">
            <button
              v-if="!pkg.installed"
              class="btn-install"
              @click.stop="installPackage(pkg)"
            >
              安装
            </button>
            <button
              v-else
              class="btn-uninstall"
              @click.stop="uninstallPackage(pkg)"
            >
              卸载
            </button>
            <button
              v-if="pkg.installed && pkg.installedVersion !== pkg.version"
              class="btn-update"
              @click.stop="updatePackage(pkg)"
            >
              更新
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 右键菜单 -->
    <div
      v-if="showContextMenu && contextMenuPackage"
      class="context-menu"
      :style="{ left: contextMenuPosition.x + 'px', top: contextMenuPosition.y + 'px' }"
      role="menu"
      aria-label="包操作菜单"
      @click.stop
    >
      <div class="context-menu-item" role="menuitem" tabindex="0" @click="contextMenuViewDetails">
        <span class="menu-icon">📋</span>
        查看详情
      </div>
      <div v-if="!contextMenuPackage.installed" class="context-menu-item" role="menuitem" tabindex="0" @click="contextMenuInstall">
        <span class="menu-icon">⬇️</span>
        安装
      </div>
      <div v-if="contextMenuPackage.installed" class="context-menu-item" role="menuitem" tabindex="0" @click="contextMenuUninstall">
        <span class="menu-icon">🗑️</span>
        卸载
      </div>
      <div
        v-if="contextMenuPackage.installed && contextMenuPackage.installedVersion !== contextMenuPackage.version"
        class="context-menu-item"
        role="menuitem"
        tabindex="0"
        @click="contextMenuUpdate"
      >
        <span class="menu-icon">🔄</span>
        更新
      </div>
      <div class="context-menu-divider"></div>
      <div class="context-menu-item" role="menuitem" tabindex="0" @click="copyPackageName">
        <span class="menu-icon">📝</span>
        复制包名
      </div>
      <div class="context-menu-item" role="menuitem" tabindex="0" @click="copyPackageVersion">
        <span class="menu-icon">🔢</span>
        复制版本号
      </div>
    </div>

    <!-- 包详情对话框 -->
    <div v-if="showDetailsDialog && selectedPackage" class="dialog-overlay" role="dialog" aria-modal="true" :aria-labelledby="`dialog-title-${selectedPackage.name}`" @click.self="showDetailsDialog = false">
      <div class="dialog">
        <div class="dialog-header">
          <h2 :id="`dialog-title-${selectedPackage.name}`">{{ selectedPackage.name }}</h2>
          <button class="btn-close" aria-label="关闭对话框" @click="showDetailsDialog = false">✕</button>
        </div>
        
        <div class="dialog-content">
          <div class="detail-info">
            <p class="description">{{ selectedPackage.description }}</p>
            
            <div class="detail-meta">
              <div class="meta-item">
                <label>版本:</label>
                <span>{{ selectedPackage.version }}</span>
              </div>
              <div class="meta-item">
                <label>作者:</label>
                <span>{{ selectedPackage.author }}</span>
              </div>
              <div class="meta-item">
                <label>许可证:</label>
                <span>{{ selectedPackage.license }}</span>
              </div>
              <div class="meta-item">
                <label>主页:</label>
                <a :href="selectedPackage.homepage" target="_blank">{{ selectedPackage.homepage }}</a>
              </div>
              <div class="meta-item">
                <label>仓库:</label>
                <a :href="selectedPackage.repository" target="_blank">{{ selectedPackage.repository }}</a>
              </div>
            </div>
            
            <div class="detail-stats">
              <div class="stat-item">
                <span class="stat-value">{{ selectedPackage.rating }}</span>
                <span class="stat-label">评分</span>
              </div>
              <div class="stat-item">
                <span class="stat-value">{{ selectedPackage.downloads }}</span>
                <span class="stat-label">下载</span>
              </div>
            </div>
            
            <div class="detail-tags">
              <span v-for="keyword in selectedPackage.keywords" :key="keyword" class="tag">
                {{ keyword }}
              </span>
            </div>
            
            <div v-if="selectedPackage.dependencies.length > 0" class="detail-dependencies">
              <h4>依赖</h4>
              <div class="dependency-list">
                <span v-for="dep in selectedPackage.dependencies" :key="dep" class="dependency">
                  {{ dep }}
                </span>
              </div>
            </div>
            
            <div class="detail-status">
              <span v-if="selectedPackage.installed" class="status installed">
                ✅ 已安装 (v{{ selectedPackage.installedVersion }})
              </span>
              <span v-else class="status not-installed">
                ⏳ 未安装
              </span>
            </div>
          </div>
        </div>
        
        <div class="dialog-footer">
          <button
            v-if="!selectedPackage.installed"
            class="btn-primary"
            :disabled="isInstalling"
            :aria-busy="isInstalling"
            @click="installPackage(selectedPackage)"
          >
            {{ isInstalling ? '安装中...' : '安装' }}
          </button>
          <button
            v-else
            class="btn-danger"
            :disabled="isUninstalling"
            :aria-busy="isUninstalling"
            @click="uninstallPackage(selectedPackage)"
          >
            {{ isUninstalling ? '卸载中...' : '卸载' }}
          </button>
          <button
            v-if="selectedPackage.installed && selectedPackage.installedVersion !== selectedPackage.version"
            class="btn-secondary"
            :disabled="isUpdating"
            :aria-busy="isUpdating"
            @click="updatePackage(selectedPackage)"
          >
            {{ isUpdating ? '更新中...' : '更新' }}
          </button>
          <button class="btn-secondary" aria-label="关闭对话框" @click="showDetailsDialog = false">关闭</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.typst-package-browser {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-primary, #ffffff);
}

.browser-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px;
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.browser-header h2 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary, #333333);
}

.header-actions {
  display: flex;
  gap: 12px;
  align-items: center;
  flex-wrap: wrap;
}

.error-message,
.success-message {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-radius: 6px;
  margin-top: 12px;
  font-size: 14px;
}

.error-message {
  background: var(--error-bg, #ffebee);
  color: var(--error-color, #f44336);
  border: 1px solid var(--error-color, #f44336);
}

.success-message {
  background: var(--success-bg, #e8f5e9);
  color: var(--success-color, #4caf50);
  border: 1px solid var(--success-color, #4caf50);
}

.btn-close-error,
.btn-close-success {
  background: none;
  border: none;
  font-size: 18px;
  cursor: pointer;
  padding: 0 4px;
  margin-left: 8px;
}

.search-box {
  position: relative;
}

.search-input {
  padding: 8px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 6px;
  font-size: 14px;
  width: 250px;
}

.btn-refresh {
  padding: 8px 16px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 6px;
  background: var(--bg-primary, #ffffff);
  cursor: pointer;
  transition: all 0.2s;
}

.btn-refresh:hover:not(:disabled) {
  background: var(--bg-secondary, #f5f5f5);
}

.btn-refresh:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.browser-filters {
  display: flex;
  gap: 20px;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color, #e0e0e0);
  align-items: center;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.filter-group label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary, #666666);
}

.filter-select {
  padding: 6px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 4px;
  font-size: 14px;
  background: var(--bg-primary, #ffffff);
}

.filter-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 4px;
  background: var(--bg-primary, #ffffff);
  cursor: pointer;
  transition: all 0.2s;
}

.filter-btn:hover {
  background: var(--bg-secondary, #f5f5f5);
}

.filter-btn.active {
  background: var(--primary-color, #007bff);
  color: white;
  border-color: var(--primary-color, #007bff);
}

.packages-container {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.loading-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-secondary, #666666);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--border-color, #e0e0e0);
  border-top-color: var(--primary-color, #007bff);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 16px;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.packages-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
}

.package-card {
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 8px;
  padding: 16px;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}

.package-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.package-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.package-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #333333);
}

.version {
  font-size: 12px;
  color: var(--text-secondary, #666666);
  background: var(--bg-secondary, #f5f5f5);
  padding: 2px 6px;
  border-radius: 4px;
}

.description {
  margin: 0 0 12px 0;
  font-size: 13px;
  color: var(--text-secondary, #666666);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.package-meta {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: var(--text-secondary, #666666);
  margin-bottom: 12px;
}

.package-stats {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: var(--text-secondary, #666666);
  margin-bottom: 12px;
}

.package-tags {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
  margin-bottom: 12px;
}

.tag {
  padding: 2px 8px;
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 12px;
  font-size: 11px;
  color: var(--text-secondary, #666666);
}

.package-actions {
  display: flex;
  gap: 8px;
}

.btn-install,
.btn-uninstall,
.btn-update {
  flex: 1;
  padding: 6px 12px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 4px;
  background: var(--bg-primary, #ffffff);
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}

.btn-install:hover {
  background: var(--primary-color, #007bff);
  color: white;
  border-color: var(--primary-color, #007bff);
}

.btn-uninstall:hover {
  background: var(--error-color, #f44336);
  color: white;
  border-color: var(--error-color, #f44336);
}

.btn-update:hover {
  background: var(--warning-color, #ff9800);
  color: white;
  border-color: var(--warning-color, #ff9800);
}

.dialog-overlay {
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

.dialog {
  background: var(--bg-primary, #ffffff);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px;
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.dialog-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary, #333333);
}

.btn-close {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--text-secondary, #666666);
  padding: 4px 8px;
}

.dialog-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.detail-info {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.detail-meta {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.meta-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.meta-item label {
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.detail-stats {
  display: flex;
  gap: 24px;
}

.stat-item {
  text-align: center;
}

.stat-value {
  font-size: 24px;
  font-weight: 600;
  color: var(--primary-color, #007bff);
}

.stat-label {
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.detail-tags {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.detail-dependencies {
  margin-top: 16px;
}

.detail-dependencies h4 {
  margin: 0 0 8px 0;
  font-size: 14px;
  font-weight: 600;
}

.dependency-list {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.dependency {
  padding: 4px 8px;
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 4px;
  font-size: 12px;
  color: var(--text-secondary, #666666);
}

.detail-status {
  margin-top: 16px;
}

.status {
  padding: 8px 12px;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
}

.status.installed {
  background: var(--success-bg, #e8f5e9);
  color: var(--success-color, #4caf50);
}

.status.not-installed {
  background: var(--warning-bg, #fff3e0);
  color: var(--warning-color, #ff9800);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 20px;
  border-top: 1px solid var(--border-color, #e0e0e0);
}

.btn-primary,
.btn-secondary,
.btn-danger {
  padding: 8px 16px;
  border: 1px solid var(--border-color, #d0d0d0);
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.btn-primary {
  background: var(--primary-color, #007bff);
  color: white;
  border-color: var(--primary-color, #007bff);
}

.btn-primary:hover {
  background: var(--primary-dark, #0056b3);
}

.btn-secondary {
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #333333);
}

.btn-secondary:hover {
  background: var(--bg-secondary, #f5f5f5);
}

.btn-danger {
  background: var(--error-color, #f44336);
  color: white;
  border-color: var(--error-color, #f44336);
}

.btn-danger:hover {
  background: var(--error-dark, #d32f2f);
}

.context-menu {
  position: fixed;
  background: var(--bg-primary, #ffffff);
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
  z-index: 2000;
  min-width: 180px;
  padding: 4px 0;
  animation: contextMenuFadeIn 0.15s ease-out;
}

@keyframes contextMenuFadeIn {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  cursor: pointer;
  transition: background 0.2s;
  font-size: 14px;
  color: var(--text-primary, #333333);
}

.context-menu-item:hover {
  background: var(--bg-secondary, #f5f5f5);
}

.context-menu-item:focus {
  outline: 2px solid var(--primary-color, #007bff);
  outline-offset: -2px;
  background: var(--bg-secondary, #f5f5f5);
}

.context-menu-item:active {
  background: var(--primary-bg, #e3f2fd);
}

.menu-icon {
  font-size: 16px;
}

.context-menu-divider {
  height: 1px;
  background: var(--border-color, #e0e0e0);
  margin: 4px 0;
}
</style>
