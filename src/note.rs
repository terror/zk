use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) struct Note {
  pub(crate) id: NoteId,
  pub(crate) path: PathBuf,
  pub(crate) matter: Matter,
  pub(crate) content: String,
}

impl SkimItem for Note {
  fn text(&self) -> Cow<str> {
    Cow::Owned(self.id.to_string())
  }

  fn preview(&self, _context: PreviewContext) -> ItemPreview {
    ItemPreview::Command(format!("cat \"{}\"", self.path.display()))
  }
}

impl Note {
  pub(crate) fn create(path: PathBuf) -> Result<Self> {
    let id =
      NoteId::parse(path.unwrapped_filename()).ok_or(Error::InvalidNoteId {
        id: path.unwrapped_filename().to_string(),
      })?;

    let mut file = File::create(&path)?;
    file.write_all(&Matter::default(&id.name)?)?;

    Note::from(path)
  }

  pub(crate) fn from(path: PathBuf) -> Result<Self> {
    let id =
      NoteId::parse(path.unwrapped_filename()).ok_or(Error::InvalidNoteId {
        id: path.unwrapped_filename().to_string(),
      })?;

    let (matter, content) =
      matter::matter(&fs::read_to_string(&path)?).unwrap_or_default();

    let matter = Matter::from(matter.as_str())?;

    Ok(Self {
      id,
      path,
      matter,
      content,
    })
  }

  pub(crate) fn has_link(&self, name: &str) -> bool {
    self
      .matter
      .links
      .to_owned()
      .unwrap_or_default()
      .contains(&name.to_string())
  }

  pub(crate) fn has_tag(&self, name: &str) -> bool {
    self
      .matter
      .tags
      .to_owned()
      .unwrap_or_default()
      .contains(&name.to_string())
  }

  pub(crate) fn add_link(&mut self, name: &str) -> Result<Self> {
    if self.has_link(name) {
      return Err(Error::LinkExists {
        link: name.to_string(),
      });
    }

    self.write(|note| {
      note
        .matter
        .links
        .get_or_insert(Vec::new())
        .push(name.to_string())
    })
  }

  pub(crate) fn remove_link(&mut self, name: &str) -> Result<Self> {
    if !self.has_link(name) {
      return Err(Error::LinkMissing {
        link: name.to_string(),
        name: self.id.to_string(),
      });
    }

    self.write(|note| {
      note
        .matter
        .links
        .get_or_insert(Vec::new())
        .retain(|link| link != name)
    })
  }

  pub(crate) fn add_tag(&mut self, name: &str) -> Result<Self> {
    if self.has_tag(name) {
      return Err(Error::TagExists {
        tag: name.to_string(),
      });
    }

    self.write(|note| {
      note
        .matter
        .tags
        .get_or_insert(Vec::new())
        .push(name.to_string())
    })
  }

  pub(crate) fn remove_tag(&mut self, name: &str) -> Result<Self> {
    if !self.has_tag(name) {
      return Err(Error::TagMissing {
        tag: name.to_string(),
        name: self.id.to_string(),
      });
    }

    self.write(|note| {
      note
        .matter
        .tags
        .get_or_insert(Vec::new())
        .retain(|tag| tag != name)
    })
  }

  pub(crate) fn remove(&self) -> Result<()> {
    Ok(fs::remove_file(&self.path)?)
  }

  fn write<F: Fn(&mut Note)>(&mut self, f: F) -> Result<Self> {
    f(self);
    let mut file = File::create(&self.path)?;
    file.write_all(Matter::into(self.matter.clone())?.as_bytes())?;
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
      let mut a = create_note("a").unwrap();

      let link = NoteId::new("b").to_string();

      a.add_link(&link).unwrap();

      assert!(a.has_link(&link));
    });
  }

  #[test]
  fn add_tag() {
    in_temp_dir!({
      let mut a = create_note("a").unwrap();

      a.add_tag("software").unwrap();
      assert!(a.has_tag("software"));
    });
  }

  #[test]
  fn remove_link() {
    in_temp_dir!({
      let mut a = create_note("a").unwrap();
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
      let mut a = create_note("a").unwrap();

      a.add_tag("software").unwrap();
      assert!(a.has_tag("software"));

      a.remove_tag("software").unwrap();
      assert!(!a.has_tag("software"));
    });
  }

  #[test]
  fn add_tag_existing() {
    in_temp_dir!({
      let mut a = create_note("a").unwrap();

      a.add_tag("software").unwrap();

      assert!(a.has_tag("software"));
      assert!(a.add_tag("software").is_err());
    });
  }

  #[test]
  fn add_link_existing() {
    in_temp_dir!({
      let mut a = create_note("a").unwrap();

      let link = NoteId::new("b").to_string();

      a.add_link(&link).unwrap();

      assert!(a.has_link(&link));
      assert!(a.add_link(&link).is_err());
    });
  }

  #[test]
  fn remove_tag_missing() {
    in_temp_dir!({
      let mut a = create_note("a").unwrap();
      assert!(a.remove_tag("software").is_err());
    });
  }

  #[test]
  fn remove_link_missing() {
    in_temp_dir!({
      let mut a = create_note("a").unwrap();
      assert!(a.remove_link("b").is_err());
    });
  }
}
