use crate::common::*;

pub(crate) trait PathExt {
  fn expand(&self) -> PathBuf;
  fn filename(&self) -> &str;
  fn ext(&self) -> &str;
}

impl PathExt for PathBuf {
  fn expand(&self) -> PathBuf {
    PathBuf::from(
      shellexpand::tilde(&self.to_str().unwrap_or_default()).to_string(),
    )
  }

  fn filename(&self) -> &str {
    self
      .file_name()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default()
  }

  fn ext(&self) -> &str {
    self.extension().and_then(OsStr::to_str).unwrap_or_default()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn filename() {
    let path = PathBuf::from("a/b/c.md");
    assert_eq!(path.filename(), "c.md");
  }

  #[test]
  fn extension() {
    let path = PathBuf::from("c.md");
    assert_eq!(path.ext(), "md");
  }
}
