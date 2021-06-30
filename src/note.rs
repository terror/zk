use crate::common::*;

#[derive(Debug, Clone)]
pub struct Note {
  /// The notes timestamp prefix and name.
  pub id: NoteId,

  /// Where the note is currently stored.
  pub path: PathBuf,

  /// Yaml frontmatter.
  pub matter: Matter,

  /// The notes content as a String.
  pub content: String,
}

impl Note {
  pub fn new(path: PathBuf) -> Self {
    let id = NoteId::parse(path.file_name().unwrap().to_str().unwrap()).unwrap();

    let (matter, content) = matter::matter(&fs::read_to_string(&path).unwrap()).unwrap();

    let matter = Matter::from(matter.as_str());

    Self {
      id,
      path,
      content,
      matter,
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
  pub fn add_link(&self, name: &str) -> Result<(), Error> {
    if self.has_link(name) {
      println!(
        "{}",
        format!(
          "Note: `{}` already contains a link to `{}`",
          self.id.name, name
        )
        .red()
      );
      return Ok(());
    }

    let mut new = self
      .matter
      .links
      .iter()
      .map(|link| link.to_owned())
      .collect::<Vec<String>>();

    new.push(name.to_string());

    let mut file = File::create(&self.path).unwrap();

    file
      .write_all(
        &Matter::to_string(Matter {
          links: new,
          ..self.matter.clone()
        })
        .as_bytes(),
      )
      .unwrap();

    file.write_all(&self.content.as_bytes()).unwrap();

    Ok(())
  }

  /// Attempts to remove `name` as a link from the current note.
  pub fn remove_link(&self, name: &str) -> Result<(), Error> {
    if !self.has_link(name) {
      println!(
        "{}",
        format!(
          "Note: `{}` already does not contain a link to `{}`",
          self.id.name, name
        )
        .red()
      );
      return Ok(());
    }

    let new = self
      .matter
      .links
      .iter()
      .filter(|link| *link != name)
      .map(|link| link.to_owned())
      .collect::<Vec<String>>();

    let mut file = File::create(&self.path).unwrap();

    file
      .write_all(
        &Matter::to_string(Matter {
          links: new,
          ..self.matter.clone()
        })
        .as_bytes(),
      )
      .unwrap();

    file.write_all(&self.content.as_bytes()).unwrap();

    Ok(())
  }

  /// Attempts to add `name` as a tag to the current note.
  pub fn add_tag(&self, name: &str) -> Result<(), Error> {
    if self.has_tag(name) {
      println!(
        "{}",
        format!(
          "Note `{}` already contains the tag `{}`",
          self.id.name, name
        )
        .red()
      );
      return Ok(());
    }

    let mut new = self
      .matter
      .tags
      .iter()
      .map(|tag| tag.to_owned())
      .collect::<Vec<String>>();

    new.push(name.to_string());

    let mut file = File::create(&self.path).unwrap();

    file
      .write_all(
        &Matter::to_string(Matter {
          tags: new,
          ..self.matter.clone()
        })
        .as_bytes(),
      )
      .unwrap();

    file.write_all(&self.content.as_bytes()).unwrap();

    Ok(())
  }

  /// Attempts to remove `name` as a tag from the current note.
  pub fn remove_tag(&self, name: &str) -> Result<(), Error> {
    if !self.has_tag(name) {
      println!(
        "{}",
        format!(
          "Note: `{}` already does not contain the tag `{}`",
          self.id.name, name
        )
        .red()
      );
      return Ok(());
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
        &Matter::to_string(Matter {
          tags: new,
          ..self.matter.clone()
        })
        .as_bytes(),
      )
      .unwrap();

    file.write_all(&self.content.as_bytes()).unwrap();

    Ok(())
  }
}
