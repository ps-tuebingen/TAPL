use super::{test::Test, test_result::TestResult};
use eval::{Eval, eval_main};
use syntax::{language::Language, program::Program, terms::Term, types::Type};
use trace::EvalTrace;

pub struct EvalTest<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
{
    name: String,
    expected: String,
    prog: Program<Lang>,
}

impl<Lang> EvalTest<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
{
    pub fn new(name: &str, prog: Program<Lang>, exp: &str) -> EvalTest<Lang> {
        EvalTest {
            name: name.to_owned(),
            prog,
            expected: exp.to_owned(),
        }
    }
}

impl<Lang> Test<EvalTrace<Lang>> for EvalTest<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
{
    fn name(&self) -> String {
        format!("Evaluating {}", self.name)
    }

    fn run(&self) -> TestResult<EvalTrace<Lang>> {
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
