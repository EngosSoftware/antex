use antex::{leaf, node, Color, ColorMode, NodeBuilder, StyledText, TreeNode};
use std::fmt::Write;

const C: Color = Color::None;
const CM: ColorMode = ColorMode::Off;

struct FailingWriter {
  pattern: String,
}

impl FailingWriter {
  fn new(pattern: impl AsRef<str>) -> Self {
    Self {
      pattern: pattern.as_ref().to_string(),
    }
  }
}

impl Write for FailingWriter {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    if self.pattern == "^" {
      return Err(std::fmt::Error);
    } else if self.pattern.len() == 1 {
      for ch in s.chars() {
        if ch.to_string() == self.pattern {
          return Err(std::fmt::Error);
        }
      }
    } else if self.pattern == s {
      return Err(std::fmt::Error);
    }
    Ok(())
  }
}

fn get_root() -> NodeBuilder {
  node(C, CM).line().s("root").end()
}

fn get_leaf() -> TreeNode {
  leaf(CM).line().s("leaf#").end().line().s("line*").end().end()
}

fn get_node(ch: char) -> NodeBuilder {
  node(C, CM).line().s("node").s(ch).end()
}

fn get_tree() -> TreeNode {
  get_root()
    .child(get_node('@').child(get_node('%').child(get_leaf()).end()).end())
    .child(get_node('&').end())
    .end()
}

#[test]
fn _0001() {
  let tree = get_tree();
  let indent = 0;
  write!(&mut FailingWriter::new("^"), "{:width$}", tree, width = indent).unwrap_err();
  write!(&mut FailingWriter::new("#"), "{:width$}", tree, width = indent).unwrap_err();
  write!(&mut FailingWriter::new("*"), "{:width$}", tree, width = indent).unwrap_err();
  write!(&mut FailingWriter::new("@"), "{:width$}", tree, width = indent).unwrap_err();
  write!(&mut FailingWriter::new("   "), "{:width$}", tree, width = indent).unwrap_err();
  write!(&mut FailingWriter::new(" └─"), "{:width$}", tree, width = indent).unwrap_err();
  write!(&mut FailingWriter::new(" ├─"), "{:width$}", tree, width = indent).unwrap_err();
  write!(&mut FailingWriter::new(" │ "), "{:width$}", tree, width = indent).unwrap_err();
}
