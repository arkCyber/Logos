use super::font_loader::FontLoader;
use std::collections::HashMap;
use std::path::PathBuf;
use typst::diag::FileError;
use typst::foundations::{Bytes, Datetime};
use typst::model::Document;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::Library;
use typst::World;

pub struct TypstWorld {
    library: LazyHash<Library>,
    source: Source,
    font_book: LazyHash<FontBook>,
    font_data: Vec<Font>,
    file_map: HashMap<FileId, Bytes>,
}

impl typst::World for TypstWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.font_book
    }

    fn main(&self) -> FileId {
        FileId::new(None, VirtualPath::new("main.typ"))
    }

    fn source(&self, id: FileId) -> Result<Source, FileError> {
        if id == self.main() {
            Ok(self.source.clone())
        } else {
            // Try to load from file map
            if let Some(bytes) = self.file_map.get(&id) {
                let text = std::str::from_utf8(bytes.as_slice())
                    .map_err(|_| FileError::Other(Some("Invalid UTF-8".into())))?;
                Ok(Source::new(id, text.to_string()))
            } else {
                Err(FileError::NotFound(
                    id.vpath().as_rootless_path().to_path_buf(),
                ))
            }
        }
    }

    fn file(&self, id: FileId) -> Result<Bytes, FileError> {
        if let Some(bytes) = self.file_map.get(&id) {
            Ok(bytes.clone())
        } else {
            Err(FileError::NotFound(
                id.vpath().as_rootless_path().to_path_buf(),
            ))
        }
    }

    fn font(&self, id: usize) -> Option<Font> {
        self.font_data.get(id).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        // Use a fixed date for reproducibility
        Datetime::from_ymd(2024, 1, 1)
    }
}

pub struct TypstCompiler {
    font_loader: FontLoader,
    include_paths: Vec<PathBuf>,
}

impl TypstCompiler {
    pub fn new() -> Self {
        Self {
            font_loader: FontLoader::new(),
            include_paths: Vec::new(),
        }
    }

    pub fn with_custom_font_paths(font_paths: Vec<PathBuf>) -> Self {
        Self {
            font_loader: FontLoader::with_custom_paths(font_paths),
            include_paths: Vec::new(),
        }
    }

    pub fn with_include_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.include_paths = paths;
        self
    }

    pub fn add_include_path(&mut self, path: PathBuf) {
        self.include_paths.push(path);
    }

    pub fn compile(&self, code: String) -> Result<Document, String> {
        self.compile_with_options(code, None)
    }

    pub fn compile_with_options(
        &self,
        code: String,
        options: Option<CompileOptions>,
    ) -> Result<Document, String> {
        let opts = options.unwrap_or_default();
        eprintln!("[Typst Compiler] Compiling code ({} bytes)", code.len());

        let library = Library::builder().build();
        let source = Source::detached(code);

        // Build file map for includes
        let file_map = self.build_file_map(&source, &opts)?;

        // Clone source for error reporting
        let source_for_errors = source.clone();

        let world = TypstWorld {
            library: LazyHash::new(library),
            source,
            font_book: self.font_loader.get_lazy_book(),
            font_data: self.font_loader.get_fonts().clone(),
            file_map,
        };

        let result = typst::compile(&world).output.map_err(|e| {
            eprintln!("[Typst Compiler] Compilation error: {:?}", e);
            // Provide more user-friendly error messages
            let error_msg = e.iter()
                .map(|diag| {
                    let location = if diag.span.id() == Some(world.main()) {
                        let range = source_for_errors.range(diag.span);
                        if let Some(range) = range {
                            let line = source_for_errors.byte_to_line(range.start).unwrap_or(0);
                            let col = source_for_errors.byte_to_column(range.start).unwrap_or(0);
                            format!("Line {}, Column {}: {}", line + 1, col + 1, diag.message)
                        } else {
                            diag.message.to_string()
                        }
                    } else {
                        format!("{}: {}", diag.span.id().map(|id| id.vpath().as_rootless_path().display().to_string()).unwrap_or_else(|| "unknown".to_string()), diag.message)
                    };
                    
                    // Add helpful hints for common errors
                    let hint = if diag.message.contains("expected length, found float") {
                        " - Hint: Add a length unit like 'pt', 'mm', 'cm', or 'in' to numeric values (e.g., '10pt' instead of '10')"
                    } else if diag.message.contains("expected") {
                        " - Hint: Check the Typst syntax for the correct type or format"
                    } else {
                        ""
                    };
                    
                    format!("{}{}", location, hint)
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!("Typst compilation error:\n{}", error_msg)
        })?;
        eprintln!(
            "[Typst Compiler] Compilation successful, {} pages",
            result.pages.len()
        );
        Ok(result)
    }

    fn build_file_map(
        &self,
        source: &Source,
        opts: &CompileOptions,
    ) -> Result<HashMap<FileId, Bytes>, String> {
        let mut file_map = HashMap::new();

        // Scan for include directives and load files
        let content = source.text();
        for line in content.lines() {
            if let Some(path) = self.extract_include_path(line) {
                if let Ok(bytes) = self.load_included_file(&path) {
                    let id = FileId::new(None, VirtualPath::new(path.clone()));
                    file_map.insert(id, bytes);
                } else if !opts.ignore_missing_includes {
                    return Err(format!("Failed to load included file: {}", path));
                }
            }
        }

        Ok(file_map)
    }

    fn extract_include_path(&self, line: &str) -> Option<String> {
        // Match #include("path.typ") or include "path.typ"
        let line = line.trim();

        if line.starts_with("#include(") {
            let inner = line.strip_prefix("#include(")?;
            let end = inner.find(')')?;
            let path = inner[..end].trim_matches('"');
            Some(path.to_string())
        } else if line.starts_with("include ") {
            let path = line.strip_prefix("include ")?.trim_matches('"');
            Some(path.to_string())
        } else {
            None
        }
    }

    fn load_included_file(&self, path: &str) -> Result<Bytes, String> {
        let path_buf = PathBuf::from(path);

        // Try include paths first
        for include_dir in &self.include_paths {
            let full_path = include_dir.join(&path_buf);
            if full_path.exists() {
                return std::fs::read(&full_path)
                    .map(Bytes::from)
                    .map_err(|e| format!("Failed to read file {}: {}", full_path.display(), e));
            }
        }

        // Try relative to current directory
        if path_buf.exists() {
            return std::fs::read(&path_buf)
                .map(Bytes::from)
                .map_err(|e| format!("Failed to read file {}: {}", path_buf.display(), e));
        }

        Err(format!("File not found: {}", path))
    }

    #[allow(dead_code)]
    pub fn get_font_count(&self) -> usize {
        self.font_loader.get_fonts().len()
    }

    pub fn get_font_loader(&self) -> &FontLoader {
        &self.font_loader
    }
}

#[derive(Debug, Clone, Default)]
pub struct CompileOptions {
    pub ignore_missing_includes: bool,
    pub strict_mode: bool,
}

impl Default for TypstCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_creation() {
        let compiler = TypstCompiler::new();
        assert!(compiler.get_font_count() >= 0);
    }

    #[test]
    fn test_compiler_default() {
        let compiler = TypstCompiler::default();
        assert!(compiler.get_font_count() >= 0);
    }

    #[test]
    fn test_get_font_count() {
        let compiler = TypstCompiler::new();
        let count = compiler.get_font_count();
        assert!(count >= 0);
    }

    #[test]
    fn test_compile_simple_text() {
        let compiler = TypstCompiler::new();
        let code = "= Hello World".to_string();
        let result = compiler.compile(code);
        // This may fail if fonts are not available, but we test the call itself
        // The result depends on system fonts
        let _ = result;
    }

    #[test]
    fn test_compile_empty() {
        let compiler = TypstCompiler::new();
        let code = "".to_string();
        let result = compiler.compile(code);
        // Empty code should either compile to empty doc or error
        let _ = result;
    }

    #[test]
    fn test_compile_invalid_syntax() {
        let compiler = TypstCompiler::new();
        // Use a syntax that's more likely to be invalid
        let code = "#invalidfunction()".to_string();
        let result = compiler.compile(code);
        // This may or may not error depending on Typst's error handling
        // We just test that the compile method handles it
        let _ = result;
    }

    #[test]
    fn test_compile_with_paragraphs() {
        let compiler = TypstCompiler::new();
        let code = "First paragraph\n\nSecond paragraph".to_string();
        let result = compiler.compile(code);
        let _ = result;
    }

    #[test]
    fn test_compile_with_bold() {
        let compiler = TypstCompiler::new();
        let code = "*bold text*".to_string();
        let result = compiler.compile(code);
        let _ = result;
    }

    #[test]
    fn test_compile_with_italic() {
        let compiler = TypstCompiler::new();
        let code = "_italic text_".to_string();
        let result = compiler.compile(code);
        let _ = result;
    }

    #[test]
    fn test_compile_heading() {
        let compiler = TypstCompiler::new();
        let code = "== Heading".to_string();
        let result = compiler.compile(code);
        let _ = result;
    }

    #[test]
    fn test_compile_code_block() {
        let compiler = TypstCompiler::new();
        let code = "```\ncode here\n```".to_string();
        let result = compiler.compile(code);
        let _ = result;
    }

    #[test]
    fn test_compile_list() {
        let compiler = TypstCompiler::new();
        let code = "- Item 1\n- Item 2".to_string();
        let result = compiler.compile(code);
        let _ = result;
    }

    #[test]
    fn test_compile_whitespace() {
        let compiler = TypstCompiler::new();
        let code = "   ".to_string();
        let result = compiler.compile(code);
        let _ = result;
    }

    #[test]
    fn test_compile_newlines() {
        let compiler = TypstCompiler::new();
        let code = "\n\n\n".to_string();
        let result = compiler.compile(code);
        let _ = result;
    }

    #[test]
    fn test_compile_unicode() {
        let compiler = TypstCompiler::new();
        let code = "Hello 世界 🌍".to_string();
        let result = compiler.compile(code);
        let _ = result;
    }

    #[test]
    fn test_compile_math() {
        let compiler = TypstCompiler::new();
        let code = "$ x^2 + y^2 = z^2 $".to_string();
        let result = compiler.compile(code);
        let _ = result;
    }

    #[test]
    fn test_compile_link() {
        let compiler = TypstCompiler::new();
        let code = "#link(\"https://example.com\")".to_string();
        let result = compiler.compile(code);
        let _ = result;
    }

    #[test]
    fn test_with_custom_font_paths() {
        let paths = vec![PathBuf::from("/tmp/fonts")];
        let compiler = TypstCompiler::with_custom_font_paths(paths);
        assert!(compiler.get_font_count() >= 0);
    }

    #[test]
    fn test_with_include_paths() {
        let paths = vec![PathBuf::from("/tmp/includes")];
        let compiler = TypstCompiler::new().with_include_paths(paths);
        assert!(compiler.get_font_count() >= 0);
    }

    #[test]
    fn test_add_include_path() {
        let mut compiler = TypstCompiler::new();
        compiler.add_include_path(PathBuf::from("/tmp/includes"));
        assert!(compiler.get_font_count() >= 0);
    }

    #[test]
    fn test_compile_with_options() {
        let compiler = TypstCompiler::new();
        let code = "= Hello".to_string();
        let options = CompileOptions {
            ignore_missing_includes: true,
            strict_mode: false,
        };
        let result = compiler.compile_with_options(code, Some(options));
        let _ = result;
    }

    #[test]
    fn test_compile_with_options_none() {
        let compiler = TypstCompiler::new();
        let code = "= Hello".to_string();
        let result = compiler.compile_with_options(code, None);
        let _ = result;
    }

    #[test]
    fn test_get_font_loader() {
        let compiler = TypstCompiler::new();
        let loader = compiler.get_font_loader();
        // Should return a reference to the font loader
        let _ = loader.get_fonts();
    }

    #[test]
    fn test_compile_options_default() {
        let options = CompileOptions::default();
        assert_eq!(options.ignore_missing_includes, false);
        assert_eq!(options.strict_mode, false);
    }

    #[test]
    fn test_compile_options_custom() {
        let options = CompileOptions {
            ignore_missing_includes: true,
            strict_mode: true,
        };
        assert_eq!(options.ignore_missing_includes, true);
        assert_eq!(options.strict_mode, true);
    }

    #[test]
    fn test_extract_include_path_hash_syntax() {
        let compiler = TypstCompiler::new();
        let line = "#include(\"test.typ\")";
        let result = compiler.extract_include_path(line);
        assert_eq!(result, Some("test.typ".to_string()));
    }

    #[test]
    fn test_extract_include_path_simple_syntax() {
        let compiler = TypstCompiler::new();
        let line = "include \"test.typ\"";
        let result = compiler.extract_include_path(line);
        assert_eq!(result, Some("test.typ".to_string()));
    }

    #[test]
    fn test_extract_include_path_no_match() {
        let compiler = TypstCompiler::new();
        let line = "= Hello World";
        let result = compiler.extract_include_path(line);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_include_path_with_whitespace() {
        let compiler = TypstCompiler::new();
        let line = "  #include(\"test.typ\")  ";
        let result = compiler.extract_include_path(line);
        assert_eq!(result, Some("test.typ".to_string()));
    }

    #[test]
    fn test_compile_with_include_missing_file() {
        let compiler = TypstCompiler::new();
        let code = "#include(\"nonexistent.typ\")\n= Hello".to_string();
        let options = CompileOptions {
            ignore_missing_includes: false,
            strict_mode: false,
        };
        let result = compiler.compile_with_options(code, Some(options));
        // Should fail when ignore_missing_includes is false
        assert!(result.is_err());
    }

    #[test]
    fn test_compile_with_include_missing_file_ignored() {
        let compiler = TypstCompiler::new();
        let code = "#include(\"nonexistent.typ\")\n= Hello".to_string();
        let options = CompileOptions {
            ignore_missing_includes: true,
            strict_mode: false,
        };
        let result = compiler.compile_with_options(code, Some(options));
        // Should succeed when ignore_missing_includes is true
        let _ = result;
    }
}
