use bevy::prelude::*; // Component, Handle など Bevys の型を使うため

/// Where to find the serialized animation graph.
pub static ANIMATION_GRAPH_PATH: &str = "animation_graphs/Fox.animgraph.ron";

/// The indices of the nodes containing animation clips in the graph.
pub static CLIP_NODE_INDICES: [u32; 3] = [2, 3, 4];

/// The help text in the upper left corner.
pub static HELP_TEXT: &str = "Click and drag an animation clip node to change its weight";

/// The node widgets in the UI.
pub static NODE_TYPES: [NodeType; 5] = [
    NodeType::Clip(ClipNode::new("Idle", 0)),
    NodeType::Clip(ClipNode::new("Walk", 1)),
    NodeType::Blend("Root"),
    NodeType::Blend("Blend\n0.5"),
    NodeType::Clip(ClipNode::new("Run", 2)),
];

/// The positions of the node widgets in the UI.
///
/// These are in the same order as [`NODE_TYPES`] above.
pub static NODE_RECTS: [NodeRect; 5] = [
    NodeRect::new(10.00, 10.00, 97.64, 48.41),
    NodeRect::new(10.00, 78.41, 97.64, 48.41),
    NodeRect::new(286.08, 78.41, 97.64, 48.41),
    NodeRect::new(148.04, 112.61, 97.64, 48.41), // was 44.20
    NodeRect::new(10.00, 146.82, 97.64, 48.41),
];

/// The positions of the horizontal lines in the UI.
pub static HORIZONTAL_LINES: [Line; 6] = [
    Line::new(107.64, 34.21, 158.24),
    Line::new(107.64, 102.61, 20.20),
    Line::new(107.64, 171.02, 20.20),
    Line::new(127.84, 136.82, 20.20),
    Line::new(245.68, 136.82, 20.20),
    Line::new(265.88, 102.61, 20.20),
];

/// The positions of the vertical lines in the UI.
pub static VERTICAL_LINES: [Line; 2] = [
    Line::new(127.83, 102.61, 68.40),
    Line::new(265.88, 34.21, 102.61),
];

/// An on-screen representation of a node.
#[derive(Debug)]
pub struct NodeRect {
    /// The number of pixels that this rectangle is from the left edge of the
    /// window.
    pub left: f32,
    /// The number of pixels that this rectangle is from the bottom edge of the
    /// window.
    pub bottom: f32,
    /// The width of this rectangle in pixels.
    pub width: f32,
    /// The height of this rectangle in pixels.
    pub height: f32,
}

/// Either a straight horizontal or a straight vertical line on screen.
///
/// The line starts at (`left`, `bottom`) and goes either right (if the line is
/// horizontal) or down (if the line is vertical).
pub struct Line {
    /// The number of pixels that the start of this line is from the left edge
    /// of the screen.
    pub left: f32,
    /// The number of pixels that the start of this line is from the bottom edge
    /// of the screen.
    pub bottom: f32,
    /// The length of the line.
    pub length: f32,
}

/// The type of each node in the UI: either a clip node or a blend node.
pub enum NodeType {
    /// A clip node, which specifies an animation.
    Clip(ClipNode),
    /// A blend node with no animation and a string label.
    Blend(&'static str),
}

/// The label for the UI representation of a clip node.
#[derive(Clone, Component)]
pub struct ClipNode {
    /// The string label of the node.
    pub text: &'static str,
    /// Which of the three animations this UI widget represents.
    pub index: usize,
}


impl ClipNode {
    /// Creates a new [`ClipNodeText`] from a label and the animation index.
    pub const fn new(text: &'static str, index: usize) -> Self {
        Self { text, index }
    }
}

impl NodeRect {
    /// Creates a new [`NodeRect`] from the lower-left corner and size.
    ///
    /// Note that node rectangles are anchored in the *lower*-left corner. The
    /// `bottom` parameter specifies vertical distance from the *bottom* of the
    /// window.
    pub const fn new(left: f32, bottom: f32, width: f32, height: f32) -> NodeRect {
        NodeRect {
            left,
            bottom,
            width,
            height,
        }
    }
}

impl Line {
    /// Creates a new [`Line`], either horizontal or vertical.
    ///
    /// Note that the line's start point is anchored in the lower-*left* corner,
    /// and that the `length` extends either to the right or downward.
    pub const fn new(left: f32, bottom: f32, length: f32) -> Self {
        Self {
            left,
            bottom,
            length,
        }
    }
}


