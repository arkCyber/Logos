# Typst Service API Documentation

## Overview

The Typst Service provides aerospace-grade typesetting capabilities with advanced typography, layout control, and color management features.

## Modules

### Typography System (`typography.rs`)

Advanced font typesetting with kerning, OpenType features, and font pairing.

#### Structures

- **KerningTable**: Manages kerning pairs for character spacing
- **OpenTypeFeatures**: Controls OpenType feature toggling (ligatures, small caps, etc.)
- **FontPairingSystem**: Provides professional font pairing suggestions
- **TypographyConfig**: Configuration for typography engine
- **TypographyEngine**: Main typography processing engine

#### Example

```rust
use typist_service::{TypographyEngine, TypographyConfig, OpenTypeFeatures};

let config = TypographyConfig::new()
    .with_kerning(KerningTable::new())
    .with_opentype_features(OpenTypeFeatures::new());

let engine = TypographyEngine::new(config);
let result = engine.apply_kerning("Typst");
```

### Grid System (`grid_system.rs`)

Professional layout control with baseline grids, layout balancing, and column width optimization.

#### Structures

- **GridConfig**: Configuration for grid system
- **GridSystem**: Manages grid snapping and alignment
- **LayoutBalanceConfig**: Configuration for layout balance
- **LayoutBalanceEngine**: Detects and fixes widows/orphans
- **ColumnWidthConfig**: Configuration for column width optimization
- **ColumnWidthOptimizer**: Calculates optimal column widths
- **LayoutSystem**: Integrated layout management

#### Example

```rust
use typist_service::{LayoutSystem, LayoutSystemConfig, GridConfig};

let config = LayoutSystemConfig::new()
    .with_grid_config(GridConfig::default());

let system = LayoutSystem::new(config);
let width = system.calculate_column_widths(595.0); // A4 width
```

### CJK Typography (`cjk_typography.rs`)

Enhanced CJK typesetting with punctuation compression, line break rules, and vertical text support.

#### Structures

- **CJKLanguage**: Supported CJK languages (Simplified Chinese, Traditional Chinese, Japanese, Korean)
- **CJKTypographyConfig**: Configuration for CJK typography
- **CJKTypographyEngine**: Main CJK typography processing engine
- **PunctuationCompressionRule**: Rules for punctuation width adjustment
- **LineBreakRule**: Rules for line break avoidance

#### Example

```rust
use typist_service::{CJKTypographyEngine, CJKTypographyConfig, CJKLanguage};

let config = CJKTypographyConfig::new(CJKLanguage::SimplifiedChinese);
let engine = CJKTypographyEngine::new(config);
let result = engine.apply_punctuation_compression("测试，文本。");
```

### Color Management (`color_management.rs`)

Professional color management with CMYK, Pantone, and ICC profile support.

#### Structures

- **RGBColor**: RGB color representation
- **CMYKColor**: CMYK color representation
- **PantoneColor**: Pantone color library
- **ICCProfile**: ICC color profile management
- **ColorManagementConfig**: Configuration for color management
- **ColorManagementSystem**: Main color management engine

#### Example

```rust
use typist_service::{ColorManagementSystem, RGBColor, CMYKColor};

let system = ColorManagementSystem::default();
let rgb = RGBColor::new(255, 0, 0);
let cmyk = system.rgb_to_cmyk(&rgb);
```

### Master Page System (`master_page.rs`)

Template page management with style inheritance and page elements.

#### Structures

- **PageStyle**: Page margin and layout configuration
- **PageElement**: Page elements (text, image, page number, etc.)
- **MasterPage**: Template page definition
- **PageInstance**: Page instance with custom elements
- **MasterPageSystem**: Master page management

#### Example

```rust
use typist_service::{MasterPageSystem, MasterPage, PageElement, PageElementType};

let mut system = MasterPageSystem::new();
let master = MasterPage::new("Default".to_string());
system.register_master_page(master).unwrap();
let instance = system.create_page_instance("Default".to_string(), 1).unwrap();
```

### Incremental Compiler (`incremental.rs`)

Thread-safe incremental compilation with caching and parallel processing.

#### Structures

- **CacheEntry**: Cache entry with hash, timestamp, and dependencies
- **IncrementalConfig**: Configuration for incremental compilation
- **IncrementalCompiler**: Thread-safe incremental compilation manager

#### Features

- SHA256 hashing for content validation
- TTL-based cache expiration
- Cache size limits with automatic cleanup
- Parallel compilation support
- Hot reload detection

#### Example

```rust
use typist_service::{IncrementalCompiler, IncrementalConfig};

let config = IncrementalConfig::default();
let compiler = IncrementalCompiler::new(config);
let hash = compiler.compute_hash("content");
```

## Tauri Commands

### render_typst

Renders Typst source to PDF or PNG.

```json
{
  "source": "#set page(paper: \"a4\")\nHello, World!",
  "output_format": "pdf",
  "dpi": 300
}
```

### check_typst_availability

Checks if Typst is available on the system.

## Safety Features

All modules implement aerospace-grade safety standards:

- **Input Validation**: All inputs are validated for size and format
- **Bounds Checking**: All array operations are bounds-checked
- **Error Handling**: Comprehensive error handling with Result types
- **Resource Management**: Automatic cleanup of resources
- **Thread Safety**: Thread-safe implementations where applicable
- **No Panics**: Production code contains no panic! calls

## Performance Optimizations

- **Incremental Compilation**: Only recompile changed content
- **Parallel Processing**: Multi-threaded compilation support
- **Caching**: Efficient caching with TTL and size limits
- **Lazy Evaluation**: Deferred computation where beneficial

## Testing

All modules include comprehensive unit tests:

```bash
cargo test --lib typist_service
```

Test coverage:
- typography.rs: 16 tests
- grid_system.rs: 14 tests
- cjk_typography.rs: 12 tests
- color_management.rs: 20 tests
- master_page.rs: 20 tests
- incremental.rs: 9 tests

## Configuration

### Environment Variables

- `TYPST_FONT_PATH`: Path to font directory
- `TYPST_CACHE_DIR`: Cache directory for incremental compilation
- `TYPST_MAX_CACHE_SIZE_MB`: Maximum cache size in MB

### Default Values

- Maximum input size: 10MB
- Maximum cache size: 1GB
- Cache TTL: 24 hours
- Parallel jobs: 4

## Error Handling

All functions return `Result<T, String>` for error handling. Error messages are descriptive and include context for debugging.

## License

Proprietary - Aerospace Grade License
