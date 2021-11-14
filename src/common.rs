// stdlib
pub use std::{
  borrow::Cow,
  env,
  ffi::OsStr,
  fmt,
  fs::{self, File},
  io::{self, prelude::*, Cursor},
  path::{Path, PathBuf},
  process::Command,
  str,
  sync::Arc,
  thread, time,
};

// dependencies
pub use {
  chrono::prelude::*,
  colored::{Colorize, *},
  matter,
  serde::{Deserialize, Serialize},
  shellexpand,
  skim::prelude::*,
  snafu::{ResultExt, Snafu},
  structopt::StructOpt,
  toml,
  walkdir::WalkDir,
  yaml_rust::{Yaml, YamlEmitter, YamlLoader},
};

// modules
pub(crate) use crate::error;

// test crates
#[cfg(test)]
pub use {crate::test_utils::*, tempfile::TempDir, textwrap::dedent};

// structs and enums
pub use crate::{
  config::Config, directory::Directory, error::Error, expand_path::Expand, handler::Handler,
  matter::Matter, note::Note, note_id::NoteId, opt::Opt, search::Search,
};
