//! # Basic example

use antex::{StyledText, Text};

fn main() {
  Text::default()
    .s("Foreground colors:\n")
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
    .printlnc();
}
