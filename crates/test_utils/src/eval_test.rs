use super::{test::Test, test_result::TestResult};
use eval::{eval_main, Eval};
use syntax::{program::Program, terms::Term, types::Type};
use trace::EvalTrace;

pub struct EvalTest<T, Ty>
where
    T: Term + Eval<Term = T>,
    Ty: Type,
{
    name: String,
    expected: String,
    prog: Program<T, Ty>,
}

impl<T, Ty> EvalTest<T, Ty>
where
    T: Term + Eval<Term = T>,
    Ty: Type,
{
    pub fn new(name: &str, prog: Program<T, Ty>, exp: &str) -> EvalTest<T, Ty> {
        EvalTest {
            name: name.to_owned(),
            prog,
            expected: exp.to_owned(),
        }
    }
}

impl<T, Ty> Test<EvalTrace<T, T::Value>> for EvalTest<T, Ty>
where
    T: Term + Eval<Term = T>,
    Ty: Type,
{
    fn name(&self) -> String {
        format!("Evaluating {}", self.name)
    }

    fn run(&self) -> TestResult<EvalTrace<T, T::Value>> {
        let evaled = match eval_main(self.prog.clone()) {
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
