use crate::common::*;

#[derive(Debug)]
pub(crate) struct Directory {
  pub(crate) path: PathBuf,
}

impl Directory {
  pub(crate) fn new(path: PathBuf) -> Self {
    Self { path }
  }

  /// Constructs a `Vec<Note>` based on a the directories path. This attempts to
  /// convert each instance of a file with extension `md` into a `Note`.
  pub fn notes(&self) -> Result<Vec<Note>> {
    WalkDir::new(&self.path)
      .into_iter()
      .collect::<Result<Vec<_>, _>>()?
      .iter()
      .cloned()
      .map(|entry| entry.into_path())
      .filter(|entry| entry.is_file() && entry.ext() == "md")
      .map(Note::from)
      .collect::<Result<Vec<_>, _>>()
  }

  /// Finds all notes that reside within this directories `path` whose name
  /// matches `name`. This method either returns a list of `Note` instances
  /// who meet this criteria or `None`, indicating that the criteria was
  /// not met.
  pub(crate) fn find(&self, name: &str) -> Result<Vec<Note>> {
    let ret = &self
      .notes()?
      .iter()
      .filter(|note| note.id.name == name)
      .cloned()
      .collect::<Vec<Note>>();

    match ret.len() {
      0 => Err(Error::NoteNotFound {
        name: name.to_owned(),
      }),
      _ => Ok(ret.to_vec()),
    }
  }

  /// Finds all notes that reside within this directories `path` whose
  /// list of tags contains the value `tag`. This method either returns a list
  /// of `Note` instances who meet this criteria or `None`, indicating
  /// that the criteria was not met.
  pub(crate) fn find_by_tag(&self, tag: &str) -> Result<Vec<Note>> {
    let ret = &self
      .notes()?
      .iter()
      .filter(|note| {
        note
          .matter
          .tags
          .to_owned()
          .unwrap_or_default()
          .contains(&tag.to_string())
      })
      .cloned()
      .collect::<Vec<Note>>();

    match ret.len() {
      0 => Err(Error::TagNotFound {
        tag: tag.to_owned(),
      }),
      _ => Ok(ret.to_vec()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn notes() {
    in_temp_dir!({
      create_note("a").unwrap();
      create_note("b").unwrap();
      create_note("c").unwrap();

      assert_eq!(
        Directory::new(env::current_dir().unwrap())
          .notes()
          .unwrap()
          .len(),
        3
      );
    });
  }

  #[test]
  fn find() {
    in_temp_dir!({
      for _ in 0..5 {
        create_note("a").unwrap();
        sleep();
      }

      let notes = Directory::new(env::current_dir().unwrap())
        .find("a")
        .unwrap();

      assert_eq!(notes.len(), 5);

      for note in notes {
        assert_eq!(note.id.name, "a");
      }
    });
  }

  #[test]
  fn find_by_tag() {
    in_temp_dir!({
      let mut a = create_note("a").unwrap();
      let mut b = create_note("b").unwrap();

      a.add_tag("software").unwrap();
      b.add_tag("software").unwrap();

      let notes = Directory::new(env::current_dir().unwrap())
        .find_by_tag("software")
        .unwrap();

      assert_eq!(notes.len(), 2);

      for note in notes {
        assert!(note.has_tag("software"));
      }
    });
  }
}
