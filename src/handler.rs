use crate::common::*;

pub struct Handler;

impl Handler {
  pub fn init(path: &Option<PathBuf>) -> Result<(), Error> {
    Ok(())
  }

  pub fn new(name: &str, config: Config) -> Result<(), Error> {
    Ok(())
  }

  pub fn open(name: &str) -> Result<(), Error> {
    Ok(())
  }

  pub fn link(left: &str, right: &str) -> Result<(), Error> {
    Ok(())
  }

  pub fn find(tag: &str) -> Result<(), Error> {
    Ok(())
  }
}
