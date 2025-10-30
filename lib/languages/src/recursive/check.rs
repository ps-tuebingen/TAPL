use super::{Recursive, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::{NoKinding, NoSubtyping, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    terms::{
        App, False, Fix, Fold, Fst, If, IsZero, Lambda, Let, Num, Pair, Pred, Record, RecordProj,
        Snd, Succ, True, Unfold, Unit, Variable, Variant, VariantCase,
    },
};

impl Typecheck for Term {
    type Lang = Recursive;

    fn check(&self, env: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        match self {
            Term::Var(v) => v.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(u) => u.check(env),
            Term::Fold(fold) => fold.check(env),
            Term::Unfold(unfold) => unfold.check(env),
            Term::Variant(var) => var.check(env),
            Term::VariantCase(case) => case.check(env),
            Term::Pair(p) => p.check(env),
            Term::Fst(fst) => fst.check(env),
            Term::Snd(snd) => snd.check(env),
            Term::Num(num) => num.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Fix(fix) => fix.check(env),
            Term::Let(lt) => lt.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<Recursive>::rules());
        rules.extend(Lambda::<Recursive>::rules());
        rules.extend(App::<Recursive>::rules());
        rules.extend(Unit::<Recursive>::rules());
        rules.extend(Fold::<Recursive>::rules());
        rules.extend(Unfold::<Recursive>::rules());
        rules.extend(Variant::<Recursive>::rules());
        rules.extend(VariantCase::<Recursive>::rules());
        rules.extend(Pair::<Recursive>::rules());
        rules.extend(Fst::<Recursive>::rules());
        rules.extend(Snd::<Recursive>::rules());
        rules.extend(Num::<Recursive>::rules());
        rules.extend(Succ::<Recursive>::rules());
        rules.extend(Pred::<Recursive>::rules());
        rules.extend(IsZero::<Recursive>::rules());
        rules.extend(True::<Recursive>::rules());
        rules.extend(False::<Recursive>::rules());
        rules.extend(If::<Recursive>::rules());
        rules.extend(Fix::<Recursive>::rules());
        rules.extend(Let::<Recursive>::rules());
        rules.extend(Record::<Recursive>::rules());
        rules.extend(RecordProj::<Recursive>::rules());
        rules
    }
}

impl Subtypecheck for Type {
    type Lang = Recursive;

    fn check_subtype(
        &self,
        _: &Self,
        _: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoSubtyping::new("Recursive").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

impl Kindcheck for Type {
    type Lang = Recursive;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("Recursive").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
