use crate::common::*;

#[cfg(test)]
#[macro_use]
mod test_utils;

mod common;
mod config;
mod directory;
mod error;
mod handler;
mod matter;
mod note;
mod note_id;
mod opt;
mod path_ext;
mod search;

fn main() {
  if let Err(error) = Opt::from_args().run() {
    eprintln!("{}: {error}", "error".red());
    process::exit(1);
  }
}
