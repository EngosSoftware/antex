use antex::{Color, ColorMode, StyledText, leaf, node};

#[test]
fn matches_should_work() {
  const EXPECTED: &str = r#"
 root
 ├─ -red
 └─ +green
"#;

  let cm = ColorMode::Off;
  let mut root = node(Color::None, cm).line().s("root").end();
  let mut flag = false;
  let child_1 = node(Color::None, cm).line().matches([(flag, '+'), (!flag, '-')]).s("red").end().end();
  flag = true;
  let child_2 = leaf(cm).line().matches([(flag, '+'), (!flag, '-')]).s("green").end().end();
  root.add_child(child_1);
  root.add_child(child_2);
  assert_eq!(EXPECTED, format!("\n{}", root.end()));
}
