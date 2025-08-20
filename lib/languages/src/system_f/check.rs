use super::{terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::{NoSubtyping, check_error::CheckError};
use syntax::{env::Environment, kinds::Kind};

impl Typecheck for Term {
    type Term = Term;
    type Type = Type;

    fn check(
        &self,
        env: Environment<Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        match self {
            Term::Var(var) => var.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::TyLambda(lam) => lam.check(env),
            Term::TyApp(app) => app.check(env),
        }
    }
}

impl Subtypecheck for Type {
    type Type = Type;
    type Term = Term;
    fn check_subtype(
        &self,
        _: &Type,
        _: Environment<Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        Err(NoSubtyping::new("System F").into())
    }
}

impl Kindcheck<Type> for Type {
    fn check_kind(&self, _: Environment<Type>) -> Result<Kind, CheckError> {
        Ok(Kind::Star)
    }
}
