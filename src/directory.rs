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
  /// convert each instance of a file with extension `ext` into a `Note`.
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
      .filter(|entry| {
        entry.is_file()
          && entry
            .extension()
            .unwrap_or_else(|| OsStr::new(""))
            .to_str()
            .unwrap()
            == self.ext
      })
      .for_each(|entry| {
        notes.push(Note::new(entry).unwrap());
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
      .filter(|note| {
        note
          .matter
          .tags
          .clone()
          .unwrap_or_default()
          .contains(&tag.to_string())
      })
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
      .filter(|note| {
        note
          .matter
          .links
          .clone()
          .unwrap_or_default()
          .contains(&name.to_string())
      })
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_utils::{create, sleep};

  #[test]
  fn test_notes() {
    in_temp_dir!({
      create(&NoteId::new("a", "md")).unwrap();
      create(&NoteId::new("b", "md")).unwrap();
      create(&NoteId::new("c", "md")).unwrap();

      let directory = Directory::new(env::current_dir().unwrap(), String::from("md"));
      let notes = directory.notes().unwrap();

      assert_eq!(notes.len(), 3);
    });
  }

  #[test]
  fn test_find() {
    in_temp_dir!({
      // create 5 notes with name `a`
      for _ in 0..5 {
        create(&NoteId::new("a", "md")).unwrap();
        sleep();
      }

      let directory = Directory::new(env::current_dir().unwrap(), String::from("md"));
      let notes = directory.find("a").unwrap();

      assert_eq!(notes.len(), 5);

      for note in notes {
        assert_eq!(note.id.name, "a");
      }
    });
  }

  #[test]
  fn test_find_by_tag() {
    in_temp_dir!({
      let a = create(&NoteId::new("a", "md")).unwrap();
      let b = create(&NoteId::new("b", "md")).unwrap();

      a.add_tag("software").unwrap();
      b.add_tag("software").unwrap();

      let directory = Directory::new(env::current_dir().unwrap(), String::from("md"));
      let notes = directory.find_by_tag("software").unwrap();

      assert_eq!(notes.len(), 2);

      for note in notes {
        assert!(note.has_tag("software"));
      }
    });
  }
}
