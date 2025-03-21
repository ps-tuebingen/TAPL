use std::path::PathBuf;
use test_common::{
    errors::Error,
    paths::{EXAMPLES_PATH, UNTYPED_ARITH_PATH},
    setup,
    testsuite::TestSuite,
};

mod eval_test;
mod parse_test;
mod reparse_test;
mod suite;

use eval_test::EvalTest;
use parse_test::ParseTest;
use reparse_test::ReparseTest;
use suite::UntypedArithTests;

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = UntypedArithTests::new(examples_dir.join(UNTYPED_ARITH_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
