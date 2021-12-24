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

pub(crate) fn create_note(note_id: &NoteId) -> Note {
  let path = env::current_dir().unwrap().join(&note_id.to_string());
  let mut file = File::create(&path).unwrap();
  file.write_all(&Matter::default(&note_id.name)).unwrap();
  Note::new(path)
}

pub(crate) fn sleep() {
  thread::sleep(time::Duration::from_millis(1000));
}
