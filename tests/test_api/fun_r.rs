use antex::{StyledText, always};

/// Calling `text.r(value)` is equivalent of calling `text.reset().s(value)`.
#[test]
fn _0001() {
  let expected = always().s("hello ").red().reset().s("world");
  let actual = always().s("hello ").red().r("world");
  assert_eq!(expected.to_string(), actual.to_string());
}
