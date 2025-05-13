use super::{terms::Term, types::Type};
use common::{
    check::{CheckEnvironment, Typecheck},
    check::{Kindcheck, Subtypecheck},
    errors::{Error, ErrorKind},
    kinds::Kind,
    Location, TypeVar, Var,
};

#[derive(Default, Clone)]
pub struct Env;

impl CheckEnvironment for Env {
    type Type = Type;
    fn get_var(&self, v: &Var) -> Result<Self::Type, ErrorKind> {
        Err(ErrorKind::FreeVariable(v.clone()))
    }
    fn add_var(&mut self, _: Var, _: Type) {}

    fn get_tyvar_kind(&self, v: &TypeVar) -> Result<Kind, ErrorKind> {
        Err(ErrorKind::FreeTypeVariable(v.clone()))
    }
    fn add_tyvar_kind(&mut self, _: TypeVar, _: Kind) {}

    fn get_tyvar_super(&self, v: &TypeVar) -> Result<Self::Type, ErrorKind> {
        Err(ErrorKind::FreeTypeVariable(v.clone()))
    }
    fn add_tyvar_super(&mut self, _: TypeVar, _: Self::Type) {}

    fn get_loc(&self, loc: &Location) -> Result<Self::Type, ErrorKind> {
        Err(ErrorKind::UndefinedLocation(*loc))
    }
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
}

impl Kindcheck<Type> for Type {
    type Env = Env;
    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}
