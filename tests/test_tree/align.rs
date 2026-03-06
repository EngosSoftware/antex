use antex::{Color, ColorMode, StyledText, leaf, node};

#[test]
fn tree_content_align_left_should_work() {
  const EXPECTED: &str = r#"
 root
 ├─ red    first
 └─ yellow second
"#;

  let cm = ColorMode::Off;
  let mut root = node(Color::None, cm).line().s("root").end();
  let child_1 = node(Color::None, cm).line().align_left("red", 6).s(" first").end().end();
  let child_2 = leaf(cm).line().align_left("yellow", 6).s(" second").end().end();
  root.add_child(child_1);
  root.add_child(child_2);
  assert_eq!(EXPECTED, format!("\n{}", root.end()));
}

#[test]
fn tree_content_align_right_should_work() {
  const EXPECTED: &str = r#"
 root
 ├─    red first
 └─ yellow second
"#;

  let cm = ColorMode::Off;
  let mut root = node(Color::None, cm).line().s("root").end();
  let child_1 = node(Color::None, cm).line().align_right("red", 6).s(" first").end().end();
  let child_2 = leaf(cm).line().align_right("yellow", 6).s(" second").end().end();
  root.add_child(child_1);
  root.add_child(child_2);
  assert_eq!(EXPECTED, format!("\n{}", root.end()));
}

#[test]
fn tree_content_align_center_should_work() {
  const EXPECTED: &str = r#"
 root
 ├─  red   first
 └─ yellow second
"#;

  let cm = ColorMode::Off;
  let mut root = node(Color::None, cm).line().s("root").end();
  let child_1 = node(Color::None, cm).line().align_center("red", 6).s(" first").end().end();
  let child_2 = leaf(cm).line().align_center("yellow", 6).s(" second").end().end();
  root.add_child(child_1);
  root.add_child(child_2);
  assert_eq!(EXPECTED, format!("\n{}", root.end()));
}
