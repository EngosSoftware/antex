//! # Styled text and tree in terminal

mod colors;
mod mode;
mod text;
mod tree;

pub use colors::{Color, RgbColor};
pub use mode::ColorMode;
pub use text::{StyledText, Text, always, auto, never};
pub use tree::{LeafBuilder, LeafLineBuilder, NodeBuilder, NodeLineBuilder, TreeNode, leaf, node};
