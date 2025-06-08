use bevy::{prelude::*, tasks::{AsyncComputeTaskPool, Task}};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::sync::{Arc, Mutex};
use futures_lite::future; // Task の完了をポーリングするために必要
//use reqwest;
mod menu;
mod img_server;

#[derive(Resource, Default)]
pub struct HtmlContent(pub Arc<Mutex<String>>);
#[derive(Resource, Default)]
pub struct CurrentUrl(pub String);
#[derive(Component)]
struct FetchHtmlTask(Task<Result<String, String>>); // Result<成功時の文字列, エラー時の文字列>
#[derive(Resource)]
pub struct ShowHtmlViewer(pub bool);

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
        .add_event::<img_server::ImageChunkReceived>()        // ここ
        .add_event::<img_server::ImageReceptionComplete>()    // ここ
        .add_event::<img_server::ImageReceptionError>()

        .insert_resource(HtmlContent::default())
        .insert_resource(CurrentUrl::default())
        .insert_resource(ShowHtmlViewer(true))
        //.add_systems(Startup, setup_tokio_runtime_handle)

        // システムの登録
        .add_systems(Startup, (
            img_server::setup_udp_receiver,
            menu::setup_ui_panel,
        ))
        .add_systems(Update, (
            menu::url_input_system,
            menu::poll_fetch_html_task,
            menu::html_viewer_system,
            img_server::poll_udp_packets,
            img_server::handle_image_chunks.after(img_server::poll_udp_packets),
            img_server::on_image_reception_complete.after(img_server::handle_image_chunks),
            img_server::on_image_reception_error.after(img_server::handle_image_chunks),
        ))
        .run();
    drop(tokio_runtime);
}
