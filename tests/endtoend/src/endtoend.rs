use std::{
    env::{current_dir, set_current_dir},
    path::PathBuf,
};

mod errors;
mod paths;
mod test;

mod nameless;
mod stlc;
mod untyped_arithmetic;
mod untyped_lambda;

use errors::Error;
use nameless::NamelessRepTests;
use paths::{EXAMPLES_PATH, STLC_PATH, UNTYPED_ARITH_PATH, UNTYPED_LAMBDA_PATH};
use stlc::StlcTests;
use test::{Test, TestResult};
use untyped_arithmetic::UntypedArithTests;
use untyped_lambda::UntypedLambdaTests;

pub trait TestRunner {
    type TestConf: for<'a> serde::Deserialize<'a>;

    fn run_test(&self, test: Test<Self::TestConf>) -> TestResult;
    fn load_tests(&self) -> Result<Vec<Test<Self::TestConf>>, Error>;

    fn run_all_tests(&self) -> Result<usize, Error> {
        let tests = self.load_tests()?;
        let num_tests = tests.len();
        let mut num_fail = 0;
        for test in tests {
            let test_name = format!("{:?}", test.source_file);
            let result = self.run_test(test);
            if matches!(result, TestResult::Fail(_)) {
                num_fail += 1
            };
            result.report(&test_name)
        }
        println!("");
        println!(
            "Ran {} tests\n\t {} successul, {} fail",
            num_tests,
            num_tests - num_fail,
            num_fail
        );
        Ok(num_fail)
    }
}

fn setup() -> Result<(), Error> {
    let dir = current_dir()
        .map_err(|_| Error::GetCurrentDir)?
        .join("..")
        .join("..");
    set_current_dir(dir).map_err(|_| Error::SetCurrentDir)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);

    println!("Running tests for untyped arithmetic\n");
    let mut fails =
        UntypedArithTests::new(examples_dir.join(UNTYPED_ARITH_PATH)).run_all_tests()?;

    println!("Running tests for untyped lambda\n");
    fails += UntypedLambdaTests::new(examples_dir.join(UNTYPED_LAMBDA_PATH)).run_all_tests()?;

    println!("Running tests for locally nameless representation\n");
    fails += NamelessRepTests::new(examples_dir.join(UNTYPED_LAMBDA_PATH)).run_all_tests()?;

    println!("Running tests for stlc\n");
    fails += StlcTests::new(examples_dir.join(STLC_PATH)).run_all_tests()?;

    println!(
        "Finished running tests with \x1b[31m{} fails\x1b[39m",
        fails
    );
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
