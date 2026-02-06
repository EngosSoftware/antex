use antex::{StyledText, Text};

#[test]
fn any_char_should_work() {
  let text = Text::default().s("Hello").s('^').s("world!");
  assert_eq!("Hello^world!", text.to_string());
}

#[test]
fn unicode_char_should_work() {
  let text = Text::default().s("Hello").s('☺').s("world!");
  assert_eq!("Hello☺world!", text.to_string());
}

#[test]
fn repeat_should_work() {
  let text = Text::default().s("Hello").repeat('☺', 5).s("world!");
  assert_eq!("Hello☺☺☺☺☺world!", text.to_string());
}

#[test]
fn plural_should_work() {
  assert_eq!("Hello world!", Text::default().s("Hello ").plural("world", 1).s('!').to_string());
  assert_eq!("Hello worlds!", Text::default().s("Hello ").plural("world", 2).s('!').to_string());
}
