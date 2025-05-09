use std::path::PathBuf;
use test_common::{
    errors::Error,
    paths::{EXAMPLES_PATH, UNTYPED_LAMBDA_PATH},
    setup,
    testsuite::TestSuite,
};

mod suite;

use suite::UntypedLambdaTests;

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = UntypedLambdaTests::new(examples_dir.join(UNTYPED_LAMBDA_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
