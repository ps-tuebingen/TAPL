pub mod values;
use super::to_err;
use crate::{syntax::Term, types::Type};
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    eval::Eval,
    terms::{False, Num, True, Unit, Variable},
};
use values::Value;

pub fn to_eval_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Eval)
}

impl Eval<Value, (), Term, Type> for Term {
    fn eval_start(self) -> Result<Value, Error> {
        self.eval(&mut ())
    }

    fn eval(self, env: &mut ()) -> Result<Value, Error> {
        match self {
            Term::Var(v) => <Variable<Term> as Eval<Value, (), Term, Type>>::eval(v, env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Unit(u) => <Unit<Term> as Eval<Value, (), Term, Type>>::eval(u, env),
            Term::True(tru) => <True<Term> as Eval<Value, (), Term, Type>>::eval(tru, env),
            Term::False(fls) => <False<Term> as Eval<Value, (), Term, Type>>::eval(fls, env),
            Term::Succ(s) => s.eval(env),
            Term::Pred(p) => p.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::IsZero(isz) => isz.eval(env),
            Term::Num(num) => <Num<Term> as Eval<Value, (), Term, Type>>::eval(num, env),
            Term::Exception(exc) => exc.eval(env),
            Term::Try(tr) => tr.eval(env),
            Term::TryWithVal(tr) => tr.eval(env),
            Term::Raise(r) => r.eval(env),
        }
    }
}

#[cfg(test)]
mod eval_tests {
    use crate::syntax::term_tests::{example_term1, example_term2};
    use common::{values::Unit, Eval};

    #[test]
    fn eval1() {
        let result = example_term1().eval_start().unwrap();
        let expected = Unit.into();
        assert_eq!(result, expected)
    }

    #[test]
    fn eval2() {
        let result = example_term2().eval_start().unwrap();
        let expected = Unit.into();
        assert_eq!(result, expected)
    }
}
