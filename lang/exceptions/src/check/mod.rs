use crate::{syntax::Term, to_err, types::Type};
use common::{
    check::{CheckEnvironment, Typecheck},
    errors::{Error, ErrorKind, ErrorLocation},
    Var,
};
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct ExceptionEnv {
    env: HashMap<Var, Type>,
}

impl CheckEnvironment<Type> for ExceptionEnv {
    fn get_var(&self, v: &Var) -> Result<Type, Error> {
        self.env
            .get(v)
            .cloned()
            .ok_or(to_check_err(ErrorKind::FreeVariable(v.clone())))
    }

    fn add_var(&mut self, v: Var, ty: Type) {
        self.env.insert(v, ty);
    }
}

impl Typecheck<ExceptionEnv, Type> for Term {
    fn check_start(&self) -> Result<Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: &mut ExceptionEnv) -> Result<Type, Error> {
        match self {
            Term::Var(v) => v.check(env),
            Term::Num(num) => num.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::Succ(s) => s.check(env),
            Term::Pred(p) => p.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::If(ift) => ift.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(u) => u.check(env),
            Term::Exception(exc) => exc.check(env),
            Term::Try(t) => t.check(env),
            Term::Raise(r) => r.check(env),
            Term::TryWithVal(t) => t.check(env),
        }
    }
}

fn to_check_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Check)
}

#[cfg(test)]
mod check_tests {
    use super::Type;
    use crate::syntax::term_tests::{example_term1, example_term2};
    use common::{check::Typecheck, types::Unit};

    #[test]
    fn check1() {
        let result = example_term1().check(&mut Default::default()).unwrap();
        let expected = Type::Unit(Unit);
        assert_eq!(result, expected)
    }

    #[test]
    fn check2() {
        let result = example_term2().check(&mut Default::default()).unwrap();
        let expected = Type::Unit(Unit);
        assert_eq!(result, expected)
    }
}
