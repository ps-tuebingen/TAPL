use super::{
    test::{Test, TestConfig},
    test_result::TestResult,
};
use common::parse::Parse;
use std::marker::PhantomData;

pub struct ParseTest<'a, T, Conf>
where
    T: Parse,
    Conf: TestConfig,
{
    conf: &'a Conf,
    phantom: PhantomData<T>,
}

impl<'a, T, Conf> ParseTest<'a, T, Conf>
where
    T: Parse,
    Conf: TestConfig,
{
    pub fn new(conf: &'a Conf) -> ParseTest<'a, T, Conf> {
        ParseTest {
            conf,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, Conf> Test<'a> for ParseTest<'a, T, Conf>
where
    T: Parse,
    Conf: TestConfig,
{
    fn name(&self) -> String {
        format!("Parsing {}", self.conf.name())
    }

    fn run(&self) -> TestResult {
        match T::parse(self.conf.contents().to_owned()) {
            Ok(_) => TestResult::Success,
            Err(err) => TestResult::from_err(err),
        }
    }
}
