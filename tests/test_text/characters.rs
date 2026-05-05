use antex::{StyledText, Text};

#[test]
fn any_char_should_work() {
  let text = Text::default().s("Hello").s('^').s("world!");
  assert_eq!("Hello^world!", text.to_string());
  assert_eq!("Hello^world!", text.chars().collect::<String>());
}

#[test]
fn unicode_char_should_work() {
  let text = Text::default().s("Hello").s('☺').s("world!");
  assert_eq!("Hello☺world!", text.to_string());
  assert_eq!("Hello☺world!", text.chars().collect::<String>());
}

#[test]
fn repeat_should_work() {
  let text = Text::default().s("Hello").repeat('☺', 5).s("world!");
  assert_eq!("Hello☺☺☺☺☺world!", text.to_string());
  assert_eq!("Hello☺☺☺☺☺world!", text.chars().collect::<String>());
}

#[test]
fn colored_repeat_should_work() {
  let text = Text::default().blue().s("Hello").reset().green().repeat('☺', 5).reset().red().s("world!").reset();
  assert_eq!("Hello☺☺☺☺☺world!", text.chars().collect::<String>());
  assert_eq!(16, text.count());
}

#[test]
fn plural_should_work() {
  assert_eq!("Hello world!", Text::default().s("Hello ").plural("world", 1).s('!').to_string());
  assert_eq!("Hello worlds!", Text::default().s("Hello ").plural("world", 2).s('!').to_string());
}
