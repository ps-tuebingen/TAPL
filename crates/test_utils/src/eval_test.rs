use super::{
    test::{Test, TestConfig},
    test_result::TestResult,
};
use eval::Eval;
use parse::Parse;
use std::fmt;
use std::marker::PhantomData;

pub struct EvalTest<'a, T, Conf>
where
    T: Eval + Parse,
    <T as Parse>::LeftRecArg: Default,
    T::Value: fmt::Display,
    Conf: TestConfig,
{
    conf: &'a Conf,
    phantom: PhantomData<T>,
}

impl<'a, T, Conf> EvalTest<'a, T, Conf>
where
    T: Eval + Parse,
    <T as Parse>::LeftRecArg: Default,
    T::Value: fmt::Display,
    Conf: TestConfig,
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
    <T as Parse>::LeftRecArg: Default,
    T::Value: fmt::Display,
    Conf: TestConfig,
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
        TestResult::from_eq(&evaled.val(), &self.conf.evaluated())
    }
}
