use crate::common::*;

#[derive(Debug)]
pub struct Note {
  /// The notes timestamp prefix.
  pub id: String,

  /// The notes name.
  pub name: String,

  /// Where the note is currently stored.
  pub path: PathBuf,

  /// The notes content as a String.
  pub content: String,

  /// Yaml frontmatter
  pub matter: Yaml,
}

/// The important thing here is to fully create a `Note` based on the
/// filename and contents of the `.md` file.
impl From<PathBuf> for Note {
  fn from(path: PathBuf) -> Self {
    let filename = path.file_name().unwrap().to_str().unwrap();

    let content = fs::read_to_string(&path).unwrap();

    let docs = YamlLoader::load_from_str(&content).unwrap();

    let matter = docs[0].clone();

    Self {
      id: Note::retrieve(filename, Part::Id).unwrap().to_string(),
      name: Note::retrieve(filename, Part::Name).unwrap().to_string(),
      path,
      content,
      matter,
    }
  }
}

impl Note {
  /// Returns the `name` or `prefix` of a note given a filename based on
  /// a passed in `Part`.
  /// Examples:
  /// `123-a.md` -> `a`
  /// `123-a.md` -> `123`
  pub fn retrieve(filename: &str, part: Part) -> Option<&str> {
    if filename == "" {
      return None;
    }

    let filename = Path::new(filename).file_stem().unwrap().to_str().unwrap();

    let mut split: Vec<&str> = filename.rsplitn(2, '-').collect();

    match part {
      Part::Id => {
        split.reverse();
        if let Some(id) = split.first() {
          return Some(id);
        }
      },
      Part::Name =>
        if let Some(name) = split.first() {
          return Some(name);
        },
    }

    None
  }

  /// Returns a new prefix (note id) concatenated with the given `name`.
  pub fn generate_name(name: &str) -> String {
    let now = chrono::Utc::now();
    format!("{}-{}.md", now.naive_utc().timestamp().to_string(), name)
  }

  /// Returns the notes filename using a `.md` extension with it's `id`
  /// and `name`.
  pub fn filename(&self) -> String {
    format!("{}-{}.md", self.id, self.name)
  }

  /// Checks if a link exists between the current note and `name`.
  pub fn has_link(&self, name: &str) -> bool {
    if let Some(links) = self.matter["links"].as_vec() {
      for link in links {
        if *link == Yaml::from_str(name) {
          return true;
        }
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_retrieve_name() {
    assert_eq!(Note::retrieve("123-a.md", Part::Name).unwrap(), "a");
    assert_eq!(Note::retrieve("./123-a.md", Part::Name).unwrap(), "a");
    assert_eq!(Note::retrieve("./123--a.md", Part::Name).unwrap(), "a");
    assert!(Note::retrieve("", Part::Name).is_none());
  }

  #[test]
  fn test_retrieve_id() {
    assert_eq!(Note::retrieve("123-a.md", Part::Id).unwrap(), "123");
    assert_eq!(Note::retrieve("./123-a.md", Part::Id).unwrap(), "123");
    assert_eq!(Note::retrieve("./123--a.md", Part::Id).unwrap(), "123-");
    assert!(Note::retrieve("", Part::Name).is_none());
  }
}
