//! 格式化工具模块
//! 
//! 提供各种格式化功能

/// 格式化日志消息
/// 
/// 添加时间戳和适当的格式
pub fn format_log_message(message: &str) -> String {
    use chrono::Local;
    format!("[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message)
}