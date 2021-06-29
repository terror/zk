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

  #[snafu(display("Unable to fetch base directory."))]
  BaseDirectoriesError {
    source: xdg::BaseDirectoriesError,
  },

  #[snafu(display("Failed to load environment variable: {}.", var))]
  Env {
    source: env::VarError,
    var:    String,
  },

  #[snafu(display("Failed to parse frontmatter for note with name: {}", name))]
  FrontmatterError {
    source: yaml_rust::ScanError,
    name:   String,
  },

  Io {
    source: io::Error,
  },
}
