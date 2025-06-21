//! 语言配置模块

use serde::{Deserialize, Serialize};

/// 语言配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    /// 当前语言设置
    pub language: String,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            language: "zh-CN".to_string(),
        }
    }
}

impl LanguageConfig {
    /// 获取当前语言
    pub fn get_current_language(&self) -> &str {
        &self.language
    }

    /// 设置当前语言
    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }
}