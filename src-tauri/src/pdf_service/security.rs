use serde::{Deserialize, Serialize};

/// 加密级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EncryptionLevel {
    /// 40 位 RC4（兼容性最好，安全性最低）
    Rc4_40,
    /// 128 位 RC4
    Rc4_128,
    /// 128 位 AES
    Aes128,
    /// 256 位 AES（安全性最高）
    Aes256,
}

impl EncryptionLevel {
    /// 获取加密级别描述
    pub fn description(&self) -> &str {
        match self {
            EncryptionLevel::Rc4_40 => "40-bit RC4 (Low security)",
            EncryptionLevel::Rc4_128 => "128-bit RC4 (Medium security)",
            EncryptionLevel::Aes128 => "128-bit AES (High security)",
            EncryptionLevel::Aes256 => "256-bit AES (Maximum security)",
        }
    }

    /// 获取密钥长度（位）
    #[allow(dead_code)]
    pub fn key_length(&self) -> u32 {
        match self {
            EncryptionLevel::Rc4_40 => 40,
            EncryptionLevel::Rc4_128 => 128,
            EncryptionLevel::Aes128 => 128,
            EncryptionLevel::Aes256 => 256,
        }
    }
}

/// PDF 安全权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPermissions {
    /// 允许打印
    pub allow_print: bool,
    /// 允许打印高分辨率
    pub allow_print_high: bool,
    /// 允许修改内容
    pub allow_modify: bool,
    /// 允许复制内容
    pub allow_copy: bool,
    /// 允许注释
    pub allow_annotate: bool,
    /// 允许填写表单
    pub allow_fill_forms: bool,
    /// 允许提取内容
    pub allow_extract: bool,
    /// 允许组装文档
    pub allow_assemble: bool,
}

impl SecurityPermissions {
    /// 创建默认权限（所有权限允许）
    pub fn new() -> Self {
        Self {
            allow_print: true,
            allow_print_high: true,
            allow_modify: true,
            allow_copy: true,
            allow_annotate: true,
            allow_fill_forms: true,
            allow_extract: true,
            allow_assemble: true,
        }
    }

    /// 创建只读权限（禁止所有修改）
    #[allow(dead_code)]
    pub fn read_only() -> Self {
        Self {
            allow_print: true,
            allow_print_high: false,
            allow_modify: false,
            allow_copy: false,
            allow_annotate: false,
            allow_fill_forms: false,
            allow_extract: false,
            allow_assemble: false,
        }
    }

    /// 创建打印限制权限
    #[allow(dead_code)]
    pub fn print_only() -> Self {
        Self {
            allow_print: true,
            allow_print_high: false,
            allow_modify: false,
            allow_copy: false,
            allow_annotate: false,
            allow_fill_forms: false,
            allow_extract: false,
            allow_assemble: false,
        }
    }

    /// 设置打印权限
    #[allow(dead_code)]
    pub fn with_print(mut self, allow: bool) -> Self {
        self.allow_print = allow;
        self
    }

    /// 设置高分辨率打印权限
    #[allow(dead_code)]
    pub fn with_print_high(mut self, allow: bool) -> Self {
        self.allow_print_high = allow;
        self
    }

    /// 设置修改权限
    #[allow(dead_code)]
    pub fn with_modify(mut self, allow: bool) -> Self {
        self.allow_modify = allow;
        self
    }

    /// 设置复制权限
    #[allow(dead_code)]
    pub fn with_copy(mut self, allow: bool) -> Self {
        self.allow_copy = allow;
        self
    }

    /// 设置注释权限
    #[allow(dead_code)]
    pub fn with_annotate(mut self, allow: bool) -> Self {
        self.allow_annotate = allow;
        self
    }

    /// 设置填写表单权限
    #[allow(dead_code)]
    pub fn with_fill_forms(mut self, allow: bool) -> Self {
        self.allow_fill_forms = allow;
        self
    }

    /// 设置提取内容权限
    #[allow(dead_code)]
    pub fn with_extract(mut self, allow: bool) -> Self {
        self.allow_extract = allow;
        self
    }

    /// 设置组装文档权限
    #[allow(dead_code)]
    pub fn with_assemble(mut self, allow: bool) -> Self {
        self.allow_assemble = allow;
        self
    }
}

impl Default for SecurityPermissions {
    fn default() -> Self {
        Self::new()
    }
}

/// PDF 安全配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfSecurity {
    /// 是否启用加密
    pub enabled: bool,
    /// 用户密码（打开文档所需）
    pub user_password: Option<String>,
    /// 所有者密码（修改权限所需）
    pub owner_password: Option<String>,
    /// 加密级别
    pub encryption_level: EncryptionLevel,
    /// 安全权限
    pub permissions: SecurityPermissions,
}

impl PdfSecurity {
    /// 创建新的安全配置（未启用）
    pub fn new() -> Self {
        Self {
            enabled: false,
            user_password: None,
            owner_password: None,
            encryption_level: EncryptionLevel::Aes256,
            permissions: SecurityPermissions::new(),
        }
    }

    /// 启用加密
    #[allow(dead_code)]
    pub fn enabled(mut self) -> Self {
        self.enabled = true;
        self
    }

    /// 设置用户密码
    #[allow(dead_code)]
    pub fn with_user_password(mut self, password: String) -> Self {
        self.user_password = Some(password);
        self.enabled = true;
        self
    }

    /// 设置所有者密码
    #[allow(dead_code)]
    pub fn with_owner_password(mut self, password: String) -> Self {
        self.owner_password = Some(password);
        self.enabled = true;
        self
    }

    /// 设置加密级别
    #[allow(dead_code)]
    pub fn with_encryption_level(mut self, level: EncryptionLevel) -> Self {
        self.encryption_level = level;
        self
    }

    /// 设置权限
    #[allow(dead_code)]
    pub fn with_permissions(mut self, permissions: SecurityPermissions) -> Self {
        self.permissions = permissions;
        self
    }

    /// 验证密码是否设置
    #[allow(dead_code)]
    pub fn has_password(&self) -> bool {
        self.user_password.is_some() || self.owner_password.is_some()
    }

    /// 获取加密强度描述
    pub fn strength_description(&self) -> String {
        if !self.enabled {
            return "No encryption".to_string();
        }
        format!(
            "{} - {}",
            self.encryption_level.description(),
            if self.permissions.allow_copy {
                "Content accessible"
            } else {
                "Content protected"
            }
        )
    }
}

impl Default for PdfSecurity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_level_description() {
        assert_eq!(
            EncryptionLevel::Rc4_40.description(),
            "40-bit RC4 (Low security)"
        );
        assert_eq!(
            EncryptionLevel::Aes256.description(),
            "256-bit AES (Maximum security)"
        );
    }

    #[test]
    fn test_encryption_level_key_length() {
        assert_eq!(EncryptionLevel::Rc4_40.key_length(), 40);
        assert_eq!(EncryptionLevel::Aes256.key_length(), 256);
    }

    #[test]
    fn test_security_permissions_new() {
        let permissions = SecurityPermissions::new();
        assert!(permissions.allow_print);
        assert!(permissions.allow_copy);
        assert!(permissions.allow_modify);
    }

    #[test]
    fn test_security_permissions_read_only() {
        let permissions = SecurityPermissions::read_only();
        assert!(permissions.allow_print);
        assert!(!permissions.allow_copy);
        assert!(!permissions.allow_modify);
    }

    #[test]
    fn test_security_permissions_print_only() {
        let permissions = SecurityPermissions::print_only();
        assert!(permissions.allow_print);
        assert!(!permissions.allow_print_high);
        assert!(!permissions.allow_copy);
    }

    #[test]
    fn test_security_permissions_with_print() {
        let permissions = SecurityPermissions::new().with_print(false);
        assert!(!permissions.allow_print);
    }

    #[test]
    fn test_security_permissions_chaining() {
        let permissions = SecurityPermissions::new()
            .with_print(false)
            .with_copy(false)
            .with_modify(false);
        assert!(!permissions.allow_print);
        assert!(!permissions.allow_copy);
        assert!(!permissions.allow_modify);
    }

    #[test]
    fn test_pdf_security_new() {
        let security = PdfSecurity::new();
        assert!(!security.enabled);
        assert!(security.user_password.is_none());
    }

    #[test]
    fn test_pdf_security_enabled() {
        let security = PdfSecurity::new().enabled();
        assert!(security.enabled);
    }

    #[test]
    fn test_pdf_security_with_user_password() {
        let security = PdfSecurity::new().with_user_password("password".to_string());
        assert!(security.enabled);
        assert_eq!(security.user_password, Some("password".to_string()));
    }

    #[test]
    fn test_pdf_security_with_owner_password() {
        let security = PdfSecurity::new().with_owner_password("owner".to_string());
        assert!(security.enabled);
        assert_eq!(security.owner_password, Some("owner".to_string()));
    }

    #[test]
    fn test_pdf_security_with_encryption_level() {
        let security = PdfSecurity::new().with_encryption_level(EncryptionLevel::Aes128);
        assert_eq!(security.encryption_level, EncryptionLevel::Aes128);
    }

    #[test]
    fn test_pdf_security_with_permissions() {
        let permissions = SecurityPermissions::read_only();
        let security = PdfSecurity::new().with_permissions(permissions);
        assert!(!security.permissions.allow_copy);
    }

    #[test]
    fn test_pdf_security_has_password() {
        let security = PdfSecurity::new().with_user_password("pass".to_string());
        assert!(security.has_password());
    }

    #[test]
    fn test_pdf_security_strength_description() {
        let security = PdfSecurity::new();
        assert_eq!(security.strength_description(), "No encryption");

        let security = PdfSecurity::new()
            .enabled()
            .with_encryption_level(EncryptionLevel::Aes256);
        assert!(security.strength_description().contains("256-bit AES"));
    }

    #[test]
    fn test_pdf_security_chaining() {
        let security = PdfSecurity::new()
            .with_user_password("user".to_string())
            .with_owner_password("owner".to_string())
            .with_encryption_level(EncryptionLevel::Aes256);
        assert!(security.enabled);
        assert_eq!(security.user_password, Some("user".to_string()));
        assert_eq!(security.owner_password, Some("owner".to_string()));
    }

    #[test]
    fn test_pdf_security_default() {
        let security = PdfSecurity::default();
        assert!(!security.enabled);
    }

    #[test]
    fn test_pdf_security_serialization() {
        let security = PdfSecurity::new();
        let json = serde_json::to_string(&security);
        assert!(json.is_ok());
    }
}
