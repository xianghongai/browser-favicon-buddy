use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;

/// Favicon缓存结构：key为域名，value为Option<String>（base64或None）
#[derive(Serialize, Deserialize)]
pub struct FaviconCache(pub HashMap<String, Option<String>>);

/// 获取缓存文件路径
pub fn get_cache_path() -> String {
    format!("{}/favicon_cache.json", AppConfig::get_app_dir())
}