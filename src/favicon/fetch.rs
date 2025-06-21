use reqwest;
use base64;

use crate::errors::{AppError, AppResult};

/// 获取favicon并转换为base64编码（异步版本）
pub async fn fetch_favicon_base64_async(url: &str) -> AppResult<String> {
    let resp = reqwest::get(url).await?;
    if !resp.status().is_success() {
        return Err(AppError::CustomError(format!("HTTP {}", resp.status())));
    }
    let mime = resp.headers().get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("image/png").to_string();
    let bytes = resp.bytes().await?.to_vec();
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{};base64,{}", mime, b64))
}

/// 获取favicon并转换为base64编码（同步版本）
pub fn fetch_favicon_base64(url: &str) -> AppResult<String> {
    let resp = reqwest::blocking::get(url)?;
    if !resp.status().is_success() {
        return Err(AppError::CustomError(format!("HTTP {}", resp.status())));
    }
    let mime = resp.headers().get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("image/png").to_string();
    let bytes = resp.bytes()?.to_vec();
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{};base64,{}", mime, b64))
}