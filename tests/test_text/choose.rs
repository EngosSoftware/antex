use antex::{StyledText, never};

#[test]
fn if_else_should_work() {
  assert_eq!("1", format!("{}", never().choose(true, "1", "0")));
  assert_eq!("0", format!("{}", never().choose(false, "1", "0")));
}

#[test]
fn if_else_with_different_types_should_work() {
  assert_eq!("1", format!("{}", never().choose(true, 1, "0")));
  assert_eq!("0", format!("{}", never().choose(false, '1', 0)));
}
