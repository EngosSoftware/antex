use antex::{StyledText, Text};

#[test]
fn plural_should_work() {
  assert_eq!("Hello world!", Text::default().s("Hello ").plural("world", 1).s('!').to_string());
  assert_eq!("Hello worlds!", Text::default().s("Hello ").plural("world", 2).s('!').to_string());
}
