use super::{test::Test, test_result::TestResult};
use parser::{GroupParse, Parse};
use std::marker::PhantomData;
use syntax::{language::Language, program::Program};

pub struct ReparseTest<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    name: String,
    content: String,
    phantom: PhantomData<Program<Lang>>,
}

impl<Lang> ReparseTest<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    pub fn new(name: &str, prog: &Program<Lang>) -> ReparseTest<Lang> {
        ReparseTest {
            name: name.to_owned(),
            content: prog.to_string(),
            phantom: PhantomData,
        }
    }
}

impl<Lang> Test<()> for ReparseTest<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn name(&self) -> String {
        format!("Reparsing {}", self.name)
    }

    fn run(&self) -> TestResult<()> {
        match Program::<Lang>::parse(self.content.clone()) {
            Ok(_) => TestResult::Success(()),
            Err(err) => TestResult::from_err(err),
        }
    }
}
