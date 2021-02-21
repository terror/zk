mod common;
#[macro_use]
mod utils;
mod init;
mod new;

use crate::common::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "zk")]
enum Opt {
  #[structopt(name = "init")]
  /// Initialize a Zettelkasten directory
  Init { path: path::PathBuf },

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
}

fn main() -> Result<()> {
  match Opt::from_args() {
    Opt::Init { path } => init::init(path)?,
    Opt::New { name } => new::new(name)?,
    Opt::Open { name } => println!("{}", name),
    Opt::Link { left, right } => println!("{} <-> {}", left, right),
    Opt::Find { tag } => println!("{}", tag),
  }
  Ok(())
}
