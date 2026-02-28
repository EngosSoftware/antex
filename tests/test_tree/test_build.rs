use antex::{Color, ColorMode, StyledText, leaf, node};

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

const EXPECTED_INDENTED: &str = r#"
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
fn building_tree_should_work() {
  let cm = ColorMode::Off;
  let root = node(Color::None, cm)
    .line()
    .s("node 1")
    .end()
    .child(
      node(Color::Yellow, cm)
        .line()
        .s("node 1_1")
        .end()
        .child(
          leaf(cm)
            .line()
            .s("line 1_1_1")
            .end()
            .line()
            .s("line 1_1_2")
            .end()
            .line()
            .s("line 1_1_3")
            .end()
            .line()
            .s("line 1_1_4")
            .end()
            .end(),
        )
        .child(leaf(cm).line().s("only one line").end().end())
        .end(),
    )
    .child(
      node(Color::None, cm)
        .line()
        .s("node 1_2")
        .end()
        .child(leaf(cm).line().s("only one line").end().end())
        .child(
          leaf(cm)
            .line()
            .s("line 1_2_1")
            .end()
            .line()
            .s("line 1_2_2")
            .end()
            .line()
            .s("line 1_2_3")
            .end()
            .line()
            .s("line 1_2_4")
            .end()
            .end(),
        )
        .child(leaf(cm).line().s("only one line").end().end())
        .end(),
    )
    .child(
      node(Color::None, cm)
        .line()
        .s("node 1_3")
        .end()
        .child(
          node(Color::Yellow, cm)
            .line()
            .s("node 1_3_1")
            .end()
            .child(
              leaf(cm)
                .line()
                .s("line 1_3_1_1")
                .end()
                .line()
                .s("line 1_3_1_2")
                .end()
                .line()
                .s("line 1_3_1_3")
                .end()
                .line()
                .s("line 1_3_1_4")
                .end()
                .end(),
            )
            .child(leaf(cm).line().s("only one line").end().end())
            .end(),
        )
        .child(
          leaf(cm)
            .line()
            .s("line 1_3_1")
            .end()
            .line()
            .s("line 1_3_2")
            .end()
            .line()
            .s("line 1_3_3")
            .end()
            .line()
            .s("line 1_3_4")
            .end()
            .end(),
        )
        .child(leaf(cm).line().s("only one line").end().end())
        .end(),
    )
    .end();

  assert_eq!(EXPECTED, format!("\n{}", root));
  assert_eq!(EXPECTED_INDENTED, format!("\n{:5}", root));
}
