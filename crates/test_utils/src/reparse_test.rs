use super::{test::Test, test_result::TestResult};
use parse::Parse;
use std::{fmt, marker::PhantomData};

pub struct ReparseTest<T>
where
    T: Parse + fmt::Display,
{
    name: String,
    content: String,
    phantom: PhantomData<T>,
}

impl<T> ReparseTest<T>
where
    T: Parse + fmt::Display,
{
    pub fn new(name: &str, t: &T) -> ReparseTest<T> {
        ReparseTest {
            name: name.to_owned(),
            content: t.to_string(),
            phantom: PhantomData,
        }
    }
}

impl<T> Test<()> for ReparseTest<T>
where
    T: Parse<LeftRecArg = ()> + fmt::Display,
{
    fn name(&self) -> String {
        format!("Reparsing {}", self.name)
    }

    fn run(&self) -> TestResult<()> {
        match T::parse(self.content.clone()) {
            Ok(_) => TestResult::Success(()),
            Err(err) => TestResult::from_err(err),
        }
    }
}
