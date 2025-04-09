use std::path::PathBuf;
use test_common::{
    errors::Error,
    paths::{EXAMPLES_PATH, LAMBDA_OMEGA_PATH},
    setup,
    testsuite::TestSuite,
};

mod suite;

use suite::LambdaOmegaTests;

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = LambdaOmegaTests::new(examples_dir.join(LAMBDA_OMEGA_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
