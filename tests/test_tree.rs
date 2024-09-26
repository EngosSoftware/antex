use antex::{ColorMode, Text, TreeNode};
use std::fmt::Write;

#[test]
fn building_tree_should_work() {
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
          .child(TreeNode::leaf().line(Text::new(cm).s("only one line")).done())
          .done(),
      )
      .child(
        TreeNode::node()
          .line(Text::new(cm).s("node 1_2"))
          .child(TreeNode::leaf().line(Text::new(cm).s("only one line")).done())
          .child(
            TreeNode::leaf()
              .line(Text::new(cm).s("line 1_2_1"))
              .line(Text::new(cm).s("line 1_2_2"))
              .line(Text::new(cm).s("line 1_2_3"))
              .line(Text::new(cm).s("line 1_2_4"))
              .done(),
          )
          .child(TreeNode::leaf().line(Text::new(cm).s("only one line")).done())
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
              .child(TreeNode::leaf().line(Text::new(cm).s("only one line")).done())
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
          .child(TreeNode::leaf().line(Text::new(cm).s("only one line")).done())
          .done(),
      )
      .done();

    let mut output = String::new();
    let _ = writeln!(&mut output);
    let _ = root.write(&mut output);
    assert_eq!(EXPECTED, output);
  }
}
