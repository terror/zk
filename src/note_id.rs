use crate::common::*;

#[derive(Debug, Clone)]
pub struct NoteId {
  pub prefix: String,
  pub name:   String,
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

  /// Essentially just splits a note id string on `-` and attempts to
  /// returns a valid `NoteId` based on the resulting parts.
  ///
  /// A proper note name should be of the form
  /// `{prefix}-{name}.{extension}`.
  ///
  /// This method cuts off anything after the last `.` when considering a
  /// note id string.
  pub fn parse(note_id: &str) -> Option<Self> {
    let mut split = note_id[..note_id.rfind('.').unwrap_or_else(|| note_id.len())].splitn(2, '-');

    if let (Some(prefix), Some(name)) = (split.next(), split.next()) {
      return Some(Self {
        prefix: prefix.to_owned(),
        name:   name.to_owned(),
      });
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse() {
    assert_eq!(NoteId::parse("123-a").unwrap().prefix, "123");
    assert_eq!(NoteId::parse("123-a").unwrap().name, "a");
    assert_eq!(NoteId::parse("123-a.md").unwrap().prefix, "123");
    assert_eq!(NoteId::parse("123-a.md").unwrap().name, "a");
    assert_eq!(NoteId::parse("abc123-").unwrap().prefix, "abc123");
    assert_eq!(NoteId::parse("abc123-").unwrap().name, "");

    assert!(NoteId::parse("").is_none());
    assert!(NoteId::parse("abc123").is_none());
    assert!(NoteId::parse("123292.md").is_none());

    assert_eq!(
      NoteId::parse("123292-binary-search.md").unwrap().prefix,
      "123292"
    );

    assert_eq!(
      NoteId::parse("123292-binary-search.md").unwrap().name,
      "binary-search"
    );

    assert_eq!(
      NoteId::parse("123.292-binary-search.md").unwrap().prefix,
      "123.292"
    );

    assert_eq!(
      NoteId::parse("123.292-binary-search.md").unwrap().name,
      "binary-search"
    );
  }
}
