use super::{test::Test, test_result::TestResult};
use parser::{GroupParse, Parse};
use std::marker::PhantomData;
use syntax::{program::Program, terms::Term, types::Type};

pub struct ReparseTest<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    name: String,
    content: String,
    phantom: PhantomData<Program<T, Ty>>,
}

impl<T, Ty> ReparseTest<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
{
    pub fn new(name: &str, prog: &Program<T, Ty>) -> ReparseTest<T, Ty> {
        ReparseTest {
            name: name.to_owned(),
            content: prog.to_string(),
            phantom: PhantomData,
        }
    }
}

impl<T, Ty> Test<()> for ReparseTest<T, Ty>
where
    T: Term + GroupParse,
    Ty: Type + GroupParse,
{
    fn name(&self) -> String {
        format!("Reparsing {}", self.name)
    }

    fn run(&self) -> TestResult<()> {
        match Program::<T, Ty>::parse(self.content.clone()) {
            Ok(_) => TestResult::Success(()),
            Err(err) => TestResult::from_err(err),
        }
    }
}
