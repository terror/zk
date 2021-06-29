use crate::common::*;

pub struct SearchItem {
  pub text: String,
  pub path: PathBuf,
}

impl SkimItem for SearchItem {
  fn text(&self) -> Cow<str> {
    Cow::Borrowed(&self.text)
  }

  fn preview(&self, _context: PreviewContext) -> ItemPreview {
    ItemPreview::Command(format!("cat {}", self.path.display()))
  }
}
