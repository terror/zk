use crate::common::*;

pub(crate) trait PathExt {
  fn expand(&self) -> PathBuf;
  fn unwrapped_extension(&self) -> &str;
  fn unwrapped_filename(&self) -> &str;
}

impl PathExt for PathBuf {
  fn expand(&self) -> PathBuf {
    PathBuf::from(
      shellexpand::tilde(&self.to_str().unwrap_or_default()).to_string(),
    )
  }

  fn unwrapped_filename(&self) -> &str {
    self
      .file_name()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default()
  }

  fn unwrapped_extension(&self) -> &str {
    self.extension().and_then(OsStr::to_str).unwrap_or_default()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn unwrapped_extension() {
    assert_eq!(PathBuf::from("c.md").unwrapped_extension(), "md");
  }

  #[test]
  fn unwrapped_filename() {
    assert_eq!(PathBuf::from("a/b/c.md").unwrapped_filename(), "c.md");
  }
}
