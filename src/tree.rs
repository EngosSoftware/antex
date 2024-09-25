//! # Styled tree

use crate::colors::ColorMode;
use crate::text::Text;

const NONE: &str = "   ";
const EDGE: &str = " └─";
const PIPE: &str = " │ ";
const FORK: &str = " ├─";

pub struct Tree {
    root: TreeNode,
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write(f)
    }
}

impl Tree {
    pub fn new(root: TreeNode) -> Self {
        Self { root }
    }

    /// Writes the tree to provided writer.
    pub fn write(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.write_node(f, &self.root, &[])
    }

    /// Writes the tree to provided writer with specified indentation.
    pub fn write_indent(&self, f: &mut dyn std::fmt::Write, indent: usize) -> std::fmt::Result {
        let mut tree = String::default();
        self.write_node(&mut tree, &self.root, &[])?;
        let indent = " ".repeat(indent);
        for line in tree.lines() {
            writeln!(f, "{}{}", indent, line)?;
        }
        Ok(())
    }

    /// Writes the specified node to provided writer.
    fn write_node(
        &self,
        f: &mut dyn std::fmt::Write,
        node: &TreeNode,
        level: &[usize],
    ) -> std::fmt::Result {
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
                    self.write_node(f, node, &level_next)?;
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

/// Types of nodes in styled tree.
pub enum TreeNode {
    /// Root or intermediary node in tree, always have one or mode child nodes.
    Node(Text, Vec<TreeNode>),
    /// Leaf node in the tree, never has any child nodes.
    Leaf(Vec<Text>),
}

impl TreeNode {
    pub fn node() -> NodeBuilder {
        NodeBuilder::default()
    }

    pub fn leaf() -> LeafBuilder {
        LeafBuilder::default()
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
            .line(Text::new(cm).str("node 1"))
            .child(
                TreeNode::node()
                    .line(Text::new(cm).str("node 1_1"))
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).str("line 1_1_1"))
                            .line(Text::new(cm).str("line 1_1_2"))
                            .line(Text::new(cm).str("line 1_1_3"))
                            .line(Text::new(cm).str("line 1_1_4"))
                            .done(),
                    )
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).str("only one line"))
                            .done(),
                    )
                    .done(),
            )
            .child(
                TreeNode::node()
                    .line(Text::new(cm).str("node 1_2"))
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).str("only one line"))
                            .done(),
                    )
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).str("line 1_2_1"))
                            .line(Text::new(cm).str("line 1_2_2"))
                            .line(Text::new(cm).str("line 1_2_3"))
                            .line(Text::new(cm).str("line 1_2_4"))
                            .done(),
                    )
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).str("only one line"))
                            .done(),
                    )
                    .done(),
            )
            .child(
                TreeNode::node()
                    .line(Text::new(cm).str("node 1_3"))
                    .child(
                        TreeNode::node()
                            .line(Text::new(cm).str("node 1_3_1"))
                            .child(
                                TreeNode::leaf()
                                    .line(Text::new(cm).str("line 1_3_1_1"))
                                    .line(Text::new(cm).str("line 1_3_1_2"))
                                    .line(Text::new(cm).str("line 1_3_1_3"))
                                    .line(Text::new(cm).str("line 1_3_1_4"))
                                    .done(),
                            )
                            .child(
                                TreeNode::leaf()
                                    .line(Text::new(cm).str("only one line"))
                                    .done(),
                            )
                            .done(),
                    )
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).str("line 1_3_1"))
                            .line(Text::new(cm).str("line 1_3_2"))
                            .line(Text::new(cm).str("line 1_3_3"))
                            .line(Text::new(cm).str("line 1_3_4"))
                            .done(),
                    )
                    .child(
                        TreeNode::leaf()
                            .line(Text::new(cm).str("only one line"))
                            .done(),
                    )
                    .done(),
            )
            .done();

        let tree = Tree::new(root);

        let mut output = String::new();
        let _ = writeln!(&mut output);
        let _ = tree.write(&mut output);
        assert_eq!(EXPECTED, output);
    }
}
