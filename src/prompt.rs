use crate::common::*;

pub struct Prompt {
  message: String,
  items:   Vec<String>,
}

impl Prompt {
  pub fn new(message: String, items: Vec<String>) -> Self {
    Self { message, items }
  }

  /// Prompts the user with `message` using `items` as the choice list
  /// returning either the selected items or `None`.
  pub fn interact(&self) -> Option<Vec<String>> {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
      .with_prompt(&self.message)
      .items(&self.items[..])
      .interact()
      .unwrap();

    let result = selections
      .into_iter()
      .map(|selection| self.items[selection].clone())
      .collect::<Vec<String>>();

    if result.is_empty() {
      return None;
    }

    Some(result)
  }

  /// Prompts the user with `message` using `items` as the choice list
  /// returning the first item chosen.
  pub fn interact_choose_first(&self) -> Option<String> {
    if self.items.len() == 1 {
      return Some(self.items.first().unwrap().to_owned());
    }

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
      .with_prompt(&self.message)
      .items(&self.items[..])
      .interact()
      .unwrap();

    let result = selections
      .into_iter()
      .map(|selection| self.items[selection].clone())
      .collect::<Vec<String>>();

    if result.is_empty() {
      return None;
    }

    Some(result.first().unwrap().to_owned())
  }
}
