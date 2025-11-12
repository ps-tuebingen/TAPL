use super::{BoundedQuantification, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::{NoKinding, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    terms::{
        App, Lambda, LambdaSub, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack, Variable,
    },
};

impl Subtypecheck for Type {
    type Lang = BoundedQuantification;
    fn check_subtype(
        &self,
        sup: &Self,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        match self {
            Type::Var(var) => var.check_subtype(sup, env),
            Type::Top(t) => t.check_subtype(sup, env),
            Type::Nat(n) => n.check_subtype(sup, env),
            Type::Fun(f) => f.check_subtype(sup, env),
            Type::Forall(f) => f.check_subtype(sup, env),
            Type::Exists(e) => e.check_subtype(sup, env),
            Type::Record(rec) => rec.check_subtype(sup, env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<BoundedQuantification>::rules());
        rules.extend(Num::<BoundedQuantification>::rules());
        rules.extend(Succ::<BoundedQuantification>::rules());
        rules.extend(Pred::<BoundedQuantification>::rules());
        rules.extend(Lambda::<BoundedQuantification>::rules());
        rules.extend(App::<BoundedQuantification>::rules());
        rules.extend(LambdaSub::<BoundedQuantification>::rules());
        rules.extend(TyApp::<BoundedQuantification>::rules());
        rules.extend(Pack::<BoundedQuantification>::rules());
        rules.extend(Unpack::<BoundedQuantification>::rules());
        rules.extend(Record::<BoundedQuantification>::rules());
        rules.extend(RecordProj::<BoundedQuantification>::rules());
        rules
    }
}

impl Kindcheck for Type {
    type Lang = BoundedQuantification;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("Bounded Quantification").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
