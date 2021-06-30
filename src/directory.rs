use crate::common::*;

#[derive(Debug)]
pub struct Directory {
  pub path: PathBuf,
}

impl Directory {
  pub fn new(path: PathBuf) -> Self {
    Self { path }
  }

  /// Constructs a `Vec<Note>` based on a the directories path. This attempts to
  /// convert each instance of a markdown file into a `Note`.
  pub fn notes(&self) -> Vec<Note> {
    let mut notes = Vec::new();

    WalkDir::new(&self.path)
      .into_iter()
      .map(|entry| entry.unwrap().into_path())
      .filter(|entry| entry.is_file() && entry.extension().unwrap() == "md")
      .for_each(|entry| {
        notes.push(Note::new(entry));
      });

    notes
  }

  /// Finds all notes that reside within this directories `path` whose name
  /// matches `name`. This method either returns a list of `Note` instances
  /// who meet this criteria or `None`, indicating that the criteria was
  /// not met.
  pub fn find(&self, name: &str) -> Option<Vec<Note>> {
    let ret = &self
      .notes()
      .iter()
      .filter(|note| note.id.name == name)
      .map(|note| note.clone())
      .collect::<Vec<Note>>();

    match ret.len() {
      0 => None,
      _ => Some(ret.to_vec()),
    }
  }

  /// Finds all notes that reside within this directories `path` whose
  /// list of tags contains the value `tag`. This method either returns a list
  /// of `Note` instances who meet this criteria or `None`, indicating
  /// that the criteria was not met.
  pub fn find_by_tag(&self, tag: &str) -> Option<Vec<Note>> {
    let ret = &self
      .notes()
      .iter()
      .filter(|note| note.matter.tags.contains(&tag.to_string()))
      .map(|note| note.clone())
      .collect::<Vec<Note>>();

    match ret.len() {
      0 => None,
      _ => Some(ret.to_vec()),
    }
  }

  /// Finds all notes that reside within this directories `path` whose
  /// list of links contains the value `name`. This method either returns a list
  /// of `Note` instances who meet this criteria or `None`, indicating
  /// that the criteria was not met.
  #[allow(dead_code)]
  pub fn find_by_link(&self, name: &str) -> Option<Vec<Note>> {
    let ret = &self
      .notes()
      .iter()
      .filter(|note| note.matter.links.contains(&name.to_string()))
      .map(|note| note.clone())
      .collect::<Vec<Note>>();

    match ret.len() {
      0 => None,
      _ => Some(ret.to_vec()),
    }
  }
}
