use std::fmt;

pub enum TestResult<T> {
    Success(T),
    Fail(String),
}

impl<T> TestResult<T> {
    pub fn from_err<Err>(err: Err) -> TestResult<T>
    where
        Err: fmt::Display,
    {
        TestResult::Fail(err.to_string())
    }

    pub fn from_eq<U>(result: T, expected: &U) -> TestResult<T>
    where
        T: fmt::Display,
        U: fmt::Display,
    {
        let res = result.to_string();
        let exp = expected.to_string();
        if res == exp {
            TestResult::Success(result)
        } else {
            TestResult::Fail(format!(
                "Result!=Expected:\n\tresult:   {res}\n\texpected: {exp}"
            ))
        }
    }

    pub fn report(&self, test_name: &str) {
        match self {
            TestResult::Success(_) => println!("Test {test_name}.....\x1b[32mok\x1b[39m"),
            TestResult::Fail(msg) => println!("Test {test_name}.....\x1b[31mfail\n\t{msg}\x1b[39m"),
        }
    }
}
