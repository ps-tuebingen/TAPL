use super::{test::Test, test_result::TestResult};
use check::Typecheck;
use derivation::Derivation;
use std::{fmt, marker::PhantomData};
use syntax::terms::Term;

pub struct CheckTest<T>
where
    T: Term + Typecheck<Term = T>,
    T::Type: fmt::Display,
{
    name: String,
    term: T,
    expected: String,
    phantom: PhantomData<T>,
}

impl<T> CheckTest<T>
where
    T: Term + Typecheck<Term = T>,
    T::Type: fmt::Display,
{
    pub fn new(name: &str, term: T, exp: &str) -> CheckTest<T> {
        CheckTest {
            name: name.to_owned(),
            term,
            expected: exp.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<T> Test<Derivation<T, T::Type>> for CheckTest<T>
where
    T: Term + Typecheck<Term = T>,
    T::Type: fmt::Display,
{
    fn name(&self) -> String {
        format!("Checking {}", self.name)
    }

    fn run(&self) -> TestResult<Derivation<T, T::Type>> {
        let checked = match self.term.check_start() {
            Ok(c) => c,
            Err(err) => return TestResult::from_err(err),
        };
        let checked_str = checked.ty().to_string();
        if checked_str == self.expected {
            TestResult::Success(checked)
        } else {
            TestResult::Fail(format!(
                "Result!=Expected:\n\tresult:   {checked_str}\n\texpected: {}",
                self.expected
            ))
        }
    }
}
