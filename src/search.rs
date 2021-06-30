use crate::common::*;

pub struct Search {
  items: Vec<Note>,
}

impl Search {
  pub fn new(items: Vec<Note>) -> Self {
    Self { items }
  }

  /// This method launches a `skim` fuzzy search with `items` and
  /// returns the selected items.
  pub fn run(&self) -> Result<Vec<String>, Error> {
    if self.items.len() == 1 {
      return Ok(
        self
          .items
          .iter()
          .map(|note| note.id.to_string())
          .collect::<Vec<String>>(),
      );
    }

    let options = SkimOptionsBuilder::default()
      .height(Some("100%"))
      .preview(Some(""))
      .multi(true)
      .build()
      .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for note in &self.items {
      tx.send(Arc::new(SearchItem {
        text: note.id.to_string(),
        path: note.path.clone(),
      }))
      .unwrap();
    }

    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))
      .map(|out| out.selected_items)
      .unwrap_or_else(Vec::new)
      .iter()
      .map(|selected_item| selected_item.output().to_string())
      .collect::<Vec<String>>();

    match selected_items.len() {
      0 => Err(error::Error::NoteNotSelected),
      _ => Ok(selected_items),
    }
  }
}
