use antex::{Color, ColorMode, StyledText, leaf, node};

#[test]
fn tree_content_pad_should_work() {
  const EXPECTED: &str = r#"
 root
 ├─ *****red first
 └─ *****yellow second
"#;

  let cm = ColorMode::Off;
  let mut root = node(Color::None, cm).line().s("root").end();
  let child_1 = node(Color::None, cm).line().pad('*', 5, "red").s(" first").end().end();
  let child_2 = leaf(cm).line().pad('*', 5, "yellow").s(" second").end().end();
  root.add_child(child_1);
  root.add_child(child_2);
  assert_eq!(EXPECTED, format!("\n{}", root.end()));
}

#[test]
fn tree_content_indent_should_work() {
  const EXPECTED: &str = r#"
 root
 ├─      red first
 └─      yellow second
"#;

  let cm = ColorMode::Off;
  let mut root = node(Color::None, cm).line().s("root").end();
  let child_1 = node(Color::None, cm).line().indent(5, "red").s(" first").end().end();
  let child_2 = leaf(cm).line().indent(5, "yellow").s(" second").end().end();
  root.add_child(child_1);
  root.add_child(child_2);
  assert_eq!(EXPECTED, format!("\n{}", root.end()));
}

#[test]
fn tree_content_pad_left_should_work() {
  const EXPECTED: &str = r#"
 root
 ├─ *****red first
 └─ **yellow second
"#;

  let cm = ColorMode::Off;
  let mut root = node(Color::None, cm).line().s("root").end();
  let child_1 = node(Color::None, cm).line().pad_left('*', "red", 8).s(" first").end().end();
  let child_2 = leaf(cm).line().pad_left('*', "yellow", 8).s(" second").end().end();
  root.add_child(child_1);
  root.add_child(child_2);
  assert_eq!(EXPECTED, format!("\n{}", root.end()));
}

#[test]
fn tree_content_pad_right_should_work() {
  const EXPECTED: &str = r#"
 root
 ├─ red***** first
 └─ yellow** second
"#;

  let cm = ColorMode::Off;
  let mut root = node(Color::None, cm).line().s("root").end();
  let child_1 = node(Color::None, cm).line().pad_right('*', "red", 8).s(" first").end().end();
  let child_2 = leaf(cm).line().pad_right('*', "yellow", 8).s(" second").end().end();
  root.add_child(child_1);
  root.add_child(child_2);
  assert_eq!(EXPECTED, format!("\n{}", root.end()));
}

#[test]
fn tree_content_pad_center_should_work() {
  const EXPECTED: &str = r#"
 root
 ├─ **red*** first
 └─ *yellow* second
"#;

  let cm = ColorMode::Off;
  let mut root = node(Color::None, cm).line().s("root").end();
  let child_1 = node(Color::None, cm).line().pad_center('*', "red", 8).s(" first").end().end();
  let child_2 = leaf(cm).line().pad_center('*', "yellow", 8).s(" second").end().end();
  root.add_child(child_1);
  root.add_child(child_2);
  assert_eq!(EXPECTED, format!("\n{}", root.end()));
}
