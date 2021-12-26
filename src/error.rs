use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(context(false), display("Unable to fetch base directory: {}", source))]
  BaseDirectories { source: xdg::BaseDirectoriesError },

  #[snafu(display("Failed to send `SkimItem` over crossbeam_channel"))]
  ChannelSend,

  #[snafu(
    context(false),
    display("Failed to Deserialize TOML configuration file: {}", source)
  )]
  DeserializeConfig { source: toml::de::Error },

  #[snafu(display("Invalid note id: {}", id))]
  InvalidNoteId { id: String },

  #[snafu(context(false), display("IO Error: {}", source))]
  Io { source: io::Error },

  #[snafu(display("Note already contains a link to `{}`", link))]
  LinkExists { link: String },

  #[snafu(display("Link `{}` does not exist on note `{}`", link, name))]
  LinkMissing { link: String, name: String },

  #[snafu(display("Note with name `{}` does not exist", name))]
  NoteNotFound { name: String },

  #[snafu(display("No note was selected"))]
  NoteNotSelected,

  #[snafu(display("Error building `skim` options"))]
  SkimOptions,

  #[snafu(display("Note already contains the tag `{}`", tag))]
  TagExists { tag: String },

  #[snafu(display("Tag `{}` does not exist on note `{}`", tag, name))]
  TagMissing { tag: String, name: String },

  #[snafu(display("No note with tag `{}` exists", tag))]
  TagNotFound { tag: String },

  #[snafu(context(false), display("Walkdir Error: {}", source))]
  Walkdir { source: walkdir::Error },

  #[snafu(context(false), display("YAML se/de error: {}", source))]
  Yaml { source: serde_yaml::Error },
}
