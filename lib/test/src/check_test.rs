use super::{test::Test, test_result::TestResult};
use check::Typecheck;
use derivations::ProgramDerivation;
use syntax::{language::Language, program::Program};

pub struct CheckTest<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
{
    name: String,
    prog: Program<Lang>,
    expected: String,
}

impl<Lang> CheckTest<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
{
    pub fn new(name: &str, prog: Program<Lang>, exp: &str) -> CheckTest<Lang> {
        CheckTest {
            name: name.to_owned(),
            prog,
            expected: exp.to_owned(),
        }
    }
}

impl<Lang> Test<ProgramDerivation<Lang>> for CheckTest<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
{
    fn name(&self) -> String {
        format!("Checking {}", self.name)
    }

    fn run(&self) -> TestResult<ProgramDerivation<Lang>> {
        let checked = match self.prog.check_start() {
            Ok(c) => c,
            Err(err) => return TestResult::from_err(err),
        };
        let checked_prog = match checked.into_prog() {
            Ok(prog) => prog,
            Err(err) => return TestResult::from_err(err),
        };

        let checked_str = checked_prog.main_derivation.ret_ty().to_string();
        if checked_str == self.expected {
            TestResult::Success(checked_prog)
        } else {
            TestResult::Fail(format!(
                "Result!=Expected:\n\tresult:   {checked_str}\n\texpected: {}",
                self.expected
            ))
        }
    }
}
