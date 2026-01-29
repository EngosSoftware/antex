use antex::{leaf, node, Color, ColorMode, NodeBuilder, StyledText, TreeNode};

const C: Color = Color::None;
const CM: ColorMode = ColorMode::Off;

struct FailingWriter {
  written: usize,
  max: usize,
}

impl FailingWriter {
  fn new(max: usize) -> Self {
    Self { written: 0, max }
  }
}

impl std::fmt::Write for FailingWriter {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    if self.written + s.len() >= self.max {
      Err(std::fmt::Error)
    } else {
      self.written += s.len();
      Ok(())
    }
  }
}

fn get_root() -> NodeBuilder {
  node(C, CM).line().s("root").end()
}

fn get_leaf() -> TreeNode {
  leaf(CM).line().s("leaf").end().end()
}

fn get_leaf_2() -> TreeNode {
  leaf(CM).line().s("leaf").end().line().s("line 2").end().end()
}

fn get_node(index: usize) -> NodeBuilder {
  node(C, CM).line().s("node ").s(index).end().line().s("line ").s(index).end()
}

#[test]
fn _0001() {
  let tree = get_root().end();
  tree.write(&mut FailingWriter::new(6)).expect_err("");
}

#[test]
fn _0002() {
  let tree = get_root().child(get_leaf()).end();
  tree.write(&mut FailingWriter::new(13)).expect_err("");
}

#[test]
fn _0003() {
  let tree = get_root().child(get_leaf()).end();
  tree.write(&mut FailingWriter::new(14)).expect_err("");
}

#[test]
fn _0004() {
  let tree = get_root().child(get_node(1).child(get_leaf()).end()).end();
  _ = tree.write(&mut FailingWriter::new(24));
}

#[test]
fn _0005() {
  let tree = get_root().child(get_node(1).child(get_leaf_2()).end()).end();
  tree.write(&mut FailingWriter::new(51)).expect_err("");
}

#[test]
fn _0006() {
  let tree = get_root().child(get_node(1).child(get_node(2).child(get_leaf_2()).end()).end()).end();
  tree.write(&mut FailingWriter::new(13)).expect_err("");
}

#[test]
fn _0007() {
  let tree = get_root().child(get_node(1).child(get_node(2).child(get_leaf_2()).end()).end()).end();
  tree.write(&mut FailingWriter::new(24)).expect_err("");
}
