# Typst Service User Guide

## Introduction

The Typst Service provides professional-grade typesetting capabilities for the Logos application. This guide will help you understand and use the advanced typography features.

## Getting Started

### Basic Typst Rendering

```rust
use typist_service::{TypstCompiler, RenderOptions};

let compiler = TypstCompiler::new();
let options = RenderOptions::default();
let result = compiler.render("Hello, World!", &options);
```

## Advanced Typography

### Font Kerning

Kerning adjusts the spacing between character pairs for better visual appearance.

```rust
use typist_service::{TypographyEngine, KerningTable, KerningPair};

let mut kerning_table = KerningTable::new();
kerning_table.add_pair(KerningPair {
    left: 'A',
    right: 'V',
    adjustment: -0.05,
});

let engine = TypographyEngine::new().with_kerning(kerning_table);
let result = engine.apply_kerning("AVATAR");
```

### OpenType Features

Enable advanced font features like ligatures and small caps.

```rust
use typist_service::{OpenTypeFeatures, OpenTypeFeature};

let mut features = OpenTypeFeatures::new();
features.enable_feature(OpenTypeFeature::Ligatures);
features.enable_feature(OpenTypeFeature::SmallCaps);

let engine = TypographyEngine::new().with_opentype_features(features);
```

### Font Pairing

Get professional font pairing suggestions.

```rust
use typist_service::{FontPairingSystem, FontPairing};

let system = FontPairingSystem::new();
let pairing = system.get_pairing(0).unwrap();
println!("Header: {}, Body: {}", pairing.header_font, pairing.body_font);
```

## Layout Control

### Grid System

Align elements to a baseline grid for consistent spacing.

```rust
use typist_service::{GridSystem, GridConfig, GridType};

let config = GridConfig::new()
    .with_grid_type(GridType::Baseline)
    .with_spacing(12.0);

let system = GridSystem::new(config);
let snapped = system.snap_to_grid(100.0);
```

### Layout Balance

Avoid widows and orphans for professional-looking documents.

```rust
use typist_service::{LayoutBalanceEngine, LayoutBalanceConfig};

let config = LayoutBalanceConfig::new()
    .with_avoid_widows(true)
    .with_avoid_orphans(true);

let engine = LayoutBalanceEngine::new(config);
let is_widow = engine.detect_widow(100, 1);
```

### Column Width Optimization

Automatically calculate optimal column widths.

```rust
use typist_service::{ColumnWidthOptimizer, ColumnWidthConfig};

let config = ColumnWidthConfig::new()
    .with_use_golden_ratio(true);

let optimizer = ColumnWidthOptimizer::new(config);
let widths = optimizer.calculate_optimal_widths(595.0);
```

## CJK Typography

### Punctuation Compression

Adjust punctuation width for better CJK typography.

```rust
use typist_service::{CJKTypographyEngine, CJKTypographyConfig, CJKLanguage};

let config = CJKTypographyConfig::new(CJKLanguage::SimplifiedChinese);
let engine = CJKTypographyEngine::new(config);
let compressed = engine.apply_punctuation_compression("测试，文本。");
```

### Line Break Rules

Avoid line breaks at inappropriate positions.

```rust
use typist_service::{CJKTypographyEngine, CJKTypographyConfig};

let config = CJKTypographyConfig::new(CJKLanguage::Japanese);
let engine = CJKTypographyEngine::new(config);
let can_break = engine.can_start_line('。');
```

### Vertical Text

Support for vertical writing mode.

```rust
use typist_service::{CJKTypographyConfig, WritingMode};

let config = CJKTypographyConfig::new(CJKLanguage::TraditionalChinese)
    .with_writing_mode(WritingMode::Vertical);
```

## Color Management

### RGB to CMYK Conversion

Convert colors for print production.

```rust
use typist_service::{ColorManagementSystem, RGBColor};

let system = ColorManagementSystem::default();
let rgb = RGBColor::new(255, 0, 0);
let cmyk = system.rgb_to_cmyk(&rgb);
println!("CMYK: {}", cmyk.to_percentage());
```

### Pantone Colors

Find the nearest Pantone color to an RGB value.

```rust
use typist_service::{ColorManagementSystem, RGBColor};

let system = ColorManagementSystem::default();
let rgb = RGBColor::new(237, 28, 36);
let pantone = system.find_nearest_pantone(&rgb);
```

### ICC Profiles

Manage color profiles for consistent color reproduction.

```rust
use typist_service::{ColorManagementSystem};

let system = ColorManagementSystem::default();
let profile = system.get_icc_profile("sRGB");
```

## Master Pages

### Creating Master Pages

Define template pages with consistent styling.

```rust
use typist_service::{MasterPage, PageElement, PageElementType};

let mut master = MasterPage::new("Default".to_string());
let page_number = PageElement::new(PageElementType::PageNumber, String::new())
    .with_position(0.0, 0.0);
master = master.add_footer_element(page_number);
```

### Page Instances

Create pages based on master pages with custom content.

```rust
use typist_service::{MasterPageSystem};

let mut system = MasterPageSystem::new();
system.register_master_page(master).unwrap();
let instance = system.create_page_instance("Default".to_string(), 1).unwrap();
```

## Performance Optimization

### Incremental Compilation

Only recompile changed content for faster builds.

```rust
use typist_service::{IncrementalCompiler, IncrementalConfig};

let config = IncrementalConfig::default();
let compiler = IncrementalCompiler::new(config);

let hash = compiler.compute_hash("content");
if compiler.is_cache_valid("key", &hash) {
    // Use cached result
} else {
    // Recompile
}
```

### Parallel Compilation

Compile multiple documents simultaneously.

```rust
use typist_service::IncrementalCompiler;

let tasks = vec![
    ("doc1".to_string(), "content1".to_string()),
    ("doc2".to_string(), "content2".to_string()),
];

let results = compiler.parallel_compile(tasks, |content| {
    // Compilation function
    Ok(vec![])
});
```

### Hot Reload

Automatically detect changes and recompile.

```rust
use typist_service::IncrementalCompiler;

let hash = compiler.compute_hash("content");
if compiler.should_hot_reload("key", &hash) {
    // Reload and recompile
}
```

## Best Practices

### Typography

1. **Use appropriate kerning** for professional-looking text
2. **Enable OpenType features** for better font rendering
3. **Use font pairing** for consistent design
4. **Set appropriate line heights** for readability

### Layout

1. **Use baseline grids** for consistent spacing
2. **Avoid widows and orphans** for professional documents
3. **Use golden ratio** for pleasing column widths
4. **Maintain consistent margins** throughout

### CJK Typography

1. **Use punctuation compression** for better spacing
2. **Follow line break rules** for proper CJK typesetting
3. **Use appropriate writing mode** (horizontal/vertical)
4. **Set correct language** for proper character handling

### Color Management

1. **Use CMYK for print** and RGB for screen
2. **Use Pantone colors** for brand consistency
3. **Apply ICC profiles** for accurate color reproduction
4. **Use appropriate rendering intent** for color conversion

### Performance

1. **Enable incremental compilation** for faster builds
2. **Use parallel compilation** for multiple documents
3. **Configure cache size** appropriately
4. **Clean expired cache** regularly

## Troubleshooting

### Compilation Errors

If you encounter compilation errors:

1. Check the error message for specific details
2. Verify your Typst syntax is correct
3. Ensure all dependencies are available
4. Check font paths are correct

### Performance Issues

If performance is slow:

1. Enable incremental compilation
2. Increase cache size
3. Use parallel compilation
4. Check system resources

### Typography Issues

If typography doesn't look right:

1. Verify font is installed
2. Check kerning settings
3. Enable appropriate OpenType features
4. Use proper font pairing

## Examples

### Complete Document Setup

```rust
use typist_service::{
    TypographyEngine, TypographyConfig,
    LayoutSystem, LayoutSystemConfig,
    ColorManagementSystem,
    IncrementalCompiler,
};

// Typography setup
let typography_config = TypographyConfig::new()
    .with_kerning(KerningTable::new())
    .with_opentype_features(OpenTypeFeatures::new());

let typography_engine = TypographyEngine::new(typography_config);

// Layout setup
let layout_config = LayoutSystemConfig::new()
    .with_grid_config(GridConfig::default());

let layout_system = LayoutSystem::new(layout_config);

// Color management
let color_system = ColorManagementSystem::default();

// Incremental compilation
let compiler = IncrementalCompiler::default();

// Render document
let content = "#set page(paper: \"a4\")\nHello, World!";
let result = compiler.render(content);
```

## Support

For additional support, refer to:
- API Documentation: `TYPIST_API_DOCUMENTATION.md`
- Architecture Documentation: `TYPIST_ARCHITECTURE.md`
- Test Suite: Run `cargo test --lib typist_service`

## License

Proprietary - Aerospace Grade License
