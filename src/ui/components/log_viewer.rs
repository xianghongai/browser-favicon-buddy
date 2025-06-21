//! 日志查看组件
//!
//! 提供日志显示功能

use eframe::egui;
use std::time::Duration;

use super::app_state::AppState;

/// 日志查看器组件
pub struct LogViewer;

impl LogViewer {
    /// 渲染日志查看器
    pub fn render(app: &mut AppState, ui: &mut egui::Ui, ctx: &egui::Context) {
        let log_title = crate::i18n::get_message("processing_log", None);
        ui.label(&log_title);
        
        // 每3秒刷新一次
        ctx.request_repaint_after(Duration::from_secs(3));

        // 获取国际化关键词
        let success_word = crate::i18n::get_message("success", None);
        let failed_word = crate::i18n::get_message("failed", None);
        let total_word = crate::i18n::get_message("total", None);
        let processing_completed_prefix = format!("{}:", crate::i18n::get_message("processing_completed", None));
        let failed_word_colon = format!("{}:", failed_word);

        ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
            let mut scroll_area = egui::ScrollArea::vertical();
            scroll_area = scroll_area
                .stick_to_bottom(true)
                .auto_shrink([false; 2])
                .max_height(ui.available_height() - 10.0); // 减少底部空白区域

            scroll_area.show(ui, |ui| {
                if let Ok(log) = app.log.lock() {
                    let lines: Vec<&str> = log.lines().collect();
                    let total_lines = lines.len();
                    
                    // 只显示最后1000行日志以提高性能
                    let start_idx = if total_lines > 1000 {
                        total_lines - 1000
                    } else {
                        0
                    };
                    
                    for line in lines.iter().skip(start_idx) {
                        if line.contains("Completed:") && line.contains("Failed:") && line.contains("Total:") {
                            let completed_idx = line.find("Completed:");
                            let failed_idx = line.find(", Failed:");
                            let total_idx = line.find(", Total:");
                            if let (Some(c_idx), Some(f_idx), Some(t_idx)) = (completed_idx, failed_idx, total_idx) {
                                let completed_part = &line[c_idx..f_idx];
                                let failed_part = &line[f_idx + 2..t_idx];
                                let total_part = &line[t_idx + 2..];
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(completed_part).color(egui::Color32::GREEN));
                                    ui.label(", ");
                                    ui.label(egui::RichText::new(failed_part).color(egui::Color32::RED));
                                    ui.label(", ");
                                    ui.label(total_part);
                                });
                                continue;
                            }
                        }
                        if line.contains(&processing_completed_prefix) && line.contains(&success_word) && line.contains(&failed_word) && line.contains(&total_word) {
                            let success_idx = line.find(&success_word);
                            let failed_idx = line.find(&format!(", {}", failed_word));
                            let total_idx = line.find(&format!(", {}", total_word));
                            if let (Some(s_idx), Some(f_idx), Some(t_idx)) = (success_idx, failed_idx, total_idx) {
                                let prefix = &line[..s_idx];
                                let success_part = &line[s_idx..f_idx];
                                let failed_part = &line[f_idx + 2..t_idx];
                                let total_part = &line[t_idx + 2..];
                                ui.horizontal(|ui| {
                                    ui.label(prefix);
                                    ui.label(egui::RichText::new(success_part).color(egui::Color32::GREEN));
                                    ui.label(", ");
                                    ui.label(egui::RichText::new(failed_part).color(egui::Color32::RED));
                                    ui.label(", ");
                                    ui.label(total_part);
                                });
                                continue;
                            }
                        }
                        // 处理单个任务的成功/失败状态
                        if line.contains(&success_word) {
                            let success_idx = line.find(&success_word);
                            if let Some(s_idx) = success_idx {
                                let prefix = &line[..s_idx];
                                let success_part = &success_word;
                                ui.horizontal(|ui| {
                                    ui.label(prefix);
                                    ui.label(egui::RichText::new(success_part).color(egui::Color32::GREEN));
                                });
                                continue;
                            }
                        } else if line.contains(&failed_word_colon) {
                            let failed_idx = line.find(&failed_word_colon);
                            if let Some(f_idx) = failed_idx {
                                let prefix = &line[..f_idx];
                                let failed_part = &line[f_idx..];
                                ui.horizontal(|ui| {
                                    ui.label(prefix);
                                    ui.label(egui::RichText::new(failed_part).color(egui::Color32::RED));
                                });
                                continue;
                            }
                        }
                        ui.label(egui::RichText::new(*line));
                    }
                }
            });
        });
    }
}