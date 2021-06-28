use crate::common::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display("Unable to load configuration from {}: {}", path.display(), source))]
  LoadConfig { source: io::Error, path: PathBuf },

  #[snafu(display("Unable to fetch base directory."))]
  BaseDirectoriesError { source: xdg::BaseDirectoriesError },

  #[snafu(display("Failed to load environment variable: {}.", var))]
  Env {
    source: env::VarError,
    var:    String,
  },

  #[snafu(display("Unable to read $EDITOR"))]
  Editor { source: std::env::VarError },
}
