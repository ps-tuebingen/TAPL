use super::{LambdaOmega, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::{NoSubtyping, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    terms::{App, False, Lambda, Num, True, TyApp, TyLambda, Unit, Variable},
};

impl Subtypecheck for Type {
    type Lang = LambdaOmega;
    fn check_subtype(
        &self,
        _: &Type,
        _: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoSubtyping::new("Lambda Omega").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

impl Kindcheck for Type {
    type Lang = LambdaOmega;
    fn check_kind(
        &self,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
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

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<LambdaOmega>::rules());
        rules.extend(Num::<LambdaOmega>::rules());
        rules.extend(True::<LambdaOmega>::rules());
        rules.extend(False::<LambdaOmega>::rules());
        rules.extend(Lambda::<LambdaOmega>::rules());
        rules.extend(App::<LambdaOmega>::rules());
        rules.extend(Unit::<LambdaOmega>::rules());
        rules.extend(TyLambda::<LambdaOmega>::rules());
        rules.extend(TyApp::<LambdaOmega>::rules());
        rules
    }
}
