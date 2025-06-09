use bevy::{
    prelude::*,
    asset::io::file::FileAssetReader, // FileAssetReader を直接 use
    tasks::IoTaskPool, // IoTaskPool を直接 use
};
use ron::ser::PrettyConfig; // PrettyConfig を直接 use
//use std::{fs::File, path::{Path, PathBuf}, io::Write};
use std::{fs::File, path::{Path}};

// 必要なコンポーネントやリソースをインポート
use crate::{Args, ExampleAnimationGraph, ExampleAnimationWeights}; // main から Args, ExampleAnimationGraph, ExampleAnimationWeights をインポート
use crate::constants::{ANIMATION_GRAPH_PATH, CLIP_NODE_INDICES}; // constants から定数をインポート
//use bevy_scene_hook::SceneHook;
use bevy::gltf::GltfAssetLabel;

/// Initializes the scene.
pub fn setup_assets(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    args: Res<Args>,
) {
    // Create or load the assets.
    if args.no_load || args.save {
        // `args.save` フラグに基づいて、setup_assets_programmatically に保存処理を任せる
        setup_assets_programmatically(
            &mut commands,
            &mut asset_server,
            &mut animation_graphs,
            args.save, // この boolean 値を `setup_assets_programmatically` に渡す
        );
    } else {
        // `--no-load` や `--save` がない場合は、既存のアセットをロードする
        setup_assets_via_serialized_animation_graph(&mut commands, &mut asset_server);
    }
}

/// Creates the assets programmatically, including the animation graph.
/// Optionally saves them to disk if `save_to_file` is true (corresponding to the
/// `--save` option).
pub fn setup_assets_programmatically(
    commands: &mut Commands,
    asset_server: &mut AssetServer,
    animation_graphs: &mut Assets<AnimationGraph>,
    save_to_file: bool, // この関数がファイル保存の責任を持つ
) {
    // Create the nodes. (この部分は変更なし)
    let mut animation_graph = AnimationGraph::new();
    let blend_node = animation_graph.add_blend(0.5, animation_graph.root);
    animation_graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset("models/animated/Fox.glb")),
        1.0,
        animation_graph.root,
    );
    animation_graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(1).from_asset("models/animated/Fox.glb")),
        1.0,
        blend_node,
    );
    animation_graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(2).from_asset("models/animated/Fox.glb")),
        1.0,
        blend_node,
    );

    // If asked to save, do so. (この部分が重要)
    #[cfg(not(target_arch = "wasm32"))]
    if save_to_file { // `save_to_file` 引数が true の場合のみ実行される
        let animation_graph_to_save = animation_graph.clone();

        IoTaskPool::get()
            .spawn(async move {
                info!("Writing animation graph to {ANIMATION_GRAPH_PATH}");

                // ron::ser::to_string_pretty で文字列にシリアライズ
                let animation_graph_string = ron::ser::to_string_pretty(
                    &animation_graph_to_save,
                    PrettyConfig::default(),
                )
                .expect("Failed to serialize animation graph to RON string");

                let base_path = FileAssetReader::get_base_path();
                let assets_dir = Path::new("assets");
                let animation_graph_relative_path = Path::new(ANIMATION_GRAPH_PATH);

                let full_path = base_path
                    .join(assets_dir)
                    .join(animation_graph_relative_path);

                if let Some(parent_dir) = full_path.parent() {
                    std::fs::create_dir_all(parent_dir)
                        .expect(&format!("Failed to create directory: {:?}", parent_dir));
                }

                // std::io::Write::write_all でファイルに書き込む
                let mut animation_graph_writer = File::create(&full_path)
                    .expect(&format!("Failed to create animation graph asset file at {:?}", full_path));

                std::io::Write::write_all(
                    &mut animation_graph_writer,
                    animation_graph_string.as_bytes(),
                )
                .expect("Failed to write animation graph to file");

                info!("Animation graph saved to {:?}", full_path);
            })
            .detach();
    }

    // Add the graph.
    let handle = animation_graphs.add(animation_graph);

    // Save the assets in a resource.
    commands.insert_resource(ExampleAnimationGraph(handle));
}

pub fn setup_assets_via_serialized_animation_graph(
    commands: &mut Commands,
    asset_server: &mut AssetServer,
) {
    commands.insert_resource(ExampleAnimationGraph(
        asset_server.load(ANIMATION_GRAPH_PATH),
    ));
}

/// Spawns the animated fox.
pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-10.0, 5.0, 13.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            intensity: 10_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-4.0, 8.0, 13.0),
    ));

    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/animated/Fox.glb")),
        ),
        Transform::from_scale(Vec3::splat(0.07)),
    ));

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(7.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
}

/// Attaches the animation graph to the scene, and plays all three animations.
pub fn init_animations(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationPlayer)>,
    animation_graph: Res<ExampleAnimationGraph>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }

    for (entity, mut player) in query.iter_mut() {
        commands.entity(entity).insert((
            AnimationGraphHandle(animation_graph.0.clone()),
            ExampleAnimationWeights::default(),
        ));
        for &node_index in &CLIP_NODE_INDICES {
            player.play(node_index.into()).repeat();
        }

        *done = true;
    }
}

/// Takes the weights that were set in the UI and assigns them to the actual
/// playing animation.
pub fn sync_weights(mut query: Query<(&mut AnimationPlayer, &ExampleAnimationWeights)>) {
    for (mut animation_player, animation_weights) in query.iter_mut() {
        for (&animation_node_index, &animation_weight) in CLIP_NODE_INDICES
            .iter()
            .zip(animation_weights.weights.iter())
        {
            // If the animation happens to be no longer active, restart it.
            if !animation_player.is_playing_animation(animation_node_index.into()) {
                animation_player.play(animation_node_index.into());
            }

            // Set the weight.
            if let Some(active_animation) =
                animation_player.animation_mut(animation_node_index.into())
            {
                active_animation.set_weight(animation_weight);
            }
        }
    }
}


