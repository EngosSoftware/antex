use antex::{StyledText, never};

#[test]
fn matches_should_work() {
  assert_eq!("a", format!("{}", never().matches([(true, "a"), (true, "b"), (true, "c")])));
  assert_eq!("a", format!("{}", never().matches([(true, "a"), (true, "b"), (false, "c")])));
  assert_eq!("a", format!("{}", never().matches([(true, "a"), (false, "b"), (true, "c")])));
  assert_eq!("a", format!("{}", never().matches([(true, "a"), (false, "b"), (false, "c")])));
  assert_eq!("b", format!("{}", never().matches([(false, "a"), (true, "b"), (true, "c")])));
  assert_eq!("b", format!("{}", never().matches([(false, "a"), (true, "b"), (false, "c")])));
  assert_eq!("c", format!("{}", never().matches([(false, "a"), (false, "b"), (true, "c")])));
  assert_eq!("", format!("{}", never().matches([(false, "a"), (false, "b"), (false, "c")])));
}
