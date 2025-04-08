use std::path::PathBuf;
use test_common::{
    errors::Error,
    paths::{EXAMPLES_PATH, F_OMEGA_PATH},
    setup,
    testsuite::TestSuite,
};

mod check_test;
mod eval_test;
use check_test::TypecheckTest;
use eval_test::EvalTest;
mod suite;

use suite::FOmegaTests;

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = FOmegaTests::new(examples_dir.join(F_OMEGA_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
