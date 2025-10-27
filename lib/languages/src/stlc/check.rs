use super::{Stlc, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::{NoKinding, NoSubtyping, check_error::CheckError};
use syntax::env::Environment;

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
}

impl Kindcheck for Type {
    type Lang = Stlc;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("Stlc").into())
    }
}
