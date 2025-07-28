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
        // 创建一个统一的标签样式
        let _tab_height = 28.0; // 使用前缀下划线避免未使用变量的警告

        // 服务标签
        let services_text = crate::i18n::get_message("services", None);
        // 使用更简单的方式，避免复杂的嵌套和兼容性问题
        if ui.selectable_label(
            app.current_settings_tab == SettingsTab::Services,
            egui::RichText::new(services_text).size(14.0)
        ).clicked() {
            app.current_settings_tab = SettingsTab::Services;
        }

        // 语言标签
        let language_text = crate::i18n::get_message("language", None);
        if ui.selectable_label(
            app.current_settings_tab == SettingsTab::Language,
            egui::RichText::new(language_text).size(14.0)
        ).clicked() {
            app.current_settings_tab = SettingsTab::Language;
        }

        // 导入导出标签
        let import_export_text = crate::i18n::get_message("import_export", None);
        if ui.selectable_label(
            app.current_settings_tab == SettingsTab::ImportExport,
            egui::RichText::new(import_export_text).size(14.0)
        ).clicked() {
            app.current_settings_tab = SettingsTab::ImportExport;
        }

        // 关于标签
        let about_text = crate::i18n::get_message("about", None);
        if ui.selectable_label(
            app.current_settings_tab == SettingsTab::About,
            egui::RichText::new(about_text).size(14.0)
        ).clicked() {
            app.current_settings_tab = SettingsTab::About;
        }        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 先创建一个临时变量来存储消息文本，然后再传递引用给按钮
            let close_text = crate::i18n::get_message("close", None);
            let close_button = crate::ui::create_styled_button(&close_text, true);
            if ui.add(close_button).clicked() {
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
