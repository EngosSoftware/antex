use antex::{Color, StyledText, Text, auto};

const LABELS: [&str; 8] = [" whole ", " error ", " compiling ", " warning ", " plan ", " finished ", " before ", " completion "];

const COLORS: [Color; 8] = [
  Color::Black,
  Color::Red,
  Color::Green,
  Color::Yellow,
  Color::Blue,
  Color::Magenta,
  Color::Cyan,
  Color::White,
];

fn bold() -> Text {
  auto().bold()
}

fn reset() -> Text {
  auto().reset()
}

fn print_colors(normal: Text, bright: Text) {
  println!("         normal: {}", normal.clone() + reset());
  println!("         bright: {}", bright.clone() + reset());
  println!("           bold: {}", bold() + normal + reset());
  println!("  bold & bright: {}", bold() + bright + reset());
}

fn print_bg_colors(normal: Text, bright: Text) {
  println!("         normal: {}", normal + reset());
  println!("         bright: {}", bright + reset());
}

fn basic_colors() {
  let normal_colors = auto()
    .black()
    .s(LABELS[0])
    .red()
    .s(LABELS[1])
    .green()
    .s(LABELS[2])
    .yellow()
    .s(LABELS[3])
    .blue()
    .s(LABELS[4])
    .magenta()
    .s(LABELS[5])
    .cyan()
    .s(LABELS[6])
    .white()
    .s(LABELS[7]);
  let bright_colors = auto()
    .bright_black()
    .s(LABELS[0])
    .bright_red()
    .s(LABELS[1])
    .bright_green()
    .s(LABELS[2])
    .bright_yellow()
    .s(LABELS[3])
    .bright_blue()
    .s(LABELS[4])
    .bright_magenta()
    .s(LABELS[5])
    .bright_cyan()
    .s(LABELS[6])
    .bright_white()
    .s(LABELS[7]);
  print_colors(normal_colors, bright_colors);
}

fn enumerated_colors() {
  let mut normal_colors = auto();
  for (index, color) in COLORS.iter().enumerate() {
    normal_colors += auto().color(*color).s(LABELS[index]);
  }
  let mut bright_colors = auto();
  for (index, color) in COLORS.iter().enumerate() {
    bright_colors += auto().bright_color(*color).s(LABELS[index]);
  }
  print_colors(normal_colors, bright_colors);
}

fn indexed_colors() {
  let mut normal_colors = auto();
  for (index, label) in LABELS.iter().enumerate() {
    normal_colors += auto().color_8(index as u8).s(label);
  }
  let mut bright_colors = auto();
  for (index, label) in LABELS.iter().enumerate() {
    bright_colors += auto().bright_color_8(index as u8).s(label);
  }
  print_colors(normal_colors, bright_colors);
}

fn basic_bg_colors() {
  let normal_colors = auto()
    .white()
    .bg_black()
    .s(LABELS[0])
    .bg_red()
    .s(LABELS[1])
    .bg_green()
    .s(LABELS[2])
    .bg_yellow()
    .s(LABELS[3])
    .bg_blue()
    .s(LABELS[4])
    .bg_magenta()
    .s(LABELS[5])
    .bg_cyan()
    .s(LABELS[6])
    .bg_white()
    .s(LABELS[7]);
  let bright_colors = auto()
    .bg_bright_black()
    .s(LABELS[0])
    .bg_bright_red()
    .s(LABELS[1])
    .bg_bright_green()
    .s(LABELS[2])
    .bg_bright_yellow()
    .s(LABELS[3])
    .bg_bright_blue()
    .s(LABELS[4])
    .bg_bright_magenta()
    .s(LABELS[5])
    .bg_bright_cyan()
    .s(LABELS[6])
    .bg_bright_white()
    .s(LABELS[7]);
  print_bg_colors(normal_colors, bright_colors);
}

fn main() {
  println!("\nColors set using color functions");
  println!("---------------------------------------------------------------------------------------");
  basic_colors();
  println!("\nColors set using color enumeration");
  println!("---------------------------------------------------------------------------------------");
  enumerated_colors();
  println!("\nColors set using color indexes");
  println!("---------------------------------------------------------------------------------------");
  indexed_colors();
  println!("\nBackground colors set using color functions");
  println!("---------------------------------------------------------------------------------------");
  basic_bg_colors();
  println!();
}
