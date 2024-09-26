mod colors;
mod text;
mod tree;

use colors::*;
use std::env;
use text::*;
use tree::*;

fn display_colors_8(cm: ColorMode) {
  println!("Foreground 8 colors:\n");
  Text::new(cm)
    .black()
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
    .clear()
    .println();
  Text::new(cm)
    .bold()
    .color(Color::Black)
    .s(" 0 ")
    .color(Color::Red)
    .s(" 1 ")
    .color(Color::Green)
    .s(" 2 ")
    .color(Color::Yellow)
    .s(" 3 ")
    .color(Color::Blue)
    .s(" 4 ")
    .color(Color::Magenta)
    .s(" 5 ")
    .color(Color::Cyan)
    .s(" 6 ")
    .color(Color::White)
    .s(" 7 ")
    .cprintln();
  println!();
}

fn display_background_colors_8() {
  println!("Background 8 colors:\n");
  let text = Text::default()
    .bg_black()
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
    .clear();
  println!("{}", text);
  println!("{}", Text::default().bold() + text);
  println!();
}

fn display_colors_256() {
  println!("Foreground 256 colors:\n");
  let mut text = Text::default();
  for i in 0..16 {
    for j in 0..16 {
      let code = i * 16 + j;
      text = text.color_256(code).s(format!(" {code:>3} "));
    }
    text = text.clear().nl()
  }
  println!("{}", text);
  println!("{}", Text::default().bold() + text);
  println!();
}

fn display_background_colors_256() {
  println!("Background 256 colors:\n");
  let mut text = Text::default();
  for i in 0..16 {
    for j in 0..16 {
      let code = i * 16 + j;
      text = text.bg_color_256(code).s(format!(" {code:>3} "));
    }
    text = text.clear().nl()
  }
  println!("{}", text);
  println!("{}", Text::default().bold() + text);
  println!();
}

fn display_some_text() {
  Text::default().s("Hello").space().red().s("world!").clear().print();

  Text::default().s("Hello ").red().s("world!").clear().println();

  Text::default()
    .s("Hello ")
    .color_256(69)
    .s("world!")
    .bold()
    .s("world!")
    .clear()
    .color_rgb((100, 230, 100))
    .bg_color_rgb((120, 120, 120))
    .s("again")
    .dot()
    .clear()
    .italic()
    .spaces(4)
    .s("and now some italic ")
    .clear()
    .underline()
    .s("and underlined")
    .nl()
    .cprint()
}

fn display_tree() {
  let cm = ColorMode::default();
  let root = node(Color::Yellow, cm)
    .line()
    .blue()
    .plural("My node", 4)
    .colon()
    .clear()
    .end()
    .child(
      node(Color::Long(124), cm)
        .line()
        .slash()
        .s("node 1")
        .end()
        .child(
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
        )
        .child(leaf(cm).line().s("only one line").end().end())
        .end(),
    )
    .child(
      node(Color::White, cm)
        .line()
        .s("node 2")
        .dots()
        .end()
        .child(leaf(cm).line().s("only one line").end().end())
        .child(
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
        )
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
        .child(leaf(cm).line().s("only one line").end().end())
        .end(),
    )
    .end();

  println!("{}", root);

  let mut buffer = String::new();
  let _ = root.write_indent(&mut buffer, 30);
  println!("{}", buffer);
}

fn display_all(cm: ColorMode) {
  display_colors_8(cm);
  display_background_colors_8();
  display_colors_256();
  display_background_colors_256();
  display_some_text();
  display_tree();
}

fn main() {
  let cm = ColorMode::On;
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    Text::default().red().s("Invalid number of arguments!").clear().println();
    return;
  }
  match args[1].to_lowercase().trim() {
    "1" => display_colors_8(cm),
    "2" => display_background_colors_8(),
    "3" => display_colors_256(),
    "4" => display_background_colors_256(),
    "5" => display_some_text(),
    "6" => display_tree(),
    "100" => display_all(cm),
    _ => {
      Text::default().red().s("Unknown command: ").clear().s(args[1].clone()).println();
    }
  }
}
