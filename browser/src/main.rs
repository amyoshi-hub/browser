use bevy::{prelude::*, tasks::{AsyncComputeTaskPool, Task}};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
//use tokio::runtime::Handle as TokioRuntimeHandleActual;
use std::sync::{Arc, Mutex};
use futures_lite::future; // Task の完了をポーリングするために必要
use reqwest;

#[derive(Resource, Default)]
pub struct HtmlContent(pub Arc<Mutex<String>>);

#[derive(Resource, Default)]
pub struct CurrentUrl(pub String);

#[derive(Component)]
struct FetchHtmlTask(Task<Result<String, String>>); // Result<成功時の文字列, エラー時の文字列>

#[derive(Resource, Clone)]
//pub struct TokioRuntimeHandle(pub TokioRuntimeHandleActual);
pub struct TokioRuntimeHandle(pub tokio::runtime::Handle);


/*
fn setup_tokio_runtime_handle(mut commands: Commands) {
    let tokio_handle = TokioRuntimeHandleActual::current();
    commands.insert_resource(TokioRuntimeHandle(tokio_handle));
}
*/


fn main() {
 //   let tokio_handle = tokio::runtime::Handle::current();

    let tokio_runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let tokio_handle = tokio_runtime.handle().clone(); // Handle を取得
    App::new()
        .add_plugins(DefaultPlugins)
         .add_plugins(EguiPlugin { enable_multipass_for_primary_context: false })
         .insert_resource(TokioRuntimeHandle(tokio_handle))

        .insert_resource(HtmlContent::default())
        .insert_resource(CurrentUrl::default())
        //.add_systems(Startup, setup_tokio_runtime_handle)

        // システムの登録
        .add_systems(Startup, setup_ui_panel) // 初期UIのセットアップ
        .add_systems(Update, (
            url_input_system,       // URL入力とリクエストのトリガー
            poll_fetch_html_task,   // HTMLフェッチタスクの監視
            html_viewer_system,     // HTMLコンテンツの表示
        ))
        .run();
    drop(tokio_runtime);
}


// 初期UIのセットアップ（現在のURLを保持するテキストボックス）
fn setup_ui_panel(mut current_url: ResMut<CurrentUrl>) {
    // 初期URLを設定
    current_url.0 = "https://example.com".to_string();
}

// URL入力とリクエストをトリガーするシステム
fn url_input_system(
    mut contexts: EguiContexts,
    mut current_url: ResMut<CurrentUrl>,
    mut commands: Commands,
    tokio_runtime: Res<TokioRuntimeHandle>,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("url_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("URL:");
            let response = ui.text_edit_singleline(&mut current_url.0);
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
fn poll_fetch_html_task(
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
fn html_viewer_system(
    mut contexts: EguiContexts,
    html_content: Res<HtmlContent>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let content = html_content.0.lock().unwrap();
            ui.label(egui::RichText::new(content.as_str()).monospace()); // monospaceで表示
        });
    });
}
