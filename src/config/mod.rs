//! 配置模块
//!
//! 提供应用程序配置的加载和保存功能

pub mod favicon_service;
mod language;
pub mod import_export;

pub use favicon_service::FaviconServiceConfig;
pub use language::LanguageConfig;
pub use import_export::{ConfigImportExport, ExportResult};

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::errors::AppResult;
use std::io;

/// 应用程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Favicon 服务提供商配置
    #[serde(flatten)]
    pub favicon_service: FaviconServiceConfig,
    /// 语言配置
    #[serde(flatten)]
    pub language: LanguageConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            favicon_service: FaviconServiceConfig::default(),
            language: LanguageConfig::default(),
        }
    }
}

impl AppConfig {
    /// 获取应用程序配置目录
    pub fn get_app_dir() -> String {
        #[cfg(target_os = "windows")]
        {
            use std::env;
            let base = env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
            let dir = format!("{}\\.config\\favicon-buddy", base);
            std::fs::create_dir_all(&dir).ok();
            dir
        }
        #[cfg(not(target_os = "windows"))]
        {
            use std::env;
            let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let dir = format!("{}/.config/favicon-buddy", home);  // 使用正斜杠代替反斜杠
            println!("创建配置目录: {}", dir);
            std::fs::create_dir_all(&dir).ok();
            dir
        }
    }

    /// 获取配置文件路径
    pub fn get_config_path() -> String {
        format!("{}/config.json", Self::get_app_dir())
    }

    /// 加载配置
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        if Path::new(&config_path).exists() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
                    return config;
                }
            }
        }
        // 如果配置文件不存在或解析失败，返回默认配置
        let default_config = AppConfig::default();
        // 尝试保存默认配置
        let _ = default_config.save();
        default_config
    }

    /// 保存配置
    pub fn save(&self) -> AppResult<()> {
        let config_path = Self::get_config_path();
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    /// 获取favicon URL
    pub fn get_favicon_url(&self, domain: &str) -> String {
        let service = &self.favicon_service.services[self.favicon_service.current_service_index];
        service.url_template.replace("{domain}", domain)
    }

    /// 导出服务配置到JSON文件
    pub fn export_services(&self, path: &Path) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.favicon_service)?;
        fs::write(path, json)
    }

    /// 从JSON文件导入服务配置
    pub fn import_services(&mut self, path: &Path) -> io::Result<()> {
        let json = fs::read_to_string(path)?;
        let services: favicon_service::FaviconServiceConfig = serde_json::from_str(&json)?;
        self.favicon_service = services;
        self.save().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(())
    }
}