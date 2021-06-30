use crate::common::*;

pub struct Handler {
  pub editor:    String,
  pub directory: Directory,
}

impl Handler {
  pub fn new(editor: String, directory: Directory) -> Self {
    Self { editor, directory }
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

    println!(
      "{}",
      format!("Success! Note with filename `{}` created.", id).green()
    );

    self.open(name)?;

    Ok(())
  }

  /// Opens a note given a `name` using the editor specified in the
  /// configuration file. If there are multiple notes present with the
  /// same `name`, the user will be prompted with `skim` to choose
  /// which file(s) is/are desired to be opened.
  pub fn open(&self, name: &str) -> Result<(), Error> {
    if let Some(candidates) = self.directory.find(name) {
      // if there's only one candidate note, open it and return
      if candidates.len() == 1 {
        let filename = candidates.first().unwrap().id.to_string();
        Command::new(&self.editor)
          .arg(&self.directory.path.join(filename))
          .status()
          .context(error::Io)?;
        return Ok(());
      }

      if let Some(selected_items) = Search::new(candidates).run() {
        for item in selected_items.iter() {
          if let Some(id) = NoteId::parse(&item.output().to_string()) {
            let path = Path::join(&self.directory.path, Path::new(&id.to_string()));
            Command::new(&self.editor)
              .arg(&path)
              .status()
              .context(error::Io)?;
          }
        }
      }

      return Ok(());
    }

    println!(
      "{}",
      format!("No note with name `{}` was found.", name).red()
    );

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
    if let (Some(l), Some(r)) = (self.directory.find(left), self.directory.find(right)) {
      if let (Some(l), Some(r)) = (Search::new(l).run(), Search::new(r).run()) {
        let left = Note::new(self.directory.path.join(l.first().unwrap()));

        let right = Note::new(self.directory.path.join(r.first().unwrap()));

        left.add_link(&right.id.to_string())?;

        right.add_link(&left.id.to_string())?;

        println!("{} <-> {}", left.id, right.id);
      } else {
        println!(
          "{}",
          "You must choose two notes in order to link them together.".red()
        );
        return Ok(());
      }
    } else {
      println!("{}", "Both notes must exist in order to be linked.".red());
      return Ok(());
    }

    Ok(())
  }

  /// Finds all notes given a `tag`. This method invokes `skim` using the
  /// names of the notes that contain `tag` within the frontmatter and
  /// attempts to open each selected item.
  pub fn find(&self, tag: &str) -> Result<(), Error> {
    if let Some(candidates) = self.directory.find_by_tag(tag) {
      if let Some(selected_items) = Search::new(candidates).run() {
        for item in selected_items.iter() {
          if let Some(id) = NoteId::parse(&item.output().to_string()) {
            Command::new(&self.editor)
              .arg(&self.directory.path.join(id.to_string()))
              .status()
              .context(error::Io)?;
          }
        }
      }
      return Ok(());
    }

    println!(
      "{}",
      format!("No notes exist with the tag `{}`.", tag).red()
    );

    Ok(())
  }

  /// Starts a fuzzy search using note id's in the Zettelkasten directory
  /// Powered by `skim` --> https://github.com/lotabout/skim
  pub fn search(&self) -> Result<(), Error> {
    if let Some(selected_items) = Search::new(self.directory.notes()).run() {
      for item in selected_items.iter() {
        if let Some(id) = NoteId::parse(&item.output().to_string()) {
          Command::new(&self.editor)
            .arg(&self.directory.path.join(id.to_string()))
            .status()
            .context(error::Io)?;
        }
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
    if let Some(candidates) = self.directory.find(name) {
      // if there's only one candidate note, delete it and return
      if candidates.len() == 1 {
        let filename = candidates.first().unwrap().id.to_string();
        fs::remove_file(&self.directory.path.join(filename)).unwrap();
        return Ok(());
      }

      let prompt = Search::new(candidates);

      // delete each candidate note
      if let Some(selections) = prompt.run() {
        for selection in selections {
          let path = Path::join(&self.directory.path, Path::new(&selection.to_string()));
          fs::remove_file(&path).unwrap();
        }
      }

      return Ok(());
    }

    println!(
      "{}",
      format!("No note with name `{}` was found.", name).red()
    );

    Ok(())
  }

  /// Removes a link between two existing notes
  pub fn remove_link(&self, left: &str, right: &str) -> Result<(), Error> {
    if let (Some(l), Some(r)) = (self.directory.find(left), self.directory.find(right)) {
      if let (Some(l), Some(r)) = (Search::new(l).run(), Search::new(r).run()) {
        let left = Note::new(self.directory.path.join(l.first().unwrap()));

        let right = Note::new(self.directory.path.join(r.first().unwrap()));

        left.remove_link(&right.id.to_string())?;

        right.remove_link(&left.id.to_string())?;

        println!("{} <-X-> {}", left.id, right.id);
      } else {
        println!(
          "{}",
          "You must choose two notes in order to link them together.".red()
        );
        return Ok(());
      }
    } else {
      println!("{}", "Both notes must exist in order to be linked.".red());
      return Ok(());
    }
    Ok(())
  }

  pub fn tag(&self, name: &str, tag: &str) -> Result<(), Error> {
    if let Some(candidates) = self.directory.find(name) {
      // if there's only one candidate note, tag it and return
      if candidates.len() == 1 {
        let candidate = candidates.first().unwrap();
        candidate.add_tag(tag)?;
        return Ok(());
      }

      if let Some(selected_items) = Search::new(candidates).run() {
        for item in selected_items.iter() {
          if let Some(id) = NoteId::parse(&item.output().to_string()) {
            let path = Path::join(&self.directory.path, Path::new(&id.to_string()));
            let note = Note::new(path.to_owned());
            note.add_tag(tag)?;
          }
        }
      }

      return Ok(());
    }

    println!(
      "{}",
      format!("No note with name `{}` was found.", name).red()
    );

    Ok(())
  }

  pub fn remove_tag(&self, name: &str, tag: &str) -> Result<(), Error> {
    if let Some(candidates) = self.directory.find(name) {
      // if there's only one candidate note, tag it and return
      if candidates.len() == 1 {
        let candidate = candidates.first().unwrap();
        candidate.remove_tag(tag)?;
        return Ok(());
      }

      if let Some(selected_items) = Search::new(candidates).run() {
        for item in selected_items.iter() {
          if let Some(id) = NoteId::parse(&item.output().to_string()) {
            let path = Path::join(&self.directory.path, Path::new(&id.to_string()));
            let note = Note::new(path.to_owned());
            note.remove_tag(tag)?;
          }
        }
      }

      return Ok(());
    }

    println!(
      "{}",
      format!("No note with name `{}` was found.", name).red()
    );

    Ok(())
  }
}
