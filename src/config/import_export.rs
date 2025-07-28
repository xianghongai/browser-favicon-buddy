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

impl CacheData {
    /// 验证文件是否为有效的缓存数据格式
    pub fn validate_file(file_path: &Path) -> AppResult<()> {
        if !file_path.exists() {
            return Err(crate::errors::AppError::FileNotFound(file_path.display().to_string()));
        }

        let content = fs::read_to_string(file_path)?;

        // 尝试解析为不同的格式，与 import_cache 逻辑一致
        if serde_json::from_str::<CacheData>(&content).is_ok() {
            return Ok(());
        }

        // 尝试解析为 FaviconCache 结构
        if serde_json::from_str::<crate::favicon::FaviconCache>(&content).is_ok() {
            return Ok(());
        }

        // 尝试解析为简单的 HashMap 格式
        if serde_json::from_str::<HashMap<String, String>>(&content).is_ok() {
            return Ok(());
        }

        // 尝试解析为 HashMap<String, Option<String>> 格式
        if serde_json::from_str::<HashMap<String, Option<String>>>(&content).is_ok() {
            return Ok(());
        }

        // 尝试解析为通用 JSON 值
        let value: serde_json::Value = serde_json::from_str(&content)?;
        if let Some(obj) = value.as_object() {
            // 检查是否是一个合理的对象结构
            if obj.get("favicon_urls").is_some() {
                return Ok(());
            }

            // 检查是否有至少一个键值对，其中值是字符串或对象
            for (_, val) in obj {
                if val.is_string() || val.is_object() {
                    return Ok(());
                }
            }
        }

        Err(crate::errors::AppError::CustomError(
            crate::i18n::get_message("invalid_cache_format", None)
        ))
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

        // 尝试多种解析方式，与 export_cache 的逻辑保持一致
        let favicon_urls = match serde_json::from_str::<CacheData>(&content) {
            Ok(cache_data) => {
                // 标准格式，包含 favicon_urls 字段
                cache_data.favicon_urls
            },
            Err(e) => {
                // 尝试解析为直接导出的 FaviconCache 结构
                match serde_json::from_str::<crate::favicon::FaviconCache>(&content) {
                    Ok(favicon_cache) => {
                        // 与 export_cache 中相同的转换逻辑
                        favicon_cache.0.into_iter()
                            .filter_map(|(k, v)| v.map(|url| (k, url)))
                            .collect()
                    },
                    Err(_) => {
                        // 尝试作为普通的 HashMap 解析
                        match serde_json::from_str::<HashMap<String, String>>(&content) {
                            Ok(map) => map,
                            Err(_) => {
                                // 尝试解析为可能的 HashMap<String, Option<String>> 格式
                                match serde_json::from_str::<HashMap<String, Option<String>>>(&content) {
                                    Ok(option_map) => {
                                        option_map.into_iter()
                                            .filter_map(|(k, v)| v.map(|url| (k, url)))
                                            .collect()
                                    },
                                    Err(_) => {
                                        // 尝试解析为通用 JSON 对象
                                        match serde_json::from_str::<serde_json::Value>(&content) {
                                            Ok(value) => {
                                                if let Some(obj) = value.as_object() {
                                                    // 如果是一个对象，尝试提取键值对
                                                    let mut map = HashMap::new();

                                                    // 首先检查是否有 favicon_urls 字段
                                                    if let Some(urls_obj) = obj.get("favicon_urls").and_then(|u| u.as_object()) {
                                                        for (key, val) in urls_obj {
                                                            if let Some(url) = val.as_str() {
                                                                map.insert(key.clone(), url.to_string());
                                                            }
                                                        }
                                                        map
                                                    } else {
                                                        // 没有 favicon_urls 字段，尝试直接解析对象
                                                        for (key, val) in obj {
                                                            if let Some(url) = val.as_str() {
                                                                map.insert(key.clone(), url.to_string());
                                                            } else if let Some(obj_val) = val.as_object() {
                                                                // 尝试寻找 URL 字段或其他可能包含 URL 的字段
                                                                for url_field in &["url", "favicon", "icon", "data"] {
                                                                    if let Some(url_val) = obj_val.get(*url_field).and_then(|u| u.as_str()) {
                                                                        map.insert(key.clone(), url_val.to_string());
                                                                        break;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        map
                                                    }
                                                } else {
                                                    // 返回原始错误
                                                    return Err(crate::errors::AppError::JsonError(e));
                                                }
                                            },
                                            Err(_) => {
                                                // 返回原始错误
                                                return Err(crate::errors::AppError::JsonError(e));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };

        // 获取当前缓存
        let cache_path = crate::favicon::get_cache_path();
        let mut current_cache = if let Ok(data) = fs::read_to_string(&cache_path) {
            serde_json::from_str::<crate::favicon::FaviconCache>(&data)
                .unwrap_or_else(|_| crate::favicon::FaviconCache(HashMap::new()))
        } else {
            crate::favicon::FaviconCache(HashMap::new())
        };

        // 合并缓存数据
        for (domain, favicon) in favicon_urls {
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