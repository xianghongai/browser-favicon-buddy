//! 服务选项卡（Favicon 服务配置）

use eframe::egui;
use crate::ui::styles::create_styled_button;
use super::super::app_state::AppState;

/// 渲染服务选项卡
pub fn render(app: &mut AppState, ui: &mut egui::Ui) {
    // 显示当前服务
    let current_service = &app.config.favicon_service.services[app.config.favicon_service.current_service_index];
    let mut args = std::collections::HashMap::new();
    args.insert("name".to_string(), current_service.name.clone());
    let current_service_text = crate::i18n::get_message("current_service", Some(args));
    ui.label(&current_service_text);
    ui.separator();

    // 服务列表
    let available_services = crate::i18n::get_message("available_services", None);
    ui.label(&available_services);
    egui::Grid::new("favicon_services_grid")
        .num_columns(3)
        .spacing([10.0, 6.0])
        .striped(true)
        .show(ui, |ui| {
            let service_name = crate::i18n::get_message("service_name", None);
            let url_template = crate::i18n::get_message("url_template", None);
            let actions = crate::i18n::get_message("actions", None);
            ui.label(&service_name);
            ui.label(&url_template);
            ui.label(&actions);
            ui.end_row();

            let services: Vec<_> = app.config.favicon_service.services.iter().enumerate()
                .map(|(i, s)| (i, s.name.clone(), s.url_template.clone(), s.is_default, i == app.config.favicon_service.current_service_index))
                .collect();

            for (i, name, url_template, is_default, is_current) in services {
                ui.label(&name);
                ui.label(&url_template);
                ui.horizontal(|ui| {
                    let use_text = crate::i18n::get_message("use_service", None);
                    let use_button = create_styled_button(&use_text, !is_current);
                    if ui.add_enabled(!is_current, use_button).clicked() {
                        app.config.favicon_service.current_service_index = i;
                        if let Err(e) = app.config.save() {
                            let mut log_lock = app.log.lock().unwrap();
                            let mut args = std::collections::HashMap::new();
                            args.insert("error".to_string(), e.to_string());
                            let error_msg = crate::i18n::get_message("config_error", Some(args));
                            log_lock.push_str(&format!("\n{}\n", error_msg));
                        }
                    }

                    if !is_default {
                        let delete_text = crate::i18n::get_message("remove_service", None);
                        let delete_button = create_styled_button(&delete_text, true);
                        if ui.add(delete_button).clicked() {
                            app.config.favicon_service.services.remove(i);
                            if let Err(e) = app.config.save() {
                                let mut log_lock = app.log.lock().unwrap();
                                let mut args = std::collections::HashMap::new();
                                args.insert("error".to_string(), e.to_string());
                                let error_msg = crate::i18n::get_message("config_error", Some(args));
                                log_lock.push_str(&format!("\n{}\n", error_msg));
                            }
                        }
                    }
                });
                ui.end_row();
            }
        });

    ui.separator();

    // 添加新服务
    let add_new_service = crate::i18n::get_message("add_new_service", None);
    ui.label(&add_new_service);
    ui.horizontal(|ui| {
        let name_label = crate::i18n::get_message("name_label", None);
        ui.label(&name_label);
        ui.text_edit_singleline(&mut app.new_service_name);
    });
    ui.horizontal(|ui| {
        let url_template_label = crate::i18n::get_message("url_template_label", None);
        ui.label(&url_template_label);
        ui.add_sized([400.0, 20.0], egui::TextEdit::singleline(&mut app.new_service_url));
    });
    ui.horizontal(|ui| {
        let add_text = crate::i18n::get_message("add_service", None);
        let add_button = create_styled_button(&add_text, true);
        if ui.add(add_button).clicked() {
            if !app.new_service_name.is_empty() && !app.new_service_url.is_empty() {
                app.config.favicon_service.services.push(crate::config::favicon_service::FaviconService {
                    name: app.new_service_name.clone(),
                    url_template: app.new_service_url.clone(),
                    is_default: false,
                });
                if let Err(e) = app.config.save() {
                    let mut log_lock = app.log.lock().unwrap();
                    let mut args = std::collections::HashMap::new();
                    args.insert("error".to_string(), e.to_string());
                    let error_msg = crate::i18n::get_message("config_error", Some(args));
                    log_lock.push_str(&format!("\n{}\n", error_msg));
                } else {
                    app.new_service_name.clear();
                    app.new_service_url.clear();
                }
            } else {
                let mut log_lock = app.log.lock().unwrap();
                let error_msg = crate::i18n::get_message("enter_service_name_url", None);
                log_lock.push_str(&format!("\n{}\n", error_msg));
            }
        }
        ui.add_space(10.0);
        let domain_placeholder = crate::i18n::get_message("domain_placeholder_hint", None);
        ui.label(egui::RichText::new(&domain_placeholder).color(egui::Color32::GRAY));
    });
}
