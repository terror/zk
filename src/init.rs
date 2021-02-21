use crate::common::*;

pub fn init(path: path::PathBuf) {
  println!("{}", &path.to_str().unwrap());
}
