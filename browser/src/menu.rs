// src/menu.rs
use bevy_egui::{
    egui::{self, Color32, TextureId, Vec2}, // TextureHandle, ColorImage は削除
    EguiContexts,
};

use bevy::{
    prelude::*,
    asset::AssetServer, // HandleId は不要なので削除
    render::render_resource::{Extent3d, TextureDimension, TextureFormat}, // GpuImage は不要なので削除
    ecs::event::EventWriter,
};

use std::collections::HashMap;
use tokio::runtime::Handle as TokioHandle;
use tokio::task;

use crate::{http_req::fetch_html, TokioRuntimeHandle, RegisterImageEvent, UdpReceiver}; // fetch_html を使う

#[derive(Resource, Default)]
pub struct InputText(pub String);

#[derive(Resource, Default)]
pub struct HtmlContent(pub String);

#[derive(Default, Resource)]
pub struct LoadedEguiImage {
    pub textures: HashMap<String, TextureId>,
    pub handles: HashMap<String, Handle<Image>>,
    pub counter: usize,
}

// 画像をBevyからEguiに登録するイベント
#[derive(Event, Debug)]
pub struct ImageRegistrationEvent {
    pub path: String,
    pub handle: Handle<Image>,
}


// EguiImageを登録するシステム
pub fn register_egui_image(
    mut egui_contexts: EguiContexts,
    mut loaded_egui_image: ResMut<LoadedEguiImage>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut events: EventReader<ImageRegistrationEvent>,
) {
    for event in events.read() {
        let path = &event.path;
        let handle = &event.handle;

        if let Some(image) = images.get(handle) {
            let egui_texture_handle = egui_contexts.add_image(image.clone());
            loaded_egui_image.textures.insert(path.clone(), egui_texture_handle);
            loaded_egui_image.handles.insert(path.clone(), handle.clone());
            println!("Registered Egui image: {}", path);
        } else {
            println!("Image not yet loaded for path: {}", path);
        }
    }
}


pub fn setup(mut commands: Commands) {
    // You might want to initialize some UI state here
}

pub fn search_box(
    mut contexts: EguiContexts,
    mut input_text: ResMut<InputText>,
    runtime_handle: Res<TokioRuntimeHandle>,
    mut html_content: ResMut<HtmlContent>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Search Box")
        .default_pos(egui::pos2(10.0, 10.0))
        .show(ctx, |ui| {
            ui.text_edit_singleline(&mut input_text.0);
            if ui.button("Search").clicked() {
                let url = input_text.0.clone();
                let html_content_clone = html_content.clone();
                runtime_handle.0.spawn(async move {
                    match fetch_html(&url).await {
                        Ok(html) => {
                            *html_content_clone.0.lock().unwrap() = html; // Assuming HtmlContent is Arc<Mutex<String>>
                            println!("Fetched HTML for {}", url);
                        }
                        Err(e) => {
                            eprintln!("Failed to fetch HTML for {}: {}", url, e);
                        }
                    }
                });
            }
        });
}

pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut loaded_egui_image: ResMut<LoadedEguiImage>,
    asset_server: Res<AssetServer>,
    mut image_events: EventWriter<ImageRegistrationEvent>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Bevy Egui Window")
        .show(ctx, |ui| {
            ui.heading("Hello from Bevy Egui!");

            if ui.button("Load Image (Bevy Assets)").clicked() {
                let image_path = "images/some_image.png"; // 仮の画像パス
                let handle: Handle<Image> = asset_server.load(image_path);
                image_events.send(ImageRegistrationEvent {
                    path: image_path.to_string(),
                    handle: handle,
                });
            }

            for (path, texture_id) in loaded_egui_image.textures.iter() {
                ui.label(format!("Image: {}", path));
                ui.image(egui::ImageSource::Texture(egui::TextureId::from_hash(texture_id.hash(), texture_id.context_id()), egui::Vec2::new(100.0, 100.0)));
            }
        });
}

pub fn text_viewer(
    mut contexts: EguiContexts,
    html_content: Res<HtmlContent>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("HTML Viewer")
        .default_size(egui::Vec2::new(400.0, 600.0))
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(html_content.0.clone());
            });
        });
}

pub fn handle_html_fetch_task(
    mut html_content: ResMut<HtmlContent>, // This resource needs to be Arc<Mutex<String>>
) {
    // This system is designed to run in Bevy's world,
    // but the actual HTML fetching (tokio task) is handled by search_box.
    // So, this system might just be a placeholder or check for updates to html_content.
    // If HtmlContent is Arc<Mutex<String>>, this system won't directly 'process' the future.
    // It will just be able to access the updated String in the mutex.
    // Ensure HtmlContent is indeed Arc<Mutex<String>> or compatible.
}

