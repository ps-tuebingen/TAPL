use std::{
    env::{current_dir, set_current_dir},
    path::PathBuf,
};

mod paths;
mod test;
mod untyped_arithmetic;

use paths::{EXAMPLES_PATH, UNTYPED_ARITH_PATH};
use test::{Test, TestResult};
use untyped_arithmetic::UntypedArithTests;

pub trait TestRunner {
    fn run_test(&self, test: Test) -> TestResult;
    fn load_tests(&self) -> Result<Vec<Test>, Box<dyn std::error::Error>>;

    fn run_all_tests(&self) -> Result<(), Box<dyn std::error::Error>> {
        let tests = self.load_tests()?;
        for test in tests {
            let test_name = format!("{:?}", test.source_file);
            let result = self.run_test(test);
            result.report(&test_name)
        }
        Ok(())
    }
}

fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let dir = current_dir()?.join("..").join("..");
    set_current_dir(dir)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup()?;

    println!("Running tests for untyped arithmetic");
    UntypedArithTests::new(PathBuf::from(EXAMPLES_PATH).join(UNTYPED_ARITH_PATH))
        .run_all_tests()?;
    Ok(())
}
