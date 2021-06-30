use crate::common::*;

#[allow(dead_code)]
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display("Unable to load configuration from {}: {}", path.display(), source))]
  LoadConfig {
    source: io::Error,
    path:   PathBuf,
  },

  #[snafu(display(
    "Failed to Deserialize TOML configuration located at `{}`. Are you missing a configuration \
     option?", path.display()
  ))]
  DeserializeConfig {
    source: toml::de::Error,
    path:   PathBuf,
  },

  #[snafu(display("Unable to fetch base directory."))]
  BaseDirectoriesError {
    source: xdg::BaseDirectoriesError,
  },

  #[snafu(display("Failed to load environment variable: {}.", var))]
  Env {
    source: env::VarError,
    var:    String,
  },

  #[snafu(display("Note with name `{}` does not exist.", name))]
  NoteNotFound {
    name: String,
  },

  #[snafu(display("No note with tag `{}` exists.", tag))]
  TagNotFound {
    tag: String,
  },

  #[snafu(display("No note contains a link to `{}`.", name))]
  LinkNotFound {
    name: String,
  },

  #[snafu(display("No note was selected."))]
  NoteNotSelected,

  #[snafu(display("Path `{}` does not exist.", path.display()))]
  PathError {
    path: PathBuf,
  },

  Io {
    source: io::Error,
  },
}
