use crate::common::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "zk")]
pub enum Opt {
  #[structopt(name = "new")]
  /// Create a new note
  New { name: String },

  #[structopt(name = "open")]
  /// Open a note
  Open { name: String },

  #[structopt(name = "link")]
  /// Link two notes
  Link { left: String, right: String },

  #[structopt(name = "find")]
  /// Find notes by tag
  Find { tag: String },

  #[structopt(name = "search")]
  /// Fuzzy search notes
  Search,

  #[structopt(name = "dir")]
  /// Output the Zettelkasten directory path
  Dir,

  #[structopt(name = "rm")]
  /// Remove a note
  Remove { name: String },

  #[structopt(name = "rmlink")]
  /// Remove a link between two notes
  RemoveLink { left: String, right: String },

  #[structopt(name = "tag")]
  /// Add a tag to a note
  Tag { name: String, tag: String },

  #[structopt(name = "rmtag")]
  /// Remove a tag from a note
  RemoveTag { name: String, tag: String },

  #[structopt(name = "explore")]
  /// Explore note links
  Explore { name: String },
}

impl Opt {
  pub fn run(self) -> Result<(), Error> {
    let config = Config::load()?;

    let handler = Handler::new(
      config.clone(),
      Directory::new(config.path.expand(), config.ext),
    );

    match self {
      Opt::New { name } => handler.create(&name)?,
      Opt::Open { name } => handler.open(&name)?,
      Opt::Link { left, right } => handler.link(&left, &right)?,
      Opt::Find { tag } => handler.find(&tag)?,
      Opt::Search => handler.search()?,
      Opt::Dir => handler.dir()?,
      Opt::Remove { name } => handler.remove(&name)?,
      Opt::RemoveLink { left, right } => handler.remove_link(&left, &right)?,
      Opt::Tag { name, tag } => handler.tag(&name, &tag)?,
      Opt::RemoveTag { name, tag } => handler.remove_tag(&name, &tag)?,
      Opt::Explore { name } => handler.explore(&name)?,
    }

    Ok(())
  }
}
