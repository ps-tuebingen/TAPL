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
