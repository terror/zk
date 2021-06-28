use crate::common::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "zk")]
pub enum Opt {
  #[structopt(name = "init")]
  /// Initialize a Zettelkasten directory
  Init { path: Option<PathBuf> },

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
}

impl Opt {
  pub fn run(self) -> Result<(), Error> {
    let config = Config::load()?;

    match self {
      Opt::Init { path } => Handler::init(&path)?,
      Opt::New { name } => Handler::new(&name, config)?,
      Opt::Open { name } => Handler::open(&name)?,
      Opt::Link { left, right } => println!("{} <-> {}", left, right),
      Opt::Find { tag } => println!("{}", tag),
      Opt::Search => println!("search!"),
    }

    Ok(())
  }
}
