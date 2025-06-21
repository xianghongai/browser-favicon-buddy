//! 国际化模块
//!
//! 提供多语言支持功能

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};

use fluent::FluentResource;
use fluent::concurrent::FluentBundle;
use fluent_syntax::parser::parse;
use once_cell::sync::Lazy;
use unic_langid::{langid, LanguageIdentifier};

use crate::errors::AppResult;
use crate::errors::AppError;

// 支持的语言列表
pub const SUPPORTED_LOCALES: &[&str] = &["en", "zh-CN"];

// 默认语言
pub const DEFAULT_LOCALE: &str = "zh-CN";

// ------ 内置语言文件（编译时嵌入） ------
// 这样可在运行时无需依赖外部 locales 目录
const EN_LOCALE_YAML: &str = include_str!("../locales/en.yml");
const ZH_CN_LOCALE_YAML: &str = include_str!("../locales/zh-CN.yml");

fn embedded_locale_content(locale: &str) -> Option<&'static str> {
    match locale {
        "en" => Some(EN_LOCALE_YAML),
        "zh-CN" => Some(ZH_CN_LOCALE_YAML),
        _ => None,
    }
}

// 当前语言
static CURRENT_LOCALE: RwLock<LanguageIdentifier> = RwLock::new(langid!("en"));

// 语言包缓存
type Bundle = FluentBundle<Arc<FluentResource>>;
static BUNDLES: Lazy<RwLock<HashMap<String, Bundle>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// 初始化国际化系统
pub fn init() -> AppResult<()> {
    // 加载所有支持的语言
    for locale in SUPPORTED_LOCALES {
        if let Err(e) = load_locale(locale) {
            eprintln!("警告: 加载语言 {} 失败: {}", locale, e);
        }
    }

    // 尝试设置系统语言
    let system_locale = detect_system_locale();
    if let Err(e) = set_locale(&system_locale) {
        eprintln!("警告: 设置系统语言失败: {}", e);
        // 回退到默认语言
        set_locale(DEFAULT_LOCALE)?;
    }

    Ok(())
}

/// 加载指定语言的资源
fn load_locale(locale: &str) -> AppResult<()> {
    let locale_path = Path::new("locales").join(format!("{}.yml", locale));

    // 优先尝试读取外部文件，若不存在则回退到内置资源
    let content = match fs::read_to_string(&locale_path) {
        Ok(c) => c,
        Err(_) => {
            if let Some(embed) = embedded_locale_content(locale) {
                embed.to_string()
            } else {
                return Err(AppError::CustomError(format!(
                    "语言文件不存在: {}",
                    locale_path.display()
                )));
            }
        }
    };

    // 检查文件是否为空
    if content.trim().is_empty() {
        return Err(AppError::CustomError(format!(
            "语言文件为空: {}",
            locale_path.display()
        )));
    }

    // 解析YAML文件
    let yaml_data: serde_json::Value = serde_yaml::from_str(&content)
        .map_err(|e| AppError::CustomError(format!("解析语言文件失败: {}", e)))?;

    // 转换为FTL格式
    let mut ftl_content = String::new();
    convert_yaml_to_ftl(&yaml_data, "", &mut ftl_content);

    // 解析FTL内容
    let resource = match parse(ftl_content.as_str()) {
        Ok(_) => FluentResource::try_new(ftl_content)
            .map_err(|_| AppError::CustomError("转换FTL资源失败".to_string()))?,
        Err(e) => return Err(AppError::CustomError(format!("解析FTL内容失败: {:?}", e))),
    };

    // 创建语言包
    let lang_id = locale.parse::<LanguageIdentifier>()
        .map_err(|_| AppError::CustomError(format!("无效的语言ID: {}", locale)))?;

    let mut bundle = FluentBundle::new_concurrent(vec![lang_id.clone()]);
    bundle.add_resource(Arc::new(resource))
        .map_err(|_| AppError::CustomError("添加资源到语言包失败".to_string()))?;

    // 保存到缓存
    let mut bundles = BUNDLES.write()
        .map_err(|_| AppError::CustomError("无法获取BUNDLES写锁".to_string()))?;
    bundles.insert(locale.to_string(), bundle);

    Ok(())
}

/// 将YAML数据转换为FTL格式
fn convert_yaml_to_ftl(data: &serde_json::Value, prefix: &str, output: &mut String) {
    match data {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                // 跳过版本信息
                if key == "_version" {
                    continue;
                }

                // 对 key 进行清洗，移除不被 FTL 标识符支持的字符，如 `$`、空格等
                let sanitized_key: String = key.chars()
                    .map(|c| {
                        if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                            c
                        } else {
                            '_' // 使用下划线替换非法字符
                        }
                    })
                    .collect();

                let new_prefix = if prefix.is_empty() {
                    sanitized_key
                } else {
                    format!("{}.{}", prefix, sanitized_key)
                };

                convert_yaml_to_ftl(value, &new_prefix, output);
            }
        },
        serde_json::Value::String(s) => {
            // 处理占位符 %{name} -> { $name }
            let value = s.replace("%{", "{ $").replace("}", " }");
            output.push_str(&format!("{} = {}\n", prefix, value));
        },
        _ => {}
    }
}

/// 设置当前语言
pub fn set_locale(locale: &str) -> AppResult<()> {
    // 检查语言是否已加载
    let bundles = BUNDLES.read()
        .map_err(|_| AppError::CustomError("无法获取BUNDLES读锁".to_string()))?;

    if !bundles.contains_key(locale) {
        return Err(AppError::CustomError(format!("不支持的语言: {}", locale)));
    }

    // 解析语言ID
    let lang_id = locale.parse::<LanguageIdentifier>()
        .map_err(|_| AppError::CustomError(format!("无效的语言ID: {}", locale)))?;

    // 设置当前语言
    let mut current = CURRENT_LOCALE.write()
        .map_err(|_| AppError::CustomError("无法获取CURRENT_LOCALE写锁".to_string()))?;
    *current = lang_id;

    Ok(())
}

/// 获取当前语言
pub fn get_locale() -> String {
    CURRENT_LOCALE.read()
        .map(|current| current.to_string())
        .unwrap_or_else(|_| DEFAULT_LOCALE.to_string())
}

/// 获取支持的语言列表
pub fn get_supported_locales() -> Vec<String> {
    SUPPORTED_LOCALES.iter().map(|s| s.to_string()).collect()
}

/// 获取翻译文本
pub fn get_message(key: &str, args: Option<HashMap<String, String>>) -> String {
    let locale = get_locale();
    let bundles = match BUNDLES.read() {
        Ok(b) => b,
        Err(_) => return key.to_string(),
    };

    // 尝试获取消息，支持语言回退链
    let locales_to_try = get_locale_fallback_chain(&locale);

    for try_locale in locales_to_try {
        if let Some(bundle) = bundles.get(&try_locale) {
            if let Some(msg) = bundle.get_message(key) {
                if let Some(pattern) = msg.value() {
                    // 转换参数
                    let fluent_args = if let Some(arg_map) = args.clone() {
                        let mut fluent_args = fluent::FluentArgs::new();
                        for (k, v) in arg_map {
                            fluent_args.set(k, v);
                        }
                        Some(fluent_args)
                    } else {
                        None
                    };

                    // 格式化消息
                    let mut errors = vec![];
                    return bundle.format_pattern(pattern, fluent_args.as_ref(), &mut errors).to_string();
                }
            }
        }
    }

    // 如果找不到翻译，返回键名
    key.to_string()
}

/// 获取语言回退链
fn get_locale_fallback_chain(locale: &str) -> Vec<String> {
    let mut chain = vec![locale.to_string()];

    // 添加基础语言（例如 zh-CN -> zh）
    if let Some(base_lang) = locale.split('-').next() {
        if base_lang != locale {
            chain.push(base_lang.to_string());
        }
    }

    // 添加默认语言
    if locale != DEFAULT_LOCALE {
        chain.push(DEFAULT_LOCALE.to_string());
    }

    chain
}

/// 获取系统语言
pub fn detect_system_locale() -> String {
    // 尝试从环境变量获取系统语言
    let env_vars = ["LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"];
    for var_name in env_vars.iter() {
        if let Ok(lang_env) = std::env::var(var_name) {
            // 环境变量可能包含多个语言，例如 zh_CN:en_US:en
            for lang in lang_env.split(':') {
                if let Some(locale) = lang.split('.').next() {
                    let locale_str = locale.replace('_', "-");
                    // 完全匹配
                    if SUPPORTED_LOCALES.contains(&locale_str.as_str()) {
                        return locale_str;
                    }
                    // 尝试匹配基础语言 (e.g., "en" from "en-US")
                    if let Some(base_lang) = locale_str.split('-').next() {
                        if SUPPORTED_LOCALES.contains(&base_lang) {
                            return base_lang.to_string();
                        }
                    }
                }
            }
        }
    }

    // 在Windows上尝试使用系统API
    #[cfg(target_os = "windows")]
    {
        // 使用 sys-locale 库替代直接调用 Windows API
        if let Some(locale) = sys_locale::get_locale() {
            let locale_str = locale.replace('_', "-");

            // 完全匹配
            if SUPPORTED_LOCALES.contains(&locale_str.as_str()) {
                return locale_str;
            }

            // 尝试匹配基础语言
            if let Some(base_lang) = locale_str.split('-').next() {
                if SUPPORTED_LOCALES.contains(&base_lang) {
                    return base_lang.to_string();
                }
            }
        }
    }

    // 如果无法检测或不支持，返回默认语言
    DEFAULT_LOCALE.to_string()
}

/// 便捷宏，用于获取翻译文本
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        {
            let msg = $crate::i18n::get_message($key, None);
            msg.to_string()
        }
    };
    ($key:expr, $($k:expr => $v:expr),*) => {
        {
            let mut args = std::collections::HashMap::new();
            $({
                let k_str = $k.to_string();
                let v_str = $v.to_string();
                args.insert(k_str, v_str);
            })*
            let msg = $crate::i18n::get_message($key, Some(args));
            msg.to_string()
        }
    };
}