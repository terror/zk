use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) struct Note {
  /// The notes timestamp prefix and name.
  pub(crate) id:      NoteId,
  /// Where the note is currently stored.
  pub(crate) path:    PathBuf,
  /// Yaml frontmatter.
  pub(crate) matter:  Matter,
  /// The notes content as a String.
  pub(crate) content: String,
}

impl SkimItem for Note {
  fn text(&self) -> Cow<str> {
    Cow::Owned(self.id.to_string())
  }

  fn preview(&self, _context: PreviewContext) -> ItemPreview {
    ItemPreview::Command(format!("cat {}", self.path.display()))
  }
}

impl Note {
  pub(crate) fn new(path: PathBuf) -> Self {
    let id = NoteId::parse(path.file_name().unwrap().to_str().unwrap()).unwrap();

    let (matter, content) = matter::matter(&fs::read_to_string(&path).unwrap()).unwrap();

    let matter = Matter::from(matter.as_str());

    Self {
      id,
      path,
      matter,
      content,
    }
  }

  /// Checks if a link exists between the current note and `name`.
  pub(crate) fn has_link(&self, name: &str) -> bool {
    self.matter.links.contains(&name.to_string())
  }

  /// Checks if a tag `name` exists within this notes tags.
  pub(crate) fn has_tag(&self, name: &str) -> bool {
    self.matter.tags.contains(&name.to_string())
  }

  /// Attempts to add `name` as a link to the current note.
  pub(crate) fn add_link(&mut self, name: &str) -> Result<Self> {
    self.write(|note| note.matter.links.push(name.to_string()))
  }

  /// Attempts to remove `name` as a link from the current note.
  pub(crate) fn remove_link(&mut self, name: &str) -> Result<Self> {
    self.write(|note| note.matter.links.retain(|link| link != name))
  }

  /// Attempts to add `name` as a tag to the current note.
  pub(crate) fn add_tag(&mut self, name: &str) -> Result<Self> {
    self.write(|note| note.matter.tags.push(name.to_string()))
  }

  /// Attempts to remove `name` as a tag from the current note.
  pub(crate) fn remove_tag(&mut self, name: &str) -> Result<Self> {
    self.write(|note| note.matter.tags.retain(|tag| tag != name))
  }

  /// Remove this notes file on disk.
  pub(crate) fn remove(&self) -> Result<()> {
    Ok(fs::remove_file(&self.path)?)
  }

  /// Write this notes contents after performing an (optional) operation.
  fn write<F: Fn(&mut Note)>(&mut self, f: F) -> Result<Self> {
    f(self);
    let mut file = File::create(&self.path)?;
    file.write_all(Matter::into_string(self.matter.clone()).as_bytes())?;
    file.write_all(self.content.as_bytes())?;
    Ok(self.to_owned())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_link() {
    in_temp_dir!({
      let mut a = create_note(&NoteId::new("a"));
      let link = NoteId::new("b").to_string();

      a.add_link(&link).unwrap();
      assert!(a.has_link(&link));
    });
  }

  #[test]
  fn add_tag() {
    in_temp_dir!({
      let mut a = create_note(&NoteId::new("a"));

      a.add_tag("software").unwrap();
      assert!(a.has_tag("software"));
    });
  }

  #[test]
  fn remove_link() {
    in_temp_dir!({
      let mut a = create_note(&NoteId::new("a"));
      let link = NoteId::new("b").to_string();

      a.add_link(&link).unwrap();
      assert!(a.has_link(&link));

      a.remove_link(&link).unwrap();
      assert!(!a.has_link(&link));
    });
  }

  #[test]
  fn remove_tag() {
    in_temp_dir!({
      let mut a = create_note(&NoteId::new("a"));

      a.add_tag("software").unwrap();
      assert!(a.has_tag("software"));

      a.remove_tag("software").unwrap();
      assert!(!a.has_tag("software"));
    });
  }
}
