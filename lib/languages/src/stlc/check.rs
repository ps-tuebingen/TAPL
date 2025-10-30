use super::{Stlc, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::{NoKinding, NoSubtyping, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    terms::{
        App, Ascribe, Cons, False, Fix, Fst, Head, If, IsNil, IsZero, Lambda, Left, Let, Nil,
        Nothing, Num, Pair, Pred, Projection, Record, RecordProj, Right, Snd, SomeCase, Something,
        Succ, SumCase, Tail, True, Tuple, Unit, Variable, Variant, VariantCase,
    },
};

impl Typecheck for Term {
    type Lang = Stlc;

    fn check(&self, env: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        match self {
            Term::Var(v) => v.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(unit) => unit.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Num(num) => num.check(env),
            Term::Pred(p) => p.check(env),
            Term::Succ(s) => s.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::Ascribe(asc) => asc.check(env),
            Term::Let(lt) => lt.check(env),
            Term::Pair(pr) => pr.check(env),
            Term::Tuple(tup) => tup.check(env),
            Term::Projection(proj) => proj.check(env),
            Term::Fst(proj) => proj.check(env),
            Term::Snd(proj) => proj.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
            Term::Left(lf) => lf.check(env),
            Term::Right(rt) => rt.check(env),
            Term::SumCase(case) => case.check(env),
            Term::Variant(var) => var.check(env),
            Term::VariantCase(case) => case.check(env),
            Term::Nothing(not) => not.check(env),
            Term::Something(some) => some.check(env),
            Term::SomeCase(case) => case.check(env),
            Term::Fix(fix) => fix.check(env),
            Term::Nil(nil) => nil.check(env),
            Term::Cons(cons) => cons.check(env),
            Term::IsNil(isnil) => isnil.check(env),
            Term::Head(hd) => hd.check(env),
            Term::Tail(tl) => tl.check(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<Stlc>::rules());
        rules.extend(Lambda::<Stlc>::rules());
        rules.extend(App::<Stlc>::rules());
        rules.extend(Unit::<Stlc>::rules());
        rules.extend(True::<Stlc>::rules());
        rules.extend(False::<Stlc>::rules());
        rules.extend(If::<Stlc>::rules());
        rules.extend(Num::<Stlc>::rules());
        rules.extend(Pred::<Stlc>::rules());
        rules.extend(Succ::<Stlc>::rules());
        rules.extend(IsZero::<Stlc>::rules());
        rules.extend(Ascribe::<Stlc>::rules());
        rules.extend(Let::<Stlc>::rules());
        rules.extend(Pair::<Stlc>::rules());
        rules.extend(Tuple::<Stlc>::rules());
        rules.extend(Projection::<Stlc>::rules());
        rules.extend(Fst::<Stlc>::rules());
        rules.extend(Snd::<Stlc>::rules());
        rules.extend(Record::<Stlc>::rules());
        rules.extend(RecordProj::<Stlc>::rules());
        rules.extend(Left::<Stlc>::rules());
        rules.extend(Right::<Stlc>::rules());
        rules.extend(SumCase::<Stlc>::rules());
        rules.extend(Variant::<Stlc>::rules());
        rules.extend(VariantCase::<Stlc>::rules());
        rules.extend(Nothing::<Stlc>::rules());
        rules.extend(Something::<Stlc>::rules());
        rules.extend(SomeCase::<Stlc>::rules());
        rules.extend(Fix::<Stlc>::rules());
        rules.extend(Nil::<Stlc>::rules());
        rules.extend(Cons::<Stlc>::rules());
        rules.extend(IsNil::<Stlc>::rules());
        rules.extend(Head::<Stlc>::rules());
        rules.extend(Tail::<Stlc>::rules());
        rules
    }
}

impl Subtypecheck for Type {
    type Lang = Stlc;

    fn check_subtype(
        &self,
        _: &Self,
        _: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoSubtyping::new("Stlc").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

impl Kindcheck for Type {
    type Lang = Stlc;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("Stlc").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
