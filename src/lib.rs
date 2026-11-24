#![doc = include_str!("../docs/lib.rs.md")]

mod colors;
mod mode;
mod text;
mod tree;

pub use colors::{Color, RgbColor};
pub use mode::ColorMode;
pub use text::{StyledText, Text, always, auto, never};
pub use tree::{LeafBuilder, LeafLineBuilder, NodeBuilder, NodeLineBuilder, TreeNode, leaf, node};
