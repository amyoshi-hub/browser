use bevy::{prelude::*, tasks::{AsyncComputeTaskPool, Task}};
use bevy_egui::EguiPlugin;
use std::sync::{Arc, Mutex};

mod menu;
mod img_server;
mod animation_ui;
mod animation_logic;
mod constants;
mod p2p; // p2pモジュールをインポート
use bevy_tokio_tasks::TokioTasksPlugin; // TokioTasksPlugin をインポート

#[cfg(not(target_arch = "wasm32"))]
use argh::FromArgs; // `argh` をインポート

#[derive(Resource, Default)]
pub struct HtmlContent(pub Arc<Mutex<String>>);
#[derive(Resource, Default)]
pub struct OtherAI {
    pub api_key: String,
}
#[derive(Resource, Default)]
pub struct CurrentUrl(pub String);
#[derive(Component)]
struct FetchHtmlTask(Task<Result<String, String>>); // Result<成功時の文字列, エラー時の文字列>
#[derive(Resource)]
pub struct ShowHtmlViewer(pub bool);
#[derive(Resource)]
pub struct ShowOptionWindow(pub bool);

///Command line arguments for the browser application.
#[derive(FromArgs, Resource)]
pub struct Args { // `pub` をつけることで他のモジュールからアクセス可能に
    /// disables loading of the animation graph asset from disk
    #[argh(switch)]
    pub no_load: bool,
    /// regenerates the asset file; implies `--no-load`
    #[argh(switch)]
    pub save: bool,
}

/// The [`AnimationGraph`] asset, which specifies how the animations are to
/// be blended together.
#[derive(Clone, Resource)]
pub struct ExampleAnimationGraph(pub Handle<AnimationGraph>); // `pub` をつける

/// The current weights of the three playing animations.
#[derive(Component, Default)] // Default トレイトを追加
pub struct ExampleAnimationWeights { // `pub` をつける
    /// The weights of the three playing animations.
    pub weights: [f32; 3],
}

#[derive(Resource, Clone)]
pub struct TokioRuntimeHandle(pub tokio::runtime::Handle);


fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let args: Args = argh::from_env();
    #[cfg(target_arch = "wasm32")]
    let args = Args::from_args(&[], &[]).unwrap();

    tracing_subscriber::fmt::init();

    // Tokio runtime を作成し、ハンドルを取得します。
    // このランタイムは `TokioTasksPlugin` が管理します。
    let tokio_runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let tokio_handle = tokio_runtime.handle().clone();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(TokioTasksPlugin::default()) // BevyがTokioランタイムを管理するプラグイン
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: false })
        .insert_resource(TokioRuntimeHandle(tokio_handle)) // TokioRuntimeHandle をリソースとして挿入
        .add_event::<p2p::P2pUdpPacketReceived>()
        .add_event::<img_server::ImageChunkReceived>()
        .add_event::<img_server::ImageReceptionComplete>()
        .add_event::<img_server::ImageReceptionError>()

        .insert_resource(HtmlContent::default())
        .insert_resource(CurrentUrl::default())
        .insert_resource(OtherAI::default())
        .insert_resource(ShowHtmlViewer(true))
        .insert_resource(ShowOptionWindow(false))
        .insert_resource(args)
        .add_systems(Startup, (
            img_server::setup_udp_receiver,
            menu::setup_ui_panel,
            animation_logic::setup_assets,
            animation_logic::setup_scene,
            animation_ui::setup_ui,
            p2p::setup_p2p_udp_listener // P2Pリスナーのセットアップ
        ))
        .add_systems(Update, (
            menu::main_input_system,
            menu::poll_fetch_html_task,
            menu::html_viewer_system,
            menu::option_window,
            menu::message_window,
            menu::warning_window,
            img_server::poll_udp_packets,
            img_server::handle_image_chunks.after(img_server::poll_udp_packets),
            img_server::on_image_reception_complete.after(img_server::handle_image_chunks),
            img_server::on_image_reception_error.after(img_server::handle_image_chunks),
            animation_ui::handle_weight_drag,
            animation_ui::update_ui,
            animation_logic::sync_weights,
            p2p::poll_p2p_udp_packets // P2Pパケットのポーリング
        ).chain())
        .add_systems(Update, animation_logic::init_animations);
    
    app.run();
}
