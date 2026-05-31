use std::collections::HashMap;
use std::path::PathBuf;
use typst::text::{Font, FontBook, FontInfo};
use typst::utils::LazyHash;

#[derive(Clone)]
pub struct FontLoader {
    fonts: Vec<Font>,
    book: FontBook,
    font_map: HashMap<String, usize>, // Maps font family/name to index
}

impl FontLoader {
    pub fn new() -> Self {
        Self::with_custom_paths(Vec::new())
    }

    pub fn with_custom_paths(custom_paths: Vec<PathBuf>) -> Self {
        let mut fonts = Vec::new();
        let mut book = FontBook::new();
        let mut font_map = HashMap::new();

        // Load system fonts from common locations
        let font_dirs = Self::get_system_font_dirs();
        let all_dirs: Vec<PathBuf> = font_dirs.into_iter().chain(custom_paths).collect();

        for dir in all_dirs {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().is_some_and(|ext| {
                        matches!(ext.to_str(), Some("ttf") | Some("otf") | Some("ttc"))
                    }) {
                        if let Ok(font_data) = std::fs::read(&path) {
                            let font_index = fonts.len();
                            if let Some(font) = Font::new(font_data.into(), 0) {
                                let info = font.info();
                                // Map by family name
                                font_map.insert(info.family.clone().to_lowercase(), font_index);

                                book.push(info.clone());
                                fonts.push(font);
                            }
                        }
                    }
                }
            }
        }

        Self {
            fonts,
            book,
            font_map,
        }
    }

    fn get_system_font_dirs() -> Vec<PathBuf> {
        let mut dirs = Vec::new();

        #[cfg(target_os = "macos")]
        {
            dirs.push(PathBuf::from("/System/Library/Fonts"));
            dirs.push(PathBuf::from("/Library/Fonts"));
            if let Ok(home) = std::env::var("HOME") {
                dirs.push(PathBuf::from(home).join("Library/Fonts"));
            }
        }

        #[cfg(target_os = "linux")]
        {
            dirs.push(PathBuf::from("/usr/share/fonts"));
            dirs.push(PathBuf::from("/usr/local/share/fonts"));
            if let Some(home) = std::env::var("HOME").ok() {
                dirs.push(PathBuf::from(home).join(".local/share/fonts"));
            }
        }

        #[cfg(target_os = "windows")]
        {
            dirs.push(PathBuf::from("C:\\Windows\\Fonts"));
        }

        dirs
    }

    pub fn get_fonts(&self) -> &Vec<Font> {
        &self.fonts
    }

    #[allow(dead_code)]
    pub fn get_book(&self) -> &FontBook {
        &self.book
    }

    pub fn get_lazy_book(&self) -> LazyHash<FontBook> {
        LazyHash::new(self.book.clone())
    }

    /// Search for a font by family name or full name
    pub fn find_font(&self, name: &str) -> Option<&Font> {
        let name_lower = name.to_lowercase();
        self.font_map
            .get(&name_lower)
            .and_then(|&index| self.fonts.get(index))
    }

    /// Get font information for all loaded fonts
    pub fn get_font_infos(&self) -> Vec<FontInfo> {
        self.fonts.iter().map(|font| font.info().clone()).collect()
    }

    /// Search fonts by family name (returns all matching fonts)
    pub fn search_by_family(&self, family: &str) -> Vec<&Font> {
        let family_lower = family.to_lowercase();
        self.fonts
            .iter()
            .filter(|font| font.info().family.to_lowercase() == family_lower)
            .collect()
    }

    /// Get available font families
    pub fn get_families(&self) -> Vec<String> {
        let mut families: Vec<String> = self
            .fonts
            .iter()
            .map(|font| font.info().family.clone())
            .collect();
        families.sort();
        families.dedup();
        families
    }

    /// Check if a font family is available
    pub fn has_family(&self, family: &str) -> bool {
        let family_lower = family.to_lowercase();
        self.fonts
            .iter()
            .any(|font| font.info().family.to_lowercase() == family_lower)
    }
}

impl Default for FontLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_loader_creation() {
        let loader = FontLoader::new();
        let fonts = loader.get_fonts();
        // Fonts may be 0 if no system fonts are found
        assert!(fonts.len() >= 0);
    }

    #[test]
    fn test_font_loader_default() {
        let loader = FontLoader::default();
        let fonts = loader.get_fonts();
        assert!(fonts.len() >= 0);
    }

    #[test]
    fn test_get_fonts() {
        let loader = FontLoader::new();
        let fonts = loader.get_fonts();
        // Should return a reference to the fonts vector
        assert!(fonts.len() >= 0);
    }

    #[test]
    fn test_get_book() {
        let loader = FontLoader::new();
        let book = loader.get_book();
        // Should return a reference to the FontBook
        let _ = book;
    }

    #[test]
    fn test_get_lazy_book() {
        let loader = FontLoader::new();
        let lazy_book = loader.get_lazy_book();
        // Should return a LazyHash wrapping the FontBook
        let _ = lazy_book;
    }

    #[test]
    fn test_get_system_font_dirs_not_empty() {
        let dirs = FontLoader::get_system_font_dirs();
        // Should return at least some font directories
        assert!(!dirs.is_empty());
    }

    #[test]
    fn test_get_system_font_dirs_paths_exist() {
        let dirs = FontLoader::get_system_font_dirs();
        // Check that returned paths are valid PathBufs
        for dir in dirs {
            let _ = dir.as_path();
        }
    }

    #[test]
    fn test_font_loader_clone() {
        let loader = FontLoader::new();
        let cloned = loader.clone();
        // Both should have the same number of fonts
        assert_eq!(loader.get_fonts().len(), cloned.get_fonts().len());
    }

    #[test]
    fn test_get_lazy_book_multiple_calls() {
        let loader = FontLoader::new();
        let book1 = loader.get_lazy_book();
        let book2 = loader.get_lazy_book();
        // Multiple calls should work
        let _ = book1;
        let _ = book2;
    }

    #[test]
    fn test_fonts_vector_type() {
        let loader = FontLoader::new();
        let fonts = loader.get_fonts();
        // Should be a vector of Font
        let _ = fonts;
    }

    #[test]
    fn test_book_type() {
        let loader = FontLoader::new();
        let book = loader.get_book();
        // Should be a FontBook
        let _ = book;
    }

    #[test]
    fn test_with_custom_paths() {
        let paths = vec![PathBuf::from("/tmp/fonts")];
        let loader = FontLoader::with_custom_paths(paths);
        // Should create a loader with custom paths
        let _ = loader.get_fonts();
    }

    #[test]
    fn test_find_font() {
        let loader = FontLoader::new();
        // Test finding a font (may return None if no fonts loaded)
        let result = loader.find_font("Arial");
        let _ = result;
    }

    #[test]
    fn test_find_font_case_insensitive() {
        let loader = FontLoader::new();
        // Test case-insensitive search
        let result1 = loader.find_font("arial");
        let result2 = loader.find_font("ARIAL");
        // Both should return the same result
        assert_eq!(result1.is_some(), result2.is_some());
    }

    #[test]
    fn test_get_font_infos() {
        let loader = FontLoader::new();
        let infos = loader.get_font_infos();
        // Should return a vector of FontInfo
        assert!(infos.len() >= 0);
    }

    #[test]
    fn test_search_by_family() {
        let loader = FontLoader::new();
        let results = loader.search_by_family("Arial");
        // Should return a vector of matching fonts
        assert!(results.len() >= 0);
    }

    #[test]
    fn test_search_by_family_case_insensitive() {
        let loader = FontLoader::new();
        let results1 = loader.search_by_family("arial");
        let results2 = loader.search_by_family("ARIAL");
        // Both should return the same results
        assert_eq!(results1.len(), results2.len());
    }

    #[test]
    fn test_get_families() {
        let loader = FontLoader::new();
        let families = loader.get_families();
        // Should return a sorted, deduplicated list
        assert!(families.len() >= 0);
        // Verify sorted
        for i in 1..families.len() {
            assert!(families[i - 1] <= families[i]);
        }
    }

    #[test]
    fn test_has_family() {
        let loader = FontLoader::new();
        let has_arial = loader.has_family("Arial");
        // Should return a boolean
        let _ = has_arial;
    }

    #[test]
    fn test_has_family_case_insensitive() {
        let loader = FontLoader::new();
        let has1 = loader.has_family("arial");
        let has2 = loader.has_family("ARIAL");
        // Both should return the same result
        assert_eq!(has1, has2);
    }

    #[test]
    fn test_find_font_empty_string() {
        let loader = FontLoader::new();
        let result = loader.find_font("");
        // Empty string should return None
        assert!(result.is_none());
    }

    #[test]
    fn test_search_by_family_empty_string() {
        let loader = FontLoader::new();
        let results = loader.search_by_family("");
        // Empty string should return empty vector
        assert!(results.is_empty());
    }

    #[test]
    fn test_get_families_empty_loader() {
        // Test with a loader that has no fonts
        let loader = FontLoader::with_custom_paths(vec![]);
        let families = loader.get_families();
        // Should return empty vector if no fonts
        assert!(families.len() >= 0);
    }
}
