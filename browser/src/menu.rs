use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{TokioRuntimeHandle, AsyncComputeTaskPool};
use futures_lite::future;


// main.rs で定義したリソースやコンポーネントをuseする
use crate::{CurrentUrl, HtmlContent, FetchHtmlTask, ShowHtmlViewer, ShowOptionWindow, OtherAI, ShowWarningWindow, ShowMessageWindow, ShowSecurityWindow, ShowFfmpegWindow};

pub fn setup_ui_panel(mut current_url: ResMut<CurrentUrl>) {
    // 初期URLを設定
    current_url.0 = "https://example.com".to_string();
}

#[derive(Default, Resource)]
pub struct CrimeReportData {
    pub message: String, // 犯した罪に対するメッセージ
}

#[derive(Default, Resource)]
pub struct SafetyMetrics {
    pub social_safety_score: f32, // 社会安全度 (例: 0.0から100.0)
    pub criminality_coefficient: f32, // 犯罪者係数 (例: 0.0から1.0, 高いほど危険)
}

// URL入力とリクエストをトリガーするシステム
pub fn main_input_system(
    mut contexts: EguiContexts,
    mut current_url: ResMut<CurrentUrl>,
    mut commands: Commands,
    tokio_runtime: Res<TokioRuntimeHandle>,
    mut show_html_viewer: ResMut<ShowHtmlViewer>,
    mut show_option_window: ResMut<ShowOptionWindow>,
    mut show_security_window: ResMut<ShowSecurityWindow>,
    mut show_message_window: ResMut<ShowMessageWindow>,
    mut show_ffmpeg_window: ResMut<ShowFfmpegWindow>,
    mut show_warning_window: ResMut<ShowWarningWindow>,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("url_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("URL:");
            let response = ui.text_edit_singleline(&mut current_url.0);
            if ui.button("Toggle HTML Viewer").clicked() {
                show_html_viewer.0 = !show_html_viewer.0;
            }
                if response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                info!("URL entered: {}", current_url.0);
                let url_to_fetch = current_url.0.clone();
                let tokio_handle_clone = tokio_runtime.0.clone(); // Handle をクローン
                let thread_pool = AsyncComputeTaskPool::get();

                let task = thread_pool.spawn(async move {
                    tokio_handle_clone.spawn(async move { // ★★★ この spawn が重要 ★★★
                        info!("Attempting to fetch: {}", url_to_fetch);
                        let fetch_result = reqwest::get(&url_to_fetch).await;

                        match fetch_result {
                            Ok(res) => {
                                if res.status().is_success() {
                                    match res.text().await {
                                        Ok(text) => Ok(text),
                                        Err(e) => Err(format!("Failed to get text from response: {}", e)),
                                    }
                                } else {
                                    Err(format!("HTTP Error: {}", res.status()))
                                }
                            },
                            Err(e) => {
                                Err(format!("Request failed: {}", e))
                            }
                        }
                    }).await.expect("Tokio task join error") // Tokio task の結果を待つ
                });

                commands.spawn(FetchHtmlTask(task));
            }
            if ui.button("P2P").clicked() {
                show_message_window.0 = !show_message_window.0;
            }
            if ui.button("Ffmpeg").clicked() {
                show_ffmpeg_window.0 = !show_ffmpeg_window.0;
            }
            if ui.button("Opption").clicked() {
                show_option_window.0 = !show_option_window.0;
            }
            if ui.button("Security").clicked() {
                show_security_window.0 = !show_security_window.0;
            }
            if ui.button("warning").clicked() {
                show_warning_window.0 = !show_warning_window.0;
            }
        });
    });
}

// HTMLフェッチタスクの完了を監視し、結果を処理するシステム
pub fn poll_fetch_html_task(
    mut commands: Commands,
    mut query_tasks: Query<(Entity, &mut FetchHtmlTask)>,
    mut html_content: ResMut<HtmlContent>,
) {
    for (entity, mut task) in &mut query_tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            let mut content = html_content.0.lock().unwrap();
            match result {
                Ok(html_text) => {
                    info!("HTML fetch successful for entity {:?}", entity);
                    *content = html_text;
                }
                Err(e) => {
                    error!("HTML fetch failed for entity {:?}: {}", entity, e);
                    *content = format!("Error: {}", e); // エラーメッセージを表示
                }
            }
            commands.entity(entity).despawn(); // タスクエンティティを削除
        }
    }
}

// 取得したHTMLコンテンツをEguiウィンドウに表示するシステム
pub fn html_viewer_system(
    mut contexts: EguiContexts,
    html_content: Res<HtmlContent>,
    show_html_viewer: Res<ShowHtmlViewer>,
) {
    let ctx = contexts.ctx_mut();
    if show_html_viewer.0 {
        egui::Window::new("Html Context View")
        .default_size(egui::vec2(600.0, 400.0))
            .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
            let content = html_content.0.lock().unwrap();
            ui.label(egui::RichText::new(content.as_str()).monospace()); // monospaceで表示
            });
        });
    }
}

pub fn option_window(
    mut contexts: EguiContexts,
    show_option_window: Res<ShowOptionWindow>,
    mut other_ai_res: ResMut<OtherAI>,
) {
    let ctx = contexts.ctx_mut();
    if show_option_window.0 {
        egui::Window::new("Option")
        .default_size(egui::vec2(600.0, 400.0))
            .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
            if ui.button("GPT (Option 1)").clicked() {
                info!("GPT Option 1 clicked!");
            }
            if ui.button("OSAI (Option)").clicked() {
                info!("GPT Option 2 clicked!");
            }
            ui.label("Other AI API Key:");
            ui.text_edit_singleline(&mut other_ai_res.api_key);
        });
    });
    }
}

pub fn message_window(
    mut contexts: EguiContexts,
    show_option_window: Res<ShowOptionWindow>,
    mut other_ai_res: ResMut<OtherAI>,
) {
    let ctx = contexts.ctx_mut();
    if show_option_window.0 {
        egui::Window::new("Option")
        .default_size(egui::vec2(600.0, 400.0))
            .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
            if ui.button("GPT (Option 1)").clicked() {
                info!("GPT Option 1 clicked!");
            }
            if ui.button("OSAI (Option)").clicked() {
                info!("GPT Option 2 clicked!");
            }
            ui.label("Other AI API Key:");
            ui.text_edit_singleline(&mut other_ai_res.api_key);
        });
    });
    }
}

pub fn warning_window(
    mut contexts: EguiContexts,
    show_option_window: Res<ShowOptionWindow>,
) {
    let ctx = contexts.ctx_mut();
    if show_option_window.0 {
        egui::Window::new("Option")
        .default_size(egui::vec2(600.0, 400.0))
            .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label("Warning list");
            if ui.button("GPT (Option 1)").clicked() {
                info!("GPT Option 1 clicked!");
            }
        });
    });
    }
}

pub fn Security_window(
    mut contexts: EguiContexts,
    show_security_window: Res<ShowSecurityWindow>, // このリソースでウィンドウの表示/非表示を切り替える
    mut crime_report_data: ResMut<CrimeReportData>, // 犯した罪に対するメッセージのリソース
    mut safety_metrics: ResMut<SafetyMetrics>,     // 社会安全度と犯罪者係数のリソース
) {
    let ctx = contexts.ctx_mut(); // Eguiコンテキストを取得

    // ShowSecurityWindowがtrueの場合のみウィンドウを表示
    if show_security_window.0 {
        egui::Window::new("社会安全度レポート") // ウィンドウのタイトルを変更
            .default_size(egui::vec2(600.0, 400.0)) // ウィンドウのデフォルトサイズ
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("現在の社会状況"); // 見出し

                    ui.add_space(10.0); // 垂直方向のスペースを追加

                    ui.label("犯した罪に対するメッセージ:");
                    // メッセージは複数行入力できるようにします
                    ui.text_edit_multiline(&mut crime_report_data.message);

                    ui.add_space(20.0);

                    // 社会安全度のスライダー
                    ui.horizontal(|ui| {
                        ui.label("社会安全度:");
                        // 0.0から100.0の範囲でスライダーを設定
                        ui.add(egui::Slider::new(&mut safety_metrics.social_safety_score, 0.0..=100.0).text("点"));
                    });

                    ui.add_space(10.0);

                    // 犯罪者係数のスライダー
                    ui.horizontal(|ui| {
                        ui.label("犯罪者係数:");
                        // 0.0から1.0の範囲でスライダーを設定
                        ui.add(egui::Slider::new(&mut safety_metrics.criminality_coefficient, 0.0..=1.0).text("（高いほど危険）"));
                    });

                    ui.add_space(20.0);

                    // レポート更新ボタン
                    if ui.button("レポートを更新").clicked() {
                        info!("社会状況レポートが更新されました！");
                        info!("メッセージ: {}", crime_report_data.message);
                        info!("社会安全度: {}", safety_metrics.social_safety_score);
                        info!("犯罪者係数: {}", safety_metrics.criminality_coefficient);
                        // ここで、これらの更新された値をアプリケーションの他の部分で使用したり、
                        // 永続化（ファイル保存やネットワーク送信など）するロジックを追加できます。
                    }

                    ui.add_space(10.0);

                    // その他のアクションボタン（例）
                    if ui.button("詳細分析を実行").clicked() {
                        info!("詳細分析を実行しました！");
                        // ここに詳細分析のロジックを追加
                    }
                });
            });
    }
}
