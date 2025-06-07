// src/graph_anime.rs
use bevy::{
    prelude::*,
    animation::{
        prelude::*,
        ui::*,
    },
    tasks::IoTaskPool,
    asset::{AssetLoader, LoadedAsset},
};
use bevy_egui::egui;
use argh::FromArgs;
use std::{collections::HashMap, fs::File, io::Write, path::Path};
use serde::{Deserialize, Serialize};
use ron::ser::PrettyConfig; // ron::ser::to_string_pretty で使う


// from main.rs
// mod imgServer;
// mod mode_s;
// mod menu;
// mod graph_anime;
// mod http_req;
// use std::env::Args; // main.rsで削除済み
// use crate::mode_s::mode_select;
// use std::collections::VecDeque;
// use std::sync::mpsc::{self, Sender, Receiver};
// use std::sync::{Arc, Mutex};
// use crate::imgServer::{start_udp_receiver};
// use bevy_egui::{egui, EguiContexts, EguiPlugin};
// use bevy::asset::LoadState;
// use bevy::ecs::event::Event;
// use tokio::task;
// use tokio::runtime::{Handle as TokioHandle, Runtime};
// use crate::menu::{InputText, HtmlContent, LoadedEguiImage};

// use crate::graph_anime::{
//     Args,
//     setup_assets,
//     setup_scene,
//     setup_ui,
//     init_animations,
//     handle_weight_drag,
//     update_ui,
//     sync_weights,
// };
// use bevy::{
//     color::palettes::{
//         basic::WHITE,
//         css::{ANTIQUE_WHITE, DARK_GREEN},
//     },
//     prelude::*,
//     ui::RelativeCursorPosition,
// };


// const FOX_GLTF_PATH: &str = "models/animated/Fox.glb";
const FOX_GLTF_PATH: &str = "models/animated/Fox.glb#Scene0"; // シーンを指定
const ANIMATION_GRAPH_PATH: &str = "animation_graphs/Fox.animgraph.ron";

// The path to the clip that is represented by the UI `ClipNode` at `index`
const CLIP_PATHS: [&str; 3] = [
    "models/animated/Fox.glb#Animation0",
    "models/animated/Fox.glb#Animation1",
    "models/animated/Fox.glb#Animation2",
];

// The starting weights of the animation blend.
const STARTING_WEIGHTS: [f32; 3] = [0.0, 0.0, 1.0];

/// Bevy Animation Graph Example CLI arguments.
#[derive(FromArgs, Resource)]
pub struct Args {
    /// whether to save the animation graph in `assets/animation_graphs/Fox.animgraph.ron`
    #[argh(switch)]
    pub save: bool,
}

// from main.rs: pub struct ExampleAnimationGraph(pub Handle<AnimationGraph>);
#[derive(Clone, Resource)]
pub struct ExampleAnimationGraph(pub Handle<AnimationGraph>);


#[derive(Clone, Component)]
pub struct ClipNode {
    pub text: &'static str,
    pub index: usize,
}

// from main.rs: pub struct ExampleAnimationWeights(pub Vec<f32>);
#[derive(Resource)]
pub struct ExampleAnimationWeights(pub Vec<f32>);

// UI elements that have a text label and a rect (position and size).
#[derive(Clone, Component)]
pub struct NodeRect {
    pub text: &'static str,
    pub rect: egui::Rect,
}

// UI elements that are lines between nodes.
#[derive(Clone, Component)]
pub struct Line {
    pub start: egui::Pos2,
    pub end: egui::Pos2,
}

// The type of node (Clip, Blend, Output).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeType {
    Clip,
    Blend,
    Output,
}

// An animation graph node.
#[derive(Clone, Component)]
pub struct GraphNode {
    pub node_type: NodeType,
    pub node_id: AnimationNodeId,
    pub input_nodes: Vec<AnimationNodeId>,
}

#[derive(Default, Resource)]
pub struct AnimationPlayers(pub Vec<AnimationPlayer>);

pub fn setup_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    args: Res<Args>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    let _ = asset_server.load::<Scene>(FOX_GLTF_PATH);
    let _ = asset_server.load::<AnimationGraph>(ANIMATION_GRAPH_PATH);

    // Create a new animation graph if one doesn't exist.
    let animation_graph_handle = if asset_server.asset_io().is_some() {
        let animation_graph_path = Path::new("assets").join(Path::new(ANIMATION_GRAPH_PATH));
        if std::fs::metadata(&animation_graph_path).is_err() || args.save {
            setup_assets_programmatically(
                &mut commands,
                &mut meshes,
                &mut materials,
                &asset_server,
                &mut animation_graphs,
                args.save,
            );
        }
        asset_server.load(ANIMATION_GRAPH_PATH)
    } else {
        // If asset_io is not available (e.g., in wasm32), always create programmatically.
        setup_assets_programmatically(
            &mut commands,
            &mut meshes,
            &mut materials,
            &asset_server,
            &mut animation_graphs,
            true, // Always save for WASM if this path is taken
        );
        asset_server.load(ANIMATION_GRAPH_PATH) // This might still fail if RON cannot be loaded in WASM directly.
    };

    commands.insert_resource(ExampleAnimationGraph(animation_graph_handle));
    commands.insert_resource(ExampleAnimationWeights(STARTING_WEIGHTS.to_vec()));
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(StandardMaterial::from(Color::srgb(0.3, 0.5, 0.3))),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 1000000.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-4.0, 5.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// Function to set up UI, nodes, lines (placeholder)
pub fn setup_ui(mut commands: Commands) {
    // Placeholder for UI setup, you'll need to fill this based on your actual UI structure
    // For example, adding nodes for the animation graph
    commands.spawn(ClipNode { text: "Clip 0", index: 0 });
    commands.spawn(ClipNode { text: "Clip 1", index: 1 });
    commands.spawn(ClipNode { text: "Clip 2", index: 2 });
}


pub fn init_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut players: ResMut<AnimationPlayers>,
    mut done: Local<bool>,
) {
    if *done { return; }

    let fox_scene_handle: Handle<Scene> = asset_server.load(FOX_GLTF_PATH);
    let mut animation_clips: Vec<Handle<AnimationClip>> = Vec::new();
    for clip_path in CLIP_PATHS.iter() {
        animation_clips.push(asset_server.load(*clip_path));
    }

    commands.spawn(SceneBundle {
        scene: fox_scene_handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.01)),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(AnimationPlayerBundle {
            animation_player: AnimationPlayer::default(),
            ..default()
        });
    });

    *done = true; // Ensures this runs only once
}


pub fn handle_weight_drag(
    mut contexts: EguiContexts,
    mut query_weights: ResMut<ExampleAnimationWeights>,
    mut query_clip_nodes: Query<&mut NodeRect, With<ClipNode>>, // Ensure ClipNode also has NodeRect
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Animation Weights")
        .default_pos(egui::pos2(10.0, 10.0))
        .show(ctx, |ui| {
            for (i, weight) in query_weights.0.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("Clip {}:", i));
                    ui.add(egui::Slider::new(weight, 0.0..=1.0).text("Weight"));
                });
            }
            if ui.button("Normalize").clicked() {
                let sum: f32 = query_weights.0.iter().sum();
                if sum > 0.0 {
                    for w in query_weights.0.iter_mut() {
                        *w /= sum;
                    }
                }
            }
        });
}

pub fn update_ui() {
    // This function might be where you update UI nodes based on animation state
    // For now, it can be empty or have placeholder logic.
}

pub fn sync_weights(
    mut query_players: Query<&mut AnimationPlayer>,
    animation_graph: Res<ExampleAnimationGraph>,
    weights: Res<ExampleAnimationWeights>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    let Some(graph) = animation_graphs.get_mut(&animation_graph.0) else {
        return;
    };

    let mut blend_node_id = None;
    for node_id in graph.nodes.node_ids() {
        if let Some(AnimationNode::Blend(blend_node)) = graph.nodes.get_mut(node_id) {
            // Assuming the first blend node found is the one we want to control
            blend_node_id = Some(node_id);
            break;
        }
    }

    if let Some(blend_node_id) = blend_node_id {
        if let Some(AnimationNode::Blend(blend_node)) = graph.nodes.get_mut(blend_node_id) {
            for (i, weight) in weights.0.iter().enumerate() {
                // Ensure there are enough blend inputs
                if i < blend_node.inputs.len() {
                    blend_node.inputs[i].weight = *weight;
                }
            }
        }
    }

    for mut player in query_players.iter_mut() {
        player.set_graph(animation_graph.0.clone());
    }
}


pub fn setup_assets_programmatically(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    animation_graphs: &mut ResMut<Assets<AnimationGraph>>,
    _save: bool,
) {
    // Create an AnimationGraph
    let mut animation_graph = AnimationGraph::new();

    let clip_nodes: Vec<AnimationNodeId> = CLIP_PATHS
        .iter()
        .map(|path| animation_graph.add_clip(asset_server.load(*path)))
        .collect();

    // Create a blend node that takes the clip nodes as input
    let blend_node = animation_graph.add_blend(AnimationNode::Blend(BlendNode {
        inputs: (0..clip_nodes.len())
            .map(|i| BlendInput {
                node: clip_nodes[i],
                weight: STARTING_WEIGHTS[i],
            })
            .collect(),
    }));

    // Set the blend node as the output
    animation_graph.set_output(blend_node);

    #[cfg(not(target_arch = "wasm32"))]
    if _save {
        let animation_graph_clone = animation_graph.clone(); // Clone for async task

        let task_pool = IoTaskPool::get(); // Use the task_pool variable
        task_pool
            .spawn(async move {
                let current_dir = std::env::current_dir().expect("Failed to get current directory");
                let assets_dir = current_dir.join("assets");
                let animation_graph_dir = assets_dir.join("animation_graphs");
                std::fs::create_dir_all(&animation_graph_dir).expect("Failed to create animation_graphs directory");

                let animation_graph_path = animation_graph_dir.join(Path::new(ANIMATION_GRAPH_PATH).file_name().unwrap());

                let animation_graph_writer = File::create(&animation_graph_path)
                    .expect(&format!("Failed to open the animation graph asset at {:?}", animation_graph_path));

                let serialized_data = ron::ser::to_string_pretty(&animation_graph_clone, PrettyConfig::default())
                    .expect("Failed to serialize the animation graph to RON string");

                let mut writer = animation_graph_writer;
                writer.write_all(serialized_data.as_bytes())
                    .expect("Failed to write serialized data to file");
            })
            .detach();
    }

    let handle = animation_graphs.add(animation_graph);
    commands.insert_resource(ExampleAnimationGraph(handle));
}

// 警告で出ていた未使用のmain関数。削除します。
// pub fn main() {
//    // ...
// }
