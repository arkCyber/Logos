# Typst Service Architecture Documentation

## Overview

The Typst Service is an aerospace-grade typesetting microservice built with Rust, providing professional document rendering capabilities with advanced typography, layout control, and color management features.

## System Architecture

### Component Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     Tauri Application                       │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              Typist Service Module                    │   │
│  ├──────────────────────────────────────────────────────┤   │
│  │                                                      │   │
│  │  ┌──────────────┐  ┌──────────────┐                 │   │
│  │  │ Typography   │  │ Grid System  │                 │   │
│  │  │   Engine     │  │   Engine     │                 │   │
│  │  └──────────────┘  └──────────────┘                 │   │
│  │                                                      │   │
│  │  ┌──────────────┐  ┌──────────────┐                 │   │
│  │  │ CJK          │  │ Color        │                 │   │
│  │  │ Typography   │  │ Management   │                 │   │
│  │  └──────────────┘  └──────────────┘                 │   │
│  │                                                      │   │
│  │  ┌──────────────┐  ┌──────────────┐                 │   │
│  │  │ Master Page  │  │ Incremental   │                 │   │
│  │  │   System     │  │  Compiler    │                 │   │
│  │  └──────────────┘  └──────────────┘                 │   │
│  │                                                      │   │
│  │  ┌──────────────┐  ┌──────────────┐                 │   │
│  │  │  Compiler    │  │  Renderer    │                 │   │
│  │  └──────────────┘  └──────────────┘                 │   │
│  │                                                      │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## Module Architecture

### Core Modules

#### Typography Module (`typography.rs`)

**Purpose**: Advanced font typesetting with kerning, OpenType features, and font pairing.

**Key Components**:
- `KerningTable`: Stores and manages kerning pairs
- `OpenTypeFeatures`: Controls OpenType feature toggling
- `FontPairingSystem`: Provides font pairing suggestions
- `TypographyEngine`: Main typography processing engine

**Data Flow**:
```
Input Text → TypographyEngine → Apply Kerning → Apply OpenType → Output
```

**Thread Safety**: Read-only operations, no shared state

#### Grid System Module (`grid_system.rs`)

**Purpose**: Professional layout control with baseline grids and layout balancing.

**Key Components**:
- `GridSystem`: Manages grid snapping and alignment
- `LayoutBalanceEngine`: Detects and fixes widows/orphans
- `ColumnWidthOptimizer`: Calculates optimal column widths
- `LayoutSystem`: Integrated layout management

**Data Flow**:
```
Layout Data → LayoutSystem → Grid Snapping → Balance Check → Output
```

**Thread Safety**: Read-only operations, no shared state

#### CJK Typography Module (`cjk_typography.rs`)

**Purpose**: Enhanced CJK typesetting with punctuation compression and line break rules.

**Key Components**:
- `CJKTypographyEngine`: Main CJK typography processing
- `PunctuationCompressionRule`: Rules for punctuation width adjustment
- `LineBreakRule`: Rules for line break avoidance

**Data Flow**:
```
CJK Text → CJKTypographyEngine → Punctuation Compression → Line Break Check → Output
```

**Thread Safety**: Read-only operations, no shared state

#### Color Management Module (`color_management.rs`)

**Purpose**: Professional color management with CMYK, Pantone, and ICC profile support.

**Key Components**:
- `ColorManagementSystem`: Main color management engine
- `RGBColor`, `CMYKColor`: Color space representations
- `PantoneColor`: Pantone color library
- `ICCProfile`: ICC color profile management

**Data Flow**:
```
Color Input → ColorManagementSystem → Space Conversion → Output
```

**Thread Safety**: Read-only operations, no shared state

#### Master Page Module (`master_page.rs`)

**Purpose**: Template page management with style inheritance.

**Key Components**:
- `MasterPageSystem`: Master page management
- `MasterPage`: Template page definition
- `PageInstance`: Page instance with custom elements

**Data Flow**:
```
Master Page → Page Instance → Apply Custom Elements → Output
```

**Thread Safety**: Read-only operations, no shared state

#### Incremental Compiler Module (`incremental.rs`)

**Purpose**: Thread-safe incremental compilation with caching and parallel processing.

**Key Components**:
- `IncrementalCompiler`: Thread-safe compilation manager
- `CacheEntry`: Cache entry with hash and dependencies
- `IncrementalConfig`: Configuration for incremental compilation

**Data Flow**:
```
Input → Hash Check → Cache Lookup → [Hit: Return Cache / Miss: Compile] → Update Cache → Output
```

**Thread Safety**: Uses `Arc<RwLock<HashMap>>` for thread-safe cache access

### Supporting Modules

#### Compiler Module (`compiler.rs`)

**Purpose**: Compiles Typst source code into a document.

**Key Components**:
- `TypstCompiler`: Main compiler
- `TypstWorld`: Manages fonts and file includes

**Thread Safety**: Not thread-safe, create separate instances per thread

#### Renderer Module (`renderer.rs`)

**Purpose**: Renders compiled documents to PDF or PNG.

**Key Components**:
- `TypstRenderer`: Main renderer
- `RenderOptions`: Rendering configuration

**Thread Safety**: Not thread-safe, create separate instances per thread

#### Fonts Module (`fonts.rs`)

**Purpose**: Manages font loading and configuration.

**Key Components**:
- `FontLoader`: Loads fonts from system paths
- `FontConfig`: Font configuration

**Thread Safety**: Read-only operations after initialization

## Data Structures

### Typography Data

```rust
pub struct KerningPair {
    pub left: char,
    pub right: char,
    pub adjustment: f64,
}

pub struct FontPairing {
    pub header_font: String,
    pub body_font: String,
    pub accent_font: String,
}
```

### Layout Data

```rust
pub struct GridConfig {
    pub grid_type: GridType,
    pub spacing: f64,
    pub origin: (f64, f64),
}

pub struct LayoutBalanceConfig {
    pub avoid_widows: bool,
    pub avoid_orphans: bool,
    pub minimum_lines_at_bottom: usize,
    pub minimum_lines_at_top: usize,
}
```

### Color Data

```rust
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

pub struct CMYKColor {
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
}
```

### Cache Data

```rust
pub struct CacheEntry {
    pub hash: String,
    pub timestamp: DateTime<Utc>,
    pub dependencies: Vec<String>,
    pub compiled_output: Vec<u8>,
}
```

## Concurrency Model

### Thread Safety Strategies

1. **Immutable Data**: Most modules use immutable data structures
2. **Arc<RwLock>**: Incremental compiler uses thread-safe cache
3. **Clone on Write**: Expensive operations clone data before modification
4. **Message Passing**: Parallel compilation uses channels for communication

### Parallel Processing

```rust
pub fn parallel_compile<F>(&self, tasks: Vec<(String, String)>, compile_fn: F) 
    -> Vec<Result<Vec<u8>, String>>
where
    F: Fn(String) -> Result<Vec<u8>, String> + Send + Sync + Clone + 'static,
```

- Tasks are chunked based on `max_parallel_jobs`
- Each chunk runs in a separate thread
- Results are collected via channels
- Cache is shared via `Arc<RwLock>`

## Error Handling

### Error Types

```rust
pub enum TypstError {
    CompilationError(String),
    RenderingError(String),
    FontError(String),
    ValidationError(String),
}
```

### Error Handling Strategy

1. **Result Types**: All functions return `Result<T, String>`
2. **Error Context**: Errors include descriptive messages
3. **Fallback Strategies**: Graceful degradation where possible
4. **Logging**: Errors are logged for debugging

## Performance Optimization

### Caching Strategy

1. **Content Hashing**: SHA256 for content validation
2. **TTL-based Expiration**: Cache entries expire after 24 hours
3. **Size Limits**: Maximum cache size of 1GB
4. **LRU Eviction**: Oldest entries removed when limit exceeded

### Incremental Compilation

1. **Dependency Tracking**: Track file dependencies
2. **Hash Comparison**: Only recompile if content changed
3. **Hot Reload**: Automatic detection of changes
4. **Parallel Builds**: Multiple documents compiled simultaneously

### Memory Management

1. **Lazy Loading**: Fonts loaded on demand
2. **Resource Pooling**: Reuse expensive resources
3. **Buffer Reuse**: Reuse buffers where possible
4. **Drop Guarantees**: RAII for resource cleanup

## Security

### Input Validation

1. **Size Limits**: Maximum input size of 10MB
2. **Format Validation**: Validate input format before processing
3. **Sanitization**: Sanitize HTML and other user input
4. **Bounds Checking**: All array operations are bounds-checked

### Resource Management

1. **Memory Limits**: Prevent memory exhaustion
2. **CPU Limits**: Limit CPU usage for compilation
3. **File Access**: Restricted file system access
4. **Timeout Protection**: Operations timeout after reasonable time

## Testing Strategy

### Unit Tests

Each module includes comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_kerning_table_creation() {
        let table = KerningTable::new();
        assert!(table.pairs.is_empty());
    }
}
```

### Integration Tests

Integration tests verify module interactions:

```rust
#[test]
fn test_typography_integration() {
    let engine = TypographyEngine::new();
    let result = engine.apply_kerning("Test");
    assert!(!result.is_empty());
}
```

### Performance Tests

Performance tests measure compilation speed:

```rust
#[test]
fn test_compilation_performance() {
    let start = Instant::now();
    let result = compiler.compile(large_document);
    let duration = start.elapsed();
    assert!(duration < Duration::from_secs(5));
}
```

## Configuration

### Environment Variables

- `TYPST_FONT_PATH`: Path to font directory
- `TYPST_CACHE_DIR`: Cache directory for incremental compilation
- `TYPST_MAX_CACHE_SIZE_MB`: Maximum cache size in MB
- `TYPST_MAX_INPUT_SIZE_MB`: Maximum input size in MB

### Default Configuration

```rust
impl Default for IncrementalConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_dir: PathBuf::from(".typst_cache"),
            max_cache_size_mb: 1024,
            cache_ttl_seconds: 86400,
            parallel_compilation: true,
            max_parallel_jobs: 4,
            hot_reload: true,
        }
    }
}
```

## Deployment

### Build Process

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

### Production Considerations

1. **Release Mode**: Always use release builds for production
2. **Static Linking**: Use static linking for portability
3. **Resource Limits**: Configure appropriate resource limits
4. **Monitoring**: Enable performance monitoring

## Maintenance

### Code Quality

1. **Rust Clippy**: Run `cargo clippy` for linting
2. **Rust Format**: Run `cargo fmt` for consistent formatting
3. **Documentation**: Keep documentation up to date
4. **Tests**: Maintain high test coverage

### Versioning

Follow semantic versioning:
- Major: Breaking changes
- Minor: New features
- Patch: Bug fixes

## Future Enhancements

### Planned Features

1. **GPU Acceleration**: Use GPU for rendering
2. **WebAssembly**: Compile to WASM for web use
3. **Real-time Preview**: Live preview during editing
4. **Advanced Layout**: More layout algorithms
5. **Font Management**: Better font management UI

### Research Areas

1. **Machine Learning**: AI-assisted typography
2. **Accessibility**: Enhanced accessibility features
3. **Internationalization**: Better i18n support
4. **Performance**: Further performance optimizations

## References

- [Typst Documentation](https://typst.app/docs/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## License

Proprietary - Aerospace Grade License
