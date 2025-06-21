//! 关于选项卡组件
//!
//! 显示软件基本信息

use eframe::egui;

/// 渲染关于选项卡
pub fn render(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading(env!("CARGO_PKG_NAME"));
        ui.label(crate::i18n::get_message("app_description", None));
        ui.label(env!("CARGO_PKG_VERSION"));
        ui.label(env!("CARGO_PKG_AUTHORS"));
        ui.add_space(10.0);

        if let Some(repo) = option_env!("CARGO_PKG_REPOSITORY") {
            if !repo.is_empty() {
                ui.hyperlink_to(crate::i18n::get_message("repository", None), repo);
            } else {
                ui.label(crate::i18n::get_message("repository_unavailable", None));
            }
        } else {
            ui.label(crate::i18n::get_message("repository_unavailable", None));
        }
    });
}
