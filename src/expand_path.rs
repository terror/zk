use crate::common::*;

pub(crate) trait Expand {
  fn expand(&self) -> PathBuf;
}

impl Expand for PathBuf {
  fn expand(&self) -> PathBuf {
    PathBuf::from(shellexpand::tilde(&self.to_str().unwrap_or_default()).to_string())
  }
}
