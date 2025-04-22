use super::testsuite::{Test, TestResult};
use common::Parse;
use std::{fmt, marker::PhantomData};

pub struct ReparseTest<T>
where
    T: Parse + fmt::Display,
{
    name: String,
    source: String,
    phantom: PhantomData<T>,
}

impl<T> ReparseTest<T>
where
    T: Parse + fmt::Display,
{
    pub fn new(name: &str, source: &str) -> ReparseTest<T> {
        ReparseTest {
            name: name.to_owned(),
            source: source.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<T> Test for ReparseTest<T>
where
    T: Parse + fmt::Display,
{
    fn name(&self) -> String {
        format!("Reparsing {}", self.name)
    }

    fn run(&self) -> TestResult {
        let parsed = match T::parse(self.source.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        match T::parse(parsed.to_string()) {
            Ok(_) => TestResult::Success,
            Err(err) => TestResult::from_err(err),
        }
    }
}
