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
  leaf(CM).line().s("leaf").end().line().s("line 2").end().end()
}

fn get_node(index: usize) -> NodeBuilder {
  node(C, CM).line().s("node ").s(index).end()
}

fn get_tree() -> TreeNode {
  get_root()
    .child(get_node(1).child(get_node(2).child(get_leaf()).end()).end())
    .child(get_node(3).end())
    .end()
}

#[test]
fn _0001() {
  let tree = get_tree();
  tree.write(&mut FailingWriter::new(6)).expect_err("");
  tree.write(&mut FailingWriter::new(13)).expect_err("");
  tree.write(&mut FailingWriter::new(26)).expect_err("");
  tree.write(&mut FailingWriter::new(33)).expect_err("");
  tree.write(&mut FailingWriter::new(49)).expect_err("");
  tree.write(&mut FailingWriter::new(62)).expect_err("");
  tree.write(&mut FailingWriter::new(81)).expect_err("");
}

#[test]
fn _0002() {
  let tree = get_tree();
  tree.write_indent(&mut FailingWriter::new(6), 0).expect_err("");
  tree.write_indent(&mut FailingWriter::new(13), 0).expect_err("");
  tree.write_indent(&mut FailingWriter::new(26), 0).expect_err("");
  tree.write_indent(&mut FailingWriter::new(33), 0).expect_err("");
  tree.write_indent(&mut FailingWriter::new(49), 0).expect_err("");
  tree.write_indent(&mut FailingWriter::new(62), 0).expect_err("");
  tree.write_indent(&mut FailingWriter::new(81), 0).expect_err("");
}
