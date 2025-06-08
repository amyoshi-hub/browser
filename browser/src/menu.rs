use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{TokioRuntimeHandle, AsyncComputeTaskPool};
use futures_lite::future;

// main.rs で定義したリソースやコンポーネントをuseする
use crate::{CurrentUrl, HtmlContent, FetchHtmlTask, ShowHtmlViewer};

pub fn setup_ui_panel(mut current_url: ResMut<CurrentUrl>) {
    // 初期URLを設定
    current_url.0 = "https://example.com".to_string();
}

// URL入力とリクエストをトリガーするシステム
pub fn url_input_system(
    mut contexts: EguiContexts,
    mut current_url: ResMut<CurrentUrl>,
    mut commands: Commands,
    tokio_runtime: Res<TokioRuntimeHandle>,
    mut show_html_viewer: ResMut<ShowHtmlViewer>,
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
