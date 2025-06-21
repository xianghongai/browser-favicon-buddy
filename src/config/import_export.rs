//! 配置导入导出模块

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::errors::AppResult;
use super::AppConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheData {
    pub favicon_urls: std::collections::HashMap<String, String>,
}

impl Default for CacheData {
    fn default() -> Self {
        Self {
            favicon_urls: std::collections::HashMap::new(),
        }
    }
}

/// 配置导入导出功能
pub trait ConfigImportExport {
    /// 导出应用配置到文件
    fn export_config(&self, file_path: &str) -> AppResult<ExportResult>;
    /// 从文件导入应用配置
    fn import_config(file_path: &str) -> AppResult<(Self, ExportResult)> where Self: Sized;
    /// 导出缓存数据
    fn export_cache(&self, file_path: &Path) -> AppResult<ExportResult>;
    /// 导入缓存数据
    fn import_cache(&mut self, file_path: &Path) -> AppResult<ExportResult>;
}

impl ConfigImportExport for AppConfig {
    fn export_config(&self, file_path: &str) -> AppResult<ExportResult> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(file_path, content)?;
        Ok(ExportResult {
            success: true,
            message: crate::i18n::get_message("export_success", None),
        })
    }

    fn import_config(file_path: &str) -> AppResult<(Self, ExportResult)> {
        if !Path::new(file_path).exists() {
            return Err(crate::errors::AppError::FileNotFound(file_path.to_string()));
        }
        let content = fs::read_to_string(file_path)?;
        let config = serde_json::from_str(&content)?;
        Ok((config, ExportResult {
            success: true,
            message: crate::i18n::get_message("import_success", None),
        }))
    }

    fn export_cache(&self, file_path: &Path) -> AppResult<ExportResult> {
        // 获取缓存文件路径
        let cache_path = crate::favicon::get_cache_path();

        // 确保缓存文件存在
        if !Path::new(&cache_path).exists() {
            return Ok(ExportResult {
                success: true,
                message: crate::i18n::get_message("cache_export_success", None),
            });
        }

        // 读取缓存数据
        let favicon_cache = match fs::read_to_string(&cache_path) {
            Ok(data) => {
                match serde_json::from_str::<crate::favicon::FaviconCache>(&data) {
                    Ok(cache) => cache.0,
                    Err(_) => std::collections::HashMap::new(),
                }
            },
            Err(_) => std::collections::HashMap::new(),
        };

        // 转换为导出格式
        let cache_data = CacheData {
            favicon_urls: favicon_cache.into_iter()
                .filter_map(|(k, v)| v.map(|url| (k, url)))
                .collect(),
        };

        // 确保目标目录存在
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // 导出到文件
        let content = serde_json::to_string_pretty(&cache_data)?;
        fs::write(file_path, content)?;

        Ok(ExportResult {
            success: true,
            message: crate::i18n::get_message("cache_export_success", None),
        })
    }

    fn import_cache(&mut self, file_path: &Path) -> AppResult<ExportResult> {
        if !file_path.exists() {
            return Err(crate::errors::AppError::FileNotFound(file_path.display().to_string()));
        }

        // 读取导入文件
        let content = fs::read_to_string(file_path)?;
        let cache_data: CacheData = serde_json::from_str(&content)?;

        // 获取当前缓存
        let cache_path = crate::favicon::get_cache_path();
        let mut current_cache = if let Ok(data) = fs::read_to_string(&cache_path) {
            serde_json::from_str::<crate::favicon::FaviconCache>(&data)
                .unwrap_or_else(|_| crate::favicon::FaviconCache(HashMap::new()))
        } else {
            crate::favicon::FaviconCache(HashMap::new())
        };

        // 合并缓存数据
        for (domain, favicon) in cache_data.favicon_urls {
            current_cache.0.insert(domain, Some(favicon));
        }

        // 保存更新后的缓存
        let updated_content = serde_json::to_string_pretty(&current_cache)?;
        fs::write(cache_path, updated_content)?;

        Ok(ExportResult {
            success: true,
            message: crate::i18n::get_message("cache_import_success", None),
        })
    }
}