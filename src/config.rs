use crate::common::*;

const FILENAME: &str = ".zk.toml";

const DEFAULT: &str = "
  path   = '~/.zk'
  editor = 'vim'
  ext    = 'md'
";

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
  pub path:   PathBuf,
  pub editor: String,
  pub ext:    String,
}

impl Config {
  fn default() -> &'static str {
    DEFAULT.trim()
  }

  fn filename() -> &'static str {
    FILENAME
  }

  fn path() -> Result<Option<PathBuf>, Error> {
    Ok(
      xdg::BaseDirectories::with_prefix(dirs::home_dir().unwrap())
        .context(error::BaseDirectories)?
        .find_config_file(Self::filename()),
    )
  }

  pub fn load() -> Result<Self, Error> {
    if let Some(path) = Self::path()? {
      let path = &path;
      let contents = fs::read_to_string(path).context(error::LoadConfig { path })?;
      Ok(toml::from_str(&contents).context(error::DeserializeConfig { path })?)
    } else {
      Ok(toml::from_str(Self::default()).unwrap())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load_default_config() {
    if Config::path().unwrap().is_none() {
      let config = Config::load();

      assert!(config.is_ok());

      let config = config.unwrap();

      assert_eq!(
        config.path.expand(),
        Path::join(&dirs::home_dir().unwrap(), ".zk")
      );
      assert_eq!(config.editor, "vim");
      assert_eq!(config.ext, "md");
    }
  }
}
