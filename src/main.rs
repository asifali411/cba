use std::{fs, process::ExitCode};

use cba;

const TEST_PATH: &str = "./test/proj/.cba";

fn main() -> ExitCode {
  match fs::read_to_string(TEST_PATH) {
    Ok(source) => cba::run(source),
    Err(err) => {
      eprintln!("{}", err);
      ExitCode::FAILURE
    }
  }
}
