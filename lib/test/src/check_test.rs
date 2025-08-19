use super::{test::Test, test_result::TestResult};
use check::Typecheck;
use derivations::ProgramDerivation;
use std::marker::PhantomData;
use syntax::{program::Program, terms::Term, types::TypeGroup};

pub struct CheckTest<T, Ty>
where
    T: Term + Typecheck<Term = T, Type = Ty>,
    Ty: TypeGroup,
{
    name: String,
    prog: Program<T, Ty>,
    expected: String,
    phantom: PhantomData<T>,
}

impl<T, Ty> CheckTest<T, Ty>
where
    T: Term + Typecheck<Term = T, Type = Ty>,
    Ty: TypeGroup,
{
    pub fn new(name: &str, prog: Program<T, Ty>, exp: &str) -> CheckTest<T, Ty> {
        CheckTest {
            name: name.to_owned(),
            prog,
            expected: exp.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<T, Ty> Test<ProgramDerivation<T, T::Type>> for CheckTest<T, Ty>
where
    T: Term + Typecheck<Term = T, Type = Ty>,
    Ty: TypeGroup,
{
    fn name(&self) -> String {
        format!("Checking {}", self.name)
    }

    fn run(&self) -> TestResult<ProgramDerivation<T, Ty>> {
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
}
