//! UI模块
//!
//! 提供图形用户界面相关功能

mod components;
mod fonts;
mod styles;
mod embedded_fonts;

use eframe::egui;
use egui::viewport::IconData;

// 在编译期嵌入 icon.png，便于可执行文件单独分发
const EMBED_ICON_PNG: &[u8] = include_bytes!("../../assets/icon.png");


pub use components::app_state::AppState;
pub use fonts::{load_system_fonts, get_system_font_paths};
pub use styles::create_styled_button;

/// 初始化并运行应用程序
pub fn run_app() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();

    // 为 macOS 添加特定的渲染选项
    #[cfg(target_os = "macos")]
    {
        // 启用硬件加速和抗锯齿
        options.hardware_acceleration = eframe::HardwareAcceleration::Preferred;
        options.multisampling = 4;

        // 设置默认窗口大小略大一些，避免缩放问题
        options.viewport = options.viewport.with_inner_size([800.0, 600.0]);
    }

    // 加载 PNG 作为窗口 icon；优先尝试运行时文件，若不存在则使用内嵌资源
    let icon_bytes = std::fs::read("assets/icon.png").ok().unwrap_or_else(|| EMBED_ICON_PNG.to_vec());
    if let Ok(img) = image::load_from_memory(&icon_bytes) {
        let img = img.to_rgba8();
        let (width, height) = img.dimensions();
        let rgba = img.into_raw();
        let icon = IconData { rgba, width, height };
        options.viewport = options.viewport.with_icon(std::sync::Arc::new(icon));
    }
    eframe::run_native(
        &crate::i18n::get_message("app_title", None),
        options,
        Box::new(|_cc| Ok::<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>>(Box::new(AppState::default()))),
    )
}