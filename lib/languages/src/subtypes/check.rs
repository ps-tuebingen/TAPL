use super::{Subtypes, types::Type};
use check::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use derivations::{Derivation, NormalizingDerivation};
use errors::{NoKinding, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    terms::{
        App, Assign, Cast, Cons, Deref, False, Fix, If, Lambda, Let, Loc, Nil, Num, Pred, Record,
        RecordProj, Ref, Succ, True, Unit, Variable, Variant, VariantCase,
    },
};

impl Subtypecheck for Type {
    type Lang = Subtypes;

    fn check_subtype(
        &self,
        sup: &Self,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        match self {
            Type::Top(top) => top.check_subtype(sup, env),
            Type::Bot(bot) => bot.check_subtype(sup, env),
            Type::Fun(fun) => fun.check_subtype(sup, env),
            Type::Record(rec) => rec.check_subtype(sup, env),
            Type::Variant(variant) => variant.check_subtype(sup, env),
            Type::List(list) => list.check_subtype(sup, env),
            Type::Ref(refty) => refty.check_subtype(sup, env),
            Type::Source(src) => src.check_subtype(sup, env),
            Type::Sink(snk) => snk.check_subtype(sup, env),
            Type::Nat(nat) => nat.check_subtype(sup, env),
            Type::Unit(unit) => unit.check_subtype(sup, env),
            Type::Bool(b) => b.check_subtype(sup, env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<Subtypes>::rules());
        rules.extend(Lambda::<Subtypes>::rules());
        rules.extend(App::<Subtypes>::rules());
        rules.extend(Unit::<Subtypes>::rules());
        rules.extend(Record::<Subtypes>::rules());
        rules.extend(RecordProj::<Subtypes>::rules());
        rules.extend(Variant::<Subtypes>::rules());
        rules.extend(VariantCase::<Subtypes>::rules());
        rules.extend(Cast::<Subtypes>::rules());
        rules.extend(Nil::<Subtypes>::rules());
        rules.extend(Cons::<Subtypes>::rules());
        rules.extend(Ref::<Subtypes>::rules());
        rules.extend(Deref::<Subtypes>::rules());
        rules.extend(Assign::<Subtypes>::rules());
        rules.extend(Loc::<Subtypes>::rules());
        rules.extend(Num::<Subtypes>::rules());
        rules.extend(Succ::<Subtypes>::rules());
        rules.extend(Pred::<Subtypes>::rules());
        rules.extend(True::<Subtypes>::rules());
        rules.extend(False::<Subtypes>::rules());
        rules.extend(If::<Subtypes>::rules());
        rules.extend(Let::<Subtypes>::rules());
        rules.extend(Fix::<Subtypes>::rules());
        rules
    }
}

impl Kindcheck for Type {
    type Lang = Subtypes;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("Subtypes").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

impl Normalize for Type {
    type Lang = Subtypes;
    fn normalize(self, _: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        NormalizingDerivation::empty(self).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
