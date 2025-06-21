//! 文件处理模块
//! 
//! 提供文件操作相关功能

use std::path::Path;
use chrono::Local;

/// 生成输出文件名
/// 
/// 基于输入文件名生成带有"-with-favicons"后缀和时间戳的输出文件名
/// 格式：原文件名-with-favicons--YYYY-MM-DD-HHMMSS.扩展名
pub fn generate_output_filename(input: &str) -> String {
    let path = Path::new(input);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("html");
    let dir = path.parent().map(|p| p.to_str().unwrap_or("")).unwrap_or("");
    
    // 获取当前时间并格式化为：YYYY-MM-DD-HHMMSS
    let timestamp = Local::now().format("%Y-%m-%d-%H%M%S").to_string();
    
    if dir.is_empty() {
        format!("{}-with-favicons--{}.{}", stem, timestamp, ext)
    } else {
        // 根据操作系统使用正确的路径分隔符
        #[cfg(target_os = "windows")]
        {
            format!("{}//{}-with-favicons--{}.{}", dir, stem, timestamp, ext)
        }
        #[cfg(not(target_os = "windows"))]
        {
            format!("{}/{}-with-favicons--{}.{}", dir, stem, timestamp, ext)
        }
    }
}