//! 导入导出选项卡

use eframe::egui;
use rfd::FileDialog;
use chrono;
use crate::ui::styles::create_styled_button;
use crate::config::{AppConfig, ConfigImportExport};
use super::super::app_state::AppState;

/// 渲染导入导出选项卡
pub fn render(app: &mut AppState, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        // 导入导出配置
        ui.horizontal(|ui| {
            let export_text = crate::i18n::get_message("export_config", None);
            let export_button = create_styled_button(&export_text, true);
            if ui.add(export_button).clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    let timestamp = chrono::Local::now().format("%Y-%m-%d-%H%M%S").to_string();
                    let filename = format!("favicon-buddy-config-{}.json", timestamp);
                    let filepath = path.join(filename);
                    match app.config.export_config(filepath.to_str().unwrap()) {
                        Ok(result) => {
                            let mut log_lock = app.log.lock().unwrap();
                            let success_msg = format!("{}\n{}: {}", result.message, crate::i18n::get_message("export_path", None), filepath.display());
                            ui.add(egui::Label::new(egui::RichText::new(&success_msg).color(egui::Color32::GREEN)));
                            log_lock.push_str("\n");
                            log_lock.push_str(&success_msg);
                            log_lock.push_str("\n");
                        }
                        Err(e) => {
                            let mut log_lock = app.log.lock().unwrap();
                            let mut args = std::collections::HashMap::new();
                            args.insert("error".to_string(), e.to_string());
                            let error_msg = crate::i18n::get_message("config_export_error", Some(args));
                            ui.add(egui::Label::new(egui::RichText::new(&error_msg).color(egui::Color32::RED)));
                            log_lock.push_str("\n");
                            log_lock.push_str(&error_msg);
                            log_lock.push_str("\n");
                        }
                    }
                }
            }

            let import_text = crate::i18n::get_message("import_config", None);
            let import_button = create_styled_button(&import_text, true);
            if ui.add(import_button).clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_file_name("config.json")
                    .pick_file() {
                    match AppConfig::import_config(path.to_str().unwrap()) {
                        Ok((imported_config, result)) => {
                            app.config = imported_config;
                            let mut log_lock = app.log.lock().unwrap();
                            log_lock.push_str(&format!("\n{}\n", result.message));
                        }
                        Err(e) => {
                            let mut log_lock = app.log.lock().unwrap();
                            let mut args = std::collections::HashMap::new();
                            args.insert("error".to_string(), e.to_string());
                            let error_msg = crate::i18n::get_message("config_import_error", Some(args));
                            log_lock.push_str(&format!("\n{}\n", error_msg));
                        }
                    }
                }
            }
        });

        ui.separator();

        // 导入导出缓存
        ui.horizontal(|ui| {
            let export_text = crate::i18n::get_message("export_cache", None);
            let export_button = create_styled_button(&export_text, true);
            if ui.add(export_button).clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    let timestamp = chrono::Local::now().format("%Y-%m-%d-%H%M%S").to_string();
                    let filename = format!("favicon-buddy-favicon_cache-{}.json", timestamp);
                    let filepath = path.join(filename);
                    match app.config.export_cache(filepath.as_path()) {
                        Ok(result) => {
                            let mut log_lock = app.log.lock().unwrap();
                            let success_msg = format!("{}\n{}: {}", result.message, crate::i18n::get_message("export_path", None), filepath.display());
                            ui.add(egui::Label::new(egui::RichText::new(&success_msg).color(egui::Color32::GREEN)));
                            log_lock.push_str("\n");
                            log_lock.push_str(&success_msg);
                            log_lock.push_str("\n");
                        }
                        Err(e) => {
                            let mut log_lock = app.log.lock().unwrap();
                            let mut args = std::collections::HashMap::new();
                            args.insert("error".to_string(), e.to_string());
                            let error_msg = crate::i18n::get_message("cache_export_error", Some(args));
                            ui.add(egui::Label::new(egui::RichText::new(&error_msg).color(egui::Color32::RED)));
                            log_lock.push_str("\n");
                            log_lock.push_str(&error_msg);
                            log_lock.push_str("\n");
                        }
                    }
                }
            }

            let import_text = crate::i18n::get_message("import_cache", None);
            let import_button = create_styled_button(&import_text, true);
            if ui.add(import_button).clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_file_name("favicon_cache.json")
                    .pick_file() {

                    // 首先验证文件格式
                    let validation_result = crate::config::import_export::CacheData::validate_file(path.as_path());

                    // 如果验证通过，尝试导入
                    match validation_result {
                        Ok(_) => {
                            match app.config.import_cache(path.as_path()) {
                                Ok(result) => {
                                    let mut log_lock = app.log.lock().unwrap();
                                    let success_msg = format!("{}", result.message);
                                    ui.add(egui::Label::new(egui::RichText::new(&success_msg).color(egui::Color32::GREEN)));
                                    log_lock.push_str("\n");
                                    log_lock.push_str(&success_msg);
                                    log_lock.push_str("\n");
                                }
                                Err(e) => {
                                    let mut log_lock = app.log.lock().unwrap();
                                    let mut args = std::collections::HashMap::new();
                                    args.insert("error".to_string(), e.to_string());
                                    let error_msg = crate::i18n::get_message("cache_import_error", Some(args));
                                    ui.add(egui::Label::new(egui::RichText::new(&error_msg).color(egui::Color32::RED)));
                                    log_lock.push_str("\n");
                                    log_lock.push_str(&error_msg);
                                    log_lock.push_str("\n");
                                }
                            }
                        },
                        Err(e) => {
                            let mut log_lock = app.log.lock().unwrap();
                            let mut args = std::collections::HashMap::new();
                            args.insert("error".to_string(), e.to_string());
                            let error_msg = crate::i18n::get_message("cache_import_error", Some(args));
                            ui.add(egui::Label::new(egui::RichText::new(&error_msg).color(egui::Color32::RED)));
                            log_lock.push_str("\n");
                            log_lock.push_str(&error_msg);
                            log_lock.push_str("\n");
                        }
                    }
                }
            }
        });
    });
}
