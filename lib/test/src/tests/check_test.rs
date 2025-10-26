use crate::{config::TestConfig, test_result::TestResult, tests::Test};
use check::Typecheck;
use derivations::ProgramDerivation;
use syntax::{language::Language, program::Program};

/// Tests type checking a program
/// The program is in the given languge ([`syntax::language::Language`])
pub struct CheckTest<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
{
    /// The name of the program
    name: String,
    /// The program to check
    prog: Program<Lang>,
    /// The expected type(s)
    expected: String,
}

impl<Lang> CheckTest<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
{
    /// Create a new check test from name, program and expected str
    pub fn new(name: &str, prog: Program<Lang>, exp: &str) -> CheckTest<Lang> {
        CheckTest {
            name: name.to_owned(),
            prog,
            expected: exp.to_owned(),
        }
    }
}

impl<Lang> Test for CheckTest<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
{
    type Result = ProgramDerivation<Lang>;
    type Input = Program<Lang>;

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

    fn from_conf(conf: &TestConfig, prog: Self::Input) -> Option<Self> {
        if !conf.include_check() {
            None
        } else {
            Some(CheckTest {
                name: conf.name.clone(),
                expected: conf.ty.clone(),
                prog,
            })
        }
    }
}
