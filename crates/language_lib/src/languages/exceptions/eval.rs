use super::{check::ExceptionEnv, terms::Term, types::Type, values::Value};
use check::Normalize;
use common::errors::Error;
use eval::Eval;
use syntax::terms::{False, Num, True, Unit, Variable};

impl Eval for Term {
    type Value = Value;
    type Env = ();

    fn eval(self, env: &mut ()) -> Result<Value, Error> {
        match self {
            Term::Var(v) => <Variable<Term> as Eval>::eval(v, env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Unit(u) => <Unit<Term> as Eval>::eval(u, env),
            Term::True(tru) => <True<Term> as Eval>::eval(tru, env),
            Term::False(fls) => <False<Term> as Eval>::eval(fls, env),
            Term::Succ(s) => s.eval(env),
            Term::Pred(p) => p.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::IsZero(isz) => isz.eval(env),
            Term::Num(num) => <Num<Term> as Eval>::eval(num, env),
            Term::Exception(exc) => exc.eval(env),
            Term::Try(tr) => tr.eval(env),
            Term::TryWithVal(tr) => tr.eval(env),
            Term::Raise(r) => r.eval(env),
        }
    }
}

impl Normalize<Type> for Type {
    type Env = ExceptionEnv;
    fn normalize(self, _: &mut Self::Env) -> Type {
        self
    }
}

#[cfg(test)]
mod eval_tests {
    use super::super::terms::term_tests::{example_term1, example_term2};
    use eval::{values::Unit, Eval};

    #[test]
    fn eval1() {
        let result = example_term1().eval_start().unwrap();
        let expected = Unit::new().into();
        assert_eq!(result, expected)
    }

    #[test]
    fn eval2() {
        let result = example_term2().eval_start().unwrap();
        let expected = Unit::new().into();
        assert_eq!(result, expected)
    }
}
