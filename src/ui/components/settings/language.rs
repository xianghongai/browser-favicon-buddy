//! 语言选项卡

use eframe::egui;
use super::super::app_state::AppState;

/// 渲染语言选项卡
pub fn render(app: &mut AppState, ui: &mut egui::Ui, ctx: &egui::Context) {
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_salt("language_selector")
            .selected_text(&app.config.language.language)
            .show_ui(ui, |ui| {
                for locale in &app.available_locales {
                    let is_selected = &app.config.language.language == locale;
                    if ui.selectable_label(is_selected, locale).clicked() && !is_selected {
                        if let Err(e) = crate::i18n::set_locale(locale) {
                            let mut log_lock = app.log.lock().unwrap();
                            let mut args = std::collections::HashMap::new();
                            args.insert("error".to_string(), e.to_string());
                            let error_msg = crate::i18n::get_message("error_setting_locale", Some(args));
                            log_lock.push_str(&format!("\n{}\n", error_msg));
                        } else {
                            app.config.language.set_language(locale.clone());
                            app.current_locale = locale.clone();
                            if let Err(e) = app.config.save() {
                                let mut log_lock = app.log.lock().unwrap();
                                log_lock.push_str(&format!("\n保存语言设置失败: {}\n", e));
                            }
                            ctx.request_repaint();
                        }
                    }
                }
            });
    });
}
