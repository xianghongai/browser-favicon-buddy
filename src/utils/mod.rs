//! 工具模块
//!
//! 提供各种工具函数

pub mod file;
pub mod format;

pub use file::generate_output_filename;
pub use format::format_log_message;