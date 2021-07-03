use crate::common::*;

#[derive(Debug, Clone)]
pub struct NoteId {
  pub prefix: String,
  pub name:   String,
  pub ext:    String,
}

impl fmt::Display for NoteId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}-{}.{}", self.prefix, self.name, self.ext)
  }
}

impl NoteId {
  /// Generates a `NoteId` using the passed in `name` and a naive UTC
  /// datetime timestamp.
  pub fn new(name: &str, ext: &str) -> Self {
    let now = chrono::Utc::now();
    Self {
      name:   name.to_owned(),
      prefix: now.naive_utc().timestamp().to_string(),
      ext:    ext.to_owned(),
    }
  }

  /// Essentially just splits a filename on `-` and attempts to
  /// return a valid `NoteId` based on the resulting parts.
  ///
  /// A proper note name should be of the form
  /// `{prefix}-{name}.{extension}`.
  ///
  /// This method cuts off anything after the last `.` when considering a
  /// note id string.
  pub fn parse(filename: &str) -> Option<Self> {
    let ext = Path::new(filename)
      .extension()
      .and_then(OsStr::to_str)
      .unwrap_or("");

    let mut split =
      filename[..filename.rfind('.').unwrap_or_else(|| filename.len())].splitn(2, '-');

    Some(Self {
      prefix: split.next().unwrap_or("").to_owned(),
      name:   split.next().unwrap_or("").to_owned(),
      ext:    ext.to_owned(),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse() {
    let cases = vec![
      ("123-a", "123", "a", ""),
      ("123-a.md", "123", "a", "md"),
      ("abc123-", "abc123", "", ""),
      ("", "", "", ""),
      ("abc123", "abc123", "", ""),
      ("123292.md", "123292", "", "md"),
      ("123292-binary-search.md", "123292", "binary-search", "md"),
      ("123.292-binary-search.md", "123.292", "binary-search", "md"),
    ];

    for case in cases {
      let (test, prefix, name, ext) = case;
      let id = NoteId::parse(test).unwrap();
      assert_eq!(id.prefix, prefix);
      assert_eq!(id.name, name);
      assert_eq!(id.ext, ext);
    }
  }
}
