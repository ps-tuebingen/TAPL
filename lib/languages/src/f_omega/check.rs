use super::{FOmega, types::Type};
use check::{Kindcheck, Typecheck};
use derivations::Derivation;
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    terms::{
        App, False, Fix, If, IsZero, Lambda, Num, Pack, Pred, Record, RecordProj, Succ, True,
        TyApp, TyLambda, Unit, Unpack, Variable,
    },
};

impl Kindcheck for Type {
    type Lang = FOmega;
    fn check_kind(
        &self,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        match self {
            Type::Var(var) => var.check_kind(env),
            Type::Fun(fun) => fun.check_kind(env),
            Type::Forall(forall) => forall.check_kind(env),
            Type::OpLambda(lam) => lam.check_kind(env),
            Type::OpApp(app) => app.check_kind(env),
            Type::Exists(ex) => ex.check_kind(env),
            Type::Record(rec) => rec.check_kind(env),
            Type::Bool(b) => b.check_kind(env),
            Type::Unit(u) => u.check_kind(env),
            Type::Nat(nat) => nat.check_kind(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<FOmega>::rules());
        rules.extend(Lambda::<FOmega>::rules());
        rules.extend(App::<FOmega>::rules());
        rules.extend(TyLambda::<FOmega>::rules());
        rules.extend(TyApp::<FOmega>::rules());
        rules.extend(Pack::<FOmega>::rules());
        rules.extend(Unpack::<FOmega>::rules());
        rules.extend(Record::<FOmega>::rules());
        rules.extend(RecordProj::<FOmega>::rules());
        rules.extend(True::<FOmega>::rules());
        rules.extend(False::<FOmega>::rules());
        rules.extend(If::<FOmega>::rules());
        rules.extend(Unit::<FOmega>::rules());
        rules.extend(Fix::<FOmega>::rules());
        rules.extend(Num::<FOmega>::rules());
        rules.extend(Succ::<FOmega>::rules());
        rules.extend(Pred::<FOmega>::rules());
        rules.extend(IsZero::<FOmega>::rules());
        rules
    }
}
