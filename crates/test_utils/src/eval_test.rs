use super::{test::Test, test_result::TestResult};
use eval::Eval;
use std::fmt;
use syntax::terms::Term;
use trace::EvalTrace;

pub struct EvalTest<T>
where
    T: Term + Eval<Term = T>,
    T::Value: fmt::Display,
{
    name: String,
    expected: String,
    term: T,
}

impl<T> EvalTest<T>
where
    T: Term + Eval<Term = T>,
    T::Value: fmt::Display,
{
    pub fn new(name: &str, t: T, exp: &str) -> EvalTest<T> {
        EvalTest {
            name: name.to_owned(),
            term: t,
            expected: exp.to_owned(),
        }
    }
}

impl<T> Test<EvalTrace<T, T::Value>> for EvalTest<T>
where
    T: Term + Eval<Term = T>,
    T::Value: fmt::Display,
{
    fn name(&self) -> String {
        format!("Evaluating {}", self.name)
    }

    fn run(&self) -> TestResult<EvalTrace<T, T::Value>> {
        let evaled = match self.term.clone().eval_start() {
            Ok(v) => v,
            Err(err) => return TestResult::from_err(err),
        };
        let evaled_str = evaled.val().to_string();
        if evaled_str == self.expected {
            TestResult::Success(evaled)
        } else {
            TestResult::Fail(format!(
                "Result!=Expected:\n\tresult:   {evaled_str}\n\texpected: {}",
                self.expected
            ))
        }
    }
}
