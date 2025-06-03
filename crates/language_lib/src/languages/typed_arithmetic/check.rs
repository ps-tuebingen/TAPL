use super::{errors::Error, terms::Term, types::Type};
use check::{CheckEnvironment, Kindcheck, Subtypecheck, Typecheck};
use common::errors::NotImplemented;
use syntax::{kinds::Kind, Location, TypeVar, Var};

#[derive(Default, Clone)]
pub struct Env;

impl CheckEnvironment for Env {
    type Type = Type;
    type CheckError = Error;

    fn get_var(&self, _: &Var) -> Result<Self::Type, Self::CheckError> {
        Err(NotImplemented.into())
    }
    fn add_var(&mut self, _: Var, _: Type) {}

    fn get_tyvar_kind(&self, _: &TypeVar) -> Result<Kind, Self::CheckError> {
        Err(NotImplemented.into())
    }
    fn add_tyvar_kind(&mut self, _: TypeVar, _: Kind) {}

    fn get_tyvar_super(&self, _: &TypeVar) -> Result<Self::Type, Self::CheckError> {
        Err(NotImplemented.into())
    }
    fn add_tyvar_super(&mut self, _: TypeVar, _: Self::Type) {}

    fn get_loc(&self, _: &Location) -> Result<Self::Type, Self::CheckError> {
        Err(NotImplemented.into())
    }
}

impl Typecheck for Term {
    type Type = Type;
    type Env = Env;
    type CheckError = Error;

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
    type CheckError = Error;

    fn check_subtype(&self, _: &Self, _: &mut Env) -> Result<(), Error> {
        Ok(())
    }
}

impl Kindcheck<Type> for Type {
    type Env = Env;
    type CheckError = Error;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}
