//! 进度显示组件
//!
//! 提供进度条和控制按钮的功能

use std::sync::atomic::Ordering;
use eframe::egui;

use crate::ui::styles::create_styled_button;
use super::app_state::AppState;

/// 进度条组件
pub struct ProgressBar;

impl ProgressBar {
    /// 渲染进度条和控制按钮
    pub fn render(app: &mut AppState, ui: &mut egui::Ui) {
        // 处理按钮
        ui.horizontal(|ui| {
            // 开始处理按钮
            let start_text = crate::i18n::get_message("start_processing", None);
            let start_enabled = !app.processing.load(Ordering::Relaxed) && app.input_path.is_some();
            let start_button = create_styled_button(&start_text, start_enabled);
            if ui.add_enabled(start_enabled, start_button).clicked() {
                if let Some(input) = &app.input_path {
                    let input = input.clone();
                    let output = crate::utils::generate_output_filename(&input);
                    app.processing.store(true, Ordering::Relaxed);
                    app.abort_flag.store(false, Ordering::Relaxed);
                    
                    // 克隆所有需要的Arc<Mutex>
                    let log = app.log.clone();
                    let abort_flag = app.abort_flag.clone();
                    let progress = app.progress.clone();
                    let processing = app.processing.clone();
                    
                    // 在新线程中执行异步任务
                    std::thread::spawn(move || {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        rt.block_on(async {
                            if let Err(e) = crate::favicon::process_bookmarks(&input, &output, log.clone(), abort_flag.clone(), progress.clone()).await {
                                if let Ok(mut log_lock) = log.lock() {
                                    use std::collections::HashMap;
                                    let mut args = HashMap::new();
                                    args.insert("error".to_string(), e.to_string());
                                    let msg = crate::i18n::get_message("processing_error", Some(args));
                                    log_lock.push_str(&format!("{}\n", msg));
                                }
                            }
                            processing.store(false, Ordering::Relaxed);
                        });
                    });
                }
            }

            // 停止处理按钮
            let stop_text = crate::i18n::get_message("stop_processing", None);
            let stop_enabled = app.processing.load(Ordering::Relaxed);
            let stop_button = create_styled_button(&stop_text, stop_enabled);
            if ui.add_enabled(stop_enabled, stop_button).clicked() {
                app.abort_flag.store(true, Ordering::Relaxed);
            }

            // 清除日志按钮
            let clear_text = crate::i18n::get_message("clear_log", None);
            let clear_enabled = !app.processing.load(Ordering::Relaxed);
            let clear_button = create_styled_button(&clear_text, clear_enabled);
            if ui.add_enabled(clear_enabled, clear_button).clicked() {
                let mut log_lock = app.log.lock().unwrap();
                log_lock.clear();
                let log_cleared = crate::i18n::get_message("log_cleared", None);
                 log_lock.push_str(&format!("[{}] {}\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), log_cleared));
            }
        });

        // 进度条或分隔线
        let (cur, total) = *app.progress.lock().unwrap();
        let processing = app.processing.load(Ordering::Relaxed);
        if processing && total > 0 {
            let percent = cur as f32 / total as f32;
            ui.add(egui::ProgressBar::new(percent).desired_height(4.0));
        } else {
            ui.separator();
        }
        ui.add_space(10.0);
    }
}