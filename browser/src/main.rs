// src/main.rs
mod img_server; // snake_case に修正
mod mode_s;
mod menu;
mod graph_anime;
mod http_req;

use std::sync::mpsc::{Sender, Receiver}; // self は削除
use std::sync::{Arc, Mutex};
use bevy_egui::{egui, EguiContexts, EguiPlugin}; // EguiPlugin は直接使うので残す
use bevy::asset::LoadState; // これが使われていたか不明だが、一旦残す
use bevy::ecs::event::Event;
use tokio::runtime::{Handle as TokioHandle, Runtime};

use crate::mode_s::mode_select;
use crate::img_server::{start_udp_receiver}; // snake_case に修正
use crate::graph_anime::{
    Args,
    setup_assets,
    setup_scene,
    setup_ui,
    init_animations,
    handle_weight_drag,
    update_ui,
    sync_weights,
};
use crate::menu::{InputText, HtmlContent, LoadedEguiImage};
use argh::FromArgs; // FromArgs は必要なので残す

use bevy::{
    color::palettes::{
        basic::WHITE,
        // css::{ANTIQUE_WHITE, DARK_GREEN}, // 使われていなかったので削除
    },
    prelude::*,
    // ui::RelativeCursorPosition, // 使われていなかったので削除
};


#[derive(Resource)]
pub struct UdpSender {
    pub sender: Arc<Mutex<Sender<Vec<u8>>>>,
}

#[derive(Resource)]
pub struct UdpReceiver {
    pub receiver: Arc<Mutex<Receiver<Vec<u8>>>>,
}

#[derive(Resource, Clone)]
pub struct TokioRuntimeHandle(pub TokioHandle);

#[derive(Event, Debug)]
struct RegisterImageEvent;

fn main() {
    let mode;
    mode = mode_select();

    match mode {
        1 => run_gui_mode(),
        2 => run_cui_mode(),
        _ => println!("unnoekn"),
    }
}

fn run_gui_mode(){
    #[cfg(not(target_arch = "wasm32"))]
    let args: Args = argh::from_env();
    #[cfg(target_arch = "wasm32")]
    let args = Args::from_args(&[], &[]).unwrap();

    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    let tokio_handle = runtime.handle().clone();
    let (tx, rx) = std::sync::mpsc::channel();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Animation Graph Example".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(InputText::default())
        .insert_resource(HtmlContent::default())
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: false,})

        .add_systems(Startup, (setup_assets, setup_scene, setup_ui))
        .insert_resource(UdpSender { sender: Arc::new(Mutex::new(tx)), })
        .insert_resource(UdpReceiver { receiver: Arc::new(Mutex::new(rx)), })
        .insert_resource(TokioRuntimeHandle(tokio_handle))
        .insert_resource(LoadedEguiImage::default())
        .add_systems(Update, init_animations)
        .add_event::<RegisterImageEvent>()

        .insert_resource(args)
        .insert_resource(AmbientLight {
            color: Color::Srgba(WHITE),
            brightness: 100.0,
            ..default()
        })

        .add_systems(Startup, (
            menu::setup,
            img_server::start_udp_receiver, // snake_case に修正
            ))
            .add_systems(Update, (
            img_server::process_udp_packets, // snake_case に修正
            menu::ui_image_display_system,
            menu::search_box,
            menu::ui_example_system,
            menu::text_viewer,
            menu::handle_html_fetch_task,
            (handle_weight_drag,
            update_ui,
            sync_weights).chain(),
        ))
        .add_systems(Update, menu::register_egui_image.run_if(on_event::<RegisterImageEvent>));
        app.run();
}

fn run_cui_mode(){
    println!("starting cui mode");
    // ここにCUIモードのロジックが書かれていたはず
}
