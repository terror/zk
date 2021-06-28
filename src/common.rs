// stdlib
pub use std::{
  env,
  fs::{self, File},
  io::{self, prelude::*},
  path::{Path, PathBuf},
  process::Command,
};

// dependencies
pub use chrono::prelude::*;
pub use colored::*;
pub use dialoguer::{theme::ColorfulTheme, MultiSelect};
pub use serde::{Deserialize, Serialize};
pub use shellexpand;
pub use snafu::{ResultExt, Snafu};
pub use structopt::StructOpt;
pub use toml;
pub use walkdir::WalkDir;

// modules
pub(crate) use crate::error;

// test crates
#[cfg(test)]
pub use tempfile::TempDir;

// structs and enums
pub use crate::{config::Config, error::Error, handler::Handler, note::Note, opt::Opt};
