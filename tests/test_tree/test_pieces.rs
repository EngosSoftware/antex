use antex::{node, Color, ColorMode, StyledText, Text};

const EXPECTED: &str = r#"
 root
 ├─ child 1
 ├─ child 2
 │  └─ child 2a
 └─ child 3
"#;

#[test]
fn building_tree_from_pieces_should_work() {
  let cm = ColorMode::Off;
  let mut root = node(Color::None, cm).line().s("root").end();

  let child_1 = node(Color::None, cm).line().s("child 1").end().end();

  let child_2a = node(Color::None, cm).line().s("child 2a").end().end();

  let mut child_2_builder = node(Color::None, cm);
  child_2_builder.set_line(Text::new(cm).s("child 2"));

  let child_3 = node(Color::None, cm).line().s("child 3").end().end();

  root.add_child(child_1);
  root.add_child(child_2_builder.opt_child(Some(child_2a)).opt_child(None).end());
  root.add_opt_child(Some(child_3));
  root.add_opt_child(None);

  assert_eq!(EXPECTED, format!("\n{}", root.end()));
}
