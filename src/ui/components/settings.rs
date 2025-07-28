//! 设置对话框组件
//!
//! 提供设置界面的功能

use eframe::egui;
use super::app_state::AppState;
// sub-modules moved into `settings/` folder
#[path = "settings/services.rs"] mod services;
#[path = "settings/language.rs"] mod language;
#[path = "settings/import_export.rs"] mod import_export;
#[path = "settings/about.rs"] mod about;

use services::render as render_services_tab;
use language::render as render_language_tab;
use import_export::render as render_import_export_tab;
use about::render as render_about_tab;

/// 设置对话框的选项卡
#[derive(PartialEq)]
pub enum SettingsTab {
    Services,
    Language,
    ImportExport,
    About,
}

/// 渲染设置对话框
pub fn render_settings(app: &mut AppState, ui: &mut egui::Ui, ctx: &egui::Context) {
    // 标签页和关闭按钮
    ui.horizontal(|ui| {
        if ui.selectable_label(app.current_settings_tab == SettingsTab::Services,
            crate::i18n::get_message("services", None)).clicked() {
            app.current_settings_tab = SettingsTab::Services;
        }
        if ui.selectable_label(app.current_settings_tab == SettingsTab::Language,
            crate::i18n::get_message("language", None)).clicked() {
            app.current_settings_tab = SettingsTab::Language;
        }
        if ui.selectable_label(app.current_settings_tab == SettingsTab::ImportExport,
            crate::i18n::get_message("import_export", None)).clicked() {
            app.current_settings_tab = SettingsTab::ImportExport;
        }
        if ui.selectable_label(app.current_settings_tab == SettingsTab::About,
            crate::i18n::get_message("about", None)).clicked() {
            app.current_settings_tab = SettingsTab::About;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(crate::i18n::get_message("close", None)).clicked() {
                app.show_settings_dialog = false;
            }
        });
    });
    ui.separator();

    // 选项卡内容
    match app.current_settings_tab {
        SettingsTab::ImportExport => render_import_export_tab(app, ui),
        SettingsTab::Services => render_services_tab(app, ui),
        SettingsTab::Language => render_language_tab(app, ui, ctx),
        SettingsTab::About => render_about_tab(ui),
    }
}
