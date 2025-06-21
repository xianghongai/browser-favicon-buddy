//! 文件选择组件
//!
//! 提供文件选择界面的功能

use eframe::egui;
use rfd::FileDialog;
use std::sync::atomic::Ordering;

use crate::ui::styles::create_styled_button;
use super::app_state::AppState;

/// 文件选择器组件
pub struct FileSelector;

impl FileSelector {
    /// 渲染文件选择器
    pub fn render(app: &mut AppState, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // 选择文件按钮
            let select_text = crate::i18n::get_message("select_bookmark_file", None);
            let select_enabled = !app.processing.load(Ordering::Relaxed);
            let select_button = create_styled_button(&select_text, select_enabled);
            if ui.add_enabled(select_enabled, select_button).clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("HTML", &["html"])
                    .pick_file() {
                    app.input_path = Some(path.display().to_string());
                }
            }

            // 设置按钮
            let settings_text = crate::i18n::get_message("settings", None);
            let settings_enabled = !app.processing.load(Ordering::Relaxed);
            let settings_button = create_styled_button(&settings_text, settings_enabled);
            if ui.add_enabled(settings_enabled, settings_button).clicked() {
                app.show_settings_dialog = true;
            }
        });
    }
}