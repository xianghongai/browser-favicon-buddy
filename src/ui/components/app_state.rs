//! 应用状态组件
//!
//! 提供应用程序状态管理

use std::sync::{Arc, Mutex, atomic::AtomicBool};
use tokio::runtime::Runtime;
use eframe::{egui, App};

use crate::config::AppConfig;
use super::settings::SettingsTab;
use super::file_selector::FileSelector;
use super::progress::ProgressBar;
use super::log_viewer::LogViewer;
use crate::ui::fonts::load_system_fonts;

/// 应用状态结构体
pub struct AppState {
    pub input_path: Option<String>,
    pub log: Arc<Mutex<String>>,
    pub processing: Arc<AtomicBool>,
    pub abort_flag: Arc<AtomicBool>,
    pub progress: Arc<Mutex<(usize, usize)>>, // (当前, 总数)
    pub runtime: Runtime, // Tokio 运行时
    pub config: AppConfig, // 应用配置
    pub new_service_name: String, // 新服务名称（用于UI输入）
    pub new_service_url: String, // 新服务URL模板（用于UI输入）
    pub show_settings_dialog: bool, // 是否显示设置对话框
    pub current_settings_tab: SettingsTab, // 当前选中的设置选项卡
    pub current_locale: String, // 当前语言
    pub available_locales: Vec<String>, // 可用语言列表
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            input_path: None,
            log: Arc::new(Mutex::new(String::new())),
            processing: Arc::new(AtomicBool::new(false)),
            abort_flag: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(Mutex::new((0, 0))),
            runtime: Runtime::new()
                .expect("Failed to create Tokio runtime"),
            config: AppConfig::load(),
            new_service_name: String::new(),
            new_service_url: String::new(),
            show_settings_dialog: false,
            current_settings_tab: SettingsTab::Services,
            current_locale: crate::i18n::get_locale(),
            available_locales: crate::i18n::get_supported_locales(),
        }
    }
}

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 加载系统字体
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            println!("正在加载系统字体...");
            load_system_fonts(ctx);
            println!("字体加载完成");

            // 在 macOS 上输出更多调试信息
            #[cfg(target_os = "macos")]
            {
                println!("macOS 系统字体加载调试信息:");
                println!("当前语言环境: {}", std::env::var("LANG").unwrap_or_else(|_| "未设置".to_string()));
                println!("当前应用语言: {}", crate::i18n::get_locale());
                println!("系统检测到的语言: {}", crate::i18n::detect_system_locale());
            }
        });

        // 设置对话框
        if self.show_settings_dialog {
            let max_width = ctx.screen_rect().width() * 0.8;
            egui::Window::new(crate::i18n::get_message("settings", None))
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .default_width(500.0)
                .default_height(400.0)
                .min_width(300.0)
                .max_width(max_width.min(600.0))
                .show(ctx, |ui| {
                    super::settings::render_settings(self, ui, ctx);
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // 文件选择和设置按钮
            FileSelector::render(self, ui);
            ui.separator();

            // 显示选中的文件
            if let Some(ref path) = self.input_path {
                let mut args = std::collections::HashMap::new();
                args.insert("path".to_string(), path.to_string());
                let selected_file = crate::i18n::get_message("selected_file", Some(args));
                ui.label(&selected_file);
            }

            ui.add_space(10.0);

            // 进度条和控制按钮
            ProgressBar::render(self, ui);

            // 日志显示
            LogViewer::render(self, ui, ctx);
        });
    }
}