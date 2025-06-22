use super::{test::Test, test_result::TestResult};
use parse::Parse;
use std::marker::PhantomData;

pub struct ParseTest<T>
where
    T: Parse,
{
    name: String,
    contents: String,
    phantom: PhantomData<T>,
}

impl<T> ParseTest<T>
where
    T: Parse,
{
    pub fn new(name: &str, contents: &str) -> ParseTest<T> {
        ParseTest {
            name: name.to_owned(),
            contents: contents.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<T> Test<T> for ParseTest<T>
where
    T: Parse<LeftRecArg = ()>,
{
    fn name(&self) -> String {
        format!("Parsing {}", self.name)
    }

    fn run(&self) -> TestResult<T> {
        match T::parse(self.contents.clone()) {
            Ok(t) => TestResult::Success(t),
            Err(err) => TestResult::from_err(err),
        }
    }
}
