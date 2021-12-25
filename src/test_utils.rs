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

pub(crate) fn create_note(name: &str) -> Note {
  Note::create(
    env::current_dir()
      .unwrap()
      .join(&NoteId::new(name).to_string()),
  )
  .unwrap()
}

pub(crate) fn sleep() {
  thread::sleep(time::Duration::from_millis(1000));
}
