use std::env::{current_dir, set_current_dir};

pub mod errors;
pub mod eval_test;
pub mod load_tests;
pub mod parse_test;
pub mod paths;
pub mod reparse_test;
pub mod testsuite;

use errors::Error;

pub fn setup() -> Result<(), Error> {
    let dir = current_dir()
        .map_err(|_| Error::GetCurrentDir)?
        .join("..")
        .join("..");
    set_current_dir(dir).map_err(|_| Error::SetCurrentDir)?;
    Ok(())
}
