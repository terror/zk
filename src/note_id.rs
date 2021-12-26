use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) struct NoteId {
  pub(crate) prefix: String,
  pub(crate) name:   String,
  pub(crate) ext:    String,
}

impl Display for NoteId {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}-{}.{}", self.prefix, self.name, self.ext)
  }
}

impl NoteId {
  /// Generates a `NoteId` using the passed in `name` and a naive UTC
  /// datetime timestamp.
  pub(crate) fn new(name: &str) -> Self {
    Self {
      name:   name.to_owned(),
      prefix: chrono::Utc::now().naive_utc().timestamp().to_string(),
      ext:    "md".to_owned(),
    }
  }

  /// Splits a filename on `-` and attempts to
  /// return a valid `NoteId` based on the resulting parts.
  pub(crate) fn parse(filename: &str) -> Option<Self> {
    let path = PathBuf::from(filename);

    let mut split =
      filename[..filename.rfind('.').unwrap_or_else(|| filename.len())].splitn(2, '-');

    Some(Self {
      prefix: split.next().unwrap_or("").to_owned(),
      name:   split.next().unwrap_or("").to_owned(),
      ext:    path.ext().to_owned(),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
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
