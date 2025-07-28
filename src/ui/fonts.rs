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

    // 存储成功加载的字体，优先使用这些字体
    let mut loaded_fonts = Vec::new();

    // 加载所有可用字体
    for (name, path) in font_paths {
        if let Ok(font_data) = fs::read(&path) {
            fonts.font_data.insert(name.clone(), Arc::new(FontData::from_owned(font_data)));
            loaded_fonts.push(name.clone());
        }
    }

    // 确保 Proportional 字体系列存在
    if !fonts.families.contains_key(&FontFamily::Proportional) {
        fonts.families.insert(FontFamily::Proportional, Vec::new());
    }

    // 重新组织字体优先级 - 确保中文字体在前面
    #[cfg(target_os = "macos")]
    {
        // 清空默认的字体家族并按优先级添加加载的字体
        fonts.families.get_mut(&FontFamily::Proportional).unwrap().clear();

        // 中文字体优先
        for preferred_font in &["pingfang_sc", "hiragino_sans_gb", "heiti_sc", "stkaiti", "stsong"] {
            if loaded_fonts.contains(&preferred_font.to_string()) {
                fonts.families.get_mut(&FontFamily::Proportional).unwrap().push(preferred_font.to_string());
            }
        }

        // 添加其他已加载的字体
        for font in loaded_fonts {
            if !fonts.families.get(&FontFamily::Proportional).unwrap().contains(&font) {
                fonts.families.get_mut(&FontFamily::Proportional).unwrap().push(font);
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        // 添加所有已加载的字体
        for font in loaded_fonts {
            fonts.families.get_mut(&FontFamily::Proportional).unwrap().push(font);
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

        // 打印字体加载情况，便于调试
        println!("已加载的字体: {:?}", fonts.families.get(&FontFamily::Proportional).unwrap());
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
        // macOS 中文字体路径 - 增加更多常用字体以确保正确显示
        paths.push(("pingfang_sc".to_string(), "/System/Library/Fonts/PingFang.ttc".to_string()));
        paths.push(("pingfang_tc".to_string(), "/System/Library/Fonts/PingFang.ttc".to_string()));
        paths.push(("hiragino_sans_gb".to_string(), "/System/Library/Fonts/Hiragino Sans GB.ttc".to_string()));
        paths.push(("hiragino_sans".to_string(), "/System/Library/Fonts/HiraginoSans.ttc".to_string()));
        paths.push(("heiti_sc".to_string(), "/System/Library/Fonts/STHeiti Light.ttc".to_string()));
        paths.push(("heiti_tc".to_string(), "/System/Library/Fonts/STHeiti Medium.ttc".to_string()));
        paths.push(("stkaiti".to_string(), "/System/Library/Fonts/Kaiti.ttc".to_string()));
        paths.push(("stkaiti_old".to_string(), "/System/Library/Fonts/STKaiti.ttc".to_string()));
        paths.push(("stsong".to_string(), "/System/Library/Fonts/Songti.ttc".to_string()));
        paths.push(("stfangsong".to_string(), "/System/Library/Fonts/STFangsong.ttf".to_string()));

        // 兼容不同版本 macOS 的路径
        paths.push(("sf_pro".to_string(), "/System/Library/Fonts/SF-Pro.ttf".to_string()));
        paths.push(("sf_pro_text".to_string(), "/System/Library/Fonts/SF-Pro-Text-Regular.otf".to_string()));

        // 系统字体目录
        paths.push(("system_font_1".to_string(), "/Library/Fonts/Arial Unicode.ttf".to_string()));
        paths.push(("system_font_2".to_string(), "/Library/Fonts/Microsoft Sans Serif.ttf".to_string()));
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