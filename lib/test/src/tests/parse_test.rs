use crate::{config::TestConfig, test_result::TestResult, tests::Test};
use parser::{GroupParse, Parse};
use std::marker::PhantomData;
use syntax::{language::Language, program::Program};

/// Test Parsing a program
/// Lang is the language ([`syntax::language::Language`]) of the program
pub struct ParseTest<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    /// The name of the program
    name: String,
    /// The source of the program
    contents: String,
    /// Phantom data saving the Lang type
    phantom: PhantomData<Lang>,
}

impl<Lang> ParseTest<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    /// Create a new parse test from a name and contents
    pub fn new(name: &str, contents: &str) -> ParseTest<Lang> {
        ParseTest {
            name: name.to_owned(),
            contents: contents.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<Lang> Test for ParseTest<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type Result = Program<Lang>;
    type Input = ();

    fn name(&self) -> String {
        format!("Parsing {}", self.name)
    }

    fn run(&self) -> TestResult<Program<Lang>> {
        match Program::<Lang>::parse(self.contents.clone()) {
            Ok(t) => TestResult::Success(t),
            Err(err) => TestResult::from_err(err),
        }
    }

    fn from_conf(conf: &TestConfig, _: Self::Input) -> Option<Self> {
        if !conf.include_parse() {
            None
        } else {
            Some(ParseTest {
                name: conf.name.clone(),
                contents: conf.contents.clone(),
                phantom: PhantomData,
            })
        }
    }
}
