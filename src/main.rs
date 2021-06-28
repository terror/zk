use crate::common::*;

#[macro_use]
mod test_utils;

mod common;
mod config;
mod error;
mod handler;
mod opt;

fn main() {
  match Opt::from_args().run() {
    Ok(()) => {}
    Err(e) => eprintln!("{}", e),
  }
}
