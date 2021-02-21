use crate::common::*;

// Parse the passed in Zettelkasten path
// Expand the users home directory (where the config file will live)
// Create the file and write sensible defaults to it

// Notes:
// - Check if file already exists and prompt user before overwrite
// - Requires $EDITOR to be set

#[derive(Serialize)]
/// Default configuration file values
struct Default<'a> {
  path:   &'a str,
  editor: &'a str,
}

fn prompt_user() -> Result<()> {
  print!("Are you sure you would like to overwrite your existing config file? [y/n]: ");

  io::stdout().flush()?;

  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer)?;

  if buffer.to_lowercase().trim() == "n" {
    process::exit(0);
  }

  Ok(())
}

fn create_config(path: path::PathBuf) -> Result<()> {
  let home_dir = shellexpand::tilde("~");

  let config = Default {
    path:   path.to_str().unwrap(),
    editor: &env::var("EDITOR")?,
  };

  let toml = toml::to_string(&config).unwrap();

  let mut file = fs::File::create(format!("{}/zk.toml", home_dir))?;

  file.write_all(&toml.as_bytes())?;
  Ok(())
}

pub fn init(path: path::PathBuf) -> Result<()> {
  let zk_dir = fs::canonicalize(&path)?;

  #[cfg(not(test))]
  if path::Path::new(&shellexpand::tilde("~/zk.toml").into_owned()).exists() {
    prompt_user()?;
  }

  create_config(zk_dir)?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_init() {
    in_temp_dir!({
      let result = init(env::current_dir().unwrap());
      assert!(result.is_ok(), result.err().unwrap().to_string());
    });
  }
}
