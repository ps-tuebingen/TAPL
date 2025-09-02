use super::{LambdaOmega, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::{NoSubtyping, check_error::CheckError};
use syntax::{env::Environment, kinds::Kind};

impl Typecheck for Term {
    type Lang = LambdaOmega;

    fn check(&self, env: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        match self {
            Term::Var(var) => var.check(env),
            Term::Num(num) => num.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(u) => u.check(env),
            Term::TyLambda(tylam) => tylam.check(env),
            Term::TyApp(tyapp) => tyapp.check(env),
        }
    }
}

impl Subtypecheck for Type {
    type Lang = LambdaOmega;
    fn check_subtype(
        &self,
        _: &Type,
        _: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoSubtyping::new("Lambda Omega").into())
    }
}

impl Kindcheck for Type {
    type Lang = LambdaOmega;
    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        match self {
            Type::Var(var) => var.check_kind(env),
            Type::Unit(u) => u.check_kind(env),
            Type::Bool(b) => b.check_kind(env),
            Type::Nat(n) => n.check_kind(env),
            Type::OpLambda(oplam) => oplam.check_kind(env),
            Type::OpApp(opapp) => opapp.check_kind(env),
            Type::Fun(fun) => fun.check_kind(env),
            Type::Forall(forall) => forall.check_kind(env),
        }
    }
}
