//! 错误处理模块
//!
//! 使用 thiserror 定义应用程序的自定义错误类型

use thiserror::Error;
use std::io;

/// 应用程序错误类型
#[derive(Error, Debug)]
pub enum AppError {
    /// 文件操作错误
    #[error("文件操作错误: {0}")]
    FileError(#[from] io::Error),

    /// 网络请求错误
    #[error("网络请求错误: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// URL 解析错误
    #[error("URL 解析错误: {0}")]
    UrlParseError(#[from] url::ParseError),

    /// JSON 序列化/反序列化错误
    #[error("JSON 错误: {0}")]
    JsonError(#[from] serde_json::Error),

    /// 正则表达式错误
    #[error("正则表达式错误: {0}")]
    RegexError(#[from] regex::Error),

    /// 图像处理错误
    #[error("图像处理错误: {0}")]
    ImageError(#[from] image::error::ImageError),

    /// 自定义错误消息
    #[error("{0}")]
    CustomError(String),

    /// 文件未找到错误
    #[error("文件未找到: {0}")]
    FileNotFound(String),
}

/// 应用程序结果类型
pub type AppResult<T> = Result<T, AppError>;

/// 从字符串创建自定义错误
impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::CustomError(error)
    }
}

/// 从字符串切片创建自定义错误
impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError::CustomError(error.to_string())
    }
}