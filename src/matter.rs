use crate::common::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct Matter {
  pub(crate) name:  String,
  pub(crate) tags:  Option<Vec<String>>,
  pub(crate) links: Option<Vec<String>>,
}

impl Matter {
  pub(crate) fn new(name: &str, tags: Option<Vec<String>>, links: Option<Vec<String>>) -> Self {
    Self {
      name: name.to_owned(),
      tags,
      links,
    }
  }

  /// Return the default YAML frontmatter as bytes.
  pub(crate) fn default(name: &str) -> Result<Vec<u8>> {
    Ok(
      Self::into(Matter::new(name, None, None))?
        .as_bytes()
        .to_owned(),
    )
  }

  /// Parse a string `content` into a `Matter` instance.
  pub(crate) fn from(content: &str) -> Result<Self> {
    Ok(serde_yaml::from_str(
      content
        .strip_prefix("---\n")
        .unwrap_or(content)
        .strip_suffix("---\n")
        .unwrap_or(content),
    )?)
  }

  /// Parse a `Matter` struct into a string.
  pub(crate) fn into(matter: Matter) -> Result<String> {
    Ok(format!("{}---\n", serde_yaml::to_string(&matter)?))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn cases<'a>() -> Vec<(Matter, &'a str)> {
    vec![
      (
        Matter {
          name:  "a".into(),
          tags:  Some(vec![String::from("code"), String::from("software")]),
          links: Some(vec![String::from("b"), String::from("c")]),
        },
        indoc! {"
          ---
          name: a
          tags:
            - code
            - software
          links:
            - b
            - c
          ---
        "},
      ),
      (
        Matter {
          name:  "b".into(),
          tags:  Some(vec![]),
          links: Some(vec![String::from("b"), String::from("c")]),
        },
        indoc! {"
          ---
          name: b
          tags: []
          links:
            - b
            - c
          ---
        "},
      ),
      (
        Matter {
          name:  "c".into(),
          tags:  Some(vec![String::from("code"), String::from("software")]),
          links: Some(vec![]),
        },
        indoc! {"
          ---
          name: c
          tags:
            - code
            - software
          links: []
          ---
        "},
      ),
      (
        Matter {
          name:  "d".into(),
          tags:  Some(vec![]),
          links: Some(vec![]),
        },
        indoc! {"
          ---
          name: d
          tags: []
          links: []
          ---
        "},
      ),
    ]
  }

  #[test]
  fn serialize() {
    for (have, want) in cases() {
      assert_eq!(Matter::into(have).unwrap(), want);
    }
  }

  #[test]
  fn deserialize() {
    for (want, have) in cases() {
      assert_eq!(Matter::from(have).unwrap(), want);
    }
  }
}
