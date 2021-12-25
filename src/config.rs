use crate::common::*;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Config {
  pub(crate) path:   PathBuf,
  pub(crate) editor: String,
}

impl Config {
  fn default() -> &'static str {
    indoc! {"
      path   = '~/.zk'
      editor = 'vim'
    "}
  }

  fn filename() -> &'static str {
    ".zk.toml"
  }

  fn path() -> Result<Option<PathBuf>> {
    Ok(
      xdg::BaseDirectories::with_prefix(dirs::home_dir().unwrap_or_default())
        .context(error::BaseDirectories)?
        .find_config_file(Self::filename()),
    )
  }

  pub(crate) fn load() -> Result<Self> {
    if let Some(path) = Self::path()? {
      let content = fs::read_to_string(&path)?;
      Ok(toml::from_str(&content)?)
    } else {
      Ok(toml::from_str(Self::default())?)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default() -> Result<()> {
    let config: Config = toml::from_str(Config::default())?;
    assert_eq!(config.path.to_str().unwrap(), "~/.zk");
    assert_eq!(config.editor, "vim");
    Ok(())
  }
}
