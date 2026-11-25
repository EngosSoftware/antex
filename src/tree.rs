//! # Styled tree

use crate::colors::Color;
use crate::mode::ColorMode;
use crate::text::{StyledText, Text, internal};
use std::fmt::{Display, Formatter, Result, Write};

const NONE: &str = "   ";
const EDGE: &str = " └─";
const PIPE: &str = " │ ";
const FORK: &str = " ├─";

#[derive(Debug, Clone)]
struct Level {
    n: usize,
    color: Color,
    cm: ColorMode,
}

/// Types of nodes in styled tree.
#[derive(Debug, Clone)]
pub enum TreeNode {
    /// Root or intermediary node in a tree, always has one or mode child nodes.
    Node(Text, Vec<Self>, Color, ColorMode),
    /// Leaf node in a tree, never has any child nodes.
    Leaf(Vec<Text>),
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let indent = " ".repeat(f.width().unwrap_or_default());
        Self::write_node(f, self, &mut vec![], &indent)
    }
}

impl TreeNode {
    /// Writes this node to provided writer.
    fn write_node(f: &mut dyn Write, node: &Self, levels: &mut Vec<Level>, indent: &str) -> Result {
        // Display lines of text.
        let max_pos = levels.len();
        let mut second_line = String::new();
        write!(f, "{indent}")?;
        for (pos, lev) in levels.iter().enumerate() {
            let color = lev.cm.color(lev.color, false);
            let clear = lev.cm.reset();
            let last_row = pos == max_pos - 1;
            if lev.n == 1 {
                if last_row {
                    write!(f, "{color}{EDGE}{clear}")?;
                } else {
                    write!(f, "{color}{NONE}{clear}")?;
                }
                _ = write!(&mut second_line, "{color}{NONE}{clear}");
            } else {
                if last_row {
                    write!(f, "{color}{FORK}{clear}")?;
                } else {
                    write!(f, "{color}{PIPE}{clear}")?;
                }
                _ = write!(&mut second_line, "{color}{PIPE}{clear}");
            }
        }
        // Traverse the child nodes.
        match node {
            Self::Node(title, children, color, cm) => {
                let mut deep = children.len();
                writeln!(f, " {title}")?;
                for node in children {
                    levels.push(Level {
                        n: deep,
                        color: *color,
                        cm: *cm,
                    });
                    deep -= 1;
                    Self::write_node(f, node, levels, indent)?;
                    levels.pop();
                }
            }
            Self::Leaf(lines) => {
                for (i, line) in lines.iter().enumerate() {
                    match i {
                        0 => writeln!(f, " {line}")?,
                        _ => writeln!(f, "{indent}{second_line} {line}")?,
                    }
                }
            }
        }
        Ok(())
    }
}

#[must_use]
pub fn node(color: Color, cm: ColorMode) -> NodeBuilder {
    NodeBuilder::new(color, cm)
}

#[must_use]
pub fn leaf(cm: ColorMode) -> LeafBuilder {
    LeafBuilder::new(cm)
}

/// Builder for [`TreeNode::Leaf`].
#[derive(Debug, Clone)]
pub struct LeafBuilder {
    /// Color mode.
    cm: ColorMode,
    /// Multiline text in a leaf node.
    lines: Vec<Text>,
}

impl LeafBuilder {
    #[must_use]
    pub fn new(cm: ColorMode) -> Self {
        Self {
            cm,
            lines: Vec::default(),
        }
    }

    #[must_use]
    pub fn line(self) -> LeafLineBuilder {
        LeafLineBuilder {
            cm: self.cm,
            lines: self.lines,
            text: Text::new(self.cm),
        }
    }

    pub fn add_line(&mut self, line: Text) {
        self.lines.push(line);
    }

    #[must_use]
    pub fn end(self) -> TreeNode {
        TreeNode::Leaf(self.lines)
    }
}

/// Builder for [`TreeNode::Leaf`]'s line.
#[derive(Debug, Clone)]
pub struct LeafLineBuilder {
    cm: ColorMode,
    lines: Vec<Text>,
    text: Text,
}

impl LeafLineBuilder {
    #[must_use]
    pub fn end(self) -> LeafBuilder {
        let mut lines = self.lines;
        lines.push(self.text.reset());
        LeafBuilder { cm: self.cm, lines }
    }
}

impl internal::Sealed for LeafLineBuilder {
    fn text(&mut self, _: internal::Token) -> &mut Text {
        &mut self.text
    }
}

impl StyledText for LeafLineBuilder {}

/// Builder for [`TreeNode::Node`].
#[derive(Debug, Clone)]
pub struct NodeBuilder {
    color: Color,
    cm: ColorMode,
    line: Text,
    children: Vec<TreeNode>,
}

impl NodeBuilder {
    #[must_use]
    pub fn new(color: Color, cm: ColorMode) -> Self {
        Self {
            color,
            cm,
            line: Text::new(cm),
            children: Vec::default(),
        }
    }

    #[must_use]
    pub fn line(self) -> NodeLineBuilder {
        NodeLineBuilder {
            color: self.color,
            cm: self.cm,
            children: self.children,
            text: Text::new(self.cm),
        }
    }

    pub fn set_line(&mut self, line: Text) {
        self.line = line;
    }

    #[must_use]
    pub fn child(mut self, child: TreeNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    #[must_use]
    pub fn opt_child(mut self, opt_child: Option<TreeNode>) -> Self {
        if let Some(child) = opt_child {
            self.children.push(child);
        }
        self
    }

    pub fn add_opt_child(&mut self, opt_child: Option<TreeNode>) {
        if let Some(child) = opt_child {
            self.children.push(child);
        }
    }

    #[must_use]
    pub fn end(self) -> TreeNode {
        TreeNode::Node(self.line, self.children, self.color, self.cm)
    }
}

/// Builder for [`TreeNode::Node`]'s text line.
#[derive(Debug, Clone)]
pub struct NodeLineBuilder {
    color: Color,
    cm: ColorMode,
    children: Vec<TreeNode>,
    text: Text,
}

impl NodeLineBuilder {
    #[must_use]
    pub fn end(self) -> NodeBuilder {
        NodeBuilder {
            color: self.color,
            cm: self.cm,
            line: self.text.reset(),
            children: self.children,
        }
    }
}

impl internal::Sealed for NodeLineBuilder {
    fn text(&mut self, _: internal::Token) -> &mut Text {
        &mut self.text
    }
}

impl StyledText for NodeLineBuilder {}
