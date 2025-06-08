use bevy::{
    color::palettes::css::{ANTIQUE_WHITE, DARK_GREEN},
    prelude::*,
    ui::RelativeCursorPosition,
};

use crate::ExampleAnimationWeights;
use crate::constants::{HELP_TEXT, NODE_TYPES, NODE_RECTS, HORIZONTAL_LINES, VERTICAL_LINES}; // constants から定数をインポート
use crate::constants::{NodeRect, Line, NodeType, ClipNode}; // constants から構造体をインポート

/// Places the help text at the top left of the window.
pub fn setup_help_text(commands: &mut Commands) {
    commands.spawn((
        Text::new(HELP_TEXT),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

/// Initializes the node UI widgets.
pub fn setup_node_rects(commands: &mut Commands) {
    for (node_rect, node_type) in NODE_RECTS.iter().zip(NODE_TYPES.iter()) {
        let node_string = match *node_type {
            NodeType::Clip(ref clip) => clip.text,
            NodeType::Blend(text) => text,
        };

        let text = commands
            .spawn((
                Text::new(node_string),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(ANTIQUE_WHITE.into()),
                TextLayout::new_with_justify(JustifyText::Center),
            ))
            .id();

        let container = {
            let mut container = commands.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(node_rect.bottom),
                    left: Val::Px(node_rect.left),
                    height: Val::Px(node_rect.height),
                    width: Val::Px(node_rect.width),
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BorderColor(Color::WHITE), // Color::WHITE を直接使用
                Outline::new(Val::Px(1.), Val::ZERO, Color::WHITE), // Color::WHITE を直接使用
            ));

            if let NodeType::Clip(clip) = node_type {
                container.insert((
                    Interaction::None,
                    RelativeCursorPosition::default(),
                    (*clip).clone(),
                ));
            }

            container.id()
        };

        // Create the background color.
        if let NodeType::Clip(_) = node_type {
            let background = commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.),
                        left: Val::Px(0.),
                        height: Val::Px(node_rect.height),
                        width: Val::Px(node_rect.width),
                        ..default()
                    },
                    BackgroundColor(DARK_GREEN.into()),
                ))
                .id();

            commands.entity(container).add_child(background);
        }

        commands.entity(container).add_child(text);
    }
}

/// Creates boxes for the horizontal and vertical lines.
///
/// This is a bit hacky: it uses 1-pixel-wide and 1-pixel-high boxes to draw
/// vertical and horizontal lines, respectively.
pub fn setup_node_lines(commands: &mut Commands) {
    for line in &HORIZONTAL_LINES {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(line.bottom),
                left: Val::Px(line.left),
                height: Val::Px(0.0),
                width: Val::Px(line.length),
                border: UiRect::bottom(Val::Px(1.0)),
                ..default()
            },
            BorderColor(Color::WHITE), // Color::WHITE を直接使用
        ));
    }

    for line in &VERTICAL_LINES {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(line.bottom),
                left: Val::Px(line.left),
                height: Val::Px(line.length),
                width: Val::Px(0.0),
                border: UiRect::left(Val::Px(1.0)),
                ..default()
            },
            BorderColor(Color::WHITE), // Color::WHITE を直接使用
        ));
    }
}

pub fn setup_ui(mut commands: Commands) {
    setup_help_text(&mut commands);
    setup_node_rects(&mut commands);
    setup_node_lines(&mut commands);
}

/// Read cursor position relative to clip nodes, allowing the user to change weights
/// when dragging the node UI widgets.
pub fn handle_weight_drag(
    mut interaction_query: Query<(&Interaction, &RelativeCursorPosition, &ClipNode)>,
    mut animation_weights_query: Query<&mut ExampleAnimationWeights>,
) {
    for (interaction, relative_cursor, clip_node) in &mut interaction_query {
        if !matches!(*interaction, Interaction::Pressed) {
            continue;
        }

        let Some(pos) = relative_cursor.normalized else {
            continue;
        };

        for mut animation_weights in animation_weights_query.iter_mut() {
            animation_weights.weights[clip_node.index] = pos.x.clamp(0., 1.);
        }
    }
}

// Updates the UI based on the weights that the user has chosen.
pub fn update_ui(
    mut text_query: Query<&mut Text>,
    mut background_query: Query<&mut Node, Without<Text>>,
    container_query: Query<(&Children, &ClipNode)>,
    animation_weights_query: Query<&ExampleAnimationWeights, Changed<ExampleAnimationWeights>>,
) {
    for animation_weights in animation_weights_query.iter() {
        for (children, clip_node) in &container_query {
            // Draw the green background color to visually indicate the weight.
            let mut bg_iter = background_query.iter_many_mut(children);
            if let Some(mut node) = bg_iter.fetch_next() {
                // All nodes are the same width, so `NODE_RECTS[0]` is as good as any other.
                node.width =
                    Val::Px(NODE_RECTS[0].width * animation_weights.weights[clip_node.index]);
            }

            // Update the node labels with the current weights.
            let mut text_iter = text_query.iter_many_mut(children);
            if let Some(mut text) = text_iter.fetch_next() {
                // ここを修正: Text コンポーネントを直接更新
                text.0 = format!(
                    "{}\n{:.2}",
                    clip_node.text, animation_weights.weights[clip_node.index]
                );
            }
        }
    }
}
