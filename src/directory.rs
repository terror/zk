use crate::common::*;

pub struct Directory {
  pub path: PathBuf,
}

impl Directory {
  pub fn new(path: PathBuf) -> Self {
    Self { path }
  }

  /// Finds all notes that reside within `path` whose name matches `name`.
  pub fn find(&self, name: &str) -> Option<Vec<String>> {
    let mut ret = vec![];

    for note in self.fetch() {
      if note.name == name {
        ret.push(note.filename());
      }
    }

    if ret.is_empty() {
      return None;
    }

    Some(ret)
  }

  /// Converts all `.md` files that reside in `path` into Notes.
  pub fn fetch(&self) -> Vec<Note> {
    let mut notes = vec![];

    for entry in WalkDir::new(&self.path) {
      let entry = entry.unwrap().into_path();
      if entry.is_file() && entry.extension().unwrap() == "md" {
        notes.push(Note::from(entry));
      }
    }

    notes
  }
}
