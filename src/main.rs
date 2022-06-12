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
  match Opt::from_args().run() {
    Ok(()) => {}
    Err(e) => {
      eprintln!("{}: {}", "error".red(), e);
      process::exit(1);
    }
  }
}
