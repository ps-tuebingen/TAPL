use super::{test::Test, test_result::TestResult};
use parser::{GroupParse, Parse};
use std::marker::PhantomData;
use syntax::{language::Language, program::Program};

pub struct ParseTest<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    name: String,
    contents: String,
    phantom: PhantomData<Program<Lang>>,
}

impl<Lang> ParseTest<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    pub fn new(name: &str, contents: &str) -> ParseTest<Lang> {
        ParseTest {
            name: name.to_owned(),
            contents: contents.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<Lang> Test<Program<Lang>> for ParseTest<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn name(&self) -> String {
        format!("Parsing {}", self.name)
    }

    fn run(&self) -> TestResult<Program<Lang>> {
        match Program::<Lang>::parse(self.contents.clone()) {
            Ok(t) => TestResult::Success(t),
            Err(err) => TestResult::from_err(err),
        }
    }
}
