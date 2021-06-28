use crate::common::*;

pub struct Prompt {
  message: String,
  items:   Vec<String>,
}

impl Prompt {
  pub fn new(message: String, items: Vec<String>) -> Self {
    Self { message, items }
  }

  pub fn interact(&self) -> Option<Vec<String>> {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
      .with_prompt(&self.message)
      .items(&self.items[..])
      .interact()
      .unwrap();

    let mut result = vec![];

    for selection in selections {
      result.push(self.items[selection].clone());
    }

    if result.is_empty() {
      return None;
    }

    Some(result)
  }
}
