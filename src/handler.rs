use crate::common::*;

#[derive(Debug)]
pub(crate) struct Handler {
  pub(crate) config:    Config,
  pub(crate) directory: Directory,
}

impl Handler {
  pub(crate) fn new(config: Config, directory: Directory) -> Self {
    Self { config, directory }
  }

  /// Creates a new note with the specified `name` in the Zettelkasten directory
  /// with an appropriate prefix, in addition to writing the default YAML
  /// frontmatter.
  pub fn create(&self, name: &str) -> Result<()> {
    self.open(
      &Note::create(self.directory.path.join(NoteId::new(name).to_string()))?
        .id
        .name,
    )
  }

  /// Opens a note given a `name` using the editor specified in the
  /// configuration file. If there are multiple notes present with the
  /// same `name`, the user will be prompted with `skim` to choose
  /// which file is desired to be opened.
  pub(crate) fn open(&self, name: &str) -> Result<()> {
    Search::new(self.directory.find(name)?)
      .run()?
      .iter()
      .try_for_each(|note| {
        Command::new(&self.config.editor).arg(&note.path).status()?;
        Ok(())
      })
  }

  /// Links two notes together. This entails checking and modifying both notes'
  /// YAML frontmatter to ensure a link is created.
  pub(crate) fn link(&self, left: &str, right: &str) -> Result<()> {
    let mut left = Search::new(self.directory.find(left)?)
      .run()?
      .first()
      .unwrap()
      .to_owned();

    let mut right = Search::new(self.directory.find(right)?)
      .run()?
      .first()
      .unwrap()
      .to_owned();

    left.add_link(&right.id.to_string())?;
    right.add_link(&left.id.to_string())?;

    Ok(())
  }

  /// Finds all notes given a `tag`. This method invokes `skim` using the
  /// names of the notes that contain `tag` within the frontmatter and
  /// attempts to open each selected item.
  pub(crate) fn find(&self, tag: &str) -> Result<()> {
    Search::new(self.directory.find_by_tag(tag)?)
      .run()?
      .iter()
      .try_for_each(|note| {
        Command::new(&self.config.editor).arg(&note.path).status()?;
        Ok(())
      })
  }

  /// Starts a fuzzy search using note id's in the Zettelkasten directory.
  /// Powered by `skim` --> https://github.com/lotabout/skim
  pub(crate) fn search(&self) -> Result<()> {
    Search::new(self.directory.notes()?)
      .run()?
      .iter()
      .try_for_each(|note| {
        Command::new(&self.config.editor).arg(&note.path).status()?;
        Ok(())
      })
  }

  /// Writes the current Zettelkasten storage location to stdout.
  pub fn dir(&self) -> Result<()> {
    println!("{}", self.directory.path.expand().display());
    Ok(())
  }

  /// Removes an existing note in the Zettelkasten directory. This will
  /// also prompt the user if more than one note exists with `name`.
  /// This method should scan all notes with a link to the soon to be
  /// deleted note and remove those links.
  pub(crate) fn remove(&self, name: &str) -> Result<()> {
    Search::new(self.directory.find(name)?)
      .run()?
      .iter()
      .try_for_each(|item| {
        self
          .directory
          .notes()?
          .iter_mut()
          .filter(|note| note.has_link(&item.id.to_string()))
          .try_for_each(|note| -> Result<()> {
            note.remove_link(&item.id.to_string())?;
            Ok(())
          })?;
        item.remove()
      })
  }

  /// Removes a link between two existing notes.
  pub(crate) fn remove_link(&self, left: &str, right: &str) -> Result<()> {
    let mut left = Search::new(self.directory.find(left)?)
      .run()?
      .first()
      .unwrap()
      .to_owned();

    let mut right = Search::new(self.directory.find(right)?)
      .run()?
      .first()
      .unwrap()
      .to_owned();

    left.remove_link(&right.id.to_string())?;
    right.remove_link(&left.id.to_string())?;

    Ok(())
  }

  /// Adds a tag to an existing note.
  pub(crate) fn tag(&self, name: &str, tag: &str) -> Result<()> {
    Search::new(self.directory.find(name)?)
      .run()?
      .iter_mut()
      .filter(|note| !note.has_tag(tag))
      .try_for_each(|note| {
        note.add_tag(tag)?;
        Ok(())
      })
  }

  /// Removes a tag from an existing note.
  pub(crate) fn remove_tag(&self, name: &str, tag: &str) -> Result<()> {
    Search::new(self.directory.find(name)?)
      .run()?
      .iter_mut()
      .filter(|note| note.has_tag(tag))
      .try_for_each(|note| {
        note.remove_tag(tag)?;
        Ok(())
      })
  }

  /// Explore a notes links recursively.
  pub(crate) fn explore(&self, name: &str) -> Result<()> {
    let note = Search::new(self.directory.find(name)?)
      .run()?
      .first()
      .ok_or(Error::NoteNotSelected)?
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
      .iter()
      .map(|link| Note::from(self.directory.path.join(link)))
      .collect::<Result<Vec<_>, _>>()?
      .iter()
      .cloned()
      .try_for_each(|note| tx.send(Arc::new(note)))
      .map_err(|_| Error::ChannelSend)?;

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
        Key::Enter =>
          if let Some(item) = selected_items.first() {
            self.explore(&item.id.name)?
          },
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
