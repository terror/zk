use crate::common::*;

pub(crate) struct Search {
  items: Vec<Note>,
}

impl Search {
  pub(crate) fn new(items: Vec<Note>) -> Self {
    Self { items }
  }

  pub(crate) fn run(&self) -> Result<Vec<Note>> {
    if self.items.len() == 1 {
      return Ok(self.items.clone());
    }

    let options = SkimOptionsBuilder::default()
      .height(Some("100%"))
      .preview(Some(""))
      .multi(true)
      .build()
      .map_err(|_| Error::SkimOptions)?;

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    self
      .items
      .iter()
      .try_for_each(|note| tx.send(Arc::new(note.to_owned())))
      .map_err(|_| Error::ChannelSend)?;

    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))
      .map(|out| out.selected_items)
      .unwrap_or_default()
      .iter()
      .map(|selected_item| {
        (**selected_item)
          .as_any()
          .downcast_ref::<Note>()
          .unwrap()
          .to_owned()
      })
      .collect::<Vec<Note>>();

    if selected_items.is_empty() {
      return Err(Error::NoteNotSelected);
    }

    Ok(selected_items)
  }
}
