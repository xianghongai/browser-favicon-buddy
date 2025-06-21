use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use std::fs;
use regex::Regex;
use chrono::Local;
use url::Url;

use crate::errors::{AppError, AppResult};
use crate::config::AppConfig;
use super::cache::FaviconCache;

/// 从URL中提取域名
fn extract_domain(url: &str) -> Option<String> {
    if let Ok(parsed_url) = Url::parse(url) {
        if let Some(host) = parsed_url.host_str() {
            return Some(host.to_string());
        }
    }
    None
}

/// 保存缓存到磁盘
fn save_cache(cache: &HashMap<String, Option<String>>, cache_path: &str, log: &Arc<Mutex<String>>) {
    let cache = FaviconCache(cache.clone());
    if let Err(e) = fs::write(cache_path, serde_json::to_string_pretty(&cache).unwrap_or_default()) {
        if let Ok(mut log_lock) = log.lock() {
            log_lock.push_str(&format!("Failed to write cache file: {}\n", e));
        }
    }
}

/// 处理书签文件，为其中的链接添加favicon
pub async fn process_bookmarks(input: &str, output: &str, log: Arc<Mutex<String>>, abort_flag: Arc<AtomicBool>, progress: Arc<Mutex<(usize, usize)>>) -> AppResult<()> {
    // favicon 缓存文件路径
    let cache_path = super::cache::get_cache_path();
    // 加载磁盘缓存
    let mut favicon_cache: HashMap<String, Option<String>> = if let Ok(data) = fs::read_to_string(&cache_path) {
        serde_json::from_str::<FaviconCache>(&data).map(|c| c.0).unwrap_or_default()
    } else {
        HashMap::new()
    };

    // 1. 读取 HTML 文件
    if let Ok(mut log_lock) = log.lock() {
        log_lock.push_str("\n----------------------------------------\n");
        log_lock.push_str(&format!("[{}] {}\n", Local::now().format("%Y-%m-%d %H:%M:%S"), crate::i18n::get_message("starting_to_process", None)));
    }
    let html_str = match fs::read_to_string(input) {
        Ok(s) => s,
        Err(e) => {
            if let Ok(mut log_lock) = log.lock() {
                log_lock.push_str(&format!("Failed to read file: {}\n", e));
            }
            return Err(AppError::FileError(e));
        }
    };
    if let Ok(mut log_lock) = log.lock() {
        log_lock.push_str(&format!("Successfully read bookmarks file, {} bytes\n", html_str.len()));
    }

    // 2. 查找所有书签链接
    let re_a = Regex::new(r#"<A\b[^>]*HREF\s*=\s*['"](.*?)['"][^>]*>"#).unwrap();
    let matches: Vec<_> = re_a.find_iter(&html_str).collect();
    let total = matches.len();
    if let Ok(mut progress_lock) = progress.lock() {
        *progress_lock = (0, total);
    }
    if let Ok(mut log_lock) = log.lock() {
        log_lock.push_str(&format!("{} {} {}\n", crate::i18n::get_message("found", None), total, crate::i18n::get_message("bookmarks", None)));
    }

    // 3. 处理每个书签链接
    let mut processed_html = html_str.clone();
    let mut offset = 0;
    let config = AppConfig::load();
    let mut processed = 0;
    let mut success_count = 0;
    let mut failed_count = 0;
    let mut last_save = 0;

    for mat in matches {
        // 检查是否需要中止
        if abort_flag.load(Ordering::Relaxed) {
            if let Ok(mut log_lock) = log.lock() {
                log_lock.push_str(&format!("{}\n", crate::i18n::get_message("processing_aborted_by_user", None)));
            }
            // 中止时保存缓存
            save_cache(&favicon_cache, &cache_path, &log);
            return Ok(());
        }

        // 提取URL
        let url = &mat.as_str();
        let href_match = Regex::new(r#"HREF\s*=\s*['"](.*?)['"]"#).unwrap()
            .find(url)
            .ok_or_else(|| AppError::CustomError("Failed to extract HREF".to_string()))?;
        let url_str = &url[href_match.start() + 6..href_match.end() - 1];

        // 提取域名
        if let Some(domain) = extract_domain(url_str) {
            // 获取favicon
            let favicon = if let Ok(mut log_lock) = log.lock() {
                log_lock.push_str(&format!("[{:>3}/{}] {} {}... ", processed, total, crate::i18n::get_message("fetching", None), domain));

                let result = if let Some(cached) = favicon_cache.get(&domain) {
                    if let Some(favicon) = cached {
                        log_lock.push_str(&format!("{}\n", crate::i18n::get_message("success", None)));
                        success_count += 1;
                        Some(favicon.clone())
                    } else {
                        log_lock.push_str(&format!("{}: {}\n", crate::i18n::get_message("failed", None), crate::i18n::get_message("last_request_failed", None)));
                        failed_count += 1;
                        None
                    }
                } else {
                    // 从网络获取favicon
                    let favicon_url = config.get_favicon_url(&domain);
                    match super::fetch::fetch_favicon_base64_async(&favicon_url).await {
                        Ok(favicon) => {
                            favicon_cache.insert(domain.clone(), Some(favicon.clone()));
                            log_lock.push_str(&format!("{}\n", crate::i18n::get_message("success", None)));
                            success_count += 1;
                            Some(favicon)
                        }
                        Err(e) => {
                            log_lock.push_str(&format!("{}: {}\n", crate::i18n::get_message("failed", None), e));
                            favicon_cache.insert(domain.clone(), None);
                            failed_count += 1;
                            None
                        }
                    }
                };
                result
            } else {
                None
            };

            // 添加favicon到书签链接
            if let Some(favicon) = favicon {
                let icon_tag = format!(" ICON=\"{}\"", favicon);
                let new_url = url.replace(">", &format!("{}>" , icon_tag));
                processed_html.replace_range(mat.start() + offset..mat.end() + offset, &new_url);
                offset += icon_tag.len();
            }

            // 更新进度
            processed += 1;
            if let Ok(mut progress_lock) = progress.lock() {
                *progress_lock = (processed, total);
            }
            if processed % 10 == 0 || processed == total {
                if let Ok(mut log_lock) = log.lock() {
                    log_lock.push_str(&format!("{}: {}/{} ({:.1}%)\n", crate::i18n::get_message("processing", None),
                        processed,
                        total,
                        (processed as f32 / total as f32) * 100.0
                    ));
                }
            }

            // 每处理50个书签保存一次缓存
            if processed - last_save >= 50 {
                save_cache(&favicon_cache, &cache_path, &log);
                last_save = processed;
            }
        }
    }

    // 4. 保存更新后的HTML文件
    if let Err(e) = fs::write(output, &processed_html) {
        if let Ok(mut log_lock) = log.lock() {
            log_lock.push_str(&format!("Failed to save output file: {}\n", e));
        }
        return Err(AppError::FileError(e));
    }

    // 5. 保存最终的缓存
    save_cache(&favicon_cache, &cache_path, &log);

    // 6. 完成处理
    if let Ok(mut log_lock) = log.lock() {
        if abort_flag.load(Ordering::Relaxed) {
            log_lock.push_str(&format!("\n[Stop] Saved: {}\nCompleted: {} bookmarks, Failed: {} bookmarks, Total: {} bookmarks\n",
                output, success_count, failed_count, total));
        } else {
            {
                let mut args = std::collections::HashMap::new();
                args.insert("success".to_string(), success_count.to_string());
                args.insert("failed".to_string(), failed_count.to_string());
                args.insert("total".to_string(), total.to_string());
                let summary = crate::i18n::get_message("processing_completed_summary", Some(args));
                let mut path_args = std::collections::HashMap::new();
                path_args.insert("path".to_string(), output.to_string());
                let saved = crate::i18n::get_message("saved_to_path", Some(path_args));
                log_lock.push_str(&format!("\n{}\n{}\n", summary, saved));
            }

        }
    }

    Ok(())
}