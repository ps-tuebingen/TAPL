use crate::{config::TestConfig, test_result::TestResult, tests::Test};
use eval::{Eval, eval_main};
use syntax::{language::Language, program::Program};
use trace::EvalTrace;

/// Test evaluating a program (i.e. the main defintion)
/// The program is in the given languge ([`syntax::language::Language`])
pub struct EvalTest<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
{
    /// The name of the program
    name: String,
    /// The program to evaluate
    prog: Program<Lang>,
    /// The expected value
    expected: String,
}

impl<Lang> EvalTest<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
{
    /// Crate a new Eval test with given name program and expected value
    pub fn new(name: &str, prog: Program<Lang>, exp: &str) -> EvalTest<Lang> {
        EvalTest {
            name: name.to_owned(),
            prog,
            expected: exp.to_owned(),
        }
    }
}

impl<Lang> Test for EvalTest<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
{
    type Result = EvalTrace<Lang>;
    type Input = Program<Lang>;

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

    fn from_conf(conf: &TestConfig, prog: Self::Input) -> Option<Self> {
        if !conf.include_eval() {
            None
        } else {
            Some(EvalTest {
                name: conf.name.clone(),
                expected: conf.evaluated.clone(),
                prog,
            })
        }
    }
}
