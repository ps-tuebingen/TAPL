use super::{FOmegaSub, types::Type};
use check::{Kindcheck, Typecheck};
use derivations::Derivation;
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    terms::{
        App, Lambda, LambdaSub, Let, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack,
        Variable,
    },
};

impl Kindcheck for Type {
    type Lang = FOmegaSub;
    fn check_kind(
        &self,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        match self {
            Type::Var(var) => var.check_kind(env),
            Type::Top(top) => top.check_kind(env),
            Type::Fun(fun) => fun.check_kind(env),
            Type::Forall(forall) => forall.check_kind(env),
            Type::OpLambdaSub(lam) => lam.check_kind(env),
            Type::OpApp(app) => app.check_kind(env),
            Type::Exists(ex) => ex.check_kind(env),
            Type::Record(rec) => rec.check_kind(env),
            Type::Nat(nat) => nat.check_kind(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<FOmegaSub>::rules());
        rules.extend(Lambda::<FOmegaSub>::rules());
        rules.extend(App::<FOmegaSub>::rules());
        rules.extend(LambdaSub::<FOmegaSub>::rules());
        rules.extend(TyApp::<FOmegaSub>::rules());
        rules.extend(Pack::<FOmegaSub>::rules());
        rules.extend(Unpack::<FOmegaSub>::rules());
        rules.extend(Record::<FOmegaSub>::rules());
        rules.extend(RecordProj::<FOmegaSub>::rules());
        rules.extend(Num::<FOmegaSub>::rules());
        rules.extend(Succ::<FOmegaSub>::rules());
        rules.extend(Pred::<FOmegaSub>::rules());
        rules.extend(Let::<FOmegaSub>::rules());
        rules
    }
}
