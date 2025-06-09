use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{TokioRuntimeHandle, AsyncComputeTaskPool};
use futures_lite::future;

// main.rs で定義したリソースやコンポーネントをuseする
use crate::{CurrentUrl, HtmlContent, FetchHtmlTask, ShowHtmlViewer, ShowOptionWindow, OtherAI};

pub fn setup_ui_panel(mut current_url: ResMut<CurrentUrl>) {
    // 初期URLを設定
    current_url.0 = "https://example.com".to_string();
}

// URL入力とリクエストをトリガーするシステム
pub fn main_input_system(
    mut contexts: EguiContexts,
    mut current_url: ResMut<CurrentUrl>,
    mut commands: Commands,
    tokio_runtime: Res<TokioRuntimeHandle>,
    mut show_html_viewer: ResMut<ShowHtmlViewer>,
    mut show_option_window: ResMut<ShowOptionWindow>,
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
            if ui.button("Opption").clicked() {
                show_option_window.0 = !show_option_window.0;
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

/*
pub fn p2p_mode(
    mut contexts: EguiContexts,
    mut current_url: ResMut<CurrentUrl>,
    mut commands: Commands,
    tokio_runtime: Res<TokioRuntimeHandle>,
    mut show_html_viewer: ResMut<ShowHtmlViewer>,
    udp_socket_res: Res<MyUdpSocket>, // MyUdpSocket リソースを追加
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Toggle HTML Viewer").clicked() {
                show_html_viewer.0 = !show_html_viewer.0;
            }

            let response = ui.text_edit_singleline(&mut current_url.0);

            if response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Return)) {
                info!("URL entered: {}", current_url.0);
                let url_to_fetch = current_url.0.clone();
                let tokio_handle_clone = tokio_runtime.0.handle().clone();
                let thread_pool = AsyncComputeTaskPool::get();
                let udp_socket_clone = udp_socket_res.socket.clone(); // Arc をクローン

                // 非同期タスクでURLフェッチとUDP送信を実行
                thread_pool.spawn(async move {
                    let fetch_result = tokio_handle_clone.spawn(async move {
                        info!("Fetching URL: {}", url_to_fetch);
                        // ここで実際のURLフェッチロジック
                        Ok::<String, String>(format!("Fetched data from {}", url_to_fetch))
                    }).await;

                    match fetch_result {
                        Ok(Ok(data)) => {
                            info!("Successfully fetched: {}", data);
                            // --- ここから pnet を使ったUDP送信のロジック ---
                            let destination_addr_str = "127.0.0.1:8080"; // 送信先のIPアドレスとポート
                            let source_port = udp_socket_clone.local_addr().unwrap().port(); // 送信元ポート

                            match SocketAddr::from_str(destination_addr_str) {
                                Ok(destination_addr) => {
                                    // 送信したいメッセージの内容を動的に変更
                                    let message = format!("URL_FETCHED: {} - Data: {}", url_to_fetch, data.chars().take(50).collect::<String>()); // データの一部を送信
                                    let payload = message.as_bytes();

                                    // UDPパケットのバッファを準備
                                    let mut vec: Vec<u8> = vec![0; UdpPacket::minimum_packet_size() + payload.len()];

                                    if let Some(mut udp_packet) = MutableUdpPacket::new(&mut vec) {
                                        udp_packet.set_source_port(source_port);
                                        udp_packet.set_destination_port(destination_addr.port());
                                        udp_packet.set_length((UdpPacket::minimum_packet_size() + payload.len()) as u16);
                                        udp_packet.set_payload(payload); // ペイロード（データ）を設定

                                        // チェックサムの計算 (UDPの場合、オプションだが設定可能)
                                        // pnet::packet::udp::ipv4_checksum を使う場合、IPヘッダ情報も必要になる
                                        // 簡易的な送信なら、設定しなくてもよい
                                        // udp_packet.set_checksum(pnet::packet::util::checksum(&udp_packet.to_immutable().packet(), 0));

                                        // pnet の transport モジュールを使わず、標準の UdpSocket で送信
                                        // send_to は &self を取るので、Arc<UdpSocket> の参照を渡す
                                        match udp_socket_clone.send_to(udp_packet.packet(), destination_addr) {
                                            Ok(bytes_sent) => {
                                                info!("PNET UDP: Sent {} bytes to {}", bytes_sent, destination_addr);
                                            }
                                            Err(e) => {
                                                error!("PNET UDP: Failed to send packet to {}: {}", destination_addr, e);
                                            }
                                        }
                                    } else {
                                        error!("PNET UDP: Failed to create UDP packet buffer.");
                                    }
                                }
                                Err(e) => {
                                    error!("PNET UDP: Invalid destination address {}: {}", destination_addr_str, e);
                                }
                            }
                        }
                        Ok(Err(e)) => {
                            error!("Error fetching URL: {:?}", e);
                        }
                        Err(e) => {
                            error!("Tokio task join error: {}", e);
                        }
                    }
                }).detach();
            }
        });
    });
}
*/
