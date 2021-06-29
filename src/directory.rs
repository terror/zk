use crate::common::*;

pub struct Directory {
  pub path: PathBuf,
}

impl Directory {
  /// Constructor
  pub fn new(path: PathBuf) -> Self {
    Self { path }
  }

  /// Constructs a `Vec<Note>` based on a the directories path. This attempts to
  /// convert each instance of a markdown file into a `Note`.
  pub fn notes(&self) -> Vec<Note> {
    let mut notes = Vec::new();

    for entry in WalkDir::new(&self.path) {
      let entry = entry.unwrap().into_path();
      if entry.is_file() && entry.extension().unwrap() == "md" {
        notes.push(Note::from(entry));
      }
    }

    notes
  }

  /// Finds all notes that reside within this directories `path` whose name
  /// matches `name`. This method either returns a list of `Note` instances
  /// who meet this criteria or `None`, indicating that the criteria was
  /// not met.
  pub fn find(&self, name: &str) -> Option<Vec<Note>> {
    let mut ret = Vec::new();

    for note in self.notes() {
      if note.id.name == name {
        ret.push(note);
      }
    }

    match ret.len() {
      0 => None,
      _ => Some(ret),
    }
  }

  /// Finds all notes that reside within this directories `path` whose
  /// list of tags contains the value `tag`. This method either returns a list
  /// of `Note` instances who meet this criteria or `None`, indicating
  /// that the criteria was not met.
  pub fn find_by_tag(&self, tag: &str) -> Option<Vec<Note>> {
    let mut ret = Vec::new();

    for note in self.notes() {
      if let Some(tags) = note.matter["tags"].as_vec() {
        if tags.contains(&Yaml::from_str(tag)) {
          ret.push(note);
        }
      }
    }

    match ret.len() {
      0 => None,
      _ => Some(ret),
    }
  }

  /// Finds all notes that reside within this directories `path` whose
  /// list of links contains the value `name`. This method either returns a list
  /// of `Note` instances who meet this criteria or `None`, indicating
  /// that the criteria was not met.
  #[allow(dead_code)]
  pub fn find_by_link(&self, name: &str) -> Option<Vec<Note>> {
    let mut ret = Vec::new();

    for note in self.notes() {
      if let Some(links) = note.matter["links"].as_vec() {
        if links.contains(&Yaml::from_str(name)) {
          ret.push(note);
        }
      }
    }

    match ret.len() {
      0 => None,
      _ => Some(ret),
    }
  }
}
