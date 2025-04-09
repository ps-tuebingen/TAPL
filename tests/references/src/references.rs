use std::path::PathBuf;
use test_common::{
    errors::Error,
    paths::{EXAMPLES_PATH, REFERENCES_PATH},
    setup,
    testsuite::TestSuite,
};

mod suite;
mod typecheck_test;

use suite::ReferencesTests;
use typecheck_test::TypecheckTest;

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = ReferencesTests::new(examples_dir.join(REFERENCES_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
