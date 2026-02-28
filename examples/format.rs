use antex::{StyledText, Text, auto};

fn display(plain_text: &str, styled_text: &Text, width: usize) {
  println!("[{}]", plain_text);
  println!("|{}|", styled_text);

  println!("[{:w$}]", plain_text, w = width);
  println!("|{:w$}|", styled_text, w = width);

  println!("[{:<w$}]", plain_text, w = width);
  println!("|{:<w$}|", styled_text, w = width);

  println!("[{:>w$}]", plain_text, w = width);
  println!("|{:>w$}|", styled_text, w = width);

  println!("[{:^w$}]", plain_text, w = width);
  println!("|{:^w$}|", styled_text, w = width);

  println!("[{:-<w$}]", plain_text, w = width);
  println!("|{:-<w$}|", styled_text, w = width);

  println!("[{:->w$}]", plain_text, w = width);
  println!("|{:->w$}|", styled_text, w = width);

  println!("[{:-^w$}]", plain_text, w = width);
  println!("|{:-^w$}|", styled_text, w = width);
}

fn main() {
  let plain_text = "Hello ☺ world 1.999";
  let styled_text = auto().blue().s("Hello").reset().s(" ☺ ").yellow().s("world").s(" ").magenta().s(1.999).reset();
  display(plain_text, &styled_text, 60);
}
