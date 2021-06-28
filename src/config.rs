use crate::common::*;

const FILENAME: &str = "zk.toml";

const DEFAULT: &str = "
path = '~/zk'
editor = 'vim'
";

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
  /// Path to the Zettelkasten directory
  pub path: PathBuf,

  /// Editor of choice when opening and editing notes
  pub editor: String,
}

impl Config {
  fn default() -> &'static str {
    DEFAULT
  }

  fn filename() -> &'static str {
    FILENAME
  }

  fn path() -> Result<Option<PathBuf>, Error> {
    Ok(
      xdg::BaseDirectories::with_prefix(dirs::home_dir().unwrap())
        .context(error::BaseDirectoriesError)?
        .find_config_file(Self::filename()),
    )
  }

  pub fn load() -> Result<Self, Error> {
    if let Some(path) = Self::path()? {
      let path = &path;
      let contents = fs::read_to_string(path).context(error::LoadConfig { path })?;
      Ok(toml::from_str(&contents).unwrap())
    } else {
      Ok(toml::from_str(Self::default()).unwrap())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn expand_tilde(path: &PathBuf) -> PathBuf {
    PathBuf::from(shellexpand::tilde(path.to_str().unwrap()).to_string())
  }

  #[test]
  fn test_load_config() {
    let config = Config::load();

    assert!(config.is_ok());

    if Config::path().unwrap().is_none() {
      let config = config.unwrap();
      assert_eq!(
        expand_tilde(&config.path),
        PathBuf::from(format!(
          "{}/zk",
          dirs::home_dir().unwrap().to_str().unwrap()
        ))
      );
      assert_eq!(config.editor, "vim");
    }
  }
}
