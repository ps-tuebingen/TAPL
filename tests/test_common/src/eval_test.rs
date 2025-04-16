use super::testsuite::{Test, TestResult};
use common::{Eval, Parse};
use std::fmt;
use std::marker::PhantomData;

pub struct EvalTest<T>
where
    T: Eval + Parse,
    T::Value: fmt::Display,
{
    name: String,
    source: String,
    expected: String,
    phantom: PhantomData<T>,
}

impl<T> EvalTest<T>
where
    T: Eval + Parse,
    T::Value: fmt::Display,
{
    pub fn new(name: &str, source: &str, exp: &str) -> EvalTest<T> {
        EvalTest {
            name: name.to_owned(),
            source: source.to_owned(),
            expected: exp.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Test for EvalTest<T>
where
    T: Eval + Parse,
    T::Value: fmt::Display,
{
    fn name(&self) -> String {
        format!("Evaluating {}", self.name)
    }

    fn run(&self) -> TestResult {
        let parsed = match T::parse(self.source.clone()) {
            Ok(p) => p,
            Err(err) => return TestResult::from_err(err),
        };
        let evaled = match T::eval_start(parsed) {
            Ok(v) => v,
            Err(err) => return TestResult::from_err(err),
        };
        TestResult::from_eq(&evaled, &self.expected)
    }
}
