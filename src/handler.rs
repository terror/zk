use crate::common::*;

pub struct Handler {
  pub config: Config,
}

impl Handler {
  fn path(&self) -> PathBuf {
    PathBuf::from(shellexpand::tilde(self.config.path.to_str().unwrap()).to_string())
  }

  /// Creates a new note with the specified `name` in the Zettelkasten directory with an
  /// appropriate prefix in addition to writing the default YAML frontmatter.
  pub fn new(&self, name: &str) -> Result<(), Error> {
    let prefix = Note::prefix();

    let mut file =
      File::create(&self.path().join(format!("{}-{}.md", prefix, name))).context(error::Io)?;

    file
      .write_all(format!("---\nname: {}\n---\n", name).as_bytes())
      .context(error::Io)?;

    self.open(name)?;

    Ok(())
  }

  /// Opens a note given a `name` using the editor specified in the
  /// configuration file. If there are multiple notes present with the
  /// same `name`, the user will be prompted interactively to choose
  /// which file is desired to be opened.
  pub fn open(&self, name: &str) -> Result<(), Error> {
    let path = self.path();
    let mut candidates = vec![];

    for entry in WalkDir::new(&path) {
      let entry = entry.unwrap();

      let filename = entry
        .path()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

      let split: Vec<&str> = filename.rsplitn(2, '-').collect();

      if let Some(candidate) = split.first() {
        if *candidate == name {
          candidates.push(format!("{}.md", filename));
        }
      }
    }

    if candidates.len() == 0 {
      println!("No note with name {} found.", name);
      return Ok(());
    }

    if candidates.len() == 1 {
      let filename = candidates.first().unwrap();
      Command::new(&self.config.editor)
        .arg(&path.join(filename))
        .status()
        .context(error::Io)?;
      return Ok(());
    }

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!("There exist multiple notes with the name `{}`, please choose which one you would like to open:", name))
        .items(&candidates[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
      println!("No note selected.");
      return Ok(());
    }

    for selection in selections {
      let filename = &candidates[selection];
      let path = Path::join(&self.path(), Path::new(&filename.to_string()));
      Command::new(&self.config.editor)
        .arg(&path)
        .status()
        .context(error::Io)?;
    }

    Ok(())
  }

  /// Links two notes together. This entails checking and modifying both notes'
  /// YAML frontmatter to ensure a link is created.
  pub fn link(&self, _left: &str, _right: &str) -> Result<(), Error> {
    Ok(())
  }

  /// Finds all notes given a `tag`.
  pub fn find(&self, _tag: &str) -> Result<(), Error> {
    Ok(())
  }

  /// Starts a fuzzy search in the notes directory. Powered by FZF.
  pub fn search(&self) -> Result<(), Error> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  // use super::*;
}
