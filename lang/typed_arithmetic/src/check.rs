use crate::{terms::Term, types::Type};
use common::{
    check::Subtypecheck,
    check::{to_check_err, CheckEnvironment, Typecheck},
    errors::{Error, ErrorKind},
    Var,
};

#[derive(Default, Clone)]
pub struct Env;

impl CheckEnvironment for Env {
    type Type = Type;
    fn get_var(&self, v: &Var) -> Result<Self::Type, Error> {
        Err(to_check_err(ErrorKind::FreeVariable(v.clone())))
    }

    fn add_var(&mut self, _: Var, _: Type) {}
}

impl Typecheck for Term {
    type Type = Type;
    type Env = Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        match self {
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::Num(num) => num.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::If(ift) => ift.check(env),
        }
    }
}

impl Subtypecheck<Type> for Type {
    type Env = Env;
    fn check_subtype(&self, _: &Self, _: &mut Env) -> Result<(), Error> {
        Ok(())
    }
    fn check_supertype(&self, _: &Self, _: &mut Env) -> Result<(), Error> {
        Ok(())
    }
}
