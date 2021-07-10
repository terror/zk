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
    let mut file = File::create(
      &self
        .directory
        .path
        .join(NoteId::new(name, &self.config.ext).to_string()),
    )
    .context(error::Io)?;

    file.write_all(&Matter::default(name)).context(error::Io)?;

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
      let note = candidates.first().unwrap();
      Command::new(&self.config.editor)
        .arg(&note.path)
        .status()
        .context(error::Io)?;
      return Ok(());
    }

    // try to open all candidate notes
    for note in Search::new(candidates).run()? {
      Command::new(&self.config.editor)
        .arg(&note.path)
        .status()
        .context(error::Io)?;
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
    let left = Search::new(self.directory.find(left)?)
      .run()?
      .first()
      .unwrap()
      .to_owned();

    let right = Search::new(self.directory.find(right)?)
      .run()?
      .first()
      .unwrap()
      .to_owned();

    left.add_link(&right.id.to_string())?;
    right.add_link(&left.id.to_string())?;

    println!(
      "{} {}",
      format!("{} <-> {}", left.id, right.id),
      "✔".green()
    );

    Ok(())
  }

  /// Finds all notes given a `tag`. This method invokes `skim` using the
  /// names of the notes that contain `tag` within the frontmatter and
  /// attempts to open each selected item.
  pub fn find(&self, tag: &str) -> Result<(), Error> {
    let candidates = self.directory.find_by_tag(tag)?;

    for note in Search::new(candidates).run()? {
      Command::new(&self.config.editor)
        .arg(&note.path)
        .status()
        .context(error::Io)?;
    }

    Ok(())
  }

  /// Starts a fuzzy search using note id's in the Zettelkasten directory
  /// Powered by `skim` --> https://github.com/lotabout/skim
  pub fn search(&self) -> Result<(), Error> {
    for note in Search::new(self.directory.notes()?).run()? {
      Command::new(&self.config.editor)
        .arg(&note.path)
        .status()
        .context(error::Io)?;
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
  ///
  /// This method should scan all notes with a link to the soon to be
  /// deleted note and remove those links.
  pub fn remove(&self, name: &str) -> Result<(), Error> {
    let candidates = self.directory.find(name)?;

    // if there's only one candidate note, delete it and return
    if candidates.len() == 1 {
      let candidate = candidates.first().unwrap();

      let id = &candidate.id.to_string();

      for note in self.directory.notes()? {
        if note.has_link(id) {
          note.remove_link(id)?;
        }
      }

      fs::remove_file(&candidate.path).unwrap();

      return Ok(());
    }

    // delete each candidate note, removing links
    // from each note that links to it
    for candidate in Search::new(candidates).run()? {
      let id = &candidate.id.to_string();

      for note in self.directory.notes()? {
        if note.has_link(id) {
          note.remove_link(id)?;
        }
      }

      fs::remove_file(candidate.path).unwrap();
    }

    Ok(())
  }

  /// Removes a link between two existing notes
  pub fn remove_link(&self, left: &str, right: &str) -> Result<(), Error> {
    let left = Search::new(self.directory.find(left)?)
      .run()?
      .first()
      .unwrap()
      .to_owned();

    let right = Search::new(self.directory.find(right)?)
      .run()?
      .first()
      .unwrap()
      .to_owned();

    left.remove_link(&right.id.to_string())?;
    right.remove_link(&left.id.to_string())?;

    println!(
      "{} {}",
      format!("{} <-X-> {}", left.id, right.id),
      "✔".green()
    );

    Ok(())
  }

  /// Adds a tag to an existing note
  pub fn tag(&self, name: &str, tag: &str) -> Result<(), Error> {
    let candidates = self.directory.find(name)?;

    // if there's only one candidate note, tag it and return
    if candidates.len() == 1 {
      let candidate = candidates.first().unwrap();
      candidate.add_tag(tag)?;
      return Ok(());
    }

    // tag each candidate note
    for note in Search::new(candidates).run()? {
      note.add_tag(tag)?;
    }

    Ok(())
  }

  /// Removes a tag from an existing note
  pub fn remove_tag(&self, name: &str, tag: &str) -> Result<(), Error> {
    let candidates = self.directory.find(name)?;

    // if there's only one candidate note, tag it and return
    if candidates.len() == 1 {
      let candidate = candidates.first().unwrap();
      candidate.remove_tag(tag)?;
      return Ok(());
    }

    // remove the tag from each candidate note
    for note in Search::new(candidates).run()? {
      note.remove_tag(tag)?;
    }

    Ok(())
  }

  /// Explores a notes links recursively.
  ///
  /// A user can either choose to
  /// A) Explore a notes links
  /// B) Edit a note
  ///
  /// Lets say we have the following simple adjacency list:
  ///
  /// a -> [b]
  /// b -> [a, c, d]
  /// c -> [b, d]
  /// d -> [b, e, c]
  /// e -> [d]
  ///
  /// A user can explore and end up at `e` in the following steps:
  /// a -> b -> d -> e
  pub fn explore(&self, name: &str) -> Result<(), Error> {
    let note = Search::new(self.directory.find(name)?)
      .run()?
      .first()
      .unwrap()
      .to_owned();

    let options = SkimOptionsBuilder::default()
      .height(Some("100%"))
      .preview(Some(""))
      .multi(true)
      .bind(vec!["ctrl-e:abort", "Enter:accept"])
      .build()
      .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    note
      .matter
      .links
      .unwrap_or_default()
      .iter()
      .for_each(|link| {
        tx.send(Arc::new(Note::new(self.directory.path.join(link)).unwrap()))
          .unwrap();
      });

    drop(tx);

    if let Some(out) = Skim::run_with(&options, Some(rx)) {
      let selected_items = out
        .selected_items
        .iter()
        .map(|selected_item| {
          (**selected_item)
            .as_any()
            .downcast_ref::<Note>()
            .unwrap()
            .clone()
        })
        .collect::<Vec<Note>>();

      match out.final_key {
        // explore the selected items links recursively
        Key::Enter =>
          if let Some(item) = selected_items.first() {
            self.explore(&item.id.name)?
          },

        // edit the selected note
        Key::Ctrl('e') =>
          if let Some(item) = selected_items.first() {
            self.open(&item.id.name)?
          },

        _ => {},
      };
    }

    Ok(())
  }
}
