//! 字体处理模块
//!
//! 提供字体加载和管理功能

use std::fs;
use std::sync::Arc;
use egui::{FontDefinitions, FontFamily, FontData};

/// 加载系统字体
pub fn load_system_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // 尝试加载系统字体
    let font_paths = get_system_font_paths();

    // 加载所有可用字体
    for (name, path) in font_paths {
        if let Ok(font_data) = fs::read(&path) {
            fonts.font_data.insert(name.clone(), Arc::new(FontData::from_owned(font_data)));
            fonts.families.get_mut(&FontFamily::Proportional).unwrap().push(name);
        }
    }

    // 添加 emoji 字体
    #[cfg(target_os = "windows")]
    {
        let emoji_fonts = [
            ("segoe_ui_emoji", "C:/Windows/Fonts/seguiemj.ttf"),
            ("segoe_ui_symbol", "C:/Windows/Fonts/seguisym.ttf"),
        ];
        for (name, path) in emoji_fonts {
            if let Ok(font_data) = fs::read(path) {
                fonts.font_data.insert(name.to_string(), Arc::new(FontData::from_owned(font_data)));
                fonts.families.get_mut(&FontFamily::Proportional).unwrap().push(name.to_string());
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let emoji_fonts = [
            ("apple_color_emoji", "/System/Library/Fonts/Apple Color Emoji.ttc"),
        ];
        for (name, path) in emoji_fonts {
            if let Ok(font_data) = fs::read(path) {
                fonts.font_data.insert(name.to_string(), Arc::new(FontData::from_owned(font_data)));
                fonts.families.get_mut(&FontFamily::Proportional).unwrap().push(name.to_string());
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let emoji_fonts = [
            ("noto_color_emoji", "/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf"),
            ("noto_emoji", "/usr/share/fonts/truetype/noto/NotoEmoji-Regular.ttf"),
        ];
        for (name, path) in emoji_fonts {
            if let Ok(font_data) = fs::read(path) {
                fonts.font_data.insert(name.to_string(), Arc::new(FontData::from_owned(font_data)));
                fonts.families.get_mut(&FontFamily::Proportional).unwrap().push(name.to_string());
            }
        }
    }

    ctx.set_fonts(fonts);
}

/// 获取系统字体路径
pub fn get_system_font_paths() -> Vec<(String, String)> {
    let mut paths = Vec::new();

    #[cfg(target_os = "windows")]
    {
        paths.push(("microsoft_yahei".to_string(), "C:/Windows/Fonts/msyh.ttc".to_string()));
        paths.push(("simhei".to_string(), "C:/Windows/Fonts/simhei.ttf".to_string()));
        paths.push(("simsun".to_string(), "C:/Windows/Fonts/simsun.ttc".to_string()));
    }

    #[cfg(target_os = "macos")]
    {
        paths.push(("pingfang_sc".to_string(), "/System/Library/Fonts/PingFang.ttc".to_string()));
        paths.push(("stkaiti".to_string(), "/System/Library/Fonts/STKaiti.ttc".to_string()));
    }

    #[cfg(target_os = "linux")]
    {
        paths.push(("noto_sans_cjk".to_string(), "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc".to_string()));
        paths.push(("dejavu_sans".to_string(), "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf".to_string()));
        // Ubuntu/Debian
        paths.push(("wqy_microhei".to_string(), "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc".to_string()));
        // CentOS/RHEL
        paths.push(("liberation_sans".to_string(), "/usr/share/fonts/liberation/LiberationSans-Regular.ttf".to_string()));
    }

    paths
}