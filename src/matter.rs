use crate::common::*;

#[derive(Debug)]
pub struct Matter {
  pub name:  String,
  pub tags:  Vec<String>,
  pub links: Vec<String>,
}

impl fmt::Display for Matter {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", Matter::build(&self.name, &self.tags, &self.links))
  }
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
  pub fn build(name: &String, tags: &Vec<String>, links: &Vec<String>) -> String {
    let mut result = String::from("---\n");

    if !name.is_empty() {
      result.push_str(&format!("name: {}\n", name))
    }

    if !tags.is_empty() {
      result.push_str("tags:\n");
      for tag in tags {
        result.push_str(&format!(" - {}\n", tag));
      }
    }

    if !links.is_empty() {
      result.push_str("links:\n");
      for link in links {
        result.push_str(&format!(" - {}\n", link));
      }
    }

    result.push_str("---\n");

    result
  }
}
