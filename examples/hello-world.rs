//! # Hello world example

use antex::{StyledText, Text};

fn main() {
  let greeting = Text::auto().yellow().s("Hello").reset().s(' ').green().s("world").reset().s('!');
  println!("{}", greeting);
}
