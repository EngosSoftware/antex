use antex::{StyledText, Text};

#[test]
fn any_char_should_work() {
  let text = Text::default().s("Hello").s('^').s("world!");
  assert_eq!("Hello^world!", text.to_string());
  assert_eq!("Hello^world!", text.chars().collect::<String>());
  assert_eq!("Hello^world!", text.characters());
  assert_eq!(12, text.count());
}

#[test]
fn unicode_char_should_work() {
  let text = Text::default().s("Hello").s('☺').s("world!");
  assert_eq!("Hello☺world!", text.to_string());
  assert_eq!("Hello☺world!", text.chars().collect::<String>());
  assert_eq!("Hello☺world!", text.characters());
  assert_eq!(12, text.count());
}

#[test]
fn repeat_should_work() {
  let text = Text::default().s("Hello").repeat('☺', 5).s("world!");
  assert_eq!("Hello☺☺☺☺☺world!", text.to_string());
  assert_eq!("Hello☺☺☺☺☺world!", text.chars().collect::<String>());
  assert_eq!("Hello☺☺☺☺☺world!", text.characters());
  assert_eq!(16, text.count());
}

#[test]
fn colored_repeat_should_work() {
  let text = Text::default().blue().s("Hello").reset().green().repeat('☺', 5).reset().red().s("world!").reset();
  assert_eq!("Hello☺☺☺☺☺world!", text.chars().collect::<String>());
  assert_eq!("Hello☺☺☺☺☺world!", text.characters());
  assert_eq!(16, text.count());
}
