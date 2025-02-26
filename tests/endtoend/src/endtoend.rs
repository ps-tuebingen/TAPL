use std::{
    env::{current_dir, set_current_dir},
    path::PathBuf,
};

mod paths;
mod test;
mod untyped_arithmetic;
mod untyped_lambda;

use paths::{EXAMPLES_PATH, UNTYPED_ARITH_PATH, UNTYPED_LAMBDA_PATH};
use test::{Test, TestResult};
use untyped_arithmetic::UntypedArithTests;
use untyped_lambda::UntypedLambdaTests;

pub trait TestRunner {
    fn run_test(&self, test: Test) -> TestResult;
    fn load_tests(&self) -> Result<Vec<Test>, Box<dyn std::error::Error>>;
    fn suite_name(&self) -> String;

    fn run_all_tests(&self) -> Result<usize, Box<dyn std::error::Error>> {
        println!("Running Tests for {}", self.suite_name());
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
            "Ran {} tests for {}\n\t {} successul, {} fail",
            num_tests,
            self.suite_name(),
            num_tests - num_fail,
            num_fail
        );
        Ok(num_fail)
    }
}

fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let dir = current_dir()?.join("..").join("..");
    set_current_dir(dir)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);

    println!("Running tests for untyped arithmetic\n");
    let mut fails =
        UntypedArithTests::new(examples_dir.join(UNTYPED_ARITH_PATH)).run_all_tests()?;

    println!("Running tests for untyped lambda\n");
    fails += UntypedLambdaTests::new(examples_dir.join(UNTYPED_LAMBDA_PATH)).run_all_tests()?;

    println!("Finished running tests with {} fails", fails);
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
