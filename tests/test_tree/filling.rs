use antex::{Color, ColorMode, StyledText, leaf, node};

#[test]
fn tree_content_fill_should_work() {
  const EXPECTED: &str = r#"
 root
 ├─ red first*********
 └─ yellow second*****
"#;

  let cm = ColorMode::Off;
  let mut root = node(Color::None, cm).line().s("root").end();
  let child_1 = node(Color::None, cm).line().s("red first").fill('*', 18).end().end();
  let child_2 = leaf(cm).line().s("yellow second").fill('*', 18).end().end();
  root.add_child(child_1);
  root.add_child(child_2);
  assert_eq!(EXPECTED, format!("\n{}", root.end()));
}
