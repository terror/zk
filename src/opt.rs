use crate::common::*;

#[derive(Debug, Parser)]
#[clap(
  about,
  author,
  version,
  help_template = "\
{before-help}{name} {version}
{author}
{about}

\x1b[1;4mUsage\x1b[0m: {usage}

{all-args}{after-help}
"
)]
pub(crate) enum Opt {
  #[clap(name = "dir", alias = "d")]
  /// Output the Zettelkasten directory path
  Dir,
  #[clap(name = "explore", alias = "e")]
  /// Explore note links
  Explore { name: String },
  #[clap(name = "find", alias = "f")]
  /// Find notes by tag
  Find { tag: String },
  #[clap(name = "link", alias = "l")]
  /// Link two notes
  Link { left: String, right: String },
  #[clap(name = "new", alias = "n")]
  /// Create a new note
  New { name: String },
  #[clap(name = "open", alias = "o")]
  /// Open a note
  Open { name: String },
  #[clap(name = "rm")]
  /// Remove a note
  Remove { name: String },
  #[clap(name = "rmtag", alias = "rt")]
  /// Remove a tag from a note
  RemoveTag { name: String, tag: String },
  #[clap(name = "rmlink", alias = "rl")]
  /// Remove a link between two notes
  RemoveLink { left: String, right: String },
  #[clap(name = "search", alias = "s")]
  /// Fuzzy search notes
  Search,
  #[clap(name = "tag", alias = "t")]
  /// Add a tag to a note
  Tag { name: String, tag: String },
}

impl Opt {
  pub(crate) fn run(self) -> Result<(), Error> {
    let config = Config::load()?;

    let handler =
      Handler::new(config.clone(), Directory::new(config.path.expand()));

    match self {
      Opt::Dir => handler.dir(),
      Opt::Explore { name } => handler.explore(&name)?,
      Opt::Find { tag } => handler.find(&tag)?,
      Opt::Link { left, right } => handler.link(&left, &right)?,
      Opt::New { name } => handler.create(&name)?,
      Opt::Open { name } => handler.open(&name)?,
      Opt::Remove { name } => handler.remove(&name)?,
      Opt::RemoveLink { left, right } => handler.remove_link(&left, &right)?,
      Opt::RemoveTag { name, tag } => handler.remove_tag(&name, &tag)?,
      Opt::Search => handler.search()?,
      Opt::Tag { name, tag } => handler.tag(&name, &tag)?,
    }

    Ok(())
  }
}
