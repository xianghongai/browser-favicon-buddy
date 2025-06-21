//! Favicon处理模块
//!
//! 提供获取和处理网站favicon的功能

pub mod cache;
pub mod fetch;
pub mod process;

pub use cache::{FaviconCache, get_cache_path};
pub use fetch::{fetch_favicon_base64, fetch_favicon_base64_async};
pub use process::process_bookmarks;