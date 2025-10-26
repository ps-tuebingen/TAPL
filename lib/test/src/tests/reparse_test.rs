use crate::{config::TestConfig, test_result::TestResult, tests::Test};
use parser::{GroupParse, Parse};
use syntax::{language::Language, program::Program};

/// Reparse a program (i.e. print and parse again)
/// Lang is the language ([`syntax::language::Language`]) of the program
pub struct ReparseTest<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    /// The name of the program
    name: String,
    /// The program to reparse
    program: Program<Lang>,
}

impl<Lang> ReparseTest<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    /// Create a new reparse test from a given name and program
    pub fn new(name: &str, program: &Program<Lang>) -> ReparseTest<Lang> {
        ReparseTest {
            name: name.to_owned(),
            program: program.clone(),
        }
    }
}

impl<Lang> Test for ReparseTest<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type Result = ();
    type Input = Program<Lang>;

    fn name(&self) -> String {
        format!("Reparsing {}", self.name)
    }

    fn run(&self) -> TestResult<()> {
        match Program::<Lang>::parse(self.program.to_string()) {
            Ok(_) => TestResult::Success(()),
            Err(err) => TestResult::from_err(err),
        }
    }

    fn from_conf(conf: &TestConfig, program: Self::Input) -> Option<Self> {
        if !conf.include_reparse() {
            None
        } else {
            Some(ReparseTest {
                name: conf.name.clone(),
                program,
            })
        }
    }
}
