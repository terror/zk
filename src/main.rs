use crate::common::*;

#[macro_use]
mod test_utils;

mod common;
mod config;
mod directory;
mod error;
mod expand_path;
mod handler;
mod note;
mod note_id;
mod opt;
mod prompt;

fn main() {
  match Opt::from_args().run() {
    Ok(()) => {},
    Err(e) => eprintln!("{}", e),
  }
}
