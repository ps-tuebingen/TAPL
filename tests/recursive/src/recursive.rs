use std::path::PathBuf;
use test_common::{
    errors::Error,
    paths::{EXAMPLES_PATH, RECURSIVE_PATH},
    setup,
    testsuite::TestSuite,
};

//mod check_test;
//mod eval_test;
//mod parse_test;
//mod reparse_test;
//use check_test::TypecheckTest;
//use eval_test::EvalTest;
//use parse_test::ParseTest;
//use reparse_test::ReparseTest;
mod suite;

use suite::RecursiveTests;

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = RecursiveTests::new(examples_dir.join(RECURSIVE_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
