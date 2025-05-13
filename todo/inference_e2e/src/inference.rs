use std::path::PathBuf;
use test_common::{
    errors::Error,
    paths::{EXAMPLES_PATH, INFERENCE_PATH},
    setup,
    testsuite::TestSuite,
};

mod bidirectional_test;
mod infer_test;
mod suite;
use bidirectional_test::BidirectionalTest;
use infer_test::InferTest;

use suite::InferenceTests;

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = InferenceTests::new(examples_dir.join(INFERENCE_PATH)).run_all()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
