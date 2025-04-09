use std::path::PathBuf;
use test_common::{
    errors::Error,
    paths::{EXAMPLES_PATH, STLC_PATH},
    setup,
    testsuite::TestSuite,
};

mod check_test;
mod eval_ctx_test;
mod suite;

use check_test::TypecheckTest;
use eval_ctx_test::EvalCtxTest;
use suite::StlcTests;

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = StlcTests::new(examples_dir.join(STLC_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
