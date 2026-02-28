//! # Styling is always active
//!
//! Running this example in terminal will display colored text:
//!
//! ```shell
//! cargo run --example always
//! ```
//!
//! Redirecting the output to another process will preserve coloring:
//!
//! ```shell
//! cargo run --example always | cat
//! ```

use antex::{StyledText, Text};

fn main() {
  println!(
    "{}",
    Text::on()
      .s("Always colored:\n")
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
      .reset()
  );
}
