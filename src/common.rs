// stdlib
pub(crate) use std::{
  borrow::Cow,
  ffi::OsStr,
  fmt::{self, Display, Formatter},
  fs::{self, File},
  io::{self, prelude::*},
  path::{Path, PathBuf},
  process::{self, Command},
  str,
  sync::Arc,
};

// dependencies
pub(crate) use {
  colored::Colorize, indoc::indoc, serde::Deserialize, skim::prelude::*, snafu::Snafu,
  structopt::StructOpt, walkdir::WalkDir, yaml_rust::YamlLoader,
};

// structs and enums
pub(crate) use crate::{
  config::Config, directory::Directory, error::Error, expand_path::Expand, handler::Handler,
  matter::Matter, note::Note, note_id::NoteId, opt::Opt, search::Search,
};

// type aliases
pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

// test dependencies
#[cfg(test)]
pub(crate) use {
  crate::test_utils::*,
  std::{thread, time, env},
  tempfile::TempDir,
  textwrap::dedent,
};
