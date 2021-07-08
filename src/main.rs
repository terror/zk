use crate::common::*;

#[macro_use]
mod test_utils;

mod common;
mod config;
mod directory;
mod error;
mod expand_path;
mod handler;
mod matter;
mod note;
mod note_id;
mod opt;
mod search;

fn main() {
  match Opt::from_args().run() {
    Ok(()) => {},
    Err(e) => eprintln!("{}", format!("{}", e).red()),
  };
}
