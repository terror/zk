#[macro_export]
macro_rules! in_temp_dir {
  ($block: block) => {
    let tempdir = TempDir::new().unwrap();
    assert!(tempdir.path().exists());

    env::set_current_dir(&tempdir.path()).unwrap();

    $block;
  };
}
