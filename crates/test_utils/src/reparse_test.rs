use super::{
    test::{Test, TestConfig},
    test_result::TestResult,
};
use parse::Parse;
use std::{fmt, marker::PhantomData};

pub struct ReparseTest<'a, T, Conf>
where
    T: Parse + fmt::Display,
    Conf: TestConfig,
{
    conf: &'a Conf,
    phantom: PhantomData<T>,
}

impl<'a, T, Conf> ReparseTest<'a, T, Conf>
where
    T: Parse + fmt::Display,
    Conf: TestConfig,
{
    pub fn new(conf: &'a Conf) -> ReparseTest<'a, T, Conf> {
        ReparseTest {
            conf,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, Conf> Test<'a> for ReparseTest<'a, T, Conf>
where
    T: Parse + fmt::Display,
    Conf: TestConfig,
{
    fn name(&self) -> String {
        format!("Reparsing {}", self.conf.name())
    }

    fn run(&self) -> TestResult {
        let parsed = match T::parse(self.conf.contents().to_owned()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        match T::parse(parsed.to_string()) {
            Ok(_) => TestResult::Success,
            Err(err) => TestResult::from_err(err),
        }
    }
}
