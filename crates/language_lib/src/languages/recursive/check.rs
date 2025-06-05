use super::{errors::Error, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivation::Derivation;
use syntax::{env::Environment, kinds::Kind};

impl Typecheck for Term {
    type Term = Term;
    type Type = Type;
    type CheckError = Error;

    fn check(
        &self,
        env: &mut Environment<Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Error> {
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
}

impl Subtypecheck<Type> for Type {
    type CheckError = Error;

    fn check_subtype(&self, _: &Self, _: &mut Environment<Type>) -> Result<(), Error> {
        Ok(())
    }
}

impl Kindcheck<Type> for Type {
    type CheckError = Error;

    fn check_kind(&self, _: &mut Environment<Type>) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}
