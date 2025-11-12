pub mod config;
pub mod paths;
pub mod test_result;
pub mod tests;
pub mod testsuite;

use errors::{DirAccess, test_error::TestError};

use std::env::{current_dir, set_current_dir};

pub fn setup() -> Result<(), TestError> {
    let dir = current_dir()
        .map_err(|err| DirAccess::new("get current dir", err))?
        .join("..")
        .join("..");
    set_current_dir(dir).map_err(|err| DirAccess::new("set current dir", err))?;
    Ok(())
}
