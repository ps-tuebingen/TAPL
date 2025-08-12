use super::{test::Test, test_result::TestResult};
use parser::{GroupParse, Parse};
use std::marker::PhantomData;
use syntax::{program::Program, terms::Term, types::Type};

pub struct ParseTest<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    name: String,
    contents: String,
    phantom: PhantomData<Program<T, Ty>>,
}

impl<T, Ty> ParseTest<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    pub fn new(name: &str, contents: &str) -> ParseTest<T, Ty> {
        ParseTest {
            name: name.to_owned(),
            contents: contents.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<T, Ty> Test<Program<T, Ty>> for ParseTest<T, Ty>
where
    T: Term + GroupParse,
    Ty: Type + GroupParse,
{
    fn name(&self) -> String {
        format!("Parsing {}", self.name)
    }

    fn run(&self) -> TestResult<Program<T, Ty>> {
        match Program::<T, Ty>::parse(self.contents.clone()) {
            Ok(t) => TestResult::Success(t),
            Err(err) => TestResult::from_err(err),
        }
    }
}
