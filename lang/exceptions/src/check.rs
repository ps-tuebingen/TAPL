use crate::{terms::Term, types::Type};
use common::{
    check::{to_check_err, CheckEnvironment, Subtypecheck, Typecheck},
    errors::{Error, ErrorKind},
    Var,
};
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct ExceptionEnv {
    env: HashMap<Var, Type>,
}

impl CheckEnvironment for ExceptionEnv {
    type Type = Type;

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

impl Typecheck for Term {
    type Type = Type;
    type Env = ExceptionEnv;

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

impl Subtypecheck<Type> for Type {
    type Env = ExceptionEnv;
    fn check_subtype(&self, _: &Self, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
    fn check_supertype(&self, _: &Self, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod check_tests {
    use super::Type;
    use crate::terms::term_tests::{example_term1, example_term2};
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
