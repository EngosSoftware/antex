//! # Styled tree

use crate::colors::ColorMode;
use crate::text::{StyledText, Text};
use std::fmt;
use std::fmt::Display;

const NONE: &str = "   ";
const EDGE: &str = " └─";
const PIPE: &str = " │ ";
const FORK: &str = " ├─";

/// Types of nodes in styled tree.
#[derive(Debug, Clone)]
pub enum TreeNode {
  /// Root or intermediary node in tree, always have one or mode child nodes.
  Node(Text, Vec<TreeNode>, ColorMode),
  /// Leaf node in the tree, never has any child nodes.
  Leaf(Vec<Text>, ColorMode),
}

impl Display for TreeNode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.write(f)
  }
}

impl TreeNode {
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
      TreeNode::Node(title, children, _color_mode) => {
        let mut deep = children.len();
        writeln!(f, " {}", title)?;
        for node in children {
          let mut level_next = level.to_vec();
          level_next.push(deep);
          deep -= 1;
          Self::write_node(f, node, &level_next)?;
        }
      }
      TreeNode::Leaf(lines, _color_mode) => {
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

pub fn node(cm: ColorMode) -> NodeBuilder {
  NodeBuilder::new(cm)
}

pub fn leaf(cm: ColorMode) -> LeafBuilder {
  LeafBuilder::new(cm)
}

/// Builder for [TreeNode::Leaf].
#[derive(Debug, Clone)]
pub struct LeafBuilder {
  /// Color mode.
  cm: ColorMode,
  /// Multiline text in leaf node.
  lines: Vec<Text>,
}

impl LeafBuilder {
  pub fn new(cm: ColorMode) -> Self {
    Self { cm, lines: Vec::default() }
  }

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

  pub fn end(self) -> TreeNode {
    TreeNode::Leaf(self.lines, self.cm)
  }
}

/// Builder for [TreeNode::Leaf]'s text line.
#[derive(Debug, Clone)]
pub struct LeafLineBuilder {
  cm: ColorMode,
  lines: Vec<Text>,
  text: Text,
}

impl LeafLineBuilder {
  pub fn end(self) -> LeafBuilder {
    let mut lines = self.lines;
    lines.push(self.text);
    LeafBuilder { cm: self.cm, lines }
  }
}

impl StyledText for LeafLineBuilder {
  fn s<T: Display>(mut self, s: T) -> Self {
    self.text = self.text.s(s);
    self
  }

  fn nl(mut self) -> Self {
    self.text = self.text.nl();
    self
  }

  fn space(mut self) -> Self {
    self.text = self.text.space();
    self
  }

  fn spaces(mut self, n: usize) -> Self {
    self.text = self.text.spaces(n);
    self
  }

  fn dot(mut self) -> Self {
    self.text = self.text.dot();
    self
  }

  fn colon(mut self) -> Self {
    self.text = self.text.colon();
    self
  }

  fn slash(mut self) -> Self {
    self.text = self.text.slash();
    self
  }

  fn dots(mut self) -> Self {
    self.text = self.text.dots();
    self
  }

  fn plural<T: Display>(mut self, s: T, n: usize) -> Self {
    self.text = self.text.plural(s, n);
    self
  }

  fn black(mut self) -> Self {
    self.text = self.text.black();
    self
  }

  fn red(mut self) -> Self {
    self.text = self.text.red();
    self
  }

  fn green(mut self) -> Self {
    self.text = self.text.green();
    self
  }

  fn yellow(mut self) -> Self {
    self.text = self.text.yellow();
    self
  }

  fn blue(mut self) -> Self {
    self.text = self.text.blue();
    self
  }

  fn magenta(mut self) -> Self {
    self.text = self.text.magenta();
    self
  }

  fn cyan(mut self) -> Self {
    self.text = self.text.cyan();
    self
  }

  fn white(mut self) -> Self {
    self.text = self.text.white();
    self
  }

  fn bg_black(mut self) -> Self {
    self.text = self.text.bg_black();
    self
  }

  fn bg_red(mut self) -> Self {
    self.text = self.text.bg_red();
    self
  }

  fn bg_green(mut self) -> Self {
    self.text = self.text.bg_green();
    self
  }

  fn bg_yellow(mut self) -> Self {
    self.text = self.text.bg_yellow();
    self
  }

  fn bg_blue(mut self) -> Self {
    self.text = self.text.bg_blue();
    self
  }

  fn bg_magenta(mut self) -> Self {
    self.text = self.text.bg_magenta();
    self
  }

  fn bg_cyan(mut self) -> Self {
    self.text = self.text.bg_cyan();
    self
  }

  fn bg_white(mut self) -> Self {
    self.text = self.text.bg_white();
    self
  }

  fn color(mut self, value: u8) -> Self {
    self.text = self.text.color(value);
    self
  }

  fn bg_color(mut self, value: u8) -> Self {
    self.text = self.text.bg_color(value);
    self
  }

  fn color_256(mut self, value: u8) -> Self {
    self.text = self.text.color_256(value);
    self
  }

  fn bg_color_256(mut self, value: u8) -> Self {
    self.text = self.text.bg_color_256(value);
    self
  }

  fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
    self.text = self.text.rgb(r, g, b);
    self
  }

  fn bg_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
    self.text = self.text.bg_rgb(r, g, b);
    self
  }

  fn bold(mut self) -> Self {
    self.text = self.text.bold();
    self
  }

  fn italic(mut self) -> Self {
    self.text = self.text.italic();
    self
  }

  fn underline(mut self) -> Self {
    self.text = self.text.underline();
    self
  }

  fn clear(mut self) -> Self {
    self.text = self.text.clear();
    self
  }
}

/// Builder for [TreeNode::Node].
#[derive(Debug, Clone)]
pub struct NodeBuilder {
  cm: ColorMode,
  line: Text,
  children: Vec<TreeNode>,
}

impl NodeBuilder {
  pub fn new(cm: ColorMode) -> Self {
    Self {
      cm,
      line: Text::new(cm),
      children: Vec::default(),
    }
  }

  pub fn line(self) -> NodeLineBuilder {
    NodeLineBuilder {
      cm: self.cm,
      children: self.children,
      text: Text::new(self.cm),
    }
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

  pub fn end(self) -> TreeNode {
    TreeNode::Node(self.line, self.children, self.cm)
  }
}

/// Builder for [TreeNode::Node]'s text line.
#[derive(Debug, Clone)]
pub struct NodeLineBuilder {
  cm: ColorMode,
  children: Vec<TreeNode>,
  text: Text,
}

impl NodeLineBuilder {
  pub fn end(self) -> NodeBuilder {
    NodeBuilder {
      cm: self.cm,
      line: self.text,
      children: self.children,
    }
  }
}

impl StyledText for NodeLineBuilder {
  fn s<T: Display>(mut self, s: T) -> Self {
    self.text = self.text.s(s);
    self
  }

  fn nl(mut self) -> Self {
    self.text = self.text.nl();
    self
  }

  fn space(mut self) -> Self {
    self.text = self.text.space();
    self
  }

  fn spaces(mut self, n: usize) -> Self {
    self.text = self.text.spaces(n);
    self
  }

  fn dot(mut self) -> Self {
    self.text = self.text.dot();
    self
  }

  fn colon(mut self) -> Self {
    self.text = self.text.colon();
    self
  }

  fn slash(mut self) -> Self {
    self.text = self.text.slash();
    self
  }

  fn dots(mut self) -> Self {
    self.text = self.text.dots();
    self
  }

  fn plural<T: Display>(mut self, s: T, n: usize) -> Self {
    self.text = self.text.plural(s, n);
    self
  }

  fn black(mut self) -> Self {
    self.text = self.text.black();
    self
  }

  fn red(mut self) -> Self {
    self.text = self.text.red();
    self
  }

  fn green(mut self) -> Self {
    self.text = self.text.green();
    self
  }

  fn yellow(mut self) -> Self {
    self.text = self.text.yellow();
    self
  }

  fn blue(mut self) -> Self {
    self.text = self.text.blue();
    self
  }

  fn magenta(mut self) -> Self {
    self.text = self.text.magenta();
    self
  }

  fn cyan(mut self) -> Self {
    self.text = self.text.cyan();
    self
  }

  fn white(mut self) -> Self {
    self.text = self.text.white();
    self
  }

  fn bg_black(mut self) -> Self {
    self.text = self.text.bg_black();
    self
  }

  fn bg_red(mut self) -> Self {
    self.text = self.text.bg_red();
    self
  }

  fn bg_green(mut self) -> Self {
    self.text = self.text.bg_green();
    self
  }

  fn bg_yellow(mut self) -> Self {
    self.text = self.text.bg_yellow();
    self
  }

  fn bg_blue(mut self) -> Self {
    self.text = self.text.bg_blue();
    self
  }

  fn bg_magenta(mut self) -> Self {
    self.text = self.text.bg_magenta();
    self
  }

  fn bg_cyan(mut self) -> Self {
    self.text = self.text.bg_cyan();
    self
  }

  fn bg_white(mut self) -> Self {
    self.text = self.text.bg_white();
    self
  }

  fn color(mut self, value: u8) -> Self {
    self.text = self.text.color(value);
    self
  }

  fn bg_color(mut self, value: u8) -> Self {
    self.text = self.text.bg_color(value);
    self
  }

  fn color_256(mut self, value: u8) -> Self {
    self.text = self.text.color_256(value);
    self
  }

  fn bg_color_256(mut self, value: u8) -> Self {
    self.text = self.text.bg_color_256(value);
    self
  }

  fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
    self.text = self.text.rgb(r, g, b);
    self
  }

  fn bg_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
    self.text = self.text.bg_rgb(r, g, b);
    self
  }

  fn bold(mut self) -> Self {
    self.text = self.text.bold();
    self
  }

  fn italic(mut self) -> Self {
    self.text = self.text.italic();
    self
  }

  fn underline(mut self) -> Self {
    self.text = self.text.underline();
    self
  }

  fn clear(mut self) -> Self {
    self.text = self.text.clear();
    self
  }
}
