use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display("Unable to fetch base directory."))]
  BaseDirectories { source: xdg::BaseDirectoriesError },
  #[snafu(
    context(false),
    display("Failed to Deserialize TOML configuration file: {}", source)
  )]
  DeserializeConfig { source: toml::de::Error },
  #[snafu(display("Failed to load environment variable: {}.", var))]
  Env {
    source: env::VarError,
    var:    String,
  },
  #[snafu(context(false), display("IO Error: {}", source))]
  Io { source: io::Error },
  #[snafu(display("No note contains a link to `{}`.", name))]
  LinkNotFound { name: String },
  #[snafu(display("Unable to load configuration from {}: {}", path.display(), source))]
  LoadConfig { source: io::Error, path: PathBuf },
  #[snafu(display("Note with name `{}` does not exist.", name))]
  NoteNotFound { name: String },
  #[snafu(display("No note was selected."))]
  NoteNotSelected,
  #[snafu(display("Path `{}` does not exist.", path.display()))]
  Path { path: PathBuf },
  #[snafu(display("No note with tag `{}` exists.", tag))]
  TagNotFound { tag: String },
}
