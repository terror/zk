use crate::common::*;

pub(crate) trait PathExt {
  fn expand(&self) -> PathBuf;
  fn filename(&self) -> &str;
}

impl PathExt for PathBuf {
  fn expand(&self) -> PathBuf {
    PathBuf::from(shellexpand::tilde(&self.to_str().unwrap_or_default()).to_string())
  }

  fn filename(&self) -> &str {
    self
      .file_name()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default()
  }
}
