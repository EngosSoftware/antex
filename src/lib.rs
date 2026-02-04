//! # Styled text and tree in terminal

mod colors;
mod mode;
mod text;
mod tree;

pub use colors::{Color, RgbColor};
pub use mode::ColorMode;
pub use text::{always, auto, never, StyledText, Text};
pub use tree::{leaf, node, LeafBuilder, LeafLineBuilder, NodeBuilder, NodeLineBuilder, TreeNode};
