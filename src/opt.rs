use crate::common::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "zk")]
pub enum Opt {
  #[structopt(name = "new")]
  /// Create a new Zettelkasten note
  New { name: String },

  #[structopt(name = "open")]
  /// Open an existing Zettelkasten note
  Open { name: String },

  #[structopt(name = "link")]
  /// Link two existing Zettelkasten notes
  Link { left: String, right: String },

  #[structopt(name = "find")]
  /// Find Zettelkasten notes by tag
  Find { tag: String },

  #[structopt(name = "search")]
  /// Fuzzy search Zettelkasten notes
  Search,

  #[structopt(name = "dir")]
  /// Zettelkasten storage location
  Dir,

  #[structopt(name = "preview")]
  /// Preview an existing Zettelkasten note in the terminal
  Preview { name: String },

  #[structopt(name = "remove")]
  /// Remove an existing Zettelkasten note
  Remove { name: String },
}

impl Opt {
  pub fn run(self) -> Result<(), Error> {
    let config = Config::load()?;

    let handler = Handler::new(config.editor, Directory::new(config.path.expand()));

    match self {
      Opt::New { name } => handler.create(&name)?,
      Opt::Open { name } => handler.open(&name)?,
      Opt::Link { left, right } => handler.link(&left, &right)?,
      Opt::Find { tag } => handler.find(&tag)?,
      Opt::Search => handler.search()?,
      Opt::Dir => handler.dir()?,
      Opt::Preview { name } => handler.preview(&name)?,
      Opt::Remove { name } => handler.remove(&name)?,
    }

    Ok(())
  }
}
