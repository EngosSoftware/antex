use antex::{StyledText, Text};

#[test]
fn nl_should_work() {
  let text = Text::default().s("Hello").nl().s("world!");
  assert_eq!("Hello\nworld!", text.to_string());
}

#[test]
fn space_should_work() {
  let text = Text::default().s("Hello").space().s("world!");
  assert_eq!("Hello world!", text.to_string());
}

#[test]
fn spaces_should_work() {
  let text = Text::default().s("Hello").spaces(5).s("world!");
  assert_eq!("Hello     world!", text.to_string());
}

#[test]
fn dot_should_work() {
  let text = Text::default().s("Hello").dot().s("world!");
  assert_eq!("Hello.world!", text.to_string());
}

#[test]
fn colon_should_work() {
  let text = Text::default().s("Hello").colon().s("world!");
  assert_eq!("Hello:world!", text.to_string());
}

#[test]
fn slash_should_work() {
  let text = Text::default().s("Hello").slash().s("world!");
  assert_eq!("Hello/world!", text.to_string());
}

#[test]
fn backslash_should_work() {
  let text = Text::default().s("Hello").backslash().s("world!");
  assert_eq!("Hello\\world!", text.to_string());
}

#[test]
fn perc_should_work() {
  let text = Text::default().s("Hello").perc().s("world!");
  assert_eq!("Hello%world!", text.to_string());
}

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
  assert_eq!("Hello world!", Text::default().s("Hello").space().plural("world", 1).s('!').to_string());
  assert_eq!("Hello worlds!", Text::default().s("Hello").space().plural("world", 2).s('!').to_string());
}
