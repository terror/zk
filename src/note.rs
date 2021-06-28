use crate::common::*;

#[derive(Debug, Default)]
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

impl From<PathBuf> for Note {
  fn from(P: PathBuf) -> Self {
    Note::default()
  }
}

impl Note {
  fn new(id: String, path: String, tags: Vec<String>, links: Vec<Note>) -> Self {
    Self {
      id,
      path,
      tags,
      links,
    }
  }

  /// Returns a note prefix using a naive utc timestamp
  pub fn prefix() -> String {
    let now = chrono::Utc::now();
    now.naive_utc().timestamp().to_string()
  }

  /// Returns the `name` of a note given a filename.
  /// i.e `123-a.md` -> `a`
  pub fn name(filename: &str) -> Option<&str> {
    let filename = Path::new(filename).file_stem().unwrap().to_str().unwrap();

    let split: Vec<&str> = filename.rsplitn(2, '-').collect();

    if let Some(name) = split.first() {
      return Some(name);
    }

    None
  }

  /// Builds a Vec<Note> given a `path`. Each instance of a `.md` file
  /// will be parsed and turned into a `Note`.
  pub fn all(path: PathBuf) -> Result<Vec<Note>, Error> {
    Ok(vec![])
  }
}

#[cfg(test)]
mod tests {
  // use super::*;
}
