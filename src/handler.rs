use crate::common::*;

#[derive(Debug)]
pub(crate) struct Handler {
  pub(crate) config: Config,
  pub(crate) directory: Directory,
}

impl Handler {
  pub(crate) fn new(config: Config, directory: Directory) -> Self {
    Self { config, directory }
  }

  pub fn create(&self, name: &str) -> Result<()> {
    self.open(
      &Note::create(self.directory.path.join(NoteId::new(name).to_string()))?
        .id
        .name,
    )
  }

  pub(crate) fn open(&self, name: &str) -> Result<()> {
    Search::new(self.directory.find(name)?)
      .run()?
      .iter()
      .try_for_each(|note| {
        Command::new(&self.config.editor).arg(&note.path).status()?;
        Ok(())
      })
  }

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

  pub(crate) fn find(&self, tag: &str) -> Result<()> {
    Search::new(self.directory.find_by_tag(tag)?)
      .run()?
      .iter()
      .try_for_each(|note| {
        Command::new(&self.config.editor).arg(&note.path).status()?;
        Ok(())
      })
  }

  pub(crate) fn search(&self) -> Result<()> {
    Search::new(self.directory.notes()?)
      .run()?
      .iter()
      .try_for_each(|note| {
        Command::new(&self.config.editor).arg(&note.path).status()?;
        Ok(())
      })
  }

  pub fn dir(&self) {
    println!("{}", self.directory.path.expand().display());
  }

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
      .unwrap_or_default()
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
        Key::Enter => {
          if let Some(item) = selected_items.first() {
            self.explore(&item.id.name)?
          }
        }
        Key::Ctrl('e') => {
          if let Some(item) = selected_items.first() {
            self.open(&item.id.name)?
          }
        }
        _ => {}
      };
    }

    Ok(())
  }
}
