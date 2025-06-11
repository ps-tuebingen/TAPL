use super::{
    test::{Test, TestConfig},
    test_result::TestResult,
};
use common::parse::Parse;
use eval::Eval;
use std::fmt;
use std::marker::PhantomData;

pub trait EvalConfig: TestConfig {
    fn expected(&self) -> &str;
}

pub struct EvalTest<'a, T, Conf>
where
    T: Eval + Parse,
    T::Value: fmt::Display,
    Conf: EvalConfig,
{
    conf: &'a Conf,
    phantom: PhantomData<T>,
}

impl<'a, T, Conf> EvalTest<'a, T, Conf>
where
    T: Eval + Parse,
    T::Value: fmt::Display,
    Conf: EvalConfig,
{
    pub fn new(conf: &'a Conf) -> EvalTest<'a, T, Conf> {
        EvalTest {
            conf,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, Conf> Test<'a> for EvalTest<'a, T, Conf>
where
    T: Eval + Parse,
    T::Value: fmt::Display,
    Conf: EvalConfig,
{
    fn name(&self) -> String {
        format!("Evaluating {}", self.conf.name())
    }

    fn run(&self) -> TestResult {
        let parsed = match T::parse(self.conf.contents().to_owned()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let evaled = match T::eval_start(parsed) {
            Ok(v) => v,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&evaled.val(), &self.conf.expected())
    }
}
