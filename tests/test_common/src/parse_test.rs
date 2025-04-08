use super::testsuite::{Test, TestResult};
use common::Parse;
use std::marker::PhantomData;

pub struct ParseTest<T>
where
    T: Parse,
{
    name: String,
    source: String,
    phantom: PhantomData<T>,
}

impl<T> ParseTest<T>
where
    T: Parse,
{
    pub fn new(name: &str, source: &str) -> ParseTest<T> {
        ParseTest {
            name: name.to_owned(),
            source: source.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<T> Test for ParseTest<T>
where
    T: Parse,
{
    fn name(&self) -> String {
        format!("Parsing {}", self.name)
    }

    fn run(&self) -> TestResult {
        match T::parse(self.source.clone()) {
            Ok(_) => TestResult::Success,
            Err(err) => TestResult::from_err(err),
        }
    }
}
