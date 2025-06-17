use super::{
    test::{Test, TestConfig},
    test_result::TestResult,
};
use check::Typecheck;
use parse::Parse;
use std::{fmt, marker::PhantomData};

pub struct CheckTest<'a, T, Conf>
where
    T: Parse + Typecheck,
    <T as Parse>::LeftRecArg: Default,
    T::Type: fmt::Display,
    Conf: TestConfig,
{
    conf: &'a Conf,
    phantom: PhantomData<T>,
}

impl<'a, T, Conf> CheckTest<'a, T, Conf>
where
    T: Parse + Typecheck,
    <T as Parse>::LeftRecArg: Default,
    T::Type: fmt::Display,
    Conf: TestConfig,
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
    <T as Parse>::LeftRecArg: Default,
    T::Type: fmt::Display,
    Conf: TestConfig,
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
        TestResult::from_eq(&checked.ty(), &self.conf.ty())
    }
}
