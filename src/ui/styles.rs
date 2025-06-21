//! UI样式模块
//!
//! 提供UI样式相关功能

use egui;

/// 辅助函数：创建统一样式的按钮
pub fn create_styled_button(text: &str, enabled: bool) -> egui::Button {
    let mut button = egui::Button::new(text);
    if enabled {
        button = button.fill(egui::Color32::from_rgb(210, 210, 210)); // 启用状态为灰色 #d2d2d2
    }
    button
}