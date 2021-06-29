use crate::common::*;

#[derive(Debug)]
pub struct NoteId {
  pub name:   String,
  pub prefix: String,
}

impl fmt::Display for NoteId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}-{}.md", self.prefix, self.name)
  }
}

impl NoteId {
  /// Generates a `NoteId` using the passed in `name` and a naive UTC
  /// datetime timestamp.
  pub fn new(name: &str) -> Self {
    let now = chrono::Utc::now();
    Self {
      name:   name.to_owned(),
      prefix: now.naive_utc().timestamp().to_string(),
    }
  }
}
