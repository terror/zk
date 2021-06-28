use crate::common::*;

pub struct Note {
  /// The notes prefix
  id: String,

  /// The notes absolute path
  path: String,

  /// A list of strings specified in the notes frontmatter
  tags: Vec<String>,

  /// A list of notes specified in the notes frontmatter
  links: Vec<Note>,
}

impl Note {
  fn new(id: String, path: String) -> Self {
    Self {
      id,
      path,
      tags: vec![],
      links: vec![],
    }
  }

  pub fn prefix() -> String {
    let now = chrono::Utc::now();
    now.naive_utc().timestamp().to_string()
  }
}
