use crate::common::*;

#[derive(Debug)]
pub struct Directory {
  pub path: PathBuf,
  pub ext:  String,
}

impl Directory {
  pub fn new(path: PathBuf, ext: String) -> Self {
    Self { path, ext }
  }

  /// Constructs a `Vec<Note>` based on a the directories path. This attempts to
  /// convert each instance of a markdown file into a `Note`.
  pub fn notes(&self) -> Result<Vec<Note>, Error> {
    let mut notes = Vec::new();

    if !&self.path.exists() {
      return Err(error::Error::PathError {
        path: self.path.to_owned(),
      });
    }

    WalkDir::new(&self.path)
      .into_iter()
      .map(|entry| entry.unwrap().into_path())
      .filter(|entry| entry.is_file() && entry.extension().unwrap().to_str().unwrap() == self.ext)
      .for_each(|entry| {
        notes.push(Note::new(entry));
      });

    Ok(notes)
  }

  /// Finds all notes that reside within this directories `path` whose name
  /// matches `name`. This method either returns a list of `Note` instances
  /// who meet this criteria or `None`, indicating that the criteria was
  /// not met.
  pub fn find(&self, name: &str) -> Result<Vec<Note>, Error> {
    let ret = &self
      .notes()?
      .iter()
      .filter(|note| note.id.name == name)
      .cloned()
      .collect::<Vec<Note>>();

    match ret.len() {
      0 => Err(error::Error::NoteNotFound {
        name: name.to_owned(),
      }),
      _ => Ok(ret.to_vec()),
    }
  }

  /// Finds all notes that reside within this directories `path` whose
  /// list of tags contains the value `tag`. This method either returns a list
  /// of `Note` instances who meet this criteria or `None`, indicating
  /// that the criteria was not met.
  pub fn find_by_tag(&self, tag: &str) -> Result<Vec<Note>, Error> {
    let ret = &self
      .notes()?
      .iter()
      .filter(|note| note.matter.tags.contains(&tag.to_string()))
      .cloned()
      .collect::<Vec<Note>>();

    match ret.len() {
      0 => Err(error::Error::TagNotFound {
        tag: tag.to_owned(),
      }),
      _ => Ok(ret.to_vec()),
    }
  }

  /// Finds all notes that reside within this directories `path` whose
  /// list of links contains the value `name`. This method either returns a list
  /// of `Note` instances who meet this criteria or `None`, indicating
  /// that the criteria was not met.
  #[allow(dead_code)]
  pub fn find_by_link(&self, name: &str) -> Result<Vec<Note>, Error> {
    let ret = &self
      .notes()?
      .iter()
      .filter(|note| note.matter.links.contains(&name.to_string()))
      .cloned()
      .collect::<Vec<Note>>();

    match ret.len() {
      0 => Err(error::Error::LinkNotFound {
        name: name.to_owned(),
      }),
      _ => Ok(ret.to_vec()),
    }
  }
}
