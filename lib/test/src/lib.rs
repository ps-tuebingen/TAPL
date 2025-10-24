pub mod config;
pub mod paths;
pub mod test;
pub mod test_result;
pub mod testsuite;

pub mod check_test;
pub mod eval_test;
pub mod latex;
pub mod parse_test;
pub mod reparse_test;

use errors::{DirAccess, test_error::TestError};

use std::env::{current_dir, set_current_dir};

pub fn setup() -> Result<(), TestError> {
    println!("{:?}", current_dir().unwrap());
    let dir = current_dir()
        .map_err(|err| DirAccess::new("get current dir", err))?
        .join("..");
    set_current_dir(dir).map_err(|err| DirAccess::new("set current dir", err))?;
    Ok(())
}
