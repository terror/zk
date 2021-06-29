use crate::common::*;

#[derive(Debug)]
pub struct Note {
  /// The notes timestamp prefix and name.
  pub id: NoteId,

  /// Where the note is currently stored.
  pub path: PathBuf,

  /// Yaml frontmatter
  pub matter: Yaml,

  /// The notes content as a String.
  pub content: String,
}

/// The important thing here is to fully create a `Note` based on the
/// filename and contents of the `.md` file.
impl From<PathBuf> for Note {
  fn from(path: PathBuf) -> Self {
    let filename = path.file_stem().unwrap().to_str().unwrap();

    let mut split = filename.split('-');

    let (matter, content) = matter::matter(&fs::read_to_string(&path).unwrap()).unwrap();

    let matter = YamlLoader::load_from_str(&matter).unwrap();

    let matter = matter[0].clone();

    Self {
      id: NoteId {
        prefix: split.next().unwrap().to_string(),
        name:   split.next().unwrap().to_string(),
      },
      path,
      content,
      matter,
    }
  }
}

impl Note {
  /// Checks if a link exists between the current note and `name`.
  pub fn has_link(&self, name: &str) -> bool {
    if let Some(links) = self.matter["links"].as_vec() {
      for link in links {
        if *link == Yaml::from_str(name) {
          return true;
        }
      }
    }
    false
  }

  /// Attempts to add `name` as a link to the current note.
  pub fn add_link(&self, name: &str) -> Result<(), Error> {
    if self.has_link(name) {
      return Ok(());
    }

    let mut frontmatter = String::from("---\n");

    if let Some(name) = self.matter["name"].as_str() {
      frontmatter.push_str(&format!("name: {}\n", name))
    }

    if let Some(tags) = self.matter["tags"].as_vec() {
      frontmatter.push_str("tags:\n");
      for tag in tags {
        frontmatter.push_str(&format!(" - {}\n", tag.as_str().unwrap()));
      }
    }

    frontmatter.push_str("links:\n");

    if let Some(links) = self.matter["links"].as_vec() {
      for link in links {
        frontmatter.push_str(&format!(" - {}\n", link.as_str().unwrap()));
      }
    }

    frontmatter.push_str(&format!(" - {}\n", name));

    frontmatter.push_str("---\n");

    let mut file = File::create(&self.path).unwrap();
    file.write_all(&frontmatter.as_bytes()).unwrap();
    file.write_all(&self.content.as_bytes()).unwrap();

    Ok(())
  }

  /// Attempts to add `name` as a link to the current note.
  pub fn remove_link(&self, name: &str) -> Result<(), Error> {
    let mut frontmatter = String::from("---\n");

    if let Some(name) = self.matter["name"].as_str() {
      frontmatter.push_str(&format!("name: {}\n", name))
    }

    if let Some(tags) = self.matter["tags"].as_vec() {
      frontmatter.push_str("tags:\n");
      for tag in tags {
        frontmatter.push_str(&format!(" - {}\n", tag.as_str().unwrap()));
      }
    }

    frontmatter.push_str("links:\n");

    if let Some(links) = self.matter["links"].as_vec() {
      for link in links {
        if *link == Yaml::from_str(name) {
          continue;
        }
        frontmatter.push_str(&format!(" - {}\n", link.as_str().unwrap()));
      }
    }

    frontmatter.push_str("---\n");

    let mut file = File::create(&self.path).unwrap();
    file.write_all(&frontmatter.as_bytes()).unwrap();
    file.write_all(&self.content.as_bytes()).unwrap();

    Ok(())
  }
}
