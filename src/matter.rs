use crate::common::*;

#[derive(Debug, Clone)]
pub struct Matter {
  pub name:  String,
  pub tags:  Vec<String>,
  pub links: Vec<String>,
}

/// Attempts to turn the str `content` into `Matter` struct with the
/// appropriate fields.
impl From<&str> for Matter {
  fn from(content: &str) -> Self {
    let matter = YamlLoader::load_from_str(&content).unwrap()[0].clone();

    let name = matter["name"].as_str().unwrap_or(&String::new()).to_owned();

    let tags = matter["tags"]
      .as_vec()
      .unwrap_or(&Vec::new())
      .iter()
      .map(|tag| tag.as_str().unwrap().to_string())
      .collect::<Vec<String>>();

    let links = matter["links"]
      .as_vec()
      .unwrap_or(&Vec::new())
      .iter()
      .map(|tag| tag.as_str().unwrap().to_string())
      .collect::<Vec<String>>();

    Self { name, tags, links }
  }
}

impl Matter {
  pub fn default(name: &str) -> Vec<u8> {
    format!("---\nname: {}\n---\n", &name).as_bytes().to_owned()
  }

  pub fn into_string(matter: Matter) -> String {
    let mut result = String::from("---\n");

    if !matter.name.is_empty() {
      result.push_str(&format!("name: {}\n", matter.name))
    }

    if !matter.tags.is_empty() {
      result.push_str("tags:\n");
      matter.tags.iter().for_each(|tag| {
        result.push_str(&format!(" - {}\n", tag));
      });
    }

    if !matter.links.is_empty() {
      result.push_str("links:\n");
      matter.links.iter().for_each(|link| {
        result.push_str(&format!(" - {}\n", link));
      });
    }

    format!("{}---\n", result)
  }
}
