use antex::{StyledText, never};
use std::fmt::Write;

struct FailingWriter {
  count: usize,
  max: usize,
}

impl FailingWriter {
  fn new(max: usize) -> Self {
    Self { count: 1, max }
  }
}

impl Write for FailingWriter {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    for _ in s.chars() {
      if self.count >= self.max {
        return Err(std::fmt::Error);
      } else {
        self.count += 1;
      }
    }
    Ok(())
  }
}

#[test]
fn _0001() {
  write!(&mut FailingWriter::new(1), "{}", never().s("ab")).unwrap_err();
  write!(&mut FailingWriter::new(1), "{:1}", never().s("ab")).unwrap_err();
  write!(&mut FailingWriter::new(4), "{:5}", never().s("ab")).unwrap_err();
  write!(&mut FailingWriter::new(1), "{:<1}", never().s("ab")).unwrap_err();
  write!(&mut FailingWriter::new(4), "{:<5}", never().s("ab")).unwrap_err();
  write!(&mut FailingWriter::new(1), "{:>1}", never().s("ab")).unwrap_err();
  write!(&mut FailingWriter::new(1), "{:>5}", never().s("ab")).unwrap_err();
  write!(&mut FailingWriter::new(1), "{:^1}", never().s("ab")).unwrap_err();
  write!(&mut FailingWriter::new(1), "{:^5}", never().s("ab")).unwrap_err();
  write!(&mut FailingWriter::new(4), "{:^5}", never().s("ab")).unwrap_err();
}
