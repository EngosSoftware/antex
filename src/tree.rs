//! # Styled tree

use crate::text::Text;
use std::fmt;
use std::fmt::Display;

const NONE: &str = "   ";
const EDGE: &str = " └─";
const PIPE: &str = " │ ";
const FORK: &str = " ├─";

/// Types of nodes in styled tree.
pub enum TreeNode {
    /// Root or intermediary node in tree, always have one or mode child nodes.
    Node(Text, Vec<TreeNode>),
    /// Leaf node in the tree, never has any child nodes.
    Leaf(Vec<Text>),
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f)
    }
}

impl TreeNode {
    pub fn node() -> NodeBuilder {
        NodeBuilder::default()
    }

    pub fn leaf() -> LeafBuilder {
        LeafBuilder::default()
    }

    /// Writes node to provided writer.
    pub fn write(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        Self::write_node(f, self, &[])
    }

    /// Writes node to provided writer with specified indentation.
    pub fn write_indent(&self, f: &mut dyn fmt::Write, indent: usize) -> fmt::Result {
        let mut tree = String::default();
        Self::write_node(&mut tree, self, &[])?;
        let indent = " ".repeat(indent);
        for line in tree.lines() {
            writeln!(f, "{}{}", indent, line)?;
        }
        Ok(())
    }

    /// Writes node.
    fn write_node(f: &mut dyn fmt::Write, node: &TreeNode, level: &[usize]) -> fmt::Result {
        let max_pos = level.len();
        let mut second_line = String::new();
        for (pos, lev) in level.iter().enumerate() {
            let last_row = pos == max_pos - 1;
            if *lev == 1 {
                if !last_row {
                    write!(f, "{}", NONE)?
                } else {
                    write!(f, "{}", EDGE)?
                }
                second_line.push_str(NONE);
            } else {
                if !last_row {
                    write!(f, "{}", PIPE)?
                } else {
                    write!(f, "{}", FORK)?
                }
                second_line.push_str(PIPE);
            }
        }
        match node {
            TreeNode::Node(title, children) => {
                let mut deep = children.len();
                writeln!(f, " {}", title)?;
                for node in children {
                    let mut level_next = level.to_vec();
                    level_next.push(deep);
                    deep -= 1;
                    Self::write_node(f, node, &level_next)?;
                }
            }
            TreeNode::Leaf(lines) => {
                for (i, line) in lines.iter().enumerate() {
                    match i {
                        0 => writeln!(f, " {}", line)?,
                        _ => writeln!(f, "{} {}", second_line, line)?,
                    }
                }
            }
        }
        Ok(())
    }
}

/// Builder for [TreeNode::Leaf].
#[derive(Default)]
pub struct LeafBuilder {
    lines: Vec<Text>,
}

impl LeafBuilder {
    pub fn line(mut self, line: Text) -> Self {
        self.lines.push(line);
        self
    }

    pub fn add_line(&mut self, line: Text) {
        self.lines.push(line);
    }

    pub fn done(self) -> TreeNode {
        TreeNode::Leaf(self.lines)
    }
}

/// Builder for [TreeNode::Node].
#[derive(Default)]
pub struct NodeBuilder {
    line: Text,
    children: Vec<TreeNode>,
}

impl NodeBuilder {
    pub fn line(mut self, line: Text) -> Self {
        self.line = line;
        self
    }

    pub fn set_line(&mut self, line: Text) {
        self.line = line;
    }

    pub fn child(mut self, child: TreeNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

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

    pub fn done(self) -> TreeNode {
        TreeNode::Node(self.line, self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::ColorMode;
    use std::fmt::Write;

    const EXPECTED: &str = r#"
 node 1
 ├─ node 1_1
 │  ├─ line 1_1_1
 │  │  line 1_1_2
 │  │  line 1_1_3
 │  │  line 1_1_4
 │  └─ only one line
 ├─ node 1_2
 │  ├─ only one line
 │  ├─ line 1_2_1
 │  │  line 1_2_2
 │  │  line 1_2_3
 │  │  line 1_2_4
 │  └─ only one line
 └─ node 1_3
    ├─ node 1_3_1
    │  ├─ line 1_3_1_1
    │  │  line 1_3_1_2
    │  │  line 1_3_1_3
    │  │  line 1_3_1_4
    │  └─ only one line
    ├─ line 1_3_1
    │  line 1_3_2
    │  line 1_3_3
    │  line 1_3_4
    └─ only one line
"#;

    #[test]
    fn test_ascii_tree() {
        let cm = ColorMode::Off;
        let root = TreeNode::node()
            .line(Text::new(cm).s("node 1"))
            .child(
                TreeNode::node()
                    .line(Text::new(cm).s("node 1_1"))
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).s("line 1_1_1"))
                            .line(Text::new(cm).s("line 1_1_2"))
                            .line(Text::new(cm).s("line 1_1_3"))
                            .line(Text::new(cm).s("line 1_1_4"))
                            .done(),
                    )
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).s("only one line"))
                            .done(),
                    )
                    .done(),
            )
            .child(
                TreeNode::node()
                    .line(Text::new(cm).s("node 1_2"))
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).s("only one line"))
                            .done(),
                    )
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).s("line 1_2_1"))
                            .line(Text::new(cm).s("line 1_2_2"))
                            .line(Text::new(cm).s("line 1_2_3"))
                            .line(Text::new(cm).s("line 1_2_4"))
                            .done(),
                    )
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).s("only one line"))
                            .done(),
                    )
                    .done(),
            )
            .child(
                TreeNode::node()
                    .line(Text::new(cm).s("node 1_3"))
                    .child(
                        TreeNode::node()
                            .line(Text::new(cm).s("node 1_3_1"))
                            .child(
                                TreeNode::leaf()
                                    .line(Text::new(cm).s("line 1_3_1_1"))
                                    .line(Text::new(cm).s("line 1_3_1_2"))
                                    .line(Text::new(cm).s("line 1_3_1_3"))
                                    .line(Text::new(cm).s("line 1_3_1_4"))
                                    .done(),
                            )
                            .child(
                                TreeNode::leaf()
                                    .line(Text::new(cm).s("only one line"))
                                    .done(),
                            )
                            .done(),
                    )
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).s("line 1_3_1"))
                            .line(Text::new(cm).s("line 1_3_2"))
                            .line(Text::new(cm).s("line 1_3_3"))
                            .line(Text::new(cm).s("line 1_3_4"))
                            .done(),
                    )
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).s("only one line"))
                            .done(),
                    )
                    .done(),
            )
            .done();

        let mut output = String::new();
        let _ = writeln!(&mut output);
        let _ = root.write(&mut output);
        assert_eq!(EXPECTED, output);
    }
}
