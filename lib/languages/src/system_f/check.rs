use super::{SystemF, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::{NoKinding, NoSubtyping, check_error::CheckError};
use syntax::env::Environment;

impl Typecheck for Term {
    type Lang = SystemF;

    fn check(&self, env: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
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
    type Lang = SystemF;
    fn check_subtype(
        &self,
        _: &Type,
        _: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoSubtyping::new("System F").into())
    }
}

impl Kindcheck for Type {
    type Lang = SystemF;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("System F").into())
    }
}
