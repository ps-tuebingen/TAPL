use std::path::PathBuf;
use test_common::{
    errors::Error,
    paths::{EXAMPLES_PATH, UNTYPED_LAMBDA_PATH},
    setup,
    testsuite::TestSuite,
};

mod remove_restore_test;
mod remove_test;
mod restore_test;
mod suite;

use remove_restore_test::RemoveRestoreTest;
use remove_test::RemoveNamesTest;
use restore_test::RestoreNamesTest;
use suite::NamelessRepTests;

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = NamelessRepTests::new(examples_dir.join(UNTYPED_LAMBDA_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
