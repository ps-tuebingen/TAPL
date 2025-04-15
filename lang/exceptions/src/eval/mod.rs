pub mod ift;
pub mod iszero;
pub mod lambda;
pub mod pred;
pub mod raise;
pub mod succ;
pub mod tryt;
pub mod tryval;
pub mod unit;
pub mod values;
use super::to_err;
use crate::syntax::Term;
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    eval::Eval,
};
use values::Value;

pub fn to_eval_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Eval)
}

impl Eval<Value, (), Term> for Term {
    fn eval_start(self) -> Result<Value, Error> {
        self.eval(&mut ())
    }

    fn eval(self, env: &mut ()) -> Result<Value, Error> {
        match self {
            Term::Var(v) => v.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Unit(u) => u.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::Succ(s) => s.eval(env),
            Term::Pred(p) => p.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::IsZero(isz) => isz.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Exception(exc) => exc.eval(env),
            Term::Try(tr) => tr.eval(env),
            Term::TryWithVal(tr) => tr.eval(env),
            Term::Raise(r) => r.eval(env),
        }
    }
}

#[cfg(test)]
mod eval_tests {
    use super::Value;
    use crate::syntax::term_tests::{example_term1, example_term2};
    use common::Eval;

    #[test]
    fn eval1() {
        let result = example_term1().eval(Default::default()).unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval2() {
        let result = example_term2().eval(Default::default()).unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }
}
