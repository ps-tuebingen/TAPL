use super::testsuite::{Test, TestResult};
use check::Typecheck;
use common::parse::Parse;
use std::{fmt, marker::PhantomData};

pub struct CheckTest<T>
where
    T: Parse + Typecheck,
    T::Type: fmt::Display,
{
    name: String,
    source: String,
    expected: String,
    phantom: PhantomData<T>,
}

impl<T> CheckTest<T>
where
    T: Parse + Typecheck,
    T::Type: fmt::Display,
{
    pub fn new(name: &str, source: &str, exp: &str) -> CheckTest<T> {
        CheckTest {
            name: name.to_owned(),
            source: source.to_owned(),
            expected: exp.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<T> Test for CheckTest<T>
where
    T: Parse + Typecheck,
    T::Type: fmt::Display,
{
    fn name(&self) -> String {
        format!("Checking {}", self.name)
    }

    fn run(&self) -> TestResult {
        let parsed = match T::parse(self.source.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let checked = match parsed.check_start() {
            Ok(c) => c,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&checked.ty(), &self.expected)
    }
}
