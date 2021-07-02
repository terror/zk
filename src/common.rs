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
pub use chrono::prelude::*;
pub use colored::{Colorize, *};
pub use matter;
pub use serde::{Deserialize, Serialize};
pub use shellexpand;
pub use skim::prelude::*;
pub use snafu::{ResultExt, Snafu};
pub use structopt::StructOpt;
pub use toml;
pub use walkdir::WalkDir;
pub use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

// modules
pub(crate) use crate::error;

// test crates
#[cfg(test)]
pub use tempfile::TempDir;

// structs and enums
pub use crate::{
  config::Config, directory::Directory, error::Error, expand_path::Expand, handler::Handler,
  matter::Matter, note::Note, note_id::NoteId, opt::Opt, search::Search, search_item::SearchItem,
};
