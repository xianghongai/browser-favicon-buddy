//! UI样式模块
//!
//! 提供UI样式相关功能

use egui;

/// 辅助函数：创建统一样式的按钮
pub fn create_styled_button(text: &str, enabled: bool) -> egui::Button {
    // 创建带有文本大小的按钮（不使用不兼容的 centered_horizontal 方法）
    let mut button = egui::Button::new(egui::RichText::new(text).size(14.0));

    if enabled {
        button = button.fill(egui::Color32::from_rgb(210, 210, 210)); // 启用状态为灰色 #d2d2d2
    }

    // 设置按钮内边距，使文字垂直居中
    button = button.min_size(egui::vec2(0.0, 28.0));
    // 不使用已废弃的 rounding 方法，egui 的较新版本会有默认的圆角

    // 在macOS上增加额外的垂直居中修正
    #[cfg(target_os = "macos")]
    {
        button = button.min_size(egui::vec2(0.0, 32.0)); // macOS上需要更大的高度
        button = button.padding(egui::vec2(12.0, 8.0)); // 增加内边距
    }

    button
}