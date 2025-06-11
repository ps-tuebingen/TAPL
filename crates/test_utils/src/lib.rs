pub mod errors;
pub mod load_tests;
pub mod paths;
pub mod testsuite;

pub mod check_test;
pub mod eval_test;
pub mod latex_buss_test;
pub mod latex_frac_test;
pub mod latex_trace_test;
pub mod parse_test;
pub mod reparse_test;

use errors::Error;

use std::env::{current_dir, set_current_dir};

pub fn setup() -> Result<(), Error> {
    println!("{:?}", current_dir().unwrap());
    let dir = current_dir()
        .map_err(|err| Error::dir_access("get current dir", err))?
        .join("..");
    set_current_dir(dir).map_err(|err| Error::dir_access("set current dir", err))?;
    Ok(())
}
