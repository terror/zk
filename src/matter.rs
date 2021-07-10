use crate::common::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Matter {
  pub name:  String,
  pub tags:  Option<Vec<String>>,
  pub links: Option<Vec<String>>,
}

impl Matter {
  /// Constructs a default frontmatter as bytes
  pub fn default(name: &str) -> Vec<u8> {
    format!("---\nname: {}\n---\n", &name).as_bytes().to_owned()
  }

  /// Converts a &str -> Matter
  pub fn from(content: &str) -> Result<Self, Error> {
    serde_yaml::from_str(content).context(error::MatterSerialize)
  }

  /// Converts a Matter -> String
  pub fn into(matter: Matter) -> Result<String, Error> {
    Ok(format!(
      "{}---\n",
      serde_yaml::to_string(&matter).context(error::MatterDeserialize)?
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn setup() -> (Matter, String, String) {
    let matter = Matter {
      name:  String::from("123-a"),
      tags:  Some(vec![String::from("software"), String::from("code")]),
      links: Some(vec![String::from("b"), String::from("c")]),
    };

    // This is what we're serializing
    let as_str = r#"
      name: 123-a
      tags:
        - software
        - code
      links:
        - b
        - c
      "#;

    // This is what we want (when deserializing)
    let want = r#"
      ---
      name: 123-a
      tags:
        - software
        - code
      links:
        - b
        - c
      ---
      "#;

    (
      matter,
      dedent(as_str.strip_prefix("\n").unwrap()),
      dedent(want.strip_prefix("\n").unwrap()),
    )
  }

  #[test]
  fn from() {
    let (matter, as_str, _) = setup();
    let res: Matter = Matter::from(as_str.as_str()).unwrap();
    assert_eq!(res, matter);
  }

  #[test]
  fn into() {
    let (matter, _, want) = setup();
    let res = Matter::into(matter).unwrap();
    assert_eq!(res, want);
  }
}
