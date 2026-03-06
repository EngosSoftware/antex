//! # Styled tree

use crate::colors::{Color, RgbColor};
use crate::mode::ColorMode;
use crate::text::{StyledText, Text};
use std::fmt::{self, Display, Write};

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
  Node(Text, Vec<TreeNode>, Color, ColorMode),
  /// Leaf node in a tree, never has any child nodes.
  Leaf(Vec<Text>),
}

impl Display for TreeNode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let indent = " ".repeat(f.width().unwrap_or_default());
    Self::write_node(f, self, &mut vec![], &indent)
  }
}

impl TreeNode {
  /// Writes this node to provided writer.
  fn write_node(f: &mut dyn Write, node: &TreeNode, levels: &mut Vec<Level>, indent: &str) -> fmt::Result {
    // Display lines of text.
    let max_pos = levels.len();
    let mut second_line = String::new();
    write!(f, "{}", indent)?;
    for (pos, lev) in levels.iter().enumerate() {
      let color = lev.cm.color(lev.color, false);
      let clear = lev.cm.reset();
      let last_row = pos == max_pos - 1;
      if lev.n == 1 {
        if !last_row {
          write!(f, "{}{}{}", color, NONE, clear)?
        } else {
          write!(f, "{}{}{}", color, EDGE, clear)?
        }
        _ = write!(&mut second_line, "{}{}{}", color, NONE, clear);
      } else {
        if !last_row {
          write!(f, "{}{}{}", color, PIPE, clear)?
        } else {
          write!(f, "{}{}{}", color, FORK, clear)?
        }
        _ = write!(&mut second_line, "{}{}{}", color, PIPE, clear);
      }
    }
    // Traverse the child nodes.
    match node {
      TreeNode::Node(title, children, color, cm) => {
        let mut deep = children.len();
        writeln!(f, " {}", title)?;
        for node in children {
          levels.push(Level { n: deep, color: *color, cm: *cm });
          deep -= 1;
          Self::write_node(f, node, levels, indent)?;
          levels.pop();
        }
      }
      TreeNode::Leaf(lines) => {
        for (i, line) in lines.iter().enumerate() {
          match i {
            0 => writeln!(f, " {}", line)?,
            _ => writeln!(f, "{}{} {}", indent, second_line, line)?,
          }
        }
      }
    }
    Ok(())
  }
}

pub fn node(color: Color, cm: ColorMode) -> NodeBuilder {
  NodeBuilder::new(color, cm)
}

pub fn leaf(cm: ColorMode) -> LeafBuilder {
  LeafBuilder::new(cm)
}

/// Builder for [TreeNode::Leaf].
#[derive(Debug, Clone)]
pub struct LeafBuilder {
  /// Color mode.
  cm: ColorMode,
  /// Multiline text in a leaf node.
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
    TreeNode::Leaf(self.lines)
  }
}

/// Builder for [TreeNode::Leaf]'s line.
#[derive(Debug, Clone)]
pub struct LeafLineBuilder {
  cm: ColorMode,
  lines: Vec<Text>,
  text: Text,
}

impl LeafLineBuilder {
  pub fn end(self) -> LeafBuilder {
    let mut lines = self.lines;
    lines.push(self.text.reset());
    LeafBuilder { cm: self.cm, lines }
  }
}

impl StyledText for LeafLineBuilder {
  fn s<T: Display>(mut self, s: T) -> Self {
    self.text = self.text.s(s);
    self
  }

  fn reset(mut self) -> Self {
    self.text = self.text.reset();
    self
  }

  fn repeat<T: Display>(mut self, s: T, n: usize) -> Self {
    self.text = self.text.repeat(s, n);
    self
  }

  fn plural<T: Display>(mut self, s: T, n: usize) -> Self {
    self.text = self.text.plural(s, n);
    self
  }

  fn indent<T: Display>(self, indent: usize, s: T) -> Self {
    self.pad(' ', indent, s)
  }

  fn align_left<T: Display>(self, s: T, width: usize) -> Self {
    self.pad_right(' ', s, width)
  }

  fn align_right<T: Display>(self, s: T, width: usize) -> Self {
    self.pad_left(' ', s, width)
  }

  fn align_center<T: Display>(self, s: T, width: usize) -> Self {
    self.pad_center(' ', s, width)
  }

  fn pad<T: Display>(mut self, ch: char, padding: usize, s: T) -> Self {
    self.text = self.text.pad(ch, padding, s);
    self
  }

  fn pad_left<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
    self.text = self.text.pad_left(ch, s, width);
    self
  }

  fn pad_right<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
    self.text = self.text.pad_right(ch, s, width);
    self
  }

  fn pad_center<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
    self.text = self.text.pad_center(ch, s, width);
    self
  }

  fn choose<T: Display>(mut self, condition: bool, when_true: T, when_false: T) -> Self {
    self.text = self.text.choose(condition, when_true, when_false);
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

  fn black(mut self) -> Self {
    self.text = self.text.black();
    self
  }

  fn bright_black(mut self) -> Self {
    self.text = self.text.bright_black();
    self
  }

  fn red(mut self) -> Self {
    self.text = self.text.red();
    self
  }

  fn bright_red(mut self) -> Self {
    self.text = self.text.bright_red();
    self
  }

  fn green(mut self) -> Self {
    self.text = self.text.green();
    self
  }

  fn bright_green(mut self) -> Self {
    self.text = self.text.bright_green();
    self
  }

  fn yellow(mut self) -> Self {
    self.text = self.text.yellow();
    self
  }

  fn bright_yellow(mut self) -> Self {
    self.text = self.text.bright_yellow();
    self
  }

  fn blue(mut self) -> Self {
    self.text = self.text.blue();
    self
  }

  fn bright_blue(mut self) -> Self {
    self.text = self.text.bright_blue();
    self
  }

  fn magenta(mut self) -> Self {
    self.text = self.text.magenta();
    self
  }

  fn bright_magenta(mut self) -> Self {
    self.text = self.text.bright_magenta();
    self
  }

  fn cyan(mut self) -> Self {
    self.text = self.text.cyan();
    self
  }

  fn bright_cyan(mut self) -> Self {
    self.text = self.text.bright_cyan();
    self
  }

  fn white(mut self) -> Self {
    self.text = self.text.white();
    self
  }

  fn bright_white(mut self) -> Self {
    self.text = self.text.bright_white();
    self
  }

  fn bg_black(mut self) -> Self {
    self.text = self.text.bg_black();
    self
  }

  fn bg_bright_black(mut self) -> Self {
    self.text = self.text.bg_bright_black();
    self
  }

  fn bg_red(mut self) -> Self {
    self.text = self.text.bg_red();
    self
  }

  fn bg_bright_red(mut self) -> Self {
    self.text = self.text.bg_bright_red();
    self
  }

  fn bg_green(mut self) -> Self {
    self.text = self.text.bg_green();
    self
  }

  fn bg_bright_green(mut self) -> Self {
    self.text = self.text.bg_bright_green();
    self
  }

  fn bg_yellow(mut self) -> Self {
    self.text = self.text.bg_yellow();
    self
  }

  fn bg_bright_yellow(mut self) -> Self {
    self.text = self.text.bg_bright_yellow();
    self
  }

  fn bg_blue(mut self) -> Self {
    self.text = self.text.bg_blue();
    self
  }

  fn bg_bright_blue(mut self) -> Self {
    self.text = self.text.bg_bright_blue();
    self
  }

  fn bg_magenta(mut self) -> Self {
    self.text = self.text.bg_magenta();
    self
  }

  fn bg_bright_magenta(mut self) -> Self {
    self.text = self.text.bg_bright_magenta();
    self
  }

  fn bg_cyan(mut self) -> Self {
    self.text = self.text.bg_cyan();
    self
  }

  fn bg_bright_cyan(mut self) -> Self {
    self.text = self.text.bg_bright_cyan();
    self
  }

  fn bg_white(mut self) -> Self {
    self.text = self.text.bg_white();
    self
  }

  fn bg_bright_white(mut self) -> Self {
    self.text = self.text.bg_bright_white();
    self
  }

  fn color(mut self, c: Color) -> Self {
    self.text = self.text.color(c);
    self
  }

  fn bright_color(mut self, c: Color) -> Self {
    self.text = self.text.bright_color(c);
    self
  }

  fn bg_color(mut self, c: Color) -> Self {
    self.text = self.text.bg_color(c);
    self
  }

  fn bg_bright_color(mut self, c: Color) -> Self {
    self.text = self.text.bg_bright_color(c);
    self
  }

  fn color_8(mut self, c: u8) -> Self {
    self.text = self.text.color_8(c);
    self
  }

  fn bright_color_8(mut self, c: u8) -> Self {
    self.text = self.text.bright_color_8(c);
    self
  }

  fn bg_color_8(mut self, c: u8) -> Self {
    self.text = self.text.bg_color_8(c);
    self
  }

  fn bg_bright_color_8(mut self, c: u8) -> Self {
    self.text = self.text.bg_bright_color_8(c);
    self
  }

  fn color_256(mut self, c: u8) -> Self {
    self.text = self.text.color_256(c);
    self
  }

  fn bg_color_256(mut self, c: u8) -> Self {
    self.text = self.text.bg_color_256(c);
    self
  }

  fn color_rgb(mut self, c: RgbColor) -> Self {
    self.text = self.text.color_rgb(c);
    self
  }

  fn bg_color_rgb(mut self, c: RgbColor) -> Self {
    self.text = self.text.bg_color_rgb(c);
    self
  }
}

/// Builder for [TreeNode::Node].
#[derive(Debug, Clone)]
pub struct NodeBuilder {
  color: Color,
  cm: ColorMode,
  line: Text,
  children: Vec<TreeNode>,
}

impl NodeBuilder {
  pub fn new(color: Color, cm: ColorMode) -> Self {
    Self {
      color,
      cm,
      line: Text::new(cm),
      children: Vec::default(),
    }
  }

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
    TreeNode::Node(self.line, self.children, self.color, self.cm)
  }
}

/// Builder for [TreeNode::Node]'s text line.
#[derive(Debug, Clone)]
pub struct NodeLineBuilder {
  color: Color,
  cm: ColorMode,
  children: Vec<TreeNode>,
  text: Text,
}

impl NodeLineBuilder {
  pub fn end(self) -> NodeBuilder {
    NodeBuilder {
      color: self.color,
      cm: self.cm,
      line: self.text.reset(),
      children: self.children,
    }
  }
}

impl StyledText for NodeLineBuilder {
  fn s<T: Display>(mut self, s: T) -> Self {
    self.text = self.text.s(s);
    self
  }

  fn reset(mut self) -> Self {
    self.text = self.text.reset();
    self
  }

  fn repeat<T: Display>(mut self, s: T, n: usize) -> Self {
    self.text = self.text.repeat(s, n);
    self
  }

  fn plural<T: Display>(mut self, s: T, n: usize) -> Self {
    self.text = self.text.plural(s, n);
    self
  }

  fn indent<T: Display>(self, indent: usize, s: T) -> Self {
    self.pad(' ', indent, s)
  }

  fn align_left<T: Display>(self, s: T, width: usize) -> Self {
    self.pad_right(' ', s, width)
  }

  fn align_right<T: Display>(self, s: T, width: usize) -> Self {
    self.pad_left(' ', s, width)
  }

  fn align_center<T: Display>(self, s: T, width: usize) -> Self {
    self.pad_center(' ', s, width)
  }

  fn pad<T: Display>(mut self, ch: char, padding: usize, s: T) -> Self {
    self.text = self.text.pad(ch, padding, s);
    self
  }

  fn pad_left<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
    self.text = self.text.pad_left(ch, s, width);
    self
  }

  fn pad_right<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
    self.text = self.text.pad_right(ch, s, width);
    self
  }

  fn pad_center<T: Display>(mut self, ch: char, s: T, width: usize) -> Self {
    self.text = self.text.pad_center(ch, s, width);
    self
  }

  fn choose<T: Display>(mut self, condition: bool, when_true: T, when_false: T) -> Self {
    self.text = self.text.choose(condition, when_true, when_false);
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

  fn black(mut self) -> Self {
    self.text = self.text.black();
    self
  }

  fn bright_black(mut self) -> Self {
    self.text = self.text.bright_black();
    self
  }

  fn red(mut self) -> Self {
    self.text = self.text.red();
    self
  }

  fn bright_red(mut self) -> Self {
    self.text = self.text.bright_red();
    self
  }

  fn green(mut self) -> Self {
    self.text = self.text.green();
    self
  }

  fn bright_green(mut self) -> Self {
    self.text = self.text.bright_green();
    self
  }

  fn yellow(mut self) -> Self {
    self.text = self.text.yellow();
    self
  }

  fn bright_yellow(mut self) -> Self {
    self.text = self.text.bright_yellow();
    self
  }

  fn blue(mut self) -> Self {
    self.text = self.text.blue();
    self
  }

  fn bright_blue(mut self) -> Self {
    self.text = self.text.bright_blue();
    self
  }

  fn magenta(mut self) -> Self {
    self.text = self.text.magenta();
    self
  }

  fn bright_magenta(mut self) -> Self {
    self.text = self.text.bright_magenta();
    self
  }

  fn cyan(mut self) -> Self {
    self.text = self.text.cyan();
    self
  }

  fn bright_cyan(mut self) -> Self {
    self.text = self.text.bright_cyan();
    self
  }

  fn white(mut self) -> Self {
    self.text = self.text.white();
    self
  }

  fn bright_white(mut self) -> Self {
    self.text = self.text.bright_white();
    self
  }

  fn bg_black(mut self) -> Self {
    self.text = self.text.bg_black();
    self
  }

  fn bg_bright_black(mut self) -> Self {
    self.text = self.text.bg_bright_black();
    self
  }

  fn bg_red(mut self) -> Self {
    self.text = self.text.bg_red();
    self
  }

  fn bg_bright_red(mut self) -> Self {
    self.text = self.text.bg_bright_red();
    self
  }

  fn bg_green(mut self) -> Self {
    self.text = self.text.bg_green();
    self
  }

  fn bg_bright_green(mut self) -> Self {
    self.text = self.text.bg_bright_green();
    self
  }

  fn bg_yellow(mut self) -> Self {
    self.text = self.text.bg_yellow();
    self
  }

  fn bg_bright_yellow(mut self) -> Self {
    self.text = self.text.bg_bright_yellow();
    self
  }

  fn bg_blue(mut self) -> Self {
    self.text = self.text.bg_blue();
    self
  }

  fn bg_bright_blue(mut self) -> Self {
    self.text = self.text.bg_bright_blue();
    self
  }

  fn bg_magenta(mut self) -> Self {
    self.text = self.text.bg_magenta();
    self
  }

  fn bg_bright_magenta(mut self) -> Self {
    self.text = self.text.bg_bright_magenta();
    self
  }

  fn bg_cyan(mut self) -> Self {
    self.text = self.text.bg_cyan();
    self
  }

  fn bg_bright_cyan(mut self) -> Self {
    self.text = self.text.bg_bright_cyan();
    self
  }

  fn bg_white(mut self) -> Self {
    self.text = self.text.bg_white();
    self
  }

  fn bg_bright_white(mut self) -> Self {
    self.text = self.text.bg_bright_white();
    self
  }

  fn color(mut self, c: Color) -> Self {
    self.text = self.text.color(c);
    self
  }

  fn bright_color(mut self, c: Color) -> Self {
    self.text = self.text.bright_color(c);
    self
  }

  fn bg_color(mut self, c: Color) -> Self {
    self.text = self.text.bg_color(c);
    self
  }

  fn bg_bright_color(mut self, c: Color) -> Self {
    self.text = self.text.bg_bright_color(c);
    self
  }

  fn color_8(mut self, c: u8) -> Self {
    self.text = self.text.color_8(c);
    self
  }

  fn bright_color_8(mut self, c: u8) -> Self {
    self.text = self.text.bright_color_8(c);
    self
  }

  fn bg_color_8(mut self, c: u8) -> Self {
    self.text = self.text.bg_color_8(c);
    self
  }

  fn bg_bright_color_8(mut self, c: u8) -> Self {
    self.text = self.text.bg_bright_color_8(c);
    self
  }

  fn color_256(mut self, c: u8) -> Self {
    self.text = self.text.color_256(c);
    self
  }

  fn bg_color_256(mut self, c: u8) -> Self {
    self.text = self.text.bg_color_256(c);
    self
  }

  fn color_rgb(mut self, c: RgbColor) -> Self {
    self.text = self.text.color_rgb(c);
    self
  }

  fn bg_color_rgb(mut self, c: RgbColor) -> Self {
    self.text = self.text.bg_color_rgb(c);
    self
  }
}
