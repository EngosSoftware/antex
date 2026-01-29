//! # Styled text and tree in terminal

mod colors;
mod mode;
mod text;
mod tree;

pub use colors::{Color, RgbColor};
pub use mode::ColorMode;
pub use text::{StyledText, Text};
pub use tree::{leaf, node, TreeNode};
