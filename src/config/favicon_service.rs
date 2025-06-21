//! Favicon 服务配置模块

use serde::{Deserialize, Serialize};

/// Favicon 服务提供商配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaviconService {
    /// 服务名称
    pub name: String,
    /// API URL 模板，使用 {domain} 作为域名占位符
    pub url_template: String,
    /// 是否为默认服务
    pub is_default: bool,
}

/// Favicon 服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaviconServiceConfig {
    /// Favicon 服务提供商列表
    pub services: Vec<FaviconService>,
    /// 当前选择的服务索引
    pub current_service_index: usize,
}

impl Default for FaviconServiceConfig {
    fn default() -> Self {
        Self {
            services: vec![
                FaviconService {
                    name: "Google".to_string(),
                    url_template: "https://www.google.com/s2/favicons?sz=64&domain={domain}".to_string(),
                    is_default: true,
                },
                FaviconService {
                    name: "DuckDuckGo".to_string(),
                    url_template: "https://icons.duckduckgo.com/ip3/{domain}.ico".to_string(),
                    is_default: false,
                },
            ],
            current_service_index: 0,
        }
    }
}