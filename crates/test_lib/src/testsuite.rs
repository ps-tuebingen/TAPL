use common::errors::Error;
use std::fmt;

pub enum TestResult {
    Success,
    Fail(String),
}

impl TestResult {
    pub fn from_err<T: fmt::Display>(err: T) -> TestResult {
        TestResult::Fail(err.to_string())
    }

    pub fn from_eq<T: fmt::Display, U: fmt::Display>(result: &T, expected: &U) -> TestResult {
        let res = result.to_string();
        let exp = expected.to_string();
        if res == exp {
            TestResult::Success
        } else {
            TestResult::Fail(format!(
                "Result!=Expected:\n\tresult:   {res}\n\texpected: {exp}"
            ))
        }
    }

    pub fn report(&self, test_name: &str) {
        match self {
            TestResult::Success => println!("Test {test_name}.....\x1b[32mok\x1b[39m"),
            TestResult::Fail(msg) => println!("Test {test_name}.....\x1b[31mfail\n\t{msg}\x1b[39m"),
        }
    }
}

pub trait Test {
    fn name(&self) -> String;
    fn run(&self) -> TestResult;
}

pub trait TestSuite {
    fn name(&self) -> String;
    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error>;

    fn run_all(&self) -> Result<usize, Error> {
        println!("Running Test Suite {}\n", self.name());
        let tests = self.load()?;
        let num_tests = tests.len();
        let mut num_fail = 0;
        for test in tests {
            let result = test.run();
            result.report(&test.name());
            if matches!(result, TestResult::Fail(_)) {
                num_fail += 1
            }
        }
        println!();
        println!(
            "\tRan {} tests: {} success, {} fail\n",
            num_tests,
            num_tests - num_fail,
            num_fail
        );
        Ok(num_fail)
    }
}
