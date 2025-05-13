pub mod load_tests;
pub mod paths;
pub mod testsuite;

pub mod check_test;
pub mod eval_test;
pub mod parse_test;
pub mod reparse_test;

use common::errors::{Error, ErrorKind, ErrorLocation};
use std::env::{current_dir, set_current_dir};

pub fn to_test_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Testing,
    }
}

pub fn setup() -> Result<(), Error> {
    println!("{:?}", current_dir().unwrap());
    let dir = current_dir()
        .map_err(|err| {
            to_test_err(ErrorKind::DirAccess {
                tried: "Get Current Dir".to_owned(),
                msg: err.to_string(),
            })
        })?
        .join("..");
    set_current_dir(dir).map_err(|err| {
        to_test_err(ErrorKind::DirAccess {
            tried: "Set Current Dir".to_owned(),
            msg: err.to_string(),
        })
    })?;
    Ok(())
}
