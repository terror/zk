use crate::common::*;

#[macro_use]
mod test_utils;

mod common;
mod config;
mod error;
mod expand;
mod handler;
mod note;
mod opt;
mod part;

fn main() {
  match Opt::from_args().run() {
    Ok(()) => {}
    Err(e) => eprintln!("{}", e),
  }
}
