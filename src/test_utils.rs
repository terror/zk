use crate::common::*;

#[macro_export]
macro_rules! in_temp_dir {
  ($body: block) => {
    let tempdir = TempDir::new().unwrap();
    assert!(tempdir.path().exists());
    env::set_current_dir(&tempdir.path()).unwrap();
    $body
  };
}

pub(crate) fn create_note(name: &str) -> Result<Note> {
  Note::create(env::current_dir()?.join(NoteId::new(name).to_string()))
}

pub(crate) fn sleep() {
  thread::sleep(time::Duration::from_millis(1000));
}
