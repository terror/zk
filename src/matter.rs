use crate::common::*;

#[derive(Debug)]
pub struct Matter {
  pub name:  Option<String>,
  pub tags:  Option<Vec<String>>,
  pub links: Option<Vec<String>>,
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

    let name = {
      if let Some(name) = matter["name"].as_str() {
        Some(name.to_string())
      } else {
        None
      }
    };

    let tags = {
      if let Some(tags) = matter["tags"].as_vec() {
        Some(
          tags
            .iter()
            .map(|tag| tag.as_str().unwrap().to_string())
            .collect::<Vec<String>>(),
        )
      } else {
        None
      }
    };

    let links = {
      if let Some(links) = matter["links"].as_vec() {
        Some(
          links
            .iter()
            .map(|link| link.as_str().unwrap().to_string())
            .collect::<Vec<String>>(),
        )
      } else {
        None
      }
    };

    Self { name, tags, links }
  }
}

impl Matter {
  pub fn build(
    name: &Option<String>,
    tags: &Option<Vec<String>>,
    links: &Option<Vec<String>>,
  ) -> String {
    let mut result = String::from("---\n");

    if let Some(name) = name {
      result.push_str(&format!("name: {}\n", name))
    }

    if let Some(tags) = tags {
      result.push_str("tags:\n");
      for tag in tags {
        result.push_str(&format!(" - {}\n", tag));
      }
    }

    if let Some(links) = links {
      result.push_str("links:\n");
      for link in links {
        result.push_str(&format!(" - {}\n", link));
      }
    }

    result.push_str("---\n");

    result
  }
}
