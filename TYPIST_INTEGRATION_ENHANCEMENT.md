# Typst 高质量输出集成方案

## 审计结果

### 现状分析

**当前导出流程**
```
HTML → DOCX/PPTX/Spreadsheet → 输出文件
```

**各服务 Typst 集成状态**
- ✅ **docx_service**: 无 Typst 集成
- ✅ **ppt_service**: 无 Typst 集成  
- ✅ **spreadsheet_service**: 无 Typst 集成
- ✅ **export_service**: 支持 Typst 独立导出，但未集成到其他格式

**现有 Typst 功能**
- 高级字体排版（字距、OpenType、字体配对）
- 专业版面控制（网格系统、版面平衡）
- CJK 排版增强（标点压缩、避头尾规则）
- 色彩管理（CMYK、Pantone、ICC 配置文件）
- 主控页面系统（模板、样式继承）
- 增量编译（缓存、并行处理）

## 增强方案

### 方案一：Typst 作为中间渲染层

**架构设计**
```
HTML → Typst → 高质量渲染 → DOCX/PPTX/Spreadsheet
```

**优势**
- 利用 Typst 的专业排版能力
- 统一的渲染质量
- 支持高级排版特性

**挑战**
- Typst 到 DOCX/PPTX/Spreadsheet 的转换复杂
- 需要实现格式映射

### 方案二：Typst 作为 PDF 渲染引擎

**架构设计**
```
HTML → Typst → PDF → DOCX/PPTX/Spreadsheet (转换)
```

**优势**
- Typst 原生支持 PDF 输出
- PDF 转换工具成熟
- 保持高质量输出

**挑战**
- PDF 到其他格式的转换可能损失质量
- 转换链路较长

### 方案三：混合方案（推荐）

**架构设计**
```
HTML → Typst → PDF (高质量输出)
HTML → 直接转换 → DOCX/PPTX/Spreadsheet (快速输出)
```

**优势**
- 提供高质量和快速两种模式
- 用户可根据需求选择
- 灵活性高

## 实现计划

### 阶段一：添加 Typst 渲染选项

**export_service 增强**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    pub format: ExportFormat,
    pub metadata: DocumentMetadata,
    pub include_toc: bool,
    pub include_page_numbers: bool,
    pub compress_images: bool,
    pub embed_fonts: bool,
    pub use_typst_rendering: bool,  // 新增：是否使用 Typst 渲染
    pub typst_quality: TypstQuality,  // 新增：Typst 渲染质量
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypstQuality {
    Standard,    // 标准质量
    High,        // 高质量（启用高级排版）
    Aerospace,   // 航空航天级（所有特性）
}
```

### 阶段二：实现 Typst 到各格式的转换

**DOCX 转换**

```rust
impl ExportGenerator {
    fn export_to_docx_with_typst(
        &self,
        content: &str,
        config: &ExportConfig,
    ) -> Result<ExportResult, String> {
        // 1. HTML → Typst
        let typst_code = self.html_to_typst(content, config)?;
        
        // 2. Typst → PDF
        let pdf_bytes = self.typst_to_pdf(&typst_code, config)?;
        
        // 3. PDF → DOCX
        let docx_bytes = self.pdf_to_docx(&pdf_bytes, config)?;
        
        Ok(ExportResult {
            output_data: docx_bytes,
            format: ExportFormat::Docx,
            file_size: docx_bytes.len(),
            success: true,
            error: None,
        })
    }
}
```

**PPTX 转换**

```rust
impl ExportGenerator {
    fn export_to_pptx_with_typst(
        &self,
        content: &str,
        config: &ExportConfig,
    ) -> Result<ExportResult, String> {
        // 1. HTML → Typst Slides
        let typst_code = self.html_to_typst_slides(content, config)?;
        
        // 2. Typst → PDF
        let pdf_bytes = self.typst_to_pdf(&typst_code, config)?;
        
        // 3. PDF → PPTX
        let pptx_bytes = self.pdf_to_pptx(&pdf_bytes, config)?;
        
        Ok(ExportResult {
            output_data: pptx_bytes,
            format: ExportFormat::Pptx,
            file_size: pptx_bytes.len(),
            success: true,
            error: None,
        })
    }
}
```

**Spreadsheet 转换**

```rust
impl ExportGenerator {
    fn export_to_spreadsheet_with_typst(
        &self,
        content: &str,
        config: &ExportConfig,
    ) -> Result<ExportResult, String> {
        // 1. HTML → Typst (表格布局)
        let typst_code = self.html_to_typst_table(content, config)?;
        
        // 2. Typst → PDF
        let pdf_bytes = self.typst_to_pdf(&typst_code, config)?;
        
        // 3. PDF → Spreadsheet
        let spreadsheet_bytes = self.pdf_to_spreadsheet(&pdf_bytes, config)?;
        
        Ok(ExportResult {
            output_data: spreadsheet_bytes,
            format: ExportFormat::Xlsx,
            file_size: spreadsheet_bytes.len(),
            success: true,
            error: None,
        })
    }
}
```

### 阶段三：集成高级排版特性

**启用 Typst 高级特性**

```rust
fn html_to_typst_with_features(
    content: &str,
    config: &ExportConfig,
) -> Result<String, String> {
    let mut typst_code = String::new();
    
    // 根据质量级别启用特性
    match config.typst_quality {
        TypstQuality::Standard => {
            typst_code += "#set page(paper: \"a4\")\n";
        }
        TypstQuality::High => {
            typst_code += "#set page(paper: \"a4\")\n";
            typst_code += "#set text(kerning: true)\n";
            typst_code += "#set text(features: (liga: true))\n";
        }
        TypstQuality::Aerospace => {
            // 字体排版
            typst_code += "#set page(paper: \"a4\")\n";
            typst_code += "#set text(kerning: true)\n";
            typst_code += "#set text(features: (liga: true, smcp: true))\n";
            typst_code += "#set text(optical-size: 12pt)\n";
            
            // 网格系统
            typst_code += "#set grid(spacing: 1pt)\n";
            
            // CJK 排版
            typst_code += "#set text(lang: \"zh\")\n";
            typst_code += "#set text(cjk-punctuation: \"compress\")\n";
            
            // 色彩管理
            typst_code += "#set text(fill: cmyk(0%, 0%, 0%, 100%))\n";
        }
    }
    
    // 转换内容
    typst_code += content;
    
    Ok(typst_code)
}
```

### 阶段四：添加 Tauri 命令

**新增命令**

```rust
#[tauri::command]
async fn export_with_typst(
    content: String,
    format: ExportFormat,
    quality: TypstQuality,
) -> Result<ExportResult, String> {
    let config = ExportConfig {
        format,
        metadata: DocumentMetadata::default(),
        include_toc: true,
        include_page_numbers: true,
        compress_images: true,
        embed_fonts: true,
        use_typst_rendering: true,
        typst_quality: quality,
    };
    
    let mut generator = ExportGenerator::new();
    generator.export(&content, &config)
}
```

## 技术实现细节

### HTML 到 Typst 转换增强

**使用现有的 HtmlToTypstConverter**

```rust
use crate::typst_conversion_service::HtmlToTypstConverter;

fn html_to_typst_enhanced(
    html: &str,
    config: &ExportConfig,
) -> Result<String, String> {
    let typst_config = TypstConversionConfig {
        enable_typography: matches!(config.typst_quality, TypstQuality::High | TypstQuality::Aerospace),
        enable_cjk: matches!(config.typst_quality, TypstQuality::Aerospace),
        enable_color_management: matches!(config.typst_quality, TypstQuality::Aerospace),
        enable_grid_system: matches!(config.typst_quality, TypstQuality::Aerospace),
        ..Default::default()
    };
    
    let converter = HtmlToTypstConverter::new(typst_config);
    let result = converter.convert(html);
    
    if result.success {
        Ok(result.typst_code)
    } else {
        Err(result.error.unwrap_or_else(|| "Conversion failed".to_string()))
    }
}
```

### PDF 到其他格式转换

**使用现有转换库**

```rust
// PDF to DOCX
fn pdf_to_docx(pdf_bytes: &[u8]) -> Result<Vec<u8>, String> {
    // 使用 pdf2docx 或类似库
    // 实现细节取决于可用库
    Ok(vec![])
}

// PDF to PPTX
fn pdf_to_pptx(pdf_bytes: &[u8]) -> Result<Vec<u8>, String> {
    // 使用 pdf2pptx 或类似库
    // 实现细节取决于可用库
    Ok(vec![])
}

// PDF to Spreadsheet
fn pdf_to_spreadsheet(pdf_bytes: &[u8]) -> Result<Vec<u8>, String> {
    // 使用 pdf2xlsx 或类似库
    // 实现细节取决于可用库
    Ok(vec![])
}
```

## 前端集成

**Vue 组件增强**

```vue
<template>
  <div class="export-dialog">
    <select v-model="exportFormat">
      <option value="pdf">PDF</option>
      <option value="docx">DOCX</option>
      <option value="pptx">PPTX</option>
      <option value="xlsx">Spreadsheet</option>
    </select>
    
    <select v-model="typstQuality">
      <option value="standard">Standard</option>
      <option value="high">High Quality</option>
      <option value="aerospace">Aerospace Grade</option>
    </select>
    
    <label>
      <input type="checkbox" v-model="useTypstRendering">
      Use Typst Rendering
    </label>
    
    <button @click="export">Export</button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const exportFormat = ref('pdf');
const typstQuality = ref('high');
const useTypstRendering = ref(true);

async function export() {
  const result = await invoke('export_with_typst', {
    content: editorContent.value,
    format: exportFormat.value,
    quality: typstQuality.value,
  });
  
  // 处理导出结果
}
</script>
```

## 性能优化

### 缓存策略

```rust
use crate::typist_service::incremental::IncrementalCompiler;

struct TypstExportCache {
    compiler: IncrementalCompiler,
}

impl TypstExportCache {
    fn get_cached_render(&self, content: &str, config: &ExportConfig) -> Option<Vec<u8>> {
        let hash = self.compute_hash(content, config);
        self.compiler.get_cached(&hash).map(|entry| entry.compiled_output)
    }
    
    fn cache_render(&self, content: &str, config: &ExportConfig, output: Vec<u8>) {
        let hash = self.compute_hash(content, config);
        self.compiler.update_cache(hash, hash, vec![], output);
    }
}
```

### 并行处理

```rust
async fn export_multiple_with_typst(
    contents: Vec<String>,
    config: &ExportConfig,
) -> Vec<Result<ExportResult, String>> {
    let tasks: Vec<_> = contents
        .into_iter()
        .map(|content| {
            let config = config.clone();
            tokio::spawn(async move {
                let mut generator = ExportGenerator::new();
                generator.export(&content, &config)
            })
        })
        .collect();
    
    let results = futures::future::join_all(tasks).await;
    results.into_iter().map(|r| r.unwrap()).collect()
}
```

## 测试计划

### 单元测试

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_export_to_docx_with_typst() {
        let content = "<h1>Test</h1><p>Content</p>";
        let config = ExportConfig {
            use_typst_rendering: true,
            typst_quality: TypstQuality::High,
            ..Default::default()
        };
        
        let mut generator = ExportGenerator::new();
        let result = generator.export_to_docx_with_typst(content, &config);
        
        assert!(result.is_ok());
        assert!(result.unwrap().success);
    }
    
    #[test]
    fn test_export_to_pptx_with_typst() {
        // 类似测试
    }
    
    #[test]
    fn test_export_to_spreadsheet_with_typst() {
        // 类似测试
    }
}
```

### 集成测试

```rust
#[tokio::test]
async fn test_typst_rendering_integration() {
    let content = "<h1>Test Document</h1><p>With advanced typography</p>";
    
    let result = invoke('export_with_typst', {
        content,
        format: 'pdf',
        quality: 'aerospace',
    }).await;
    
    assert!(result.success);
}
```

## 优势总结

1. **高质量输出**: 利用 Typst 的专业排版能力
2. **统一渲染**: 所有格式使用相同的渲染引擎
3. **灵活配置**: 用户可选择质量级别
4. **性能优化**: 支持缓存和并行处理
5. **向后兼容**: 保留原有的快速导出选项

## 实施优先级

**高优先级**
1. 添加 Typst 渲染选项到 ExportConfig
2. 实现 HTML → Typst → PDF → DOCX 转换链
3. 添加 Tauri 命令

**中优先级**
4. 实现 PPTX 和 Spreadsheet 转换
5. 集成高级排版特性
6. 添加缓存和性能优化

**低优先级**
7. 完善测试覆盖
8. 优化转换质量
9. 添加更多自定义选项

## 结论

通过将 Typst 作为高质量渲染引擎集成到现有的导出流程中，可以显著提升 DOCX、PPTX 和 Spreadsheet 的输出质量。建议采用混合方案，提供快速和高质量两种模式，让用户根据需求选择。
