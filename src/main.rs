mod colors;
mod text;
mod tree;

use colors::*;
use text::*;
use tree::*;

fn foreground_colors_8(cm: ColorMode) {
  println!("\nForeground 8 colors:\n");
  Text::new(cm)
    .color(Color::Black)
    .s(" 0 ")
    .red()
    .s(" 1 ")
    .green()
    .s(" 2 ")
    .yellow()
    .s(" 3 ")
    .blue()
    .s(" 4 ")
    .magenta()
    .s(" 5 ")
    .cyan()
    .s(" 6 ")
    .white()
    .s(" 7 ")
    .cprintln();
}

fn background_colors_8(cm: ColorMode) {
  println!("\nBackground 8 colors:\n");
  Text::new(cm)
    .bg_color(Color::Black)
    .s(" 0 ")
    .bg_red()
    .s(" 1 ")
    .bg_green()
    .s(" 2 ")
    .bg_yellow()
    .s(" 3 ")
    .bg_blue()
    .s(" 4 ")
    .bg_magenta()
    .s(" 5 ")
    .bg_cyan()
    .s(" 6 ")
    .bg_white()
    .s(" 7 ")
    .clear()
    .println();
}

fn foreground_colors_256(cm: ColorMode) {
  println!("\nForeground 256 colors:\n");
  let mut text = Text::new(cm);
  for i in 0..16 {
    for j in 0..16 {
      let code = i * 16 + j;
      text = text.color_256(code).s(format!(" {code:>3} "));
    }
    text = text.clear().nl()
  }
  text.cprintln();
}

fn background_colors_256(cm: ColorMode) {
  println!("\nBackground 256 colors:\n");
  let mut text = Text::new(cm);
  for i in 0..16 {
    for j in 0..16 {
      let code = i * 16 + j;
      text = text.bg_color_256(code).s(format!(" {code:>3} "));
    }
    text = text.clear().nl()
  }
  text.cprintln();
}

fn text_properties(cm: ColorMode) {
  Text::new(cm).s("    Colour: ").s("Hello").space().cyan().s("world!").cprintln();
  Text::new(cm).s("Background: ").bg_color(Color::Yellow).s("Hello").space().cyan().s("world!").cprintln();
  Text::new(cm).s("      Bold: ").bold().s("Hello ").cyan().s("world!").cprintln();
  Text::new(cm).s("Bgnd++Bold: ").bg_color(Color::Yellow).bold().s("Hello ").cyan().s("world!").cprintln();
  Text::new(cm).s("    Italic: ").italic().s("Hello").space().cyan().s("world!").cprintln();
  Text::new(cm).s("Underlined: ").underline().s("Hello").space().cyan().s("world!").cprintln();
  println!("Characters and new line:");
  Text::new(cm).dot().colon().slash().spaces(2).dots().print();
  Text::new(cm).nl().cprint();
}

fn tree(cm: ColorMode) -> TreeNode {
  let mut leaf_builder = leaf(cm);
  leaf_builder.add_line(Text::new(cm).s("only one line"));
  let last_leaf = leaf_builder.end();
  let mut node_builder = node(Color::Cyan, cm);
  node_builder.set_line(Text::new(cm).slash().s("Node 1"));
  node_builder.add_child(
    leaf(cm)
      .line()
      .s("line 1_1")
      .end()
      .line()
      .s("line 1_2")
      .end()
      .line()
      .s("line 1_3")
      .end()
      .line()
      .s("line 1_4")
      .end()
      .end(),
  );
  node_builder.add_opt_child(Some(leaf(cm).line().s("only one line").end().end()));
  let tree_node = node_builder.end();

  node(Color::Yellow, cm)
    .line()
    .blue()
    .plural("Nodes", 4)
    .colon()
    .clear()
    .end()
    .child(tree_node)
    .child(
      node(Color::White, cm)
        .line()
        .s("node 2")
        .dots()
        .end()
        .child(leaf(cm).line().s("only one line").end().end())
        .opt_child(Some(
          leaf(cm)
            .line()
            .s("line 2_1")
            .end()
            .line()
            .s("line 2_2")
            .end()
            .line()
            .s("line 2_3")
            .end()
            .line()
            .s("line 2_4")
            .end()
            .end(),
        ))
        .child(leaf(cm).line().s("only one line").end().end())
        .end(),
    )
    .child(
      node(Color::Magenta, cm)
        .line()
        .bg_color(Color::Blue)
        .s("node 3")
        .clear()
        .end()
        .child(
          node(Color::Green, cm)
            .line()
            .s("node 3_1")
            .end()
            .child(
              leaf(cm)
                .line()
                .s("line 3_1_1")
                .end()
                .line()
                .s("line 3_1_2")
                .end()
                .line()
                .s("line 3_1_3")
                .end()
                .line()
                .s("line 3_1_4")
                .end()
                .end(),
            )
            .child(leaf(cm).line().s("only one line").end().end())
            .end(),
        )
        .child(
          leaf(cm)
            .line()
            .s("line 3_1")
            .end()
            .line()
            .s("line 3_2")
            .end()
            .line()
            .s("line 3_3")
            .end()
            .line()
            .s("line 3_4")
            .end()
            .end(),
        )
        .child(last_leaf)
        .end(),
    )
    .end()
}

fn left_aligned_tree(cm: ColorMode) {
  println!("\nStyled tree:\n");
  let root = tree(cm);
  println!("{}", root);
}

fn indented_tree(cm: ColorMode) {
  println!("\nIndented styled tree:\n");
  let root = tree(cm);
  let mut buffer = String::new();
  let _ = root.write_indent(&mut buffer, 10);
  println!("{}", buffer);
}

fn main() {
  let cm = ColorMode::default();
  foreground_colors_8(cm);
  background_colors_8(cm);
  foreground_colors_256(cm);
  background_colors_256(cm);
  text_properties(cm);
  left_aligned_tree(cm);
  indented_tree(cm);
}
