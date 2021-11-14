use crate::common::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Matter {
  pub name:  String,
  pub tags:  Vec<String>,
  pub links: Vec<String>,
}

/// Attempts to turn the str `content` into a `Matter` struct with the
/// appropriate fields.
impl From<&str> for Matter {
  fn from(content: &str) -> Self {
    let matter = YamlLoader::load_from_str(content).unwrap()[0].clone();

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

#[cfg(test)]
mod tests {
  use super::*;

  fn cases() -> Vec<(Matter, String)> {
    vec![
      (
        Matter {
          name:  "a".into(),
          tags:  vec![String::from("code"), String::from("software")],
          links: vec![String::from("b"), String::from("c")],
        },
        r#"
        ---
        name: a
        tags:
         - code
         - software
        links:
         - b
         - c
        ---
      "#
        .into(),
      ),
      (
        Matter {
          name:  "b".into(),
          tags:  vec![],
          links: vec![String::from("b"), String::from("c")],
        },
        r#"
        ---
        name: b
        links:
         - b
         - c
        ---
      "#
        .into(),
      ),
      (
        Matter {
          name:  "c".into(),
          tags:  vec![String::from("code"), String::from("software")],
          links: vec![],
        },
        r#"
        ---
        name: c
        tags:
         - code
         - software
        ---
      "#
        .into(),
      ),
      (
        Matter {
          name:  "d".into(),
          tags:  vec![],
          links: vec![],
        },
        r#"
        ---
        name: d
        ---
      "#
        .into(),
      ),
    ]
  }

  fn strip(s: String) -> String {
    dedent(s.strip_prefix('\n').unwrap())
  }

  #[test]
  fn serialize() {
    for (have, want) in cases() {
      assert_eq!(Matter::into_string(have), strip(want));
    }
  }

  #[test]
  fn deserialize() {
    for (want, have) in cases() {
      assert_eq!(Matter::from(strip(have).as_str()), want);
    }
  }
}
