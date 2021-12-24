use crate::common::*;

pub(crate) struct Search {
  items: Vec<Note>,
}

impl Search {
  pub(crate) fn new(items: Vec<Note>) -> Self {
    Self { items }
  }

  /// This method launches a `skim` fuzzy search with `items` and
  /// returns the selected items as their original type.
  pub(crate) fn run(&self) -> Result<Vec<Note>, Error> {
    if self.items.len() == 1 {
      return Ok(self.items.clone());
    }

    let options = SkimOptionsBuilder::default()
      .height(Some("100%"))
      .preview(Some(""))
      .multi(true)
      .build()
      .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    self
      .items
      .iter()
      .for_each(|note| tx.send(Arc::new(note.to_owned())).unwrap());

    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))
      .map(|out| out.selected_items)
      .unwrap_or_else(Vec::new)
      .iter()
      .map(|selected_item| {
        (**selected_item)
          .as_any()
          .downcast_ref::<Note>()
          .unwrap()
          .to_owned()
      })
      .collect::<Vec<Note>>();

    match selected_items.len() {
      0 => Err(error::Error::NoteNotSelected),
      _ => Ok(selected_items),
    }
  }
}
