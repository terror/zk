pub(crate) use std::{
  borrow::Cow,
  ffi::OsStr,
  fmt::{self, Display, Formatter},
  fs::{self, File},
  io::{self, prelude::*},
  path::PathBuf,
  process::{self, Command},
  str,
  sync::Arc,
};

pub(crate) use {
  colored::Colorize,
  indoc::indoc,
  serde::{Deserialize, Serialize},
  skim::prelude::*,
  snafu::Snafu,
  structopt::StructOpt,
  walkdir::WalkDir,
};

pub(crate) use crate::{
  config::Config, directory::Directory, error::Error, handler::Handler,
  matter::Matter, note::Note, note_id::NoteId, opt::Opt, search::Search,
};

pub(crate) use crate::path_ext::PathExt;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
pub(crate) use {
  crate::test_utils::*,
  std::{env, thread, time},
  tempfile::TempDir,
};
