//use ffmpeg_next::media;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::collections::HashMap;
use std::path::Path;
use ffmpeg_next as ffmpeg; // Cargo.toml に "ffmpeg_next = "0.x" を追加してください
use ffmpeg_next::format::input;
use ffmpeg_next::frame::Video;
use ffmpeg_next::software::scaling::{Context, Flags};
use ffmpeg_next::util::format::Pixel;
use bevy::render::render_resource::{TextureDimension, TextureFormat, TextureUsages};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::color::palettes::css::PINK;
use tracing::{info, error};
use crate::ShowFfmpegWindow;
use crate::ffmpeg::egui::load::SizedTexture;
use ffmpeg_sys_next::AVMediaType;

// FFmpegの初期化はアプリケーション起動時に一度だけ行います
pub fn initialize_ffmpeg() {
    ffmpeg::init().unwrap();
}

// 非SendデータをBevyコンポーネントに格納できないためのワークアラウンド
// NonSendリソースとして動画プレイヤー関連データを保持します
#[derive(Default)]
pub struct VideoResource {
    pub video_players: HashMap<Entity, VideoPlayerNonSendData>,
}

// FFmpegのデコーダー、入力コンテキスト、スケーラーコンテキストを格納する構造体
// これらはSendトレイトを実装しないため、NonSendリソースに含めます
pub struct VideoPlayerNonSendData {
    pub decoder: ffmpeg::decoder::Video,
    pub input_context: ffmpeg::format::context::Input,
    pub scaler_context: Context,
}

// Bevyエンティティにアタッチされるコンポーネント
// 動画の画像ハンドルとストリームインデックスを保持します
#[derive(Component)]
pub struct VideoPlayer {
    pub image_handle: Handle<Image>,
    pub video_stream_index: usize,
}

impl VideoPlayer {
    fn new<'a, P>(
        path: P,
        mut images: ResMut<Assets<Image>>,
    ) -> Result<(VideoPlayer, VideoPlayerNonSendData), ffmpeg::Error>
    where
        P: AsRef<Path>,
    {
        let input_context = input(&path)?;

        let input_stream = input_context
            .streams()
            .best(AVMediaType::AVMEDIA_TYPE_VIDEO.into()) // Change this line
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input_stream.index();

        let context_decoder =
            ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())?;
        let decoder = context_decoder.decoder().video()?;

        // ピクセルフォーマット変換用のスケーラーを初期化 (例: YUV -> RGBA)
        let scaler_context = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGBA,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR, // 双線形補間
        )?;

        // BevyのImageアセットを作成し、ハンドルを取得
        // 初期はピンク色で塗りつぶされますが、後で動画フレームで更新されます
        let mut image = Image::new_fill(
            bevy::render::render_resource::Extent3d {
                width: decoder.width(),
                height: decoder.height(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &PINK.to_u8_array(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        );
        // 画像をGPUにコピーし、シェーダーでバインドできるように設定
        image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING;

        let image_handle = images.add(image);

        Ok((
            VideoPlayer {
                image_handle,
                video_stream_index,
            },
            VideoPlayerNonSendData {
                decoder,
                input_context,
                scaler_context,
            },
        ))
    }
}

// 動画プレイヤーを初期化し、Bevyエンティティとして追加するシステム
pub fn init_video_player_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut video_resource: NonSendMut<VideoResource>,
) {
    //file pass
    let video_path = "./assets/video/video.mp4"; 

    match VideoPlayer::new(video_path, images) {
        Ok((video_player, video_player_non_send)) => {
            let entity = commands.spawn(video_player).id();
            video_resource.video_players.insert(entity, video_player_non_send);
            info!("Video player initialized for: {}", video_path);
        }
        Err(e) => {
            error!("Failed to initialize video player: {}", e);
        }
    }
}

// 動画フレームをデコードし、BevyのImageアセットを更新するシステム
// このシステムは毎フレーム実行され、動画の進行を管理します
pub fn play_video(
    mut video_player_query: Query<(&mut VideoPlayer, Entity)>,
    mut video_resource: NonSendMut<VideoResource>,
    mut images: ResMut<Assets<Image>>,
) {
    for (video_player, entity) in video_player_query.iter_mut() {
        let video_player_non_send = video_resource.video_players.get_mut(&entity).unwrap();
        
        // 1フレームを処理するまでパケットを読み込み、デコードを試みます
        while let Some((stream, packet)) = video_player_non_send.input_context.packets().next() {
            // パケットが動画ストリームのものであることを確認
            if stream.index() == video_player.video_stream_index {
                // パケットをデコーダーに送信
                video_player_non_send.decoder.send_packet(&packet).unwrap();
                let mut decoded = Video::empty();
                
                // 完全なフレームがデコードされたか確認
                if let Ok(()) = video_player_non_send.decoder.receive_frame(&mut decoded) {
                    let mut rgb_frame = Video::empty();
                    
                    // フレームをスケーラーに通してRGBAに変換
                    video_player_non_send
                        .scaler_context
                        .run(&decoded, &mut rgb_frame)
                        .unwrap();
                    
                    let image = images.get_mut(&video_player.image_handle).unwrap();
                    if let Some(data) = image.data.as_mut() {
                        data.copy_from_slice(rgb_frame.data(0));
                    } else {
                        error!("Image data is None");
                    }
                    
                    // 1フレーム更新したら、一旦システムを終了して次のBevyフレームで続きを処理
                    return;
                }
            }
        }
        // フレームが受信できなかった場合（ファイルの終端など）
        // デコーダーに再生の終了を通知
        match video_player_non_send.decoder.send_eof() {
            Err(ffmpeg::Error::Eof) => {
                // info!("End of video stream reached for entity: {:?}", entity);
                // 必要であれば、ここで動画のループ再生や停止などのロジックを追加
            }
            other => other.unwrap(), // その他のエラーはパニック
        }
    }
}

pub fn ffmpeg_window(
    mut contexts: EguiContexts,
    show_ffmpeg_window: Res<ShowFfmpegWindow>,
    video_player_query: Query<(&VideoPlayer, Entity)>,
    images_assets: Res<Assets<Image>>,
) {
    // Collect `TextureId` and `Vec2` data before the `show` closure.
    let mut egui_images_data: Vec<(egui::TextureId, egui::Vec2)> = Vec::new();
    for (video_player, _entity) in video_player_query.iter() {
        let texture_id = contexts.add_image(video_player.image_handle.clone_weak());

        let image_asset = images_assets.get(&video_player.image_handle);
        let size = if let Some(img) = image_asset {
            egui::Vec2::new(img.width() as f32, img.height() as f32)
        } else {
            egui::Vec2::new(100.0, 100.0) // Fallback size
        };
        egui_images_data.push((texture_id, size));
    }

    let ctx = contexts.ctx_mut(); // Now get ctx_mut() after contexts.add_image() calls are done

    // Show the window only if `ShowOptionWindow` is true.
    if show_ffmpeg_window.0 {
        egui::Window::new("Video Player & Options")
            .default_size(egui::vec2(600.0, 600.0))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Existing UI elements
                    if ui.button("GPT (Option 1)").clicked() {
                        info!("GPT Option 1 clicked!");
                    }
                    if ui.button("OSAI (Option)").clicked() {
                        info!("GPT Option 2 clicked!");
                    }
                    // --- Video display logic starts here ---
                    ui.separator(); // Separator line
                    ui.heading("MP4 Playback"); // Heading

                    for (texture_id, size) in egui_images_data.iter() {
                        // Corrected: Use `egui::widgets::SizedTexture` to create the `egui::Image`.
                        let egui_image = egui::widgets::Image::new(SizedTexture::new(*texture_id, *size));

                        ui.add(egui_image.fit_to_exact_size(ui.available_size()));
                    }
                });
            });
    }
}
