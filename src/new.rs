use crate::common::*;

// Create a new note with a unique prefix
// - [prefix]-[name].md

// Open this new note with default editor (from config file)

pub fn new(name: String) -> Result<()> {
  let prefix = Utc::now().format("%Y%m%d%H%M%S");
  let note = format!("{}-{}.md", prefix, name);
  fs::File::create(&note)?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    in_temp_dir!({
      let note_name = String::from("test");

      let result = new(note_name);

      assert!(result.is_ok(), result.err().unwrap().to_string());
    });
  }
}
