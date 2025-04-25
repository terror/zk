use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) struct NoteId {
  pub(crate) prefix: String,
  pub(crate) name: String,
}

impl Display for NoteId {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}-{}.md", self.prefix, self.name)
  }
}

impl NoteId {
  pub(crate) fn new(name: &str) -> Self {
    Self {
      prefix: chrono::Utc::now()
        .naive_utc()
        .and_utc()
        .timestamp()
        .to_string(),
      name: name.to_owned(),
    }
  }

  pub(crate) fn parse(filename: &str) -> Option<Self> {
    let mut split = filename[..filename.rfind('.').unwrap_or(filename.len())]
      .splitn(2, |c| ['-', ' '].contains(&c));

    Some(Self {
      prefix: split.next().unwrap_or("").to_owned(),
      name: split.next().unwrap_or("").to_owned(),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
    let cases = vec![
      ("a-b", "a", "b"),
      ("a-b.md", "a", "b"),
      ("a-", "a", ""),
      ("a", "a", ""),
      ("a.md", "a", ""),
      ("a-b-c.md", "a", "b-c"),
      ("a.b-c-d.md", "a.b", "c-d"),
      ("a b.md", "a", "b"),
      ("a b c.md", "a", "b c"),
      ("", "", ""),
    ];

    for case in cases {
      let (test, prefix, name) = case;
      let id = NoteId::parse(test).unwrap();
      assert_eq!(id.prefix, prefix);
      assert_eq!(id.name, name);
    }
  }
}
