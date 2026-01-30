//! # Color switching example

use antex::{ColorMode, StyledText, Text};

fn main() {
  let args = std::env::args().collect::<Vec<String>>();
  let color_mode = if args.len() != 2 { ColorMode::default() } else { ColorMode::new(&args[1]) };
  println!(
    "{}",
    Text::new(color_mode)
      .s("Switching colors:\n")
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
  );
}
