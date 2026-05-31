use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// 包信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Vec<String>,
    pub typst_version: String,
    pub dependencies: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub downloads: u64,
}

/// 包仓库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageRepository {
    pub name: String,
    pub url: String,
    pub enabled: bool,
}

impl Default for PackageRepository {
    fn default() -> Self {
        Self {
            name: "typst-universe".to_string(),
            url: "https://packages.typst.org".to_string(),
            enabled: true,
        }
    }
}

/// 包管理器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManagerConfig {
    pub repositories: Vec<PackageRepository>,
    pub cache_dir: PathBuf,
    pub install_dir: PathBuf,
    pub auto_update: bool,
}

impl Default for PackageManagerConfig {
    fn default() -> Self {
        Self {
            repositories: vec![PackageRepository::default()],
            cache_dir: PathBuf::from(".typst_packages/cache"),
            install_dir: PathBuf::from(".typst_packages/install"),
            auto_update: true,
        }
    }
}

/// 包管理器
pub struct PackageManager {
    #[allow(dead_code)]
    config: PackageManagerConfig,
    installed_packages: HashMap<String, PackageInfo>,
    cached_packages: HashMap<String, PackageInfo>,
}

impl PackageManager {
    pub fn new(config: PackageManagerConfig) -> Self {
        Self {
            config,
            installed_packages: HashMap::new(),
            cached_packages: HashMap::new(),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(PackageManagerConfig::default())
    }

    /// 搜索包
    pub fn search(&self, query: &str) -> Vec<&PackageInfo> {
        let query_lower = query.to_lowercase();
        self.cached_packages
            .values()
            .filter(|pkg| {
                pkg.name.to_lowercase().contains(&query_lower)
                    || pkg.description.to_lowercase().contains(&query_lower)
                    || pkg
                        .keywords
                        .iter()
                        .any(|k| k.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// 获取包信息
    pub fn get_package(&self, name: &str) -> Option<&PackageInfo> {
        self.cached_packages.get(name)
    }

    /// 安装包
    pub fn install(&mut self, name: &str, version: Option<&str>) -> Result<String, String> {
        if let Some(pkg) = self.cached_packages.get(name) {
            let version_to_install = version.unwrap_or(&pkg.version);

            // 检查是否已安装
            if let Some(installed) = self.installed_packages.get(name) {
                if installed.version == version_to_install {
                    return Ok(format!(
                        "Package {}-{} is already installed",
                        name, version_to_install
                    ));
                }
            }

            // 模拟安装过程
            let installed_pkg = pkg.clone();
            self.installed_packages
                .insert(name.to_string(), installed_pkg);

            Ok(format!(
                "Successfully installed {}-{}",
                name, version_to_install
            ))
        } else {
            Err(format!("Package '{}' not found", name))
        }
    }

    /// 卸载包
    pub fn uninstall(&mut self, name: &str) -> Result<String, String> {
        if self.installed_packages.remove(name).is_some() {
            Ok(format!("Successfully uninstalled {}", name))
        } else {
            Err(format!("Package '{}' is not installed", name))
        }
    }

    /// 更新包
    pub fn update(&mut self, name: &str) -> Result<String, String> {
        if let Some(_installed) = self.installed_packages.get(name) {
            if let Some(latest) = self.cached_packages.get(name) {
                let updated_pkg = latest.clone();
                self.installed_packages
                    .insert(name.to_string(), updated_pkg);
                Ok(format!(
                    "Successfully updated {} to {}",
                    name, latest.version
                ))
            } else {
                Err(format!("Package '{}' not found in repository", name))
            }
        } else {
            Err(format!("Package '{}' is not installed", name))
        }
    }

    /// 列出已安装的包
    pub fn list_installed(&self) -> Vec<&PackageInfo> {
        self.installed_packages.values().collect()
    }

    /// 列出可用的包
    pub fn list_available(&self) -> Vec<&PackageInfo> {
        self.cached_packages.values().collect()
    }

    /// 添加包到缓存
    pub fn add_to_cache(&mut self, package: PackageInfo) {
        self.cached_packages.insert(package.name.clone(), package);
    }

    /// 检查依赖
    pub fn check_dependencies(&self, name: &str) -> Result<Vec<String>, String> {
        if let Some(pkg) = self.cached_packages.get(name) {
            let mut missing = Vec::new();
            for (dep_name, dep_version) in &pkg.dependencies {
                if !self.installed_packages.contains_key(dep_name) {
                    missing.push(format!("{}-{}", dep_name, dep_version));
                }
            }
            if missing.is_empty() {
                Ok(vec![])
            } else {
                Ok(missing)
            }
        } else {
            Err(format!("Package '{}' not found", name))
        }
    }

    /// 获取包统计信息
    pub fn get_stats(&self) -> PackageStats {
        PackageStats {
            total_packages: self.cached_packages.len(),
            installed_packages: self.installed_packages.len(),
            total_downloads: self.cached_packages.values().map(|p| p.downloads).sum(),
        }
    }
}

/// 包统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageStats {
    pub total_packages: usize,
    pub installed_packages: usize,
    pub total_downloads: u64,
}

impl Default for PackageManager {
    fn default() -> Self {
        Self::with_default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_manager_default() {
        let manager = PackageManager::default();
        assert_eq!(manager.config.repositories.len(), 1);
        assert!(manager.config.auto_update);
    }

    #[test]
    fn test_add_to_cache() {
        let mut manager = PackageManager::default();
        let package = PackageInfo {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Test package".to_string(),
            author: "Test Author".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: vec!["test".to_string()],
            typst_version: "0.12.0".to_string(),
            dependencies: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            downloads: 100,
        };

        manager.add_to_cache(package);
        assert!(manager.get_package("test-package").is_some());
    }

    #[test]
    fn test_search() {
        let mut manager = PackageManager::default();

        let package1 = PackageInfo {
            name: "math-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Math utilities".to_string(),
            author: "Test".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: vec!["math".to_string(), "utilities".to_string()],
            typst_version: "0.12.0".to_string(),
            dependencies: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            downloads: 100,
        };

        let package2 = PackageInfo {
            name: "text-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Text formatting".to_string(),
            author: "Test".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: vec!["text".to_string()],
            typst_version: "0.12.0".to_string(),
            dependencies: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            downloads: 50,
        };

        manager.add_to_cache(package1);
        manager.add_to_cache(package2);

        let results = manager.search("math");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "math-package");
    }

    #[test]
    fn test_install() {
        let mut manager = PackageManager::default();

        let package = PackageInfo {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            author: "Test".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: vec![],
            typst_version: "0.12.0".to_string(),
            dependencies: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            downloads: 100,
        };

        manager.add_to_cache(package);

        let result = manager.install("test-package", None);
        assert!(result.is_ok());

        let installed = manager.list_installed();
        assert_eq!(installed.len(), 1);
    }

    #[test]
    fn test_install_not_found() {
        let mut manager = PackageManager::default();
        let result = manager.install("nonexistent", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_uninstall() {
        let mut manager = PackageManager::default();

        let package = PackageInfo {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            author: "Test".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: vec![],
            typst_version: "0.12.0".to_string(),
            dependencies: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            downloads: 100,
        };

        manager.add_to_cache(package);
        manager.install("test-package", None).unwrap();

        let result = manager.uninstall("test-package");
        assert!(result.is_ok());

        let installed = manager.list_installed();
        assert_eq!(installed.len(), 0);
    }

    #[test]
    fn test_check_dependencies() {
        let mut manager = PackageManager::default();

        let mut deps = HashMap::new();
        deps.insert("dep1".to_string(), "1.0.0".to_string());

        let package = PackageInfo {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            author: "Test".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: vec![],
            typst_version: "0.12.0".to_string(),
            dependencies: deps,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            downloads: 100,
        };

        manager.add_to_cache(package);

        let missing = manager.check_dependencies("test-package").unwrap();
        assert_eq!(missing.len(), 1);
        assert!(missing[0].contains("dep1"));
    }

    #[test]
    fn test_get_stats() {
        let mut manager = PackageManager::default();

        let package = PackageInfo {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            author: "Test".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            keywords: vec![],
            typst_version: "0.12.0".to_string(),
            dependencies: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            downloads: 100,
        };

        manager.add_to_cache(package);

        let stats = manager.get_stats();
        assert_eq!(stats.total_packages, 1);
        assert_eq!(stats.installed_packages, 0);
        assert_eq!(stats.total_downloads, 100);
    }
}
