#[cfg(test)]
use crate::common::*;

#[macro_export]
/// Run a test using a temporary directory
macro_rules! in_temp_dir {
  ($body: block) => {
    let tempdir = TempDir::new().unwrap();
    assert!(tempdir.path().exists());
    env::set_current_dir(&tempdir.path()).unwrap();
    $body
  };
}

#[cfg(test)]
pub fn create(note_id: &NoteId) -> Result<Note, Error> {
  let path = env::current_dir().unwrap().join(&note_id.to_string());

  let mut file = File::create(&path).context(error::Io)?;

  file
    .write_all(&Matter::default(&note_id.name))
    .context(error::Io)?;

  Ok(Note::new(path))
}

#[cfg(test)]
pub fn sleep() {
  thread::sleep(time::Duration::from_millis(1000));
}
