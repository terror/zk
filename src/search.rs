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
  pub fn run(&self) -> Option<Vec<String>> {
    if self.items.len() == 1 {
      return Some(
        self
          .items
          .iter()
          .map(|note| note.id.to_string())
          .collect::<Vec<String>>(),
      );
    }

    let options = SkimOptionsBuilder::default()
      .height(Some("50%"))
      .multi(true)
      .preview(Some(""))
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
      0 => None,
      _ => Some(selected_items),
    }
  }
}
