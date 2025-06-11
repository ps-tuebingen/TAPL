use super::{
    test::{Test, TestConfig},
    test_result::TestResult,
};
use check::Typecheck;
use common::parse::Parse;
use std::{fmt, marker::PhantomData};

pub trait CheckConfig: TestConfig {
    fn expected(&self) -> &str;
}

pub struct CheckTest<'a, T, Conf>
where
    T: Parse + Typecheck,
    T::Type: fmt::Display,
    Conf: CheckConfig,
{
    conf: &'a Conf,
    phantom: PhantomData<T>,
}

impl<'a, T, Conf> CheckTest<'a, T, Conf>
where
    T: Parse + Typecheck,
    T::Type: fmt::Display,
    Conf: CheckConfig,
{
    pub fn new(conf: &'a Conf) -> CheckTest<'a, T, Conf> {
        CheckTest {
            conf,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, Conf> Test<'a> for CheckTest<'a, T, Conf>
where
    T: Parse + Typecheck,
    T::Type: fmt::Display,
    Conf: CheckConfig,
{
    fn name(&self) -> String {
        format!("Checking {}", self.conf.name())
    }

    fn run(&self) -> TestResult {
        let parsed = match T::parse(self.conf.contents().to_owned()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let checked = match parsed.check_start() {
            Ok(c) => c,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&checked.ty(), &self.conf.expected())
    }
}
