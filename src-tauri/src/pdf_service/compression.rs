use serde::{Deserialize, Serialize};

/// 压缩级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionLevel {
    /// 无压缩
    None,
    /// 快速压缩（压缩率低）
    Fast,
    /// 标准压缩（平衡）
    Normal,
    /// 最大压缩（压缩率高，速度慢）
    Maximum,
}

impl CompressionLevel {
    /// 获取压缩级别描述
    pub fn description(&self) -> &str {
        match self {
            CompressionLevel::None => "No compression",
            CompressionLevel::Fast => "Fast compression (low ratio)",
            CompressionLevel::Normal => "Normal compression (balanced)",
            CompressionLevel::Maximum => "Maximum compression (high ratio)",
        }
    }

    /// 获取压缩级别数值（0-9）
    #[allow(dead_code)]
    pub fn as_number(&self) -> u8 {
        match self {
            CompressionLevel::None => 0,
            CompressionLevel::Fast => 3,
            CompressionLevel::Normal => 6,
            CompressionLevel::Maximum => 9,
        }
    }
}

/// 图像压缩模式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImageCompression {
    /// 不压缩
    None,
    /// JPEG 压缩（适合照片）
    Jpeg { quality: u8 },
    /// JPEG 2000 压缩
    Jpeg2000 { quality: u8 },
    /// CCITT Group 4（适合黑白图像）
    CcittGroup4,
    /// ZIP 压缩（适合图形）
    Zip,
}

impl ImageCompression {
    /// 创建 JPEG 压缩
    pub fn jpeg(quality: u8) -> Self {
        Self::Jpeg {
            quality: quality.min(100),
        }
    }

    /// 创建 JPEG 2000 压缩
    #[allow(dead_code)]
    pub fn jpeg2000(quality: u8) -> Self {
        Self::Jpeg2000 {
            quality: quality.min(100),
        }
    }

    /// 获取压缩质量（0-100）
    #[allow(dead_code)]
    pub fn quality(&self) -> Option<u8> {
        match self {
            ImageCompression::Jpeg { quality } => Some(*quality),
            ImageCompression::Jpeg2000 { quality } => Some(*quality),
            _ => None,
        }
    }
}

/// PDF 压缩配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfCompression {
    /// 是否启用压缩
    pub enabled: bool,
    /// 压缩级别
    pub level: CompressionLevel,
    /// 图像压缩模式
    pub image_compression: ImageCompression,
    /// 是否子集化字体（只嵌入使用的字符）
    pub subset_fonts: bool,
    /// 是否删除未使用的对象
    pub remove_unused_objects: bool,
    /// 是否线性化 PDF（用于网络浏览）
    pub linearize: bool,
}

impl PdfCompression {
    /// 创建新的压缩配置
    pub fn new() -> Self {
        Self {
            enabled: true,
            level: CompressionLevel::Normal,
            image_compression: ImageCompression::jpeg(85),
            subset_fonts: true,
            remove_unused_objects: true,
            linearize: false,
        }
    }

    /// 禁用压缩
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            level: CompressionLevel::None,
            image_compression: ImageCompression::None,
            subset_fonts: false,
            remove_unused_objects: false,
            linearize: false,
        }
    }

    /// 设置压缩级别
    #[allow(dead_code)]
    pub fn with_level(mut self, level: CompressionLevel) -> Self {
        self.level = level;
        self
    }

    /// 设置图像压缩
    #[allow(dead_code)]
    pub fn with_image_compression(mut self, compression: ImageCompression) -> Self {
        self.image_compression = compression;
        self
    }

    /// 设置是否子集化字体
    #[allow(dead_code)]
    pub fn with_subset_fonts(mut self, subset: bool) -> Self {
        self.subset_fonts = subset;
        self
    }

    /// 设置是否删除未使用的对象
    #[allow(dead_code)]
    pub fn with_remove_unused_objects(mut self, remove: bool) -> Self {
        self.remove_unused_objects = remove;
        self
    }

    /// 设置是否线性化 PDF
    #[allow(dead_code)]
    pub fn with_linearize(mut self, linearize: bool) -> Self {
        self.linearize = linearize;
        self
    }

    /// 获取压缩配置描述
    pub fn description(&self) -> String {
        if !self.enabled {
            return "Compression disabled".to_string();
        }
        let image_desc = match self.image_compression {
            ImageCompression::None => "None".to_string(),
            ImageCompression::Jpeg { quality } => format!("JPEG ({}%)", quality),
            ImageCompression::Jpeg2000 { quality } => format!("JPEG2000 ({}%)", quality),
            ImageCompression::CcittGroup4 => "CCITT Group 4".to_string(),
            ImageCompression::Zip => "ZIP".to_string(),
        };
        format!(
            "{} - Image: {}, Font subsetting: {}, Linearize: {}",
            self.level.description(),
            image_desc,
            if self.subset_fonts {
                "Enabled"
            } else {
                "Disabled"
            },
            if self.linearize {
                "Enabled"
            } else {
                "Disabled"
            }
        )
    }
}

impl Default for PdfCompression {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_level_description() {
        assert_eq!(CompressionLevel::None.description(), "No compression");
        assert_eq!(
            CompressionLevel::Maximum.description(),
            "Maximum compression (high ratio)"
        );
    }

    #[test]
    fn test_compression_level_as_number() {
        assert_eq!(CompressionLevel::None.as_number(), 0);
        assert_eq!(CompressionLevel::Normal.as_number(), 6);
        assert_eq!(CompressionLevel::Maximum.as_number(), 9);
    }

    #[test]
    fn test_image_compression_jpeg() {
        let compression = ImageCompression::jpeg(85);
        assert_eq!(compression.quality(), Some(85));
    }

    #[test]
    fn test_image_compression_jpeg2000() {
        let compression = ImageCompression::jpeg2000(90);
        assert_eq!(compression.quality(), Some(90));
    }

    #[test]
    fn test_image_compression_none_quality() {
        let compression = ImageCompression::None;
        assert_eq!(compression.quality(), None);
    }

    #[test]
    fn test_pdf_compression_new() {
        let compression = PdfCompression::new();
        assert!(compression.enabled);
        assert_eq!(compression.level, CompressionLevel::Normal);
    }

    #[test]
    fn test_pdf_compression_disabled() {
        let compression = PdfCompression::disabled();
        assert!(!compression.enabled);
        assert_eq!(compression.level, CompressionLevel::None);
    }

    #[test]
    fn test_pdf_compression_with_level() {
        let compression = PdfCompression::new().with_level(CompressionLevel::Maximum);
        assert_eq!(compression.level, CompressionLevel::Maximum);
    }

    #[test]
    fn test_pdf_compression_with_image_compression() {
        let img_comp = ImageCompression::jpeg(90);
        let compression = PdfCompression::new().with_image_compression(img_comp);
        assert_eq!(compression.image_compression.quality(), Some(90));
    }

    #[test]
    fn test_pdf_compression_with_subset_fonts() {
        let compression = PdfCompression::new().with_subset_fonts(false);
        assert!(!compression.subset_fonts);
    }

    #[test]
    fn test_pdf_compression_chaining() {
        let compression = PdfCompression::new()
            .with_level(CompressionLevel::Maximum)
            .with_subset_fonts(false)
            .with_linearize(true);
        assert_eq!(compression.level, CompressionLevel::Maximum);
        assert!(!compression.subset_fonts);
        assert!(compression.linearize);
    }

    #[test]
    fn test_pdf_compression_description() {
        let compression = PdfCompression::new();
        let desc = compression.description();
        assert!(desc.contains("Normal compression"));
    }

    #[test]
    fn test_pdf_compression_description_disabled() {
        let compression = PdfCompression::disabled();
        let desc = compression.description();
        assert_eq!(desc, "Compression disabled");
    }

    #[test]
    fn test_pdf_compression_default() {
        let compression = PdfCompression::default();
        assert!(compression.enabled);
    }

    #[test]
    fn test_pdf_compression_serialization() {
        let compression = PdfCompression::new();
        let json = serde_json::to_string(&compression);
        assert!(json.is_ok());
    }
}
