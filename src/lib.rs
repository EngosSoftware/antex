//! # Styled text and tree in terminal

mod colors;
mod text;
mod tree;

pub use colors::{Color, ColorMode, RgbColor};
pub use text::{StyledText, Text};
pub use tree::{leaf, node, TreeNode};
