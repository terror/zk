use crate::common::*;

#[derive(Debug)]
pub struct Handler {
  pub config:    Config,
  pub directory: Directory,
}

impl Handler {
  pub fn new(config: Config, directory: Directory) -> Self {
    Self { config, directory }
  }

  /// Creates a new note with the specified `name` in the Zettelkasten directory
  /// with an appropriate prefix in addition to writing the default YAML
  /// frontmatter.
  pub fn create(&self, name: &str) -> Result<(), Error> {
    let id = NoteId::new(name);

    let mut file = File::create(&self.directory.path.join(id.to_string())).context(error::Io)?;

    file
      .write_all(format!("---\nname: {}\n---\n", name).as_bytes())
      .context(error::Io)?;

    self.open(name)?;

    Ok(())
  }

  /// Opens a note given a `name` using the editor specified in the
  /// configuration file. If there are multiple notes present with the
  /// same `name`, the user will be prompted with `skim` to choose
  /// which file(s) is/are desired to be opened.
  pub fn open(&self, name: &str) -> Result<(), Error> {
    let candidates = self.directory.find(name)?;

    // if there's only one candidate note, open it and return
    if candidates.len() == 1 {
      let filename = candidates.first().unwrap().id.to_string();
      Command::new(&self.config.editor)
        .arg(&self.directory.path.join(filename))
        .status()
        .context(error::Io)?;
      return Ok(());
    }

    // try to open all candidate notes
    for item in Search::new(candidates).run()? {
      if let Some(id) = NoteId::parse(&item.output().to_string()) {
        Command::new(&self.config.editor)
          .arg(&self.directory.path.join(&id.to_string()))
          .status()
          .context(error::Io)?;
      }
    }

    Ok(())
  }

  /// Links two notes together. This entails checking and modifying both notes'
  /// YAML frontmatter to ensure a link is created.
  ///
  /// Some things to consider:
  ///
  /// - Prompt the user if `left` or `right` exist more than once in the
  /// storage location
  ///
  /// - Check if `left` and `right` do not already contain each other in
  /// the yaml frontmatter
  pub fn link(&self, left: &str, right: &str) -> Result<(), Error> {
    let left = Note::new(
      self.directory.path.join(
        Search::new(self.directory.find(left)?)
          .run()?
          .first()
          .unwrap(),
      ),
    );

    let right = Note::new(
      self.directory.path.join(
        Search::new(self.directory.find(right)?)
          .run()?
          .first()
          .unwrap(),
      ),
    );

    left.add_link(&right.id.to_string())?;
    right.add_link(&left.id.to_string())?;

    println!("{}", format!("{} <-> {}", left.id, right.id).green());

    Ok(())
  }

  /// Finds all notes given a `tag`. This method invokes `skim` using the
  /// names of the notes that contain `tag` within the frontmatter and
  /// attempts to open each selected item.
  pub fn find(&self, tag: &str) -> Result<(), Error> {
    let candidates = self.directory.find_by_tag(tag)?;

    for item in Search::new(candidates).run()? {
      if let Some(id) = NoteId::parse(&item.output().to_string()) {
        Command::new(&self.config.editor)
          .arg(&self.directory.path.join(id.to_string()))
          .status()
          .context(error::Io)?;
      }
    }

    Ok(())
  }

  /// Starts a fuzzy search using note id's in the Zettelkasten directory
  /// Powered by `skim` --> https://github.com/lotabout/skim
  pub fn search(&self) -> Result<(), Error> {
    for item in Search::new(self.directory.notes()?).run()? {
      if let Some(id) = NoteId::parse(&item.output().to_string()) {
        Command::new(&self.config.editor)
          .arg(&self.directory.path.join(id.to_string()))
          .status()
          .context(error::Io)?;
      }
    }
    Ok(())
  }

  /// Writes the current Zettelkasten storage location to stdout.
  pub fn dir(&self) -> Result<(), Error> {
    println!("{}", self.directory.path.expand().display());
    Ok(())
  }

  /// Removes an existing note in the Zettelkasten directory. This will
  /// also prompt the user if more than one note exists with `name`.
  pub fn remove(&self, name: &str) -> Result<(), Error> {
    let candidates = self.directory.find(name)?;

    // if there's only one candidate note, delete it and return
    if candidates.len() == 1 {
      fs::remove_file(
        &self
          .directory
          .path
          .join(candidates.first().unwrap().id.to_string()),
      )
      .unwrap();
      return Ok(());
    }

    // delete each candidate note
    for selection in Search::new(candidates).run()? {
      fs::remove_file(&self.directory.path.join(&selection.to_string())).unwrap();
    }

    Ok(())
  }

  /// Removes a link between two existing notes
  pub fn remove_link(&self, left: &str, right: &str) -> Result<(), Error> {
    let left = Note::new(
      self.directory.path.join(
        Search::new(self.directory.find(left)?)
          .run()?
          .first()
          .unwrap(),
      ),
    );

    let right = Note::new(
      self.directory.path.join(
        Search::new(self.directory.find(right)?)
          .run()?
          .first()
          .unwrap(),
      ),
    );

    left.remove_link(&right.id.to_string())?;
    right.remove_link(&left.id.to_string())?;

    println!("{}", format!("{} <-X-> {}", left.id, right.id).green());

    Ok(())
  }

  pub fn tag(&self, name: &str, tag: &str) -> Result<(), Error> {
    let candidates = self.directory.find(name)?;

    // if there's only one candidate note, tag it and return
    if candidates.len() == 1 {
      let candidate = candidates.first().unwrap();
      candidate.add_tag(tag)?;
      return Ok(());
    }

    // tag each candidate note
    for item in Search::new(candidates).run()? {
      if let Some(id) = NoteId::parse(&item.output().to_string()) {
        let note = Note::new(self.directory.path.join(&id.to_string()).to_owned());
        note.add_tag(tag)?;
      }
    }

    Ok(())
  }

  pub fn remove_tag(&self, name: &str, tag: &str) -> Result<(), Error> {
    let candidates = self.directory.find(name)?;

    // if there's only one candidate note, tag it and return
    if candidates.len() == 1 {
      let candidate = candidates.first().unwrap();
      candidate.remove_tag(tag)?;
      return Ok(());
    }

    // remove the tag from each candidate note
    for item in Search::new(candidates).run()? {
      if let Some(id) = NoteId::parse(&item.output().to_string()) {
        let note = Note::new(self.directory.path.join(&id.to_string()).to_owned());
        note.remove_tag(tag)?;
      }
    }

    Ok(())
  }
}
