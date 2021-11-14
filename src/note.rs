use crate::common::*;

#[derive(Debug, Clone)]
pub struct Note {
  /// The notes timestamp prefix and name.
  pub id:      NoteId,
  /// Where the note is currently stored.
  pub path:    PathBuf,
  /// Yaml frontmatter.
  pub matter:  Matter,
  /// The notes content as a String.
  pub content: String,
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
  pub fn new(path: PathBuf) -> Self {
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
  pub fn has_link(&self, name: &str) -> bool {
    if self.matter.links.contains(&name.to_owned()) {
      return true;
    }
    false
  }

  /// Checks if a tag `name` exists within this notes tags.
  pub fn has_tag(&self, name: &str) -> bool {
    if self.matter.tags.contains(&name.to_owned()) {
      return true;
    }
    false
  }

  /// Attempts to add `name` as a link to the current note.
  pub fn add_link(&self, name: &str) -> Result<Note, Error> {
    if self.has_link(name) {
      println!(
        "{}",
        format!(
          "Note `{}` already contains a link to `{}`",
          self.id.name, name
        )
        .yellow()
      );
      return Ok(self.clone());
    }

    let mut new = self
      .matter
      .links
      .iter()
      .map(|link| link.to_owned())
      .collect::<Vec<String>>();

    new.push(name.to_string());

    let mut file = File::create(&self.path)?;

    file.write_all(
      Matter::into_string(Matter {
        links: new,
        ..self.matter.clone()
      })
      .as_bytes(),
    )?;

    file.write_all(self.content.as_bytes())?;

    Ok(Note::new(self.path.to_owned()))
  }

  /// Attempts to remove `name` as a link from the current note.
  pub fn remove_link(&self, name: &str) -> Result<Note, Error> {
    if !self.has_link(name) {
      println!(
        "{}",
        format!(
          "Note `{}` does not contain a link to `{}`",
          self.id.name, name
        )
        .yellow()
      );
      return Ok(self.clone());
    }

    let new = self
      .matter
      .links
      .iter()
      .filter(|link| *link != name)
      .map(|link| link.to_owned())
      .collect::<Vec<String>>();

    let mut file = File::create(&self.path)?;

    file.write_all(
      Matter::into_string(Matter {
        links: new,
        ..self.matter.clone()
      })
      .as_bytes(),
    )?;

    file.write_all(self.content.as_bytes())?;

    Ok(Note::new(self.path.to_owned()))
  }

  /// Attempts to add `name` as a tag to the current note.
  pub fn add_tag(&self, name: &str) -> Result<Note, Error> {
    if self.has_tag(name) {
      println!(
        "{}",
        format!(
          "Note `{}` already contains the tag `{}`.",
          self.id.name, name
        )
        .red()
      );
      return Ok(self.clone());
    }

    let mut new = self
      .matter
      .tags
      .iter()
      .map(|tag| tag.to_owned())
      .collect::<Vec<String>>();

    new.push(name.to_string());

    let mut file = File::create(&self.path)?;

    file.write_all(
      Matter::into_string(Matter {
        tags: new,
        ..self.matter.clone()
      })
      .as_bytes(),
    )?;

    file.write_all(self.content.as_bytes())?;

    Ok(Note::new(self.path.to_owned()))
  }

  /// Attempts to remove `name` as a tag from the current note.
  pub fn remove_tag(&self, name: &str) -> Result<Note, Error> {
    if !self.has_tag(name) {
      println!(
        "{}",
        format!(
          "Note `{}` does not contain the tag `{}`.",
          self.id.name, name
        )
        .red()
      );
      return Ok(self.clone());
    }

    let new = self
      .matter
      .tags
      .iter()
      .filter(|tag| *tag != name)
      .map(|tag| tag.to_owned())
      .collect::<Vec<String>>();

    let mut file = File::create(&self.path).unwrap();

    file
      .write_all(
        Matter::into_string(Matter {
          tags: new,
          ..self.matter.clone()
        })
        .as_bytes(),
      )
      .unwrap();

    file.write_all(self.content.as_bytes())?;

    Ok(Note::new(self.path.to_owned()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_link() {
    in_temp_dir!({
      let a = create_note(&NoteId::new("a", "md"));
      let link = NoteId::new("b", "md").to_string();

      let a = a.remove_link(&link).unwrap();
      assert!(!a.has_link(&link));

      let a = a.add_link(&link).unwrap();
      assert!(a.has_link(&link));
    });
  }

  #[test]
  fn test_add_tag() {
    in_temp_dir!({
      let a = create_note(&NoteId::new("a", "md"));

      let a = a.remove_tag("software").unwrap();
      assert!(!a.has_tag("software"));

      let a = a.add_tag("software").unwrap();
      assert!(a.has_tag("software"));
    });
  }

  #[test]
  fn test_remove_link() {
    in_temp_dir!({
      let a = create_note(&NoteId::new("a", "md"));
      let link = NoteId::new("b", "md").to_string();

      let a = a.remove_link(&link).unwrap();
      assert!(!a.has_link(&link));

      let a = a.add_link(&link).unwrap();
      assert!(a.has_link(&link));

      let a = a.remove_link(&link).unwrap();
      assert!(!a.has_link(&link));
    });
  }

  #[test]
  fn test_remove_tag() {
    in_temp_dir!({
      let a = create_note(&NoteId::new("a", "md"));

      let a = a.remove_tag("software").unwrap();
      assert!(!a.has_tag("software"));

      let a = a.add_tag("software").unwrap();
      assert!(a.has_tag("software"));

      let a = a.remove_tag("software").unwrap();
      assert!(!a.has_tag("software"));
    });
  }
}
