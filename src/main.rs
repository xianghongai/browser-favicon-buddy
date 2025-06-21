//! Favicon Buddy 主程序
//!
//! 一个用于处理书签HTML文件并添加favicon的工具

#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

use browser_favicon_buddy::{ui, i18n, config::AppConfig};

fn main() -> eframe::Result<()> {
    // 初始化国际化系统
    if let Err(e) = i18n::init() {
        eprintln!("初始化国际化系统失败: {}", e);
    }

    // 加载配置
    let config = AppConfig::load();

    // 优先使用配置中的语言设置
    if let Err(e) = i18n::set_locale(&config.language.language) {
        eprintln!("设置配置语言 {} 失败: {}, 尝试使用系统语言", config.language.language, e);
        // 如果配置语言设置失败，尝试使用系统语言
        let system_locale = i18n::detect_system_locale();
        if i18n::SUPPORTED_LOCALES.contains(&system_locale.as_str()) {
            if let Err(e) = i18n::set_locale(&system_locale) {
                eprintln!("设置系统语言 {} 失败: {}, 将使用默认语言 {}", system_locale, e, i18n::DEFAULT_LOCALE);
                // 尝试设置默认语言
                if let Err(e_default) = i18n::set_locale(i18n::DEFAULT_LOCALE) {
                    eprintln!("设置默认语言 {} 失败: {}", i18n::DEFAULT_LOCALE, e_default);
                }
            }
        } else {
            eprintln!("系统语言 {} 不被支持, 将使用默认语言 {}", system_locale, i18n::DEFAULT_LOCALE);
            // 尝试设置默认语言
            if let Err(e_default) = i18n::set_locale(i18n::DEFAULT_LOCALE) {
                eprintln!("设置默认语言 {} 失败: {}", i18n::DEFAULT_LOCALE, e_default);
            }
        }
    }

    // 运行应用
    ui::run_app()
}

