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
  fn new(id: String, name: String, path: PathBuf, content: String, matter: Yaml) -> Self {
    Self {
      id,
      name,
      path,
      content,
      matter,
    }
  }

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
      }
      Part::Name => {
        if let Some(name) = split.first() {
          return Some(name);
        }
      }
    }

    None
  }

  /// Returns a new prefix (note id) concatenated with the given `name`.
  pub fn generate_name(name: &str) -> String {
    let now = chrono::Utc::now();
    format!("{}-{}.md", now.naive_utc().timestamp().to_string(), name)
  }

  /// Builds a Vec<Note> given a `path`. Each instance of a `.md` file
  /// will be parsed and turned into a `Note`.
  pub fn all(path: &PathBuf) -> Result<Vec<Note>, Error> {
    let mut notes = vec![];

    for entry in WalkDir::new(&path) {
      let entry = entry.unwrap().into_path();
      if entry.is_file() {
        notes.push(Note::from(entry));
      }
    }

    Ok(notes)
  }

  pub fn filename(&self) -> String {
    format!("{}-{}.md", self.id, self.name)
  }

  /// Writes to the notes `path` with current data on `self`. Used for
  /// updating a notes content & frontmatter.
  pub fn update(&self) {}
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
